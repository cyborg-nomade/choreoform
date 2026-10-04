<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Bounded near-English authoring frontend

Implementation slice of [ADR-0015](../../docs/decisions/0015-authoring-grammar-bindings.md),
profile 0.2.0 / companion 0.1.0. This prototype produces **candidate IR**, not
semantic admission, execution or a production authoring editor.

## API boundaries

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

The shared library has no IO, filesystem discovery or network access. Hosts
supply resources explicitly. Identity/name lookup is indexed; dependency/control
cycle checks use iterative topological traversal. The grammar is compiled once
from trusted checked-in EBNF, not caller input. Ordered-alternative recognition
agrees with the shipped catalogue; that is not a future-grammar ambiguity proof.

## Supported candidate-lowering slice

Scoped interfaces, data/protection, actors, explicit/generated expressions,
purpose/outcome aliases and conservative node reads are preserved. Supported
actions are human work, compute, invoke, decision, split/join, untimed wait,
repeat, fanout and finish. Closed non-resource policy payloads are mapped; read
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

**Nonempty external-resource inventories refuse lowering**, even with matching
hashes. Provider/capability requests, named types and non-null clock/calendar
timers require supported readers and dependency evidence. Their syntax recognition
is not lowering support. The historic JSON probe's registry/admission is unchanged.

Complete expression/type checking, typed fact contexts, protection/authority and
executable semantic validation remain separate work. Unicode confusable diagnostics
are not implemented. Syntax/reference errors usually have exact spans; companion
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
