<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0006: Adopt a representative process corpus

**Status:** Accepted<br>
**Date:** 2026-09-01<br>
**Decider:** Project owner

## Context

Choreoform intends to be a general information-process language rather than a
notation optimized for one industry or one style of automation. Before the
project chooses formal semantics or notation, the Roadmap therefore calls for
8–12 representative processes spanning multiple domains and complexity levels.

A collection of titles or happy paths would not provide enough evidence. It
would be easy to select familiar office approvals and miss long-running work,
concurrency, physical resources, late and duplicate events, external effects,
human judgment, recovery, privacy, or version changes. At the other extreme,
attempting authoritative models for regulated industries would turn an early
language-design corpus into operational or legal guidance it cannot safely be.

The corpus must expose design pressures without prematurely defining language
constructs. The following Roadmap deliverable will establish criteria for
evaluating competing semantic and notation designs; this decision selects the
evidence those criteria will be applied to.

## Decision criteria

The corpus should:

1. span materially different domains, organization sizes, durations, and
   combinations of human, software, and physical work;
2. exercise sequence, choice, repetition, concurrency, waiting, recovery,
   cancellation, compensation, and change over time;
3. make information, state, external effects, and observable outcomes explicit;
4. include safety-, privacy-, security-, or compliance-sensitive pressures
   without presenting the examples as professional guidance;
5. be detailed enough to compare textual and visual designs and later derive
   executable acceptance examples;
6. remain understandable to contributors without domain-specific tooling; and
7. preserve semantic freedom by describing scenarios and invariants rather than
   proposed syntax or notation.

## Decision

Adopt the ten synthetic composite cases in
[`docs/representative-processes`](../representative-processes/README.md) as the
Phase 0 representative process corpus:

1. expense reimbursement;
2. customer-support SLA escalation;
3. e-commerce order fulfillment;
4. employee onboarding;
5. clinical referral triage;
6. manufacturing quality hold and release;
7. data-pipeline backfill;
8. cybersecurity incident response;
9. contract review and approval; and
10. subscription renewal and payment recovery.

Each case follows a common template. It identifies scope, participants, trigger,
information and state, a main path, alternatives and failures, time and
concurrency, capabilities and effects, invariants, observable outcomes,
acceptance scenarios, semantic pressures, and exclusions. A corpus index maps
the cases to major pressures and defines a four-level navigation scale.

The cases are non-normative requirements evidence. They are not Choreoform
syntax, a conformance suite, executable process definitions, reference
architectures, or instructions for operating a business. In particular, the
healthcare, quality, security, employment, billing, and contract cases are not
medical, regulatory, security, employment, financial, or legal advice. A real
deployment must be adapted and reviewed by qualified people in its jurisdiction
and context.

The corpus records outcomes and tensions, not a required implementation. A
future design need not encode every domain phrase as a core feature; it must
show how its general concepts can express the underlying pressure or explain
why the pressure is outside Choreoform's scope.

Routine review may clarify a case, add an acceptance scenario, improve a source,
or add a supplementary case. Replacing or removing one of the ten adopted
cases, materially narrowing the coverage model, or turning a case into a
normative requirement requires a new ADR. Formal semantics and the selection of
the first vertical-slice cases remain separate decisions.

## Options considered

| Option | Advantages | Costs and risks | Outcome |
| --- | --- | --- | --- |
| **Balanced synthetic composite corpus** | Broad pressure coverage; safe to publish; comparable structure; domain-neutral evidence | Simplifies real operations; needs later validation with practitioners | Adopt |
| One real domain in depth | High fidelity and coherent vocabulary | Strong domain bias; may contain confidential or regulated details | Reject |
| Abstract feature microexamples | Isolates language constructs; easy to test | Presupposes constructs and misses interactions and operational context | Reject |
| Exhaustive industry catalog | Very broad discovery surface | Unbounded, shallow, and difficult to review or maintain | Reject |

The adopted option uses realistic composite scenarios rather than claiming to
reproduce a particular organization's process. External sources provide
pressure checks, not imported specifications.

## Consequences

- Semantic and notation proposals gain a shared, traceable body of scenarios
  rather than relying on invented examples during review.
- The corpus deliberately includes conflicting pressures: deterministic audit
  history alongside human discretion, aggressive automation alongside guarded
  effects, and stable definitions alongside long-running instances.
- Ten substantial cases cost more to maintain than a feature checklist, and
  their simplified domain assumptions must remain visible.
- Coverage breadth does not prove universality. Practitioner research and new
  cases will still be needed as the language matures.
- The next Roadmap deliverable can define an evaluation method without choosing
  its evidence after seeing which design performs best.
- Phase 1 can select a smaller vertical slice while retaining harder cases as
  forward-looking checks.

## Confirmation

The decision is implemented when:

- the index lists exactly ten adopted cases across at least eight domains and
  all four complexity levels;
- every case contains the required template sections and at least three
  observable acceptance scenarios;
- the coverage matrix includes human and automated work, concurrency, time,
  external effects, recovery, cancellation or compensation, sensitive
  information, and definition change;
- repository entry points link to the corpus; and
- the corpus is reviewed as requirements evidence without adopting syntax or
  formal semantics.

## Sources

- [Workflow Patterns](https://mitpress.mit.edu/9780262029827/workflow-patterns/)
- [NIST SP 800-61 Rev. 3, Incident Response Recommendations and Considerations for Cybersecurity Risk Management](https://csrc.nist.gov/pubs/sp/800/61/r3/final)
- [FDA, Current Good Manufacturing Practice Requirements — Records and Reports](https://www.fda.gov/drugs/guidances-drugs/questions-and-answers-current-good-manufacturing-practice-requirements-records-and-reports)
- [Google Cloud, Dataflow pipeline lifecycle](https://docs.cloud.google.com/dataflow/docs/pipeline-lifecycle)
- [NHS England, National elective access policy](https://www.england.nhs.uk/long-read/national-elective-access-policy/)
- [Stripe, Using webhooks with subscriptions](https://docs.stripe.com/billing/subscriptions/webhooks)

## Acceptance and action items

The Project Owner approved this ADR on 2026-09-01. The representative process
corpus becomes effective when pull request #7 is merged.

1. [x] Obtain Project Owner approval.
2. [x] Change this ADR's status to Accepted and record the approval.
3. [x] Publish the corpus index, template, coverage matrix, and ten cases.
4. [x] Link the corpus from contributor entry points and the decision index.
5. [x] Mark the Roadmap deliverable complete after approval.
6. [ ] Define design-evaluation criteria in the next Phase 0 deliverable.
7. [ ] Validate and extend the cases with practitioners as implementation
   domains are selected.
