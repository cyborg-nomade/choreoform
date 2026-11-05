<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Ordinary control and initialization: evidence and review guide

**Date:** 2026-11-02<br>
**Status:** Accepted; owner approved PR #22 on 2026-11-05, effective upon merge<br>
**Decision:** [ADR-0017](../decisions/0017-control-availability-checks.md)<br>
**Baseline:** `99857fc658b98e1fcb9e82893764062a2d547833` (merged PR #21)<br>
**Frozen plan:** [0017-control-plan.md](0017-control-plan.md), committed in
`dc63a97` before implementation

## Result and boundary

The explicit `check-control` stage runs existing static checks, verifies ordinary
flow structure and proves conservative initialization along reachable ordinary
paths. Its immutable report binds the original revision, root input assumptions
and covered/uncovered node IDs. It grants no execution authority. The old `check`
stage still accepts payloads whose control or outputs this later stage refuses.

The new [branch source](../authoring/examples/control.choreo) and
[explicit identity companion](../authoring/examples/control.bindings.json) write
the output on both routes. Redirecting the second route straight to completion
still lowers and passes `check`, but `check-control` refuses
`output-availability-unproved`. This remains true with a literal-true first guard:
the proof does not prune branches using constant evaluation. The fixture is
synthetic, not a complete representative process or a usability result.

## Reproduced evidence

The local run passed **109 Rust tests plus five compile-fail doctests**, including
24 new control integration groups, workspace format/Clippy, all existing Wasm
library lints and release portability/frontend Wasm builds. All existing Python
oracles and the four new control groups passed. The native portability report
retains 89 case results, 15 JCS/hash comparisons, three unchanged fixtures, all
ten node variants and seven CLI checks. These results do not close residual gates.

| Area | Observations | Deliberate limit |
| --- | --- | --- |
| All declared ordinary flow | Missing/duplicate successor, reachable/unreachable cycles refuse; 1,200-edge DAG passes iteratively | No termination or progress guarantee |
| Fork/join obligations | Reciprocal pair, direct activation and nested population-token mutations | No join satisfiability/scheduling proof; thresholds can exceed population |
| Reads and outputs | Branch intersections, each finish, simultaneous pre-snapshot reads, sequential writes, root input assumptions | Initialization only, not stale-evidence or authorization proof |
| Fresh scopes and ports | Invocation locals/inputs/outputs, lexical ancestor writes, target uniqueness, exact writes | Exceptional/canceled invocations and reconciliation remain open |
| Repeat and parallel | Zero-iteration path; no sibling handoff or join credit; explicit fanout item seed | Valid programs needing parallel/order proofs may refuse |
| Initializers and coverage | Entry-input dependencies pass; guessed local initializer ordering refuses; unreachable declarations reported | Exceptional/unreachable declarations have no availability proof |
| Bounds | 1,000-node × 600-cell dense propagation refuses the cumulative work limit | No latency, throughput or compatibility claim |
| Source/package boundary | Five format/reparse/check cases across four source packages, including a root-input variant; static success/control refusal mutations | No general IR exporter, safe edit transaction or browser execution |

Rust regressions live in
[`control_check.rs`](../../crates/authoring-frontend/tests/control_check.rs).
Each negative control mutation first passes the unchanged static stage, isolating
the newly checked obligation. The four-group
[`check_control_availability.py`](../../tools/check_control_availability.py) oracle
independently checks schema/JCS/revision/package hashes, coverage partitions and
source-cycle stability. It invokes Rust for semantic results; it is not an
independent semantic prover. It also checks explicit resource refusal, diagnostics,
document links and licenses. CI runs it after the existing static oracle.

## PR #22 review follow-up (2026-11-04)

The [CodeRabbit review](https://github.com/cyborg-nomade/choreoform/pull/22#pullrequestreview-5739164221)
has three valid coverage findings, all addressed independently:

- **Nonempty CLI inputs:** add a temporary branch-package variant with a root
  input whose display name, port wire name and cell ID differ. Assert that
  `requiredInputs` contains the cell ID `branch_result`, and preserve the report
  through formatting/reparse. An always-empty serializer now fails this case.
- **Unreachable missing flow:** add an unconnected compute node with an ordinary
  outcome but no flow. It passes payload checks and refuses `flow-missing`, in
  addition to the existing reachable-node and duplicate-flow mutations.
- **Diagnostic value assertion:** embed a distinct integer marker in the actual
  fixture, verify its presence, then assert it is absent from the complete debug
  diagnostic. This replaces an assertion against an absent `SECRET` string; it
  covers that integer payload, not every possible value or a demonstrated leak.

The separate CodeRabbit **80% docstring coverage** advisory is declined. Its
complexity-filtered percentage is not an adopted repository requirement in
`CONTRIBUTING.md` or the workflow. The public proof boundary, assumptions and
budgets are already documented in the API/README/ADR. Blanket comments on private
helpers and tests would not close a demonstrated documentation or behavior gap.
No production semantics or admission boundary changes are needed for these findings.

## Reproduce

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy -p choreoform-semantic-check --lib --target wasm32-unknown-unknown --locked -- -D warnings
cargo clippy -p choreoform-authoring-frontend --lib --target wasm32-unknown-unknown --locked -- -D warnings
cargo build -p choreoform-authoring-frontend --lib --release --target wasm32-unknown-unknown --locked
cargo build -p choreoform-authoring-frontend --locked
.tools/ir-check/bin/python tools/check_control_availability.py
cargo run -p choreoform-authoring-frontend --locked -- check-control \
  docs/authoring/examples/control.choreo \
  docs/authoring/examples/control.bindings.json
```

Existing IR, contract, authoring, frontend, static, technical-text and portability
oracles remain gates; no old fixture, contract snapshot, wire/profile version or
third-party dependency changes. Native/Wasm builds demonstrate portability of the
library, not a running process host or new browser runtime evidence.

## Review questions and recommendations

1. **Keep a separately named conservative stage?** Recommend yes. It closes useful
   ordinary-flow/initialization checks while exposing root input assumptions and
   uncovered work. Passing it cannot authorize execution; the full-validator
   Roadmap item stays open. Folding it into `check` later is an API decision after
   the combined stage has an honest, complete contract.
2. **Accept conservative initializer/repeat/parallel rules?** Recommend yes for
   this slice. Intersections and snapshot reads have straightforward evidence.
   Guessing local initialization order, carrying a prior occurrence's locals or
   crediting any child's writes at a join could create false passes. Future
   precision can prove order/predicate/context facts explicitly; a current
   `unproved` refusal is not a new semantic ban.
3. **Next priority?** Recommend control-sensitive information flow and access
   obligations, followed by race/invalidation/reconciliation and exceptional-flow
   proofs, with a separately reviewed supported-clock contract/host proposal.
   Then establish complete benchmark/conformance evidence before asking to close
   the full validator or executable-IR gate. Each follow-up needs its own scope
   and review. Near-English studies, safe editing/export and visual parity retain
   their explicit Roadmap deliverables; none is silently deferred away.

## Stop boundary

The Project Owner approved PR #22 on 2026-11-05 and authorized continued work,
review and merge within the phase, with a report/approval boundary before the next
phase. The original item-level review pause is closed. Full structural/
semantic admission, rich stable source diagnostics, complete benchmark and forty
Partial corpus scenarios, G1–G4 and Phase 1 exit remain open. No process engine,
visual notation or participant recruitment starts here.
