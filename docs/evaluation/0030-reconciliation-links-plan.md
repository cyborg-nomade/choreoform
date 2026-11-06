<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Owned reconciliation declaration links: frozen plan

**Date:** 2026-11-06<br>
**Baseline:** `4d95e3610488ad98383ab9ddadeff36e5a8fca68` (PR #39)<br>
**Authority:** ADR-0018 delegation; ADR-0013 P3/P6 and ADR-0020 unique wait-site links.

Publish ADR-0030 and a separate reconciliation declaration-link stage after the
existing conditional handler availability stage. The existing unique wait-node
count proves neither a surviving follow-up boundary nor a reachable wait inside
it. Reproduce that the old service fixture passes earlier stages while its scope
has terminal closure and an unattached ordinary reconciliation wait.

For every declared capability activity, resolve its capability/effect/wait policy
and uniquely inferred wait node. Require the activity's own scope to declare a
reconcile closure whose direct child follow-up owns that wait definition, with
the wait syntactically reachable from that follow-up's entry. Reuse a bounded
ordinary graph/reachability index and wait-site index rather than scanning all
nodes per activity. Check every declared use without inventing data contexts;
report ordinary-unvisited sites and unused effect-policy contexts separately.

These are conservative supported declaration conditions, not a language-wide ban
on deeper or delegated ownership schemes. Refuse missing/unproved ownership/route
links with stable declaration paths. Do not guess a receiver from map order,
accept a root ordinary wait as a surviving follow-up, or exempt read-only providers
from the closed effect contract's reconciliation requirement.

Expose immutable per-site capability/effect/closure/follow-up/wait references and
explicit host/entry/lifetime requirements. No actual obligation/subscription set,
successful follow-up activation, accepted transfer, current authority, correlated
fact, snapshot, wait matching, multiple-obligation completion or atomic settlement
is proved. Prior availability/routing/timing/protection requirements remain open;
reports/native output stay bounded and non-executable. Keep old APIs, snapshots,
fixtures, wire/core/source versions and dependencies unchanged.

This PR supplies direct-IR positive/refusal evidence. The source follow-up must
be explicit in Roadmap: a separately versioned authoring profile will allow a
parent wait policy to name its designated reconciliation follow-up's wait outcomes
through a qualified endpoint reference, preserving ordinary lexical/data privacy.
Current source binding ties the wait policy to a lexically visible node and cannot
express that owned child endpoint correctly; do not proxy it with a dummy root
node or silently broaden old source visibility. Review/freeze that profile before
implementing it, without changing the already expressible raw IR contract.

Tests: prior-stage false confidence; correct direct-IR child owner and reachable
wait; wrong/missing/nested owner, terminal closure, unconnected wait, reused policy
across incompatible activity scopes, ordinary-unvisited declarations, unused
effect contexts, read-only providers and immutable revision/host requirements;
bounded repeated site lookups. Source oracles preserve earlier behavior and
refuse the missing follow-up at the new stage honestly. Run native/Wasm and
relevant artifact/stage checks; self-review separately; record delegated acceptance,
require final-head CI/expected-head merge, then complete the source follow-up and
remaining lifecycle/fault-protection/invalidation/full-validator Phase 1 evidence.
