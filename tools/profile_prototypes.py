# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Informational release CLI timings; no wall-clock assertions or file writes.

Use the existing hash-locked .tools/ir-check environment after building release
choreoform-portability and choreoform-text-prototype. Synthetic graphs exercise
partial inspection, not executable dialect semantics.
"""
import copy
import json
from pathlib import Path
import statistics
import subprocess
import time

import check_ir_fixtures as wire

ROOT = Path(__file__).resolve().parents[1]


def measure(binary, command, raw):
    """Warm once, then return three process-inclusive samples and result."""
    samples = []
    for repeat in range(4):
        start = time.perf_counter()
        result = subprocess.run([str(ROOT / "target/release" / binary), command],
                                input=raw, stdout=subprocess.DEVNULL,
                                stderr=subprocess.PIPE, timeout=20)
        if repeat:
            samples.append((time.perf_counter() - start) * 1000)
    return {"input_bytes": len(raw), "median_ms": statistics.median(samples),
            "samples_ms": samples, "exit": result.returncode,
            "error": result.stderr.decode().strip()}


def main():
    """Match the critique's scope shapes and annotation-expansion workload."""
    base = wire.load(wire.FIXTURES[0].read_bytes())
    for size in (128, 512, 1024, 2048):
        for shape in ("star", "chain"):
            document = copy.deepcopy(base)
            body = document["body"]
            template = copy.deepcopy(next(iter(body["scopes"].values())))
            for name in wire.MAPS:
                if name != "policies":
                    body[name] = {}
            body["root"] = "s0"
            for policy in body["policies"].values():
                policy["scope"] = "s0"
            for i in range(size):
                scope = copy.deepcopy(template)
                scope.update(parent=None if i == 0 else ("s0" if shape == "star" else f"s{i-1}"),
                             entry=f"n{i}", inputs={}, outputs={}, outcomes={"done": True})
                body["scopes"][f"s{i}"] = scope
                body["nodes"][f"n{i}"] = dict(kind="finish", scope=f"s{i}", reads={},
                                               writes={}, outcomes={}, outcome="done")
            document["revision"] = wire.revision(document)
            raw = json.dumps(document, separators=(",", ":")).encode()
            result = measure("choreoform-portability", "inspect", raw)
            if result["exit"] != 0:
                raise ValueError(result)
            print(json.dumps({"shape": shape, "scopes": size, **result}), flush=True)
    value = [0] * 150000
    for _ in range(40):
        value = [value]
    base["annotations"] = {"nest": value}
    raw = json.dumps(base, separators=(",", ":")).encode()
    result = measure("choreoform-text-prototype", "export", raw)
    if result["exit"] == 0 or result["error"] != f"source size limit at bytes 0..{len(raw)}":
        raise ValueError(result)
    print(json.dumps({"shape": "annotation-expansion", **result}))


if __name__ == "__main__":
    main()
