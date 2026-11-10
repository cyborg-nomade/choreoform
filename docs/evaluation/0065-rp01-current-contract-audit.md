<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Full RP-01 current-contract and late-capture critique

**Date:** 2026-11-10<br>
**Baseline:** `d747ead7416d4bd885c1f586bafcc4b3501af9cf`, draft #46 after #79<br>
**Authority:** ADR-0018 / [ADR-0065](../decisions/0065-rp01-current-contract-direction.md)<br>
**Frozen plan:** `8728265`, [plan](0065-rp01-current-contract-audit-plan.md)

This is a test/docs audit, not benchmark adoption. Every committed RP-01 source /
binding/resource/manifest/candidate/report/guard byte stays fixed at a92e472:
source 0.7/companion 0.6/core 0.4/IR 0.1, 56,874 source bytes/9,470 tokens,
semantic revision `sha256:97e5c346442d5b051e40bc91d2c3014be1bd990688324be846739c5d35a265c4`,
wire digest `sha256:c012c1493ad7b04abc5e5f81529eea6231b720a54ee6a8db867fd2e207fd546a`.
Production rules and all accepted contracts/profiles/artifacts stay unchanged.
Native calls operate on temporary source packages; no actual observation, accepted
input, grant, storage, handoff, host execution or representative-user data is created.

## Demonstrated full-case results

The frozen baseline still passes all eleven current checking/advisory views. Its
historical manifest continues to record ten stages; the extra advisory invocation
is new audit evidence, not a rewrite of that historical count. A naive header /
pair/core promotion refuses syntax at the old retry fault-source clause. Explicitly
move that binding to Recognise faults from and use the current retry sentence;
the entire promoted graph then lowers and passes all eleven views.

The neutral promoted body differs only in selected core, IR 0.4's required nullable
scope faultInput/retainedRequests and wait capture fields, and the nullable general
work faultSource (the payment binding equals its unchanged retry source). Removing
exactly those changes yields complete equality to the frozen body. All IDs, ordinary
guards/actors/ports/expressions/flow/data/metadata dependencies remain. Existing
provider/type/clock/input/protection bytes and recipient sets are not broadened.

| Experimental package | Source bytes / tokens | Companion bytes (normal JSON spacing) | Lowered bytes including newline | Result |
| --- | --- | --- | --- | --- |
| Neutral explicit current-profile promotion | 56,884 / 9,471 | 38,809 | 101,278 | All eleven native views pass |
| Payment terminal capture + exact readonly original request | 58,485 / 9,691 | 39,675 | 103,976 | All eleven pass; separate owned local/request proof |
| Above + local receipt-matched late category | 61,257 / 10,223 | 40,388 | 108,515 | All eleven pass; no ordinary parent write/result |

These fit existing limits without quota changes. Neutral semantic revision is
`sha256:7a17e3c9a7adec618e975c0895c95a3f130491e987930f808348590fe6a5e896`;
capture/request revision `sha256:9fbb5037ea8282d51a5ce742dfef34d4a66d1c234cfbee824b284ac63f57143a`;
receipt-matched classification revision
`sha256:4e2c5232ba6f9f8e12ffe483f95d719afa9fb94872122f05267c8955a91790c4`.
They are experiments, not published checkpoint identities.

A fresh synthetic terminal-source descriptor pins the unchanged exact payment
provider and accepted normalization profile. The existing observation type already
matches its derived normalized type, retaining nominal Paid/Problem evidence.
The actual-use payment_observe wait captures payment_request_step into a fresh local
cell. The readonly request has the actual capability's full claim/manager/finance
input type. The existing Expense access predicate/envelopes/participants/recipients
remain; no predicate is replaced by true. The original source still reads
current_claim in expense and uses real request/deadline ports.

Flat owned proof accepts this full-case shape and retains current_claim as required
source/control provenance owned by payment_service; it does not supply its live
value to payment_recovery. Entry seeds only the exact accepted original request
before the capture supplies its own value. New captures/records stay local. With
retention, final retainedRequestScopes is payment_recovery and those conditional
nodes stay legacy-uncovered. Without a retained seed, fresh capture can also pass
legacy zero-inherited-value availability; that separate meaning is preserved. An
initial test incorrectly expected the retained-seed omission behavior in both cases
and was corrected to the actual distinct coverage. Owned lifetime/protection/host
premises are required in both, not inferred from legacy availability alone.

## Review finding: a paid tag is not a matched receipt

The first local classifier distinguished confirmed paid from confirmed problem but
otherwise trusted the business tag. Review identified a genuine gap: validly typed
Paid evidence may carry the wrong amount/currency/claim/revision. A production E1
pure-evaluator counterexample confirms the tag-only form returns paid for the
existing typed wrong-amount value. The final experiment reuses the unchanged
payment_work completion predicate, rebinding only its result fact to the captured
confirmed payload and its payment_request read to the retained original instruction.
No receipt comparison is dropped, no mutable ancestor is read and no authority /
race predicate is replaced by a permission claim. Failure records mismatched-evidence,
retaining the original captured payload for accountable conflict review.

A new, separate thirteen-case pure corpus and actual production type/value/evaluator
test cover paid/problem, four wrong paid fields, two wrong problem identities,
noeffect/rejected/cancelled and old-versus-corrected retained revisions. All inputs
are synthetic pure values, not authenticated observations or accepted history; the
original 44 guard cases/46 values/expectations remain unchanged. Complete type/read
checks see exactly capture + original request, and the whole source uses the same
shared AST. Correlation record strings alone still prove no actual no-effect or
permission to retry. A matched receipt also grants no current authority/new dispatch.

## Original scenario accountability and next dependency

| Scenario | Evidence advanced or preserved | Still open |
| --- | --- | --- |
| RP-01-A ordinary reimbursement | Neutral full-body equality preserves approval/payment/notice graph and all ordinary guards; current-profile checks pass | Complete cross-form package/runtime/host/participant evidence and phase gates |
| RP-01-B correction invalidates review | Original guarded revisions remain; retained original instruction is frozen and cannot approve a new revision | Actual atomic invalidation/conflict/current authority and editing/user evidence |
| RP-01-C ambiguous response | Unchanged stable-key retry/definitive-no-effect source rules; exact typed late capture/request and receipt-matched local category now demonstrated in full graph | Actual provider truth/correlation/dispatch/recovery/receiver handoff, explicit accountable mismatch review/settlement and full scenario evidence |
| RP-01-D withdrawal race | Existing acceptance-order/commit/cancellation guards unchanged; late record writes no ancestor, publishes no parent result and clears no cancellation | Actual acceptance ordering/cutoff/receiver registration/all-transitive settlement and protected observable outcomes |

Unknown/acknowledged are excluded from terminal normalization. A new payable under
a new business key is not a safe technical retry; the retained request does not
reopen an original activity or make an unresolved effect absent. A local mismatched
evidence text or finish cannot settle an outstanding conflict. Scope disposal still
requires accepted settlement of every actually activated effect/subscription/child.
Do not turn this partial static checkpoint into a complete C/D claim.

An isolated four-group SDK audit also confirms the finite active-work family still
rejects ordinary-root ancestor reads and writes after normal static checks. This
matches a property of the actual payment source, but is not a failure of the passing
flat capture/request experiment. Generalizing active work/root contexts, broader P4
ownership and preterminal/possibly-completed compensation remain later if required.
Do not add refunds to RP-01 or broaden metadata merely to exercise a new prototype.

Recommend next: freeze a separate controlled adoption plan for the demonstrated
source 0.14/core 0.11 capture/request/matched-local-record checkpoint. Preserve an
exact tested historical source-0.7 counterpart and all original resource/guard logic;
explicitly repin generated candidate/manifest/report/guard identities and test-only
transport/resources where required. Then reassess remaining original scenario and
RP-03/08/validator/text-visual/editing gaps. A new semantic/core increment is not
needed merely to publish the already-passing full-case judgments.

Four new SDK integration groups, one production pure-evaluation group and six
bounded Python groups reproduce full graphs/refusals/deltas, exact source provenance,
local/no-parent writes and byte preservation. Wrong provider capture, initialized /
written request target and missing invalidation retain genuine first refusals,
including owned-local-invalidation-coverage. Final local verification passes 548 Rust tests plus six compile-fail doctests,
native workspace clippy, five library Wasm clippy checks, release portability /
frontend Wasm builds, native portability and all sixty workflow Python commands.
A separate completed-diff agent review checked correctness, security, performance,
scope, exact resource/guard preservation and receipt rebinding; no remaining
actionable finding. Required exact-head CI/comment audit/signed merge follow before
integration. Logs use
`/private/tmp/rp01-current-contract-{sdk,pure,python,workspace,verify}.log`.
Agent self-review is not independent/human or personal owner approval; a skipped
bot supplies no evidence. Browser compilation is not browser execution. Parent #46
remains draft/incomplete, forty scenarios Partial and G1–G4 conditional. Complete
executable-IR/validator/benchmarks/general near-English/protected accessible visual
and actual host/browser/user/independent/phase exits stay open. No Phase 2.
