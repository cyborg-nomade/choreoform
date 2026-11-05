<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0022: Check conservative ordinary parallel independence

**Status:** Accepted under Project Owner delegation; effective upon PR merge<br>
**Date:** 2026-11-05<br>
**Decider:** Codex under Project Owner delegation, ADR-0018

## Context and criteria

Accepted semantics require explicit conflict handling, safe irreversible races
and ownership-aware closure. Ordinary control checking does not prove that two
children can safely update shared data, and a true join does not dispose of
unfinished work. Repeated templates/fanout use fresh local storage; treating data
declaration IDs as global runtime addresses would invent conflicts between locals.

Implement a bounded, useful independence subset without claiming that arbitrary
Boolean invariants prove safety. Include initialization, policy and inherited
control dependencies, distinguish shared ancestor storage from fresh locals,
and retain explicit runtime/domain/exceptional obligations.

## Decision

Add `check_concurrency` / `Candidate::check_concurrency` / CLI `check-concurrency`
after invalidation and earlier stages. Build scope summaries bottom-up from
declared reads/writes, initialized/input cells and their access dependencies,
nested lexical templates, and visited incoming control cell origins. Union
repeated contexts. Include unvisited declarations conservatively in summaries;
report ordinary-unvisited fork sites separately. A nonvisible incoming completion
origin refuses `scope-context-unproved` at the scope instead of aliasing or
discarding it. The private context cache confers no authority or live identity.

Promoting a child's summary removes that template's own fresh storage, retaining
ancestor storage. Nested promotions remove their respective fresh locals. Split
branch identities still count distinct occurrences when templates repeat. Fanout
compares the shared footprint with itself, without guessing zero/one members from
a literal collection or scheduling limit. Reject shared write/write or read/write
hazards as `parallel-storage-conflict-unproved`.

External work has an independent boundary: human work is opaque; capability work
uses its exact validated effect class. If either side is opaque/non-read-only and
the other can observe/perform external work, refuse `parallel-effects-unproved`.
Two read-only declarations can pass this particular check, conditional on truthful
provider/host enforcement and the remaining obligations. Disjoint result cells,
compensation, idempotency or an arbitrary guarded-true policy do not certify that
external effects commute or preserve a domain invariant.

For split/fanout, require a predicate that implies every identified member has
reported an outcome, plus terminal child closure contracts. `all` suffices;
`any` suffices only for one fixed split member; a threshold suffices for at least
the fixed split population; `and` needs one sufficient term, `or` every term.
Fanout cardinality is unknown. This implication does not prove that a predicate
can become true or that membership is actually sealed. Otherwise refuse
`parallel-lifetime-unproved` at the join. Invoked/repeated bodies and parallel
children with surviving reconcile closure refuse `scope-lifetime-unproved` at
the creator. Do not mistake a cancellation request or retained ownership for
settlement. These are conservative supported-analysis boundaries, not a ban on
all valid early joins or reconciliation designs.

Use accumulated read/write indices and external-work flags rather than a
quadratic scan of branch pairs. This detects the same pair hazards by union
membership. Charge summary/context members, copies, visibility/predicate walks,
lookups and promotions under existing budgets. The report privately wraps prior
evidence, binds its revision and exposes checked/uncovered fork declarations;
the CLI retains earlier coverage/input assumptions and `executable:false`.

## Options considered

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Conservative independence, quiescent-contract boundaries, explicit unproved refusals | Useful checkable subset with no safety flag or migration | Some valid guarded/early/reconcile programs need more precise proofs | Adopt |
| Compare every data declaration as globally shared | Simple | Falsely aliases fresh local occurrences | Reject |
| Compare explicit node sets only | Smaller | Misses initialization, nested and pre-fork control dependencies | Reject |
| Accept disjoint result cells, guarded true, compensation or idempotency as safety | Broad apparent acceptance | None proves domain/external conflict safety | Reject |
| Full occurrence/invariant/lifecycle analysis first | Greater precision | Larger follow-up requiring stronger rules/evidence | Follow-up; preserve partial-stage boundaries |

## Consequences and limits

The supported stage can establish absence of modeled ordinary shared-storage
hazards at checked forks under accepted contracts. It does not prove arbitrary
invariants, actual scope disposal, live sealing, receiver acceptance, current
authority, exceptional/reconciliation concurrency, non-data invalidation or host
atomicity/trust. Full validator, supported clocks, complete benchmarks/conformance,
source/visual parity, actual user studies and Phase 1 exit remain open. No syntax,
wire, resource version, dependency, execution interface or safe Boolean changes.

## Confirmation and action items

Follow the [frozen plan](../evaluation/0022-concurrency-plan.md), committed
`6fd6161`, and [evidence](../evaluation/0022-ordinary-concurrency.md). Review found
and fixed missing inherited pre-fork control reads, added explicit nonvisible
context refusal and replaced pair scans with accumulated indices. Preserve those
regressions and require final-head CI before expected-head merge.

Accepted after separate automated self-review under the owner's 2026-11-05
delegation. Local verification passed 158 Rust tests plus five compile-fail
doctests, native/Wasm checks and relevant stage oracles. Required final-head CI
remains a merge prerequisite. No independent human or individual owner review
of this ADR is claimed.

1. [x] Finish implementation, regression/source cycles and native/Wasm checks.
2. [x] Record separate automated self-review and delegated acceptance.
3. [ ] Continue lifecycle/clock/full-validator and aggregate Phase 1 gates.
