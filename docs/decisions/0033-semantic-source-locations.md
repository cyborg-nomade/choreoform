<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0033: Locate semantic diagnostics in original source declarations

**Status:** Accepted under Project Owner delegation; effective upon PR merge<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

Semantic stages report stable codes and IR declaration paths. The Candidate
already holds immutable original UTF-8 spans for declarations and generated
records, but callers must currently connect those themselves. Source 0.4.0 adds
useful lifecycle paths that should point back to the authored definition without
inventing exact field locations or changing semantic report authority.

## Decision

Add `Candidate::source_span(&Diagnostic) -> Option<Range<usize>>`. Match only an
existing `/body/{record-map}/{id}` prefix against the same immutable document and
span map. Missing field suffixes may still locate their containing declaration;
missing rows, wrong tables, global and resource paths have no guessed fallback.
Return a cloned half-open original UTF-8 byte range. Use borrowed path segments
and indexed maps, without scanning source, copying literals or modifying errors.

The native frontend uses one formatter for all semantic-check commands. Preserve
`code at IR-path`, appending `; source bytes START..END` only when known. Preserve
frontend syntax/binding errors' existing spans and all source/IR hashes/versions.
Document selected interpretations and practical limits in the
[diagnostic guide](../authoring/diagnostics.md), not a purported complete catalogue.

## Options and consequences

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Existing immutable declaration/generated spans | Useful source locations; small additive API, stable codes/paths | Not exact field/token precision; reparse after edits | Adopt |
| Guess a root/default source span | Every error appears located | Misleads about external/global failures | Reject |
| Search source for wire IDs/labels | No map API needed | IDs need not appear in text; aliases and Unicode can mislead or leak literals | Reject |
| Introduce editor coordinates/JSON diagnostic protocol now | Richer integration | Requires explicit coordinate/protocol design and broader editor work | Defer |

Callers must use diagnostics from the same candidate; the helper does not prove
the origin/revision of a freely constructed Diagnostic. It is source metadata,
not semantic admission, a repair, execution authority or exact error-token proof.
Keep full field/source diagnostics, complete validator/proof coverage, benchmark/
conformance, general text/visual/protected accessible editing and actual user
evidence open. No new contract snapshot, dependency or source version is required.

Follow [frozen plan](../evaluation/0033-source-diagnostics-plan.md), `15aa0c2`.
[Evidence](../evaluation/0033-source-diagnostics.md) records 246 Rust tests plus
five compile-fail doctests, native/Wasm and all relevant artifact/source/stage
checks. Separate completed-diff automated self-review found no remaining actionable
issue. Accepted under delegation; require passing final-head CI/expected-head merge.
No independent or individual owner review is claimed.

The [static-path correction plan](../evaluation/0033-static-paths-plan.md),
`eed84c2`, applies this decision to previously bare scope/input, actor, data and
capability errors, preserving codes, deeper paths and evaluation order. Global
graph/resource/structure/revision failures retain no guessed location. This is
diagnostic coverage, not a changed semantic contract or a complete catalogue.
Its [evidence](../evaluation/0033-static-diagnostic-paths.md) records separate
delegated automated review and 251 Rust tests plus five compile-fail doctests,
native/Wasm and relevant artifact/source/stage checks. Final-head CI and expected-
head merge remain required; no independent or individual owner review is claimed.
