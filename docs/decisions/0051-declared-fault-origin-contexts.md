<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0051: Retain declared emitting sites and complete handler contexts

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-07<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

The history type/claim foundation does not supply handler entry. Existing routing
retains a scope origin and last active selection, which cannot describe complete
incoming emitting-site/source contexts or all failed-handler exclusions. Before
changing the scope wire, follow [frozen prerequisite plan](../evaluation/0051-declared-fault-origins-plan.md),
`d5c227d`. This dependency was identified by inspecting the current routing code.

## Decision

Add a separate raw SDK `check_fault_origins` / `FaultOriginReport`, reusing one
existing timing and routing pass. Preserve exact nearest lexical selection, local
handling before escape, failed-selection exclusion and routing-cycle refusal.
Retain the old FaultReport as a nested result with unchanged abstract routes and
counts. Existing check_fault_routes collects no origin index/stacks/rows, and its
native reports, quotas, work accounting and contracts remain unchanged.

Index every activity declaration once, including unsupported, unhandled, unvisited
and unexplored cases. Expose its site/scope, ordinary syntactic reachability,
abstract scope exploration flag and exact retained core-0.6 general source use where
available. Human and missing/legacy general-source contexts remain explicitly
unsupported; a matching name cannot assign another provider's source. Scope and
node flags are separate and neither implies actual activation or current values.

For explored scopes, match supported-source routes only to the exact source's
emission names. Conservatively explore every known name for unsupported activities,
including ordinary-unvisited ones. Expose selected handler/fault and complete active
selection stacks before and after existing selection/exclusion. Count declared
unhandled origin cases separately, including an unsupported subset. Unexplored
activity scopes remain uncovered. No null source or absent route proves no fault
can occur; no engine fault becomes catchable from this metadata.

Rows borrow stable source/site/selection indices in the retained report. Intern
stack vectors once, avoiding per-route metadata/string copies. Charge indexing,
every comparison, cached stack lookup, vector copies, row storage and unhandled
counts to the same cumulative routing allowance. Any failure returns no partial
report or truncated coverage. Legacy collection off has no origin walk/budget cost.

## Alternatives and consequences

| Option | Benefit | Limitation / decision |
| --- | --- | --- |
| Separate declared site/full-context report | Exact declaration identity and visible unsupported cases | Adopt as prerequisite; no actual event or entry proof |
| Use existing scope/top-selection routes to initialize history | Reuses short API | Reject: drops complete origin/context information |
| Infer provider source from fault spelling or old retry metadata | Fewer explicit bindings | Reject: conflates source identity, general emissions and eligibility |
| Duplicate static metadata per route or rerun admission | Straightforward implementation | Reject: unnecessary copies/work and inconsistent evidence ownership |
| Treat ordinary-unvisited sites as impossible, or null source as no faults | Smaller cases | Reject: loses conservative declaration coverage |

[Evidence](../evaluation/0051-declared-fault-origins.md) checks actual SDK source
relations, nested selection/exclusion, unsupported/unvisited/uncovered contexts,
cumulative budgets, legacy compatibility and independent finite vectors.
This is delegated automated self-review, not independent/human or personal owner
approval. Declared site relations are not actual emitting occurrences, source truth,
cause/input revision correlation, full protection snapshots or current authority.

Next freeze the explicit handler-local entry wire/core contract. Require complete
incoming supported-source and control/provenance contexts, actual nearest live
selection, fresh immutable local initialization/current access and strict invalidation.
Unsupported/uncovered origins must not gain fabricated history or body coverage.
Then readable entry authoring and permitted owned continuation/cutoff/all-settlement.
No current wire/core/source/native command/package bytes change here. Parent #46
stays draft, all forty scenarios Partial, and complete validator/benchmark/general
authoring/visual/protected editing and actual host/user/browser/independent/phase
gates remain open.
