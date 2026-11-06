<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Value-expression conformance and validator audit: frozen plan

**Date:** 2026-11-06<br>
**Baseline:** `ac6edfa6402c67575de4740c83017e2d97d836bb` (PR #36)<br>
**Authority:** ADR-0018 delegation; ADR-0013 V1/V2/E1–E3 and Phase 1 conformance.

Consolidate current static/ordinary/lifecycle/host and Phase 1 evidence in one
coverage/gap audit, referencing accepted contracts and executable checks. Preserve
the distinction between missing static judgments, conservative unsupported cases,
required current host facts, full benchmark/round-trip/editor evidence and actual
external studies. Do not relabel earlier partial reports as complete admission
or create a process engine before Phase 2. Record the next verified dependency.

Begin a reusable data-driven value-expression conformance corpus. Run vectors
through the production static checker and existing private nominal-invariant
evaluator, checking exact inferred types, canonical values and static/evaluation
fault codes. Keep it a test harness with explicit immutable resource bytes and
contract-internal value parameters; introduce no public process evaluator,
ambient reads/facts, host trust booleans, wire/profile change or new dependencies.

Cover every operator executable in a nominal invariant: scalar/container/option/
variant/named operations, exact signed arithmetic/decimal rescaling, branch and
Boolean short-circuit behavior, full static checking of unselected branches,
overflow/inexact/bounds/absent/tag/invariant faults and malformed forms. Process
read and policy-fact operations receive explicit invariant-context refusals;
their snapshot availability/permission/current-fact semantics remain separately
tracked. Use expected results as assertions against the actual implementation,
not generated observations presented as independently established semantics.

Validate corpus identities, exact field shapes, case coverage, resource pins and
documentation with an independent Python integrity/catalogue oracle; do not claim
it independently proves evaluation semantics. Keep existing fixtures/snapshots
unchanged. If vectors expose valid bugs, fix narrowly with regressions; record
findings and rejected alternatives. Run relevant native/Wasm/artifact/stage checks,
self-review separately, require final-head CI and expected-head merge. Continue
remaining Phase 1 obligations afterward; no aggregate gate closure is inferred.
