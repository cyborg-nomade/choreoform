<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0014: Develop structured English sentences as a provisional authoring direction

**Status:** Accepted — provisional authoring direction<br>
**Date:** 2026-09-22<br>
**Decider:** Project Owner

## Context

The Project Owner wants the text language as close to plain English as practical
for broad accessibility. ADR-0011's JSON-record parser is an explicit technical
baseline, not that language. ADR-0012/0013 now constrain names, exact types,
closed expressions and policies, making it possible to study a readable surface
without inventing semantics in the parser.

Familiar wording can also mislead: “when” may suggest a global trigger, “paid”
may hide an unknown provider outcome, and “three business days” requires more
than English interpretation. Ordinary authors should not manage opaque IDs or
JSON, but stable identity, complete policy meaning and exact dependency pins
must survive editing and source-control merges.

The [frozen plan](../evaluation/0014-authoring-plan.md) and
[evaluation](../evaluation/0014-near-english-authoring.md) compare two paper
surfaces with the same semantics. No participants were recruited, no full new
compiler was built, and all three full lowerable benchmarks retain gaps.

## Decision criteria

1. Preserve accepted meaning and fail closed on ambiguity or unsupported rules.
2. Make human authority, revision evidence, effects and remaining obligations
   visible to readers and editable without JSON or spatial-only operations.
3. Prefer ordinary verbs and multiword names while keeping exact token boundaries.
4. Preserve identity and unknown metadata through explicit, portable editing.
5. Base final learnability/accessibility claims on representative people, not
   the author's taste or similarity to familiar English.

## Decision

Adopt **Candidate A: named steps with structured sentences and explicit outcome
clauses**, as the reversible direction for the next iteration of this same
authoring deliverable. Retain Candidate B's local guarded rules as the matched
study alternative. Do not select a final language or promise source compatibility.

Use quoted exact multiword names, fixed sentence forms, explicit block endings
and outcome destinations, with no control meaning inferred from layout. Spell
work as “Ask”, capability effects as “Request”, observations as “Wait”, and
completion as “Finish”, without erasing their distinct policies. Expose the
entire closed type/expression/policy vocabulary through typed language forms;
unsupported forms must not become raw JSON or arbitrary step callbacks.

Prefer a portable, tool-managed binding companion to user-maintained wire IDs.
Authors can work in a plain editor and use explicit identity-aware rename/copy
operations. Missing or stale bindings must not cause guessed identity recovery.
The companion format and complete grammar need a reviewed specification and
tests before implementation can claim identity-preserving lowering.

G1–G4 remain conditional. Approval authorizes a **working direction only**;
the Roadmap item stays open until complete lowerable benchmark and representative-
user evidence is supplied. It does not authorize automatically advancing to
visual notation or the next Roadmap deliverable. No accepted IR/contract is
superseded, no new runtime feature is added, and the old parser remains unchanged.

## Options considered

| Option | Advantages | Costs and risks | Outcome |
| --- | --- | --- | --- |
| A: structured sentences and named steps | Local action/outcome structure; distinct verbs for human/effect/evidence; no global firing implication | Repeated names, quotes and explicit policy detail are verbose; usability untested | Develop provisionally, test with people |
| B: local `when ... then ...` rules | Uniform condition/action reading; may suit rule-oriented authors | Repeated “when”; may imply persistent triggers or first-rule priority absent from semantics | Retain as matched study alternative |
| Compact declaration/JSON baseline | Existing parser/IR fidelity and reproducible tests | Requires technical bookkeeping; does not meet ordinary-author goal | Retain as implementation baseline, not final authoring choice |
| Unrestricted English or custom phrase callbacks | Freer phrasing and potentially low initial writing effort | Interpretation/callback semantics can diverge across tools; intent and authority may be guessed | Reject as authoritative source; any future assistant outputs reviewed controlled source |

The A/B comparison is paper-level; no measured implementation or human ranking
exists. Quoted names and exact keywords trade naturalness for deterministic
boundaries. The user study may favor different wording, fewer quotes, or B.

## Resolved review questions — approved 2026-09-22

1. **Degree of English freedom:** use a closed controlled-English grammar with
   quoted names initially, not pronoun/synonym/intent inference. Test less
   punctuation later rather than promising it is already unambiguous.
2. **Surface structure:** develop A while retaining equally complete B study
   tasks. This is a provisional preference based on local explicitness, not a
   final accessibility verdict.
3. **Identity packaging:** prefer tool-managed text plus binding companion, with
   explicit plain-editor reconciliation and no hidden database dependency.
   Compare inline anchors in a separate editing task before locking the format.
4. **Completion bar:** keep this Roadmap item open. Prepare complete bounded
   study materials, resolve full-benchmark contract/encoding gaps, then coordinate
   representative-user evaluation. Do not mark a study protocol as a study result.

## Consequences

The next compiler iteration has a bounded surface and explicit mapping checklist.
It cannot hide scope/race/protection decisions in prose or regenerate identity
from changing names. Authors need more policy detail than unrestricted English
might suggest, and tool-assisted binding maintenance becomes a real dependency.
Localizing the language requires a tested profile, not translating source at
runtime. Protected metadata and summaries remain an enforcement/interface issue.

The whole-case walkthroughs expose unresolved requirements: rich calendar
construction, order aggregation with accepted exact operations/item interfaces,
ad hoc incident work and partial-order outage import. They must be resolved or
explicitly bounded in further review, not silently added to current contracts.
Complete source/IR benchmarks and user evaluation remain within this deliverable;
production validation, visuals and execution remain their separately authorized
Roadmap work. No date or externally coordinated study is assumed.

## Confirmation

Inspect [syntax](../authoring/syntax.md), [mapping](../authoring/mapping.md),
[benchmark walkthroughs](../authoring/benchmarks.md), forty scenario rows and
[study protocol](../authoring/study.md). Repository checks establish package
inventory, references and unchanged baselines only. The full grammar, identity
editing, deterministic lowering and protected source must later have executable
positive/negative tests. Human tasks need recorded participant evidence.

Stop final language selection for any conditional/failed gate. Reconsider this
direction if readers confuse request/confirmation, miss ownership, infer rule
order or cannot perform predictable source edits. Keep evidence revisions and
critical errors even if a later candidate changes the preferred wording.

## Acceptance and action items

The Project Owner approved ADR-0014 and PR #18 on 2026-09-22 with all four
recommendations, preferring Candidate A because its form is shorter and simpler.
This is the owner's design judgment, not a measured representative-user result.
The accepted scope is controlled English with exact quoted names initially,
Candidate A as the provisional direction with B retained for matched study,
portable tool-managed identity bindings, and an unchanged completion bar.
Merge of [PR #18](https://github.com/cyborg-nomade/choreoform/pull/18) makes this
working-direction decision effective. Final syntax, source compatibility,
complete lowering and accessibility remain unproven; the Roadmap item and G1–G4
stay open. Further implementation of this same item requires its own review;
no next Roadmap item is started by this acceptance.

1. [x] Review the four questions and the bounded recommendation.
2. [x] Record approval/status/index without closing the entire authoring item.
3. [ ] Complete and test grammar/binding-package specifications and full source/IR
   benchmark representations in further authorized work on this item.
4. [ ] Coordinate representative participants; run and review the study results.
5. [ ] Reassess all gates before final selection; pause before the next Roadmap item.
