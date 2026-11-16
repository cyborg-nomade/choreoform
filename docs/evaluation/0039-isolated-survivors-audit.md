<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Surviving called-scope lifetime: pre-change critique

**Date:** 2026-11-06<br>
**Baseline:** `b95b2be93c14626adfc248fdf3d6a30235ea8343`<br>
**Plan:** [frozen scope](0039-isolated-survivors-plan.md), `445b23e`

`concurrency::terminal` currently requires a terminal closure for every covered
invoke/repeat and every parallel child. This is an honest conservative subset,
not evidence that reconcile closures are invalid. It blocks even an isolated
wait-only follow-up, with no declared storage, data reads/writes or effect dispatch.
Summary already indexes every declared node, including unvisited work. Avoid a
blanket exemption based on a read-only provider, true predicate or policy name.

Actual native source witnesses built from `check_closed_provenance.package`
before any checker change produced:

| Witness | Lower | Strict invalidation | Concurrency |
| --- | --- | --- | --- |
| Invoke with isolated timer-free wait, then finish | pass | pass | `scope-lifetime-unproved /body/nodes/run` |
| Surviving writer to completed worker's local | pass | `invalidation-context-unproved /body/nodes/review` | same prerequisite refusal |
| Wait reads mutable ancestor Seed | pass | pass | `scope-lifetime-unproved /body/nodes/run` |
| Unvisited initialized storage in follow-up | pass | pass | same lifetime refusal |
| Follow-up enters finish while wait declaration is unreachable | pass | pass | same lifetime refusal |

All lifetime refusals locate original source bytes 2948..3103. Writer context
refusal locates bytes 2746..2901. Results are retained locally in
`/private/tmp/isolated-survivors-baseline.json`. These are synthetic static native
checks, not executions or actual host receipts. Full RP-01 likewise refuses at
`run_checks`, bytes 40983..41175, after its four strict preceding stages pass.

P6 explicitly separates ordinary closure from full terminal disposal, creates a
direct-child surviving owner before ordinary disposal, and requires fresh
authority, exact accepted atomic handoff, one accountable occurrence owner and
all-obligation settlement before terminal closure. It does not permit arbitrary
ordinary work to restart. ADR-0008 likewise keeps late observations and outstanding
follow-up visible while the instance remains nonterminal. P7 still blocks planning
for unproved irreversible races. A declaration report cannot discharge any of
these live premises, even if a wait predicate is literally true.

Recommendation: prove only declared isolation for synchronous continuation when
the exact owned follow-up has no data/storage/nested work, contains only data-free
timer-free waits and finishes, has a reachable wait-to-finish path and terminal
own closure. Inspect unvisited declarations and scope-management dependencies too.
Record each creator/body/closure/follow-up/wait set plus explicit host premises.
Retain all parallel lifetime/conflict rules. Reject the other witnesses; do not
claim safety/progress of general reconciliation, transitive actual settlement or
the full benchmark. Use a shared indexed proof cache and existing work/output
bounds, with no later-stage recursion. No syntax/core/IR change is required to
prove this narrower subset under the existing conditional P6 contract.
