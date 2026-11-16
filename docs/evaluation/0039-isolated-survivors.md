<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Isolated surviving-wait evidence

**Date:** 2026-11-06<br>
**Plan:** [frozen scope](0039-isolated-survivors-plan.md), `445b23e`<br>
**Critique:** [actual pre-change witnesses](0039-isolated-survivors-audit.md), `ab1e939`<br>
**Decision:** [ADR-0039](../decisions/0039-isolated-surviving-waits.md)

The synchronous invoke/repeat prerequisite now proves declared isolation for an
explicit direct-child follow-up with no storage, ports or nested scopes and only
data-free timer-free waits/finishes. Its management dependencies must be data-free,
its own closure terminal, and its entry must reach a wait-to-finish route.
Every declaration is inspected, including unvisited hazards. Existing ordinary
conflict/all-member rules and terminal-only parallel children remain unchanged.
A lazy graph/storage/parent index and per-check proof-template cache avoid
rescanning reused bodies; copying evidence consumes the same work budget.

`ConcurrencyReport::isolated_followups()` returns private-constructor typed
`IsolatedFollowup` records with creator/body/closure/follow-up/reachable wait IDs.
All six native composed views from concurrency onward publish `isolatedFollowups`
and its outstanding P6 host requirements. Record/final JSON is bounded before
stdout. The evidence contains no occurrence ID, accepted transfer or actual
settlement receipt, and grants no authority or execution. P6 ownership enforcement
can still block ordinary publication, and unproved irreversible races still block
planning. A wait-to-finish route is not a progress proof.

Seven Rust control groups cover invoke/repeat, exact metadata and immutable inputs,
shared reads/unvisited ancestor writers, initialized/uninitialized storage, nested
scopes, unvisited control and opaque human work, scope-management reads, finish-only
entry/nonterminal follow-up, unchanged split/fanout refusal, reused templates and
fresh revision rechecks. One internal budget test checks index work and cached
evidence copies against an exhausted cumulative budget. Existing finish-only
historical and all-creator lifetime negatives remain refused.

Six Python groups exercise all six native report views for invoke/repeat,
shared-read/finish-only/writer negatives, exact formatting/revision cycles, actual
RP-01 disclosure, a valid timed follow-up excluded after strict invalidation passes,
and 230 calls sharing a 100-wait template. That grammar-valid dense case lowers
but refuses `report-budget` before any stdout, rather than truncating evidence.
Two earlier positive isolated cases now assert conditional evidence and retain
explicit shared-input counterexamples, including a tautological shared read.
No negative is discarded without its premise being made explicit.

Actual unchanged RP-01 now passes all ten existing partial static stages. All six
later reports expose the three exact creators `run_checks`, `run_payment` and
`run_notice`; each names its own service/recovery boundary and outstanding host
premises. Source, bindings, resources, semantic revision and canonical IR bytes
are unchanged from the precise-link checkpoint. The manifest pins six new actual
reports. Historical evidence remains 88 records, strict ordinary requirements 84.
Full benchmark conformance, actual supported host normalization/ownership/progress,
exceptional protection, domain/irreversible safety, user/browser evidence and phase
exit remain open. All forty scenarios are still Partial. Do not equate the ten
partial stages with executable admission or completion of any scenario.

Local verification passed 291 Rust tests plus six compile-fail doctests, format,
workspace native lint, all five library Wasm lints, release portability/frontend
Wasm builds, native portability report/oracle, every CI Python oracle and exact
RP-01 regeneration/historical profile probes. The initial Python run found an old
isolated-case refusal expectation; the updated test now checks conditional evidence
plus a genuine shared-read refusal, and the remaining oracle sequence passed.
Logs: `/private/tmp/survivors-{workspace,clippy,wasm,native,oracles}.log`.
The new Python oracle is included in required CI; final-head CI remains required.

Separate completed-diff automated self-review checked static prerequisite ordering,
exact ownership/tree references, complete declaration inventory, data and management
dependencies, terminal/fresh-context conditions, structural routes versus actual
progress, unchanged parallel/effect checks, cache scope and copying charges, native
disclosure on every composed path, final-output limits and honest benchmark status.
No actionable issue remains. This is delegated automated review, not independent
or individual owner review. Audit all PR comments/CodeRabbit summaries and merge
only the passing exact head with explicit sign-off.

Next return to draft #46 and audit full A–D conformance/negative evidence and
remaining contract/host dependencies. Ten passing partial stages do not finish
the original complete-package deliverable. Keep the parent draft until its scoped
evidence and unresolved limits are explicit, then continue remaining Phase 1 work.
