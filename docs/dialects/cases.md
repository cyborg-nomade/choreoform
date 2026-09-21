<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Positive and negative contract evidence

**Status:** Accepted ADR-0013 evidence upon merge of PR #17, not executable process fixtures.
Rules: [values/expressions](values-expressions.md), [policies](policies.md).
“C” means the exact selected pure judgment runs in the
[Rust probe tests](../../tools/contract-probe/src/tests.rs). “B” means worked
paper reasoning only. Neither proves a complete validator, trusted host or engine.

## Worked value/expression example

For descriptor `{"kind":"decimal","scale":2}`, value `"12345"` is exactly
123.45 units. JSON number `123.45`, string `"123.45"`, string `"012345"` and
null are invalid. Rescaling coefficient 12345 from scale 2 to 3 yields 123450;
rescaling to 1 faults because 12345 is not divisible by 10. No amount/currency
semantics is inferred from the generic Decimal type.

For this expression body, both `allowed` and `secret` belong to its read set,
even when allowed is false:

```json
{"op":"if","condition":{"op":"read","cell":"allowed"},
 "then":{"op":"read","cell":"secret"},
 "else":{"op":"literal","type":{"kind":"integer"},"value":"0"}}
```

The expected cell types are Boolean and Integer. The expression result is Integer.
The owning expression's reads must be `{"allowed":true,"secret":true}`. Both
cells require read permission and availability before evaluation. A false branch
avoids evaluating secret's value in the expression, but cannot remove its declared
dependency/protection from admission. The probe implements the equivalent typed
AST subset, not parsing this JSON.

| Case | Input/change | Expected result | Evidence |
| --- | --- | --- | --- |
| C01 | Canonical signed-64 endpoints, zero and negatives | Admit exact integer | C: integer boundary test |
| C02 | Plus, leading zero, negative zero, exponent, fraction, Unicode digit or out-of-range value | Refuse encoding/range; no float conversion | C: integer boundary test |
| C03 | Scale 2 coefficient 123 to scale 4, then back | 12300 then 123 | C: decimal test incl. all scale pairs |
| C04 | Scale reduction with remainder, scale >18 or overflow | Fault; no rounded/saturated result | C: decimal test |
| C05 | Dead branch reads undeclared secret | Reject dependency mismatch | C: branch test |
| C06 | Dead branch reads absent cell or has different result type | Unavailable/type failure before branch selection | C: branch test |
| C07 | Checked addition overflows only in an unselected valid branch | Selected branch succeeds; selected overflow faults | C: branch test |
| C08 | AST exceeds depth 64 or 10,000 visited nodes | Resource refusal before execution | C: AST budget test |
| C09 | Equal anonymous record fields reordered; nominal ID changes with same shape | First equal, second unequal | B: V1/V2; full type universe not implemented |
| C10 | Unknown operation, impure clock read, or process expression uses policy fact | Static rejection | B: E1/E3; no JSON AST decoder implemented |
| C11 | Option absent versus uninitialized cell | First typed value, second unavailable fault | B: V1/E2; only unavailable read reproduced |
| C12 | Nominal invariant false or cyclic nominal definition | Reject value/contract before use | B: V2 |

## RP-01 reimbursement: a decision is not payment evidence

1. Review activity binds claim revision c7 to assignment a1. A synthetic actor
   initially qualifies. The human completion must carry the same principal,
   occurrence, assignment/input revisions and a fresh observation identity.
2. If authority has been revoked, refuse completion; original assignment does
   not grant permanent permission. A deadline expires/escalates, never approves.
3. Accepted approval authorizes only the explicit next transition. Payment still
   needs its own capability, protection and effect checks.
4. A timed-out payment request is unknown. With no supported stable-key promise
   or confirmed no-effect evidence, another attempt is ineligible despite spare
   attempt budget. Correlated late success confirms the original effect once.
5. A corrected claim c8 invalidates dependent approval sufficiency; it does not
   rewrite c7 or undo payment. Correction/reconciliation remains explicit work.

## RP-03 order: cancellation and joins do not erase reservations

Stock and payment reservations proceed in distinct child occurrences. If one
fails while the other has requested a provider operation, cancellation stops new
ordinary work but the requested effect may still succeed. A satisfied join must
apply its settlement policy to every unfinished child. Transfer succeeds only
when a live allowed owner accepts atomically; it does not disappear into an
unobserved callback. A terminal instance requires no obligations/subscriptions.
Compensation is separately authorized new work linked to the exact reservation,
not reversal of history or reuse of the reservation's effect key.

## RP-08 incident: exact scope and guarded consequential action

An asset list revision creates children by stable key; a duplicate key faults
before creation. Removal requests declared settlement rather than deleting a
child. Once sealed, a later discovery needs a new explicit fan-out occurrence.
Isolation requires current authority and an enforced protection/race invariant;
an adapter name or Text host address does not grant isolation permission.
Reconciliation ownership can survive ordinary closure. Fully terminal incidents
cannot reopen for a late provider fact; start/link separately identified work.

| Case | Input/change | Expected result | Evidence |
| --- | --- | --- | --- |
| C13 | Assigned/accepted, exact principal/input revision, fresh completion, current authority | Synthetic acceptance eligible | C: completion test (not host authentication) |
| C14 | Revoke authority, change principal/revision or duplicate completion | Refuse ordinary completion | C: completion test |
| C15 | Transfer widens purposes/access or changes sensitivity/policy identity | Refuse, even if value type matches | B: P2 |
| C16 | Unknown effect, budget remains, no idempotency/no-effect proof | No retry; retain unknown/reconcile | C: retry eligibility test; reconciliation path B |
| C17 | Unknown effect, supported stable key and unchanged request | Retry eligible if all other gates pass | C: synthetic retry test; adapter promise unproved |
| C18 | Changed request, cancelled work, succeeded effect or exhausted budget | No retry, including after no-effect evidence for old request | C: retry test |
| C19 | Correlated late success, then duplicate delivery | Confirm logical effect once; no second transition | B: P3 transition table |
| C20 | Terminal success followed by contradictory provider evidence | Retain success; create conflict obligation, no regression | B: P3 |
| C21 | now = due−1 versus now = due | Ineligible versus eligible | C: timer test |
| C22 | Paused timer or stale timer revision | Ineligible | C: timer test |
| C23 | Pause adds 3 to due 10; now 12 then 13 | Ineligible then eligible; negative duration/overflow fault | C: timer test |
| C24 | Interval [10,15), now 14 then 15; overlapping intervals | Eligible then ineligible; overlap invalid | C: timer test |
| C25 | Calendar pin changed without rescheduling, wrong clock or untrusted schedule | Refuse change/fire, retain old timer revision | B: P5 (host/registry not implemented) |
| C26 | One outstanding effect, exactly one accepted live owner | Accounted, not terminal | C: ownership test |
| C27 | Missing, duplicate or dead owner | Closure blocked | C: ownership test |
| C28 | Zero ordinary work but one reconciliation subscription | Cannot terminate | C: terminality test |
| C29 | Unknown provider/type/clock/policy contract or mismatched digest | Admission refused, no fallback | B: suite; existing wire tests cover old artifact corruption only |
| C30 | Unknown fault or failing handler | Nearest exact match or explicit failure/propagation, no self-reentry | B: P4 |
| C31 | Irreversible race lacks proven invariant coverage | Planning refused; scheduler order is not authorization | B: P7 |
| C32 | Timer used to manufacture expert approval | Invalid work/outcome mapping | B: P1/P5 |
| C33 | Duplicate, removed or changed fan-out item | Duplicate faults; removal settles; change invalidates without erasure | B: P7 |
| C34 | New member after seal | New explicit occurrence required | B: P7 |
| C35 | Running instance observes newly published contract revision | Existing exact binding unchanged | B: suite/ADR-0012 |
| C36 | Budget refusal during a large fan-out | No silent population truncation or discarded obligations | B: suite/P7; no scale run |

## Limits of the traces

The three narratives omit full process graphs, genuine authority/provider
contracts, histories and executable operational paths. They are not replacement
benchmark fixtures. Code-backed rows exercise pure isolated predicates against
synthetic facts; no actual effect, authentication, atomic transfer or cancellation
is performed. Forty corpus scenarios remain individually Partial in the
evaluation. Positive primitive results cannot compensate for those gaps.
