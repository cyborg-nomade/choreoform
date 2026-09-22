<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Whole-case authoring walkthroughs

**Evidence:** B, single-author paper work. These treat the complete case briefs,
including alternative paths, but are **not complete source programs, IR fixtures,
provider contracts or execution traces**. A stage inventory plus sample sentences
does not discharge the Roadmap's full lowerable-benchmark requirement. Names below
identify proposed declarations; absent resource/type/guard implementations are
listed explicitly. Nothing here is accepted by the existing `.choreo` parser.

Both candidates use the same stage inventory, data, authority and obligations.
The tables spell the equivalent control/action fragments for each; surrounding
declarations, full maps and policies are shared per [syntax](syntax.md). B is not
given a more capable event language. Outcomes name exact typed results, not any
English sentence following “when”. Whole-case completeness is reviewed against
the frozen corpus, not inferred from the number of syntax lines.

## RP-01 — Reimbursement, including corrections and uncertainty

Source: [complete case brief](../representative-processes/01-expense-reimbursement.md).

### Data and boundary inventory

The request record includes stable business identity, submitter, exact amount and
currency, date, category, purpose, cost allocation and receipt/missing-reason
variant. Keep immutable request/policy revisions, duplicate evidence, decision
variants, assignment revisions, correction basis, payable reference, payment
evidence and withdrawal status. Avoid a single “approved” Boolean detached from
its evidence revision. No credentials or live receipt data belong in examples.

Actor requirements: Employee, Manager, Finance reviewer, payment operator and
authorized clock observer. Capabilities: policy/duplicate checks, create payable,
payment reconciliation and notification. Each needs exact supported resources,
input/output types, current authority and result protection, not just this name.
Manager/finance decisions are sequential in this case; checks may run in parallel.
An expense amount type/currency and business calendar need explicit contracts.

### Shared process and two surfaces

| Stage and complete responsibility | Candidate A fragment | Candidate B fragment |
| --- | --- | --- |
| Root runs a single-outcome review cycle while its typed `Review needed` condition is true; entry never inferred | `While "Review needed" is true, repeat "Review cycle".` | `When entered, then while "Review needed" is true, repeat "Review cycle".` |
| The cycle starts duplicate and policy checks, then gathers their recorded evidence; suspected duplicate goes to human resolution, not deletion | `Start together ... and gather at "Checks gathered".` | `When entered, then start together ... and gather at "Checks gathered".` |
| Manager decides on current request; approved continues to finance, correction to employee, rejection records disposition | `Ask "Manager" to decide using "Current request" under "Manager rules" and record in "Manager decision".` | `When entered, then ask "Manager" to decide using "Current request" under "Manager rules" and record in "Manager decision".` |
| Finance checks coding/evidence and exceptions after approval; correction invalidates old decision sufficiency | `On "correction needed", continue with "Request correction".` | `When the result is "correction needed", then start "Request correction".` |
| Employee correction publishes a new revision and finishes the cycle with its one ordinary outcome; only the repeat creates another occurrence | `On "resubmitted", continue with "Record revised request".` | `When the result is "resubmitted", then start "Record revised request".` |
| A pure end-of-cycle assignment records whether another review is needed and the resulting disposition; root branches after repeat | `Choose by ... ; otherwise none.` | `When entered, then choose by ... ; otherwise none.` |
| Approved current revision alone enables one payable request under guarded commits | `Request "Create payable" using "Approved payment input" under "Payment rules" and record in "Payable reference".` | `When entered, then request "Create payable" using "Approved payment input" under "Payment rules" and record in "Payable reference".` |
| Timeout/crash ambiguity retains the effect and enters declared surviving reconciliation work through lifecycle/fault policy, not an invented outcome | `Wait under "Payment reconciliation".` | `When entered, then wait under "Payment reconciliation".` |
| Supported confirmation completes the payment obligation; paid/rejected/withdrawn/payment-problem dispositions remain distinct | `Finish as "paid".` | `When entered, then finish as "paid".` |

Ellipses mark unfinished maps, so those rows are deliberately non-lowerable.
All named ordinary outcomes still need exactly one explicit successor. Every
cycle path must finish its single outcome; there is no graph back-edge from a
correction to manager review. Reassignment is the work-policy operation with a
new assignment revision, not a new decision authority or arbitrary source edit.
Notifications have their own effect/settlement handling; delivery is not payment.

Withdrawal uses root cancellation authority, not a parallel ordinary “withdrawn”
human outcome. When it is effective before payment commitment, new ordinary work
is blocked. A request already dispatched remains accountable to reconciliation;
the provider's irreversible point and guarded invariant must establish whether
withdrawal can take effect. No source-order rule resolves that question.
Correction/review deadlines and reminders use separate waits/timer revisions;
expiry routes to explicit human/fault handling and never chooses a business result.

### Worked acceptance answers

| Scenario | Required reading in both candidates | Evidence/remaining task |
| --- | --- | --- |
| RP-01-A | Current-revision manager and required finance approval precede one payable; payment confirmation, not request acceptance, justifies paid | Whole flow still needs exact contracts, guards and complete source/IR |
| RP-01-B | Revision 2 does not reuse revision 1 approval; retain history, complete the cycle, then new manager/finance occurrences | Invalidation sets and availability/type tests remain unimplemented |
| RP-01-C | Timeout means unknown; reconcile stable effect/request before deciding retry eligibility; unchanged request/key across attempts | Provider stable-key/no-effect contract and lifecycle routing required |
| RP-01-D | Accepted withdrawal blocks payment if effective; stale approval is recorded; conflicting commits must recheck protective invariant | Full withdrawal/dispatch invariant and provider cutoff evidence absent |

Missing exact policy/duplicate providers, calendar construction, receipt/decision
types, authority predicates, effect routing and a proven withdrawal invariant
are draft blockers, not authoring defaults. A payment-problem summary cannot hide
an unknown payable: ordinary closure must leave an accepted live owner or remain
pending. At-most-one settled reimbursement is a process/provider obligation,
not a guarantee created by the phrase “Create payable”.

## RP-03 — Order, including partial fulfillment and compensation

Source: [complete case brief](../representative-processes/03-ecommerce-order-fulfillment.md).

### Data and boundary inventory

Keep order/line IDs, priced line and address revisions, quantities and currency,
payment/reservation references, expiry bases, shipment membership/key data,
per-line outcomes, delivery evidence, capture/refund ledger and pending
compensation. Payment data is a protected provider reference, not card details.
Human actors include customer-policy authority, warehouse/support and authorized
payment operators. Each inventory, authorization, capture, void, refund,
warehouse, carrier and notification effect has a distinct resource and policy.

### Shared process and two surfaces

| Stage and complete responsibility | Candidate A fragment | Candidate B fragment |
| --- | --- | --- |
| Start reservation and payment authorization independently, each in a no-port lexical child; record confirmed results, not success-on-dispatch | `Start together ... and gather at "Acceptance gathered".` | `When entered, then start together ... and gather at "Acceptance gathered".` |
| Join after both children reach declared completion dispositions; inspect business result data before fulfillment, not `any succeeded` | `Gather from "Accept order" when all report "recorded"; for unfinished work use "Retain acceptance work".` | `When entered, then gather from "Accept order" when all report "recorded"; for unfinished work use "Retain acceptance work".` |
| Stock failure plus authorized payment creates a void/expiry obligation; no shipment on that route | `On "stock unavailable", continue with "Resolve authorization".` | `When the result is "stock unavailable", then start "Resolve authorization".` |
| Partial stock asks the customer to accept split/backorder/substitution/cancellation; changed commercial commitment is explicit new evidence | `Ask "Customer decision authority" to decide using "Partial offer" under "Offer rules" and record in "Offer decision".` | `When entered, then ask "Customer decision authority" to decide using "Partial offer" under "Offer rules" and record in "Offer decision".` |
| Allocate confirmed lines to shipments with stable keys; freeze a finite batch and start each shipment child | `For each item of "Shipments", using key "Shipment key", run "Deliver shipment" with item "Shipment"; seal when "Allocation sealed" is true; for changes use "Shipment changes"; gather at "Deliveries gathered".` | `When entered, then for each item of "Shipments", using key "Shipment key", run "Deliver shipment" with item "Shipment"; seal when "Allocation sealed" is true; for changes use "Shipment changes"; gather at "Deliveries gathered".` |
| Child obtains pick/pack/handover milestones, then delivery; rejection before handover can create an explicitly authorized later allocation | `Wait under "Carrier evidence".` | `When entered, then wait under "Carrier evidence".` |
| Capture only at the explicitly configured commitment point under the authorized ceiling, separately from each delivery | `Request "Capture payment" using "Capture instruction" under "Capture rules" and record in "Capture evidence".` | `When entered, then request "Capture payment" using "Capture instruction" under "Capture rules" and record in "Capture evidence".` |
| Cancellation requests stopping active ordinary work; surviving settlement handles releases/voids/refunds and uncertain attempts | `Wait under "Settlement evidence".` | `When entered, then wait under "Settlement evidence".` |
| Gather all sealed shipment dispositions; derived per-line summary distinguishes fulfilled/cancelled/unresolved and checks owned follow-up | `Finish as "order recorded".` | `When entered, then finish as "order recorded".` |

“Recorded” in this paper design is a normal typed child outcome after recording
an explicit business disposition. It must **not** relabel a technical fault,
unknown provider state or lost worker as success. Such paths keep settlement
pending under the declared fault/closure rules. Permanent integration problems
require accountable support ownership; the root cannot just finish as failed.

Warehouse handover is not reversible packing. A late packed/handover observation
must be reconciled with provider facts; intercept is new capability work and
post-delivery return remains outside the corpus case. Compensation links the
prior effect occurrence and gets a new logical key. Reservation/authorization
expiry and carrier reordering remain separate time/evidence rules. Capacity
limits constrain scheduling, not which lines count toward fulfillment.

### Worked acceptance answers

| Scenario | Required reading in both candidates | Evidence/remaining task |
| --- | --- | --- |
| RP-03-A | Both required reservations/payment evidence precede acceptance; two keyed shipments may finish at different times; capture justified once | Complete amount/line aggregation and resource contracts missing |
| RP-03-B | No shipment after stock failure; void or expiry stays owned until confirmed, including ambiguous void | Provider status, fault routing and settlement graph missing |
| RP-03-C | Packing/cancellation order plus confirmed provider cutoff decides whether stopping is possible; no fictional rollback; refunds explicit | Guarded invariant over all dispatch/cancel paths not yet proved |
| RP-03-D | Same capture observation ID may be recorded twice but causes one transition and no second financial change | Runtime duplicate/correlation evidence is not supplied by syntax |

Notational gaps expose contract pressure: arbitrary list summation is not an
ADR-0013 expression operator, and Decimal multiplication/division are unsupported.
The complete design must supply a reviewed bounded algorithm or an explicitly
typed supported calculation capability, not smuggle “calculate total” into a
pure English expression. Per-line aggregation across item-only fan-out children
needs a concrete safe data model; implicit output ports are forbidden. These
block complete lowerable RP-03 evidence in this proposal.

## RP-08 — Incident coordination, including changing scope and outage

Source: [complete case brief](../representative-processes/08-cybersecurity-incident-response.md).
This is fictional language evidence, not security response guidance.

### Data and boundary inventory

Keep incident/report identity, observed facts distinct from hypotheses, provenance,
confidence, evidence custody, scope/severity revisions with rationale, objectives,
asset/task keys, current owners, containment approvals, recovery/monitoring
states and follow-up obligations. Restricted communications have an approved
content revision and audience. “Analyst” and “Commander” denote authority
requirements, not credentials or proof that a tool's recommendation is authorized.

### Shared process and two surfaces

| Stage and complete responsibility | Candidate A fragment | Candidate B fragment |
| --- | --- | --- |
| Accept an assessment with an authorized human, severity/rationale and coordination assignment | `Ask "Responder" to decide using "Accepted report" under "Assessment rules" and record in "Assessment".` | `When entered, then ask "Responder" to decide using "Accepted report" under "Assessment rules" and record in "Assessment".` |
| Not-an-incident route records rationale and settles existing obligations instead of deleting the report | `On "not an incident", continue with "Settle assessment".` | `When the result is "not an incident", then start "Settle assessment".` |
| Investigation, preservation, impact and permitted mitigation are explicit parallel children | `Start together ... and gather at "Initial evidence gathered".` | `When entered, then start together ... and gather at "Initial evidence gathered".` |
| Commander authorizes objectives/actions against current evidence; untrusted proposals cannot invoke containment | `Ask "Commander" to decide using "Containment proposal" under "Containment approval" and record in "Authorization".` | `When entered, then ask "Commander" to decide using "Containment proposal" under "Containment approval" and record in "Authorization".` |
| Discover related assets in finite keyed batches; retained changed items invalidate only declared affected work | `For each item of "Affected assets", using key "Asset key", run "Asset response" with item "Asset"; seal when "Discovery sealed" is true; for changes use "Asset changes"; gather at "Assets gathered".` | `When entered, then for each item of "Affected assets", using key "Asset key", run "Asset response" with item "Asset"; seal when "Discovery sealed" is true; for changes use "Asset changes"; gather at "Assets gathered".` |
| Each asset child requests guarded containment, validates staged recovery with its owner and keeps monitoring explicit | `Request "Approved containment" using "Authorized action" under "Containment work" and record in "Containment evidence".` | `When entered, then request "Approved containment" using "Authorized action" under "Containment work" and record in "Containment evidence".` |
| Post-seal new evidence starts a later explicit batch/occurrence; recurrence is a new objective/linked instance decision, not reopening terminal history | `While "More response needed" is true, repeat "Response batch".` | `When entered, then while "More response needed" is true, repeat "Response batch".` |
| Close active response only after exit review and accepted accountable destinations for every remaining obligation | `Finish as "active response closed".` | `When entered, then finish as "active response closed".` |

Shift handover changes assignment/ownership with acceptance and revision evidence.
Timer bases for cadence, evidence lifetime and service objectives are distinct.
Contradictory or compromised tool facts cannot overwrite confirmed evidence;
they require attributable conflict resolution. Restricted communications are
separate authorized effects; their successful delivery does not close response.
Failure of the main coordination system does not authorize an inferred total
event order. No containment command or domain severity threshold is defined here.

### Worked acceptance answers

| Scenario | Required reading in both candidates | Evidence/remaining task |
| --- | --- | --- |
| RP-08-A | Containment and staged recovery need current authority; exit criteria and remaining remediation owners must be explicit | Full ownership-transfer paths, predicate/resources and protected summaries missing |
| RP-08-B | Before seal, stable keys preserve unaffected occurrences; changed inputs retain prior evidence; after seal use a new batch | Full batch state/reuse model required; arbitrary ad hoc templates unsupported |
| RP-08-C | A tool proposal without current human authorization cannot dispatch isolation; it remains evidence for a decision | Independent capability/work authority and guarded invariant must both be implemented |
| RP-08-D | Alternate-channel timestamps/provenance/known causal links are retained; engine import acceptance is not historic occurrence order | Runtime import schema/reconciliation unsupported; this case cannot lower to a claimed complete implementation |

Free creation of new runtime process templates, a generic external-history importer
and general declassification are not provided by ADR-0013. A repeatable approved
task template is different from an arbitrary ad hoc task. For now, both designs
must show an unresolved requirement for those pressures and refuse complete
lowering. Labeling them “manual work” does not implement the missing semantics.

## Common perturbations and paper outcomes

| Perturbation | Same expected outcome in A and B; not an executed test |
| --- | --- |
| Duplicate/late event | Do not fire a second ordinary outcome or attach it to a replacement assignment; keep correlation/provenance |
| Partial failure | Preserve successful effects and unresolved obligations; no all-or-nothing fiction |
| Concurrent cancellation | Recheck guard/current authority at conflicting commits; stop ordinary dispatch, keep declared surviving settlement |
| Decision-relevant revision | Retain historical decision, invalidate its sufficiency, create renewed work as needed |
| Definition upgrade while active | New definition revision does not rebind old instance/plan |
| Large fan-out | One child template and stable keys; scheduler limits do not erase members; no performance result claimed |
| Layout/format edit | No flow or identity change; source digest may change |
| Business deadline edit | Exact timer/clock/calendar/basis and pause authority required; no implicit local-time calculation |
| Restricted review | Incomplete view clearly labeled; no source export that silently drops protected rules |

The observed desk comparison is therefore about wording and exposed obligations,
not successful full-process construction. All three benchmarks retain material
gaps; the next iteration of this same Roadmap item must produce complete source
and matching IR before claiming it is satisfied.
