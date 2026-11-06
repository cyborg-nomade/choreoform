<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Static record path correction evidence

**Date:** 2026-11-06<br>
**Plan:** [correction plan](0033-static-paths-plan.md), `eed84c2`<br>
**Decision:** [ADR-0033](../decisions/0033-semantic-source-locations.md)

Before changes, actual source packages reproduced `input-initializer at` and
`capability-type at` without IR/source locations. The corrected checker attaches
real scope/port and capability input/output paths, enabling the existing immutable
source mapping. Scope policy/outcome/ancestry, actor requirement, data protection/
initializer/invalidation and capability contract/authority/effect failures also
gain selected record/field paths. A private annotator preserves deeper paths;
formatting allocates only on error. Codes and validation meaning are unchanged.

Capability input/output checks keep their old short-circuit order. Tests prove
input mismatch still precedes an unsupported output descriptor under both core
profiles. No cycle member, external/global location or exact source token is
guessed. Global revision/structure/resource/graph failures retain their scope.

Five Rust groups exercise selected raw-IR record/field failures with structural
admission and real JSON pointer targets, unused actor/private data metadata,
capability roles, both core profiles, ancestry limit and global revision behavior.
Three Python groups cover actual initialized-input source diagnostics, capability
input/output failures across source 0.2/0.3/0.4 with UTF-8 source spans, and docs.
Source binding may reject other invalid references before semantic stages; those
are not mislabeled as these semantic failures. A structurally invalid empty purpose
set was excluded from semantic test evidence in favor of a genuine visibility
failure reaching the data protection judgment.

Passed 251 Rust tests plus five compile-fail doctests, native format/lint,
semantic/frontend Wasm lint/release frontend build and all relevant artifact/
source/stage Python oracles. Logs: `/private/tmp/static-path-rust.log`,
`/private/tmp/static-path-clippy.log`, `/private/tmp/static-path-oracles.log`.

Separate completed-diff automated self-review checked unchanged judgment order/
codes, actual pointer targets and ASCII wire keys, preserved deeper paths,
success-path allocation behavior, source lookup and literal privacy, both core
profiles and honest remaining precision limits. No actionable issue remains.
Accepted under owner delegation as ADR-0033 implementation follow-up; require
passing final-head CI/expected-head merge. No independent or individual owner
review is claimed. The complete diagnostic catalogue, full validator/proof,
benchmark/conformance and text/visual/protected editing/user gates remain open.

Next return to the full RP-01/03/08 benchmark gap audit. The existing synthetic
fixtures and isolated semantic checks are not those complete process packages;
keep all forty historical scenarios accountable without upgrading paper/host/
user evidence. Phase 1 remains incomplete.
