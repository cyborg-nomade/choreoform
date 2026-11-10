<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Finite read-only owned continuation: frozen raw SDK plan

**Date:** 2026-11-08<br>
**Baseline:** `ae28e9467724d1acf681ac0800f82e49485aff8c`, draft #46 after PR #71<br>
**Authority:** ADR-0018 / ADR-0056; conditional declarations, never live authorization

Use a dedicated stacked implementation PR and ADR-0057 after the contract audit.
Publish an exact core 0.9 artifact requiring all eight inherited bases, selecting
existing IR 0.4 explicitly. No new serialized relation is needed: scope/closure,
activity/capability/work/source, capture and retainedRequests already identify the
finite ownership family. Keep every prior wire/schema/core/source/hash/artifact and
committed package/report fixed. Reject mixed core 0.8/0.9 aliases: scopes have no
per-scope dialect tag, so a dangling new alias must not loosen old scope rules.
Old aliases selecting one exact prior core remain unchanged. Add only explicit
known-core family checks and exact inherited verification order, no version ordering.

Extend static request declarations only in selected core 0.9: a retained-request
scope may have terminal closure and no children, or reconcile closure with exactly
its designated direct child. Preserve local exact origin-input type, readonly
uninitialized result, no ports/ordinary creators/fault-handler reuse, normal
visibility/provider/protection and every capture in a body bound to that body's
one original source. No arbitrary extra lexical children. Metadata admission is
separate from completed conditional source/lifetime/flow proof.

Keep old flat OwnedLocalReport semantics/path. Add a distinct private-constructor
immutable OwnedContinuationReport on concurrency and later SDK chains, optional
when absent. Root the new family at a covered synchronous invoke/repeat and an
ordinary-covered exact source activity in that body. Initially require that source
owner's ordinary body to contain only local compute/decision/finish plus its one
external capability activity; refuse hidden ordinary calls/parallel/human sources.
The designated continuation has its exact retained request and timerless capturing
entry. Each intermediate body has at most one declared new capability activity and
exactly its own designated surviving child. Terminal leaves stay passive flat local
wait/compute/decision/finish. Inspect every declaration, including unvisited ones.
No ordinary invoke/repeat/split/fanout/human work, arbitrary nested scope, ports,
foreign writes/reads, independent creator or parent outputs/resumption in the new
continuation family. Preserve separate ordinary, legacy and old-flat checked sets.

New work must use its exact supported read-only provider/effect contract, one attempt,
no timer/retry/clock, explicit general fault source matching the actual provider and
normal current actor/work/capability/access/management premises. Its input initially
is the exact readonly retained-request cell, not re-evaluation/transform of ancestor
input; require complete reads confined to that cell so mutable peers do not silently
change the request or replay it. Each new logical request has a new effect/key identity
under existing P3; add no ad-hoc key field. Original commands may remain pending;
only new owned work is restricted read-only. Command/compensation generality is later.

Index the finite ownership tree once and use owner-before-child analyses with shared
bounded graphs/templates. A root source uses retained ordinary source/control /
provider context; each nested source requires actual prior conditional coverage in
its parent, never an empty fallback or ordinary coverage promotion. Check target
write access with zero inherited live values before each seed, then fresh local
initialization, capture/query/branch/access/publication and strict invalidation.
Retain complete input/policy/source/fault-existence/control/provider/creator-owner
labels through every handoff. Label references are not values or current grants.
Require exact source-provider origins in retained context. Avoid cloning an entire
context map per child; consume/share bounded per-owner sets and charge every copy.
Reject unvisited/unsupported source contexts and wider constant/initializer/branch /
provider publication. Normal guard/protection recipient restrictions still apply;
never broaden accepted benchmark metadata to make a query pass.

Keep nearest/active fault routing and existing ordinary-only history-entry gates.
Conditional query sources cannot be omitted from possible incoming handler checks
or relabeled as ordinary. All declaration/conditional/uncovered relations and
unsupported/unhandled contexts remain explicit. No event values or host callbacks
are parameters to the proof; no actual request/receipt/storage/grant is cached.

Expose immutable revision/creator/owner/body/source/query/child/request/type /
conditional coverage/access/provenance/invalidation relations and declaration-level
registration, handoff, ordinary-closure and terminal-settlement gates. These require
all actually activated transitive effects/subscriptions/children, not a count of
declarations. Enforce no runtime transitions: actual current permission and open /
noncancelled registration cutoff, fresh occurrence-owned immutable storage, exact
source/request/value/input revisions/digest/effect/key/attempt correlation, receiver
acceptance and atomic single-owner handoff/ledger enforcement remain host premises.
Missing acceptance leaves the old owner accountable and closure pending. A terminal
leaf does not reopen, clear cancellation, return a parent activity result or claim
all live work settled. Finite means definition shape, not eventual settlement.

Charge indexing, graph/flow, context copying, template sharing and pending/public
rows cumulatively against existing semantic allowances, with depth bounds and no
partial proof on exhaustion. Preserve the old no-continuation cost/path where
possible; do not recursively call public composed admission or rerun for output.
Raw SDK first; source/native selection follows a separate explicit version/profile.

Test the audit baseline and new nested read-only query/capture/request/read/branch
chain through actual SDK stages; ordinary/legacy/old-flat sets unchanged. Check exact
pins/base/mixed-alias/wire/type/readonly/creator/child/source boundaries, parent and
conditional-unvisited sources, empty self/peer/ancestor access, source/control /
constant/initializer/branch/provider egress leaks, strict invalidation, wrong/missing
request/capture, hidden/unvisited command or extra query, retry/timer/human/parallel /
ordinary resumption/foreign storage, conditional fault-context refusal, fresh reuse
and cumulative graph/context/copy/publication exhaustion. Add a shared finite corpus
with limited independent Python premises, exact core hash checks and old source /
report/package compatibility, never generated host evidence. All actual grants,
cutoffs/handoff/settlement, complete validator/benchmarks/editing and external gates
remain open. Use genuine probes/precise refusal results for any full-package limit.

Run native/Wasm/all Python/final-head CI, separate completed-diff delegated review,
all unresolved threads/CodeRabbit summaries and signed expected-head merge. Record
progress and next readable selection, then command/compensation and richer ownership
judgments. Parent #46 stays draft/incomplete, all forty scenarios Partial, G1–G4
conditional; no Phase 2 or phase exit. Sign a plan refinement first if a concrete
serialized-relation gap or audit incompatibility requires changing this scope.
