<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-06: Manufacturing quality hold and release

| Attribute | Value |
| --- | --- |
| Domain | Manufacturing quality |
| Complexity | Level 4 — Adaptive |
| Primary participants | Operator, quality unit, laboratory, production planning, warehouse, manufacturing systems |
| Typical duration | Hours to several months |
| Automation mix | Human-controlled quality decisions with system and physical-resource coordination |

## Purpose and corpus role

Contain potentially affected material, investigate a discrepancy, and reach an
authorized disposition without losing genealogy or allowing use during a hold.
The case stresses the relationship between information state and physical
reality, expanding scope, evidence provenance, independent decision authority,
and actions that cannot be undone merely by changing a flag.

This is a fictional language-design case, not manufacturing, laboratory,
quality-system, or regulatory guidance. Real procedures require product- and
jurisdiction-specific controls and qualified review.

## Scope and assumptions

The case begins when a discrepancy or suspect result identifies one or more
lots, batches, samples, or assets. It ends when all in-scope material has an
authorized disposition and required follow-up is transferred or closed.
Detailed test methods, hazard evaluation, regulatory reporting, and corrective
action systems are external.

## Participants

- **Operator or automated monitor:** reports the initial discrepancy and
  preserves immediate evidence.
- **Quality unit:** owns scope, investigation oversight, and final disposition.
- **Laboratory or investigator:** performs assigned evidence-gathering work.
- **Production planning and warehouse:** stop, segregate, locate, and account
  for affected physical material.
- **Manufacturing systems:** expose genealogy, inventory, status, and effect
  confirmations without replacing physical verification.

## Trigger and preconditions

A discrepancy-recorded event identifies its source, time, initial material or
equipment scope, reporter, available evidence, and immediate actions. Relevant
lots and genealogy must have stable identities; uncertainty in identity is
itself recorded.

## Information and state

- Discrepancy identity, evidence items with provenance and versions,
  investigation questions, hypotheses, findings, and approvals.
- A versioned scope graph of lots, inputs, outputs, equipment, locations, and
  related process instances.
- Logical hold instructions, provider confirmations, physical-verification
  results, samples, custody, disposition per material unit, and remaining
  obligations.

## Main success path

1. The quality unit establishes an initial scope and requests immediate holds.
2. Warehouse and production confirm system status and physical segregation;
   discrepancies remain open.
3. Investigation work proceeds, including evidence collection and any
   authorized sampling or tests.
4. Findings may expand or narrow scope through attributable revisions.
5. An authorized quality decision assigns a disposition to every in-scope unit.
6. Release, rework, destruction, or other permitted effects are executed and
   confirmed; follow-up obligations are linked before closure.

## Alternatives and failures

- Genealogy analysis can discover related material after some investigation
  branches have started; new scope receives holds and work without erasing why
  it was added.
- A system says a lot is held while physical verification cannot locate it;
  the conflict escalates and the information flag is not treated as containment.
- A sample is invalid, lost, or its custody uncertain; only affected evidence
  is invalidated, and required work is repeated or reassessed by authority.
- Conflicting findings require resolution or an explicitly recorded unresolved
  risk decision rather than last-write-wins.
- A release command that times out is reconciled before retry because physical
  use may already have resumed.
- Cancellation of one test does not cancel containment or unrelated evidence
  obligations.

## Time, concurrency, and scale

Containment is urgent, while investigation may be long-running. Holds across
locations and related lots fan out and complete independently. Evidence work
can proceed in parallel subject to sample availability, custody, equipment, and
qualified-person constraints. Scope may grow during execution. Retention and
review dates outlive the active investigation.

## Capabilities and effects

- **Query genealogy:** return versioned relationships and known uncertainty.
- **Place or remove system hold:** request an identifiable status effect and
  reconcile the provider outcome.
- **Verify physical containment:** assign accountable human work at a location.
- **Collect or test sample:** preserve method reference, custody, raw evidence,
  and interpretation as distinct information.
- **Record quality disposition:** require an authorized, attributable decision
  against a fixed scope and evidence revision.

## Invariants and protections

- Material subject to an effective hold cannot be intentionally consumed,
  shipped, or released through this process without authorized disposition.
- System status and physical containment are distinct facts and conflicts are
  never collapsed into apparent success.
- Every disposition applies to identified material and fixed evidence and scope
  revisions; changing them invalidates the affected decision.
- Original observations are retained; correction adds provenance rather than
  overwriting inconvenient data.
- The same person cannot satisfy independent roles where applicable policy
  requires separation of duties.

## Observable outcomes

- Quality can see the completeness of containment and disposition by material
  unit, location, and scope revision.
- Operations can explain why a lot was added or removed and which downstream
  work that change triggered.
- An auditor can distinguish raw evidence, interpretation, decision, requested
  effect, and confirmed physical result.

## Acceptance scenarios

### RP-06-A — Contain, investigate, release

- **Given** one identified lot with a reported discrepancy,
- **when** system and physical holds are confirmed, evidence supports release,
  and an authorized quality decision is recorded,
- **then** release is requested and closure waits for confirmed disposition of
  every unit and linked follow-up.

### RP-06-B — Scope expands through genealogy

- **Given** an active investigation with one contained lot,
- **when** verified genealogy identifies two related lots,
- **then** a new scope revision records why they were added and starts their
  containment without restarting completed valid evidence work.

### RP-06-C — Digital hold conflicts with reality

- **Given** a system reports a lot on hold,
- **when** physical verification cannot locate all of it,
- **then** containment remains incomplete and escalated even though the digital
  status call succeeded.

### RP-06-D — Ambiguous release effect

- **Given** an authorized release whose provider call times out,
- **when** the instance recovers,
- **then** it reconciles provider and physical status before retrying or
  declaring the lot released.

## Semantic pressures exposed

- How can instance scope grow while preserving the identity of earlier work?
- How are physical observations modeled alongside requested digital effects?
- How do provenance, evidence versions, and independent approvals compose?
- Can visual and text views expose incomplete fan-out and conflicting facts
  without implying a false total order?

## Out of scope

- Product-specific acceptance criteria, laboratory methods, risk calculation,
  regulatory submission, recall execution, and corrective/preventive action.

## Sources and inspiration

- [FDA, Current Good Manufacturing Practice Requirements — Records and Reports](https://www.fda.gov/drugs/guidances-drugs/questions-and-answers-current-good-manufacturing-practice-requirements-records-and-reports)
- The source highlights review, investigation, and record pressures. This case
  does not implement FDA requirements or establish compliance.
