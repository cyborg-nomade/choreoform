<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Bounded near-English authoring frontend

Implementation slice of [ADR-0015](../../docs/decisions/0015-authoring-grammar-bindings.md),
profiles 0.2.0 / companion 0.1.0, 0.3.0 / companion 0.2.0 and 0.4.0 /
companion 0.3.0. This prototype produces **candidate IR**, not
semantic admission, execution or a production authoring editor.

## API boundaries

`check-reconciliation-availability` and its Candidate method check conditional
follow-up bodies with zero inherited values and fresh locals. They expose exact
declared entries and separate ordinary/handler/follow-up coverage, with bounded
non-executable output. Current permission, retained storage, actual activation,
handoff, correlation and settlement stay open. See
[ADR-0032](../../docs/decisions/0032-conditional-reconciliation-availability.md).

`check-reconciliation-links` and its Candidate method check surviving declaration
routes, with bounded non-executable output and explicit host/entry/lifetime
requirements. The old service example honestly refuses its missing follow-up.
The [0.4.0 profile](../../docs/authoring/profile-v0.4.md) can express the owned
child wait with a qualified outcome reference only at that scope's designated
reconciliation endpoint. Old source and general data/outcome visibility stay
unchanged. See [ADR-0030](../../docs/decisions/0030-reconciliation-declaration-links.md)
and [ADR-0031](../../docs/decisions/0031-qualified-wait-endpoints.md).

`check-exceptional-availability` and its Candidate method expose conditional
handler body checks, explicit required ancestor inputs and separate ordinary/
exceptional/routing coverage. The native metadata/report output remains bounded
to 1 MiB. No input values, actual activation/current access, fault snapshot or
execution permission is supplied. See
[ADR-0029](../../docs/decisions/0029-conditional-handler-availability.md).

`Candidate::check_fault_routes` and `check-fault-routes` expose conservative P4
active-context routing after prior timing/ordinary stages. Routing coverage is
separate from ordinary handler data coverage; a checked routing scope may still
have uncovered ordinary nodes. The native adapter bounds expanded route metadata
and final report output to 1 MiB (`report-budget`, no partial output). No fault
observation, invocation or execution authority is supplied. See
[ADR-0028](../../docs/decisions/0028-conservative-fault-routing.md).

The explicit [0.3.0 profile](../../docs/authoring/profile-v0.3.md) adds
`For retries use clock "Elapsed clock", basis "Audit basis".` for multiple
attempts. One attempt omits it. Grammar, companion, core pin and integrity domain
are versioned together; mixed packages refuse. `Syntax::profile()` identifies the
parsed version; fragment parsing keeps the legacy grammar default. The CLI embeds
both required base/new core artifacts for new packages; library callers supply
them explicitly. Basis/source protection dependencies are derived. See
[ADR-0027](../../docs/decisions/0027-explicit-retry-authoring.md) and
[evidence](../../docs/evaluation/0027-explicit-retry-authoring.md).

- `parse(bytes)` recognizes the checked-in EBNF, retaining original UTF-8 source,
  production tree and half-open byte spans. `parse_fragment` is a syntax-test API.
- `Ledger::decode(bytes)` checks strict JSON, the closed companion structure,
  scope tree, non-shadowing names, unique active/tombstoned IDs, aliases and slots.
  It does not compare the companion to source.
- `Package::parse(source, companion)` performs these independent checks.
  `integrity()` reports source, canonical-companion and framed-package hashes;
  integrity establishes neither agreement nor authority.
- `Package::lower(explicit_resources)` checks uniquely supplied contract bytes,
  exact declaration/symbol/slot inventories and supported references, then emits
  an immutable `Candidate`. It never allocates IDs or reconciles stale names.
- `Candidate::document()` / `into_document()` expose **unvalidated** revision-bearing
  IR through the existing strict transport/JCS/envelope implementation. This
  document may still be ill-typed or inadmissible. Do not execute it.
- `Candidate::check_static(resources)` explicitly runs the new
  [static suite stage](../semantic-check/README.md). Its report lists remaining
  obligations; it is not a fully admitted or executable definition.

The shared library has no IO, filesystem discovery or network access. Hosts
supply resources explicitly. Identity/name lookup is indexed; dependency/control
cycle checks use iterative topological traversal. The grammar is compiled once
from trusted checked-in EBNF, not caller input. Ordered-alternative recognition
agrees with the shipped catalogue; that is not a future-grammar ambiguity proof.

## Supported candidate-lowering slice

Scoped interfaces, data/protection, actors, explicit/generated expressions,
purpose/outcome aliases and conservative node reads are preserved. Supported
actions are human work, compute, invoke, decision, split/join, wait (including
supported instance-clock timers/calendars), repeat, fanout and finish. Closed policy payloads
are mapped for supported resource bindings; read
closure follows referenced policies and actor requirements. Integer/decimal
coefficients are exact signed-64 strings, never floats. Decimal value markers
cannot silently change the declared scale. Literal versus constructor ASTs and
unknown annotations survive lowering.

Selected checks cover local port ownership, direct children, exact invocation
interfaces, split/repeat/fanout interface restrictions, same-scope acyclic control
outside explicit repeat, policy variants, wait outcome ownership, paired joins
and population-wide wire collisions. Qualified descendant invalidation and
ancestor settlement references are narrow exceptions, not general child/sibling
visibility. Static ancestor binding does not establish live runtime ownership.

Accepted [ADR-0016](../../docs/decisions/0016-static-contract-checks.md) enables
closed named-type/provider resources and capability request candidate lowering,
using exact-byte readers and transitive dependencies. Nominal literal membership
is checked by the explicit static stage, not by merely lowering a candidate.
Only the exact implemented clock/construction profiles are recognized. Timer
typing/clock flow and host requirements are provided by ADR-0024/0025. Live
calendar activation and live retry-domain/fault/basis bindings remain unproved. Typed decoding does not
establish current clock facts, host trust or timer execution authority.
The historic JSON probe's registry/admission is unchanged.

Closed expression/type/policy-fact checks now exist in the explicit static stage.
Complete process/control/protection/authority judgments and executable semantic
admission remain separate work. Unicode confusable diagnostics are not implemented.
Syntax/reference errors usually have exact spans; companion
and some graph errors use `0..0` (no source location). Candidate span maps cover
declarations/generated records, not all alias occurrences. These profile requirements
remain open, not waived.

## Formatting and bounds

`syntax().source()` retains verbatim source. `formatted()` returns token-normalized
source and canonical companion: **it removes comments** and does not bind inventories.
Call `lower` explicitly for consistency. This is not a general IR exporter,
transactional identity editor, safe rename/move API or production formatter.
Tests exercise original-package format/reparse/lower cycles and declared alias edits.

Source/companion are each at most 1 MiB; source at most 4,096 tokens, 64 delimiter
levels and 100,000 recognizer pattern visits. A separate 512-call recognizer-depth
guard can refuse deeply nested legal fragments. Scope/type/expression and ledger/IR
depth are bounded; final envelopes retain existing 1 MiB / 64-level limits.
These are prototype budgets, not measured scalability.

## Native test adapter

With the pinned workspace Rust toolchain:

```sh
cargo run -p choreoform-authoring-frontend --locked -- lower \
  docs/authoring/examples/review.choreo \
  docs/authoring/examples/review.bindings.json
cargo run -p choreoform-authoring-frontend --locked -- format \
  docs/authoring/examples/terminal.choreo \
  docs/authoring/examples/terminal.bindings.json
cargo test -p choreoform-authoring-frontend --locked
cargo build -p choreoform-authoring-frontend --locked
.tools/ir-check/bin/python tools/check_authoring_frontend.py
```

`lower` prints candidate IR; `format` checks lowering then prints a JSON object
containing source/companion strings; `integrity` reports hashes only. The adapter
embeds the two accepted core snapshots for these synthetic examples. It reads
explicit paths and writes stdout only; errors never echo source values. A zero
exit code means only that command's selected checks passed. Wasm compilation covers
the library, not a browser adapter or execution. See [evidence and remaining work](../../docs/evaluation/0015-rust-authoring-frontend.md).

`check SOURCE COMPANION` reports `static-checks-passed`, `executable:false`, the
candidate revision and residual obligations. `lower`, `check` and `format` accept
explicit `--resource ID REVISION PATH` groups (at most 254 external files, 1 MiB
each; registry total 8 MiB including core snapshots). No path/ID discovery or
implicit rebind occurs. See the [service example](../../docs/authoring/examples/service.choreo),
[its pins](../../docs/authoring/examples/service.bindings.json) and the
[static-stage evidence](../../docs/evaluation/0016-static-contract-checks.md).

`check-control SOURCE COMPANION` (or `Candidate::check_control`) adds the proposed
conservative ordinary-flow/initialization proof after unchanged static checks.
It also accepts explicit resource groups. Reports say `control-checks-passed`,
`executable:false` and expose input assumptions, covered/uncovered node IDs and
residual obligations. Valid programs needing local initializer ordering or
parallel/exceptional proofs can refuse as `unproved`. See the
[branch example](../../docs/authoring/examples/control.choreo),
[ADR-0017](../../docs/decisions/0017-control-availability-checks.md) and
[review guide](../../docs/evaluation/0017-control-availability-checks.md).

`check-invalidation SOURCE COMPANION` / `Candidate::check_invalidation` adds
conservative ordinary invalidation declarations after earlier stages. It reports
checked/uncovered consequential sites and required site/cell links, refuses
missing coverage or unrepresentable context dependencies, and always remains
non-executable. It does not withdraw work, undo effects or prove concurrency safe.
See [ADR-0021](../../docs/decisions/0021-ordinary-invalidation.md).

`check-concurrency SOURCE COMPANION` / `Candidate::check_concurrency` checks the
conservative ordinary parallel independence subset in
[ADR-0022](../../docs/decisions/0022-ordinary-concurrency.md). It separates fresh
local storage from shared ancestor data, refuses unproved hazards/effect overlap
and lifetimes, and reports checked/uncovered forks. Earlier evidence and host/
domain obligations remain; the report is non-executable.

`check-timing SOURCE COMPANION` / `Candidate::check_timing` adds supported timer
typing/clock-origin checks and immutable timer uses with required host judgments.
It includes no observations/current facts or execution authority. Calendar-domain
and live retry bindings remain explicitly unproved. See
[ADR-0024](../../docs/decisions/0024-timer-contract-checks.md).

`check-protection SOURCE COMPANION` / `Candidate::check_protection` adds ordinary
control-origin protection constraints and explicit access obligations after the
earlier checks. The CLI supports the same explicit resource flags and returns
`protection-checks-passed`, `executable:false`, unchanged revision, control coverage,
input assumptions, transfer count, access obligations and residual host/validation
judgments. The [evidence](../../docs/evaluation/0019-protected-ordinary-transfers.md)
explains conservative refusals and the absence of actual access authorization.
