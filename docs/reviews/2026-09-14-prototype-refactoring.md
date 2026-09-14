<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Bounded prototype refactoring — implementation and evidence

Status: implemented for review; not permission to merge or begin the next
Roadmap deliverable. The user approved the first two recommended slices in the
[pre-change critique](2026-09-11-code-critique.md). Baseline:
`167633e3bc20807cc2dfcff3f5a1e98161bfce94` (merged PR #14).

## Scope and changes

| Finding | Implementation | Boundary retained |
| --- | --- | --- |
| F1 | Memoize rooted scope paths in Rust and Python; resolve each parent once across successful forest checking | Missing parents, disconnected roots, and cycles still fail; no arbitrary graph-depth cap |
| F2 | Shared byte-budget writer stops pretty export during generation | Exact 1 MiB output limit; no host output until complete export and round-trip guard succeed |
| F3 | Python cycle checking uses an explicit DFS stack | Independent oracle remains Python; graph depth does not consume Python call-stack depth |
| F4 | Borrow canonical projection fields; write canonical JSON into one buffer; admit constructed envelopes without serialize/reparse | Strict byte-input decoder, whole-envelope limits, integer rules, UTF-16 key ordering, and revision bytes retained |
| F5 | Typed flat metadata/section items; one record list, borrowed source IDs; consuming and combined lowering APIs | Exact source, source-order spans, opaque payloads, stable IDs, and both identities retained |
| F7 (relevant subset) | Edge-count, budget, envelope, Unicode truncation, golden export, combined-lowering and deep graph regressions | No time-based CI assertions or broader conformance claim |

No new dependencies, language syntax, serialization version, semantic contract,
or accepted ADR decision changed. This implements the approved internal cleanup,
not a new durable architecture choice. F6 (the product validator and dialect
registry) remains deferred. The executable-IR conditional gates, benchmark
completion, and near-plain-English authoring deliverable remain open; no
Roadmap checkbox was changed.

## API and safety review notes

- `DefinitionEnvelope` owns a private document and certifies only transport and
  envelope admission. `decode` does **not** verify the supplied revision or
  validate executable semantics. `from_parts` checks constructed values and
  computes a revision. Immutable access preserves admission; extracting and
  mutating the returned `Value` carries no continuing guarantee.
- Constructed values already cannot contain duplicate object keys or invalid
  Rust strings. Byte input still passes the strict lexical decoder, including
  escaped duplicate keys and exponent/fraction rejection. In-memory admission
  checks bounded depth and integer validity before counting compact serialized
  bytes across the **whole** envelope, including annotations and escaping.
- On multiply-invalid in-memory inputs, depth/number errors can now precede size
  errors. Export checks section shapes before writing. Prototype diagnostic
  precedence is not a stable contract; accepted valid inputs are unchanged.
- `Item` is now an enum with `MetadataKind` and `SectionKind`; consumers of the
  prototype's old public item fields must migrate to variants/accessors.
  `Syntax::into_lowered()` moves payloads; `lower_with_binding()` clones once
  when retaining the syntax is useful. Both return the document, identities,
  borrowed source, and spans. Calling separate convenience methods still
  repeats work. The CLI uses the consuming path.
- Export still reparses/lowers its generated source and compares the **entire**
  IR value. No annotations, ordered arrays, or unknown payload fields are dropped.
- A capped writer bounds forwarded bytes, not vector capacity or total memory.
  Canonicalization still allocates its final string and per-object borrowed-key
  sorting lists. Visibility checks are not a full ancestor-index optimization.
  These are deliberate limits on this refactoring's scope.

## Reproduction and verification

Use the repository-local pinned Rust environment from the
[text guide](../text/README.md#running-and-testing) and existing hash-locked
`.tools/ir-check` Python environment. Commands from the repository root:

```sh
cargo fmt --all -- --check
cargo test --workspace --locked --offline
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo clippy -p choreoform-text-prototype --lib --target wasm32-unknown-unknown --locked --offline -- -D warnings
cargo build -p choreoform-text-prototype --locked --offline
.tools/ir-check/bin/python tools/check_ir_fixtures.py
.tools/ir-check/bin/python tools/check_text_prototype.py
cargo build --release -p choreoform-text-prototype -p choreoform-portability --locked --offline
cargo run -p choreoform-text-prototype --example profile --release --locked --offline
.tools/ir-check/bin/python tools/profile_prototypes.py
```

Regression coverage includes exact-limit/one-byte-over output and constructed
envelopes, escaped-byte accounting, short writes and sink errors, unsafe numbers,
deep values, lexical rejection, frozen normalized export text, comment/source
identity, Unicode truncation, and consuming/combined lowering equivalence.
The scope unit test counts exactly N−1 resolved parents even when starting at
the deepest scope (128/512/2048), plus invalid deep paths. Python tests cover a
5,000-node adjacency chain/back-edge and a full 1,100-node wire document.

Final local results: 32 Rust unit/integration tests and three compile-fail doc
tests passed; formatting and both Clippy commands passed; all 16 Python wire
tests passed. The text oracle passed three complete IR/schema/JCS comparisons,
three stable export cycles, and seven CLI refusals. The portability preparation
script also passed its native and wasm build/lint checks.

The existing browser probe was rebuilt with `tools/portability/prepare.sh` and
run in the in-app browser: **89 cases, exact native/browser parity, repeatable,
six byte-boundary cases passed**. It exercises the changed shared core, not
browser execution of the text parser; the latter has wasm compile/lint evidence
only. Browser: Chrome/152.0.0.0, macOS user-agent compatibility platform
`Macintosh; Intel Mac OS X 10_15_7`. Only wasm import:
`./choreoform_portability_bg.js::__wbindgen_init_externref_table` (function).

- Wasm SHA-256: `9bf58bc46672dd6139c7934d11ea4ccada7f1866364b1488cd892908a0ac6e7b`
- Matching native/browser report SHA-256: `0a7f12f49ebda3ef66effcd54a26e5a9e942d0e72583b6abe50ab4ce6044755c`

The report hash and three frozen semantic revisions are unchanged. This is
portability and regression evidence, not completion of Phase 1's full benchmark
or semantic-validation gates.

## Informational timings

Local macOS 26.6.2 arm64, pinned Rust 1.98.1, optimized builds. Scope numbers
include process startup and IO: one warm-up, median of three samples. Baseline
is the critique's run; after is this refactor on the same host. These are small,
non-interleaved diagnostic samples, not statistically controlled benchmarks.

| Scopes | Star before / after (ms) | Chain before / after (ms) |
| --- | ---: | ---: |
| 128 | 3.089 / 7.796 | 4.468 / 4.004 |
| 512 | 6.097 / 6.312 | 33.977 / 5.569 |
| 1024 | 11.061 / 7.988 | 123.594 / 8.020 |
| 2048 | 20.072 / 13.874 | 504.202 / 14.069 |

The small star cases show noise/regression, so no universal speedup is claimed.
The parent-edge count is the deterministic evidence for removal of repeated
chain traversal. Tree-map/set operations still cost logarithmic lookup time.

Library stages: one warm-up, five batches of 50 operations, median ns/operation;
same harness for the before/after common stages. The fixture has 4,930 source
bytes; the semantic-array variant has 94,953 and adds 10,000 zeros to an opaque
policy payload. This synthetic payload is not an accepted executable dialect.

| Stage | Fixture before / after (ns) | Array before / after (ns) |
| --- | ---: | ---: |
| Parse | 63,078 / 57,798 | 179,257 / 179,511 |
| Lower | 155,525 / 73,580 | 530,324 / 342,926 |
| Separate lower + binding | 221,287 / 101,704 | 1,200,389 / 684,244 |
| Export (guard retained) | 226,292 / 102,281 | 1,361,808 / 816,166 |
| Combined lowering (new API) | — / 37,204 | — / 345,019 |
| Parse + consuming lowering (new API) | — / 50,180 | — / 493,342 |

The compatibility `lower()` now also computes the binding internally; new API
rows are alternatives, not stages to add to the other rows. Batch order/cache
effects can affect comparisons between rows.

The 303,929-byte annotation-expansion workload is refused at the source budget
with the input IR span. After median: 10.175 ms (three samples); baseline was a
single 17.593 ms observation, so this is not a reliable speedup estimate. No RSS
or allocation count was measured. Budget tests, rather than timing, establish
that an over-budget chunk is refused before forwarding.
