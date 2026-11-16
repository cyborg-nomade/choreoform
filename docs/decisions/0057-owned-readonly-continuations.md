<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0057: Check finite read-only surviving continuations separately

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-10<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

[ADR-0056](0056-owned-continuation-direction.md) demonstrated that flat terminal
retention cannot justify capability work that needs a further reconciliation
child. The [signed implementation plan](../evaluation/0057-owned-readonly-continuations-plan.md),
`22654bb`, freezes a raw SDK increment before production changes. IR 0.4 already
contains the necessary source, request, capability, scope and closure relations.
Source/result metadata and a true completion predicate cannot supply an ownership
proof, complete information flow or current authority.

## Decision

Publish exact [core 0.9](../dialects/core-owned-readonly-v0.9.md), requiring all
eight prior bases and existing IR 0.4. Require uniform exact selected aliases;
scopes have no dialect tag that could safely distinguish mixed 0.8/0.9 meanings.
Earlier contracts, wire/source/hash domains, quotas and accepted artifacts stay
fixed. Static retention admits exactly a designated nested reconciliation child,
preserving all existing exact local readonly request and capture checks.

Keep old flat owned proof semantics. Add a distinct immutable
OwnedContinuationReport rooted at an ordinarily covered synchronous creator and
its one ordinarily covered source capability activity. Intermediate surviving
bodies have one exact original-request seed, timerless capturing entry, at most
one new read-only capability activity and exactly its own designated child.
Terminal leaves remain passive local wait/compute/decision/finish. Check all
declarations, including unvisited queries and lexical children. Exclude commands,
compensation, timers/retries, human/parallel/ordinary nested calls, foreign storage,
ports and ordinary parent resumption in this initial family.

Analyze each shared template once, owner before child. A nested source needs
actual prior conditional coverage and complete retained source/control/provider
provenance, never an empty fallback or ordinary promotion. Check target write access
with zero inherited live values, fresh initialization, complete protected transfers
and strict invalidation. Query input must be exactly its readonly retained request;
complete reads cannot add peers or live ancestor state. Each actual query capability
ID must be an allowed recipient of every input/access/control/retained source label,
even when no result is published. Provider resultProtection is a source lower
bound, not a provider input-recipient contract. This additional check applies only
to the new conditional flow. Existing ordinary behavior remains unchanged.

Index once, share immutable templates across creator declarations, and consume
bounded per-owner provenance rather than clone the entire context map per level.
Charge indexes, graph/flow, copying, pending relations and final publication against
existing cumulative stage allowances. Refuse without partial proof on exhaustion.
Keep ordinary, legacy, old flat and new conditional sets separate through the final
SDK chain. Conditional faults retain existing possible-incoming and ordinary-only
history-entry refusals; richer fault entry is not inferred from query coverage.

Expose revision/creator/owner/source/query/child/request, access/provenance /
invalidation and registration/handoff/ordinary-closure/terminal-settlement gates.
These are mandatory future host premises, not accepted transitions. Require fresh
current authority and open/noncancelled registration before cutoff, exact occurrence /
request/input/value/digest/effect/key/attempt correlation, fresh immutable storage,
authenticated provenance and atomic accepted single-owner handoff. Missing receiver
acceptance leaves the old owner accountable and closure pending. Terminal disposal
requires all actually activated transitive effects/subscriptions/children settled;
a capture/finish or declaration count cannot prove it. No report dispatches work,
clears cancellation, reopens terminal instances or guarantees eventual settlement.

## Alternatives and consequences

Loosening the old flat node-kind check would omit transitive ownership/source /
protection obligations; reject it. Broad command/compensation admission would add
irreversible guard/key/conflict responsibilities before those rules are checked;
defer it explicitly. Same-scope terminal continuation needs different activation
rules and remains later work. A new wire relation is unnecessary for this finite
family, so retain IR 0.4 and make semantic selection explicit instead.

The bounded tree is narrower than general recovery, but its evidence and remaining
host premises are reviewable. [Evaluation](../evaluation/0057-owned-readonly-continuations.md)
records genuine SDK tests, limited independent Python vectors and compatibility.
Readable/native selection follows separately, then command/compensation and richer
ownership judgments. Parent #46 stays draft/incomplete; all forty scenarios remain
Partial, G1–G4 conditional. Full validator, benchmarks, near-English/protected visual
editing and real host/browser/user/independent/phase gates remain open. No Phase 2.
