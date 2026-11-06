<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Semantic diagnostic source locations: frozen plan

**Date:** 2026-11-06<br>
**Baseline:** `b6dbb9f6c16b68afdc8a9928d7ac68f8bfe90852` (PR #42)<br>
**Authority:** ADR-0018 delegation; Roadmap stable/actionable diagnostics.

Publish ADR-0033 and a bounded additive Candidate utility mapping a semantic
diagnostic's `/body/{record-map}/{id}` prefix to its original declaration/generated
record span. Require the table and row to exist in this exact immutable candidate;
do not match an ID in a different table or guess a location for global/resource,
unknown or missing-record paths. Preserve suffixes/codes/IR paths unchanged. A
missing field suffix can still locate its containing existing declaration.

Return an optional cloned half-open UTF-8 byte range from the existing source map,
without copying literals or scanning the source. This is a declaration location,
not an exact erroneous token/field, fix, validation result or cross-revision proof.
Use diagnostics from the same candidate; old original-source spans do not apply
to a formatted/revised package until it is reparsed. Preserve immutable maps.

Use one native formatter for all semantic-check commands, retaining the existing
`code at IR-path` prefix and appending `; source bytes START..END` only when a
source declaration is known. Existing frontend parse/binding errors retain their
own spans. No line/column/editor coordinate assumption, guessed source fallback,
JSON error protocol, file read or editor product is introduced.

Tests: actual static/control/lifecycle refusals map to original declarations;
generated initializer/assignment/flow records map to their existing source spans;
UTF-8 and formatting/reparse behavior; missing suffix/row/table/global paths;
strict table isolation and read-only metadata; native stderr prefix/code/path
stability and no source literal leakage. Preserve all old hash/revision/profile
behavior and bounded outputs. Document practical interpretation of invalid,
unproved, resource and budget codes with selected examples, without claiming a
complete diagnostic catalogue or full-validator closure.

Run relevant native/Wasm/source/stage checks, separately review the completed
diff, inspect PR comments, record delegated acceptance and require passing
final-head CI/expected-head merge. Then continue full validator/exceptional/
follow-up proof gaps, full benchmarks/conformance, general text/visual/protected
accessible editing and actual user evidence. No phase exit or Phase 2 work.
