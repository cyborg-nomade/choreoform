<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Representative-user study protocol — not yet run

**Status:** Protocol proposed; zero recruited participants, zero measured results.<br>
**Owner:** Phase 1 author prepares materials; Project Owner coordinates recruitment
and approves study scope. Participation and publication require explicit consent.

The objective is to learn whether A or B lets non-programmers comprehend, author,
correct and review the same processes without dangerous misinterpretation. This
is not an experiment to confirm that A is better. The
[frozen comparison plan](../evaluation/0014-authoring-plan.md) predates drafting.
Freeze actual materials, revisions and success criteria again before enrollment.

## 1. Prerequisites and participants

Before recruiting, remove ellipses and unresolved requirements from each study
task's bounded fragment. Provide all referenced definitions/policy details and
explicit input facts; pilot the answer keys with an independent reviewer.
Full-process tasks need complete lowerable benchmarks first. Do not ask a
participant to author an impossible feature or penalize finding a contract gap.
The current walkthroughs are material for preparing tasks, not ready-to-run
full-process study instruments.

Proposed formative sample: 8–12 consenting adults who work with process decisions,
including readers/reviewers and occasional authors from at least the three case
domains where feasible. No programming experience is required; record it without
assuming it predicts success. Include participants with different English
proficiency and assistive-technology needs, including keyboard-only and screen
reader use, through accessible recruitment. Do not claim those populations are
represented if they are not recruited. This small formative study cannot prove
population-wide accessibility or statistically rank languages.

Record pseudonymous participant ID, process familiarity, self-reported programming
and English experience, chosen access setup, and assistance used. Do not collect
real cases, credentials, employee performance data or medical details. Let people
withdraw. Store only agreed de-identified observations; no recording by default.
Agree retention/deletion and publication with participants before collection.
Do not upload participant data or recordings to this public repository by default.

## 2. Procedure and controls

Use equally complete A/B fragments of the same semantics. Give each participant
a neutral 10-minute reference walkthrough per candidate and allow reference use
during tasks. Counterbalance order (half A then B, half B then A) and alternate
equivalent case variants to reduce recall. Publish which variants/order were used.
The facilitator must not coach toward the recommended candidate. Record help and
clarifications verbatim where consent permits. Separate assisted and unassisted
success. Candidate labels should not disclose the author's preference.

Proposed session length: up to 60 minutes, with breaks or split sessions as needed.
Timing is descriptive; accessibility accommodations are not errors or automatic
failures. Stop a task on participant request or its agreed time budget. A second
reviewer independently checks task outcomes; retain disagreements, do not average
away critical interpretation failures.

## 3. Common tasks and answer keys

| Task | Brief, identical meaning for A/B | Correctness key and evidence to record |
| --- | --- | --- |
| C1 comprehension, RP-01 | A payable request timed out; no later provider evidence. Is it paid, failed or unresolved, and who must act? | Unresolved, owned reconciliation; no second unconstrained payment. Record explanation and confidence, not a guessed checkbox alone |
| C2 comprehension, RP-03 | Cancellation and packed confirmation conflict. Does moving one rule above another let cancellation win? | No; guarded commit and provider facts govern. Request is not a confirmed stop |
| C3 comprehension, RP-08 | New asset appears after a batch is sealed. May the old join quietly include it? | No; a new explicit occurrence/batch, with earlier evidence retained |
| A1 authoring, RP-01 | From a brief, add a human finance review with approved/correction/rejected outcomes | Correct actor/input/work rule/result binding and every explicit successor; no implicit approval or back-edge loop |
| A2 authoring, RP-03 | Describe waiting for confirmation after a shipment request | Separate request/evidence, exact correlated wait and reachable owned failure/unknown handling |
| F1 correction, RP-01 | A revised amount incorrectly reuses the former manager approval | Identify exact dependency and required renewed decision; preserve old evidence and IDs |
| F2 correction, RP-08 | A tool recommendation currently appears able to dispatch containment alone | Add independent required human/capability authority, not simply a reassuring label |
| R1 review, all | Compare comment/name/format changes with instruction, policy and deadline changes | Distinguish source-only from semantic change; flag stale identity bindings and unsupported calendar shorthand |
| R2 review, RP-03 | One carrier event is duplicated and one capture outcome is unknown | Explain one accepted transition per observation and continued ownership of unknown effect |
| X1 access, all | Find and edit authority/settlement clauses without spatial navigation/color | All required meaning discoverable and editable using chosen access setup; capture barriers rather than infer success from text availability |

Use the [benchmark acceptance answers](benchmarks.md) as background, not as
unseen assumptions. Supply exact scenario facts and show no preferred answers
before tasks. Compare a generated-companion editing exercise with inline-ID
anchors as a separate exploratory packaging task; do not confound it with the
A/B wording score. Include one plain-editor rename conflict and its recovery.

## 4. Measures and decision rules

Record task-level correct/partial/incorrect/unattempted, explanation, elapsed
time, assistance, confidence and concrete errors. Do not combine these into one
score. Report sample/context and distributions; do not present percentages from
a tiny convenience sample as population estimates.

Predeclare these critical errors: treating unknown payment as safe to repeat;
assuming rule order grants authority or wins a race; silently dropping unsealed
members/unfinished work; treating a redacted view as a complete approvable
definition; mistaking an old approval for current-revision authorization.
Any uncorrected critical error requires investigation and another design/test
iteration before a final syntax recommendation, not majority-vote compensation.
This is a conservative project study rule, not a certification standard.

Record whether confusion arises from sentence syntax, domain knowledge, missing
contract/tool behavior or material preparation. Missing material invalidates that
task comparison; do not count it against a candidate. If order/familiarity seems
to explain a preference, retain that uncertainty. Participant preference alone
does not override incorrect interpretation or G1–G4.

## 5. Current results and closure

| Evidence | Present state |
| --- | --- |
| Participants, consent, task sessions | Not collected |
| Independent task-key pilot | Not performed |
| Task-level raw observations and accessibility barriers | Not collected |
| Independent scoring/reconciliation | Not performed |
| Full lowerable benchmark materials | Not complete; named gaps in walkthroughs |
| A/B usability or accessibility winner | Not established |

Before calling the Roadmap item complete, the Project Owner must review the
de-identified findings, complete-source evidence and remaining gate decisions.
Record the actual study revision, sample, method, failures and changes needed.
If recruitment is not feasible, keep H1 unscored and G4 conditional; owner review
of a paper design cannot stand in for representative-user evidence.
