<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Full benchmark readiness audit

**Date:** 2026-11-06<br>
**Baseline:** `dc529b0a9ce639601661fc0833702786ec663d24` (PR #44)<br>
**Plan:** [frozen audit plan](0034-benchmark-readiness-plan.md), `c54365a`<br>
**Evaluator:** Codex under ADR-0018 delegation; author/evaluator are the same agent.

## Finding

The project has executable checks for many isolated judgments and lowerable small
source packages, but still lacks complete RP-01, RP-03 and RP-08 packages. None of
the twelve target acceptance scenarios is now demonstrated end to end. All forty
adopted RP scenarios remain Partial in [current accounting](0034-corpus-accountability.md).
The C01–C36 dialect examples and 69 value-expression vectors are different sets;
their counts or test success must not substitute for the forty corpus scenarios.
G1–G4 and Phase 1 remain open. No actual user, host/browser/process execution or
independent review is supplied by this audit.

The next substantial implementation should begin RP-01's complete data/resource
and graph package, preserving all four scenarios and alternatives, with exact
current-stage results and remaining refusals. Do not relabel the small manager
review fixture as reimbursement or call an inventory of future nodes a package.
Cross-case aggregation, fault continuation and surviving correlation must be
resolved explicitly where the full graphs need them; they cannot be hidden in a
provider returning an unexplained approval or safety Boolean.

## Verified implementation boundaries

| Boundary | Current accepted/implemented evidence | Consequence for full packages |
| --- | --- | --- |
| Values and pure computation | Exact closed descriptors, nominal invariants, pure AST checking and 69 conformance vectors; [value rules](../dialects/values-expressions.md) | Money/currency, request/evidence/revision and per-item records need actual complete descriptors/fixtures; shape is not provenance or approval |
| Local invocation | Exact input/output maps, atomic typed protected output transfers; [binding P1–P4](../bindings/README.md#4-parameters-and-invocation) and static/control tests | Can compose a child result explicitly. No implicit sharing or arbitrary parent access to private child storage |
| Split/repeat/fan-out | [Binding encoding restrictions](../bindings/README.md#6-compatibility-and-follow-through): split/repeat no ports; fan-out one item input, no outputs; [source grammar](../authoring/candidate-a-v0.3.ebnf) | No generic collection of child output bundles. Explicit disjoint ancestor cells can model a fixed finite branch set subject to checks; they do not implement dynamic keyed result aggregation |
| Ordinary availability/flow | Creation footprints, initializer/input/output dependencies, ordinary protection/invalidation and conservative independence | Full correction/review/dispatch paths still need actual dependency sets and combined scenario evidence; cross-context or irreversible conflicts can remain unproved |
| Faults | Exact nearest selection, active exclusion, conservative cycle checking, conditional handler availability; [P4](../dialects/policies.md#p4-retry-and-fault-routing) | No readable typed fault-data port. A handler's ordinary finish must not be invented as an ordinary outcome of the failed activity. Specify any required continuation explicitly before relying on it |
| Timers/retries | Exact clock/calendar/core 0.2.0 profiles, protected source origins, explicit retry clock/basis and host requirements | Need actual example resources and due/basis rules. A timer cannot supply a human decision; accepted stamps/current authority remain host obligations |
| Reconciliation | Own-scope direct follow-up/wait links, source 0.4.0 endpoint binding, conditional zero-inherited-value body checks | No automatic retained parent values, obligation payload, accepted live handoff, exact request correlation or multiple-obligation completion proof |
| Cancellation/races | Monotonic policy contract, receiver authority dependencies, ordinary conservative concurrency; [P6/P7](../dialects/policies.md) | A Boolean invariant alone proves no irreversible safety. Full cutoff, guard coverage, current facts, accepted settlement and exceptional invalidation are still needed |
| Tooling/scale | Strict packages/pins, immutable revisions/source spans, bounded native reports, original-source formatting | No ordinary companion manager, arbitrary-IR export, complete visual/editor cycles or new representative-user evidence. Measure full files before changing quotas |

These are inspections of existing contracts/code/tests, not observations of a
running full benchmark. Potential future refusals are requirements to test,
not recorded outcomes for nonexistent packages.

## RP-01 package requirements

Source brief: [expense reimbursement](../representative-processes/01-expense-reimbursement.md).
Preserve the [whole-case paper responsibilities](../authoring/benchmarks.md#rp-01--reimbursement-including-corrections-and-uncertainty).

The request must carry business identity, submitter, exact amount/currency, date,
category, purpose, allocation, receipt-or-missing-reason evidence and policy/input
revision identity. Separate manager, finance, correction and payment evidence;
do not reduce approval to an unversioned Boolean. Synthetic participant/provider
identities are data, not authentication. History and actual principal/assignment/
input revision checks remain explicit host contracts.

Include complete duplicate/policy checking, suspected-duplicate human resolution,
manager/finance approvals and reasons, employee correction, rejection/withdrawal,
payable request/confirmation, ambiguity and visible payment problem, notifications
and surviving obligations. Parallel checks are optional in the brief, so an
explicit sequential variant is legitimate if declared consistently; omission of
the checks or their evidence is not. Correction uses explicit repeated scoped
work, not a back edge in the acyclic graph. Reassignment keeps decision authority
and old assignment evidence distinct.

| Scenario | Whole-package proof/evidence required | Current witness and remaining dependency |
| --- | --- | --- |
| RP-01-A | Complete current-revision request, checks, authorized manager/finance, one payable request, correlated confirmation and distinct paid outcome; all other outcomes/obligations retained | Review/service fixtures demonstrate fragments only. Supply exact nominal/provider/protection resources, full graph and every stage result |
| RP-01-B | Finance correction creates a new claim revision, withdraws prior decision sufficiency, completes the current cycle and requires new approvals without erasing old records | Ordinary invalidation and fresh scope rules exist. Prove full correction dependencies/availability and how review-cycle results become parent-visible under no-port repeat restrictions |
| RP-01-C | Timeout/ambiguous effect remains unknown with one accountable owner; reconcile stable request/effect before retry; clock, basis, original request and key stay explicit | Core 0.2.0/source 0.3 retry and source 0.4 reconciliation links exist. Full correlated follow-up data/authority, accepted transfer and handler/ordinary continuation are not supplied |
| RP-01-D | Define effective withdrawal ordering/cutoff; current authority and request revision gate approval/payment; retain stale/losing observations and outstanding effects | Root cancellation is not an ordinary "withdrawn" activity result. Supply full guard/conflict coverage and provider cutoff contract; no claim that source order resolves it |

Any optional deadline/reminder variant needs its own named clock, finite calendar
resource, versioned basis and explicit authorized pause/reschedule policy. Omit
optional business-calendar behavior only by recording the chosen benchmark variant,
never by silently claiming that the calendar-generation problem was solved.

## RP-03 package requirements

Source brief: [order fulfillment](../representative-processes/03-ecommerce-order-fulfillment.md).
Preserve [partial fulfillment and compensation responsibilities](../authoring/benchmarks.md#rp-03--order-including-partial-fulfillment-and-compensation).

Define exact order/line, amount/currency, inventory, authorization/capture/void/
refund, warehouse shipment and carrier evidence records. Each consequential
operation has its own provider/effect identity, authority, protected input/result,
confirmation and reconciliation path. A shipment request is not handover or
delivery. Per-line outcomes and outstanding compensation are first-class evidence,
not a global fulfilled flag. Do not embed real payment or address data.

| Scenario | Whole-package proof/evidence required | Current witness and remaining dependency |
| --- | --- | --- |
| RP-03-A | Independent reservation/authorization, explicit join, two warehouse shipments, per-line delivery, allowed capture total and completion after both | Ordinary disjoint footprints are checked. A two-line finite package can use explicit disjoint cells, but a generic keyed result bundle/aggregation needs a reviewed versioned interface; no hidden fan-out outputs |
| RP-03-B | Stock failure prevents shipment; separately authorized void/expiry remains owned until confirmed, including unknown void | Exact provider states/settlement contracts exist as partial rules. Supply whole failure/compensation graph, correlation and no-shipment evidence, including fault continuation needs |
| RP-03-C | Cancellation/packed evidence has explicit order and physical provider cutoff; irreversible work is not rolled back; capture/refund obligations remain visible | Conservative concurrency can refuse unproved effect conflicts. Supply guard coverage and provider facts, not just a true invariant or cancellation request |
| RP-03-D | Same provider event/effect is recorded without a second capture or transition; contradictory evidence remains a conflict obligation | P3 defines deduplication/terminal monotonicity. No full package or running duplicate-delivery evidence exists; preserve synthetic/static versus actual host distinction |

A fixed two-line benchmark is a declared bounded input, not proof of arbitrary
line counts, dynamic result collection or capacity/back-pressure behavior. Any
later dynamic interface must preserve item keys, protection, atomic results,
invalidation, removed-item settlement and seal behavior.

## RP-08 package requirements

Source brief: [incident coordination](../representative-processes/08-cybersecurity-incident-response.md).
Preserve [changing scope and outage responsibilities](../authoring/benchmarks.md#rp-08--incident-coordination-including-changing-scope-and-outage).

Keep observations, hypotheses, decisions and actions distinguishable. Define
report/evidence references with provenance/confidence, asset/scope revisions,
commander/analyst/service-owner decisions, objectives, containment/recovery and
follow-up ownership. High-impact capability authority is independent of a tool
proposal and requires supported guard evidence. All content remains fictional
language-design material without operational commands or severity advice.

| Scenario | Whole-package proof/evidence required | Current witness and remaining dependency |
| --- | --- | --- |
| RP-08-A | Authorized parallel containment, per-service staged recovery, explicit exit predicate, owned remediation/notification/evidence obligations and protected summaries | Partial ordinary/follow-up checks exist. Supply full transfer/exit/protection/conflict graph and result aggregation/retention where required |
| RP-08-B | Verified scope revision retains valid unaffected work, adds identified assets, settles removals/changes and uses new batch after seal | Stable-key/seal semantics are specified, but full dynamic membership/result/invalidation evidence is absent. No arbitrary runtime template injection is implied |
| RP-08-C | Tool recommendation alone cannot dispatch; explicit current human/capability authority and protective guard are both required; proposal remains reviewable | P1/P2/P7 require these boundaries. Supply exact independent policy/resource predicates and whole-graph negative evidence; a trusted-looking label is insufficient |
| RP-08-D | Alternate-channel records retain event/source identities, known timestamps, provenance and only known causal links; import acceptance order is separate from historical occurrence order | There is no accepted general runtime history-import schema. A wait payload or timestamp sort is not an import implementation; propose a separate contract before claiming this scenario lowerable/executable |

Restricted audience views and accessible summaries must retain meaning without
leaking labels, topology, timing or content. Source clauses do not prove protected
editing or representative-user comprehension.

## Dependency order and package conventions

1. Freeze the RP-01 implementation plan: exact synthetic types/resources and
   complete request/review/correction/payment/follow-up graph; four scenario
   records, every alternative, and honest stage/refusal results. Resolve concrete
   continuation/retention requirements with a separate ADR/profile if needed.
2. Produce original source, companion, exact resource bytes and derived canonical
   IR together. A manifest must pin profile/resource digests and map scenario
   requirements to actual declaration IDs and tests. Store expected codes/paths
   and remaining host/proof requirements. Formatting is a package reparse check,
   not arbitrary-IR export. Never publish placeholder ellipses as a full graph.
3. Use the whole graphs to drive missing guard/invalidation/exceptional/lifetime
   judgments, with supported-subset refusals kept visible. Decide any fault-data/
   continuation or dynamic-result extension separately; preserve old frozen bytes.
4. Build RP-03's complete declared two-line variant, then a generalized keyed
   interface if required; build RP-08's batch/history/protected-view representation
   with explicit unresolved import boundaries. Keep all forty corpus rows current.
5. General text/IR export and ordinary companion management, visual notation/layout
   and full edit cycles, protected accessible materials and actual participants
   remain necessary. Do not transfer Phase 1 acceptance to a future engine.

Measure source bytes/tokens, parser work, semantic budgets, canonical output and
report size for actual full packages before changing limits. A larger fixture
that fails a quota is data for a bounded budget decision, not grounds for silently
raising limits, skipping checks or truncating obligations.

The audit records requirements and code-backed boundaries, not new benchmark
results or a revised semantic contract. Check the separate corpus accounting and
original evaluations for their evidence scope. Owner delegation permits this
implementation order; it does not mean the owner personally reviewed this audit
or approved Phase 1 closure.

## Verification and delegated review

Compared the two new tables directly with the ten adopted case files: exactly
forty unique RP scenario IDs, twelve target IDs and forty Partial statuses.
Verified local links/anchors and SPDX, checked current port/grammar/policy/API
restrictions against primary repository files, and passed all six existing
authoring-evidence groups. No implementation, frozen profile/snapshot or
historical score changed. Required final-head CI still applies to this docs PR.

Separate completed-document automated self-review checked corpus scope and
alternatives, count distinctions, actual versus proposed evidence, no hidden
aggregation/continuation/retained values, explicit host/user/visual limits, package
dependency order and unchanged historical counts. No actionable issue remains.
Accepted under ADR-0018 owner delegation as a readiness record; no new semantic
contract or independent/individual owner review is claimed. Merge requires
passing final-head CI and the expected head. Continue RP-01 implementation after
its own frozen plan; this audit is not a phase report or approval boundary.
