# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Build the synthetic RP-01 draft package; never execute or admit a process."""
import argparse
from hashlib import sha256
import json
import re
from pathlib import Path
import subprocess
import tempfile

import rfc8785

from authoring_probe import lex, validate_ledger, recognize
from benchmarks.rp01_graph import build, endpoint_probe
from benchmarks.rp01_invalidation import declare_links
from benchmarks.rp01_faults import adopt, PROFILE as FAULT_PROFILE
from benchmarks.rp01_capture import adopt as adopt_capture, PROFILE as TERMINAL_PROFILE
from benchmarks.rp01_resources import types, providers, clock_resources, encoded, pin

ROOT = Path(__file__).resolve().parents[1]
DESTINATION = ROOT / 'docs/benchmarks/rp01'
LICENSE = b'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n'


HISTORICAL_STAGES = ['check', 'check-control', 'check-protection', 'check-invalidation',
          'check-concurrency', 'check-timing', 'check-fault-routes',
          'check-exceptional-availability', 'check-reconciliation-links',
          'check-reconciliation-availability']

STAGES = [*HISTORICAL_STAGES[:4], 'invalidation-requirements', *HISTORICAL_STAGES[4:]]


def derive(files, registry, stages=STAGES):
    """Use the native frontend, retaining actual refusals without admission."""
    with tempfile.TemporaryDirectory() as directory:
        base = Path(directory)
        for name, raw in files.items():
            (base / name).parent.mkdir(parents=True, exist_ok=True)
            (base / name).write_bytes(raw)
        args = []
        for r in registry:
            name = r['path'].removeprefix('docs/benchmarks/rp01/')
            path = base / name if name in files else ROOT / r['path']
            args.extend(['--resource', r['id'], r['revision'], str(path)])
        binary = ROOT / 'target/debug/choreoform-authoring-frontend'
        def invoke(stage):
            return subprocess.run([str(binary), stage, str(base / 'source.choreo'), str(base / 'bindings.json'), *args], capture_output=True, timeout=30)
        lowered = invoke('lower')
        if lowered.returncode != 0:
            raise ValueError('whole-source lowering refused: ' + lowered.stderr.decode('utf-8'))
        candidate = json.loads(lowered.stdout)
        # Stored canonical bytes are the actual IR JCS encoding, without LF.
        files['candidate.ir.json'] = rfc8785.dumps(candidate)
        results = []
        for stage in stages:
            result = invoke(stage)
            if result.returncode == 0:
                report = json.loads(result.stdout)
                if report['executable'] is not False or report['revision'] != candidate['revision']:
                    raise ValueError('unexpected stage authority/revision')
                path = 'reports/' + stage + '.json'
                files[path] = encoded(report)
                results.append({'stage': stage, 'result': 'passed', 'report': path,
                                'reportDigest': 'sha256:' + sha256(files[path]).hexdigest()})
            else:
                refusal = result.stderr.decode('utf-8').strip()
                match = re.fullmatch(r'([a-z0-9-]+) at ([^;]*)(?:; source bytes (\d+)\.\.(\d+))?', refusal)
                if result.stdout or result.returncode != 1 or not match:
                    raise ValueError('unexpected stage failure: ' + refusal)
                results.append({'stage': stage, 'result': 'refused', 'code': match[1],
                                'path': match[2], 'sourceBytes': [int(match[3]), int(match[4])] if match[3] else None})
        return candidate, results


def historical_artifacts():
    files = {}
    _, type_raw, type_pin = types()
    resources = providers(type_pin, shared=True) | clock_resources(shared=True)
    names = {'checks': 'checks.provider.json', 'payment': 'payment.provider.json',
             'notice': 'notice.provider.json', 'clock-source': 'clock.provider.json', 'clock': 'clock.json'}
    files['types.json'] = type_raw
    for name, (_, raw, _) in resources.items():
        files[names[name]] = raw
    source, package = build(type_pin, {k: resources[k][0] for k in ['checks', 'payment', 'notice']},
        {k: v[2] for k, v in resources.items()}, profile="0.6.0", payment_policy_revisions=True)
    source = declare_links(source, package.ledger)
    source, package.ledger, fault_raw, fault_pin = adopt(source, package.ledger, resources["payment"][2])
    files["payment.fault-source.json"] = fault_raw
    # Counting individual complete lines uses the unchanged bounded lexer only
    # for measurement. It does not recognize/admit the over-budget whole program.
    tokens = sum(len(lex(line)) for line in source.splitlines())
    validate_ledger(package.ledger)
    files['source.choreo'] = source.encode('utf-8')
    files['bindings.json'] = package.companion()
    policy = {
        'kind': 'synthetic-rp01-policy-data', 'version': '0.1.0',
        'currencies': ['EUR', 'USD'], 'categories': ['travel', 'supplies'],
        'amountScale': 2, 'ordinaryAmountLimit': '100000',
        'missingReceipt': 'requires-finance-reasoned-review',
        'duplicate': 'requires-attributable-human-resolution',
        'meaning': 'Fictional benchmark inputs, not organizational policy or financial guidance.',
    }
    files['policy.json'] = encoded(policy)
    files['receipt.txt'] = b'Fictional receipt evidence for RP-01. No real purchase or reimbursement is represented.\n'
    claim = {
        'claim ID': 'rp01-fictional-claim', 'revision': 'claim-v1',
        'policy revision': pin('policy', files['policy.json'])['revision'],
        'employee': 'fictional-employee', 'date': '2026-11-06', 'amount': '12345',
        'currency': 'EUR', 'category': 'travel', 'business purpose': 'Fictional corpus meeting',
        'cost allocation': 'fictional-project',
        'receipt': {'tag': 'provided', 'value': {
            'identity': 'urn:choreoform:benchmarks:rp01:receipt', 'revision': 'receipt-v1',
            'digest': pin('receipt', files['receipt.txt'])['revision'], 'provenance': 'synthetic fixture',
        }},
    }
    files['claim-v1.json'] = encoded(claim)
    corrected = dict(claim, revision='claim-v2', amount='12000')
    files['claim-v2.json'] = encoded(corrected)
    files['launch-values.json'] = encoded({'claim': claim, 'deadline': '10000000'})
    files['correction-value.json'] = encoded({'tag': 'resubmitted', 'value': corrected})
    registry = [{'id': type_pin['id'], 'revision': type_pin['revision'], 'path': 'docs/benchmarks/rp01/types.json'}]
    registry += [{'id': p['id'], 'revision': p['revision'], 'path': 'docs/benchmarks/rp01/' + names[k]} for k, (_, _, p) in resources.items()]
    registry.append({'id': resources['clock'][0]['profile']['id'], 'revision': resources['clock'][0]['profile']['revision'],
        'path': 'docs/ir/contracts/sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt'})
    registry += [{'id':fault_pin['id'],'revision':fault_pin['revision'],'path':'docs/benchmarks/rp01/payment.fault-source.json'}, {'id':FAULT_PROFILE['id'],'revision':FAULT_PROFILE['revision'],'path':'docs/ir/contracts/'+FAULT_PROFILE['revision'].replace(':','-')+'.txt'}]
    canonical_bindings = rfc8785.dumps(package.ledger)
    source_raw = files['source.choreo']
    frame = b'choreoform-authoring-package:0.7.0\0' + len(source_raw).to_bytes(8, 'big') + source_raw + len(canonical_bindings).to_bytes(8, 'big') + canonical_bindings
    requirements = [
        'authenticated-current-principal-role-relationship-and-assignment-binding',
        'exact-input-policy-claim-and-evidence-revisions-before-acceptance',
        'truthful-implemented-check-payment-notice-and-clock-contracts',
        'confirmed-no-effect-fault-classification-before-eligible-retry',
        'exact-original-request-digest-key-and-registered-clock-domain',
        'authenticated-terminal-provider-fact-envelope-with-exact-obligation-correlation',
        'accepted-authorized-atomic-surviving-ownership-and-subscription-handoff',
        'transitive-lifetime-and-all-obligation-settlement-before-terminal-disposal',
        'atomic-cancellation-order-and-provider-irrevocable-cutoff',
        'history-retention-late-duplicate-and-contradictory-observation-handling',
    ]
    manifest = {
        'format': 'choreoform-rp01-benchmark-draft', 'version': '0.1.0',
        'status': 'whole-source-draft-awaiting-measured-profile-and-checks', 'executable': False,
        'sourceProfile': '0.7.0', 'companionVersion': '0.6.0', 'coreProfile': '0.4.0',
        'sourceBytes': len(source_raw), 'measuredTokens': tokens,
        'tokenMeasurement': 'sum of complete-line lexing; not whole-program recognition',
        'sourceDigest': 'sha256:' + sha256(source_raw).hexdigest(),
        'bindingsDigest': 'sha256:' + sha256(canonical_bindings).hexdigest(),
        'packageDigest': 'sha256:' + sha256(frame).hexdigest(),
        'canonicalIR': None, 'semanticStageResults': [],
        'resources': registry, 'contracts': package.ledger['contracts'],
        'inputArtifacts': [{'path': name, 'digest': 'sha256:' + sha256(files[name]).hexdigest()} for name in ['policy.json', 'receipt.txt', 'claim-v1.json', 'claim-v2.json', 'launch-values.json', 'correction-value.json']],
        'hostRequirements': requirements,
        'scenarios': [
            {'id': 'RP-01-A', 'status': 'Partial', 'declarations': ['review_again', 'run_checks', 'manager_review', 'finance_review', 'run_payment', 'record_paid', 'run_notice', 'finish_paid']},
            {'id': 'RP-01-B', 'status': 'Partial', 'declarations': ['finance_review', 'correct_claim', 'publish_correction', 'review_again', 'may_pay'], 'inputs': ['claim-v1.json', 'correction-value.json']},
            {'id': 'RP-01-C', 'status': 'Partial', 'declarations': ['payment_request_step', 'payment_work', 'payment_close', 'payment_wait', 'payment_observe']},
            {'id': 'RP-01-D', 'status': 'Partial', 'declarations': ['withdraw_request', 'manager_review', 'payment_commits', 'payment_request_step']},
        ],
    }
    candidate, stage_results = derive(files, registry, HISTORICAL_STAGES)
    manifest.update(status='lowered-candidate-with-partial-static-evidence',
        canonicalIR={'path': 'candidate.ir.json', 'revision': candidate['revision'],
                     'digest': 'sha256:' + sha256(files['candidate.ir.json']).hexdigest()},
        semanticStageResults=stage_results)
    files['manifest.json'] = encoded(manifest)
    files['README.md'] = f'''<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-01 package draft

This generated synthetic whole-source draft lowers to **candidate IR** but is
**not semantically admitted or executable**. The manifest records all ten actual
semantic-stage results/refusals and the exact canonical IR digest. It follows
the [frozen plan](../../evaluation/0034-rp01-package-plan.md) and preserves all four
scenarios from the [adopted case](../../representative-processes/01-expense-reimbursement.md).

The source has {len(source_raw)} UTF-8 bytes and {tokens} measured tokens. The
measurement lexes complete lines independently and is not whole-program admission.
Source 0.7 / companion 0.6 and core 0.4 are selected explicitly under
ADR-0042/0043; old cores/source profiles remain unchanged. Coherent fictional
business/clock recipient labels pass static, ordinary control and protection
checks. The exact 72 missing ordinary invalidation links derived under ADR-0038
are now declared; strict invalidation also passes. All ten existing partial
static stages now pass. Concurrency and later views disclose three isolated
surviving follow-up declarations under ADR-0039, with outstanding live ownership,
correlation, atomic handoff and terminal-settlement premises. These reports prove
no actual lifetime, progress, general race safety or whole-process admission.
Closed private influence is handled only by the bounded ADR-0037 proof subset,
retaining all mutable ancestor dependencies. This fixture patch preserves all
existing invalidation links and changes no other semantic body fields. The
ADR-0039 checker refinement changes no source, IR revision or resource bytes.
The ADR-0040 finite-snapshot correction explicitly rechecks both approval policy
revisions and the current claim policy revision at the payment commit guard;
it preserves the six earlier clauses. This changes the current candidate's
semantic revision, while historical profile fixtures keep their original guard.
The [draft host contract](host-contract.md) specifies the remaining authority,
provider normalization, retry classification, correlation and lifetime obligations.

Build the pinned native frontend first (`cargo build -p choreoform-authoring-frontend
--locked`). Regenerate with `.tools/ir-check/bin/python tools/build_rp01_benchmark.py`
at the repository root; use `--check` for exact file comparison. Stable IDs are supplied
explicitly by the fixture assembler, independent of readable names. This is not
a general IR exporter, ordinary authoring editor or automatic source migration.

The first variant runs policy/duplicate checks sequentially and omits optional
business-calendar reminders. Payment timeout/retry use an explicit instance clock
and input deadline, with no implicit current time. A named claim invariant checks
selected fictional constraints, not dates, receipt authenticity or policy truth.
Human result variants retain attributable reasons and current revision checks;
actual identity/assignment authentication remains a host requirement.
`launch-values.json` supplies only synthetic port values, and `correction-value.json`
is only a typed proposed resubmission value. Neither is an accepted observation,
authenticated completion or running instance. Claim v2 is correction data for the
same claim, not a replacement definition or a second payable request.

Check findings, payment command business results, unknown effects and faults are
distinct. A successful payment-command `problem` result means a confirmed terminal
business problem; it does not turn an unknown debit into a rejection. Reconciliation
uses an explicit terminal-fact envelope (confirmed result or correlated no-effect/
rejected/cancelled evidence), whose supported authenticated normalization is still
a required host boundary. A true wait predicate alone grants no correlation or
ownership. `payment.fault-source.json` explicitly pins the unchanged payment
provider and maps confirmed no effect to declared `no-effect` evidence; provider
unknown maps to unresolved and remains noneligible. All five reports from timing
onward disclose exact retry bindings and live requirements. Both names contribute
to abstract unhandled-name accounting; these are declared possibilities, not
actual faults. The exact `confirmed no effect` retry still requires truthful
authenticated classification; it is not inferred from HTTP errors or spelling.

Each service occurrence owns one capability site and its declared follow-up.
Root data names root-visible capabilities; source 0.7 ancestor wait policies name
their actual activity-use follow-ups. Older profile 0.4 keeps its narrower binding. The remaining-owner finish is a P6
closure gate, not proof that transitive obligations vanished. Full lifetime,
retention, invalidation/conflict/exceptional protection and accepted handoff remain
open until actual checks and any necessary contract work establish them.

Broadly authorized fictional expense participants share the declared expense
purpose/envelope; request payloads still separate claim, approval, payment and
notice information. This is not a demonstrated least-privilege editor or protected
audience view. All three business adapters and the clock-result source explicitly
permit that same downstream recipient circle; data/pin equality still grants no
current authority, truthful observation or least-privilege guarantee. Policy and receipt files are fictional input artifacts, not trusted
execution contracts or real operational evidence. All four scenarios and the
forty-corpus accounting remain Partial. No user, host/browser/process execution,
independent review or Phase 1 closure is claimed.
'''.encode('utf-8')
    for name in list(files):
        if name.endswith(('.json', '.txt')):
            files[name + '.license'] = LICENSE
    return files


def artifacts():
    historical = historical_artifacts()
    assert len(historical['source.choreo']) == 56874
    assert json.loads(historical['candidate.ir.json'])['revision'] == 'sha256:97e5c346442d5b051e40bc91d2c3014be1bd990688324be846739c5d35a265c4'
    assert sha256(historical['candidate.ir.json']).hexdigest() == 'c012c1493ad7b04abc5e5f81529eea6231b720a54ee6a8db867fd2e207fd546a'
    assert sha256(historical['manifest.json']).hexdigest() == 'fd1ca83a022d7ccd8e3f21cf85305d662725e2ac5172cd201615ded0c7471ffb'
    files = dict(historical)
    manifest = json.loads(historical['manifest.json'])
    body = json.loads(historical['candidate.ir.json'])['body']
    provider = next({'id': r['id'], 'revision': r['revision']} for r in manifest['resources'] if r['path'].endswith('/payment.provider.json'))
    source, ledger, raw, source_pin = adopt_capture(historical['source.choreo'].decode(), json.loads(historical['bindings.json']), body, provider)
    files.update({'source.choreo': source.encode(), 'bindings.json': encoded(ledger), 'payment.terminal-source.json': raw, 'payment.terminal-source.json.license': LICENSE})
    registry = manifest['resources']
    registry += [{'id': source_pin['id'], 'revision': source_pin['revision'], 'path': 'docs/benchmarks/rp01/payment.terminal-source.json'},
                 {'id': TERMINAL_PROFILE['id'], 'revision': TERMINAL_PROFILE['revision'], 'path': 'docs/ir/contracts/'+TERMINAL_PROFILE['revision'].replace(':','-')+'.txt'}]
    candidate, results = derive(files, registry)
    canonical_bindings = rfc8785.dumps(ledger)
    source_raw = files['source.choreo']
    frame = b'choreoform-authoring-package:0.14.0\0' + len(source_raw).to_bytes(8, 'big') + source_raw + len(canonical_bindings).to_bytes(8, 'big') + canonical_bindings
    manifest.update(sourceProfile='0.14.0', companionVersion='0.13.0', coreProfile='0.11.0',
        sourceBytes=len(source_raw), measuredTokens=len(lex(source)),
        tokenMeasurement='whole-source bounded lexing; not semantic admission',
        sourceDigest='sha256:'+sha256(source_raw).hexdigest(), bindingsDigest='sha256:'+sha256(canonical_bindings).hexdigest(),
        packageDigest='sha256:'+sha256(frame).hexdigest(), contracts=ledger['contracts'],
        canonicalIR={'path': 'candidate.ir.json', 'revision': candidate['revision'], 'digest': 'sha256:'+sha256(files['candidate.ir.json']).hexdigest()},
        semanticStageResults=results)
    manifest['hostRequirements'] += ['immutable-original-request-and-terminal-capture-lifetime', 'receipt-matching-and-accountable-contradictory-evidence-review']
    manifest['scenarios'][2]['declarations'] += ['retained_payment_request', 'late_payment_capture', 'classify_late_payment', 'late_payment_category']
    files['manifest.json'] = encoded(manifest)
    files['reports/invalidation-requirements.json.license'] = LICENSE
    files['README.md'] = current_readme(manifest).encode()
    for name, raw in historical.items():
        if name == 'README.md': continue
        files['history/source-0.7/'+name] = raw
    files['history/source-0.7/README.md'] = b"""<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Historical source-0.7 RP-01 checkpoint

Exact generated historical source/companion/IR/resource/input/ten-report evidence
from a92e472, preserved under ADR-0066. This is source 0.7/core 0.4/IR 0.1,
not the current checkpoint. The original manifest and all resource/report digests
stay fixed. Its repository-relative resource paths resolve the identical original
artifacts at the package root. Tests explicitly select this history for old profile
semantics. See [current package](../../README.md) for current evidence and open gates.
No accepted event, host execution, scenario or phase closure is claimed.
"""
    return files


def current_readme(manifest):
    return f'''<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-01 typed late-capture checkpoint

This synthetic whole-source package is **candidate IR**, **not semantically admitted
or executable**. Source 0.14/companion 0.13 explicitly selects existing core 0.11 /
IR 0.4 under [ADR-0066](../../decisions/0066-rp01-capture-adoption.md) and its
[frozen plan](../../evaluation/0066-rp01-capture-adoption-plan.md). The manifest
records eleven actual checking/advisory views and their report digests. All four
[original scenarios](../../representative-processes/01-expense-reimbursement.md)
remain Partial. No provider, host/browser/user or independent execution evidence.

Source is {manifest['sourceBytes']} UTF-8 bytes / {manifest['measuredTokens']} measured
tokens. Revision: `{manifest['canonicalIR']['revision']}`. The exact generated
[historical checkpoint](history/source-0.7/README.md) retains source 0.7/core 0.4 /
IR 0.1 and its original ten-view evidence. Original provider/type/clock/policy/input /
fault-source bytes and ordinary guards are unchanged; current candidate identity
and report/package digests are explicitly regenerated. All 44 guard cases / 46
values / expectations are preserved with only their candidate binding repinned.

The payment wait now captures an exact typed terminal envelope in its local recovery
body. The new terminal-source descriptor pins the unchanged payment provider and
accepted normalization profile. The exact accepted original request is a separate
readonly seed. The local classifier reuses the original payment completion predicate,
substituting only captured confirmed payload and retained original request; it checks
claim identity, revision, amount and currency. A paid tag alone is insufficient.
Mismatches record mismatched-evidence and preserve the capture for accountable
conflict review. Definitive noeffect/rejected/cancelled remain distinct; unknown /
acknowledged stay pending. Neither matched receipt nor no-effect text grants retry
or a new dispatch; new business work is not a technical retry.

The new capture/request/category keep the same Expense access/purpose/participant /
capability envelope. Ordinary ports and ancestor current_claim dependencies remain.
Reports disclose separate conditional flat owned proof and retained-request scope;
legacy reconciliation coverage leaves the retained-seed body uncovered. The other
two surviving declarations remain isolated. No value is copied from a live ancestor
into recovery, no ordinary parent result is written/resumed, and cancellation is
never cleared. Local finish/mismatch text does not settle a conflict. Required
protection provenance contains labels, not live storage or authority.

All actual provider truth/normalization/correlation/current authority/immutable
request and capture storage/P6 cutoff/receiver acceptance/atomic handoff/transitive
settlement requirements in the [host contract](host-contract.md) remain open.
The stable-key confirmed-no-effect fault-source retry and ordinary approval/policy /
withdrawal guards are preserved. Fictional input ports are not accepted observations;
broadly shared fictional recipients do not demonstrate least privilege or protected
accessible editing. Sequential checks and omitted optional business-calendar reminders
remain the same first variant. General tool-managed near-English/text-visual editing,
complete validator/RP-01/03/08 and all Phase 1 exit gates remain open.

Build the pinned native frontend, then regenerate with
`.tools/ir-check/bin/python tools/build_rp01_benchmark.py`; `--check` compares exact
bytes including the historical checkpoint. The controlled assembler has explicit
stable IDs and is not a general IR exporter, editor or automatic migration.
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--probe-profile', action='store_true', help='Reproduce historical source 0.4 refusals; does not admit the source')
    args = parser.parse_args()
    files = artifacts()
    if args.check:
        changed = [name for name, raw in files.items() if not (DESTINATION / name).is_file() or (DESTINATION / name).read_bytes() != raw]
        if changed:
            raise SystemExit('generated RP-01 files differ: ' + ', '.join(changed))
    else:
        DESTINATION.mkdir(parents=True, exist_ok=True)
        for name, raw in files.items():
            (DESTINATION / name).parent.mkdir(parents=True, exist_ok=True)
            (DESTINATION / name).write_bytes(raw)
    manifest = json.loads(files['manifest.json'])
    print(f'RP-01 draft: {manifest["sourceBytes"]} bytes, {manifest["measuredTokens"]} measured tokens; no admission claimed.')
    if args.probe_profile:
        # The refusal witness selects the exact historical source/profile pair.
        files = historical_artifacts()
        manifest = json.loads(files['manifest.json'])
        args_for_resources = [item for r in manifest['resources'] for item in ['--resource', r['id'], r['revision'], str(ROOT / r['path'])]]
        binary = ROOT / 'target/debug/choreoform-authoring-frontend'
        with tempfile.TemporaryDirectory() as directory:
            source, bindings = Path(directory) / 'old.choreo', Path(directory) / 'old.json'
            source.write_text(files['source.choreo'].decode('utf-8').replace('"0.7.0"', '"0.4.0"', 1), encoding='utf-8')
            ledger = json.loads(files['bindings.json'])
            ledger.update(profile='0.4.0', version='0.3.0')
            bindings.write_bytes(encoded(ledger))
            strict = subprocess.run([str(binary), 'lower', str(source), str(bindings), *args_for_resources], capture_output=True, timeout=20)
        if strict.returncode == 0 or b'tokens' not in strict.stderr or strict.stdout:
            raise SystemExit('expected the current whole-program token refusal, got: ' + strict.stderr.decode('utf-8', errors='replace'))
        _, _, type_pin = types()
        provider, _, provider_pin = providers(type_pin)['checks']
        probe, package = endpoint_probe(type_pin, provider, provider_pin)
        recognize(probe)
        validate_ledger(package.ledger)
        with tempfile.TemporaryDirectory() as directory:
            source, bindings = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
            source.write_text(probe, encoding='utf-8')
            bindings.write_bytes(package.companion())
            provider_path = Path(directory) / 'checks.provider.json'
            provider_path.write_bytes(encoded(provider))
            probe_resources = [item for r in manifest['resources'] if r['id'] != provider_pin['id'] for item in ['--resource', r['id'], r['revision'], str(ROOT / r['path'])]]
            probe_resources += ['--resource', provider_pin['id'], provider_pin['revision'], str(provider_path)]
            result = subprocess.run([str(binary), 'lower', str(source), str(bindings), *probe_resources], capture_output=True, timeout=20)
        if result.returncode == 0 or b'reference-missing' not in result.stderr or result.stdout:
            raise SystemExit('expected the ancestor-policy endpoint binding refusal, got: ' + result.stderr.decode('utf-8', errors='replace'))
        print('Strict whole draft:', strict.stderr.decode('utf-8').strip())
        print(f'Syntax/schema-valid isolated endpoint witness ({len(lex(probe))} tokens):', result.stderr.decode('utf-8').strip())


if __name__ == '__main__':
    main()
