<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0053: Integrate readable history entry with separate conditional reports

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-08<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

ADR-0052 supplies explicit raw IR 0.3/core 0.7 declarations and conditional local
history proof. Source 0.9 cannot express them and the legacy exceptional pipeline
cannot initialize that reserved cell. Follow the [frozen plan](../evaluation/0053-handler-entry-authoring-plan.md),
`d1e1b70`. Preserve earlier profile meanings, evidence boundaries and current RP-01
bytes while making the accepted raw contract readable and reviewable.

## Decision

Publish [source 0.10 / companion 0.9](../authoring/profile-v0.10.md), selecting exact
core 0.7/IR 0.3 and all six inherited bases. Add only an optional scope clause after
its interface: `Receive fault history from "Payment history" and record in "Fault history".`
The conjunction separates two normally qualified references without ambiguity.
Only this pair adds provider-fault-history resources. Resolve the descriptor through
normal resource visibility and require the target cell to be local. Reuse exact SDK
metadata/type/provider readers, including unused declared resources. Every scope
lowers required nullable faultInput. Neither source nor lowering supplies a value.

When non-null bindings exist, exceptional availability reuses check_fault_entries
and its one retained prior-stage/origin chain. Share immutable FaultReport through
an internal Arc; keep public borrowed getters. Skip bound handlers in legacy input
assumptions. Expose history_entries separately and retain it through reconciliation.
Do not add typed body nodes to legacy/ordinary checked sets or erase legacy
uncovered sets. This intentionally replaces ADR-0052's raw-only pipeline refusal
with a separate conditional judgment, without granting ordinary availability.
The old no-binding path pays a fixed Arc allocation, but no extra semantic pass.
No live value, permission, receipt or occurrence is cached.

All eleven native checking/advisory views disclose nonempty declaredFaultEntries
and a per-exact-descriptor declaredFaultHistories catalogue from retained static
rows. Exceptional and later views add conditionalFaultEntries with distinct
coverage/flow/access/invalidation/provenance sets, supported incoming sites and
full selection stacks, plus mandatory host premises. Metadata sharing is distinct
from occurrence identity. Omit empty additions; old report content stays stable.
Budget static and conditional rows cumulatively with bounded serialization, include
inherited report content, and bound the complete final JSON to 1 MiB before stdout.
Refuse rather than truncate or drop declarations. No second admission is run for
reporting.

Keep exact old grammar/schema/core/wire/hash domains. Known whole-program quotas
remain 16,384 tokens / 1,000,000 parser visits and the existing lowering/dependency
allowance. Unknown headers select no new quota. Formatting and coordinated name
changes preserve semantic identity within this explicit new profile; promotion
changes package framing, core and wire revision.

## Alternatives and consequences

Keeping raw-only entry would leave readable review and composed evidence unusable.
Treating history as an ordinary input or merging checked sets would erase the
conditional initialization, source-control and host premises. Re-running the SDK
for output would duplicate expensive validation and risk inconsistent evidence.
Deep-copying the prior report would scale with the graph; immutable sharing costs
one small allocation and a private representation change instead.

The syntax remains an intermediate near-English design. General identity/resource
editing tools, complete English expressions and real representative-user evidence
remain explicit roadmap work. Actual authenticated event/correlation, current
source/work/access/management grants, fresh owned immutable storage, retained
source/control provenance and exact live selection remain host obligations. A
handler cannot resume its parent or publish an activity result. Permitted owned
continuation with retained request/value identity, cancellation/closure cutoff and
all-settlement follows separately. No scenario or phase exit is inferred; parent
#46 remains draft and all forty scenarios Partial. See [evidence](../evaluation/0053-handler-entry-authoring.md).
