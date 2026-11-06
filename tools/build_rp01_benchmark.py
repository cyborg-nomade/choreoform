# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Build the synthetic RP-01 draft package; never execute or admit a process."""
import argparse
from hashlib import sha256
import json
from pathlib import Path
import subprocess
import tempfile

import rfc8785

from authoring_probe import lex, validate_ledger, recognize
from benchmarks.rp01_graph import build, endpoint_probe
from benchmarks.rp01_resources import types, providers, clock_resources, encoded, pin

ROOT = Path(__file__).resolve().parents[1]
DESTINATION = ROOT / 'docs/benchmarks/rp01'
LICENSE = b'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n'


def artifacts():
    files = {}
    _, type_raw, type_pin = types()
    resources = providers(type_pin) | clock_resources()
    names = {'checks': 'checks.provider.json', 'payment': 'payment.provider.json',
             'notice': 'notice.provider.json', 'clock-source': 'clock.provider.json', 'clock': 'clock.json'}
    files['types.json'] = type_raw
    for name, (_, raw, _) in resources.items():
        files[names[name]] = raw
    source, package = build(type_pin, {k: resources[k][0] for k in ['checks', 'payment', 'notice']},
        {k: v[2] for k, v in resources.items()})
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
    canonical_bindings = rfc8785.dumps(package.ledger)
    source_raw = files['source.choreo']
    frame = b'choreoform-authoring-package:0.4.0\0' + len(source_raw).to_bytes(8, 'big') + source_raw + len(canonical_bindings).to_bytes(8, 'big') + canonical_bindings
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
        'sourceProfile': '0.4.0', 'companionVersion': '0.3.0', 'coreProfile': '0.2.0',
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
    files['manifest.json'] = encoded(manifest)
    files['README.md'] = f'''<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-01 package draft

This generated synthetic whole-source draft is **not lowered, admitted or
executable**. No canonical IR or semantic stage result is claimed yet. It follows
the [frozen plan](../../evaluation/0034-rp01-package-plan.md) and preserves all four
scenarios from the [adopted case](../../representative-processes/01-expense-reimbursement.md).

The source has {len(source_raw)} UTF-8 bytes and {tokens} measured tokens. The
measurement lexes complete lines independently and is not whole-program admission.
The unchanged source 0.4 lexer/parser limits and ancestor-policy endpoint binding
must be checked as actual dependencies before a supported profile is selected.
No quota or visibility rule has been silently changed to compile this draft.
The [draft host contract](host-contract.md) specifies the remaining authority,
provider normalization, retry classification, correlation and lifetime obligations.

Regenerate with `.tools/ir-check/bin/python tools/build_rp01_benchmark.py` at the
repository root; use `--check` for exact file comparison. Stable IDs are supplied
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
ownership. The exact `confirmed no effect` retry fault name requires truthful
authenticated classification; it is not inferred from HTTP errors or spelling.

Each service occurrence owns one capability site and its declared follow-up.
Root data names root-visible capabilities; ancestor wait policies name their
actual activity-use follow-ups. Profile 0.4's narrower policy-own-scope source
binding does not implement that relation. The remaining-owner finish is a P6
closure gate, not proof that transitive obligations vanished. Full lifetime,
retention, invalidation/conflict/exceptional protection and accepted handoff remain
open until actual checks and any necessary contract work establish them.

Broadly authorized fictional expense participants share the declared expense
purpose/envelope; request payloads still separate claim, approval, payment and
notice information. This is not a demonstrated least-privilege editor or protected
audience view. Policy and receipt files are fictional input artifacts, not trusted
execution contracts or real operational evidence. All four scenarios and the
forty-corpus accounting remain Partial. No user, host/browser/process execution,
independent review or Phase 1 closure is claimed.
'''.encode('utf-8')
    for name in list(files):
        if name.endswith(('.json', '.txt')):
            files[name + '.license'] = LICENSE
    return files


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--probe-profile', action='store_true', help='Reproduce strict native draft/profile refusals; does not admit the source')
    args = parser.parse_args()
    files = artifacts()
    if args.check:
        changed = [name for name, raw in files.items() if not (DESTINATION / name).is_file() or (DESTINATION / name).read_bytes() != raw]
        if changed:
            raise SystemExit('generated RP-01 files differ: ' + ', '.join(changed))
    else:
        DESTINATION.mkdir(parents=True, exist_ok=True)
        for name, raw in files.items():
            (DESTINATION / name).write_bytes(raw)
    manifest = json.loads(files['manifest.json'])
    print(f'RP-01 draft: {manifest["sourceBytes"]} bytes, {manifest["measuredTokens"]} measured tokens; no admission claimed.')
    if args.probe_profile:
        args_for_resources = [item for r in manifest['resources'] for item in ['--resource', r['id'], r['revision'], str(ROOT / r['path'])]]
        binary = ROOT / 'target/debug/choreoform-authoring-frontend'
        strict = subprocess.run([str(binary), 'lower', str(DESTINATION / 'source.choreo'), str(DESTINATION / 'bindings.json'), *args_for_resources], capture_output=True, timeout=20)
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
            result = subprocess.run([str(binary), 'lower', str(source), str(bindings), *args_for_resources], capture_output=True, timeout=20)
        if result.returncode == 0 or b'reference-missing' not in result.stderr or result.stdout:
            raise SystemExit('expected the ancestor-policy endpoint binding refusal, got: ' + result.stderr.decode('utf-8', errors='replace'))
        print('Strict whole draft:', strict.stderr.decode('utf-8').strip())
        print(f'Syntax/schema-valid isolated endpoint witness ({len(lex(probe))} tokens):', result.stderr.decode('utf-8').strip())


if __name__ == '__main__':
    main()
