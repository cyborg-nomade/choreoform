<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Explicit retry fault-source authoring: frozen dependency plan

**Date:** 2026-11-06<br>
**Baseline:** `f6cffd2a88635e84b39b49261d984be11e5ac263`, draft #46 after #55; ADR-0042 raw core 0.4<br>
**Authority:** ADR-0018; no parent/scenario/phase admission

Use a dedicated stacked dependency branch/PR. Publish ADR-0043, source 0.7.0 /
companion 0.6.0 selecting exact core 0.4 plus all inherited 0.3/0.2/0.1 artifacts.
Copy and version the existing EBNF, changing only its retry binding to require
`For retries use clock "Clock", basis "Input", fault source "Source".` for
multiple attempts. Single attempts omit the sentence and lower null clock/basis/
faultSource; core revalidation requires empty faults. Add a closed companion
resource kind `provider-fault-source` only in this version. Exact source/profile/
provider/type bytes must be explicitly supplied; CLI embeds only known core bases.
Old grammars, schemas, source/core pairs, package hash domains, quotas and body
fields remain unchanged; move formerly unknown 0.7 negative sentinels to 0.8.

Retain source-0.5 whole-program quotas/actual-use endpoints and source-0.6 finite
metadata closure. Add source-provider result-protection references to derived
metadata reads, with existing visibility and the shared 100,000 dependency-work
allowance. Preserve all strong references and complete reads, semantic revalidation,
source diagnostics and resource failure behavior. No manual derived-read annotations,
JSON editing, generated identity bookkeeping or implicit resource inference is a
proposed ordinary-author workflow. Tool-managed authoring/editing remains open.

Disclose raw core-0.4 typed retry fault bindings through all five native stages
from timing through reconciliation availability. Include exact revision/site/source/
profile/provider, eligible declaration classes/observations and explicit host
requirements. Use a shared helper and the existing final 1 MiB bounded writer;
charge/limit row accumulation before stdout, never truncate. Legacy reports must
remain byte-structure compatible when no binding exists. The new fields prove
only static relationships, never accepted observations/faults or retry authority.

Test actual promoted RP-01 copies without changing checked-in package bytes:
new clause/kind/version/strict pair, all exact pins/bases/corruptions, missing/extra/
wrong-kind/unknown eligible sources, single/multiple attempts, human and exact
provider mismatches including unvisited uses, source-only metadata reads and stale
read rejection, all five bounded disclosures, format/reparse/coordinated alias
identity and independent package/semantic digests. Preserve legacy full-package
probes and genuinely unknown headers/quota refusals. Add independent Python
EBNF/schema/hash/native evidence and CI coverage. Keep source resource metadata
separate from actual P3/P4/P6 acceptance/permission/correlation/settlement.

Run native/Wasm/all Python checks and final-head CI; completed-diff automated
self-review, all comments/CodeRabbit summaries and signed expected-head merge.
Then freeze a separate RP-01 adoption plan, add the exact synthetic payment source,
regenerate all partial reports and repin existing guard vectors without changing
expected values. Record actual next lifecycle/data/continuation gap. Parent #46
remains draft/incomplete and all scenario/phase gates remain open.
