<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Owned local follow-up plan refinement: retain protection provenance

**Date:** 2026-11-07<br>
**Baseline:** frozen plan `afed903`; implementation still unpublished on
`codex/owned-local-followups`<br>
**Authority:** ADR-0018; refinement frozen before its implementation

Completed-diff inspection of the initial draft identifies an incorrect premise:
zero inherited data values does not mean zero inherited source/control provenance.
Analyzing a surviving body with empty protection origins could publish a low-label
capture after a more restricted creator/owner control context. Local storage
isolation and source-only capture comparison do not prove that transfer.

Replace the original plan's empty-protection-origin choice. Retain a bounded union
of ordinary creator/owner scope entry and internal source/control influence in the
already composed protection report, including partial child influence and distinct
provider/clock roles. Enable this extra collection only for definitions with
explicit captures, preserving old-profile work/behavior. Reuse these immutable
revision-bound label references for all selected owner contexts; do not rerun static
or ordinary validation to reconstruct them.

Conditional entry still assumes zero inherited *values*. Convert retained cell /
ordinary historical influences to a distinct private conditional protection-
provenance origin. It references the immutable definition's label, not a live cell
value, permission cache, ordinary completion or historical-value availability.
Preserve provider source envelopes as distinct origins. Propagate all these labels
through conditional initialization/writes/branches using the existing P2 engine.
Do not describe them as ordinary historical completion or fabricate a runtime
receipt. Require an explicit live P6 premise: the owner retains authenticated,
occurrence-bound source/control protection provenance through handoff/disposal,
without inheriting ancestor values. Actual current access stays mandatory.

These immutable provenance labels are not mutable ancestor data dependencies and
cannot satisfy a read or initializer. Current local reads still require fresh
availability and strict conditional invalidation. No conditional report establishes
that the host has retained the required provenance. Preserve all source/authority /
cutoff/settlement and observable-channel gates. If inherited labels cannot transfer
to a local destination, refuse the conditional proof, even when its direct source
and destination agree. Add a restricted creator-control/public-capture regression,
plus shared-context/cumulative-work checks, before publishing.
