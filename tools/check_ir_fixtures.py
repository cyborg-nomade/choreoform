# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
# /// script
# requires-python = ">=3.11"
# dependencies = ["jsonschema==4.25.1", "rfc8785==0.1.4"]
# ///
"""Working IR wire/linkage evidence, NOT a semantic or execution validator."""

import copy
import hashlib
import json
from pathlib import Path
import sys
import unittest
from unittest.mock import patch

import jsonschema
import rfc8785

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = sorted((ROOT / "examples/ir").glob("*.json"))
SCHEMA = json.loads((ROOT / "schemas/ir/definition-0.1.schema.json").read_text(encoding="utf-8"))
CONTRACTS = {
    ("urn:choreoform:semantics:adr-0008",
     "sha256:0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141"):
        ROOT / "docs/ir/contracts/sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt",
    ("urn:choreoform:examples:illustrative-dialect",
     "sha256:282dd3c02983141da13e8f700e852d5472ffc4af176a53ca3578c418482a09b9"):
        ROOT / "docs/ir/contracts/sha256-282dd3c02983141da13e8f700e852d5472ffc4af176a53ca3578c418482a09b9.txt",
}
MAPS = ("scopes", "data", "expressions", "actors", "capabilities", "policies", "nodes", "flows")
MAX_INT = 9007199254740991


def reject(message):
    """Refuse evidence input without repairing or silently dropping content."""
    raise ValueError(message)


def pairs(items):
    """Build a JSON object while rejecting duplicate keys at any nesting level."""
    result = {}
    for key, value in items:
        if key in result:
            reject(f"duplicate key: {key}")
        result[key] = value
    return result


def scalars(value, depth=0):
    """Check the prototype depth bound and IR integer/Unicode restrictions."""
    if depth > 64:
        reject("prototype nesting limit")
    if isinstance(value, str):
        if any(0xD800 <= ord(c) <= 0xDFFF for c in value):
            reject("unpaired surrogate")
    elif isinstance(value, bool) or value is None:
        return
    elif isinstance(value, int):
        if abs(value) > MAX_INT:
            reject("unsafe integer")
    elif isinstance(value, float):
        reject("floating-point number")
    elif isinstance(value, dict):
        for key, item in value.items():
            scalars(key, depth + 1)
            scalars(item, depth + 1)
    elif isinstance(value, list):
        for item in value:
            scalars(item, depth + 1)


def load(raw):
    """Decode bounded UTF-8 JSON without lossy key or number coercion."""
    if len(raw) > 1024 * 1024:
        reject("prototype byte limit")
    value = json.loads(raw.decode("utf-8"), object_pairs_hook=pairs,
                       parse_float=lambda _: reject("non-integer token"),
                       parse_constant=lambda _: reject("non-finite token"))
    scalars(value)
    return value


def canonical(document):
    """Return JCS bytes of the semantic projection, excluding annotations."""
    scalars(document)
    return rfc8785.dumps({k: document[k] for k in ("format", "version", "kind", "body")})


def revision(document):
    """Hash semantic bytes; this is neither equivalence proof nor signature."""
    return "sha256:" + hashlib.sha256(canonical(document)).hexdigest()


def source_revision(path):
    """Hash exact artifact bytes without text decoding or normalization."""
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


def check_contracts(document):
    """Verify fixture bindings against local snapshots, not dialect meaning."""
    bindings = [document["body"]["semantics"], *document["body"]["dialects"].values()]
    for binding in bindings:
        path = CONTRACTS.get((binding["id"], binding["revision"]))
        if path is None:
            reject("unsupported local contract identity/revision")
        if source_revision(path) != binding["revision"]:
            reject("contract artifact digest mismatch")


def check(document, verify_revision=True):
    """Shape and selected link checks only; dialect meaning is NOT validated."""
    scalars(document)
    try:
        jsonschema.Draft202012Validator(SCHEMA).validate(document)
    except jsonschema.ValidationError as exc:
        reject("shape: " + exc.message)
    b = document["body"]
    ids = [key for name in MAPS for key in b[name]]
    if len(set(ids)) != len(ids):
        reject("ID reused across maps")

    def ref(name, key):
        """Resolve a local ID in the expected declaration map."""
        if key not in b[name]:
            reject(f"missing {name} reference: {key}")
        return b[name][key]

    root = ref("scopes", b["root"])
    if root["parent"] is not None:
        reject("root has parent")
    for key, scope in b["scopes"].items():
        seen = set()
        current = key
        while current is not None:
            if current in seen:
                reject("scope cycle")
            seen.add(current)
            current = ref("scopes", current)["parent"]
        if b["root"] not in seen:
            reject("disconnected scope")
        if ref("nodes", scope["entry"])["scope"] != key:
            reject("scope entry belongs elsewhere")
        for field in ("inputs", "outputs"):
            for data_id in scope[field].values():
                cell = ref("data", data_id)
                if cell["scope"] != key:
                    reject("scope port names foreign data")
                if field == "inputs" and "initial" in cell:
                    reject("input cell has independent initializer")

    def visible(scope_id, name, key):
        """Resolve a declaration visible in this scope or a lexical ancestor."""
        target = ref(name, key)
        current = scope_id
        while current is not None:
            if target["scope"] == current:
                return target
            current = ref("scopes", current)["parent"]
        reject(f"out-of-scope reference: {key}")

    def dialect(value):
        """Require a declared dialect ID without interpreting its payload."""
        ref("dialects", value["dialect"])

    for name in MAPS[1:-1]:
        for record in b[name].values():
            ref("scopes", record["scope"])
    for key, scope in b["scopes"].items():
        for field in ("cancellation", "faults", "closure", "race"):
            visible(key, "policies", scope[field])
    for record in b["policies"].values():
        dialect(record)
    for record in b["expressions"].values():
        dialect(record)
        dialect(record["resultType"])
        for parameter in record["parameters"].values():
            dialect(parameter)
        for key in record["reads"]:
            visible(record["scope"], "data", key)
    for record in b["actors"].values():
        visible(record["scope"], "policies", record["requirement"])
    for record in b["capabilities"].values():
        dialect(record["input"])
        dialect(record["output"])
        visible(record["scope"], "actors", record["authority"])
        for field in ("contract", "effects"):
            visible(record["scope"], "policies", record[field])
    for data_id, record in b["data"].items():
        scope_id = record["scope"]
        dialect(record["type"])
        if "initial" in record:
            if visible(scope_id, "expressions", record["initial"])["parameters"]:
                reject("initial expression has unbound parameters")
        for key in record["invalidates"]:
            # Parent data may explicitly invalidate descendant work.
            target = ref("nodes", key)
            visible(target["scope"], "data", data_id)
        p = record["protection"]
        visible(scope_id, "policies", p["policy"])
        for name, field in (("actors", "participants"), ("capabilities", "capabilities")):
            for key in p[field]:
                visible(scope_id, name, key)

    def child(node, key):
        """Require an invocation target owned directly by the caller's scope."""
        if ref("scopes", key)["parent"] != node["scope"]:
            reject("invocation target is not a direct child template")

    for key, node in b["nodes"].items():
        scope_id, kind = node["scope"], node["kind"]
        for name in ("reads", "writes"):
            for target in node[name]:
                visible(scope_id, "data", target)
        for field in ("input", "condition", "collection", "itemKey", "seal"):
            if field in node:
                expr = visible(scope_id, "expressions", node[field])
                if field == "itemKey":
                    if set(expr["parameters"]) != {"item"} or expr["reads"]:
                        reject("key expression must depend only on item parameter")
                elif expr["parameters"]:
                    reject("unbound expression parameters")
                if not set(expr["reads"]) <= set(node["reads"]):
                    reject("node omits expression read dependency")
        for field in ("policy", "remaining", "changes"):
            if field in node:
                visible(scope_id, "policies", node[field])
        if kind == "activity":
            visible(scope_id, "actors" if node["mode"] == "human" else "capabilities", node["target"])
            if node["result"] is not None:
                visible(scope_id, "data", node["result"])
                if node["result"] not in node["writes"]:
                    reject("activity result omitted from writes")
        if kind in ("invoke", "compute"):
            bindings = node["inputs"] if kind == "invoke" else node["assignments"]
            for expr_id in bindings.values():
                expr = visible(scope_id, "expressions", expr_id)
                if expr["parameters"] or not set(expr["reads"]) <= set(node["reads"]):
                    reject("invalid expression binding/dependencies")
            targets = node["outputs"].values() if kind == "invoke" else node["assignments"]
            for data_id in targets:
                visible(scope_id, "data", data_id)
                if data_id not in node["writes"]:
                    reject("assignment omitted from writes")
            if kind == "compute" and set(node["writes"]) != set(node["assignments"]):
                reject("compute write set differs from assignments")
            if kind == "invoke":
                body_scope = ref("scopes", node["body"])
                if set(node["inputs"]) != set(body_scope["inputs"]) or set(node["outputs"]) != set(body_scope["outputs"]):
                    reject("incomplete invocation interface")
        if kind in ("invoke", "repeat", "fanout"):
            child(node, node["body"])
        if kind == "repeat" and len(b["scopes"][node["body"]]["outcomes"]) != 1:
            reject("repeat body must have one ordinary outcome")
        if kind == "repeat" and (b["scopes"][node["body"]]["inputs"] or b["scopes"][node["body"]]["outputs"]):
            reject("repeat ports outside initial profile")
        if kind == "fanout":
            item = ref("data", node["item"])
            if item["scope"] != node["body"] or "initial" in item:
                reject("invalid fanout item binding")
            if item["type"] != b["expressions"][node["itemKey"]]["parameters"]["item"]:
                reject("item/key parameter structural type mismatch")
            template = b["scopes"][node["body"]]
            if template["inputs"] != {"item": node["item"]} or template["outputs"]:
                reject("invalid fanout port interface")
        if kind == "split":
            for target in node["children"].values():
                child(node, target)
                if b["scopes"][target]["inputs"] or b["scopes"][target]["outputs"]:
                    reject("split ports outside initial profile")
        if kind in ("split", "fanout"):
            join = ref("nodes", node["join"])
            if join["kind"] != "join" or join["source"] != key or join["scope"] != scope_id:
                reject("unpaired join")
        if kind == "join":
            source = ref("nodes", node["source"])
            if source["kind"] not in ("split", "fanout") or source["join"] != key or source["scope"] != scope_id:
                reject("unpaired source")
        if kind == "finish" and node["outcome"] not in b["scopes"][scope_id]["outcomes"]:
            reject("undeclared scope outcome")
        if kind == "decision":
            expected = set(node["guards"])
            if node["default"] is not None:
                if node["default"] in expected:
                    reject("default also has guard")
                expected.add(node["default"])
            if expected != set(node["outcomes"]):
                reject("decision outcomes do not match guards/default")
            for target in node["guards"].values():
                expr = visible(scope_id, "expressions", target)
                if expr["parameters"]:
                    reject("guard has unbound parameters")
                if not set(expr["reads"]) <= set(node["reads"]):
                    reject("decision omits guard read dependency")
        if kind == "invoke" and node["outcomes"] != b["scopes"][node["body"]]["outcomes"]:
            reject("invocation outcomes differ from child scope")

    observed = set()
    adjacency = {key: [] for key in b["nodes"]}
    for flow in b["flows"].values():
        source = ref("nodes", flow["source"])
        target = ref("nodes", flow["target"])
        if source["scope"] != target["scope"]:
            reject("cross-scope flow")
        if flow["outcome"] not in source["outcomes"]:
            reject("undeclared node outcome")
        pair = (flow["source"], flow["outcome"])
        if pair in observed:
            reject("implicit fork")
        observed.add(pair)
        adjacency[flow["source"]].append(flow["target"])
    expected = {(key, outcome) for key, node in b["nodes"].items() for outcome in node["outcomes"]}
    if observed != expected:
        reject("missing outcome flow")
    for key, node in b["nodes"].items():
        if node["kind"] in ("split", "fanout"):
            adjacency[key].append(node["join"])
    visiting, visited = set(), set()

    def acyclic(key):
        """Reject implicit control cycles, including split/join return edges."""
        if key in visiting:
            reject("graph cycle outside repeat")
        if key in visited:
            return
        visiting.add(key)
        for target in adjacency[key]:
            acyclic(target)
        visiting.remove(key)
        visited.add(key)

    for key in adjacency:
        acyclic(key)
    check_contracts(document)
    if verify_revision and document["revision"] != revision(document):
        reject("semantic revision mismatch")


class WireEvidence(unittest.TestCase):
    """Serialization and selected linkage regressions, not conformance tests."""

    def setUp(self):
        """Load fresh fixture objects so mutations cannot leak between tests."""
        self.documents = [load(path.read_bytes()) for path in FIXTURES]

    def test_schema_and_three_fixtures(self):
        """Check the schema and all three structurally valid benchmark excerpts."""
        jsonschema.Draft202012Validator.check_schema(SCHEMA)
        self.assertEqual(len(self.documents), 3)
        for d in self.documents:
            check(d)

    def test_local_contract_digests(self):
        """Verify exact registered identities and content-addressed filenames."""
        for d in self.documents:
            check_contracts(d)
        for (_, digest), path in CONTRACTS.items():
            self.assertEqual(digest, source_revision(path))
            self.assertEqual(path.stem, digest.replace(":", "-"))

    def test_editorial_source_edits_preserve_bindings(self):
        """Simulate live-document edits; admission must read only frozen bytes."""
        read_bytes = Path.read_bytes
        live = {ROOT / "docs/decisions/0008-core-process-semantics.md",
                ROOT / "docs/ir/examples.md"}
        observed = set()

        def edited_source(path):
            """Model an editorial change in memory, leaving disk untouched."""
            observed.add(path)
            raw = read_bytes(path)
            return raw + b"\nEditorial correction.\n" if path in live else raw

        with patch.object(Path, "read_bytes", edited_source):
            for d in self.documents:
                check(d)
                self.assertEqual(d["revision"], revision(d))
        self.assertEqual(observed, set(CONTRACTS.values()))

    def test_contract_binding_and_corruption_failures(self):
        """Refuse unknown IDs/digests and tampered snapshots for both bindings."""
        for kind in ("semantics", "dialect"):
            for field, value in (("id", "urn:unknown"), ("revision", "sha256:" + "0" * 64)):
                d = copy.deepcopy(self.documents[0])
                binding = d["body"]["semantics"] if kind == "semantics" else d["body"]["dialects"]["illustrative"]
                binding[field] = value
                with self.subTest(kind=kind, field=field), self.assertRaisesRegex(ValueError, "unsupported local contract"):
                    check(d, verify_revision=False)
        read_bytes = Path.read_bytes
        for artifact in CONTRACTS.values():
            def corrupt_snapshot(path):
                """Change one registered artifact's bytes without editing disk."""
                raw = read_bytes(path)
                return raw + b"\n" if path == artifact else raw

            with patch.object(Path, "read_bytes", corrupt_snapshot):
                with self.subTest(artifact=artifact.name), self.assertRaisesRegex(ValueError, "artifact digest mismatch"):
                    check(self.documents[0])

    def test_map_order_and_pretty_print_preserve_revision(self):
        """Object insertion order and transport formatting are non-semantic."""
        def reverse(value):
            """Reverse object insertion order recursively, preserving arrays."""
            if isinstance(value, dict):
                return {k: reverse(v) for k, v in reversed(list(value.items()))}
            if isinstance(value, list):
                return [reverse(v) for v in value]
            return value
        for d in self.documents:
            self.assertEqual(canonical(d), canonical(reverse(d)))
            self.assertEqual(revision(d), revision(load(json.dumps(d, indent=4).encode())))

    def test_annotation_edit_preserves_revision(self):
        """Unknown editor metadata stays outside the semantic projection."""
        for d in self.documents:
            before = revision(d)
            d["annotations"]["unknown-editor"] = {"label": "changed", "x": 123}
            self.assertEqual(before, revision(d))

    def test_semantic_edit_changes_revision(self):
        """Protection, policy, expression, and definition identity affect hashes."""
        for field, value in (("sensitivity", "public"), ("policy", "p_cancel")):
            d = copy.deepcopy(self.documents[0])
            d["body"]["data"]["expense"]["protection"][field] = value
            self.assertNotEqual(self.documents[0]["revision"], revision(d))
        d = copy.deepcopy(self.documents[0])
        d["body"]["expressions"]["expense_input"]["body"]["changed-guard"] = True
        self.assertNotEqual(self.documents[0]["revision"], revision(d))
        d = copy.deepcopy(self.documents[0])
        d["body"]["id"] += "-copy"
        self.assertNotEqual(self.documents[0]["revision"], revision(d))

    def test_array_order_is_significant(self):
        """Canonicalization must not sort ordered dialect payload arrays."""
        d = self.documents[0]
        payload = d["body"]["expressions"]["expense_input"]["body"]
        payload["arguments"] = ["a", "b"]
        before = revision(d)
        payload["arguments"].reverse()
        self.assertNotEqual(before, revision(d))

    def test_jcs_utf16_key_order_and_escaping(self):
        """Exercise JCS ordering across BMP boundaries and string escaping."""
        self.assertEqual(rfc8785.dumps({"\ue000": 1, "😀": 2}), '{"😀":2,"\ue000":1}'.encode())
        self.assertEqual(rfc8785.dumps({"q": '\n"'}), b'{"q":"\\n\\\""}')

    def test_invalid_transport(self):
        """Reject duplicate keys, unsafe numbers, BOMs, and invalid Unicode."""
        for raw in (b'{"x":1,"x":2}', b'{"x":1.0}', b'{"x":1e0}', b'{"x":NaN}',
                    b'{"x":9007199254740992}', b'{"x":"\\ud800"}', b'\xef\xbb\xbf{}', b'\xff'):
            with self.subTest(raw=raw), self.assertRaises(ValueError):
                load(raw)

    def test_negative_mutations(self):
        """Reject selected shape, reference, dependency, and revision defects."""
        mutations = {
            "unknown version": lambda d: d.update(version="0.2.0"),
            "unknown core field": lambda d: d["body"].update(hiddenPriority=1),
            "unknown node kind": lambda d: d["body"]["nodes"]["review"].update(kind="script"),
            "missing protection": lambda d: d["body"]["data"]["expense"].pop("protection"),
            "dangling flow": lambda d: d["body"]["flows"]["review_approved"].update(target="absent"),
            "wrong-kind target": lambda d: d["body"]["nodes"]["review"].update(target="expense"),
            "cross-map duplicate": lambda d: d["body"]["policies"].update(expense=d["body"]["policies"]["p_work"]),
            "scope cycle": lambda d: d["body"]["scopes"]["root"].update(parent="root"),
            "missing dialect": lambda d: d["body"]["dialects"].clear(),
            "implicit fork": lambda d: d["body"]["flows"].update(duplicate=d["body"]["flows"]["review_approved"]),
            "missing flow": lambda d: d["body"]["flows"].pop("review_approved"),
            "unguarded read omission": lambda d: d["body"]["nodes"]["pay"]["reads"].clear(),
            "implicit cycle": lambda d: d["body"]["flows"]["review_approved"].update(target="review"),
        }
        for label, mutate in mutations.items():
            d = copy.deepcopy(self.documents[0])
            mutate(d)
            with self.subTest(label=label), self.assertRaises(ValueError):
                check(d, verify_revision=False)
        d = copy.deepcopy(self.documents[0])
        d["revision"] = "sha256:" + "0" * 64
        with self.assertRaisesRegex(ValueError, "revision mismatch"):
            check(d)

    def test_join_and_cross_scope_failures(self):
        """Require reciprocal joins, closed predicates, and lexical flows."""
        for label, mutate in {
            "unpaired join": lambda b: b["nodes"]["acceptance_join"].update(source="reserve"),
            "nonmonotone join": lambda b: b["nodes"]["acceptance_join"].update(predicate={"op": "not", "outcomes": {"success": True}}),
            "cross-scope flow": lambda b: b["flows"]["stock_confirmed"].update(target="accepted"),
        }.items():
            d = copy.deepcopy(self.documents[1])
            mutate(d["body"])
            with self.subTest(label=label), self.assertRaises(ValueError):
                check(d, verify_revision=False)

    def test_fanout_binding_failures(self):
        """Require a child item cell and a key based solely on its item argument."""
        for label, mutate in {
            "parent item cell": lambda b: b["nodes"]["assets_fanout"].update(item="assets"),
            "mutable key dependency": lambda b: b["expressions"]["item_key"].update(reads={"assets": True}),
            "missing item argument": lambda b: b["expressions"]["item_key"].update(parameters={}),
        }.items():
            d = copy.deepcopy(self.documents[2])
            mutate(d["body"])
            with self.subTest(label=label), self.assertRaises(ValueError):
                check(d, verify_revision=False)

    def test_interfaces_and_compute(self):
        """Check pure assignment writes, activity results, and port ownership."""
        d = copy.deepcopy(self.documents[0])
        d["body"]["nodes"]["review"] = {
            "kind": "compute", "scope": "root", "outcomes": {"approved": True},
            "reads": {"expense": True}, "writes": {"expense": True},
            "assignments": {"expense": "expense_input"},
        }
        del d["body"]["flows"]["review_rejected"]
        d["body"]["scopes"]["root"]["outputs"] = {"result": "expense"}
        check(d, verify_revision=False)
        d["body"]["nodes"]["review"]["writes"].clear()
        with self.assertRaisesRegex(ValueError, "omitted from writes"):
            check(d, verify_revision=False)
        d = copy.deepcopy(self.documents[0])
        d["body"]["nodes"]["pay"]["result"] = "expense"
        with self.assertRaisesRegex(ValueError, "result omitted"):
            check(d, verify_revision=False)
        d = copy.deepcopy(self.documents[2])
        d["body"]["scopes"]["root"]["outputs"] = {"foreign": "asset_item"}
        with self.assertRaisesRegex(ValueError, "foreign data"):
            check(d, verify_revision=False)


if __name__ == "__main__":
    if sys.argv[1:] == ["--revisions"]:
        for path in FIXTURES:
            print(path.relative_to(ROOT), revision(load(path.read_bytes())))
    else:
        unittest.main(verbosity=2)
