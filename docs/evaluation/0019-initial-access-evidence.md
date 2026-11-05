<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Initialization access dependencies: follow-up evidence

**Date:** 2026-11-05<br>
**Status:** Accepted implementation under owner delegation; effective upon merge<br>
**Baseline:** `ecc29c965191504b18bd98c75d6d9f64f44afe3d` (PR #24)<br>
**Plan:** [frozen follow-up plan](0019-initial-access-plan.md), `6c59705`<br>
**Authority:** [ADR-0018](../decisions/0018-phase-review-delegation.md), implementing
the accepted ADR-0013/0017/0019 rules without changing their contracts

## Result

The ordinary availability stage now checks the data needed by access policies at
initialization, not just the initial value expression. A shared bounded helper
expands protection-policy, participant-requirement and capability/authority/effect
read dependencies, including recursive access-metadata dependencies. Data cycles
terminate through visited sets and refuse unavailable reads rather than recursing
or inventing an initialization order.

Input-cell policy reads must be available from the ancestor snapshot before fresh
inputs are seeded. Peer input values cannot authorize one another's creation.
Initializer value reads and all accessed source/destination metadata dependencies
must be available from entry inputs/ancestors. Local initializer ordering remains
unproved. Errors identify actual data declarations with
`initial-access-availability-unproved`.

Invocation and fanout item destination metadata now contributes to the caller's
required read set. Source lowering derives the same fixed point without treating
the fresh child input value itself as a caller read. Missing declarations refuse;
invisible fresh-child dependencies cannot be repaired by guessing scope access.
Affected source packages can acquire corrected dependency sets/revisions; this is
a prototype correctness fix, not a compatibility commitment or rewritten frozen
artifact. Existing checked-in example revisions remain stable.

The protection report drops only `initial-access-policy-availability`, now checked
for its modeled ordinary input/initializer paths. Actual principal/purpose/
capability facts, policy truth/current revisions, atomic host enforcement,
exceptional/observable channels, race/invalidation/reconciliation and complete
benchmark/validation/Phase 1 gates remain open. Terminal output-port metadata
availability is a separate remaining transfer-boundary check, not implied by
this initialization result.

## Evidence and review

Six Rust control regression groups cover unavailable initial policy reads,
available entry/ancestor cases, peer-input/local ordering, recursive metadata
cycles, participant requirements, caller read declarations and fanout item
metadata. A source regression removes the old explicit audit read, adds a child
input policy that needs it, and proves lowering derives that caller dependency
without reading the child's fresh value; format/reparse preserves the candidate.
Each control refusal first passes payload checking.

The protection Python oracle adds a source package that remains syntactically
lowerable and payload typed while its initialization permission reads unavailable
data. Both control and protection stages refuse. Positive report cycles assert
the resolved initial-access residual is absent. This oracle now has four groups;
it checks artifact properties independently and invokes Rust for semantic results.

Local verification passed **125 Rust tests plus five compile-fail doctests**,
formatting, workspace Clippy, semantic/frontend Wasm lints, release frontend Wasm
build and the authoring/frontend/static/control/protection Python oracles.

Automated self-review under delegation on 2026-11-05 checked helper reuse/bounds,
snapshot timing, caller visibility, source derivation and the limited report claim.
It replaced silent filtering of malformed lowered input metadata with explicit
errors. Relevant source/fanout regressions and lint pass after that change. No
remaining actionable finding in this scope was identified; terminal output-port
metadata is recorded as the next separate transfer-boundary task. Required final-
head CI must pass before merge. No new external dependency,
contract snapshot, source grammar or host authorization interface is introduced.

## Reproduce

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy -p choreoform-semantic-check --lib --target wasm32-unknown-unknown --locked -- -D warnings
cargo clippy -p choreoform-authoring-frontend --lib --target wasm32-unknown-unknown --locked -- -D warnings
cargo build -p choreoform-authoring-frontend --lib --release --target wasm32-unknown-unknown --locked
cargo build -p choreoform-authoring-frontend --locked
.tools/ir-check/bin/python tools/check_protection_flow.py
```
