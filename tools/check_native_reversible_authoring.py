# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Actual native document/operation CLI evidence; no UI, user or host proof."""
from hashlib import sha256
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

import jsonschema
from check_authoring_evidence import check_links
from check_metadata_authoring import invoke
from check_readonly_continuation_authoring import STAGES
from check_reversible_authoring_contract import BASE, admit, authoring
from check_rp01_capture_checkpoint import package as expense
from check_rp03_package import package as order
from check_source_capacity import fixed
from check_static_contracts import ROOT, BINARY


class Cli:
    def __init__(self, directory, resources=()):
        self.base = Path(directory)
        self.resources = []
        self.serial = 0
        for index, (identity, revision, raw) in enumerate(resources):
            path = self.base / f'resource-{index}'
            path.write_bytes(raw)
            self.resources += ['--resource', identity, revision, str(path)]

    def run(self, command, *artifacts):
        paths = []
        for raw in artifacts:
            path = self.base / f'artifact-{self.serial}'
            self.serial += 1
            path.write_bytes(raw)
            paths.append(str(path))
        return subprocess.run([str(BINARY), command, *paths, *self.resources], capture_output=True, timeout=30)

    def checked(self, command, *artifacts):
        result = self.run(command, *artifacts)
        if result.returncode:
            raise AssertionError((command, result.stderr))
        return result.stdout


def change(base, operation):
    return json.dumps(dict(format='choreoform-authoring-change', version='0.1.0', base=base, operation=operation), ensure_ascii=False).encode()


class NativeReversibleAuthoring(unittest.TestCase):
    def test_exact_preimplementation_bindings_links_and_closed_private_stage_schema(self):
        adr = (ROOT / 'docs/decisions/0095-native-reversible-authoring.md').read_text()
        for name in ['docs/authoring/native-reversible-v0.1.md', 'schemas/authoring/inspection-0.1.schema.json']:
            self.assertIn('sha256:' + sha256((ROOT / name).read_bytes()).hexdigest(), adr)
        for name in ['docs/decisions/0095-native-reversible-authoring.md', 'docs/authoring/native-reversible-v0.1.md', 'docs/evaluation/0095-native-reversible-authoring-plan.md', 'docs/evaluation/0095-native-reversible-authoring-design.md', 'docs/authoring/README.md', 'docs/decisions/README.md']:
            check_links(ROOT / name)
        schema = json.loads((ROOT / 'schemas/authoring/inspection-0.1.schema.json').read_bytes())
        jsonschema.Draft202012Validator.check_schema(schema)
        with tempfile.TemporaryDirectory() as directory:
            view = json.loads(Cli(directory).checked('document-inspect', (BASE / 'base.json').read_bytes()))
        jsonschema.validate(view, schema)
        for key, value in [('executable', True), ('status', 'validated'), ('permission', True), ('version', '0.2.0')]:
            invalid = dict(view); invalid[key] = value
            with self.assertRaises(jsonschema.ValidationError): jsonschema.validate(invalid, schema)
        self.assertIn('tools/check_native_reversible_authoring.py', (ROOT / '.github/workflows/rust-portability.yml').read_text())

    def test_actual_native_golden_changes_exports_and_private_atomic_cli_refusals(self):
        vectors = json.loads((BASE / 'vectors.json').read_bytes())
        raw = (BASE / 'base.json').read_bytes()
        state = admit(raw)
        with tempfile.TemporaryDirectory() as directory:
            cli = Cli(directory)
            exported = cli.checked('document-export', raw)
            self.assertEqual(json.loads(exported), state[0])
            self.assertFalse(exported.endswith(b'\n'))
            self.assertEqual(cli.checked('document-export', exported), exported)
            view = json.loads(cli.checked('document-inspect', raw))
            self.assertEqual(view['authoringDigest'], authoring(state[0])); self.assertIs(view['executable'], False)
            for pair in vectors['pairs']:
                actual = cli.checked('document-change', raw, (BASE / pair['request']).read_bytes())
                expected = cli.checked('document-export', (BASE / pair['after']).read_bytes())
                self.assertEqual(actual, expected, pair)
                data = json.loads(actual)
                imported = cli.checked('document-import', data['source'].encode(), data['bindings'].encode(), json.dumps(data['layout']).encode())
                self.assertEqual(imported, actual)
            for name, code in vectors['refusals'].items():
                if name.endswith('.change.json'): result = cli.run('document-change', raw, (BASE / name).read_bytes())
                else: result = cli.run('document-export', (BASE / name).read_bytes())
                self.assertEqual(result.returncode, 1, name); self.assertEqual(result.stdout, b'')
                self.assertTrue(result.stderr.startswith((code + ' at ').encode()), result.stderr)
            for invalid in [b'{"SECRET":0,"SECRET":1}', b'{"x":-0}', b'{"x":1.0}', b'"\\ud800"', b'[]', b'null', b'{} trailing']:
                result = cli.run('document-export', invalid)
                self.assertEqual(result.returncode, 1); self.assertEqual(result.stdout, b'')
                self.assertNotIn(b'SECRET', result.stderr)
            for operation in [dict(kind='rename', target='SECRET-MUST-NOT-LEAK', name='New name'), dict(kind='replace-layout', layout=dict(format='choreoform-authoring-layout', version='0.1.0', entries=[dict(entity='SECRET-MUST-NOT-LEAK', x=0, y=0, collapsed=False)]))]:
                result = cli.run('document-change', raw, change(view['authoringDigest'], operation))
                self.assertEqual((result.returncode, result.stdout), (1, b''))
                self.assertNotIn(b'SECRET-MUST-NOT-LEAK', result.stderr)
            result = subprocess.run([str(BINARY), 'document-export', str(Path(directory) / 'SECRET-MISSING')], capture_output=True, timeout=5)
            self.assertEqual((result.returncode, result.stdout, result.stderr), (1, b'', b'read\n'))
            result = cli.run('document-change', raw, b' ' * (1024 * 1024 + 1))
            self.assertEqual((result.returncode, result.stdout, result.stderr), (1, b'', b'authoring-size\n'))

    def test_whole_current_rp01_rp03_real_cycles_all_target_kinds_and_all_checking_views(self):
        before = fixed(); measurements = []
        for name, package in [('rp01', expense()), ('rp03', order())]:
            source, ledger, resources = package
            original = invoke(package); self.assertEqual(original.returncode, 0, original.stderr)
            with tempfile.TemporaryDirectory() as directory:
                cli = Cli(directory, resources)
                raw_source = (ROOT / f'docs/benchmarks/{name}/source.choreo').read_bytes()
                raw_bindings = (ROOT / f'docs/benchmarks/{name}/bindings.json').read_bytes()
                placement = b'{"format":"choreoform-authoring-layout","version":"0.1.0","entries":[]}'
                raw = cli.checked('document-import', raw_source, raw_bindings, placement)
                document = json.loads(raw)
                self.assertEqual(document['source'].encode(), raw_source)
                self.assertEqual(document['bindings'].encode(), raw_bindings)
                view = json.loads(cli.checked('document-inspect', raw))
                self.assertEqual(view['semanticRevision'], json.loads(original.stdout)['revision'])
                self.assertEqual(cli.checked('document-export', raw), raw)
                formatted = cli.checked('document-change', raw, change(view['authoringDigest'], dict(kind='format')))
                formatted_data = json.loads(formatted)
                reimported = cli.checked('document-import', formatted_data['source'].encode(), formatted_data['bindings'].encode(), json.dumps(formatted_data['layout']).encode())
                self.assertEqual(reimported, formatted)
                formatted_ir = invoke((formatted_data['source'], json.loads(formatted_data['bindings']), resources))
                self.assertEqual(formatted_ir.stdout, original.stdout, formatted_ir.stderr)
                predicate = None
                for kind in ['predicate', 'type-alias', 'data-protection', 'predicate-input']:
                    rows = ledger['symbols'] if kind == 'predicate-input' else ledger['declarations']
                    row = next(row for row in rows if row.get('role', row.get('kind')) == kind)
                    operation = dict(kind='rename', target=row['id'], name='Native renamed β')
                    result = cli.checked('document-change', raw, change(view['authoringDigest'], operation))
                    data = json.loads(result)
                    lowered = invoke((data['source'], json.loads(data['bindings']), resources))
                    self.assertEqual(lowered.returncode, 0, lowered.stderr)
                    self.assertEqual(lowered.stdout, original.stdout, (name, kind))
                    self.assertEqual(data['projection'], document['projection'])
                    self.assertNotEqual(data['source'], document['source'])
                    if kind == 'predicate': predicate = data
                # Real native checking outcomes still have their existing stages;
                # expected RP03 refusals keep codes/IR paths, with fresh source spans.
                for stage in STAGES:
                    a = invoke(package, stage)
                    b = invoke((predicate['source'], json.loads(predicate['bindings']), resources), stage)
                    self.assertEqual(a.returncode, b.returncode, (name, stage, b.stderr))
                    if a.returncode:
                        self.assertEqual(a.stderr.split(b'; source bytes ')[0], b.stderr.split(b'; source bytes ')[0])
                        self.assertTrue(b.stderr.startswith(b'parallel-effects-unproved at /body/nodes/start_acceptance; source bytes '), b.stderr)
                        self.assertEqual(b.stdout, b'')
                    else:
                        self.assertEqual(json.loads(a.stdout), json.loads(b.stdout), (name, stage))
                        self.assertIs(json.loads(b.stdout)['executable'], False)
                # The same declared pin with changed supplied bytes cannot admit/export.
                first = Path(cli.resources[3]); saved = first.read_bytes(); first.write_bytes(saved + b'\n')
                refused = cli.run('document-export', raw)
                self.assertEqual((refused.returncode, refused.stdout), (1, b''))
                self.assertTrue(refused.stderr.startswith(b'resource-integrity at resources'), refused.stderr)
                measurements.append((name, len(raw), len(formatted), view['authoringDigest']))
        capture = json.loads((ROOT / 'docs/evaluation/0095-native-authoring-observations.json').read_bytes())
        self.assertEqual(capture['stage'], 'native-authoring-operations-only')
        self.assertEqual([list(row) for row in measurements], capture['observations'])
        print('Observed native document/format bytes and authoring digests:', measurements)
        self.assertEqual(fixed(), before)
        inventory = ''.join(name + '\0' + digest + '\n' for name, digest in sorted(before.items()))
        self.assertEqual((len(before), sha256(inventory.encode()).hexdigest()), (965, 'a25c6de850fdaa51436023fffe011c8f96695e9d8660191e7598cd4f4bf2e9f7'))


if __name__ == '__main__': unittest.main()
