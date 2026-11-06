<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-01 scenario conformance: frozen audit plan

**Date:** 2026-11-06<br>
**Baseline:** `a89e481cddae40fea18684953d421bcdf0a1447e`, draft #46 after #52<br>
**Authority:** ADR-0018; original RP-01 A–D and whole-package plan

All ten existing partial static stages now pass. Audit the actual complete source,
resources and stage evidence against each original A–D requirement and alternative;
do not equate that milestone with executable admission or complete scenarios.
Record exact source declaration IDs, available judgments, missing checkable policy/
transition evidence, actual host requirements and unexpressed continuation/retention
needs. Keep authentic external evidence separate from synthetic static inputs.

Use temporary native probes for missing/weak guard counterexamples, unmapped
retry-fault classification and attempted observation-result capture. Record actual
lower/stage/refusal outcomes, not hypothetical process runs. Do not weaken source
or contracts in the checked-in benchmark during this audit. If a partial stage
accepts a definition without proving the domain invariant, state that limitation;
do not report an executable unsafe acceptance or invent a transition evaluator.

Recommend the next independent implementation from the findings. In particular,
consider finite guard-snapshot conformance using production pure judgments before
any public evaluator/runtime API; snapshots would be synthetic immutable test
inputs, never authenticated observations or a process execution. A new test-only
harness must still statically check full expressions, validate bound values/types,
use exact source ASTs and preserve errors/short-circuit behavior. Freeze that scope
separately before code. Any new wait-result/fault/continuation contract requires
its own ADR and explicit versioning rather than an implicit grammar extension.

Update parent status/progress and keep every scenario Partial and draft #46 open.
Review the audit separately for scope/evidence and run relevant package oracles;
retain required final-head CI and comment review. Continue the selected dependency
under delegation; no per-item owner approval or phase exit is implied.
