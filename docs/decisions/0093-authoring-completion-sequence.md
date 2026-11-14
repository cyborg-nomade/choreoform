<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0093: Complete reversible managed authoring before broader work syntax

**Status:** Accepted direction under Project Owner delegation; effective on integration<br>
**Date:** 2026-11-14<br>
**Decider:** Codex under ADR-0018; no personal owner or independent review

Signed [scope93](../evaluation/0093-authoring-completion-critique-plan.md)c30eb10
follows verified PR #107 integration4042081c3b8ea990ad1d4e93357ad368889cdaf8,
CI43581960139 atdd809f4568bd0642a5ec5376da9360a303339d66, separate automated
COMMENT review5903518032 and fresh full-body/thread/head audit. The measured
[critique](../evaluation/0093-authoring-completion-critique.md) was signed as
c364bd168503023ef0c83e4a5429789b5fd50221 before any new support-tool implementation.

Select a separately published **reversible authoring-package and managed-change
contract** as the next bounded deliverable. Then implement native managed operations
and a small paired text/visual prototype in dedicated scopes. Preserve one semantic
meaning, explicit authoring-only structure, stable identities, non-semantic layout,
versioned consistency checks and protection/authority boundaries. Untrusted
editorial metadata must not hide or replace the actual semantic model. A report or
digest grants no live host permission. Exact language/API/schema/identity/permission
changes require their own publication before later production code.

Actual native observations motivate the sequence. RP-01/03 are54172/291309 source
bytes with53726/242843-byte companions; RP-03 still has59 scopes and198 steps.
Remaining exact-expression repetition upper bounds128/7156 are not savings and
cannot supply missing navigation/managed editing/visual parity. Complete archived
source18 and current source19 RP-01 produce byte-identical IR but different native
formatted sources: source-only test structure is deliberately erased from execution
IR. Original-package formatting alone cannot supply arbitrary reverse export.
An actual invalid text Instruction is refused privately at a685-byte policy span;
full domain-oriented diagnostics still require work. These are implementation
observations and design reasoning, not user-study results.

Full RP-08 design/coverage proceeds alongside the authoring foundation, with a
complete lowerable package and all required cross-form evidence explicitly later
within Phase1. Refine diagnostics and collect actual representative-user evidence
after adequate independent preparation. Preserve complete validator/operational
semantics/conformance/negative-example and aggregate IR/G1–G4 obligations.

Broader source-only service/work blueprints may improve reuse, but require a larger
typing/provenance/identity/capture/compensation contract and faithful visual support.
They are a separate later decision, not an implicit extension of pure tests. More
English sugar or bigger quotas cannot substitute for reversible state, managed IDs,
three full representative processes, visuals or actual users. The Phase1 prototype
is not the Phase3 production studio; runtime simulation/collaboration/deployment
remain later deliveries.

This direction changes no compiler/SDK, syntax, runtime/host behavior, accepted
contract/quota, benchmark/corpus/report/guard/receipt bytes or phase admission.
All965 benchmark files retain inventory
a25c6de850fdaa51436023fffe011c8f96695e9d8660191e7598cd4f4bf2e9f7.
Independent user/host/browser/review evidence is never generated or inferred from
compilation. Required final-head checks/CI, separate completed-diff automated review,
full comment audit and expected-head signed merge precede integration. All40
scenarios remain Partial, parent46 draft, all full Phase1 criteria open; no Phase2.

Four read-only oracle groups and complete local688 Rust+6 compile-fail/native/
Wasm/portability/all88 Python commands pass. Final-head CI, separate completed-diff
automated review, full comment audit and signed merge remain pending.
