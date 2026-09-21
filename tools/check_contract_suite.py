# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Publication registry and integrity evidence, NOT executable dialect support."""
from hashlib import sha256
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
SUITE_ID = "urn:choreoform:contracts:core-profile:0.1.0"
REVISION = "sha256:323e6a1333efe412088e52e2fda5cf37c4651dfff3f36b284d95020ea2753bab"
PATHS = (
    "docs/dialects/README.md",
    "docs/dialects/values-expressions.md",
    "docs/dialects/policies.md",
)
REGISTRY = {
    (SUITE_ID, REVISION): ROOT / "docs/ir/contracts/sha256-323e6a1333efe412088e52e2fda5cf37c4651dfff3f36b284d95020ea2753bab.txt"
}


def unpack(raw):
    """Read exactly three ordered path/byte-count/content frames, without IO."""
    sections = []
    cursor = 0
    for expected in PATHS:
        end = raw.find(b"\n", cursor)
        if end < 0 or raw[cursor:end] != expected.encode("utf-8"):
            raise ValueError("snapshot path/order")
        cursor = end + 1
        end = raw.find(b"\n", cursor)
        count = raw[cursor:end]
        if (end < 0 or not count or len(count) > 8
                or not count.isdigit() or count.startswith(b"0")):
            raise ValueError("snapshot byte count")
        cursor = end + 1
        size = int(count)
        content = raw[cursor:cursor + size]
        if len(content) != size:
            raise ValueError("truncated snapshot")
        content.decode("utf-8")
        if b"SPDX-License-Identifier: CC-BY-4.0" not in content:
            raise ValueError("missing source license notice")
        sections.append(content)
        cursor += size
    if cursor != len(raw):
        raise ValueError("trailing snapshot bytes")
    return sections


def verify(identity, revision, raw):
    """Verify an explicitly registered pin and framing, not semantic validity."""
    if (identity, revision) not in REGISTRY:
        raise ValueError("unregistered contract")
    if "sha256:" + sha256(raw).hexdigest() != revision:
        raise ValueError("snapshot digest mismatch")
    return unpack(raw)


class PublishedSuiteEvidence(unittest.TestCase):
    """Keep frozen bytes independent of editable source documents."""

    def setUp(self):
        self.raw = REGISTRY[(SUITE_ID, REVISION)].read_bytes()

    def test_registered_snapshot_and_filename(self):
        self.assertEqual(len(verify(SUITE_ID, REVISION, self.raw)), 3)
        self.assertEqual(REGISTRY[(SUITE_ID, REVISION)].stem,
                         REVISION.replace(":", "-"))

    def test_changed_bytes_and_unknown_bindings_fail(self):
        for raw in (self.raw[:-1], self.raw + b"\n", b"x" + self.raw[1:]):
            with self.assertRaisesRegex(ValueError, "digest mismatch"):
                verify(SUITE_ID, REVISION, raw)
        for identity, revision in (("unknown", REVISION), (SUITE_ID, "sha256:" + "0" * 64)):
            with self.assertRaisesRegex(ValueError, "unregistered"):
                verify(identity, revision, self.raw)

    def test_framing_rejects_order_counts_truncation_and_trailing_bytes(self):
        first_end = self.raw.index(b"\n")
        count_end = self.raw.index(b"\n", first_end + 1)
        malformed = (
            b"wrong\n" + self.raw[first_end + 1:],
            self.raw[:first_end + 1] + b"01" + self.raw[count_end:],
            self.raw[:first_end + 1] + b"99999999" + self.raw[count_end:],
            self.raw[:-1],
            self.raw + b"x",
        )
        for raw in malformed:
            with self.assertRaises(ValueError):
                unpack(raw)

    def test_exact_reframing_preserves_all_bytes(self):
        sections = unpack(self.raw)
        rebuilt = b"".join(path.encode() + b"\n" + str(len(data)).encode()
                           + b"\n" + data for path, data in zip(PATHS, sections))
        self.assertEqual(rebuilt, self.raw)


if __name__ == "__main__":
    unittest.main(verbosity=2)
