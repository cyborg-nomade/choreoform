<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-03 whole-source draft and real capacity evidence

**Date:** 2026-11-11<br>
**Baseline:** `fc5a80dd3ead724806811adc20b86af820fd6f41`, draft #46 after #81<br>
**Frozen plan:** `c63367b`, [plan](0067-rp03-package-plan.md)<br>
**Decision:** [ADR-0067](../decisions/0067-rp03-package-draft.md)

The [generated two-line proposal](../benchmarks/rp03/README.md) has 505,980 UTF-8
source bytes / 77,135 tokens measured by complete-line lexing, a 229,261-byte stored
companion, 658 declarations/445 symbols/263 generated identities/48 resource bindings.
Source 0.14/companion 0.13/core 0.11/intended IR 0.4 remain unchanged.
Source digest `sha256:0b6a4c2b18367f47707003cf93358106c8fe20950a677ed7444e182ead938797`;
canonical companion digest `sha256:4b06fbbe97c90b6a63e465ec1eb4c9f7824126b28ea5584227537a702d21a1a4`;
package digest `sha256:b3af3f83a0bac615b4d5fb77d361617e50e43c51d1538d9b0cad34c23249cc2e`.
These are content identities, not admission or authorization.

Actual native lowering refuses `tokens at 108626..108627`, exit 1, empty stdout.
There is no candidate IR, revision or semantic-stage report; not-run stages are an
empty list, never passed or inferred. The 16,384-token existing profile is preserved.
The companion passes its actual closed schema/identity/namespace rules independently.
Its success proves no whole-program syntax/references beyond the cutoff. An initial
438,171-byte / 65,635-token prototype also refused; its identities were not published.
Review corrections and explicit receipt/quantity/recovery evidence produced the
final measured draft above. Do not use the early measurements as current evidence.

## Original scenario representation and remaining proof

| Scenario | Actual declared draft responsibilities | Evidence still absent |
| --- | --- | --- |
| A | Three independent reservation/authorization branches and explicit join; two pack/label/handover branches; one allowed capture; separate delivery branches and per-line dispositions | Full lowering/semantic admission; actual reserve/payment/physical/delivery/current authority/cutoff/clock/settlement behavior |
| B | No shipment on stock-failure/customer-refusal route; explicit reserved-stock release and authorization void/expiry; partial offer preserves original order | Full branch/compensation proof, actual confirmed release/void, residual authorization and uncertain-effect ownership |
| C | Monotonic customer cancellation; separate surviving recovery direction; stop/cutoff/interception/label release/reservation release/refund and accountable unresolved owner claims | Actual entry cause/context/ancestor retention/current authority/race/physical cutoff/eligible refund and accepted all-obligation handoff/settlement |
| D | One explicit capture activity and exact stable provider/effect/request/event declarations; terminal source capture and original request retention | Real duplicate/terminal-monotonicity/conflict ledger and observed suppression of a second transition/effect |

Every row is Partial. Full source content is not a statement that all proposed syntax,
qualification, cross-context invocation/compensation/retention or ordinary/exceptional
flow is accepted. Those are next actual judgments after capacity. Capture/refund /
release are new effects under their own authority/key; no rollback or changed-input
retry is invented. Optional backorder/substitution variants and general arbitrary
line aggregation are not claimed; the declared customer option is ship available
unchanged lines or cancel all, retaining unresolved residual authorization explicitly.

Exact fictional resources distinguish command receipt from optional operation subject
reference, business result from unknown/rejection, target quantity/warehouse and
order/line/input/address revisions, money/currency, provider event and basis identity.
Twenty-three providers each have their own terminal descriptor; the registered clock
profile/source are separate. Actual supported provider semantics, receipt/correlation /
normalizer truth, clock domains/stamps/current dispatch authority and atomic behavior
remain host contracts. Typed field values and digest claims are not those observations.
The broad recipient circle is explicit and does not demonstrate least privilege.

## Review findings fixed before checkpoint

The initial recovery proposal could infer too much from missing handover/capture
records. It now asks for attributable recovery direction before stop work, requires
positive stopped evidence for reservation release, and does not issue a fresh void
from an absent/possibly unknown capture result. Refund requires positive matched
capture amount equal to confirmed commitments and positive interception of each
confirmed committed line. Unknown physical/financial work remains accountable;
local text or finish settles nothing. Human returned principals must equal the
current P1 principal; assignment/receiver claims still require actual host binding.
Normal stock-failure cancellation dispositions require matching stock/authorization
release evidence; unresolved cases go to accountable support, while a fully matched
ordinary delivery does not force unnecessary human exception work.

Reference and subject identity were separated; quantity/warehouse were added to
actual instruction/result types and every receipt equality. Typed wrong quantity,
warehouse, amount, currency, input/order revision and basis values now fail actual
production pure evaluation. Declaration construction also caught invalid spaced
wire IDs, one duplicate node ID and ancestor caption shadowing; captions/IDs were
corrected before publishing. No rule was weakened to hide these failures.

The default separate-shipment order now routes partial stock failure to release /
void; a new customer partial-offer decision is possible only after explicit original
opt-in. Unknown delivery-policy strings refuse the nominal invariant. Recovery
and exception input records now include every known operation result; summaries
cannot hide those reads/provenance. This partial known view still is not the actual
activation/unknown/obligation ledger, whose accepted entry representation remains
open. A receiver/obligations-reference claim supplies no such ledger or handoff.

## Reproducible independent checks and limits

Three SDK groups validate actual named invariants (wrong totals, duplicate line /
warehouse, zero quantity, currency/reference errors), every provider/source/clock
and immutable byte pin. All 23 emitted P1 receipt ASTs type-check with complete
single-instruction dependency. Two private production E1 groups evaluate 22 synthetic receipt cases (3 matched /
19 meaningful typed mismatches across reservation/capture/delivery) and eight
recovery/partial-choice guard cases. The latter reproduce default stock failure,
explicit partial opt-in, unknown stop/capture/interception and positive evidence
proposals; absent records never prove an effect absent.
No public evaluator, actual provider event, accepted history or process trace is added.
An initial test incorrectly accumulated unrelated positive/negative evaluations
under one resource budget; each independent provider/value family now uses its own
SDK judgment. No process-check budget resets or quota increases are introduced;
full packet/semantic/report budget evidence remains unmeasured behind the lexer gate.

Eight bounded Python groups reproduce generator/first refusal/exact versions/digests /
resources/scenario IDs/pure corpus/licensing/CI and RP-01 byte preservation. A small
native source witness checks decimal units, variant construction and nominal wrap
encoding under the existing profile, not a truncated order. All 113 RP-01 files have
aggregate content digest `sha256:d3aad696ec8ec3bc325a42b7467c8725c73bb15f1c8620b58fc76531d2f1e592`;
its 44 guard cases/46 values/candidate binding remain fixed. Configured fixture
formatting defaults regenerate both current and historical RP-01 byte for byte.

Final local verification passes 553 Rust tests plus six compile-fail doctests,
native workspace/five library Wasm clippy checks, release portability/frontend Wasm
builds, native portability and all 63 Python workflow commands. Separate completed-
diff delegated agent review found and fixed the recovery/opt-in/context gaps above;
no remaining actionable checkpoint finding. Full source admission and optional
backorder/substitution/reallocation variants remain open. Required final-head CI /
full comment audit/signed expected-head merge follow before integration.
Logs: `/private/tmp/rp03-{resources,receipts,package-python,workspace,verify}.log`.
Agent review is not independent/human/personal owner review; skipped CodeRabbit
supplies no review evidence. Wasm compilation is not browser execution.

Next separately freeze a measured bounded source-profile capacity plan. Keep every
old quota/pin/meaning and retain the next real parser/qualification/semantic/report
refusal rather than force success. The extensive repeated protection/receipt text is
also an authoring bottleneck: reusable near-English formulations, identity-managed
editing and protected accessible summaries remain required **within Phase 1**.
Larger quotas will not close that requirement, full RP-01/03/08, complete validator,
actual users/hosts/browsers/independent or G1–G4/phase exits. Parent #46 remains draft,
all forty scenarios Partial; this is independent work still in progress, not an
external blocker or phase approval boundary. No Phase 2.
