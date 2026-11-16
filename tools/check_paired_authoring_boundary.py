# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Real loopback boundary/native bridge tests; not browser or representative users."""
import http.client
import importlib.util
import json
from pathlib import Path
import tempfile
import threading
import unittest
from unittest.mock import patch
import subprocess
from check_static_contracts import ROOT

spec=importlib.util.spec_from_file_location('paired_server',ROOT/'tools/authoring-prototype/server.py')
server=importlib.util.module_from_spec(spec);spec.loader.exec_module(server)


class Boundary(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.directory=tempfile.TemporaryDirectory()
        fixture=ROOT/'docs/authoring/paired-0.1'
        header=json.loads((fixture/'registry.json').read_bytes())['resources'][0]
        blob=(ROOT/'docs/authoring/examples/service.named-types.json').read_bytes()
        state=server.Snapshot(cls.directory.name,'example',resources=[(header['contract'],header['digest'],blob)],document=(fixture/'document.json').read_bytes())
        cls.http=server.PrototypeServer({'example':state},{'/':(b'<!doctype html><title>Shell only</title>','text/html')})
        cls.thread=threading.Thread(target=cls.http.serve_forever,daemon=True);cls.thread.start()
        cls.tokens={role:token for token,role in cls.http.sessions.items()}
    @classmethod
    def tearDownClass(cls):
        cls.http.shutdown();cls.thread.join(timeout=3);cls.http.server_close();cls.directory.cleanup()
    def request(self,path,role=None,method='GET',data=None,headers=None):
        connection=http.client.HTTPConnection('127.0.0.1',self.http.server_port,timeout=10)
        actual={} if role is None else {'X-Choreoform-Session':self.tokens[role]}
        actual.update(headers or {})
        connection.request(method,path,body=data,headers=actual)
        response=connection.getresponse();raw=response.read();result=response.status,raw,dict(response.getheaders());connection.close();return result
    def test_shell_contains_no_artifact_and_every_data_route_checks_session(self):
        status,raw,headers=self.request('/');self.assertEqual(status,200);self.assertNotIn(b'Choreoform authoring',raw)
        self.assertIn("object-src 'none'",headers['Content-Security-Policy'])
        for path in ['/api/catalog','/api/process/example','/api/registry/example','/api/resources/example','/api/reference/example','/api/export/example']:
            status,raw,_=self.request(path);self.assertEqual(status,401,path);self.assertEqual(json.loads(raw)['code'],'access')
        for token in ['0'*64,'é'*64,'short']:
            status,raw,_=self.request('/api/process/example',headers={'X-Choreoform-Session':token});self.assertEqual(status,401);self.assertNotIn(token.encode(),raw)
    def test_origin_host_paths_and_forged_permissions_cannot_grant_actions(self):
        for headers in [{'Origin':'https://outside.example'},{'Host':'outside.example'}]:
            status,raw,_=self.request('/api/process/example','editor',headers=headers);self.assertEqual(status,403);self.assertNotIn(b'outside',raw)
        for path in ['/api/process/../../.git/config','/api/process/%2e%2e','/.tools/cargo','/api/catalog?session=SECRET']:
            status,raw,_=self.request(path,'editor');self.assertIn(status,[400,404]);self.assertNotIn(b'SECRET',raw)
        forged=json.dumps({'authorized':True,'role':'editor','format':'choreoform-authoring-change'}).encode()
        status,raw,_=self.request('/api/change/example','viewer','POST',forged,{'Content-Type':'application/json'});self.assertEqual(status,403);self.assertEqual(json.loads(raw)['code'],'access')
        status,_,_=self.request('/api/export/example','viewer');self.assertEqual(status,403)
    def test_real_native_changes_conflicts_exports_and_readable_views_are_atomic(self):
        status,raw,_=self.request('/api/process/example','viewer');self.assertEqual(status,200);old=raw
        digest=self.http.catalog['example'].digest
        status,view,_=self.request('/api/reference/example','viewer',headers={'X-Authoring-Base':digest});self.assertEqual(status,200);self.assertEqual(len(json.loads(view)['entities']),21)
        request={'format':'choreoform-authoring-change','version':'0.1.0','base':digest,'operation':{'kind':'rename','target':'same','name':'Readable test'}}
        data=json.dumps(request).encode();status,new,_=self.request('/api/change/example','editor','POST',data,{'Content-Type':'application/json'});self.assertEqual(status,200)
        self.assertNotEqual(old,new);self.assertEqual(json.loads(old)['projection'],json.loads(new)['projection'])
        self.assertIn('Define test "Readable test"',json.loads(new)['source'])
        status,_,_=self.request('/api/change/example','editor','POST',data,{'Content-Type':'application/json'});self.assertEqual(status,409)
        status,_,_=self.request('/api/export/example','exporter',headers={'X-Authoring-Base':digest});self.assertEqual(status,409)
        status,snapshot,headers=self.request('/api/export/example','exporter',headers={'X-Authoring-Base':self.http.catalog['example'].digest});self.assertEqual(status,200);self.assertEqual(snapshot,new)
        self.assertEqual(headers['Content-Disposition'],'attachment; filename="process-example.choreoform.json"')
        # Unknown ID-shaped data is neither a managed identity nor a diagnostic echo.
        request.update(base=self.http.catalog['example'].digest,operation=dict(kind='rename',target='SECRET-MUST-NOT-LEAK',name='New name'))
        status,raw,_=self.request('/api/change/example','editor','POST',json.dumps(request).encode(),{'Content-Type':'application/json'});self.assertEqual(status,422);self.assertNotIn(b'SECRET',raw)
        self.assertEqual(self.http.catalog['example'].document,new)
    def test_request_sizes_unknown_methods_and_native_panic_text_do_not_disclose_payload(self):
        status,raw,_=self.request('/api/change/example','editor','POST',b'',{'Content-Type':'application/json','Content-Length':str(server.MIB+1)});self.assertEqual(status,413)
        status,raw,_=self.request('/api/change/example','editor','POST',b'{}',{'Content-Type':'text/plain'});self.assertEqual(status,400)
        status,raw,_=self.request('/','editor','TRACE');self.assertEqual(status,400);self.assertEqual(json.loads(raw)['code'],'request')
        # A failed native subprocess is simulated only to test the disclosure boundary.
        with patch.object(server.subprocess, 'run', return_value=subprocess.CompletedProcess([], 1, b'', b'panic at /PRIVATE/PATH: SECRET_SOURCE')):
            with self.assertRaisesRegex(server.NativeFailure, '^native-refusal$'):
                server.Snapshot.invoke(['unused-native-test-command'])


if __name__=='__main__':unittest.main()
