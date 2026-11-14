<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0056: Extend owned work through explicit finite surviving ownership

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-08<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

Readonly original-request retention is raw/readable and conditionally checked in
ADR-0054/0055. It does not permit new capability work. The [frozen audit](../evaluation/0056-owned-continuation-audit-plan.md),
`f734d3e`, and [seven actual SDK probes/critique](../evaluation/0056-owned-continuation-audit.md)
show independent restrictions: the owned body is flat and terminal, while every
capability use requires its own reachable designated reconciliation child. Removing
one node-kind check would omit that lifetime/source/ownership dependency. Earlier
metadata checks do not supply current grants, values or settlement evidence.

## Decision

Preserve all production behavior and accepted contracts in this audit. Next define
an explicit new exact core and distinct immutable conditional proof for finite
nested surviving ownership, starting with read-only capability work. Freeze/sign
that implementation plan before code. IR 0.4 already represents scope/closure,
capability/work/source/capture/request/type/protection relations; use it unless a
concrete missing serialized relation justifies a separately recorded wire change.
Earlier source/core/wire/hash/report meanings stay fixed. Readable selection follows
separately; acceptance of this direction is not implementation or host admission.

A covered ordinary synchronous creator roots the supported family. Each body has
its own exact readonly original request and timerless capturing entry. Intermediate
bodies may perform at most one explicitly declared read-only capability activity,
with a direct designated surviving child for its new request. Terminal leaves stay
flat and passive. Exclude general command/compensation, human work, parallel/fanout,
nested ordinary calls/repeats, ports, arbitrary ancestor retention and ordinary
parent resumption. Classify by exact supported provider/effect metadata, not a name
or true guard. Preserve complete source/fault/control/protection and current
actor/work/capability/access/management authority requirements.

Analyze the bounded tree owner-before-child, retaining proven conditional source
contexts separately from ordinary ones. A nested source cannot use an empty
provenance fallback, invented inherited value or unvisited declaration as coverage.
Check empty pre-entry write access, local initialization, every declared body /
source/provider/publication and strict invalidation before returning proof. Share
immutable templates, charge indexes/work/context copies/publication cumulatively,
and keep ordinary/legacy/old-flat report sets distinct. Preserve fault-selection
and unsupported-context refusals; do not infer richer history-entry coverage.

Expose explicit declaration-level registration/handoff/closure/all-settlement gates.
Actual registration must occur before owner disposal and before cancellation /
closure cutoff, under fresh current authority. Each new request has its own logical
effect/key identity; only technical retries preserve old identity. Receiver
acceptance and atomic single-owner update are mandatory; unaccepted transfer leaves
the old owner accountable. Intermediate ordinary closure is not disposal, and
terminal leaves require all actually activated transitive effects/subscriptions /
children settled. A declaration count, callback, join or finish cannot prove that.
Keep actual occurrence/request/input/value revisions, immutable owned storage,
authenticated provenance and current permission/atomic ledger enforcement as host
premises. No report dispatches work or clears cancellation.

## Alternatives and consequences

Same-scope reconciliation under terminal closure needs new activation/ownership
rules and cannot reuse the current designated-child proof; defer it. Ordinary
parent work is a different success path, not late resumption after cancellation /
disposal; reject it as a substitute. General command/compensation first would add
irreversible guards, key/budget/conflict responsibilities before transitive
ownership is checked; defer explicitly. A capture/finish-as-settlement shortcut
would hide outstanding live work and invent evidence; reject it.

This adds implementation and bounded context/proof work, but makes ownership,
cutoffs and remaining obligations reviewable using existing explicit relations.
It does not remove general validator/benchmark/editing or actual host/browser/user /
independent evidence gates. Continue independent implementation after audit review /
CI/merge without per-item owner approval under ADR-0018. Parent #46 remains draft /
incomplete, every scenario Partial, G1–G4 conditional; no Phase 2 or phase exit.
