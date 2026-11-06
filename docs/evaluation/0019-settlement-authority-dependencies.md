<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Settlement receiver authority dependency correction

**Date:** 2026-11-06<br>
**Plan:** [frozen settlement plan](0019-settlement-access-plan.md), `4220f3a`<br>
**Contracts:** ADR-0013 P6; ADR-0019 static dependency mechanism; ADR-0018 delegation.

## Finding and correction

The remaining-validator audit found that transfer settlement checked an ancestor
owner but omitted the receiver's management actor requirement from policy read
closure. A pre-change regression with distinct child/receiver authorities and
omitted transfer reads passed static checking (local `/private/tmp/settlement-before.log`).
That is evidence of the metadata gap, not a successful ownership transfer.

Static checking now resolves the receiver's actual cancellation policy and actor
requirement in the receiver context and adds that requirement edge. Source
lowering derives the same edge. Existing policy closure propagates its reads into
child cancellation/membership/join users and nodes. It deliberately does not add
the receiver cancellation policy's own children-settlement reads. Cancel/retain
keep null owner and acquire no receiver dependency. No contract/schema/version
change or new ADR decision is needed to enforce the existing P6 requirement.

Tests exposed an existing empty path on policy dependency mismatch. The closure
now reports `/body/policies/<id>/body/reads` for that fixed code, without exposing
data values. Initial policy-body kind/visibility errors retain declaration paths.

## Evidence

Five direct-IR groups cover omitted/exact/transitive/node reads, inherited
receiver cancellation and exclusion of unrelated children-settlement dependencies,
cancel/retain, receiver kind/visibility and 500 repeated dependency edges. One Rust source group covers
both profiles, receiver reads through child cancellation and finish declarations,
ordinary protection coverage and exact format/reparse identity.

Three independent Python groups cover both grammar/schema/JCS/revision/report
cycles with an ordinarily invoked child; unchanged cancel/retain behavior; and
documentation/licenses. Synthetic labels in the source test are compatible;
the prior child-input fixture's deliberately different label is not silently
treated as a permitted transfer. No observation or current authority is supplied.

Passed 201 Rust tests plus five compile-fail doctests, format/native lint,
semantic/frontend Wasm lint and release frontend Wasm build. Relevant artifact/
source/stage Python oracles passed, including the three new groups; final-head CI
and expected-head merge remain prerequisites under ADR-0018. Local logs are
temporary (`/private/tmp/settlement-rust.log`, `/private/tmp/settlement-all-oracles.log`).

Separate completed-diff automated self-review checked actual receiver context,
exact authority-only edges, mode/version compatibility, transitive declared/node
reads, useful diagnostic paths and static/source parity. Review also added
explicit edge/member work charges for policy closure in both stages; the new
dependency cannot bypass those limits. The 500-edge regression remains within
budget; no oversized transport is presented as a valid budget test. No actionable
finding remains. This is delegated implementing-agent review, not independent
approval, actual receiver acceptance or runtime ownership evidence.

## Remaining obligations

Current receiver authority, non-cancelled/live receiver occurrence, exact
obligation/source/target revisions, authenticated acceptance and atomic accountable
ownership remain mandatory runtime judgments. The partial reports establish none
of them. Exceptional/fault routing/lifecycle, full validator, benchmarks,
text/visual/protected accessible editing and actual user evidence stay open.
