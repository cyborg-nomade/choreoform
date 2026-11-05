<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Ordinary parallel independence: frozen implementation plan

**Date:** 2026-11-05<br>
**Baseline:** `6067d9037c6dd77f01e2b7895673c00c9c066027` (PR #29)<br>
**Authority:** ADR-0018 delegation; accepted ADR-0008 / ADR-0013 P6–P7.

Add an explicit conservative ordinary concurrency stage after invalidation.
Index scope read/write and external-work summaries bottom-up, including declared
node reads/writes, initialization/input access dependencies and nested templates.
When promoting a child's footprint, exclude its own fresh local cells. Reusing a
template or a fanout body creates fresh local storage, not aliases of its data IDs.
Keep ancestor storage shared. Include unvisited declarations conservatively in
summaries; separately report unvisited fork sites without inventing activation.

For ordinary-covered split sites, compare identified child occurrence footprints;
duplicate template references still represent distinct occurrences. For fanout,
compare the shared footprint with itself without assuming zero/one items from a
literal collection. Refuse shared write/write or read/write hazards as unproved.
Also refuse overlapping external work when either side is human/opaque or a
non-read-only capability. A typed `guarded` policy, arbitrary Boolean invariant,
compensation or idempotency does not establish conflict/domain safety.

Only infer quiescence from a join predicate that requires all identified members
to have outcomes and terminal child closure contracts. Analyze monotone all/any/
threshold/and/or structure conservatively; fanout cardinality is unknown. Refuse
early-join or surviving reconciliation lifetimes as unproved, including invoked/
repeated bodies that could outlive ordinary completion. Do not assume cancellation
requests, retained ownership or true join predicates settle unfinished children.

Expose immutable revision-bound checked/uncovered fork evidence through library,
source and CLI, always non-executable. Bind refusals to actual creator/join sites.
ADR-0022 will record this supported boundary and alternatives. Budget indexing,
scope promotion, copies, predicate walks and pair comparisons. Preserve existing
wire/grammar/resources/dependencies and earlier stage contracts.

Test fresh-local template reuse, fanout locals, ancestor read/read and disjoint
writes, shared hazards, initializer-only reads, nested footprints, early joins,
reconcile lifetimes, arbitrary guarded-true rejection, external-work overlap,
unvisited coverage, source cycles and bounded work. Run required checks, review
separately, record delegated acceptance, require final-head CI and expected-head
merge. Runtime conflicts, domain invariants, exceptional/reconciliation concurrency,
host trust, clocks, complete benchmarks and aggregate Phase 1 gates remain open.
