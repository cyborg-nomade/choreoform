<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0015 continuation: bounded Rust frontend evidence

**Date:** 2026-10-04<br>
**Status:** Implementation proposal awaiting Project Owner review<br>
**Specification baseline:** `e65d82d15aa2a5f68d8d0ec60edd6a56d126976a` (merged PR #19)<br>
**Frozen plan:** [0015-frontend-plan.md](0015-frontend-plan.md), committed before
implementation in `515ba25`<br>
**Implementation and limits:** [frontend README](../../crates/authoring-frontend/README.md)

This slice implements syntax, identity consistency and deterministic **candidate**
lowering. It does not rerank A/B, replace the frozen ADR-0014 comparison, admit
executable processes or complete the authoring item.

## Reproducible observations

`cargo test -p choreoform-authoring-frontend --locked` runs 21 integration tests
with table-driven/subcase coverage. The new public strict canonical decoder has
a regression test in the existing core. Workspace tests, compile-fail doctests,
formatting and warnings-as-errors lint remain regression gates.

Build the native adapter, then run
`.tools/ir-check/bin/python tools/check_authoring_frontend.py` with the existing
hash-locked dependencies: four independent test groups, no new Python packages.

| Observation | Evidence | Boundary |
| --- | --- | --- |
| Accepted syntax | 106 positive fragments agree with the Python EBNF oracle; 12 lexical/structural negatives refuse | Not grammar ambiguity proof; external forms recognition-only |
| Hashes and IR shape | Terminal and synthetic review packages match Python source/JCS/framed hashes, IR JSON schema and independently recomputed revisions | Shape/hash success is not semantic admission |
| Identity consistency | Missing/extra/stale/retired IDs, wrong kinds, shadowing, unused symbols and slot mismatch refuse | No reconciliation editor or general import/export |
| Package cycles | Format/reparse/lower, repeated formatting, Unicode/alias rename and reordered ledger preserve IR, IDs and annotations | Formatter drops comments; not arbitrary IR-to-source |
| Semantic artifact edits | Work instructions change IR revision; alias/format edits do not | Not runtime evidence |
| Expressions/values | 25 operation examples; exact signed-64 decimals, negative-zero/overflow refusal, nested marker checks, literal/constructor distinction | No full expression/type checker |
| Binding/control | Human/compute, invoke/repeat, decision/wait, split/join, fanout, ancestor transfer, qualified invalidation and local port ownership | Synthetic, not full RP-01/03/08 |
| Dependency/refusal | All-branch reads, conservative reads, policy/actor closure; policy/control cycles and wrong variants refuse | External payloads explicitly unsupported |
| Transport/bounds | Duplicate/numeric/UTF-8/depth/token/byte refusals, source-byte mutations without panic, UTF-16 JCS key ordering | Regressions, not security/scale certification |

Both examples have explicit companions. The review fixture uses permissive
synthetic human-decision predicates for inspection, not production authorization.
Native lint and release Wasm library compilation/lint are included in CI. No new
browser binding or browser execution result is claimed. Existing portability,
IR/contract, authoring-specification and technical-text checks remain unchanged.

## Deliberate partial boundaries

1. Every nonempty external-resource inventory refuses lowering. Supported
   provider/clock/calendar/named-type readers and dependency contracts must precede
   enabling those constructs; hashing opaque bytes is insufficient.
2. Full expression/type/policy/protection/authority validation remains separate
   executable-contract/validator work. Candidates may still be inadmissible.
   Confusable-name diagnostics and richer related spans remain frontend work.
3. Original-source formatting is not a general identity-preserving IR exporter or
   transactional create/import/rename/move/delete/copy/merge tool. Original source
   is retained; formatting removes comments.
4. The bounded byte/token/depth/work budgets are not measured scalability.

## Roadmap and review boundary

Review recommendations:

1. Accept this as bounded candidate-lowering infrastructure, not admission or
   authoring completion? Recommend yes: it makes identity/mapping behavior
   inspectable while preserving the explicit semantic-validation boundary.
2. Accept the prototype budgets and comment-dropping formatter for tests only?
   Recommend yes, without making compatibility/editor promises. Keep richer
   diagnostics and general identity-preserving export/editing explicitly open.
3. Prioritize supported contract readers and semantic admission in the next
   authorized slice, coordinated with the existing validator/IR gates? Recommend
   yes before scaling to full benchmarks; never enable opaque external payloads
   by hash alone. General export and representative-user evaluation still follow
   within the same unchecked authoring item.

The near-English item stays unchecked. Next work within that item requires
supported resource/semantic admission, general identity-preserving export/editing,
complete RP-01/03/08 source/IR benchmarks with their explicit contract gaps resolved,
and matched A/B and identity-editing representative-user study materials/results.
Propose and authorize the next slice after this review, coordinated with the open
executable-IR/validator gates; do not label this frontend complete.

G1–G4, forty Partial scenarios, accessibility evidence and Phase 1 exit are unchanged.
No visual notation or participant recruitment is included. Rust remains accepted;
ADR-0015 decisions stand without a duplicate implementation ADR. Pause for owner
review before merge or another implementation slice.
