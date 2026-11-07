<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# General provider fault-source authoring: frozen integration plan

**Date:** 2026-11-07<br>
**Baseline:** `af62713386b4b5c6d437fa4ad87bee43f4e4b7f5`, draft #46 after PR #63; raw core 0.6 / IR 0.2<br>
**Authority:** ADR-0018; no parent/scenario/phase admission

Use a dedicated stacked dependency branch/PR and ADR-0049. Publish explicit source
0.9.0 / companion 0.8.0 selecting exact core 0.6 and all five inherited artifacts
on IR 0.2. Keep every previous grammar/schema/core/wire/profile/hash domain fixed.
Reuse existing fault/terminal source companion kinds; no new resource type or host
source inference. CLI embeds only known semantics/core artifacts; other exact
source/profile/provider/type bytes stay explicit.

Copy and version source-0.8 EBNF. Add optional work-policy sentence after authority:
`Recognise faults from "Source".` Resolve it as a normal visible provider-fault-source
resource, lowering outer faultSource. Single attempts may omit it and retain null /
empty inner retry fields. Multiple attempts require the sentence and retain clock /
basis binding; the source-0.9 retry sentence returns to the shorter `For retries use
clock "Clock", basis "Input".` The single explicit general source populates the inner
retry source too, guaranteeing exact agreement without repeating the source name.
Earlier 0.7/0.8 retry sentences keep their existing explicit source clause. Reject
missing/inconsistent/wrong-kind/unsupported actual use via lowering and semantic
revalidation. Readable names, not manual IDs/JSON, remain the authoring surface.

Derive outer-source provider protection references in metadata reads for every
work policy, even unused ones. Reuse one chosen source per policy so N>1 does not
walk/copy the same general/inner metadata twice. Preserve finite closure, actual-use
endpoints, normal visibility, source spans and all capture/retry/protection/lifetime
limits. Keep known program 16,384-token/1,000,000-pattern quotas, fragment/unknown
legacy bounds and existing 100,000 dependency allowance. Move unknown source
sentinels from 0.9 to genuinely unsupported 1.0; no quota increase.

All ten checking stages and advisory invalidation requirements disclose nonempty
`declaredFaultSourceUses` from retained static rows, covering declared unvisited
uses without claiming activation. Include revision/site/policy/source/profile /
provider and mandatory host requirements. Separately deduplicate all-name emission /
observation/evidence metadata once per exact source pin in
`declaredFaultSourceCatalogues`, preserving the distinction from retry eligibility
and actual emitted faults. Do not rerun admission for disclosure. Omit both fields
when empty, keeping earlier JSON unchanged. Reuse a single static-output helper
with shared row/copy budget and final combined 1 MiB bound before stdout; overlarge
reports fail report-budget without truncation or partial output. Preserve capture,
retry, historical, isolated and conditional owned-local views.

Test readable one/multiple attempts through actual frontend/all partial stages,
exact pins/bases/kinds/provider identity/human/unvisited/unused contexts, complete
source-only metadata reads and stale raw refusal, shorter new retry grammar versus
unchanged old grammar, nullable fields/IR selection, format/reparse/coordinated
aliases and independent package/semantic hashes. Check catalogue deduplication,
all eleven disclosures, distinct eligible names, sharing and overlarge output
from a bounded accepted source using long source/provider pins. Add Rust and
independent EBNF/schema/hash/native Python CI evidence. Keep checked-in full RP-01
package and guard/report bytes unchanged; use explicit promoted copies only.

Run native/Wasm/all Python/final-head CI, separate completed-diff automated review,
every unresolved thread/CodeRabbit summary and signed expected-head merge into
draft #46. Then freeze exact typed fault-history/source-control protection and
reviewed handler-local entry; current routing is still an abstract scope/name
judgment, not actual fault data. Permitted owned continuation/cutoff/all-settlement,
complete validator/benchmarks, general authoring/visual/protected editing and all
actual host/user/browser/independent/scenario/phase gates remain open.
