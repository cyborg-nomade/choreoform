<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0035: Derive finite reads through provider recipient metadata

**Status:** Accepted under Project Owner delegation; effective upon integration
into main through the parent benchmark PR<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

The [RP-01 checkpoint](../evaluation/0034-rp01-checkpoint.md) reproduces incompatible
provider-result and destination capability labels. Making the intended shared
fictional recipients explicit instead creates `policy-cycle` under core 0.2:
a permitted capability's wrapper names its own provider/result protection.
The cycle is finite recipient metadata, not a recursive predicate invocation.
The old contract nonetheless explicitly refuses policy reference cycles; changing
that behavior requires a separately pinned version. Removing dependency edges
would hide real protection/authority/effect/wait/clock reads. Denying downstream
provider reuse would obstruct required correction/recheck and composition.

## Decision

Adopt [core profile 0.3.0](../dialects/core-metadata-v0.3.md), inheriting exact
core 0.2/0.1 artifacts and all existing payloads/semantics except the stated read
closure rule. For explicitly selecting policy records, references derived from
provider result protection are metadata edges. Other references remain strong;
retain an edge in both sets when both origins exist. Validate all refs, lexical
visibility, expected kinds, bytes, types and protection envelopes as before.

Keep the strong graph acyclic and depth-bounded. Compute the least finite union
of direct reads over both edge sets with iterative SCC condensation. A cyclic
component is permitted only if every member selects core 0.3 and its strong edges
are acyclic. Legacy/mixed cycles refuse; old-only definitions retain their existing
checker path. Bound the component graph depth as well as the strong graph; charge
vertices, edges and set copies against the same total semantic work budget.

A pure typed read-closure helper exposes sets and work count for later compiler
reuse. It validates no contracts or host facts and grants no admission; callers
validate eligibility and charge against their remaining budget. No runtime
predicate evaluator, permission cache, dispatch or general graph recursion is added.
Source profiles retain their existing core pins; explicit authoring support is a
separate dependency before RP-01 can adopt this contract through source lowering.

## Alternatives and consequences

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Versioned finite metadata closure, with strong DAG and legacy boundary | Allows declared provider-result reuse while preserving complete reads | Adds graph analysis and profile maintenance | Adopt |
| Drop recipient references/self edges | Small implementation | Omits real authority/effect/wait/protection dependencies | Reject |
| Allow every policy cycle by default | Simpler reference rule | Reinterprets old contracts and unsupported recursive execution | Reject |
| Remove permitted capabilities from all result/data labels | Fits old checker | Denies required downstream use/correction or tempts unauthorized transfer | Reject |
| Runtime-only dependency discovery | Defers analysis | Loses portable static read evidence and invalidation checks | Reject |

The resulting information-flow check still does not authenticate a provider or
prove least privilege. The raw promoted RP-01 graph with coherent business/clock
labels passes static/control/protection checks but still refuses private-child
invalidation context. Surviving lifetime, exceptional protection, full validator,
benchmark execution, protected editing, real host/browser/user and Phase 1 gates
remain open. This is no full reimbursement or phase approval.

Follow [frozen plan](../evaluation/0035-provider-metadata-plan.md), `52596b0`.
[Evidence](../evaluation/0035-provider-metadata.md) records exact pins, actual raw
checks/refusals, independent finite-union vectors, full checks and separate
completed-diff automated self-review. Accepted under ADR-0018 delegation; no
independent or individual owner review is claimed. Require passing final-head CI,
comment audit and expected-head merge into the still-unmerged parent.
