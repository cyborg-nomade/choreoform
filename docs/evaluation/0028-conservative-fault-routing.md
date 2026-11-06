<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Conservative fault-routing evidence

**Date:** 2026-11-06<br>
**Plan:** [fault graph plan](0028-fault-routing-plan.md), `c6967c7`<br>
**Decision:** [ADR-0028](../decisions/0028-conservative-fault-routing.md)

## Implemented judgment

The new stage follows timing/ordinary checks and builds an indexed possible
handler-selection graph with active lexical contexts. It considers all declared
business-fault names without using retry eligibility as an emission bound. A
handler's local policies can catch body faults; propagation escaping an active
handler root excludes that failed selection while retaining outer frames until
they too are escaped. Global exclusions or scope-only identity are not substituted.

An iterative graph check refuses potential cycles as unproved, with an actual
scope faults path. Integer context/edge keys and borrowed public views keep names
interned. Every expansion/lookup/stack copy/edge is charged. The report counts
abstract contexts/known-name unhandled cases and separates routing coverage from
ordinary node coverage. It accepts no fault observation or host safety assertion.
Review also bounds the native adapter's expanded route metadata and final report
to 1 MiB before output; the core retains indexed borrowed views. Excess output
refuses `report-budget` without printing a partial report.

## Regression evidence

Eleven Rust groups cover empty/unvisited contexts, immediate failed-selection
exclusion through ordinary nested scopes, exact lexical precedence and no prefix
matching, nested active frames, different-key mutual/same-template cycles,
ordinary versus active-handler uses, one thousand uncalled scopes, 63 nested
handlers, explicit work-budget refusal, spelling versus actual engine class and
retry eligibility versus emissions. Earlier timing stages pass before the new
cycle/budget refusals. No real handler is invoked.

Three Python groups cover both source-profile grammar/schema/JCS/revision/report/
format cycles, exact metadata and still-uncovered handler data contexts; cycle
refusal after timing success; documentation/licenses. They exercise the native
adapter, not an independently implemented semantic selector or actual dispatcher.

## Verification

Passed 215 Rust tests plus five compile-fail doctests, format/native lint,
semantic/frontend Wasm lint and release frontend Wasm build. All relevant
artifact, authoring/profile/frontend and static/control/protection/lifecycle/
invalidation/concurrency/clock/timing/calendar/retry/settlement/value-conformance
and new fault-route Python oracles passed. Temporary local logs:
`/private/tmp/fault-routes-rust.log`, `/private/tmp/fault-routes-review-checks.log`,
`/private/tmp/fault-routes-all-oracles.log`.

Separate completed-diff automated self-review checked lexical/active selection
identity, outer-frame exclusion, ordinary/handler role separation, unused policy
names versus actual-use selection coverage, indexed expansion/graph budgets,
stable value-free failure paths and source/native/Wasm parity. Review added an
explicit concurrent-occurrence residual and bounded native output before metadata
expansion can grow without limit. No actionable finding remains. ADR-0028 is
accepted under delegation; require final-head CI/expected-head merge. No independent
reviewer, individual owner review or actual host/browser/process/user evidence is
claimed.

## Residual obligations

The analysis may over-refuse valid restricted-emission or mutually exclusive
contexts; an explicit future proof/profile can improve precision. It cannot
infer that all named business faults are truly possible or authenticate a class
from its name. Actual engine faults remain uncatchable, and the target must
enforce that class boundary. Other names are unhandled, without wildcard/default
matching. Actual handler entry data/access/invalidation, fault identity/revisions,
current authority, cancellation, reconciliation and accountable ownership remain
unproved. Prior timing/host obligations, full benchmark/conformance/cross-form
and actual-user evidence remain required; no Phase 1 gate is closed.
The modeled stack is one lexical propagation chain. Concurrent fault events,
handler occurrence activation and their lifetime/ownership interactions remain
explicitly unproved; abstract context deduplication supplies no occurrence proof.
