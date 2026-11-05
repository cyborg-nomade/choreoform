<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0019: Track ordinary control protection and access obligations

**Status:** Accepted under Project Owner delegation — effective upon merge<br>
**Date:** 2026-11-05<br>
**Decider:** Codex under Project Owner delegation, ADR-0018

## Context

ADR-0016 checks direct declared-read/write transfers. ADR-0017 checks ordinary
flow and initialization. A decision reading protected data can nevertheless
select a branch that writes a less-protected literal: the write has no direct
read, so the existing checks do not establish the accepted control-dependency
rule in ADR-0013. Initializers and invocation input transfers also need protection
judgments. Neither static predicates nor resource digests authenticate access.

## Decision criteria

1. Enforce accepted no-declassification rules across ordinary control paths.
2. Expose access obligations without guessing principal/purpose/current facts.
3. Preserve old explicit stages, frozen resources and honest residual gates.
4. Bound graph, provenance and protection-metadata comparison work.

## Decision

Add explicit `check_protection` / `check-protection` after the existing static and
control checks. Track a monotone set of cell/provider protection origins through
ordinary paths. Union at merges; retain origins after rejoining. Node reads,
access-policy dependencies and published results influence subsequent completion/
outcome context. Child calls inherit the context; completion summaries are unioned
back into invoke/repeat/split/fanout continuations, conservatively including zero
and parallel cases. Include provider result protection even with null publication.

Check inherited control origins plus direct dependencies against every destination
using identical sensitivity and resolved protection-policy identity, and narrowing
purpose/participant/capability sets. Check initializer dependencies, caller argument
to child input transfers and output/finish transfers. No constant-guard pruning,
postdominator-based release, policy implication or declassification is inferred.
The analysis can reject valid programs requiring a more precise proof.

Enumerate immutable revision-bound access obligations by site, cell and operation
(`read`, `write`, `transfer`). Close access-metadata data dependencies to a fixed
point; refuse any modeled access with both participant and capability sets empty.
Actual principal, purpose, capability membership, actor authority, predicate truth,
current revisions and atomic enforcement are still required host/full-validator
judgments. Initial access-policy availability is separately unproved. Obligation
enumeration is not access authorization, an executable plan or evidence of host
support. No authenticated caller-supplied "safe" flag is invented.

Reports say `protection-checks-passed`, `executable:false`, bind the unchanged
revision and expose access obligations, transfer count, control coverage/input
assumptions and residual judgments. Earlier `check` and `check-control` contracts
retain their boundaries. Shared direct transfer code preserves existing codes;
work accounting now charges access-set membership comparisons as well as pairs.

## Options considered

| Option | Advantages | Costs and risks | Outcome |
| --- | --- | --- | --- |
| Conservative origin propagation and explicit access obligations | Detects ordinary implicit transfers; separates metadata proof from real authority | Some valid programs refuse; host/exceptional proofs remain | Adopt |
| Direct reads/writes only | Smaller checker | Misses protected branch choice and input/initializer transfers | Reject |
| Drop protection at branch merge or guess predicate equivalence | Accepts more programs | Can silently declassify control facts without accepted authority | Reject |
| Treat static predicates/digests as access permission | Appears to close admission | No actual principal, purpose, current snapshot or trusted enforcement | Reject |

## Consequences

Origin and graph traversal are bounded by cumulative 1,000,000-unit work budgets,
including transfer-envelope member comparisons, with depth 64 and the existing
transport/resource bounds. Topological graph traversal is iterative. Resource
provider metadata uses shared cached objects rather than cloning full resources
per use. These are refusal bounds, not a scalability or security certification.

Exceptional and observable channels (including effect dispatch, timing, actor/
diagnostic presentation and correlation), complete access authorization, initial
access-policy availability, race/invalidation/reconciliation, supported clocks,
trusted host/atomic enforcement, full benchmarks, stable diagnostics, safe export/
editing, visual parity and user evaluation remain open. Covered ordinary data
transfers do not prove whole-process noninterference. Phase 1 remains incomplete.

## Confirmation

Follow the [preimplementation plan](../evaluation/0019-protection-plan.md), committed
as `f5c4db8`. The [evidence](../evaluation/0019-protected-ordinary-transfers.md)
includes ordinary branch/merge, initializer/input, nested/parallel and unpublished
provider-result regressions; access obligations, bounds, source cycles and native/
Wasm checks. Test earlier-stage success before each new refusal.

## Acceptance and action items

Accepted on 2026-11-05 under the Project Owner's explicit phase delegation in
ADR-0018. A separate automated self-review checked completed code/claims, fixed
destination policy-dependency and wide-envelope work-accounting gaps, and added
regressions. Local verification passed; required CI at the final head remains a
merge prerequisite. This does not claim personal owner or independent human review.

1. [x] Complete implementation, tests and a separate automated self-review pass.
2. [x] Record acceptance under delegation; require final-head CI before merge.
3. [ ] Continue the remaining full-validator/host/benchmark and Phase 1 gates.
