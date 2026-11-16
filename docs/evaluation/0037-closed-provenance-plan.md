<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Closed ordinary provenance: frozen audit and proof plan

**Date:** 2026-11-06<br>
**Baseline:** `3f67ab3bf54ed0e564f27dc8d6a0690b12b9f620`, draft #46<br>
**Authority:** ADR-0018; actual RP-01 invalidation context refusal.

Use a dedicated stacked dependency PR targeting the unmerged RP-01 branch.
First write a critique/audit with actual native baseline witnesses, before changing
the analysis. Reproduce a simple completed child's private-input/control origin
escaping through invoke/repeat, the full RP-01 refusal, and cases with mutable
ancestor inputs and descendant/follow-up writers. Review the core's declared writes,
input/output transfers, fresh occurrences and P6 ordinary-closure rules. If a required
semantic premise is absent, freeze a versioned contract dependency instead of
inventing it. No source/core pin is changed merely to bypass a checker.

Proposed bounded proof subset, subject to that audit: on synchronous invoke or
repeat-body ordinary completion, a private cell **owned by that exact completed
body scope** may become a historical origin only if no declared descendant node
can write it. Own ordinary nodes cannot run again after that occurrence closes;
new invocations/iterations create fresh storage. Index every descendant writer,
including ordinary-unvisited, handler/follow-up and explicit output-binding writes;
do not infer safety from ordinary reachability or a wait-looking name. Keep all
unproved descendant-cell origins and all mutable ancestor/root origins live.
Do not freeze split/fanout children from a partial/all-hypothetical summary.

Verify transferred/initialized values retain all mutable ancestor input/access/
control origins through the required node reads; do not erase lineage behind a
copy. Historical origins retain their original protection label for every transfer.
They are not guaranteed values, exported private data, current authorization or
proof of effect settlement. Preserve live policy/fact/host requirements and
nonterminal lifetime refusals. Report exact conditional closure boundaries/cells
in invalidation evidence, without actual occurrence/value/clock claims.

If the subset is justified, record ADR-0037 and implement it only in ordinary
invalidation provenance, sharing one immutable writer index and existing total
budget. Keep protection-only checks and existing source/IR revisions unchanged.
Return typed non-executable evidence, preserving mutable invalidation obligations.
Do not broaden exceptional/surviving availability, concurrency, handler continuation
or ordinary output guarantees. Scope contexts used by later checks may omit only
proved historical origins; they must retain all live ancestor influences.

Tests must contrast invoke/repeat positives, mutable ancestor omission, descendant
and surviving writers (even unreachable), private policy/access dependencies,
output transfers, repeated/multiple call sites, partial fork/join, forged read/write
inventories, exact conditional report metadata and budget/report bounds. Preserve
old negative coverage except the explicitly proved subset. A positive synthetic
snapshot is not a complete benchmark or a real host observation.

Rerun actual RP-01 stages and retain the next genuine refusal; update only derived
report/refusal artifacts and assertions if analysis progress changes that checkpoint.
Do not hide subsequent invalidation/lifetime faults or mark #46 ready. Run native/
Wasm/all relevant Python checks, separate completed-diff self-review, inspect all
comments/CodeRabbit summaries, sign off commits and merge passing expected heads.
Then return to the next real package/proof dependency. All forty scenarios remain
Partial and Phase 1/actual host/browser/user gates stay open.

## Inherited-context evidence clarification

The planned substitution can remove a spurious `scope-context-unproved` refusal
when a parallel child inherits only a proved historical influence. It does not
change the concurrency/conflict/lifetime algorithm. Record that use with its
creator node and optional destination context scope, not just later consequential
sites; preserve the original refusal when an unvisited descendant writer keeps
the origin live. A live origin already in a caller context is never upgraded by
a later call to the same template. These are explicit tests of the planned context
substitution, not permission to freeze fork/fanout summaries or mutable peers.
