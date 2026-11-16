# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Loopback experimental artifact boundary. No process execution or Python semantics."""
import argparse
from hashlib import sha256
from http.server import BaseHTTPRequestHandler, HTTPServer
import json
from pathlib import Path
import re
import secrets
import subprocess
import tempfile
from urllib.parse import urlsplit

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
MIB = 1024 * 1024
BINARY = ROOT / 'target/debug/choreoform-authoring-prototype'
FRONTEND = ROOT / 'target/debug/choreoform-authoring-frontend'
ROLES = {'editor': dict(read=True, edit=True, export=True), 'viewer': dict(read=True, edit=False, export=False), 'exporter': dict(read=True, edit=False, export=True)}
CAPTIONS = {'example': 'Example process', 'rp01': 'Expense reimbursement', 'rp03': 'Order fulfillment'}


def encoded(value): return json.dumps(value, ensure_ascii=False, separators=(',', ':')).encode()


def read_approved(path, roots, maximum=MIB):
    path = path.resolve()
    if not any(path.is_relative_to(root.resolve()) for root in roots): raise ValueError('read')
    with path.open('rb') as stream: raw = stream.read(maximum + 1)
    if len(raw) > maximum: raise ValueError('size')
    return raw


class NativeFailure(Exception): pass


class Snapshot:
    def __init__(self, directory, key, source=None, bindings=None, resources=(), document=None):
        self.base = Path(directory) / key
        self.base.mkdir()
        self.serial = 0
        rows = []; blob = bytearray()
        if len(resources) > 128: raise ValueError('registry-limit')
        for identity, digest, raw in resources:
            if len(raw) > MIB or len(blob) + len(raw) > 8*MIB or digest != 'sha256:' + sha256(raw).hexdigest(): raise ValueError('registry-limit')
            rows.append(dict(contract=identity, digest=digest, start=len(blob), length=len(raw)))
            blob.extend(raw)
        self.registry = encoded(dict(format='choreoform-authoring-resource-registry', version='0.1.0', resources=rows))
        self.blob = bytes(blob)
        (self.base/'registry').write_bytes(self.registry); (self.base/'blob').write_bytes(self.blob)
        if document is None:
            (self.base/'source').write_bytes(source); (self.base/'bindings').write_bytes(bindings)
            (self.base/'layout').write_bytes(b'{"format":"choreoform-authoring-layout","version":"0.1.0","entries":[]}')
            extra = []
            for index, row in enumerate(resources):
                path = self.base/f'resource-{index}'; path.write_bytes(row[2]); extra += ['--resource', row[0], row[1], str(path)]
            document = self.invoke([str(FRONTEND), 'document-import', str(self.base/'source'), str(self.base/'bindings'), str(self.base/'layout'), *extra])
        self.document = self.bridge('canonical', document)
        self.view = self.bridge('view', self.document)
        self.digest = json.loads(self.view)['binding']['authoring']

    @staticmethod
    def invoke(args):
        try: result = subprocess.run(args, capture_output=True, timeout=30)
        except (OSError, subprocess.TimeoutExpired): raise NativeFailure('io') from None
        if result.returncode:
            # Native adapters own private diagnostics; never include argv/paths.
            message = result.stderr.decode('utf-8', errors='replace').strip()
            if len(message) > 1024 or not re.fullmatch(r'[a-z][a-z0-9-]*(?: at (?:document|source|bindings|layout|change|resources)(?:/[A-Za-z0-9_/-]*)?(?:; entity [A-Za-z][A-Za-z0-9_-]{0,63})?; source bytes [0-9]{1,7}\.\.[0-9]{1,7})?', message): message = 'native-refusal'
            raise NativeFailure(message)
        if len(result.stdout) > 16*MIB: raise NativeFailure('size')
        return result.stdout

    def bridge(self, command, document, change=None):
        path = self.base/f'input-{self.serial}'; self.serial += 1
        path.write_bytes(document)
        args = [str(BINARY), command, str(path)]
        change_path = None
        if change is not None:
            change_path = self.base/f'change-{self.serial}'; change_path.write_bytes(change); args.append(str(change_path))
        args += [str(self.base/'registry'), str(self.base/'blob')]
        try: return self.invoke(args)
        finally:
            path.unlink(missing_ok=True)
            if change_path is not None: change_path.unlink(missing_ok=True)

    def apply(self, change):
        result = self.bridge('change', self.document, change)
        view = self.bridge('view', result)  # Never commit a partial/unrenderable state.
        digest = json.loads(view)['binding']['authoring']
        temporary = self.base/'next-document'; temporary.write_bytes(result)
        temporary.replace(self.base/'current-document')
        self.document, self.view, self.digest = result, view, digest
        return result


def load_catalog(directory):
    result = {}
    fixture = ROOT/'docs/authoring/paired-0.1'
    document = read_approved(fixture/'document.json', [fixture], 16*MIB)
    registry = json.loads(read_approved(fixture/'registry.json', [fixture], 4*MIB))
    raw = read_approved(ROOT/'docs/authoring/examples/service.named-types.json', [ROOT/'docs/authoring/examples'])
    row = registry['resources'][0]
    result['example'] = Snapshot(directory, 'example', resources=[(row['contract'], row['digest'], raw)], document=document)
    for key in ['rp01','rp03']:
        base = ROOT/'docs/benchmarks'/key
        manifest = json.loads(read_approved(base/'manifest.json', [base]))
        rows = [(r['id'], r['revision'], read_approved(ROOT/r['path'], [base, ROOT/'docs/ir/contracts'])) for r in manifest['resources']]
        result[key] = Snapshot(directory, key, read_approved(base/'source.choreo', [base]), read_approved(base/'bindings.json', [base]), rows)
    return result


def load_assets():
    paths = {'/': (HERE/'web/index.html','text/html; charset=utf-8'), '/app.mjs': (HERE/'web/app.mjs','text/javascript; charset=utf-8'), '/style.css': (HERE/'web/style.css','text/css; charset=utf-8'), '/generated/choreoform_authoring_prototype.js': (HERE/'web/generated/choreoform_authoring_prototype.js','text/javascript; charset=utf-8'), '/generated/choreoform_authoring_prototype_bg.wasm': (HERE/'web/generated/choreoform_authoring_prototype_bg.wasm','application/wasm'), '/imports.json': (HERE/'web/generated/imports.json','application/json')}
    return {name: (read_approved(path, [HERE/'web'], 32*MIB), kind) for name, (path, kind) in paths.items()}


class PrototypeServer(HTTPServer):
    def __init__(self, catalog, assets, port=0):
        self.catalog, self.assets = catalog, assets
        self.sessions = {secrets.token_hex(32): role for role in ROLES}
        super().__init__(('127.0.0.1', port), Handler)
        self.origin = f'http://127.0.0.1:{self.server_port}'
        self.expected_host = f'127.0.0.1:{self.server_port}'

    def get_request(self):
        connection, address = super().get_request()
        connection.settimeout(10)  # Bound idle/header reads too, before authentication.
        return connection, address


class Handler(BaseHTTPRequestHandler):
    def log_message(self, *args): pass
    def send_error(self, code, message=None, explain=None): self.error(400, 'request')
    def reply(self, code, body, kind='application/json', extra=()):
        self.send_response(code)
        for key,value in [('Content-Type',kind), ('Content-Length',str(len(body))), ('Cache-Control','no-store'), ('Connection','close'), ('X-Content-Type-Options','nosniff'), ('Referrer-Policy','no-referrer'), ('Content-Security-Policy',"default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'"), *extra]: self.send_header(key,value)
        self.end_headers()
        try: self.wfile.write(body)
        except (BrokenPipeError, ConnectionResetError): pass
    def error(self, code, message, diagnostic=None):
        self.reply(code,encoded(dict(format='choreoform-prototype-error',version='0.1.0',code=message,diagnostic=diagnostic)))
    def context(self):
        hosts=self.headers.get_all('Host',[]); origins=self.headers.get_all('Origin',[])
        if hosts != [self.server.expected_host] or len(origins)>1 or origins and origins[0] != self.server.origin:
            self.error(403,'access'); return False
        if len(self.path)>256:self.error(400,'request');return False
        try:path=urlsplit(self.path)
        except ValueError:self.error(400,'request');return False
        if path.scheme or path.netloc or path.query or path.fragment:
            self.error(400,'request'); return False
        return True
    def role(self):
        tokens=self.headers.get_all('X-Choreoform-Session',[])
        if len(tokens)!=1 or len(tokens[0])!=64 or any(c not in '0123456789abcdef' for c in tokens[0]):
            self.error(401,'access'); return None
        role=next((role for token,role in self.server.sessions.items() if secrets.compare_digest(token,tokens[0])),None)
        if role is None: self.error(401,'access')
        return role
    def do_GET(self):
        if not self.context(): return
        if self.path in self.server.assets:
            raw,kind=self.server.assets[self.path];self.reply(200,raw,kind);return
        role=self.role()
        if role is None:return
        permissions=ROLES[role]
        if self.path=='/api/catalog':
            self.reply(200,encoded(dict(format='choreoform-prototype-catalog',version='0.1.0',permissions=permissions,items=[dict(key=k,caption=CAPTIONS[k]) for k in sorted(self.server.catalog)])));return
        parts=self.path.split('/')
        if len(parts)!=4 or parts[:2]!=['','api'] or parts[3] not in self.server.catalog:
            self.error(404,'not-found');return
        action,key=parts[2:];state=self.server.catalog[key]
        if action in ['reference','export']:
            if action=='export' and not permissions['export']:self.error(403,'access');return
            if self.headers.get_all('X-Authoring-Base',[]) != [state.digest]:self.error(409,'conflict');return
        if action=='process':self.reply(200,state.document)
        elif action=='registry':self.reply(200,state.registry)
        elif action=='resources':self.reply(200,state.blob,'application/octet-stream')
        elif action=='reference':self.reply(200,state.view)
        elif action=='export':self.reply(200,state.document,extra=[('Content-Disposition',f'attachment; filename="process-{key}.choreoform.json"')])
        else:self.error(404,'not-found')
    def do_POST(self):
        if not self.context():return
        role=self.role()
        if role is None:return
        if not ROLES[role]['edit']:self.error(403,'access');return
        parts=self.path.split('/')
        if len(parts)!=4 or parts[:3]!=['','api','change'] or parts[3] not in self.server.catalog:self.error(404,'not-found');return
        lengths=self.headers.get_all('Content-Length',[])
        if len(lengths)!=1 or not lengths[0].isascii() or not lengths[0].isdigit() or len(lengths[0])>8 or self.headers.get('Transfer-Encoding') is not None or self.headers.get('Content-Type')!='application/json':self.error(400,'request');return
        length=int(lengths[0])
        if length>MIB:self.error(413,'size');return
        self.connection.settimeout(10)
        try:raw=self.rfile.read(length)
        except (TimeoutError,ConnectionError,OSError):self.error(400,'request');return
        if len(raw)!=length:self.error(400,'request');return
        try:self.reply(200,self.server.catalog[parts[3]].apply(raw))
        except NativeFailure as error:
            message=str(error)
            if message.startswith('stale-base'):self.error(409,'conflict')
            elif message in ['io','read','write']:self.error(500,'io')
            else:self.error(422,'native-refusal',message)
        except OSError:self.error(500,'io')


def main():
    parser=argparse.ArgumentParser(description='Loopback experimental paired authoring; no execution engine.')
    parser.add_argument('--port',type=int,default=0); args=parser.parse_args()
    if not 0<=args.port<=65535:raise SystemExit('port')
    with tempfile.TemporaryDirectory(prefix='choreoform-prototype-') as directory:
        server=PrototypeServer(load_catalog(directory),load_assets(),args.port)
        for token,role in server.sessions.items():print(f'{role}: {server.origin}/#session={token}',flush=True)
        try:server.serve_forever()
        except KeyboardInterrupt:pass
        finally:server.server_close()


if __name__=='__main__':main()
