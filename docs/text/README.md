<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Text profile 0.1.0 — initial parser prototype

**Status:** Proposed in [ADR-0011](../decisions/0011-initial-textual-grammar.md).
This is experimental syntax, not a released language or executable format.

## Reading it

A file starts with `choreoform "0.1.0";`. Metadata uses `name = JSON;`.
Declarations are grouped in explicit sections, with stable IDs as labels:

```text
nodes {
  complete = {
    "kind": "finish",
    "scope": "rootScope",
    "outcomes": {},
    "reads": {},
    "writes": {},
    "outcome": "done"
  };
}
```

This fragment is not a complete definition: every metadata item and section
below is required. References inside records are exact JSON strings, not
variable lookups or names inferred from surrounding text. Section order and
record order have no meaning. A record is still governed by the
[IR specification](../ir/definition-v0.1.md); the parser does not implement that
specification's complete validation rules.

Read complete, synthetic benchmark excerpts here:

- [Reimbursement](../../examples/text/01-reimbursement.choreo): human review,
  payment, and a wait for reconciliation.
- [Order](../../examples/text/03-order.choreo): parallel reservations and join.
- [Incident](../../examples/text/08-incident.choreo): keyed fan-out and sealing.

Their omissions and illustrative contract limitations are the same as the
[original IR examples](../ir/examples.md). No process is run by this prototype.

## Grammar

The following EBNF describes the outer grammar; quoted literals are exact.
`json` is one strict JSON value under the IR integer/Unicode rules, not arbitrary
JavaScript. `json-object` restricts that value to an object.

```ebnf
document = "choreoform", json-version, ";", { item } ;
json-version = JSON string whose decoded value is "0.1.0" ;
item = metadata | section ;
metadata = metadata-name, "=", json, ";" ;
metadata-name = "id" | "semantics" | "dialects" | "root" | "annotations" ;
section = section-name, "{", { record }, "}" ;
section-name = "scopes" | "data" | "expressions" | "actors"
             | "capabilities" | "policies" | "nodes" | "flows" ;
record = identifier, "=", json-object, ";" ;
identifier = ASCII-letter, { ASCII-letter | digit | "_" | "-" } ;
```

Additional well-formedness rules:

- Each of the five metadata items and eight sections occurs exactly once.
  Empty sections are written explicitly, e.g. `capabilities {}`. No defaults.
- `id` and `root` are nonempty JSON strings. The other metadata values are
  objects. Further ID/reference/contract shape checks belong to validation.
- Declaration identifiers are 1–64 ASCII bytes and unique across all sections,
  including different kinds. Keywords may be declaration IDs: position determines
  their role. Case is significant. No quoted or Unicode declaration IDs.
- Outer whitespace is space, tab, CR, or LF. `//` comments end at LF or EOF and
  are allowed between outer tokens, including before payloads. JSON interiors
  and whitespace between a payload and its semicolon follow strict JSON rules:
  no comments there. Block comments and trailing commas are unsupported.
- A section's closing brace has no semicolon. Every record and metadata value
  has one. Semicolons, braces, and comment markers inside JSON strings are data.
- The header keyword is one identifier token; whitespace before its JSON string
  is optional. Escaped spellings of the version JSON string decode normally.
- Input is UTF-8 with no BOM. Duplicate JSON keys (including escaped equivalent
  names), unpaired surrogates, fractional/exponent number tokens, and integers
  outside ±9007199254740991 are rejected at every payload depth. Integer `-0`
  follows the accepted IR canonicalization to `0`; Unicode is not normalized.
- Input is at most 1 MiB. The scanner refuses more than 64 simultaneously open
  JSON containers, then the existing strict JSON depth check applies. The whole
  lowered IR must also pass its 1 MiB/depth transport limits: accepted individual
  payloads do not guarantee successful lowering once wrapped in the envelope.
  No unbounded recursive outer grammar or input-triggered IO is used.

All input must be consumed. The first error stops parsing; no repaired or
partial tree is returned as success. Resource refusal never drops a field.

## Lowering and identity

The four body metadata values and eight section maps form the complete IR body.
The fifth metadata value becomes top-level `annotations`, without discarding
unknown keys. No value is inferred from a label, comment, missing section, or
declaration order. Add the fixed `format: choreoform-ir`, `version: 0.1.0`, and
`kind: definition` envelope and calculate `revision` using ADR-0009's canonical
projection. Text version and IR version are separate contracts despite sharing
the initial spelling; unsupported text versions are rejected exactly.

The source does not carry a user-maintained semantic checksum. Lowering computes
it. Immutable semantic/dialect resource digests inside the body remain explicit,
unmodified values. Lowering does not resolve them or access the network.

Successful syntax and hashing produce an **unvalidated IR candidate**. A node
with missing kind, dangling reference, or opaque policy may parse and lower.
Do not call this semantic validation, accepted IR, or executable output. The
subsequent validator must reject invalid shapes, unknown required contracts,
incorrect scope/dependency rules, and unenforceable policies before planning.

## Source tree and export boundaries

`parse(&[u8])` returns an immutable borrowed `Syntax` with the exact source,
source-order items, decoded payload values, and item/declaration spans. Spans
are half-open byte offsets into UTF-8, not characters or display columns.
`source()` includes all comments and whitespace verbatim. Payload diagnostics
cover the whole payload; missing-token diagnostics can be zero-width.
Diagnostic wording and codes are prototype-local, not the later stable API.

`binding()` returns the SHA-256 of exact source bytes and the lowered semantic
revision. Store both with extracted spans; a comment-only edit changes the
source digest even though it leaves the semantic revision unchanged. No source
map is silently inserted into annotations, and source comments never become the
sole copy of required human instructions.

`export(IR bytes)` checks strict transport, envelope, revision, body field set,
and exact representability. It emits normalized text in a fixed section order
with deterministic record ordering, then checks that parsing/lowering it returns
the entire input IR value. It preserves ordered arrays and all annotations.
It rejects information loss and revision mismatch. This is a normalizing
**IR-only export**, not a comment-preserving formatter: it cannot reconstruct
comments that were only in a previous source artifact. Repeated exports are
stable; retaining `Syntax::source()` preserves original source separately.

## Running and testing

Use the pinned toolchain as in the [Rust probe](../../tools/portability/README.md).
For a repository-local installation, first set:

```sh
export CARGO_HOME="$PWD/.tools/cargo"
export RUSTUP_HOME="$PWD/.tools/rustup"
export PATH="$CARGO_HOME/bin:$PATH"
```

From the repository root:

```sh
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy -p choreoform-text-prototype --lib --target wasm32-unknown-unknown --locked -- -D warnings
cargo run -p choreoform-text-prototype --locked -- lower < examples/text/01-reimbursement.choreo
cargo run -p choreoform-text-prototype --locked -- export < examples/ir/01-reimbursement.json
cargo build -p choreoform-text-prototype --locked
.tools/ir-check/bin/python tools/check_text_prototype.py
```

The CLI reads bounded stdin and writes only stdout; errors go to stderr with
nonzero status, including output/flush failures. It does not overwrite files.
The wasm check compiles the IO-free library; no browser execution is claimed.
See [the evaluation](../evaluation/0011-textual-grammar.md) for exact evidence
and open gates. Visual notation and full round-trip editing are later items.

The Python oracle uses the existing hash-locked portability environment (see
its README to provision `.tools/ir-check`). It independently compares the CLI's
lowered outputs with the frozen IR, structural checker, and RFC 8785 hash, then
exercises repeat export and refusal behavior. It is test tooling, not a second
product parser or a complete semantic validator.
