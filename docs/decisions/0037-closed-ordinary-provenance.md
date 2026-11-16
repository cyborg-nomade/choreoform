<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0037: Separate proved historical origins from live invalidation inputs

**Status:** Accepted under Project Owner delegation; effective on parent/main
integration<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

The [pre-change critique](../evaluation/0037-closed-provenance-audit.md) reproduces
private child origins escaping ordinary invoke/repeat and blocking invalidation.
The same old Cell representation holds both protection provenance and a live data
dependency. Erasing private origins would hide descendant writers or ancestor
revision dependencies. Treating every completed descendant as frozen would also
misread partial fork/join summaries or reuse one template's live occurrence.

## Decision

Under existing value-copy/fresh-occurrence and P6 ordinary-closure semantics,
prove a conservative historical-origin subset only for synchronous invoke or
repeat-body completion. A cell must belong to that exact completed body scope,
not an arbitrary descendant; no declared node in another scope may write it.
Index all declared writes, including unvisited descendant/handler/follow-up and
output-binding writes. Own ordinary work cannot restart in the closed occurrence;
future invocations/iterations use fresh storage. Retain live caller origins when
another occurrence of that same template may already influence the context.
Do not freeze split/fanout summaries. Keep every unproved or mutable origin live.

Historical origins retain original protection labels and exact conditional
boundary references. Mutable ancestor/input/access-policy/provider influences
remain separate; semantic exact read/write checking precedes the proof. This is
not an exported private namespace, guaranteed value, authorization cache or actual
completion/ownership receipt. A future host must enforce occurrence identity,
atomic ordinary closure and correct snapshots. Effects/subscriptions and surviving
lifetime still require their existing separate proofs and judgments.

Apply the refinement only while collecting ordinary invalidation provenance;
preserve protection-only results, source/IR contracts and semantic revisions.
Report typed historical evidence at consequential sites and scope-entry contexts
separately from live invalidation obligations,
under existing total work and output bounds. Later checks expose the same snapshot
enforcement requirement and retain their other residual gates.

## Alternatives and consequences

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Exact completed owner, no nonlocal writer, retained labels/ancestors | Proves useful ordinary copy/control cases without new semantics | Conservative refusal for many safe descendant writers; requires host closure/snapshot enforcement | Adopt |
| Ignore every private dependency | Small change | Loses mutable-survivor and input revision obligations | Reject |
| Freeze all descendants or infer safety from terminal policy names | Broad acceptance | Hypothetical/partial completions and surviving writes are not proved | Reject |
| Add a user-supplied immutable/safe flag | Easy authoring workaround | Boolean claim supplies no proof | Reject |
| Keep every case unproved indefinitely | No new proof burden | Prevents simple ordinary composition supported by existing contracts | Reject |

The inherited-context substitution can prove a previously unproved read-only
parallel context; it changes no concurrency/conflict/lifetime rule. Reports retain
the creator and destination scope for that use. An unvisited descendant writer
preserves the old live-context refusal, and an inherited live template origin is
not upgraded by a later synchronous call.

The existing contracts already forbid ordinary work restarting in a closed
occurrence and require declared writes/value copies. This expands a conservative
proved subset; it does not alter process meaning, source syntax or any pin. If
those premises change in a later contract, this proof must be reevaluated. Full
validator, exceptional protection, surviving lifetime, complete benchmark and
actual host/browser/user/phase evidence remain open.

Follow [frozen plan](../evaluation/0037-closed-provenance-plan.md), `a3fc112`, and
pre-change audit checkpoint `1996854`. Acceptance requires counterexample/report/
budget/native/Wasm checks, separate completed-diff automated review, comment audit
and passing final-head CI/expected-head merge into the still-unmerged parent.
No independent or individual owner review is claimed.

[Evidence](../evaluation/0037-closed-provenance.md) records 277 Rust tests plus
five doctests, native/Wasm/Python checks and separate completed-diff automated
review. Require final-head CI and complete comment audit before expected-head
merge; no phase exit or executable authority is granted.
