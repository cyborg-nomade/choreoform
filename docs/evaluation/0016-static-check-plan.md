<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Static contract checking: preimplementation plan

**Date:** 2026-11-02<br>
**Baseline:** `9820346bdb848b34436c50621aa1689cfa14f5e3` (merged PR #20)<br>
**Authority:** Owner accepted PR #20's recommendations and authorized the next
item through review readiness. Freeze this engineering plan before implementation.

## Scope and authority

Implement the next stage of admission: strict supported artifact readers and
static type/expression/policy-payload judgments for ADR-0013's accepted suite.
Keep the result explicitly **static checks passed**, not full-definition semantic
admission or execution authority. Reuse existing envelope/structural checks;
do not reinterpret the historical illustrative registry or mutate frozen contracts.
Propose the separately named stage/API boundary in ADR-0016 with reviewable code.

Provide exact-byte local named-type/provider/calendar readers, complete closed
type/value and pure-expression checks, finite nominal invariants, typed policy
facts, reference-kind/visibility and dependency checks, and selected node/interface
type obligations. No implicit network, IDs, host clock, authentication or effects.
Clock-provider format/support is unspecified in the accepted suite: refuse non-null
timers, not accept arbitrary bytes as clock support. A calendar reader checks
closed tables/provenance pins, never generates intervals or certifies host trust.

Integrate explicit checking with the authoring candidate API/native adapter.
Enable supported named-type/provider candidate lowering only after readers and
transitive dependencies are implemented and tested. Keep lower/check/execute
authority distinct. Do not attempt a complete planner or silently claim all
information-flow, definite-output, race/invalidation or runtime obligations.

## Required observations

- Exact core pins and role separation, external identity/digest, duplicate/missing/
  corrupt resources, closed fields/kinds, bounds and cycle refusals.
- All closed type/value and expression operations, exact nominal identity and
  invariant/literal admission; typed dead branches and complete inferred reads.
- Parameter binding limited to actual fanout item sites; policy facts only in
  their typed contexts; no process reads/facts in nominal invariants.
- Provider outcome/result/effect consistency, calendar sorted finite intervals
  and exact provenance pins; unsupported clocks and unknown forms fail closed.
- Closed policy shapes and dependency closure including external protection,
  actor/capability references. No opaque payload is treated as dependency-free.
- Positive source-to-candidate-to-static-report examples, negative candidate/IR
  mutations, unchanged IDs/revisions/annotations, native/Wasm lint/build and all
  existing regression oracles. Reports/diagnostics must not leak source values.
- Document exact supported judgments and residual obligations, with reproducible
  commands; no browser/runtime/usability or complete benchmark claim.

## Completion / stop boundary

Deliver implementation, tests, ADR, evidence and roadmap updates in one review PR.
Complete this scope before handing back; do not merge it without owner approval.
Keep the near-English item, complete validator, executable IR aggregate, G1–G4,
full RP-01/03/08 evidence, forty Partial cases and Phase 1 exit open. No visual
notation, participant recruitment, semantic extension or Phase 2 engine work.
