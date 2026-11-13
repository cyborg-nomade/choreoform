<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0085: Prefer explicit transparent type names before parameterized reuse

**Status:** Accepted under Project Owner delegation; direction only, effective on integration<br>
**Date:** 2026-11-13<br>
**Decider:** Codex under ADR-0018; no personal owner or independent approval

## Context

Signed [scope85](../evaluation/0085-type-naming-critique-plan.md) `ff075233` follows
PR #99's signed merge `1bd547425c126500fb8d83d6c0e267a5471982ea`, exact-head CI
`43404656137` and automated COMMENT review/fresh full audit `5889074244`.
The [critique](../evaluation/0085-type-naming-critique.md) was separately signed as
`7263cf9` before further read-only resolved-IR inspection. No new utility/compiler
code is needed to repeat established native type-byte measurements.

Named protection produces a real 22.25% RP-03 test-variant reduction with exact IR,
yet leaves 448421 bytes of source. ADR-0083 independently measured 70621 redundant
outer type bytes before definition/use costs. Current resolved RP-03 data/expression
types each contain 50 exact common request records; all 25 capability inputs share
one shape. Distinct nominal response/evidence members remain important. RP-01 also
has repeated type families; RP-08 still lacks a complete lowerable source. None of
these facts proves comprehension, user feedback or future type-alias savings.

## Decision

Select explicit **local transparent type names** as the next separately specified
production direction. Names are source shorthands for exact complete descriptions,
including nested nominal pins, fields/tags/scales/optional/list meanings and exact
lexically bound outcome/resource references. They create no new semantic type,
erase no nominal membership and never share logical work/storage/endpoints.
Prefer an explicit type cue at definition/use; paper spellings in the critique
are not final grammar. Plain-English accessibility still requires actual people.

Require a separate signed production scope and exact grammar/profile/companion
publication before compiler changes. Prefer source 0.18 / companion 0.17 on unchanged
C12 / IR 0.4 for exact expanded-body/revision comparison; keep all earlier profiles
closed and C13 source selection separate. Explicit managed source-only identities,
rename/reorder/fresh copy/tombstones, all unused/forward definitions, lexical
privacy/kinds/qualification and nominal preservation are mandatory.

Compute cycles and expanded type depth/node/byte costs bottom-up before allocation;
charge every cached definition and repeated use cumulatively. Retain current
1 MiB/depth 64 / parser 1M / lowering 100k/semantic shared limits. Acyclic references and
short source alone cannot prevent exponential expansion. Require actual full-case
native lowering/checking/format equivalence and measured source benefit before
claiming improvement. Do not implement the production form in this critique.

## Alternatives and consequences

New nominal resources solely for naming would alter accepted values and type
identity; they are not transparent factoring. Layout helps future inspection but
does not remove persisted repetition. Parameterized types/libraries and scoped
work/predicate templates carry substantially more binding/hygiene/instance proof;
they remain separate. Explicit local names are a finite independent step, with
added companion/version/inspection complexity and no automatic usability claim.

Benchmark adoption/archive remains separate. Full reusable receipt/work language,
RP-08, managed protected accessible inspection/editing, general export, text/visual
parity and real representative-user/host/browser/independent evidence remain
Phase 1 obligations. Aggregate executable IR and all region/semantic/exit gates
stay open. Required checks, separate automated completed-diff review, all comments /
CodeRabbit audit, successful exact-head CI and signed merge precede next work.
Parent #46 stays draft; no execution, phase completion or Phase 2.
