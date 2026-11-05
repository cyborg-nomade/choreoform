<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0017: Check ordinary control and initialization conservatively

**Status:** Accepted — conservative ordinary-control/initialization stage<br>
**Date:** 2026-11-02<br>
**Decider:** Project Owner

## Context

Accepted ADR-0016 provides explicit static payload/type/dependency checks, with
control and definite outputs among its remaining judgments. A payload can pass
while missing a successor, reading an uninitialized cell or finishing without an
output. The owner authorized this next bounded slice after merging PR #21.
Full exceptional, information-flow, race and trusted-host proofs remain open.

The accepted IR already specifies ordinary outcome coverage, reciprocal joins,
acyclic control, fresh scope occurrences, snapshot assignment and output transfer.
It does not specify an evaluation order among local initial expressions. A useful
initial analysis must expose assumptions and conservative refusals explicitly.

## Decision criteria

1. Enforce existing ordinary-flow rules without revising frozen contracts.
2. Prove initialization under explicit contexts; never manufacture a value.
3. Bound graph work independently of graph-edge depth, native and Wasm.
4. Preserve the distinction between a partial proof report and executable admission.

## Decision

Add `check_control` / `check-control`, which first runs unchanged ADR-0016 static
checks and then checks all ordinary outcome/flow coverage, unique source/outcome
pairs, acyclicity and reciprocal same-scope split/fanout joins. Joins activate only
from their source obligation: direct ordinary flows and scope entry at a join
refuse. Validate every join-predicate outcome token against its source population;
do not claim predicate satisfiability, termination or liveness.

Analyze ordinary paths from the root entry with sets of definitely initialized
cells. Root input cells are explicit launch assumptions reported to the caller.
Initial expressions must read only entry inputs or available ancestors; local
initializer ordering is unproved. At merges, intersect reachable predecessor
states. Require every declared read, including policy/conservative dependencies,
before publishing any writes. Every reachable ordinary finish must supply all
scope outputs. Unreachable and exceptional work stays type/shape checked, but has
no invented availability entry context; report visited and unvisited declarations.

Invocations analyze fresh locals and exact input/output boundaries; output targets
must be distinct and the invocation write set must equal those targets. Propagate
only guaranteed ancestor initialization and explicit output transfers to callers.
Analyze each repeat body with fresh locals but credit no body-only initialization
to its zero-iteration continuation. Analyze split/fanout children from the fork
snapshot; supply the fanout item input explicitly. Credit no child-only
initialization to siblings or the join continuation, including `all` joins.

Return `control-checks-passed`, `executable:false`, the unchanged revision, root
input assumptions, coverage and residual obligations. A refusal ending `unproved`
means this conservative analysis could not establish initialization; it need not
mean the language forbids the program. The report is not a planner credential.
The earlier `check` command and report retain their original boundary.

## Options considered

| Option | Benefit | Cost/risk | Recommendation |
| --- | --- | --- | --- |
| Explicit conservative proof stage | Useful flow/output checks with inspectable assumptions now | Additional API; some valid programs refuse | Adopt |
| Include this subset silently in full admission | Single apparent pass/fail | Hides missing exceptional, protection, race and host judgments | Reject |
| Credit parallel writes or local initializer order heuristically | Accepts more examples | Scheduling/order assumptions can create false availability | Reject |
| Build complete scheduling/exceptional proofs first | Wider eventual coverage | Much larger review; delays ordinary correctness checks | Separate follow-up |

## Consequences

Linear ordinary graphs use iterative topological traversal; scope recursion and
predicate nesting remain depth bounded. A cumulative 1,000,000-unit budget charges
visits and retained/copied availability membership, in addition to existing static
and transport limits. Dense state propagation may refuse; this is a prototype
limit, not a measured scalability promise. Diagnostics use fixed codes/IR IDs.

Initialization means a typed value has been supplied or published on a modeled
ordinary path. It does not certify current validity after invalidation, authorized
access, control-sensitive information flow, competing writes, reconciliation,
exceptional outputs, authenticated root inputs, effect handling or atomic host
enforcement. Parallel/exceptional availability, complete RP-01/03/08 benchmarks,
supported clocks, stable source diagnostics, safe editing/export, near-English
user evaluation, visual parity, G1–G4 and Phase 1 exit remain open.

## Confirmation and acceptance

The [frozen engineering plan](../evaluation/0017-control-plan.md) precedes
implementation in `dc63a97`. [Evidence and review questions](../evaluation/0017-control-availability-checks.md)
describe positive/negative graph and initialization cases, bounds, source cycles
and native/Wasm checks. No frozen artifact or semantic version changes.

The Project Owner reviewed and approved [PR #22](https://github.com/cyborg-nomade/choreoform/pull/22)
on 2026-11-05. This accepts the separate conservative stage, its explicit input/
coverage assumptions and the stated follow-up priority. The three CodeRabbit
coverage findings were addressed in `c53e4d4`; no unresolved inline threads remain.
The owner also authorized continued implementation, review and merge within each
phase, with a report/approval boundary before starting the next phase. This
approval does not close any residual full-validator or Phase 1 gate.

1. [x] Owner reviews the stage boundary, conservative rules and next priority.
2. [x] Record owner approval before merge; subsequent work is explicitly authorized.
3. [x] Supply bounded implementation and reproducible evidence for review.
4. [ ] Complete the separately tracked full validator/host/benchmark gates.
