<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Closed ordinary provenance critique, before checker changes

**Date:** 2026-11-06<br>
**Product baseline:** `3f67ab3bf54ed0e564f27dc8d6a0690b12b9f620` (draft #46)<br>
**Frozen plan:** [audit/proof plan](0037-closed-provenance-plan.md), `a3fc112`<br>
**Method:** same-agent code/contract review and actual native synthetic checks.
No participant, host execution, independent review or approval is implied.

## Finding and reproduced boundary

Protection-flow `Origin::Cell` currently serves two purposes: the original label
of an information/control influence, and a still-live invalidation dependency.
The ordinary flow carries child-private input/decision cells back to callers.
Invalidation then correctly refuses a cell outside the caller's ancestor context.
A child cannot repair this by listing a parent site: static invalidation visibility
also refuses that declaration. The model has no way to express a proved historical
influence while retaining its protection label and separate mutable ancestors.

Five minimal source-0.6 witnesses are reproducible with
`tools/check_closed_provenance.py`. All lower and pass protection. Each currently
refuses `invalidation-context-unproved at /body/nodes/review` at source bytes
`2746..2901`:

| Witness | Exact source digest (SHA-256 suffix) | Actual relevant declaration evidence |
| --- | --- | --- |
| Synchronous invoke, ordinary completed child | `558a0477c4cfe2a171d953d67947055292e7f21622d27ec376a26ef70696e69a` | Caller reads `seed`; no descendant writes `local` |
| Repeat-body ordinary completion | `025dc22ae213f1631b1f625aad47394df25c62687876a1a6903da8d7e7d610c9` | Caller reads `again` and `seed`; no descendant writes `local` |
| Reconcile child, read-only follow-up | `ac81416fe6c467b66535165972298c003a54b50db2e03b19d35bbf66687f6eeb` | Caller reads `seed`; declared surviving wait/finish writes no cell |
| Reconcile child, surviving descendant writer | `55c9ff67c9459c88df677aa77976744467d8cb8a29f28b2eb64158349f298812` | `change_local` in `recovery` writes `local`; ordinarily unvisited |
| Invoke with missing mutable-ancestor invalidation | `4f0c492527cfbb5b3c6ba1bc682a8eec7ad72b1d63b6bb07ad495b3fc70c2771` | Caller still reads `seed`; its parent review invalidation is omitted |

These are actual declaration/stage results, not executions. The last two must not
be turned into success by deleting private or transitive dependencies. The read-only
reconcile case can prove a data-origin subset while still failing separate lifetime
checks; snapshot evidence is not full disposal or obligation settlement.

## Contract reasoning and recommended scope

Accepted [ADR-0012](../decisions/0012-binding-type-foundations.md) specifies value
arguments, atomic output publication without aliasing and occurrence boundaries.
The frozen core's E1 rules read an immutable revision snapshot; P6 closes ordinary
work and permits surviving work only in the declared separate ownership boundary.
P7 retains current input dependencies and renews work with fresh occurrences.
All actual cell writes in the closed node profile must match validated write
inventories; input/initializer writes create fresh occurrence-local storage.
This permits a conservative additional proof subset, without changing core pins:

1. A synchronous invoke/iteration has returned its ordinary result. Its own scope
   cannot start ordinary work again in that occurrence. Do not infer this for
   hypothetical split/fanout summaries or arbitrary descendant origins.
2. For a cell owned by **that exact completed scope**, inspect every declared
   descendant node write, including unvisited handlers/follow-ups and explicit
   output-binding writes. If any can target it, retain the live/unproved origin.
   Do not assume a fully-terminal-looking policy alone erases descendant writers.
3. Otherwise the private cell's old value/control influence can be historical.
   Keep its original label for all protection transfers. Keep mutable ancestor
   origins, input/access-policy dependencies, provider labels and host obligations.
4. Record the conditional invoke/repeat boundary and source cell/scope as static
   evidence. Do not record invented actual occurrence IDs, completion receipts,
   values or current permission. A later host must enforce these core premises.

The generated caller `reads` sets already include source argument/creation reads
(`seed` in the witnesses), and semantic checking verifies them before this stage.
Thus historical private inputs need not hide their mutable ancestor lineage.
Tests must prove omitted ancestor invalidation still refuses after improvement;
forged read/write inventories must fail earlier semantic checks.

## Implementation critique

Do not weaken lexical visibility, add author annotations to declare a cell safe,
ignore all private origins, freeze every descendant of a completed parent, or
classify surviving work by its name. Each would bypass the demonstrated writer
counterexample or current authority/input dependencies. No new source syntax or
syntax-tree representation is needed for this analysis correction.

Use a typed historical origin alongside live cells/provider roles. Build one
immutable index of descendant-writable cells from validated node writes; repeated
per-origin full-node scans would add unnecessary cost. Charge traversal, copies
and report records to existing budgets. Apply the classification only when
collecting ordinary invalidation dependencies; preserve protection-only reports
and the IR/source identity. Report conditional historical influences separately
from live invalidation obligations and keep later concurrency/lifetime refusal.

The recommendation is accepted for implementation under ADR-0018 delegation,
subject to ADR-0037, counterexample/visibility/budget tests and separate completed-
diff review. No individual owner or independent review is claimed. The current
checker has not yet been changed. Full RP-01 remains a draft, all forty scenarios
are Partial and all remaining Phase 1 gates stay open.
