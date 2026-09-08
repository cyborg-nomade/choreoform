# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Independent native text CLI oracle; use the portability hash-locked venv."""

import hashlib
import json
from pathlib import Path
import subprocess
import sys

import rfc8785

import check_ir_fixtures as wire


ROOT = Path(__file__).resolve().parents[1]
BINARY = ROOT / "target/debug" / (
    "choreoform-text-prototype.exe" if sys.platform == "win32"
    else "choreoform-text-prototype"
)


def require(condition, message):
    """Fail the oracle on an observed contract violation."""
    if not condition:
        raise ValueError(message)


def run(command, raw):
    """Invoke the native adapter with bounded test input and a timeout."""
    return subprocess.run(
        [str(BINARY), command], input=raw, capture_output=True, timeout=10,
    )


def main():
    """Compare complete fixtures, independent hashes, and CLI refusal paths."""
    for path in wire.FIXTURES:
        expected = wire.load(path.read_bytes())
        source = ROOT / "examples/text" / (path.stem + ".choreo")
        lowered = run("lower", source.read_bytes())
        require(lowered.returncode == 0 and not lowered.stderr, f"lower: {path.name}")
        actual = wire.load(lowered.stdout)
        require(actual == expected, f"lossy text lowering: {path.name}")
        wire.check(actual)
        projection = {key: actual[key] for key in ("format", "version", "kind", "body")}
        revision = "sha256:" + hashlib.sha256(rfc8785.dumps(projection)).hexdigest()
        require(actual["revision"] == revision, f"text revision: {path.name}")
        exported = run("export", lowered.stdout)
        require(exported.returncode == 0 and not exported.stderr, f"export: {path.name}")
        repeated = run("lower", exported.stdout)
        require(repeated.returncode == 0 and wire.load(repeated.stdout) == expected,
                f"repeat lowering: {path.name}")
        repeated_export = run("export", repeated.stdout)
        require(repeated_export.returncode == 0 and repeated_export.stdout == exported.stdout,
                f"stable export: {path.name}")

    for command, raw in [
        ("lower", b"\xff"), ("lower", b" " * (1024 * 1024 + 1)),
        ("lower", b'choreoform "9.0.0";'), ("lower", b'choreoform "0.1.0";'),
        ("export", b"{}"), ("unknown", b""),
    ]:
        result = run(command, raw)
        require(result.returncode != 0 and not result.stdout and result.stderr,
                f"CLI must refuse without partial stdout: {command}, {raw[:30]!r}")
    oversized = wire.load(wire.FIXTURES[0].read_bytes())
    oversized["annotations"] = {"large": [0] * 200_000}
    raw = json.dumps(oversized, separators=(",", ":")).encode("utf-8")
    require(len(raw) < 1024 * 1024, "oversized-export fixture must fit the IR input limit")
    wire.check(oversized)
    result = run("export", raw)
    require(result.returncode != 0 and not result.stdout,
            "oversized generated source must not produce partial output")
    require(result.stderr.decode().strip() == f"source size limit at bytes 0..{len(raw)}",
            "generated-source limit must retain input span and not imply unrepresentability")
    print("Text oracle passed: 3 complete IR/schema/JCS comparisons, 3 stable export cycles, 7 CLI refusals (including generated-source expansion).")


if __name__ == "__main__":
    main()
