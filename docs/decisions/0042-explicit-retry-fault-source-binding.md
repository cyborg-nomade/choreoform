<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0042: Bind provider retry fault sources explicitly

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

ADR-0041 decodes exact provider fault-source metadata without binding it to a
definition. RP-01's old core still admits arbitrary eligible fault names within
partial checks. Inferring a binding from registry bytes or spelling would change
frozen definitions and hide provider identity, evidence-class and live acceptance
requirements. Follow [frozen raw-core plan](../evaluation/0042-retry-fault-binding-plan.md),
`0c9479d`, supplemented by source-name accounting scope `222a5b7`.

## Decision

Publish separately pinned [core 0.4](../dialects/core-fault-source-v0.4.md), requiring
exact inherited 0.3/0.2/0.1 bytes. Its closed retry record adds required nullable
`faultSource`. Single attempts require empty faults/null bindings; multiple
attempts require nonempty eligible faults and explicit source/clock/basis.
Resolve every eligible name in the exact source. Unresolved eligible declarations
require the provider's declared stable-key support in this conservative subset;
definitive-no-effect still needs actual correlated evidence before any dispatch.

Every declared multi-attempt use, including ordinary-unvisited nodes, must be a
capability activity whose actual provider ID/revision equals the source provider.
Equal types/labels or reserialization of equivalent bytes cannot substitute for
that identity. Human contexts remain unproved. Unused policies can validate source
declarations without an actual use or activation claim. Source-provider protection
references contribute metadata edges and exact reads. Core 0.3/0.4 share finite
metadata closure semantics; strong/mixed-legacy cycles retain refusal.

Library `RetryUse::fault_source` exposes immutable revision/site/source/profile/
provider and eligible fault/observation/evidence declarations. None on legacy
cores means no inferred binding. Outstanding accepted-fault, actual provider
support, correlation, authority, stable-key/no-effect, delay/budget, cancellation/
closure and ownership premises remain explicit. All source emissions, including
noneligible names, enter conservative routing/unhandled-name accounting; no actual
fault emission is inferred. Deduplicate source pins, borrow old handler names and
share resource/name/graph work under the existing allowance.

## Options and consequences

| Option | Benefit | Cost or limitation | Decision |
| --- | --- | --- | --- |
| Explicit new retry field and exact actual provider binding | Checkable declared source/name/evidence relation | New core and subsequent authoring integration; narrow provider-only subset | Adopt |
| Infer source by matching names or available bytes | Smaller surface | Reinterprets old definitions and can bind the wrong provider | Reject |
| Trust a no-effect/safe Boolean | Easy acceptance | Supplies no provider/attempt/request evidence | Reject |
| Require source to prove actual live fault acceptance now | Stronger-looking report | Static declaration bytes cannot provide host observations | Reject |

Source 0.6 still selects core 0.3; it gains neither fields nor syntax. A separately
versioned authoring/companion profile and native disclosure must precede package
adoption. Old snapshots/provider schemas/IR wire structure/current source/IR and
reports remain unchanged. Raw-core reports are non-executable. The binding does
not supply fault-data ports, wait-result capture, continuation, engine-fault
catchability, full irreversible safety or actual P3/P4/P6 host enforcement.

[Evidence](../evaluation/0042-retry-fault-binding.md) records raw-IR/counterexample/
budget/native/Wasm/oracle checks and separate completed-diff automated review.
Acceptance is delegated, not individual owner or independent review. Require
final-head CI, every comment/CodeRabbit audit and signed expected-head merge into
draft #46. Then implement explicit authoring integration and remaining C/phase
dependencies. No complete scenario, parent or phase exit is granted.
