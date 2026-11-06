<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Qualified reconciliation endpoint evidence

**Date:** 2026-11-06<br>
**Plan:** [0031](0031-qualified-endpoint-plan.md), `4f7b2e7`<br>
**Decision:** [ADR-0031](../decisions/0031-qualified-wait-endpoints.md)

Source 0.4.0 / companion 0.3.0 supplies a narrowly qualified wait-outcome owner
binding using the unchanged 0.3 EBNF and core 0.2.0 bytes. Ordinary lexical lookup
takes precedence; only missing explicit endpoint owners may fall back to the
actual designated direct child. Actual wait kind and exact policy are checked
after steps. Trees are borrowed, ledger lookup indexed, and original spans kept.
General expression/data/type/outcome resolution is unchanged. Old profiles still
refuse the child endpoint; old grammar/schema/example/snapshot bytes stay intact.

The new [owned service example](../authoring/examples/reconciliation.choreo)
lowers and passes the reconciliation declaration stage without a dummy root proxy.
Its follow-up wait remains ordinary-data-uncovered. No actual snapshot, current
authority, accepted handoff, correlation, multiple-obligation progress or terminal
settlement is inferred. The example remains synthetic and not a full benchmark.

Six Rust groups cover exact links/host requirements, formatting and coordinated
alias identity, declaration order, boundary/kind/policy/qualification errors with
spans, private data/general outcome refusal, shared observation/timer owner and
strict source/companion/core/resource requirements. Existing bounded-parser tests
also include profile 0.4. Five Python groups cover grammar/schema/hash/revision/
report cycles, unchanged IR/timing on 0.3 promotion with one/multiple attempts and
timeout, non-designated/nested boundary refusals, strict versions/pins, and docs.
The Python oracle checks independent representations/integrity; it is not a
second semantic implementation or actual host/browser/user evidence.

Passed 235 Rust tests plus five compile-fail doctests, native format/lint,
frontend Wasm lint/release build, and all relevant artifact/source/stage Python
oracles including the five new groups. Logs: `/private/tmp/endpoint-rust.log`,
`/private/tmp/endpoint-clippy.log`, `/private/tmp/endpoint-oracles.log`.

Separate completed-diff automated self-review checked strict version/core pairing,
fallback precedence, actual closure scope and child ownership, post-step exact
wait/policy validation, private references, deferred declaration order, immutable
IDs/hash domains, bounded lookup/output and honest uncovered contexts. No remaining
actionable finding was identified. ADR-0031 is accepted under owner delegation;
require passing final-head CI and expected-head merge. No independent review or
individual owner review is claimed.

Continue reconciliation entry/protection/invalidation/lifetime, exceptional fault
control and host-label boundaries, full validator and benchmark/conformance
evidence, general text/visual/protected accessible editing and actual users.
This closes only ADR-0030's source endpoint binding dependency, not phase gates.
