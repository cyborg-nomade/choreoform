<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Control/availability checking: frozen engineering plan

**Date:** 2026-11-02<br>
**Baseline:** `99857fc658b98e1fcb9e82893764062a2d547833` (merged PR #21)<br>
**Authority:** Owner authorized the next item. Commit this plan before implementation.

## Scope

Add an explicit conservative `check-control` stage after ADR-0016 static checks,
not a process engine or full executable admission. Preserve existing `check` and
frozen artifacts. Propose the analysis boundary in ADR-0017 with implementation.

Check all ordinary flow/outcome coverage, implicit forks, reciprocal same-scope
split/fanout joins, direct barrier activation and acyclic control graphs. Validate
join outcome tokens against the source population. Use iterative graph traversal
and explicit work/storage budgets, not recursion over graph edges.

Analyze reachable ordinary paths from the root entry. Root input ports are explicit
launch preconditions. Initialize independent cells only after proving their initial
expression reads available from entry inputs/ancestors; do not invent local
initializer ordering. At a control merge intersect definitely available cells.
Check every declared node read before adding writes (one snapshot), including
conservative/policy dependencies. Require outputs available at every reachable
ordinary finish. Invoke checks fresh child locals, input/output boundaries,
distinct output targets and exact caller output writes; summaries may retain
definite ancestor availability, never leak child locals into the caller.

Repeat bodies are analyzed once with fresh locals and a zero-iteration path;
credit no body-only writes to continuation. Analyze split/fanout children against
the fork snapshot (fanout item input is explicit), but credit no child-only writes
to the join continuation. This conservative mode may refuse valid programs whose
availability requires scheduling/join-predicate or initializer-order proofs. Label
these unproved, not language-invalid. No constant-guard pruning or liveness proof.
Type/schema-check unreachable declarations, but do not invent entry contexts or
claim availability proof for ordinary-unreachable/exceptional work. Report coverage.

## Required evidence

- Flow duplicate/missing/cycle, orphan/unpaired/bypassed join and invalid join-token
  mutations; deep DAG checks without graph-edge recursion; bounded refusal.
- Branch intersections; reads before/after assignment; simultaneous assignment;
  missing outputs at one finish; explicit input and initial-expression assumptions.
- Fresh invocation locals, port transfer availability, duplicate output target,
  exact writes, nested/ancestor access, repeat zero iterations, conservative parallel
  handoff and fanout item initialization; no sibling/previous-occurrence data.
- Original static-stage regressions unchanged; new source/IR proof and refusal
  examples, format/reparse/revision stability, independent Python artifact/schema
  checks, native/Wasm lint/build. Diagnostics never echo values.
- ADR/evidence/roadmap updates clearly retain full protection/access/race/
  exceptional/host/clock/benchmark and Phase 1 gates.

## Stop boundary

Produce a tested review PR and pause for owner approval before merging or starting
another item. Do not close the near-English, complete validator, aggregate IR or
Phase 1 gates. No visual notation, participant recruitment or Phase 2 runtime work.
