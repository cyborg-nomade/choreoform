<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Near-English authoring: a working design, not final syntax

[ADR-0090](../decisions/0090-pure-predicate-recipes.md) contracts source0.19 /
companion0.18 typed pure Boolean tests on unchanged C12/IR0.4. The native prototype
expands explicit named inputs and checks their complete types in actual caller
contexts before emitting a candidate. [Finite complete variants](../evaluation/0090-pure-predicates.md)
reduce RP-03 from410752 to291309bytes and RP-01 from55453 to54172bytes while
preserving exact current IR. PR #105 passed required CI and delegated automated review before integration. These are
experimental alternatives; the separately archived working RP-03 adoption is
described below, while RP-01 adoption remains a separate scope.
This supplies no representative-user evidence or complete language/phase gate.

The current working [RP-01 package](../benchmarks/rp01/README.md) adopts source0.19 /
companion0.18 on unchanged C12/IR0.4 under [ADR-0092](../decisions/0092-rp01-pure-test-adoption.md).
Payment receipt agrees factors the complete work predicate and late-classifier
inner condition, with explicit Result/Instruction inputs and every caller guard
preserved. Full source54172bytes/9145tokens is2.31% below source18 at55453. The
finite producer matches the verified experiment and preserves all11 reports/IR/
resources/guard pins exactly; legacy source18 production stays byte-identical.
Its59-file prior package was archived before code. PR #107 integrated after required checks, delegated automated review, CI and
full audit; no user, runtime, complete language or phase result is claimed.

The current working [RP-03 package](../benchmarks/rp03/README.md) adopts source0.19 /
companion0.18 on unchanged C12/IR0.4 under [ADR-0091](../decisions/0091-rp03-pure-test-adoption.md).
Two complete pure tests reduce the full source from410752 to291309bytes (29.08%),
with41306 tokens/386333 native parser visits. Full canonical IR, five reports,
resources/receipts/guards/semantic identities remain exact. Its previous126-file
source18 package was archived before code. PR #106 integrated after required final checks, delegated automated review, CI
and full audit; no representative-user or phase evidence follows.
RP-01 source19 adoption is a separate working scope as described above.

The [source 0.18 / companion 0.17 profile](profile-v0.18.md) adds explicit local
transparent type descriptions on unchanged C12 / IR 0.4. `Describe type ... as ...`
and `type ...` uses preserve the complete existing type, including nested nominal
pins, tags and lexical visibility. A graph preflight checks unused/forward/cyclic
references and cumulative expanded depth/work/bytes before building cached types.
Actual combined test variants are 55702 bytes for RP-01 and 412288 for RP-03, with
unchanged expanded meanings and the legitimate external-effects refusal. These are
additional reductions of 0.64% and 8.06% over protection-only variants, not user
comprehension evidence or current benchmark adoption. See
[the implementation evidence](../evaluation/0086-local-type-names.md).

The [source 0.17 / companion 0.16 profile](profile-v0.17.md) adds explicit named
complete data protection on unchanged C12 / IR 0.4. Each `Keep ... protected as ...`
use expands to the same policy, sensitivity, purposes, participants and capabilities;
there is no ambient default or permission grant. Source-only managed identities,
per-data purpose identities and cumulative expansion/work/output bounds are checked.
New test variants reduce RP-03 from 576759 to 448421 bytes (22.25%) with its exact
current IR/revision and concurrency refusal. RP-01 becomes 56059 bytes; comparison
uses an explicit C12 baseline promotion of its current C11 source. Current benchmark
files stay fixed. See [the implementation evidence](../evaluation/0084-named-data-protection.md).
Protected accessible inspection/editing, visual parity and actual user studies are
still required before the near-English Phase 1 gate can close.

The [source0.16 / companion0.15 profile](profile-v0.16.md) selects core0.12/IR0.4
for purpose-only, exactly qualified prior-activity identity. Ordinary private values /
outcomes/policies/control remain lexical and owned-source/live host gates stay fixed.
Whole RP-03 passes static, control, protection and strict invalidation checks,
then retains the external concurrency refusal. Core0.13 adds raw SDK descriptor
selection only; this source profile remains C12. The
[request-region audit](../evaluation/0082-request-region-proof-audit.md) keeps
physical alias/revision/lifetime proof open and prioritizes independent near-English
factoring. Managed protected accessible text/visual editing remains Phase1 work.

The [source 0.14 / companion 0.13 profile](profile-v0.14.md) explicitly selects
existing core 0.11/IR 0.4 for finite capture-first commands/compensators and pure
local inputs. Existing sentences/quotas remain; bounded native work/class/input /
compensation/premise disclosure preserves old query meanings and actual host gates.

The [source 0.13 / companion 0.12 profile](profile-v0.13.md) explicitly selects
existing core 0.10/IR 0.4 for finite read-only intermediate owned entry before
original capture. Existing sentences express the layout; bounded native reports
expose optional entry/capture relations and mandatory live host requirements.
Older profiles and capturing-first/passive-leaf output remain fixed.

The [source 0.12.0 / companion 0.11.0 profile](profile-v0.12.md) selects existing
core 0.9/IR 0.4 for finite read-only surviving continuations. Existing sentences
and quotas remain; bounded native conditional scope/gate disclosure stays separate
from ordinary/legacy coverage and actual host acceptance. Earlier profiles stay fixed.

The [source 0.11.0 / companion 0.10.0 profile](profile-v0.11.md) adds readable
original-request retention on existing core 0.8/IR 0.4 with bounded declarations
and separate owned proof. Actual storage/current authority and permitted new work /
cutoff/handoff/all-settlement remain open.

The [source 0.10.0 / companion 0.9.0 profile](profile-v0.10.md) adds readable
handler-local fault history declarations on core 0.7/IR 0.3, with separate bounded
conditional reports. Actual events, owned continuation and general tool-managed
English editing remain open.

The [source 0.9.0 / companion 0.8.0 profile](profile-v0.9.md) names general
provider fault sources once with `Recognise faults from "Source".` and retains
short clock/basis retry clauses. It selects core 0.6/IR 0.2 and shares native
catalogues per exact pin; actual fault history/handler entry remains separate.

The [source 0.8.0 / companion 0.7.0 profile](profile-v0.8.md) adds readable
terminal-source/origin/result capture, explicitly selecting core 0.5 and IR 0.2.
Bounded reports disclose declarations and remaining host requirements. Owned
recovery storage and full plain-English/tool-managed editing goals remain open.

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

[Source 0.15 capacity](profile-v0.15.md) explicitly keeps core 0.11/IR 0.4 and
source-0.14 meaning while allowing 131,072 program tokens; all old/other limits
remain. It recognizes the whole order grammar but retains an actual private
compensation reference refusal. It supplies no benchmark adoption or readability /
semantic/host/phase closure. See [measured evidence](../evaluation/0068-source-capacity.md).
