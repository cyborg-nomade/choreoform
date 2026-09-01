<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-01: Expense reimbursement

| Attribute | Value |
| --- | --- |
| Domain | Small-business administration |
| Complexity | Level 1 — Bounded |
| Primary participants | Employee, manager, finance reviewer, accounting system |
| Typical duration | Minutes to two weeks |
| Automation mix | Human decisions with software checks and payment integration |

## Purpose and corpus role

Reimburse a legitimate business expense while enforcing policy and preserving
an understandable record of submission, correction, approval, and payment.
This deliberately familiar case anchors the corpus at low complexity while
still exposing human authority, a correction loop, policy versions, and an
external monetary effect.

## Scope and assumptions

The case begins with a reimbursement request and ends when it is paid,
rejected, withdrawn, or closed after an unrecoverable payment problem. Policy,
tax, payroll, and accounting details vary by organization and jurisdiction;
this synthetic case is not financial, tax, employment, or legal guidance.

## Participants

- **Employee:** submits the expense, supplies evidence, and may correct or
  withdraw it.
- **Manager:** decides business justification within delegated authority.
- **Finance reviewer:** resolves policy exceptions and confirms payment
  readiness.
- **Accounting system:** records the liability and performs or schedules
  payment through a capability boundary.

## Trigger and preconditions

An employee requests reimbursement with an amount, currency, date, category,
business purpose, cost allocation, and receipt or a declared reason it is
missing. The employee and cost allocation must be active, and the applicable
policy version must be identifiable.

## Information and state

- Request identity, submitter, expense details, evidence, policy version, and
  immutable revisions.
- Current review assignment, decision, comments, correction deadline, and
  payment reference.
- Distinct conditions such as draft, awaiting manager, needs correction,
  awaiting finance, approved, rejected, withdrawn, payment pending, and paid.

## Main success path

1. Automated checks identify missing fields, obvious duplicates, and policy
   conditions.
2. The manager approves the business purpose and amount.
3. Finance reviews any exception and confirms coding and evidence.
4. The accounting system accepts one payable instruction.
5. Payment confirmation closes the request and notifies the employee.

## Alternatives and failures

- A correctable submission returns to the employee with reasons; resubmission
  creates a new revision and preserves earlier review history.
- The manager or finance reviewer rejects with an attributable reason.
- A suspected duplicate is linked for human resolution rather than silently
  discarded.
- The employee withdraws before payment becomes irrevocable.
- A transient accounting failure is retried without creating a second payable;
  an ambiguous outcome is reconciled before another effect is attempted.
- Delegation or absence may reassign a pending review without changing the
  recorded decision authority.

## Time, concurrency, and scale

A correction or review may have a business-calendar deadline and reminder.
Manager and finance review are sequential in this policy variant, although
automated duplicate and policy checks may run together. A small organization
may have tens of active requests, but each monetary effect still needs a stable
request identity.

## Capabilities and effects

- **Check policy:** evaluate a versioned organizational rule set.
- **Request human decision:** assign, notify, and record a decision with the
  actor's authority at decision time.
- **Create payable:** ask an accounting provider to record exactly one payable
  for the approved revision.
- **Notify participant:** communicate requests and outcomes without making
  notification delivery equal process completion.

## Invariants and protections

- A request cannot be paid for a revision that lacks all required approvals.
- A request identity produces at most one settled reimbursement unless an
  explicit adjustment process is started.
- Editing decision-relevant information invalidates approvals for an older
  revision.
- Only authorized roles see receipt and payment data, and history attributes
  every human decision.

## Observable outcomes

- Employee and reviewers can distinguish waiting, correction, rejection,
  payment pending, payment failure, and paid.
- An operator can explain which policy and request revision each decision used.
- Reconciliation can determine whether an accounting effect occurred after an
  ambiguous response.

## Acceptance scenarios

### RP-01-A — Ordinary reimbursement

- **Given** a complete in-policy request,
- **when** the authorized manager approves and accounting confirms payment,
- **then** the instance closes as paid with one payable reference and an
  attributable decision history.

### RP-01-B — Correction invalidates review

- **Given** a manager-approved request returned by finance for a wrong amount,
- **when** the employee submits a corrected revision,
- **then** the earlier approval remains in history but cannot authorize the new
  revision.

### RP-01-C — Ambiguous payment response

- **Given** an approved request whose create-payable call times out,
- **when** the process recovers,
- **then** it reconciles using the stable request identity before deciding
  whether another call is safe.

### RP-01-D — Withdrawal races with approval

- **Given** a pending manager review,
- **when** withdrawal and approval are received nearly together,
- **then** one defined ordering determines the outcome and history retains both
  observations without permitting payment after an effective withdrawal.

## Semantic pressures exposed

- How are document revisions related to decisions made against them?
- How are authority at assignment time and authority at decision time checked?
- How is a monetary effect made idempotent and reconciled after uncertainty?
- How do text and visual forms show a correction loop without hiding history?

## Out of scope

- Corporate-card settlement, cash advances, mileage calculation, taxation,
  foreign-exchange accounting, and appeal procedures.

## Sources and inspiration

- Synthetic composite based on common reimbursement practice; no external
  policy is incorporated as a requirement.
