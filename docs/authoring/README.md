<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Near-English authoring: a working design, not final syntax

The [source 0.7.0 / companion 0.6.0 profile](profile-v0.7.md) adds an explicit
retry fault-source clause and selects core 0.4. Typed source/provider declarations
remain separate from actual faults or retry permission.

The earlier [source 0.6.0 / companion 0.5.0 profile](profile-v0.6.md) explicitly selects
core 0.3 and derives finite provider-recipient metadata reads. Older profiles keep
their original cores, limits and integrity domains. Full benchmark, validation,
editing and actual user gates remain open.

The earlier [source 0.5.0 / companion 0.4.0 profile](profile-v0.5.md) adds explicit
whole-program quotas and actual-use reconciliation endpoints for larger benchmark
drafts. It preserves private data and older profile limits; lowering remains
separate from semantic admission.

The earlier [source 0.4.0 / companion 0.3.0 profile](profile-v0.4.md) adds narrowly
qualified wait outcomes inside a designated reconciliation follow-up. The
[owned service example](examples/reconciliation.choreo) remains a synthetic
declaration/body fixture, not a complete benchmark or live handoff. See the
[diagnostic guide](diagnostics.md), [full benchmark readiness audit](../evaluation/0034-benchmark-readiness.md)
and [forty-scenario accountability](../evaluation/0034-corpus-accountability.md).

The earlier [source 0.3.0 / companion 0.2.0 profile](profile-v0.3.md) adds an
explicit English retry clock/basis sentence and selects core 0.2.0. The earlier
source 0.2.0 package remains supported with its original grammar/core/digests.
This version evolution supplies no user-study, full compiler or host execution
evidence; the broader authoring and ordinary tool-managed editing work stays open.

**Status:** Accepted working direction in [ADR-0014](../decisions/0014-near-english-authoring.md),
owner approved 2026-09-22, effective upon merge of PR #18; not final syntax.
The technical JSON-record parser remains unchanged. A bounded
[Rust authoring frontend](../../crates/authoring-frontend/README.md), approved in PR #20, implements
syntax/binding consistency and partial **candidate** IR lowering, not semantic
admission or an executable compiler. The Roadmap deliverable remains
open: representative-user evaluation, complete lowerable benchmarks and visual
parity have not happened.

The grammar/package iteration is accepted provisionally in
[ADR-0015](../decisions/0015-authoring-grammar-bindings.md): a concrete
[Candidate A grammar/profile](profile.md), [portable identity package](bindings.md)
and [executable specification checks](../evaluation/0015-authoring-grammar-bindings.md).
Owner approved 2026-10-04, effective upon merge of PR #19. Read those for the
current specified encoding. The A/B paper comparison below is
retained as the original ADR-0014 evidence; it is not retroactively upgraded into
compiler or user-study evidence.

For the current implementation review, read the
[frontend evidence and remaining work](../evaluation/0015-rust-authoring-frontend.md),
[synthetic source example](examples/review.choreo) and
[its identity companion](examples/review.bindings.json). The comparison table below
describes the original paper proposal, not this later implementation slice.

The accepted [static-checking stage](../decisions/0016-static-contract-checks.md)
adds supported named-type/provider readers and explicit checks for closed payloads,
types, facts and dependencies. [Evidence and limits](../evaluation/0016-static-contract-checks.md)
remain separate from full semantic admission, clock/host support and user evaluation.

The aim is to let someone describe a process using readable sentences and familiar
names, without writing JSON or managing wire IDs. The computer must still know
exactly which decision, revision, authority and effect each sentence describes.
The accepted direction is **structured sentences with named steps and explicit outcomes** for
further study, not unrestricted English interpreted by a model.

## A first look

Candidate A, structured sentences (an illustrative fragment, with referenced
declarations omitted here):

```text
Step "Manager review".
  Ask "Manager" to decide using "Current request" under "Review rules"
    and record in "Manager decision".
  On "approved", continue with "Finance review".
  On "correction needed", continue with "Request correction".
  On "rejected", continue with "Record rejection".
End step.
```

Candidate B, guarded rules, expresses the same node and flows:

```text
Rule "Manager review".
  When entered, then ask "Manager" to decide using "Current request"
    under "Review rules" and record in "Manager decision".
  When the result is "approved", then start "Finance review".
  When the result is "correction needed", then start "Request correction".
  When the result is "rejected", then start "Record rejection".
End rule.
```

In both, the words inside quotes are exact names, not instructions to guess an
actor or find a similarly named step. The rules for the review must state the
human instructions, authority, revision checks and allowed result type. “Ask”
does not grant permission or let a timer make the manager's decision.

## Read this review package in order

1. [ADR and review questions](../decisions/0014-near-english-authoring.md): what
   to decide now and what must remain provisional.
2. [Candidate syntax](syntax.md): names, punctuation, control, expressions and
   explicit policy clauses; both candidates use the same meanings.
3. [Mapping and identity](mapping.md): lowering obligations, durable IDs, safe
   edits, protected source, dependencies and known encoding gaps.
4. [Three benchmark walkthroughs](benchmarks.md): whole-case paper treatments,
   not just the main paths; unresolved contracts are visible.
5. [Evaluation](../evaluation/0014-near-english-authoring.md) and
   [study protocol](study.md): observed desk evidence versus work requiring people.

## What is and is not demonstrated

| Present in this proposal | Not demonstrated |
| --- | --- |
| Two comparable paper surfaces and all ten core-node mappings | A new compiler or complete grammar implementation |
| Explicit handling of policies, identity and uncertain effects | Enforceable provider/clock/identity contracts |
| RP-01/03/08 walkthroughs including failures and corpus gaps | Complete executable benchmark definitions or runtime traces |
| Forty scenario rows, a frozen comparison plan and proposed study protocol | Any participant results, accessibility certification or usability scores |
| Repository evidence/link checks and existing regression tests | Text/IR/visual round trips or final language selection |

English-like appearance is a hypothesis about usability, not proof of it.
Quoted names, repeated references and visible policy details add reading effort.
The study must test whether those costs are preferable to implicit meaning.
English-first is not universal accessibility; localized keywords need separately
specified, tested profiles with equivalent semantics, not machine translation
at execution time.

## Research informing the comparison

Gherkin illustrates readable keyword-led specifications, but its steps acquire
behavior through matching step definitions. Our inference is that borrowing its
appearance alone would not supply Choreoform's portable semantics; this design
uses a closed mapping instead of user-authored step callbacks.
[Gherkin reference](https://cucumber.io/docs/gherkin/reference/).

Kuhn's controlled-language evaluation work uses human experiments to test
interpretation against situations. We take the need for measured interpretation,
not that study's particular visual task or its results, as relevant precedent.
The process-specific tasks here also test authoring, correction and review, and
provide text-only evidence alternatives.
[How to Evaluate Controlled Natural Languages](https://arxiv.org/abs/0907.1251).
