<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Lifecycle policy use-site validation: evidence

**Date:** 2026-11-05<br>
**Status:** Accepted under delegated review; final-head CI required before merge<br>
**Decision:** [ADR-0020](../decisions/0020-lifecycle-policy-links.md)<br>
**Baseline:** `be697012c15374ca05082a235d15cd16bd442e45` (PR #26)<br>
**Plan:** [frozen scope](0020-lifecycle-link-plan.md), `8135d26`

## Result

Fault handlers and reconcile follow-ups must be direct children of the actual
scope using the policy. Valid declaration context alone is insufficient. This
also checks uncalled child scopes, while preserving reusable empty-handler and
terminal policies. Effect reconciliation requires a declared wait site; an
indexed unique-site check refuses missing owners and leaves ambiguous inference
explicitly unproved. Ordinary IR wait-policy reuse without that non-node use
continues to pass.

These checks do not create or authenticate a live owner, accept an ownership
transfer, correlate a receipt or prove atomic settlement. Existing ordinary
coverage and residual report obligations stay unchanged. No wire/grammar/version,
frozen snapshot, old fixture, public mutable report or dependency changes.

## Evidence

Five new Rust groups cover inherited empty/terminal policies; positive and
negative fault/closure use contexts with actual declaration paths; ordinary wait
reuse; and positive, missing and ambiguous service reconciliation sites. Positive
ordinary control reports do not invent availability contexts for uncalled work.

The [three-group Python oracle](../../tools/check_lifecycle_links.py) checks:

- The service package's schema/JCS/revision, exactly one declared wait site,
  static/control/protection reports and format/reparse stability. Its wait site
  remains ordinary-uncovered, which does not imply runtime reconciliation proof.
- A syntactically valid source variant whose reconciliation policy still names
  its old outcome owner, but that node now computes a text value instead of
  waiting. It lowers as an unvalidated candidate; all explicit check stages
  refuse `reconciliation-owner-missing` without echoing the literal payload.
- Documentation links and SPDX notices.

Artifact checks are independent; semantic results come from the Rust stage. The
oracle is not an independent runtime or proof implementation. CI runs it after
the existing protection oracle, and previous checks remain required gates.

## Reproduce and review

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy -p choreoform-semantic-check --lib --target wasm32-unknown-unknown --locked -- -D warnings
cargo clippy -p choreoform-authoring-frontend --lib --target wasm32-unknown-unknown --locked -- -D warnings
cargo build -p choreoform-authoring-frontend --lib --release --target wasm32-unknown-unknown --locked
cargo build -p choreoform-authoring-frontend --locked
.tools/ir-check/bin/python tools/check_lifecycle_links.py
```

Local verification passed 133 Rust tests and five compile-fail doctests, format,
native lint, semantic/frontend Wasm lint and frontend release Wasm compilation.
Authoring evidence/frontend, static contracts, control, protection and lifecycle
Python oracles passed. Required CI covers the remaining portability checks.

Separate automated self-review checked actual versus declaration scope, all
declared scopes, ambiguous versus missing sites, valid policy reuse, paths and
ownership limits. The index stores counts rather than node-ID vectors; it scans
nodes once and charges each visit against the bounded budget. Counts cannot
overflow within the artifact and cumulative work limits. No independent review
or runtime observation is claimed. Record review and final-head required CI
before merge under ADR-0018 delegation, then continue
the [Phase 1 checkpoint](phase-1-progress.md). No item-level owner pause or phase
transition follows from this item.
