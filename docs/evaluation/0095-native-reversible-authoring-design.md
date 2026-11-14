<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Native reversible authoring: implementation boundary comparison

**Date:** 2026-11-14<br>
**Status:** design preparation only; API publication/code await verified PR #109 integration<br>
**Authority:** signed [scope95](0095-native-reversible-authoring-plan.md)
5bfea4b93b8144d97b9088ea27079667b26e5576

## Alternatives

| Approach | Benefit | Cost / decision |
| --- | --- | --- |
| Add focused modules to the existing Rust authoring frontend | Reuses exact syntax tree/token spans, strict indexed ledger, source-only kind/owner resolution, native lowering and private immutable Candidate | Preferred; introduce a distinct immutable admitted-document type and focused IO-free modules, not mutable fields on Package/Candidate |
| A new independent Rust editor/compiler crate | Separate packaging | Duplicates or exposes frontend-private binding details; can drift from actual lexical/type rules; unjustified for this bounded native layer |
| Implement document/changes in Python or browser JavaScript | Easy schema/UI prototyping | Creates a second production semantics/refactoring engine; Wasm/native parity and exact caller-context typing would need duplication; reject for production |

The existing grammar tree retains exact token extents; the ledger already indexes
explicit declarations/symbols and resolves qualified references with lexical privacy.
Use those real identities. Preserve full native proof at the new result boundary,
not an assumption that lexical substitution alone is semantic equivalence. A new
immutable document type can own original strings/layout and its checked Candidate;
short-lived Package parsing avoids a self-referential borrowed struct. Expose shared
inspection only; no mutable metadata or unchecked candidate replacement.

## Resolved changes and bounds

Rename has a deliberately small source-only target matrix. A bounded structural
walk tracks actual lexical scope and enclosing predicate owner. It selects only
declaration names, local-type/protection references, predicate-use references and
predicate input/body-parameter/use-argument keys resolved to the target ID. Preserve
qualification components, ordinary parameters/cell names, opaque literal/type field
payloads, required instructions and unknown annotations. A nested call's argument
keys belong to its resolved callee; argument expression parameters remain in the
caller context. Do not confuse them or recursively rebind a substituted expression.

Build source splices from exact name-token spans, check order/non-overlap and total
output bytes before allocation, emit native canonical quoted names and preserve all
other bytes. Only the selected companion name changes; canonicalize without LF.
Reparse/rebind/re-lower, compare exact complete native IR including annotations and
check stable inventory/layout references. Stale whole-authoring digest refuses before
mutation. Format and layout replacement follow the same atomic checked result rule.
No arbitrary copy/delete/semantic edit/IR reconstruction is smuggled into this API.

Use indexed maps and one bounded explicit-stack syntax walk; avoid repeated full
source scans, name-derived IDs or deep copied parent environments. Bound traversal,
reference/source-splice metadata, decoded values and output separately. The existing
IR transport defaults must remain1MiB with their legacy number/depth meanings;
a separately bounded document transport path may share strict parsing/canonical
writer code while enforcing document-specific limits, item counting and negative-
zero refusal. Its larger whole-bundle byte limit must not change existing callers.
Resource byte identity is a static premise, not actual provider truth or permission.
The exact public API/CLI, built-in contract handling, counters and private diagnostics
must be published before code; this comparison is not that normative binding.

## Evidence and remaining phase work

Require actual native operation outputs against the already pinned finite vectors,
plus nested/qualified/private/same-caption/Unicode/opaque and cumulative limits,
compile-fail immutability and whole current RP-01/03 preservation with all checking
views. Existing Python after-states are contract examples, not native edit evidence.
Keep every previous publication/benchmark capture exact. Separate completed-diff
review/full comment assessment/final-head CI and expected-head signed integration
remain mandatory. No independent/human review or external observations are implied.

The native layer supplies no live authorization, complete validation/diagnostics,
paired accessible/protected UI, visual notation, full RP-08, real three-process
cycles or representative-user/host/browser/independent evidence. All remain Phase1
requirements. No Phase2/new phase chat; phase report still needs owner approval.
