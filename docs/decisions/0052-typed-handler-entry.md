<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0052: Bind immutable history to an explicit conditional handler-local entry

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-07<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

P4 specifies immutable fault history, while old scopes have no ordinary fault port.
ADR-0050 supplies an exact nominal type/source descriptor and deliberately limited
untrusted claim check. ADR-0051 retains declared emitting sites and full handler
contexts without initializing anything. A suggestive local cell, ordinary literal
or scope/name route cannot substitute for actual authenticated entry. Follow the
[frozen raw SDK plan](../evaluation/0052-typed-handler-entry-plan.md), `2cbb375`.

## Decision

Publish [Definition IR 0.3](../ir/definition-v0.3.md) and exact
[core 0.7](../dialects/core-fault-entry-v0.7.md), requiring all six inherited bases.
Only scopes change wire shape: required nullable faultInput is null or
`{history:Ref,result:DataID}`. Wait capture stays exactly as in IR 0.2. Keep the
unversioned constructor on IR 0.1 and every earlier grammar/schema/core/wire/hash
fixed. Older selected cores cannot ignore the new field. The original illustrative
contract inspector refuses IR 0.3; transport and structural projection are separate.

History pins the exact ADR-0050 descriptor and transitive source/profile/provider /
fixed nominal bytes. Result is owned locally with exactly that nominal type. It is
a reserved immutable handler cell, with zero ordinary ports, initializer, declared
writer, ordinary creator or P6 follow-up creator, including unvisited declarations.
Validate all bindings, normal source-protection visibility and type identities;
never initialize from a name, available registry bytes or a fabricated nominal value.
Static rows share exact metadata/revision and retain mandatory host premises only.
Validate context read closure without retaining unused per-row read-set copies.

Add separate raw SDK check_fault_entries / FaultEntryReport over one retained origin
pass. Match every incoming supported-source route to the descriptor, requiring an
ordinary covered/reachable source and empty active-before stack. Human/null/legacy,
wrong-source, unvisited or exceptional sources refuse. Conservatively inspect
unexplored source sites along lexical ancestors too, because their omitted active
context cannot justify dropping a possible incoming relation. No incoming context
means an uncovered binding, never vacuous history initialization.

The initial conditional body subset is portless/childless/terminal flat local-only
compute/decision/finish with a reachable finish. It supplies no effects, wait/timer,
parent value/storage/result or continuation. Before conditional initialization,
check the history cell's write-access dependencies against an empty pre-entry value
set. Self/circular/ancestor prechecks refuse. Only then seed its availability for
fresh local initializers and body access/read/write/finish checks. The function
takes no caller history Value and returns no runtime event or initialized storage.

Retain complete conservative ordinary source-scope label contexts, including exact
activity providers and source-input/fault-existence/control/creator-owner provenance.
An upper-bound scope union may overtaint; it is not a fault-site value snapshot.
Generalize the internal P6 retained-label mechanism for both conditional kinds,
without changing old behavior/diagnostics or serialized evidence. Retained cell /
historical labels confer no live ancestor values, grants, ordinary completion or
mutable dependencies. Missing contexts refuse; zero inherited values never erases
provenance. Check complete history initialization, later publication/branch and
initializer flow, current access obligations and strict local invalidation before
returning conditional sets. Preserve all older ordinary/legacy exceptional sets.

Reuse bounded graph/availability/flow/strict helpers. Keep copy/metadata/provenance /
row work inside inherited allowances. Shared revision/metadata avoids repeated
payloads; static required scope fields avoid per-scope heap allocation. A failed
judgment returns no partial proof. A new raw conditional report does not promote
older exceptional coverage, native admission or phase status.

## Alternatives and consequences

| Option | Benefit | Limitation / decision |
| --- | --- | --- |
| Explicit scope reference and reserved local cell | Visible role/ownership, readonly invariant and auditable initialization | Adopt on new wire/core only |
| Ordinary input port, initializer or hidden fault facts | Reuses existing syntax | Reject: permits invented histories and violates old P4 boundaries |
| Seed history before access prechecks | Simplifies availability | Reject: circular permission can read the value it is meant to authorize |
| Borrow ordinary labels for active-handler/unexplored source contexts | More apparent coverage | Reject: incomplete source/control premise; refuse conservatively |
| Retain source/control labels without parent values | Protects exceptional existence and fresh locals | Adopt conditionally; actual provenance must be authenticated/retained by the host |
| Merge conditional sets into ordinary/legacy coverage | Shorter reports | Reject: conflates different initialization and lifetime premises |

[Evidence](../evaluation/0052-typed-handler-entry.md) covers promoted real-package
and finite declarations, type/pin/role/creator/writer/visibility boundaries, access
ordering, source/context completeness, constant/branch/initializer leaks and strict
invalidation. These are not actual faults, host observations, handler executions,
user studies or independent reviews. Delegated automated self-review is recorded
honestly, without personal owner approval or a purported GitHub self-approval.

Actual authenticated non-engine emission, exact occurrence/attempt/cause/input /
request/key/effect correlation, nearest live selection/exclusion, current authority,
fresh read-only owned storage and complete provenance remain mandatory. Cancellation
is monotonic; handler finish cannot resume a parent or return an activity result.
Readable source/companion/native integration and permitted owned continuation /
cutoff/handoff/all-settlement follow separately. Current RP-01 bytes stay unchanged;
parent #46 draft/incomplete, all forty scenarios Partial, and complete validator /
benchmarks/general authoring/visual/protected editing and actual host/user/browser /
independent/phase gates open.
