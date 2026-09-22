# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Check the ADR-0014 evidence inventory, not candidate language validity."""

from collections import Counter
from hashlib import sha256
from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[1]
PLAN = ROOT / "docs/evaluation/0014-authoring-plan.md"
PLAN_DIGEST = "d77dd543ad74ef22f6d16c583d5dc59f0119b80d86bc752577a5391c647353d6"
AUTHORING = ROOT / "docs/authoring"
EVALUATION = ROOT / "docs/evaluation/0014-near-english-authoring.md"
ADR = ROOT / "docs/decisions/0014-near-english-authoring.md"
DOCUMENTS = [PLAN, EVALUATION, ADR] + [
    AUTHORING / name for name in
    ("README.md", "syntax.md", "mapping.md", "benchmarks.md", "study.md")
]
KINDS = {"activity", "compute", "invoke", "decision", "split", "join",
         "wait", "repeat", "fanout", "finish"}
STATUSES = {"Supported", "Partial", "Outside scope", "Unknown"}


def check_scenarios(text, expected):
    """Require one table row per scenario and a primary status per candidate."""
    rows = re.findall(r"^\| (RP-\d\d-[A-D]) \| ([^|]+) \| ([^|]+) \|", text, re.M)
    if Counter(row[0] for row in rows) != Counter(expected):
        raise ValueError("missing, extra or duplicate scenario row")
    if any(a.strip() not in STATUSES or b.strip() not in STATUSES for _, a, b in rows):
        raise ValueError("unknown scenario status")


def check_kinds(text):
    """Check coverage of the accepted core kinds, not the correctness of mapping."""
    section = re.search(r"^## 3\. Candidate A: structured steps\n(.*?)^## 4\.", text, re.M | re.S)
    if section is None:
        raise ValueError("missing core mapping section")
    kinds = re.findall(r"^\| ([a-z]+) \|", section.group(1), re.M)
    counts = Counter(kinds)
    expected = Counter({kind: 1 for kind in KINDS})
    expected["activity"] = 2  # Human and capability action forms.
    if counts != expected:
        raise ValueError("missing, extra or duplicate core mapping row")


def anchors(text):
    """Return heading anchors for this package's simple ATX Markdown headings."""
    seen = Counter()
    result = set()
    for heading in re.findall(r"^#{1,6} (.+)$", text, re.M):
        slug = "".join(c for c in heading.lower() if c.isalnum() or c in " _-")
        slug = slug.replace(" ", "-")
        count = seen[slug]
        seen[slug] += 1
        result.add(f"{slug}-{count}" if count else slug)
    return result


def check_links(path, text=None):
    """Verify repository-local inline link targets; never fetch external links."""
    if text is None:
        text = path.read_text(encoding="utf-8")
    for target in re.findall(r"\]\(([^\s)]+)\)", text):
        if ":" in target or target.startswith("//"):
            continue
        relative, _, anchor = target.partition("#")
        linked = (path.parent / relative).resolve() if relative else path
        if not linked.is_relative_to(ROOT) or not linked.is_file():
            raise ValueError(f"missing local link from {path.name}: {target}")
        if anchor and linked.suffix == ".md":
            if anchor not in anchors(linked.read_text(encoding="utf-8")):
                raise ValueError(f"missing anchor from {path.name}: {target}")


class AuthoringEvidenceTests(unittest.TestCase):
    def test_inventory_and_frozen_plan(self):
        self.assertEqual(sha256(PLAN.read_bytes()).hexdigest(), PLAN_DIGEST)
        for path in DOCUMENTS:
            self.assertIn("SPDX-License-Identifier: CC-BY-4.0", path.read_text(encoding="utf-8"))
            check_links(path)

    def test_corpus_and_core_mapping_coverage(self):
        expected = []
        for path in (ROOT / "docs/representative-processes").glob("[0-9][0-9]-*.md"):
            expected.extend(re.findall(r"^### (RP-\d\d-[A-D]) —", path.read_text(encoding="utf-8"), re.M))
        self.assertEqual(len(expected), 40)
        self.assertEqual(len(set(expected)), 40)
        check_scenarios(EVALUATION.read_text(encoding="utf-8"), expected)
        check_kinds((AUTHORING / "syntax.md").read_text(encoding="utf-8"))
        benchmark_rows = re.findall(r"^\| (RP-\d\d-[A-D]) \|", (AUTHORING / "benchmarks.md").read_text(encoding="utf-8"), re.M)
        self.assertEqual(Counter(benchmark_rows), Counter(
            scenario for scenario in expected if scenario[3:5] in {"01", "03", "08"}
        ))

    def test_missing_duplicate_and_invalid_status_are_refused(self):
        row = "| RP-01-A | Partial | Partial | Example | Gap |\n"
        check_scenarios(row, ["RP-01-A"])
        for invalid in ("", row + row, row.replace("Partial", "Complete", 1)):
            with self.assertRaises(ValueError):
                check_scenarios(invalid, ["RP-01-A"])
        with self.assertRaises(ValueError):
            check_kinds("| activity | Just one kind |")

    def test_heading_anchors_include_duplicates(self):
        self.assertEqual(anchors("# A — B\n## A — B\n# Values & types\n"),
                         {"a--b", "a--b-1", "values--types"})

    def test_core_mapping_mutations_are_refused(self):
        text = (AUTHORING / "syntax.md").read_text(encoding="utf-8")
        for invalid in (
            text.replace("| compute |", "| unknown |", 1),
            text.replace("| invoke |", "| compute |", 1),
            re.sub(r"^\| fanout \|.*\n", "", text, flags=re.M),
        ):
            with self.assertRaises(ValueError):
                check_kinds(invalid)

    def test_missing_link_file_and_anchor_are_refused(self):
        for target in ("absent-authoring-evidence-file.md", "#absent-heading"):
            with self.assertRaises(ValueError):
                check_links(ADR, f"[Broken]({target})")


if __name__ == "__main__":
    unittest.main(verbosity=2)
