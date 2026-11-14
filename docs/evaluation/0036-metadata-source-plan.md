<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Explicit metadata-core source support: frozen dependency plan

**Date:** 2026-11-06<br>
**Baseline:** Draft #46 plus expected passing metadata-core dependency #48<br>
**Authority:** ADR-0018; ADR-0035 and measured full RP-01 refusals.

After the expected passing head of #48 merges into the unmerged parent, create a
separate stacked source dependency PR. Publish ADR-0036, source 0.6.0 / companion
0.5.0 selecting exact core 0.3 plus both inherited artifacts. Keep the unchanged
0.3 EBNF, source-0.5 whole-program quotas/actual-use endpoint rules, all earlier
source/companion/core pairs, hash domains and refusal behavior. Unknown versions
never select new quotas. No automatic source migration or new IR fields.

In the source-0.6 dependency derivation only, classify all provider-result envelope
references as metadata (capability, timeout clock and retry clock), retaining every
other reference as strong including overlap. Invoke the shared pure read-closure
helper with the remaining existing dependency-work allowance (100,000 total for
that pass), charging its returned work to that allowance. Preserve the old source
algorithm. Populate policy reads from complete closure, then use the existing
creation/data/node dependency derivation. Do not bypass semantic revalidation,
visibility or caller resource bytes. No new quota is justified without measurement.

Use a closed companion schema and exact new pair/core pin; libraries require
explicit semantics/core-0.1/core-0.2/core-0.3 bytes. CLI embeds the three verified
core artifacts only for the supported profile; no provider or host is inferred.
Source/package integrity uses the explicit 0.6 domain. A profile promotion changes
the selected core and therefore semantic revision even when other compatible
body fields are identical; do not claim 0.5/0.6 revisions equal.

Test full RP-01 promoted copies with coherently declared synthetic business and
clock recipients, native/Python grammar/schema/integrity, canonical IR/revision,
format/reparse and alias identities; actual static/control/protection success and
honest private-child invalidation refusal. The parent stays source 0.5/core 0.2
until this dependency is accepted. Retain old profile/pin/cycle/quota refusals.
Use actual protection/actor/effect/wait/timeout/retry reads in a lowerable fixture,
including metadata self/mutual references, and semantic refusal of stale derived
reads. Test unknown/mismatched pairs, missing/corrupt bases, private/wrong-kind
metadata, strong/legacy cycle behavior and dependency work exhaustion. Keep pure
graph-vector checks separate from actual compiler/checker evidence.

Run native/Wasm and all relevant Python checks, separately review completed diff,
inspect all comments/CodeRabbit summaries, record delegated acceptance and merge
only passing expected heads into the parent. Then explicitly adopt the source/core
and coherent artifact labels in RP-01, retain real stage results and freeze the
next genuine invalidation/lifetime/proof dependency. No complete scenario, engine,
external evidence or phase-exit claim follows from this source slice.
