<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Static contract-reader/checking implementation evidence

**Date:** 2026-11-02<br>
**Status:** Proposed; awaiting Project Owner review<br>
**Decision:** [ADR-0016](../decisions/0016-static-contract-checks.md)<br>
**Baseline:** `9820346bdb848b34436c50621aa1689cfa14f5e3` (merged PR #20)<br>
**Frozen plan:** [0016-static-check-plan.md](0016-static-check-plan.md), committed
before implementation in `4048de8`<br>
**API/supported subset/budgets:** [semantic-check README](../../crates/semantic-check/README.md)

## Outcome

The next bounded item supplies exact-byte named-type/provider/calendar readers,
closed type/value/expression and policy-payload checks, selected node/type/transfer
judgments and source integration. It is a **static stage**, not a complete validator
or execution capability. The service source package uses both a provider and a
nominal invariant; it is synthetic, not an adapter, production authorization or a
complete representative-process benchmark.

No frozen semantic/core/illustrative artifact or old fixture was changed. There
are no new third-party dependencies. Source labels/IDs/annotations and existing
IR hash rules remain unchanged. The historical probe's supported registry stays
illustrative-only; a new explicit checker recognizes the accepted core suite.

## Evidence and commands

Run workspace tests/format/Clippy, frontend/semantic library Wasm Clippy and release
frontend Wasm compilation. Then build the native adapter and run
`tools/check_static_contracts.py` in the existing hash-locked Python environment.
Existing authoring, IR, snapshot, technical-text and portability checks remain gates.

The local verification run passed 85 Rust tests plus five compile-fail doctests,
workspace formatting/warnings-as-errors lint, both new library Wasm lints and
release frontend Wasm compilation. New semantic coverage comprises ten public
contract-helper tests, one private nominal graph-guard test and ten source/static
integration tests. Existing control examples and the new core transport/structure
boundary test also pass. The six-group Python static oracle, four-group frontend
oracle (106 grammar positives), IR (16), snapshot (4), authoring evidence (6),
authoring profile (17) and text oracle all pass. The native portability report
retains 89 case results, 15 JCS/hash comparisons, three unchanged fixtures, all ten
node variants and seven CLI checks. These observations do not close residual gates.

PR review regression: invocation output ports now apply the direct transfer rule
from the child-owned source cell to the caller destination, independently of
parent read sets. The broader-destination test reproduced the original false
success before the fix. Purpose/participant/capability widening and mismatched
sensitivity/policy identity refuse; equal/narrower envelopes, multiple ports and
source-package reparse pass. Existing visibility/write/type guards remain intact.

| Judgment | Reproduced observations | Boundary |
| --- | --- | --- |
| Exact resource reading | Missing/duplicate/corrupt pins; valid digest with unknown shape; strict complete provider observations/result/effect declarations | No adapter/calendar trust or host operational support |
| Types/values | All nine forms, signed-64 boundaries, exact scales, record/variant/list/option values and nominal identity/invariants | No domain-specific sufficiency claim |
| Expressions | Closed operations, typed dead branches, full inferred reads, exact binding sites; invariant constructors/short-circuit/arithmetic/rescale/faults | No process interpreter or permission to access a snapshot |
| Policies | Twelve closed variants, typed fact contexts, contextual work results, dependency closure and cyclic-policy refusal | No authentication, atomic transitions, race/reconciliation proof |
| Calendars/clocks | Finite sorted half-open intervals and exact provenance; hashed unknown clock still refuses source lowering | No clock format invented; no time-zone/calendar generation or timer execution |
| Source integration | Terminal/review/service lower → check → format → reparse cycles; independent IR schema/JCS/revision and package hash checks | Original-package cycles only; no general IR exporter or safe edit transaction |
| Dependency/protection | Provider-only protection reads and actor/capability closure; data-access reads; missing dependencies and incompatible direct transfers refuse | Whole control-sensitive flow and access authorization remain open |
| Bounds/diagnostics | Artifact/AST/value limits, unknowns, private synthetic nominal graph cycle/depth guards, errors without literal payloads | Bounded prototype, not security/scale certification |

Rust regression groups are in `crates/semantic-check/tests/contracts.rs` and
`crates/authoring-frontend/tests/static_check.rs`, with additional original control
examples now explicitly checked. Synthetic post-verification graph tests isolate
cycle/depth algorithms; they do not pretend a self-referential content hash was
constructed. Public exact-byte registry verification is tested independently.
The new core transport/structural entry points are tested as non-authoritative.

The Python checker has six groups: three complete package/schema/hash/static-report
cycles; syntactically lowerable but statically invalid sources; byte-pin/nominal
failures; unknown provider shapes; unsupported clocks/malformed flags; and local
documentation/license checks. It invokes the Rust adapter, not a second production
semantic implementation. Independent Python checks establish artifact properties,
not independent proof of every Rust semantic rule. Native/Wasm compilation is not
new browser execution or runtime trace evidence.

## Review questions and recommendations

1. **Separate static-stage report:** accept this boundary rather than calling the
   subset full admission? Recommend yes. `remaining()` and CLI `executable:false`
   make the unproved obligations explicit; neither is an execution credential.
2. **External support:** enable closed named-type/provider candidate lowering,
   while keeping all non-null timers and clock/calendar source resources unsupported?
   Recommend yes. Calendar format checking and provenance hashes cannot fill the
   deliberately unspecified clock-provider/trust contract.
3. **Next priority:** complete control/definite-output, information-flow/access,
   race/invalidation/reconciliation judgments and a supported clock proposal before
   treating full benchmarks as executable? Recommend yes, with separately scoped
   review and no implicit Phase 2 engine. General IR export/editing and user-study
   evidence remain required within the unchecked authoring item.

## Remaining gates and stop boundary

This completes the committed bounded implementation scope, not the aggregate
Roadmap deliverable. Full structural/link/semantic validation and stable rich
diagnostics, complete RP-01/03/08 examples, conformance/visual parity, safe protected
editing, user-study results, G1–G4 and Phase 1 exit remain open. Forty Partial
corpus scenarios retain their status. No visual notation or participant recruitment.

Pause for owner review of this ADR/PR before merge or another item. No successful
static report may be used as permission to plan, execute or certify host policies.
