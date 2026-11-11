<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Compensation reference audit and critique

**Date:** 2026-11-11<br>
**Baseline:** `f628d0eb50dd490be35288145f82c58278da068a`, draft #46 after #83<br>
**Frozen plan:** `e86e2fc`, [scope](0069-compensation-reference-audit-plan.md)<br>
**Decision:** [ADR-0069](../decisions/0069-compensation-reference-direction.md)

## Contract and implementation findings

[P1](../dialects/policies.md) requires lexical visibility and a prior capability
activity; an actual eligible occurrence/new authority/new logical effect remains a
runtime obligation. [ADR-0008](../decisions/0008-core-process-semantics.md) calls
compensation new explicit work, not guaranteed restoration. Raw `definition/policies.rs`
calls the ordinary `visible(scope, "nodes", ...)` before its activity/mode check.
The frontend's `Ledger::resolve` searches ancestors; qualification checks that
ancestor chain, rather than descending into a private sibling. Global ID presence
does not override either rule. Thus the measured full-case reference failure is a
contract dependency in both layers, not a source typo or source-only bug.

The separate C11 owned-work check requires `compensates == incoming source` when
present. Static declaration acceptance alone does not admit a different owned
compensation source. The existing finite-work matrix separately covers all three
own effect classes with/without purpose and read-only/irreversible original classes;
neither purpose nor class proves an actual business reversal or eligible effect.

## Actual bounded evidence

The raw SDK fixture extends an existing C11 synthetic finite-work declaration with
an unused work rule and a sibling terminal scope. It keeps all existing completion
predicates, providers, protection, capture/request bindings and flow. Every altered
policy scope has a passing unlinked static baseline. These are small isolated
declaration tests, **not** direct-IR generation/admission of full RP-03. The readable
fixture independently adds an unused sibling rule/scope to the source-0.15 promoted
small fixture, with a real grammar/ledger/static positive baseline.

| Case | Actual result |
| --- | --- |
| Raw same-scope or ancestor prior | Static check passes |
| Raw prior in child or sibling | `visibility` |
| Raw missing or foreign-definition ID | `reference` |
| Raw visible finish/invoke target | `compensation-target` |
| Raw private finish target | `visibility` before target-kind check |
| Raw data ID supplied as node identity | `reference` |
| Raw same-site work policy link | Static check passes; no prior dynamic occurrence established |
| Raw linked owned activity changed from incoming source to itself | Static passes, concurrency refuses `owned-continuation-compensation-source` |
| Raw sibling private expression read | `cell-reference` |
| Raw cross-scope outcome flow | Earlier `structure` refusal |
| Readable same/ancestor activity, exact ancestor qualification | Lower/static pass |
| Readable same-site adjustment | Static passes, concurrency refuses the same source restriction |
| Readable unqualified/fully qualified sibling or foreign activity | `reference-missing` with exact source-byte span |
| Readable visible activity with incorrect qualification | `reference-qualification` with exact span |
| Readable missing / actor where node expected | `reference-missing` / `reference-kind` with exact spans |
| Readable visible finish node | Lower passes, SDK `compensation-target` |
| Duplicate declaration name in same scope | `binding-duplicate` at 0..0, no candidate |
| Sibling private value/outcome literal in valid grammar | `reference-missing` at the private reference |

The outcome literal test independently recognizes its grammar before lowering;
the cross-scope raw flow has an earlier structural refusal and is not described as
a semantic outcome-visibility proof. Initial test expectations for private raw reads,
duplicate names and cross-scope flow were corrected to the real existing diagnostic
codes, without changing production behavior. The fixture's literal-true completion
is inherited synthetic input; no real predicate was replaced to obtain acceptance.
No new live observation, retained value, authority or dispatch exists.

The full promoted RP-03 still stops at `reference-missing at 60937..61023`, empty
stdout, as covered by the existing source-capacity regression. The small raw tests
do not predict every later full-case refusal. All committed RP-01/RP-03 bytes stay
fixed and all current/historical generator checks remain required.

## Recommendation and next dependency

Publish a new exact raw core contract changing only P1 prior-activity identity
lookup within the same definition. Preserve static kind checks, old core behavior,
ordinary lexical privacy, budgets, C11 immediate-source proof and all live eligible
occurrence/receipt/authority/new-key/ownership/settlement requirements. Follow it
with an explicit readable profile requiring exact full qualification for nonlexical
prior identity. Do not treat that link as readable access to target data/outcomes or
an actual occurrence selector. Freeze each implementation plan before code.

Hoisting benchmark internals, deleting a purpose link, broad global name lookup,
silently mutating C11/source0.15 or relaxing actual owned-source matching would hide
the dependency or change meaning. General aggregate ownership and possibly-completed
compensation remain later separate work. Source capacity and exact qualification do
not solve near-English usability; factoring, managed stable identities, protected
accessible summaries/editing and text-visual cycles remain explicit Phase 1 items.

## Validation and review

Five raw SDK groups and five Python/native groups cover these boundaries. Local
verification passed 562 Rust tests plus six compile-fail doctests, pinned workspace /
five library Wasm lints, release portability/frontend Wasm builds, native portability
and all 65 Python CI commands. Separate completed-diff delegated review checked
correctness, privacy/security, bounded work, scope and evidence; no remaining
actionable audit-scope issue. Required final-head CI/comment audit precedes merge.
Logs: `/private/tmp/compensation-reference-{workspace,verify}.log`. CodeRabbit skips supply no review evidence; agent review is automated,
not independent or personal owner review. Compilation is not browser execution.

Forty scenarios remain Partial; G1–G4, parent #46 and complete validator/full/optional
benchmark/RP-08/near-English/identity-managed protected accessible text-visual/actual
user/host/browser/independent/phase gates remain open. This is an independent contract
dependency, not an external blocker or phase report. No Phase 2.
