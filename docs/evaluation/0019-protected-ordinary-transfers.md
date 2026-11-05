<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Ordinary protection constraints: implementation evidence

**Date:** 2026-11-05<br>
**Status:** Accepted under owner delegation; effective upon PR merge<br>
**Decision:** [ADR-0019](../decisions/0019-protected-ordinary-transfers.md)<br>
**Baseline:** `384e3add214bbf1be7aecbf520a9c027d563898a` (PR #23)<br>
**Frozen plan:** [0019-protection-plan.md](0019-protection-plan.md), `f5c4db8`

## Behavior

`check-protection` builds on static/control checking. A protected guard can select
a literal write to a less-protected cell even when that write has no direct read.
The new stage carries protection origins through ordinary paths and rejects that
transfer. It keeps origins after merges and through child completion, repeat and
parallel continuations, with no declassification or guessed predicate implication.
It also checks initializer and child-input transfers and provider-result origins
without requiring a published result cell.

The immutable report enumerates required site/cell/operation access judgments,
revision, transfer count and inherited control coverage/input assumptions. Empty
participant-and-capability access contexts refuse. Actual access authorization,
policy truth/current snapshot, initial access-policy availability, exceptional/
observable channels, race/invalidation and trusted atomic host enforcement remain
explicitly unproved. There is no execution/dispatch API or authority flag.

## Evidence

Nine new Rust integration groups extend existing fixture helpers without adding
duplicate implementations. They cover protected branch literals, merge retention
and constant guards; initializers and child inputs; nested/parallel completion;
explicit access obligations and empty access contexts; uncovered declarations;
iterative/budgeted work; and null-publication provider-result protection. Negative
cases pass `check-control` first, isolating the additional constraint. A separate
wide-envelope subcase confirms static-stage metadata comparison work is bounded.
The local run passed **118 Rust tests plus five compile-fail doctests**, formatting,
workspace Clippy, semantic/frontend Wasm lints and the release frontend Wasm build.
All prior IR/contract/authoring/frontend/static/control oracles and the new three-
group protection oracle passed. Required final-head CI remains the merge gate.

The [Python oracle](../../tools/check_protection_flow.py) has three groups: five
original-package/source variants with schema/JCS/revision/report and format/reparse
checks; source examples where control success is followed by protection refusal;
and documentation/license checks. The rooted branch variant has explicit input
and purpose symbols. With matching labels it passes; narrowing the input's purpose
set to `Private` while the literal output permits `Illustration` refuses. These
are synthetic examples, not a user study or complete benchmark.

The oracle independently checks artifacts and invokes Rust for semantic results;
it is not an independent proof implementation. No frozen contract, old fixture,
source grammar, semantic version or third-party dependency changes. Existing
native/Wasm and prior Python checks remain required CI gates.

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

## Review and next work

Automated self-review under ADR-0018 on 2026-11-05 inspected transfer boundaries,
origin retention, actual-vs-symbolic access claims, resource sharing, work limits
and diagnostics. It found and fixed two issues before acceptance: destination
access-predicate dependencies at non-write transfer boundaries, and undercounted
wide-envelope comparison work. Both have regression cases. Diagnostics resolve
to actual declaration paths. No remaining actionable finding was identified.

Acceptance is under the owner's explicit delegation, not a claim of personal
owner or independent human review. Required final-head CI must pass before merge.
This item does not pause for individual owner approval or start Phase 2. Continue
the [Phase 1 checkpoint](phase-1-progress.md); every aggregate technical/empirical
exit criterion stays open until its evidence exists.
