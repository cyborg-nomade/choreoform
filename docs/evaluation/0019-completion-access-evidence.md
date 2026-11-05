<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Completion access metadata: follow-up evidence

**Date:** 2026-11-05<br>
**Status:** Accepted implementation under owner delegation; effective upon merge<br>
**Baseline:** `89d648266702d13746ffd4a7b9c04c5998994b59` (PR #25)<br>
**Frozen plan:** [completion dependency plan](0019-completion-access-plan.md), `798f899`

## Result and limits

Finish nodes now require the policy-data dependencies of their scope output
metadata. Source lowering derives the same fixed point, including access policies
on data read by those predicates. This is metadata dependency declaration, not an
invented read of unrelated/fresh values or an actual access grant. Existing port
value/transfer checks still apply separately. Static checking covers every finish,
including uncalled child templates; ordinary availability remains limited to
reported visited contexts.

The change exposes reads needed by output publication and future conflict/access
enforcement. It does not establish current predicate truth, authentication, atomic
publication, noninterference of observable/exceptional channels, stale-evidence
handling, race/invalidation safety or host support. Phase 1 remains incomplete.

## Evidence and review

Three Rust regression groups cover missing output metadata declarations in an
uncalled child with lexical ancestor dependencies, transitive access-policy data
closure, and the absence of spurious value reads. The last group also validates
700 terminal declarations sharing one 4,000-purpose immutable output envelope,
with all unvisited declarations explicitly reported as uncovered.

The protection Python oracle checks the derived `review_done` metadata read set
in its process-data-policy variant and preserves it through format/reparse. That
variant passes payload checking and still correctly refuses its unavailable
initialization access dependency; it is not presented as an executable process.
All existing positive example package revisions stay stable.

The separate automated review pass identified repeated output-envelope validation
as a new multiplicative cost. Data protection reference sets are now validated
once per immutable definition check, cached privately and copied under a lookup
work charge. Raw metadata validation charges visits and purpose members even with
empty actor/capability sets. Provider metadata still undergoes its original raw
validation in the proper definition context. The wide shared-envelope regression
passes under the bounded budget; existing wide transfer-comparison refusal remains.

No public mutable authority, new dependency, wire/grammar/semantic version, frozen
snapshot or old fixture changes. Required final-head native/Wasm/oracle CI and
recorded delegated review remain merge prerequisites.

Local verification passed **128 Rust tests plus five compile-fail doctests**,
formatting, workspace Clippy, semantic/frontend Wasm lints, release frontend Wasm
build and the authoring/frontend/static/control/protection Python oracles. The
separate automated self-review on 2026-11-05 checked all scope outputs, transitive
dependencies, caller visibility, immutable cache ownership, work charges and report
claims. The repeated-validation cost was fixed before acceptance; no remaining
actionable finding in this scope was identified. Acceptance is under ADR-0018
delegation, not personal owner or independent human review of this PR.

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

Continue with lifecycle/race/invalidation and supported-clock work after merge,
under [ADR-0018](../decisions/0018-phase-review-delegation.md), without an item-level
approval pause. The [phase checkpoint](phase-1-progress.md) retains all aggregate
validator, benchmark, editing, visual and representative-user obligations.
