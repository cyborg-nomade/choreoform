<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Finite provider metadata closure evidence

**Date:** 2026-11-06<br>
**Plan:** [frozen plan](0035-provider-metadata-plan.md), `52596b0`<br>
**Decision:** [ADR-0035](../decisions/0035-provider-metadata-read-closure.md)

The new exact core 0.3 artifact inherits both frozen 0.2/0.1 pins. Old contracts
and source profiles remain unchanged. Supported core-0.3 policy records classify
validated provider result-protection dependencies as metadata edges. All other
references stay strong, including targets also named by metadata. An iterative
indexed graph checks strong acyclicity/depth, combined SCC eligibility, condensed
DAG depth and least read union, without recursive predicate evaluation or dropped
edges. Returned work is charged against the existing remaining semantic budget;
the public pure helper is graph computation, not contract or execution admission.

Nine new Rust groups cover:

- Real raw RP-01 promotion with shared business-result recipients, static success,
  legacy cycle refusal using each legacy retry shape, exact/missing/corrupt/unknown
  pins and both inherited resource requirements.
- Actual protection, actor, effect and wait data reads through a self-recipient
  provider, a real ordinarily unvisited wait declaration, and exact stale-read
  refusal. Core 0.3 does not invent an ordinary entry or weaken visibility.
- Mixed legacy cycles, wrong-kind result references and private capability refusal.
- A 5,000-policy metadata ring without recursive traversal; exact returned-work
  caller exhaustion, oversized budget and large read-union exhaustion.
- Strong and condensed 64/65 depth boundaries and an edge that is both strong and
  metadata, which must not lose its strong cycle refusal.
- Twelve conformance vectors independently recomputed by a separate Python
  transitive-relation/finite-union implementation, not the Rust SCC algorithm.
- Actual timeout-only, retry-only and combined clock metadata closure, retaining
  all transitive protection/actor/effect/wait reads and refusing their omission.
- The business-only raw graph's ordinary control success and separate clock-label
  `control-information-flow` refusal at `/body/nodes/payment_request_step`.
- The raw graph with coherent business **and** clock labels passing protection,
  then refusing `invalidation-context-unproved` at `/body/nodes/check_rejection`.

The last refusal concerns private closed-child provenance. It is not fixed by
this graph contract. Raw-IR construction recomputes a real definition revision;
no source profile is relabeled as targeting core 0.3. The checked-in parent
candidate/manifest still use core 0.2 and retain their initial static refusal.
No provider, role, clock, approval, payment or lifetime fact is authenticated.

Full local checks passed 267 Rust tests plus five compile-fail doctests, native
workspace format/lint/report, all five library Wasm lints, release portability/
frontend Wasm builds, original draft probes and every CI Python oracle. The new
three-group oracle verifies graph vectors plus exact new/base snapshot bytes.
Logs: `/private/tmp/metadata-tests.log`, `/private/tmp/metadata-native.log`,
`/private/tmp/metadata-wasm.log`, `/private/tmp/metadata-report.log`,
`/private/tmp/metadata-oracles.log`. Review refinements rerun the focused Rust
suite and final workspace lint; required final-head CI repeats all checks.

Separate completed-diff automated self-review checked least rather than arbitrary
fixed points, strong/metadata overlap, legacy SCC boundaries, reference/kind/scope
validation, per-record retry versions, exact inherited bytes, bounded iterative
traversal/set copying and existing total-budget accounting. It removed an unused
SCC-membership allocation and explicitly charged stored-read materialization in
the new validation path. Tests retain later real package refusals and source
non-support. No actionable issue remains. Acceptance is delegated automated
self-review, not independent/individual owner review; final-head CI, comment audit
and expected-head merge into the unmerged parent remain required.

Next freeze explicit authoring support for this exact core, with strict new
source/companion pairing and unchanged older hashes/limits. Then return to the
whole RP-01 package, including coherent provider/clock protection and actual later
invalidation/lifetime/proof checks. Full benchmarks, all forty Partial scenarios,
text/visual/protected editing, actual host/browser/user evidence and Phase 1 exit
remain open. Main effectiveness waits for parent integration.
