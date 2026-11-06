<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Value-expression conformance evidence

**Date:** 2026-11-06<br>
**Plan:** [0028](0028-value-conformance-plan.md), `a76c655`<br>
**Scope:** ADR-0013 V1/V2/E1–E3 evidence under ADR-0018 delegation.

The [corpus and run instructions](../conformance/README.md) supply 69 explicit
expected-result vectors: 42 exact values/types, 16 static refusals and 11 selected
evaluation faults. Three Rust test groups execute the actual production static
checker/private invariant evaluator against fresh immutable registries. The data
was authored from contract rules, not generated from successful implementation
outputs. The existing integer-encoding diagnostic spelling was aligned during
fixture verification; no product behavior was relaxed and no evaluator bug has
been found in this slice.

All 29 E1 operators are present in the catalogue. Read/fact cases refuse in the
invariant context; this is not process snapshot/policy-fact execution. The other
operators cover canonical scalars, containers, options, variants, nominal
wrap/erasure, exact arithmetic/rescaling, evaluation order and faults. This is
case coverage, not exhaustive input coverage or proof of full semantic admission.
The harness does not add a public evaluator, host callback, authority flag,
dependency, contract pin or new wire version.

The Python catalogue oracle checks exact publication/resource pins, strict case
fields/IDs/phase counts, normative operator accountability and licenses/links.
It independently checks catalogue integrity, not evaluation semantics. The
[gap audit](0028-validator-gap-audit.md) records remaining validator/lifecycle,
benchmark, text/visual/editing and actual-user obligations and the next dependency.

## Verification

Passed 204 Rust tests plus five compile-fail doctests; format/native lint,
semantic Wasm lint and release frontend Wasm build. All relevant artifact,
authoring/profile/frontend and static/control/protection/lifecycle/invalidation/
concurrency/clock/timing/calendar/retry/settlement Python oracles passed, including
three new catalogue groups. Local logs are temporary:
`/private/tmp/value-conformance-rust.log` and
`/private/tmp/value-conformance-oracles.log`.

Separate completed-diff automated self-review checked exact resource provenance,
strict case fields and phase separation, hand-authored expected values, all
normative operators, selected/unselected faults, nominal/type distinctions and
sanitized diagnostics. The audit was checked against current report boundaries
and the accepted gate/exit wording; marking the conformance suite started does
not close its aggregate completion gate. No production evaluator/API behavior
changes, dependencies or old artifact/fixture changes were introduced. No
actionable finding remains. Final-head CI/expected-head merge remain prerequisites.
No independent reviewer or actual host, browser, process or participant
observations are claimed.
