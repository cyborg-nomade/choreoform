<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Explicit retry authoring: frozen plan

**Date:** 2026-11-06<br>
**Baseline:** `0eb2b77f0258b8fbe8bc7ec1c23b61cae48a9e43` (PR #34)<br>
**Authority:** ADR-0018 delegation; ADR-0026 source follow-up.

Publish ADR-0027, source profile 0.3.0, a separate normative grammar and companion
0.2.0 schema. Preserve every old source/profile/grammar/schema/example byte.
The new profile inherits the old lexical/binding/identity rules and selects exact
core 0.2.0. Its optional work sentence is `For retries use clock "Elapsed clock",
basis "Audit basis".` before Timeout. Multiple attempts require this explicit
binding; one attempt has no sentence and lowers null clock/basis fields. Do not
default from timeout or scheduler, introduce free-form executable English or
force authors to write hashes in sentences.

Select bounded grammar caches by explicit source header; keep fragment API's
legacy default. Expose parsed profile and reject source/companion version
mismatches. Select the exact companion/core version pair and package integrity
domain, preserving old digests. Supply the new built-in contract in the CLI;
library callers still provide explicit immutable resources. Lower references and
derive basis/source-protection dependencies without changing ordinary work
identity, generated slots, symbols or tombstones. No automatic source migration
or arbitrary-IR exporter is introduced.

Extend the independent Python grammar/schema/integrity oracle with explicit
version selection, not a union grammar admitting new clauses in old files.
Test new one/multi-attempt work, mismatched versions and core pins, missing/extra
bindings, role/visibility errors, clock-source transitive dependencies, format/
reparse IR and report identity, independent timeout/retry clocks, null results,
old source/digest regressions and bounded parsing. Keep syntax/binding/static/
host stages distinct. Actual user comprehension and live retry execution are
not established by synthetic examples. Verify native/Wasm and relevant oracles,
self-review separately, record delegated acceptance, require final-head CI and
expected-head merge, then continue remaining Phase 1 judgments/evidence.
