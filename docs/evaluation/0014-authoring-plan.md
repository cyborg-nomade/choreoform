<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Frozen plan: Near-plain-English authoring comparison

**Plan date:** 2026-09-22<br>
**Status:** Frozen before candidate drafting and assessment<br>
**Framework and corpus revision:** `8a90c092fec740436ad248e88ca864f3ccead4fd`<br>
**Evaluation owner:** Proposal author; selection authority: Project Owner

## Question and boundaries

Which controlled-English surface should be developed and tested for Choreoform,
without selecting final syntax before the adopted evaluation gates pass?
Compare two equally worked paper candidates: A, named sections with structured
sentences; B, named guarded rules using `when ... then ...`. Both must expose the
same accepted semantics. A compact technical/JSON baseline and unrestricted
English interpretation are contextual alternatives, not equally worked or scored
candidates. Do not infer a familiarity advantage from the author's background.

This is a design and evaluation package, not the later complete validator,
production compiler, visual notation, engine, module linker or user study result.
No existing IR record, accepted contract, fixture binding or runtime policy may
change silently. Ordinary authors should not write JSON, checksums or opaque IDs;
identity and dependency data must remain durable and accessible, not disappear.

## Common maturity, artifacts and constraints

Freeze this file in Git before writing candidate results. Candidate revisions
will be the exact review commit, with the plan commit cited by the scorecard.
Both candidates receive a reading guide, explicit syntax/binding rules, an
IR/contract mapping, all-path worked treatments of RP-01 (Level 1), RP-03
(Level 3), RP-08 (Level 4), and scenario-level traceability for all forty adopted
scenarios. Mark any missing lowering, domain contract or unsupported corpus
behavior; a complete narrative is not a complete executable definition.

Retain every accepted core node kind and policy boundary in the mapping.
Unspecified words cannot become callbacks, authority, time conventions or
coercions. No guess-based name resolution, inferred race ordering or silent
fallback to low-level JSON. A draft with an unresolved requirement cannot lower.
Specify identity-preserving rename/copy/merge, source/IR revision boundaries,
protected summaries, explicit nondeterminism, and source diagnostics.

No product parser is required to compare paper candidates. Any repository
checker can establish only the properties it actually tests, not parsing,
semantic correctness, lowering, accessibility or user comprehension. Existing
native/Wasm checks and fixture oracles remain regression evidence only.

## Roles, tasks and perturbations

Target process readers and authors with no programming prerequisite, operational
reviewers, and implementers. English fluency, assistive technology and prior
process experience are recorded separately rather than assumed. All study data
must be synthetic and de-identified; do not recruit or contact people without
owner coordination. No participants are currently enrolled.

For both candidates, use the same case briefs and answer keys:

1. Explain normal outcomes, unknown effects and outstanding obligations.
2. Author from a case brief, then correct a wrong revision or missing authority.
3. Add a calendar deadline; revise approval-bound data; change one parallel
   branch; identify semantic versus presentation changes.
4. Diagnose stale/duplicate observations, a race, and an ambiguous payment.
5. Work through concurrent cancellation, a long-running definition upgrade,
   and a large keyed fan-out without pretending they were executed.
6. Inspect the planned text/IR/visual round trip and identify unproven mappings.
7. Read and edit without color, pointer gestures or spatial arrangement alone.

Require a reproducible human-study protocol for comprehension, authoring,
correction and review. Record timing, correctness, help, confidence and critical
misinterpretations separately. Missing studies remain missing: author simulation
and a model-generated answer are not representative-user evidence.

## Assessment and stop conditions

Use the adopted G1–G5 and twelve-criterion profile; no weighted total or winner
by arithmetic. Priorities: faithful/protected meaning, deterministic refusal,
then accessible comprehension/authoring, local changes and implementation cost.
Both candidates have B-level worked evidence at most. Leave human outcomes
unscored until observed. The same author drafts and assesses both, so independent
review remains a named limitation. No fabricated second evaluator or consensus.

Conditional gates prevent final selection. Recommend a reversible study direction
only if its trade-offs are explicit. The Roadmap item stays open while full
definitions, supported lowering or representative-user evidence are missing.
Owner approval of a working direction must not silently close those gates.
Stop before visual-notation work or any next Roadmap deliverable.
