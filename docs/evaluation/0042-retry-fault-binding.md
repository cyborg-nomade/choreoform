<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Explicit raw-core retry fault-source evidence

**Date:** 2026-11-06<br>
**Plan:** [frozen scope](0042-retry-fault-binding-plan.md), `0c9479d`;
source-name accounting supplement `222a5b7`<br>
**Decision:** [ADR-0042](../decisions/0042-explicit-retry-fault-source-binding.md)

Core 0.4 identity is `urn:choreoform:contracts:core-profile:0.4.0`, exact revision
`sha256:0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3`.
Its raw definition support requires exact inherited 0.3/0.2/0.1 artifacts and
adds required nullable retry `faultSource`. Single attempts require empty eligible
faults/null bindings; multiple attempts require explicit source/clock/basis and
nonempty names resolved in its verified emissions. Unresolved eligible declarations
require provider stable-key metadata. Actual definitive evidence or implemented
key support, accepted fault, authority, correlation, delay and closure cutoff
remain live host requirements; no new attempt is dispatched by these checks.

Every declared multi-attempt use, including ordinary-unvisited nodes, must be a
capability use matching the exact provider ID/revision. Human contexts and a
reserialized provider with identical types/labels still refuse. Unused policies
can validate source metadata without creating an ordinary entry or RetryUse.
Source provider protection dependencies contribute exact metadata reads. Compatible
0.3/0.4 finite components work; strong/mixed-legacy cycles keep refusing. Old cores
retain their original payload/read/source rules and unbound retry metadata.

`RetryUse::fault_source()` exposes a typed private-constructor `RetryFaultBinding`
with exact definition revision/site/source/profile/provider and eligible fault /
observation/evidence declarations. Legacy None supplies no binding proof. No actual
observation, fault receipt, current authority or executable capability is returned.
The copied evidence shares timing's existing cumulative resource budget.

Inspection found that conservative fault-name discovery previously used only
handler keys. Before changing it, the frozen supplement added every declared
emission of bound sources, including noneligible names. Repeated source pins are
deduplicated; old handler names remain borrowed, resource names owned. Source
decoding and graph/name copying share one work allowance. The existing selection /
active exclusion / cycle logic remains unchanged. Unhandled-source counts concern
abstract possibilities, not actual emitted faults or observed failures.

Ten new Rust integration groups exercise full raw-package binding/revision and
typed premises; every exact inherited/source/profile/provider/type resource;
closed/null/unused/missing fields and unknown eligible names; exact provider bytes
even for unvisited sites; no-key/no-effect versus unresolved evidence; complete
source-name/unhandled accounting and noneligible nearest-handler selection; human
and unused contexts; source metadata read closure in unused policies; compatible
finite versus legacy cycles and legacy unbound reports. One internal group checks
copied typed evidence against exhausted cumulative work. Four Python groups check
exact new/inherited snapshots, independent new shape/name/class/use identity rules
atop prevalidated source metadata, unchanged source/core pairing and refusal of new
syntax in old source, licensing/registry/CI and unchanged Partial/non-executable
status. These are static/metadata tests, not a supported host execution.

Local native verification passed 315 Rust tests plus six compile-fail doctests,
format/workspace lint, all five library Wasm lints and release portability/frontend
Wasm builds. Logs: `/private/tmp/retry-fault-bind-{workspace,clippy,wasm}.log`.
Native portability oracle, all required Python oracles and current-package exact
regeneration/historical probes passed (`/private/tmp/retry-fault-bind-oracles.log`).
Final-head CI/comment audit/signed merge remain required before acceptance.

Separate completed-diff automated review checked version/pin/field boundaries,
all-use exact provider matching, actual-site versus unused declarations, eligible
class constraints versus actual truth, source metadata dependency closure, mixed
version behavior, immutable revision-bound rows, copying budgets and old behavior.
It added noneligible source-name accounting, fused collection into one policy pass,
kept old handler names borrowed and shared resource/graph budget rather than adding
an independent allowance. It clarified the proposal's explicit metadata-family
extension before freezing final artifact bytes. No actionable issue remains.
Review is delegated automated self-review, not independent or individual owner
review. Inspect all comments/CodeRabbit findings and merge the passing expected
head only with explicit sign-off.

Current source 0.6 / companion 0.5 still selects core 0.3. No source/IR/resource/
package report changes or silent migrations are made here. Next freeze and
implement an explicit authoring/companion version and bounded native disclosure,
then adopt a real RP-01 source resource and regenerate evidence. Late-observation
data/continuation, complete validator/benchmarks, actual host/user/browser and all
phase gates remain open; the parent is still draft/incomplete/unmerged.
