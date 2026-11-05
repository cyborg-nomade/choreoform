<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Protected ordinary transfers: frozen implementation plan

**Date:** 2026-11-05<br>
**Baseline:** `384e3add214bbf1be7aecbf520a9c027d563898a` (PR #23)<br>
**Authority:** Owner's phase delegation, ADR-0018. Commit before implementation.

Implement an explicit `check-protection` stage after existing static/control
checks. No executable credential, actual access authorization, clock/host or
phase-exit claim. ADR-0019 will record the chosen conservative boundary and
alternatives. Preserve earlier report contracts and all frozen resources.

Reuse iterative ordinary graph order. Carry a monotone set of protection origins
through each ordinary path; union at merges and retain origins after rejoining
(no postdominator-based release or declassification). Include node read and
published-result origins in subsequent control context. Analyze child scopes with
inherited control origins and fresh contexts; union child completion origins at
invoke/repeat/split/fanout continuations, including conservative zero/parallel
cases. Include provider result protection even when no result cell is published.

Apply the accepted identical sensitivity/policy and narrowing access-set transfer
rule to inherited control origins and writes, initializer dependencies, input
arguments to child input cells, and all output/finish transfers. Include validated
policy/access-metadata read dependencies rather than assuming predicates have no
data provenance. Do not prune constant guards or invent ordering/scheduling.

Enumerate immutable revision-bound access obligations by site/cell/operation for
the modeled reads, writes and transfers, including root ports and initializers.
Refuse access whose participant and capability sets are both empty. Keep actual
principal/purpose/capability membership, policy truth/current snapshot, actor
authority, atomic enforcement and initial access-policy availability explicit
remaining host/full-validator obligations. A report is a constraint/proof summary,
not evidence of authenticated access or permission to dispatch.

Bound scope depth and cumulative graph/origin/transfer work. Error codes and ID
paths never echo values. Keep unsupported exceptional paths explicit via coverage
and residual flags. No new third-party dependencies or public mutable report state.

Required evidence: implicit branch/merge leaks including literals and constant
guards; initialization and invocation input/output transfers; nested and parallel
control origins; provider results without publication; access-set emptiness and
immutable obligation metadata; unreachable coverage and bounded refusal. Add
source/format/reparse/CLI evidence and native/Wasm lint/build coverage. Self-review
the completed diff separately, address all available review findings, verify final
head CI and merge under delegation. Continue Phase 1 after this item.
