<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0083: Factor explicit data protection before scoped work reuse

**Status:** Accepted under Project Owner delegation; direction only, pending parent/main integration<br>
**Date:** 2026-11-12<br>
**Decider:** Codex under ADR-0018; no personal owner or independent review claimed

## Context

Signed [scope83](../evaluation/0083-near-english-factoring-critique-plan.md)
`b81edda` precedes the separately signed
[critique](../evaluation/0083-near-english-factoring-critique.md) `e995b48` and
read-only native measurements, after #97 merge/CI/automated review. Candidate A
remains provisional; broad plain-English accessibility requires actual people.
RP-03 source contains 155 copies of an 866-byte data protection clause, all
resolving to one exact IR protection value. Its 133364 repeated-byte upper bound
is 23.1% of the full file before reuse overhead. Types/expressions have different
reuse opportunities; RP-08 still lacks a full lowerable source package.

## Decision

Choose explicit **named data protection definitions** as the first separately
specified production factoring slice. Reuse the complete policy/sensitivity /
purpose/participant/capability envelope without ambient defaults or new permission.
Each data use names the definition; bind its references in its declared scope,
then retain use-site visibility, type, invalidation, protection and flow checks.
No provider claim/resource is repinned or widened. Do not instantiate or share
work/storage/endpoints/retention/capture/compensation as part of this slice.

Require a separately signed grammar/companion/identity/bounded lowering/diagnostic
contract before production code. Prefer new source0.17/companion0.16 targeting
unchanged C12/IR0.4 so actual full-source expanded bodies/revisions can be compared
exactly. Keep C13 source selection separate and all earlier versions unchanged.
The proposed 15% RP-03 byte reduction target must be measured on real lowered
packages; current upper bounds and paper sketches are not implementation evidence.

## Alternatives and consequences

Local type names are promising (70621 repeated type bytes in RP-03) but protection
is the larger exact, resolved-identical opportunity. Parameterized service and
receipt helpers may ultimately reduce more, yet require hygiene, typed fact/input
parameters, unique logical work and exact P3 recipient/endpoint/ownership handling.
Unrestricted verbs, string substitution and larger quotas do not supply that proof.
Formatting/progressive disclosure helps but does not remove source repetition.

Named definitions can hide exposure; require visible inspectable meaning and
protected accessible summaries, with no implication of current actor permission.
Preserve exact nominal types, stable source-only identities, tombstones/explicit
reconciliation, semantic data IDs and resource pins. Bound total expansion/work /
output cumulatively and fail without partial IR. General export/editing and actual
text/visual/user evaluation remain later work within this same Phase 1 item.

This ADR selects a reversible direction and first scope, not final wording or
syntax. The metric utility changes no production library/parser/core/source
behavior or existing benchmark/artifact/report bytes. No measured comprehension,
actual user feedback, host execution or generic region independence is claimed.
Validate metrics/bounds/identity assumptions, run required final-head checks,
separate automated review/full comment audit and signed expected-head merge.
Parent #46/all unproved Phase 1 gates remain open; no Phase 2.
