# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
# /// script
# requires-python = ">=3.11"
# dependencies = ["jsonschema==4.25.1", "rfc8785==0.1.4"]
# ///
"""Independent JCS/hash/retention oracle for the generated Rust suite report."""

import hashlib
import json
from pathlib import Path
import sys
import subprocess

import rfc8785

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools"))
import check_ir_fixtures as wire  # noqa: E402


def require(condition, message):
    if not condition:
        raise ValueError(message)


def main():
    path = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).parent / "generated/native.json"
    report = json.loads(path.read_bytes())
    require(report["reportVersion"] == "0.1.0" and report["passed"], "failed/unsupported report")
    rows = {row["name"]: row for row in report["cases"]}
    require(len(rows) == len(report["cases"]), "duplicate case name")
    canonical_count = 0
    for row in rows.values():
        actual = row["actual"]
        require(row["passed"] and actual["category"] == row["expected"], row["name"])
        if "canonical" not in actual:
            continue
        canonical = actual["canonical"].encode("utf-8")
        document = actual.get("document")
        projection = ({key: document[key] for key in ("body", "format", "kind", "version")}
                      if document is not None else wire.load(canonical))
        require(rfc8785.dumps(projection) == canonical, f"JCS mismatch: {row['name']}")
        require("sha256:" + hashlib.sha256(canonical).hexdigest() == actual["revision"],
                f"SHA-256 mismatch: {row['name']}")
        canonical_count += 1

    names = ("reimbursement", "order", "incident")
    for name, path in zip(names, wire.FIXTURES):
        expected = wire.load(path.read_bytes())
        actual = rows[name]["actual"]
        require(actual["document"] == expected, f"lossy fixture: {name}")
        wire.check(actual["document"])
        require(actual["revision"] == expected["revision"], f"frozen revision: {name}")

    binary = ROOT / "target/debug" / ("choreoform-portability.exe" if sys.platform == "win32" else "choreoform-portability")
    for name, path in zip(names, wire.FIXTURES):
        result = subprocess.run([str(binary), "inspect"], input=path.read_bytes(), capture_output=True, timeout=10)
        require(result.returncode == 0 and result.stdout == rows[name]["actual"]["canonical"].encode("utf-8"), f"CLI canonical bytes: {name}")
    for raw, category in [(b'\xff', b'utf8'), (b'{"x":0,"x":1}', b'duplicate-key'),
                          (b'1e0', b'number-token'), (b' ' * (1024 * 1024 + 1), b'size')]:
        result = subprocess.run([str(binary), "inspect"], input=raw, capture_output=True, timeout=10)
        require(result.returncode != 0 and not result.stdout and result.stderr.strip() == category, f"CLI refusal: {category!r}")

    base = rows["reimbursement"]["actual"]
    for name in ("annotation-only", "map-order-and-whitespace"):
        require(rows[name]["actual"]["canonical"] == base["canonical"], name)
    changed = rows["empty-access-retained"]["actual"]["document"]
    protection = changed["body"]["data"]["expense"]["protection"]
    require(protection["participants"] == {} and protection["capabilities"] == {}, "empty access")
    require(protection.keys() == base["document"]["body"]["data"]["expense"]["protection"].keys(), "lost protection fields")
    require(rows["policy-edit"]["actual"]["revision"] != base["revision"], "policy edit hash")
    array_rows = [row for name, row in rows.items() if name.startswith("ordered-array-")]
    require(len(array_rows) == 2 and array_rows[0]["actual"]["revision"] != array_rows[1]["actual"]["revision"], "array ordering")
    kinds = {kind for row in rows.values() for kind in row["actual"].get("nodeKinds", [])}
    require(kinds == {"activity", "compute", "invoke", "decision", "split", "join", "wait", "repeat", "fanout", "finish"}, "variant coverage")
    print(f"Independent oracle passed: {len(rows)} case results, {canonical_count} JCS/hash comparisons, 3 unchanged fixtures, all 10 node variants, 7 native CLI checks.")


if __name__ == "__main__":
    main()
