<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Finite RP-01 guard snapshot evidence

**Date:** 2026-11-06<br>
**Plan:** [frozen harness scope](0040-rp01-guard-plan.md), `2826cce`<br>
**Correction:** [reproduced failure and precise plan](0040-rp01-policy-guard-fix-plan.md), `d0c13bf`<br>
**Decision:** [ADR-0040](../decisions/0040-finite-guard-snapshot-conformance.md)

The test-only harness selects actual candidate expression or actor/work/race
predicate ASTs, derives lexical cell/accepted fact types and binds work result type
to the actual activity site. It statically checks the full original AST, validates
explicit bound values with production exact/nominal judgments, and rebinds read/
fact leaves to distinct immutable parameters. It preserves literal data/type
descriptors and all other operations, checks the complete rebound type/read set,
uses the existing production pure evaluator and validates the returned value.
Missing/unknown/unused/ill-typed bindings refuse; no values are guessed. Existing
cumulative work/depth limits charge traversal, AST and bound-value copying.
There is no public evaluator, IO, observation, transition or permission API.

The [closed catalogue](../../crates/semantic-check/tests/conformance/rp01-guard-snapshots.json)
has 44 vectors and 46 explicitly synthetic named values, pinned to the canonical
candidate revision/digest. There are 41 Boolean results (12 true, 29 false), one
selected `evaluation-tag` absence fault and two binding refusals (`snapshot-binding`
and `invalid-nominal-value`). The 12 A / 16 B / 6 C / 10 D mappings concern only
predicate fragments. They are not completed scenarios or accepted observations.

Cases contrast manager attribution/self-approval/claim/policy revisions, finance
participant separation and missing prior evidence, old versus corrected approvals,
absent-evidence short circuit, correction identity/new revision/principal, payable
amount/currency/claim mismatches and current commit evidence, plus role/relationship
predicates. A true completion predicate for a legitimate rejection/business problem
does not mean approval or payment. A true actor predicate does not authenticate
roles or supply mandatory acceptance/correlation checks.

The first source-derived run failed `commit_wrong_policy`: the actual predicate
returned true while expected false. The required result was retained. After
recording the finding and freezing the correction, the current fixture adds three
policy-revision comparisons (both approvals against payable input claim, that claim
against current root claim). All six prior clauses remain in order. Symmetric
wrong-finance/current-policy cases also return false; valid v1/v2 cases stay true.
The source builder's stronger guard is an explicit current-package option; default
historical source/profile/metadata fixtures retain the old guard. The precise-link
comparison uses a strong-guard baseline to isolate the same 72 invalidation pairs.
Native comparison proves no other semantic body field, ID, binding or resource
changes. This is a predicate correction, not a general reachability/race proof.

New source size is 56,843 bytes / 9,466 measured tokens. New semantic revision:
`sha256:d0bca38fb470cd0485b9a1cc22767182e6276669ca9ca131ac45d7af268269f3`.
Canonical byte digest:
`sha256:cab58022696be22dd9ec89c1357ca60c5b0781b1e0ceaad14f47ae3bfce2d693`.
All ten native partial stages still pass; actual report/package digests are
regenerated. No expected predicate value was changed to conceal the gap. Local
before/fixed logs: `/private/tmp/rp01-guards-first.log`,
`/private/tmp/rp01-guards-fixed.log`.

Six new Rust groups exercise the 44 vectors and harness boundaries: unselected
branch static rejection before binding, literal objects containing op-like data,
short-circuit versus selected absence faults, missing/unknown/unused/ill-typed
bindings and sanitized diagnostics, bad catalogue revision/shape/selectors and
missing/corrupt exact resources, and work/depth/copy bounds. Four Python groups
independently check closed transport/header hashes, exact counts/IDs/source
references/dependency bindings, required negative/positive/fault accountability,
synthetic input identity and licensing/test-only CI wiring. This oracle supplies
no independent predicate evaluation. A seventh package-oracle group proves that
only the three clauses were added and the old six remain.

Native local verification passed 297 Rust tests plus six compile-fail doctests,
format/workspace lint, all five library Wasm lints, release portability/frontend
Wasm builds and native portability oracle. All CI Python oracles and exact
regeneration/historical probes also passed after the new evidence document was
present. Logs:
`/private/tmp/rp01-guards-{workspace,clippy,wasm,native,oracles}.log`. Final-head CI
remains required before acceptance; the catalogue oracle is in required CI.

Separate completed-diff automated self-review checked full-original-first typing,
lexical/actual-site fact types, resource pins/nominal values, cell/fact namespace
separation, opaque literal/type preservation, exact binding sets, short-circuit/
fault behavior, copying budgets, cfg(test) isolation, catalogue accountability,
three-clause semantic body difference, preserved historical fixtures and honest
scenario counts. Review added explicit bound-value copy charges. No actionable
issue remains. This is delegated automated review, not independent/human/owner
review. Audit all PR comments and require the passing exact signed-off merge.

C still needs checkable authenticated no-effect fault-source mapping, exact
original obligation/request/key correlation, owned late-observation data and
explicit continuation/retry semantics. D still needs atomic effective ordering,
provider cutoff, stale/losing history and full conflicting-path evidence. Finite
snapshots prove neither reachable states nor process transitions, host truth,
current permission, actual lifetime/progress or irreversible safety. All forty
scenarios, full benchmark/validator/text/visual/protected editing/actual user and
Phase 1 gates remain open; draft #46 remains incomplete and unmerged.
