<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0036: Select finite metadata closure through an explicit source profile

**Status:** Accepted under Project Owner delegation; effective on parent/main
integration<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

ADR-0035/core 0.3 permits finite provider-recipient metadata closure with complete
reads and strong/legacy cycle refusal. Existing source profiles correctly select
older cores and cannot lower the required shared-result labels. The RP-01 package
needs explicit source support; relabeling old profiles would alter their meaning,
resource requirements and integrity without a version boundary.

## Decision

Adopt [source 0.6 / companion 0.5](../authoring/profile-v0.6.md), selecting the exact
core 0.3 artifact and requiring both inherited core snapshots. Reuse the unchanged
0.3 EBNF, source-0.5 quotas/actual-use endpoints and existing wire fields. Keep old
pairs/cores/hash domains and cycle refusals unchanged; unknown headers select no
new quotas. The new package hash domain is explicitly 0.6.0.

In this profile only, collect provider-result protection references separately
from strong references for capability/timeout/retry policies. Invoke the shared
pure read-closure helper with the remaining dependency-pass allowance; charge
its work against that same existing 100,000 limit. Preserve strong/metadata overlap
and all reads, then apply existing creation/data/node dependency derivation and
semantic revalidation. Do not infer resource bytes, visibility or host authority.

## Alternatives and consequences

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Closed new source/core pair | Explicit compatibility, resource and hash boundary | Another maintained provisional profile | Adopt |
| Rebind source 0.5 to core 0.3 | Fewer visible versions | Silently changes accepted pins/meaning and revision behavior | Reject |
| Accept manual derived-read annotations | Less compiler work | Burdens authors and tempts omitted dependencies | Reject |
| Duplicate the SCC algorithm in the frontend | Independent derivation | Divergence risk and extra graph maintenance | Reject; use independent oracle vectors and semantic revalidation instead |

A core promotion changes semantic revision; it is not just a spelling change.
Within-profile format/alias cycles still preserve meaning and IDs. Source syntax
and metadata labels remain provisional Candidate A and do not establish ordinary
authoring usability, protected views or an accessible editor.

Full promoted RP-01 copies pass static/control/protection checks but still refuse
private-child invalidation context. The parent retains its previous profile until
explicit adoption; this decision neither closes that proof gap nor upgrades any
scenario/phase gate or external host/user evidence. No engine is introduced.

Follow [frozen plan](../evaluation/0036-metadata-source-plan.md), `8c287e0`.
[Evidence](../evaluation/0036-metadata-source.md) records actual lowering/checks,
compatibility/pins/budgets and separate completed-diff automated self-review.
Accepted under ADR-0018 delegation, not independent or individual owner review.
Require passing final-head CI, comment audit and expected-head merge into the
still-unmerged parent.
