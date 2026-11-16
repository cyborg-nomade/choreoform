<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0045: Represent accepted wait capture explicitly and version the wire

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-07<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

Audit #57 establishes that current waits cannot retain payloads or jump into parent
ordinary work, and the closed IR 0.1 variant rejects new fields. ADR-0044 now provides
exact declared terminal source/type/protection premises, separate from actual
observation truth. A capture needs per-use destination/origin/source, complete
protection/dependencies and sound outcome initialization, without changing old
programs or claiming surviving storage/continuation. Follow the
[frozen plan](../evaluation/0045-wait-capture-plan.md), `0e01746`.

## Decision

Adopt [Definition IR 0.2](../ir/definition-v0.2.md), adding only a required nullable
wait capture object `{source:Ref,origin:NodeID,result:DataID}`. Preserve every old
wire/core/schema/fixture/hash. The old constructor stays fixed 0.1; callers select
0.2 explicitly. Version already participates in canonical semantic hashing.
Keep role-typed projection and exact closed shape/ref checks distinct from semantics.

Publish exact [core 0.5](../dialects/core-wait-capture-v0.5.md) inheriting all four
prior artifacts. Only its wait policy can use non-null capture on IR 0.2; old
policies allow null only. Require timerless sole onObservation completion, normal
actual-scope visibility, capability origin with exact source provider identity,
exact normalized/wait/destination type and P2 no-declassification source transfer.
Check unvisited declarations too. Derive exact node writes and all wait/management/
source/destination access reads; include ordinary control provenance, invalidation
and writer inventories. Capture metadata links neither activate the origin nor
supply prior runtime values. Accepted actual correlation/authority remains mandatory.

Expose immutable revision-bound declaration metadata only. Retain the checked
static report through composed results and expose it without rerunning validation.
Cache only immutable type equality by policy/cell and exact source profile/provider;
recheck actual visibility, provider identity, writes/protection and access references
per use. A 421-capture/2,500-field stress case reuses metadata without a new quota.
A completed accepted
observation is the sole initialization point; unmatched, duplicate, stale,
nonterminal or ordinary-cancelled input never writes or completes. Initially refuse
capturing timers because existing write-based availability cannot imply a value
on a timeout branch. Preserve data-free survivor and zero-inherited-value limits;
owned local storage requires a separate proof. No fault-data port or parent resume.

## Options and consequences

| Option | Benefit | Cost or limitation | Decision |
| --- | --- | --- | --- |
| Explicit per-use IR-0.2 capture | Inspectable origin/source/target roles and actual-scope checks | New wire/schema/typed API plus later authoring support | Adopt |
| Versioned policy-body capture table | Stable outer node shape | Shared ancestor policy needs complex per-use private context/dependency semantics | Defer; unnecessary for this foundation |
| Add a field to old IR or use annotations | Small patch | Silently changes closed contracts or hides semantics | Reject |
| Implicit payload facts or handler-to-result conversion | Short source | Leaks policy facts, bypasses ownership/cancellation and conflates lifecycle outcomes | Reject |
| Timed capture with unconditional writes | Broad initial syntax | Would claim initialization on the timer branch | Reject until branch-sensitive proof exists |

A structural pass or capture declaration cannot accept observations, authenticate
host/normalizer support, grant current access/retry permission or establish owned
storage, source truth, cutoff and all-settlement. Succeeded business problems remain
business results. Subsequent explicit source/companion/native integration must use
readable names and tool-managed identities; ordinary authors are not expected to
edit this JSON. Current source 0.7/core 0.4/IR 0.1 package remains unchanged until
separate adoption after its remaining contracts are resolved.

Run raw SDK/version/schema/type/flow/availability/refusal/budget evidence, full
native/Wasm/Python/final-head CI and separate completed-diff self-review; inspect
every PR/CodeRabbit finding and merge only the signed expected head under delegation.
[Evidence](../evaluation/0045-wait-capture.md) records actual checks and separate
completed-diff automated review. No independent/individual owner review or
scenario/phase exit is implied. Parent
#46 stays draft/incomplete; owned storage/fault data/continuation and all external /
validator/visual/protected-authoring gates remain open.
