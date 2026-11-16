# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Test-only shape/identity/order oracle; NOT native evidence admission or host truth."""
from copy import deepcopy
from hashlib import sha256
import json
import os
import re
import subprocess
import unittest

import jsonschema
import rfc8785
from check_authoring_evidence import check_links
from check_reversible_authoring_contract import strict, Refusal
from check_source_capacity import fixed
from check_static_contracts import ROOT

BASE = ROOT/'docs/evidence/recorded-0.1'
MIB = 1024*1024
SCHEMA = json.loads((ROOT/'schemas/evidence/recorded-0.1.schema.json').read_bytes())
REPORT = json.loads((ROOT/'schemas/evidence/report-0.1.schema.json').read_bytes())
VALIDATOR = jsonschema.Draft202012Validator(SCHEMA)


def refuse(code): raise Refusal(code)
def pin(raw): return 'sha256:'+sha256(raw).hexdigest()
def canonical(value): return rfc8785.dumps(value)
def artifact_digest(value): return pin(b'choreoform-recorded-evidence:0.1.0\0'+canonical(value))
def claim_core(doc, row):
    claim=deepcopy(row['claim'])
    if claim['reportedTime']:
        time=claim['reportedTime'];clock=doc['clocks'][time['basis']]
        claim['reportedTime']=dict(source=doc['sources'][clock['source']]['identity'],basis=clock['basis'],unit=clock['unit'],origin=clock['origin'],instant=time['instant'])
    return dict(namespace=doc['namespace'],source=doc['sources'][row['source']]['identity'],event=row['event'],claim=claim)
def claim_digest(doc,row): return pin(b'choreoform-recorded-claim:0.1.0\0'+canonical(claim_core(doc,row)))
def integer(value):
    if len(value)>20 or not re.fullmatch(r'(0|-?[1-9][0-9]*)',value): refuse('evidence-format')
    result=int(value)
    if not -(2**63)<=result<2**63: refuse('evidence-format')
    return result


def check_reference_model(raw):
    """Only the published structural/reference mathematics; typed values checked separately by existing Rust."""
    if len(raw)>MIB: refuse('evidence-limit')
    doc=strict(raw,MIB,'evidence-format')
    for error in VALIDATOR.iter_errors(doc):
        path=list(error.absolute_path)
        if path and path[0] in ['profile','core']:refuse('evidence-profile')
        if 'payload' in path:refuse('evidence-payload')
        if error.validator in ['maxProperties','maxItems','maxLength']:refuse('evidence-limit')
        refuse('evidence-format')
    encoded=canonical(doc)
    if len(encoded)>MIB:refuse('evidence-limit')
    work=0; framing=len(b'choreoform-recorded-evidence:0.1.0\0')+len(encoded)
    def step(n=1):
        nonlocal work
        work+=n
        if work>1_000_000:refuse('evidence-limit')
    def frame(n):
        nonlocal framing
        framing+=n
        if framing>8*MIB:refuse('evidence-limit')
    pending=[doc]
    while pending:
        value=pending.pop();step()
        if isinstance(value,dict):step(len(value));pending.extend(value.values())
        elif isinstance(value,list):pending.extend(value)
    sources=doc['sources'];clocks=doc['clocks'];records=doc['records'];boundaries=doc['boundaries']
    if set(records)&set(boundaries):refuse('evidence-reference')
    if len({v['identity'] for v in sources.values()})!=len(sources):refuse('evidence-identity')
    clock_keys=set()
    for clock in clocks.values():
        step(2)
        if clock['source'] not in sources:refuse('evidence-reference')
        key=clock['source'],clock['basis']
        if key in clock_keys:refuse('evidence-identity')
        clock_keys.add(key)
    def time(value,source):
        if value is None:return
        step(2)
        if source is None or value['basis'] not in clocks or clocks[value['basis']]['source']!=source:refuse('evidence-reference')
        integer(value['instant'])
    identities={};node={}
    for identity,row in records.items():
        step(4)
        if row['source'] not in sources:refuse('evidence-reference')
        claim=row['claim'];delivery=row['delivery']
        if delivery['recorder'] is not None and delivery['recorder'] not in sources:refuse('evidence-reference')
        time(claim['reportedTime'],row['source']);time(delivery['reportedTime'],delivery['recorder'])
        if claim['context'] is not None:
            context=claim['context'];attempt=integer(context['attempt'])
            if not 1<=attempt<=1000:refuse('evidence-format')
            cells=[r['cell'] for r in context['inputRevisions']];step(len(cells))
            if len(set(cells))!=len(cells):refuse('evidence-identity')
        original=claim['original']['text'].encode();frame(len(original))
        if len(original)>65536:refuse('evidence-limit')
        if pin(original)!=claim['original']['digest']:refuse('evidence-integrity')
        core=canonical(claim_core(doc,row));frame(len(b'choreoform-recorded-claim:0.1.0\0')+len(core))
        key=(doc['namespace'],sources[row['source']]['identity'],row['event'])
        if key in identities and identities[key]!=core:refuse('evidence-identity')
        identities[key]=core;node[identity]=('local',*key)
    external={}
    for identity,row in boundaries.items():
        step(3);key=(row['namespace'],row['source'],row['event'])
        if key in identities:refuse('evidence-reference')
        if key in external and external[key]!=row['digest']:refuse('evidence-identity')
        external[key]=row['digest'];node[identity]=('external',*key)
    adjacency={key:set() for key in node.values()};degree={key:0 for key in node.values()}
    for relation in doc['relations'].values():
        step(4)
        if relation['from'] not in node or relation['to'] not in node or relation['basis'] not in records:refuse('evidence-reference')
        a,b=node[relation['from']],node[relation['to']]
        if a==b:refuse('evidence-order')
        if relation['kind']!='asserted-disputes' and b not in adjacency[a]:adjacency[a].add(b);degree[b]+=1
    ready=[key for key,n in degree.items() if n==0];visited=0
    while ready:
        key=ready.pop();visited+=1;step()
        for target in adjacency[key]:
            step();degree[target]-=1
            if degree[target]==0:ready.append(target)
    if visited!=len(degree):refuse('evidence-order')
    return doc,dict(sources=len(sources),clocks=len(clocks),records=len(records),claims=len(identities),boundaries=len(boundaries),relations=len(doc['relations']))


def check_fixture_registry(inputs, header):
    """Exact developer fixture framing, not production path resolution or native admission."""
    schema=json.loads((ROOT/'schemas/authoring/resource-registry-0.1.schema.json').read_bytes())
    if not jsonschema.Draft202012Validator(schema).is_valid(header):refuse('evidence-registry')
    rows=header['resources']
    if len(rows)!=len(inputs) or len({(r['contract'],r['digest']) for r in rows})!=len(rows):refuse('evidence-registry')
    offset=0
    for row,framing in zip(inputs,rows):
        raw=(ROOT/row['path']).read_bytes()
        if pin(raw)!=row['digest']:refuse('evidence-resource')
        if framing!=dict(contract=row['id'],digest=row['digest'],start=offset,length=len(raw)):refuse('evidence-registry')
        offset+=len(raw)
    return offset


class RecordedEvidenceContract(unittest.TestCase):
    def test_publication_shapes_pins_and_exact_authored_results_without_native_reader_claim(self):
        manifest=json.loads((BASE/'vectors.json').read_bytes())
        adr=(ROOT/'docs/decisions/0098-recorded-evidence-typed-claims.md').read_text()
        for name in ['docs/evidence/recorded-v0.1.md','schemas/evidence/recorded-0.1.schema.json','schemas/evidence/report-0.1.schema.json','docs/evidence/recorded-0.1/vectors.json']:
            self.assertIn(pin((ROOT/name).read_bytes()),adr)
        for name,digest in manifest['rawPins'].items():self.assertEqual(pin((ROOT/name).read_bytes()),digest,name)
        for schema in [SCHEMA,REPORT]:jsonschema.Draft202012Validator.check_schema(schema)
        for name in manifest['positive']:
            doc,counts=check_reference_model((BASE/name).read_bytes());self.assertEqual(canonical(doc),canonical(json.loads((BASE/name).read_bytes())))
            if name=='example.json':
                expected=json.loads((BASE/'expected-report.json').read_bytes());jsonschema.validate(expected,REPORT)
                self.assertEqual(expected['integrity'],artifact_digest(doc));self.assertEqual(expected['counts'],counts)
                self.assertFalse(expected['authenticated']);self.assertFalse(expected['acceptedHistory']);self.assertFalse(expected['executable'])
                self.assertEqual({k:claim_digest(doc,r) for k,r in doc['records'].items()},json.loads((BASE/'expected-claim-digests.json').read_bytes()))
                self.assertEqual(claim_digest(doc,doc['records']['report']),claim_digest(doc,doc['records']['report_again']))
        original=json.loads((BASE/'example.json').read_bytes());renamed=json.loads((BASE/'clock-id-renamed.json').read_bytes());changed=json.loads((BASE/'clock-origin-changed.json').read_bytes())
        self.assertEqual(claim_digest(original,original['records']['report']),claim_digest(renamed,renamed['records']['report']))
        self.assertNotEqual(claim_digest(original,original['records']['report']),claim_digest(changed,changed['records']['report']))
        rows=json.loads((BASE/'resource-inputs.json').read_bytes());header=json.loads((BASE/'registry.json').read_bytes())
        self.assertEqual(len(rows),15)
        self.assertEqual(check_fixture_registry(rows,header),manifest['registryBytes'])
        extra=deepcopy(header);extra['resources'].append(deepcopy(extra['resources'][-1]))
        with self.assertRaisesRegex(Refusal,'^evidence-registry$'):check_fixture_registry(rows,extra)
        gap=deepcopy(header);gap['resources'][0]['start']=1
        with self.assertRaisesRegex(Refusal,'^evidence-registry$'):check_fixture_registry(rows,gap)
        authority=deepcopy(header);authority['authorized']=True
        with self.assertRaisesRegex(Refusal,'^evidence-registry$'):check_fixture_registry(rows,authority)
        for name in ['docs/evidence/recorded-v0.1.md','docs/evaluation/0098-recorded-evidence-contract-plan.md','docs/evaluation/0098-recorded-evidence-design.md']:check_links(ROOT/name)
    def test_reference_identity_order_refusals_and_cumulative_framing_with_incomparable_claims(self):
        manifest=json.loads((BASE/'vectors.json').read_bytes())
        for name,code in manifest['refusals'].items():
            if name=='untyped-value.json':continue # Only the existing native literal API establishes this value refusal below.
            with self.subTest(vector=name),self.assertRaises(Refusal) as error:check_reference_model((BASE/name).read_bytes())
            self.assertEqual(str(error.exception),code,name)
        doc=json.loads((BASE/'example.json').read_bytes())
        # No inferred time edge: reversing claimed times while preserving explicit order stays representable.
        doc['records']['decision']['claim']['reportedTime']['instant']='999';check_reference_model(json.dumps(doc).encode())
        small=json.loads((BASE/'empty.json').read_bytes());small['namespace']='N'*4096;small['sources']={'s':dict(identity='S'*4096,caption='Source')}
        claim=dict(kind='observation',subject='Test',context=None,reportedTime=None,payload=dict(type=dict(kind='boolean'),value=True),original=dict(text='',digest=pin(b'')))
        small['records']={f'r{i}':dict(source='s',event=f'event-{i}',claim=claim,delivery=dict(receipt=None,channel='Test',recorder=None,reportedTime=None)) for i in range(1024)}
        raw=json.dumps(small,separators=(',',':')).encode();self.assertLess(len(raw),MIB)
        with self.assertRaisesRegex(Refusal,'^evidence-limit$'):check_reference_model(raw)
    def test_actual_existing_native_payload_api_and_unchanged_benchmarks(self):
        before=fixed();env=os.environ.copy()
        if (ROOT/'.tools/cargo/bin/cargo').is_file():
            env.update(CARGO_HOME=str(ROOT/'.tools/cargo'),RUSTUP_HOME=str(ROOT/'.tools/rustup'));env['PATH']=str(ROOT/'.tools/cargo/bin')+os.pathsep+env['PATH']
        result=subprocess.run(['cargo','run','--quiet','-p','choreoform-semantic-check','--example','recorded_payload_baseline','--locked'],cwd=ROOT,env=env,capture_output=True,timeout=60)
        self.assertEqual(result.returncode,0,result.stderr)
        self.assertEqual(result.stdout,b'Existing native payload literals: 5 valid, 2 refused; no whole evidence reader, authentication or accepted event.\nExisting native raw transport: 8 malformed inputs refused; no artifact admission.\n')
        self.assertEqual(fixed(),before)
        inventory=''.join(name+'\0'+digest+'\n' for name,digest in sorted(before.items()))
        self.assertEqual((len(before),sha256(inventory.encode()).hexdigest()),(965,'a25c6de850fdaa51436023fffe011c8f96695e9d8660191e7598cd4f4bf2e9f7'))


if __name__=='__main__':unittest.main()
