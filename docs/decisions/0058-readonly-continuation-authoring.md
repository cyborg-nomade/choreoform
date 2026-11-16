<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0058: Select and disclose finite continuations explicitly in readable source

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-10<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

[ADR-0057](0057-owned-readonly-continuations.md) defines a distinct finite read-only
conditional ownership proof on existing IR 0.4. Source 0.11 deliberately selects
older core 0.8, so nested continuation proof cannot be exposed by silently changing
that profile. Existing sentences already express the supported relations; another
wire field or new runtime-looking sentence is unnecessary. The
[signed plan](../evaluation/0058-readonly-continuation-authoring-plan.md), `7e7dfb3`,
freezes this integration before production changes.

## Decision

Add explicit [source 0.12 / companion 0.11](../authoring/profile-v0.12.md), selecting
exact existing core 0.9, all eight inherited bases and unchanged IR 0.4. Keep grammar
productions, normal roles/visibility/resource validation and all old profiles,
quotas, schemas, artifacts, hashes and reports fixed. Preserve private original UTF-8
clause spans and complete metadata reads. Libraries supply required exact artifacts;
CLI embeds only known semantics/core bytes. Source selection changes no raw rules.

In native views whose SDK stage includes concurrency, disclose bounded creator /
owner relations, a shared conditional scope catalogue, separate proof sets,
access/provenance/invalidation and declared registration/handoff/ordinary-closure /
terminal-settlement gates. Earlier views retain declarations only. Final
reconciliation names continuationScopes separately from retainedRequestScopes and
legacy checked sets. The same-revision static request relation identifies its exact
validated type; avoid schema copies or a second authority model. Empty new fields
stay omitted and old/default report bytes stay fixed.

Reuse immutable SDK proof rather than rerun admission for serialization. Reserve
inherited output, bound all pending rows, deduplicate scope relations for reused
creators, and keep the final complete 1 MiB pre-stdout bound. Refuse without partial
output. These are nonexecuting conditional declarations, not requests, storage,
grants, accepted transitions, live ownership or actual settlement.

## Alternatives and consequences

Implicitly upgrading source 0.11 would change accepted meaning and hash selection;
reject it. Adding bespoke continuation syntax duplicates relations already expressed
by normal request/capture/retention/closure sentences. Reusing old flat/ordinary
checked fields would overclaim coverage; retain distinct conditional evidence.
Repeating scope/type schemas for every creator would inflate reports; share scope
relations and retain validated static type references instead.

[Evidence](../evaluation/0058-readonly-continuation-authoring.md) uses real SDK/native
checks on synthetic static declarations, qualified/Unicode/format identity and
original clause locations, raw refusal and legacy barriers, exact profile/pins,
shared catalogue and genuine combined output exhaustion. It supplies no external
execution/user/independent evidence. Next command/compensation and richer ownership /
fault contexts, with live current-authority/cutoff/atomic accepted handoff and all
activated transitive settlement premises intact. Parent #46 remains draft /
incomplete, every scenario Partial and G1–G4 conditional. General near-English /
protected visual editing, complete validator/benchmarks and every genuine phase /
external gate stay open; no Phase 2 or phase exit.
