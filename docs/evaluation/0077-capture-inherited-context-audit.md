<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Capture inherited context: pre-correction critique

**Date:** 2026-11-11<br>
**Frozen plan:** `0a04032`, [scope](0077-capture-inherited-context-plan.md)<br>
**Dependency:** #91 final-head CI `43038900965`, review `5860323505`, signed merge
`23d37155215614f020e9a65e1defc6ba87dded6c` before probes<br>
**Correction decision:** [ADR-0077](../decisions/0077-fixed-all-join-history.md)

## Observed baseline, before production correction

A bounded private cfg(test) recorder at the existing requirements refusal records
one declaration-only tuple per thread (up to64 ancestor IDs, explicit completeness
flag). Both actual strict/advisory pipelines on the complete current RP-03 candidate
produce exactly:

| Field | Actual value |
| --- | --- |
| Use site | `capture_activity` |
| Live influence cell | `authorize_call_value` |
| Cell lexical scope | `accept_payment` |
| Use lexical scope | `capture_service` |
| Use ancestors | `capture_service`, `order` |
| Result | `invalidation-context-unproved`, `/body/nodes/capture_activity` |

The cell is the invocation output in the authorization branch. `authorize_call`
publishes its typed child output there; `authorize_call_record` reads it to publish
the optional root authorization record. The branch reports `authorization-recorded`.
The paired `start_acceptance_gather` uses direct `all` over all three fixed split
members, before later fulfillment/capture. It is not a newly invented private
read or permission in the capture expression.

Two complete raw baseline groups pass static/control/protection, then confirm
that both a direct all-join and partial any-join leave private branch influence
live today. A synchronous invoke control passes. An unvisited descendant writer
keeps even the all-join context unproved. Together with the whole-case tuple group,
these tests passed before the correction. Log: `/private/tmp/capture-context-baseline.log`.
This critique describes that actual baseline; later acceptance is separate evidence.

## Contract and implementation distinction

ADR-0037 deliberately proves exact-owner/no-nonlocal-writer history only at
synchronous invoke/repeat completion. Its general split/fanout summaries stay live.
The implementation collects each split child completion summary immediately and
unions it into the parent context; the paired join currently supplies no additional
completion refinement. The invalidation guard correctly refuses the resulting
private **live** cell. Removing that lexical guard or its dependency would be wrong.

The existing [IR join contract](../ir/definition-v0.1.md#joins-and-dynamic-membership)
defines direct `all` as every member having a terminal outcome in the set. A fixed
split supplies an exact nonempty child membership. Validated control graphs forbid
ordinary flows into a join and join-as-entry; each join has only its reciprocal
split/fanout activation. Thus a direct all-join of a fixed split provides a stronger
conditional completion boundary than the earlier raw split summary. An any/threshold /
composite predicate or dynamic fanout is not this proof. Terminal member outcomes
still do not discharge arbitrary surviving obligations or give live values.

## Chosen bounded correction and required counterexamples

Extend the existing historical-origin proof at the matching **direct all-join of
fixed split** only. Retain the split's pre-child live snapshot. At the join, classify
only cells owned by each exact completed branch, with no declared nonlocal writer
and absent from that inherited live snapshot. Reuse the same writer inventory,
protection labels and conditional closure/snapshot premise; do not freeze arbitrary
descendants. Charge snapshot storage/lookups, member traversal and origin visits to
the existing cumulative work budget. Protection-only analysis/report bytes stay fixed.

This is an expansion of a conservative proved subset under unchanged terminal /
closure/value-copy meanings, not a new policy, source syntax, IR/core version,
immutable/safe flag or actual host receipt. Retain partial joins, fanout, possible
nonlocal/unvisited writers, unrelated descendants and inherited live template origins
as negatives. Check historical boundary/site/scope/closure and child-entry records,
original protection labels, mutable ancestor coverage and bounded dense work. Raw
split summaries themselves remain live. Later concurrency/lifetime/authority gates
are independent and must retain their actual results.

Before updating any current benchmark result, preserve every changed old artifact
and label its then-current refusal honestly. Prefer unchanged source/candidate /
resource/guard bytes where no source correction is needed; any actual new advisory
or checking result must come from the real whole pipeline. Do not promise the next
first diagnostic, executable support or Phase 1 exit. Forty/all four scenarios,
G1–G4/parent #46/full/optional/RP-08/validator/near-English/managed protected accessible
text-visual/actual users/hosts/browsers/independent/phase gates remain open.
