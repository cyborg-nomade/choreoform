<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0062: Separate finite owned work from read-only query semantics

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-10<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

Core 0.10/source 0.13 supports useful read-only inquiry before original capture,
with exact original-request input. Commands, compensation and constructed inputs
remain deliberately excluded. The signed [audit plan](../evaluation/0062-owned-command-audit-plan.md),
`0d4f884`, precedes eight SDK and one frontend probe groups. Production behavior
and accepted artifacts remain unchanged by this audit.

ADR-0008 defines compensation as new work linked to a prior confirmed or possibly
completed effect, with its own authority/outcome and no fictional rollback. Its
own effect class is independent of its compensates link. P3 assigns new logical
keys to new business work and compensation; only technical attempts retain a key.
Irreversible/consequential races require appropriate declared conflict/invariant
constraints, not accidental scheduler order. None is supplied by a static report.

## Decision

Freeze a separate signed implementation plan for an explicit new core on existing
IR 0.4: support bounded pure input construction over available local values and a
distinct finite owned work relation for matched commands/explicit compensation.
Preserve all old core/query/source/report meanings; do not relabel commands as
queries. No new wire field is demonstrated necessary by the concrete probe.

Keep the finite one-capability-per-intermediate-body/direct designated child shape,
readonly original accepted request seed, empty pre-entry access, complete source /
input/control/provider/owner provenance and actual recipient/invalidation checks.
Constructed inputs must remain ordinary supported pure expressions, exactly typed
for their actual capability with complete declared reads, no parameters or invented
facts, no foreign live data and normal conditional availability/protection. Their
accepted immutable values/revisions/digest remain host obligations. Child request
seeds use the new activity's actual input type, not its parent's seed type.

New non-read-only work should initially require a capturing entry at the original
source's unique actual-use timerless wait, with local uninitialized exclusively
written capture target. This serializes its declared path after an actual accepted
terminal capture; broader preterminal commands/compensation remain separate work.
Any explicit compensator, including read-only class, also uses capture-first entry;
only uncompensated read-only inquiry retains pre-capture entry and availability barriers.
Keep new command providers/effects matched, stable-key, one-attempt, timerless and
bound to their own general source/next designated child. An optional compensates
link must identify the immediate original source; never erase the old effect or
reuse its logical key as a new command. Do not infer compensation success or the
prior live effect's eligibility from a declaration, tag spelling or effect class.

Expose distinct immutable work/class/input/compensation relations and explicit live
new-effect/compensation gates without changing query getters/old row meanings.
Require fresh current authority, actual eligible correlated prior effect for
compensation, safe conflict/invariant acceptance, exact request revisions/digests /
occurrences/effect/key/attempt, supported provider promises, immutable storage,
P6 registration/cutoff/accepted atomic handoff and all actually activated transitive
accountability/settlement. Failed receiver acceptance leaves the old owner liable.
No generated receipt, missing fact, query/command success, finish or graph outcome
supplies activation, original settlement, parent resumption or liveness.

## Alternatives and consequences

Effect-class loosening alone ignores input, compensation, source/child ownership,
identity/conflict and report terminology. Supporting only constructors is useful but
leaves explicitly linked new work unrepresented in the conditional proof. A combined
bounded work contract can reuse existing graph/flow/provenance machinery while
keeping clear restrictions. Broad preterminal commands/general P4 activation or
arbitrary nesting require later judgments and cannot borrow this finite proof.
The capture-first restriction does not implement ADR-0008's entire possibly-completed
compensation generality; keep that limitation explicit in the Roadmap.

The [critique](../evaluation/0062-owned-command-audit.md) records actual first
refusals, existing representation and report gaps. This audit grants no production
permission. Parent #46 remains draft, every scenario Partial and G1–G4 conditional;
complete validator/benchmarks/general near-English/protected accessible visual and
real host/browser/user/independent/phase evidence remain open. No Phase 2.
