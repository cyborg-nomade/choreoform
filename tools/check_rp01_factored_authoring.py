# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Exact RP-01 epoch and explicit core/revision/report delta; no host execution."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest
from check_rp01_capture_checkpoint import BASE,ROOT,package
from check_metadata_authoring import invoke
from check_authoring_evidence import check_links
from check_rp03_factored_authoring import mutate
from benchmarks.rp01_factoring_checkpoint import artifacts as archive,BASE as OLD,PATHS,original_tree
from benchmarks.rp01_factored_source import Scope
from benchmarks.rp01_source import Package
from benchmarks.rp01_graph import PEOPLE,CAPABILITIES
from benchmarks.rp01_resources import record,TEXT
from build_rp01_benchmark import capture_artifacts,STAGES

OLD_REV='sha256:55413bc5be721c103f56bd61a30bbf829e3cc97b1f867157e554baec4470e285'
NEW_REV='sha256:cd0a7b6923ccc8a0afa31a27fca3aff280c3da601089ba12946f2d768fa1a829'


def differences(a,b,path=''):
    if type(a)!=type(b):return [(path,a,b)]
    if isinstance(a,dict):
        if a.keys()!=b.keys():return [(path+'/keys',list(a),list(b))]
        return [d for k in a for d in differences(a[k],b[k],path+'/'+k)]
    if isinstance(a,list):
        if len(a)!=len(b):return [(path+'/length',len(a),len(b))]
        return [d for i,(x,y) in enumerate(zip(a,b)) for d in differences(x,y,path+'/'+str(i))]
    return [] if a==b else [(path,a,b)]


def historical_package():
    m=json.loads((OLD/'manifest.json').read_bytes())
    return ((OLD/'source.choreo').read_text(),json.loads((OLD/'bindings.json').read_bytes()),
        [(r['id'],r['revision'],(OLD/r['path'].removeprefix('docs/benchmarks/rp01/')).read_bytes() if r['path'].startswith('docs/benchmarks/rp01/') else (ROOT/r['path']).read_bytes()) for r in m['resources']])


class FactoredExpenseAuthoring(unittest.TestCase):
    def test_exact_old_tree_allowed_current_files_all_other_benchmarks_and_legacy_producer(self):
        old=archive();self.assertEqual(len(old),60);self.assertEqual(len(original_tree()),113)
        allowed={'README.md','source.choreo','bindings.json','candidate.ir.json','manifest.json'}|{'reports/'+s+'.json' for s in STAGES}
        changed={name for name in PATHS if (BASE/name).read_bytes()!=old[name]};self.assertEqual(changed,allowed)
        table=json.loads((ROOT/'docs/evaluation/0088-rp01-adoption-digests.json').read_bytes())
        self.assertEqual(set(table),allowed)
        for name,pins in table.items():self.assertEqual(pins,{'old':sha256(old[name]).hexdigest(),'new':sha256((BASE/name).read_bytes()).hexdigest()})
        for name,raw in capture_artifacts().items():self.assertEqual((OLD/name).read_bytes() if not name.startswith('history/') else (BASE/name).read_bytes(),raw,name)
        # Whole adopted RP-03, including every prior archive, is immutable here.
        other=ROOT/'docs/benchmarks/rp03';paths=sorted(p for p in other.rglob('*') if p.is_file())
        rows=''.join(p.relative_to(other).as_posix()+'\0'+sha256(p.read_bytes()).hexdigest()+'\n' for p in paths)
        self.assertEqual((len(paths),sha256(rows.encode()).hexdigest()),(605,'c68b965e389f3dc36e731b4a70448d23e4f4d580d9b9141199cdc1617d3a536c'))
        for path in ['docs/decisions/0088-rp01-factored-authoring.md','docs/evaluation/0088-rp01-factored-authoring.md','docs/benchmarks/rp01/README.md']:check_links(ROOT/path)

    def test_actual_old_new_lowering_changes_only_explicit_core_and_revision_and_report_attribution(self):
        old=invoke(historical_package());new=invoke(package())
        self.assertEqual((old.returncode,new.returncode),(0,0),(old.stderr,new.stderr))
        a,b=json.loads(old.stdout),json.loads(new.stdout)
        self.assertEqual((a['revision'],b['revision']),(OLD_REV,NEW_REV))
        self.assertEqual(set(d[0] for d in differences(a,b)),{'/body/dialects/core/id','/body/dialects/core/revision','/revision'})
        clone=deepcopy(b);clone['body']['dialects']['core']=a['body']['dialects']['core'];clone['revision']=a['revision'];self.assertEqual(clone,a)
        for index,stage in enumerate(STAGES):
            old_report=json.loads((OLD/('reports/'+stage+'.json')).read_bytes())
            report=json.loads((BASE/('reports/'+stage+'.json')).read_bytes())
            expected={'/revision','/declaredFaultSourceUses/0/revision','/declaredRetainedRequests/0/revision','/declaredWaitCaptures/0/revision'}
            if index>=5:expected.add('/conditionalOwnedLocalChecks/revision')
            if index>=6:expected.add('/retryFaultBindings/0/revision')
            delta=differences(old_report,report);self.assertEqual({d[0] for d in delta},expected,stage)
            self.assertTrue(all((x,y)==(OLD_REV,NEW_REV) for _,x,y in delta),stage)
            self.assertIs(report['executable'],False)
        # Per-data purpose IDs and every old source identity remain exact.
        l1,l2=historical_package()[1],package()[1]
        self.assertEqual([d for d in l2['declarations'] if d['id'] not in {'expense_data_protection','payable_instruction_type'}],l1['declarations'])
        self.assertEqual([d for d in l2['symbols'] if d['owner']!='expense_data_protection'],l1['symbols'])
        self.assertEqual(l2['generated'],l1['generated']);self.assertEqual(l2['resources'],l1['resources'])

    def test_real_current_mutations_refuse_and_finite_definition_uses_snapshot_and_complete_envelope(self):
        source,ledger,resources=package()
        for old,new,code in [('Purposes ["Reimbursement"]','Purposes empty',b'data-purposes'),('Describe type "Payable instruction"','Describe type "SECRET-MUST-NOT-LEAK"',b'binding-mismatch')]:
            bad=mutate(source,old,new);r=invoke((bad,ledger,resources),'check')
            self.assertNotEqual(r.returncode,0);self.assertEqual(r.stdout,b'');self.assertIn(code,r.stderr);self.assertNotIn(b'SECRET-MUST-NOT-LEAK',r.stderr)
        bad=mutate(source,'Describe type "Payable instruction" as record with ["claim" as type "Claim" from "Expense types"','Describe type "Payable instruction" as record with ["claim" as text')
        r=invoke((bad,ledger,resources),'check');self.assertNotEqual(r.returncode,0);self.assertEqual(r.stdout,b'')
        ty=record(message=deepcopy(TEXT));p=Package();root=Scope(p,'expense','Expense',None,'Done');root.define(PEOPLE,CAPABILITIES,ty)
        before=deepcopy(p.ledger)
        with self.assertRaises(ValueError):root.data('wrong','Wrong',TEXT,participants=PEOPLE,capabilities=CAPABILITIES[:-1])
        with self.assertRaises(ValueError):root.data('duplicate','Duplicate',TEXT,participants=PEOPLE+PEOPLE[:1],capabilities=CAPABILITIES)
        self.assertEqual(p.ledger,before)
        ty['fields']['message']={'kind':'integer'};self.assertEqual(p.factoring.instruction,record(message=TEXT))
        self.assertNotEqual(p.factoring.type_text(ty),'type "Payable instruction"')


if __name__=='__main__':unittest.main()
