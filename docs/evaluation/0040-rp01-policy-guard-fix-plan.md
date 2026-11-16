<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-01 payment policy revision guard: frozen precise correction

**Date:** 2026-11-06<br>
**Baseline:** `80037c65c4f681d39199f120153fb11ec60f49cb` plus the uncommitted
test-only snapshot harness following frozen plan `2826cce`<br>
**Authority:** ADR-0018; original current-policy approval and P7 commit requirements

The first native run of source-derived snapshots reaches `commit_wrong_policy`:
the production pure evaluator returns Boolean true for the actual payment commit
predicate, while the required result is false. The snapshot has a valid claim,
matching claim identity/revision and otherwise valid manager/finance evidence,
but the manager evidence names a different nonempty policy revision. Exact nominal
value checking accepts its shape; it cannot establish that cross-record relation.
Local evidence: `/private/tmp/rp01-guards-first.log`. This is a finite predicate
result, not an admitted or executed unsafe payment or a reachability proof.

`may_pay` checks both approval policy revisions earlier; `payment_commits` only
rechecks approval claim identity/revision and current root claim identity/revision.
P7 requires protective conditions at the current conflicting commit. Comparing
opaque business revision strings does not itself compare the separately explicit
policy revision, and source typing supplies no implication between them. Strengthen
the fixture predicate explicitly rather than changing the negative expectation.

In this dedicated guard-conformance branch, add exactly three equality clauses:
manager and finance evidence policy revision each equal the payable input claim's
policy revision; payable input claim policy revision equals the current root claim
policy revision. Preserve all existing clauses. Add symmetric finance and changed
current-policy negatives to the finite catalogue; retain valid v1/v2 positives.
This is a precise benchmark predicate correction, not a proof of all conflicting
paths, authenticated policy truth or irreversible safety.

Preserve historical baseline fixture behavior by making the stronger construction
an explicit fixture-builder option, default off for existing profile probes and
unpatched historical metadata examples; the current package generator selects it
explicitly. Parameterize the precise-link comparison fixture to use that same
strong guard when comparing only invalidation declarations. No source grammar,
core/IR contract, IDs, provider/type/clock bytes, bindings, control or other data
and policy fields may change. Candidate semantic revision and all report/package
digests must change because the guard meaning changes. Repin the snapshot header
only; never change expected values to hide the failure.

Before accepting, compare old/new bodies allowing only the three added predicate
clauses, confirm all original subexpressions remain, and rederive all ten native
stages. Retain all forty Partial statuses and every outstanding host/phase gate.
Finish harness boundaries/oracle, required native/Wasm/Python/CI and separate
completed-diff review for the combined scoped conformance/predicate deliverable;
inspect comments and merge only the passing expected head with explicit sign-off.
