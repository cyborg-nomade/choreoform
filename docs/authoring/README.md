<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Near-English authoring: a working design, not final syntax

**Status:** Proposed in [ADR-0014](../decisions/0014-near-english-authoring.md).
Nothing here is accepted by the current parser. The Roadmap deliverable remains
open: representative-user evaluation, complete lowerable benchmarks and visual
parity have not happened.

The aim is to let someone describe a process using readable sentences and familiar
names, without writing JSON or managing wire IDs. The computer must still know
exactly which decision, revision, authority and effect each sentence describes.
We propose **structured sentences with named steps and explicit outcomes** for
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
