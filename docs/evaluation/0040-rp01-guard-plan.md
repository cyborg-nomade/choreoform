<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Finite RP-01 guard snapshots: frozen implementation plan

**Date:** 2026-11-06<br>
**Baseline:** `80037c65c4f681d39199f120153fb11ec60f49cb`, draft #46 scenario audit<br>
**Authority:** ADR-0018; [actual scenario gaps](0040-rp01-scenario-audit.md)

Use a dedicated stacked dependency PR and ADR-0040 to preserve the evidence
boundary. Add a closed, revision-bound, test-only finite snapshot catalogue and
Rust harness for actual candidate expression/policy ASTs. No public evaluator,
source/core/IR feature, accepted observation, process transition, permission or
runtime API is introduced. Keep source/IR/resources and all scenario statuses
unchanged. Ordinary authors are not asked to manage this fixture format.

Select original expressions or exact actor/work/race predicates by existing IDs;
derive lexical cell types and supported policy-fact types from the real definition
and accepted contexts. Work result type must agree with its actual activity site.
Check the complete original AST first. Validate every supplied bound value with
the production exact type/nominal judgment. Rebind read/fact leaves to explicitly
named immutable parameters only inside the test harness, preserving literal data,
types/fields and every other operation. Check the full rebound AST/type/read set,
then use the existing production pure evaluator and validate the returned value.
Fail missing/unknown bindings and malformed catalogue/selector/revision/resources;
do not infer values or use unchecked host data. Charge traversal/copying under
existing resource work/depth bounds and keep diagnostics sanitized.

Include finite cases for valid manager/finance results, principal/claim/policy
revision separation, original versus corrected claim/approval snapshots, absent
approval short-circuit behavior, payment amount/currency/claim mismatches,
correction same-identity/new-revision requirements and current role/relationship
predicates. Assert expected Boolean or explicit binding/evaluation faults. Tie all
cases honestly to A–D predicate fragments; C unknown-effect/retry classification,
D ordering/cutoff/history and all process/host obligations remain uncovered.
Do not count snapshot vectors as completed scenarios or observations.

Test harness boundaries independently: full unselected-branch static rejection,
literal objects containing op-like data are not rewritten, missing/ill-typed/
invalid nominal values, wrong selectors/revisions and work bounds, and exact
short-circuit/evaluation faults. Add an independent Python oracle for closed
transport/schema/identity/reference/expected-outcome accountability and licensing,
without claiming independent semantic evaluation. Integrate required CI and
document commands, counts, scope and remaining gates.

If finite cases expose a genuine benchmark predicate defect, record it before
changing source and freeze a separate precise fix; never change expected values
to hide it. Run relevant native/Wasm/Python checks, separate completed-diff
automated review, audit all comments/CodeRabbit summaries and merge the passing
exact head with explicit sign-off. Then return to the parent and concrete fault-
source/late-observation/continuation/validator work, keeping Phase 1 gates open.
