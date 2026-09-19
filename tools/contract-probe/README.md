<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0013 contract evidence probe

Disposable, dependency-free Rust **subset**, not a parser, validator, complete
expression interpreter or process engine. It cannot grant execution admission.
Inputs are synthetic typed values; Boolean fields stand for previously checked
host facts, not a mechanism to authenticate or enforce those facts.

Implemented evidence: canonical signed-64 strings; exact decimal rescaling;
four AST forms (literal/read/add/if) with full-branch type/dependency preflight;
synthetic completion and retry eligibility; due/pause/calendar interval bounds;
unique live ownership and terminality predicates. Everything else in the
[contract proposal](../../docs/dialects/README.md) has paper evidence only.

Use the repository's pinned toolchain (`.tools/cargo`/`.tools/rustup` setup in
[the text guide](../../docs/text/README.md#running-and-testing)):

```sh
cargo test -p choreoform-contract-probe --locked --offline
cargo clippy -p choreoform-contract-probe --all-targets --locked --offline -- -D warnings
cargo clippy -p choreoform-contract-probe --lib --target wasm32-unknown-unknown --locked --offline -- -D warnings
```

The workspace CI also runs these unit tests and native lint. Wasm compilation
does not establish browser execution. No filesystem, network, clock or provider
IO is performed by the library. A caller supplies one immutable snapshot.

Important limits: snapshot value kinds stand in for declaration types, rather
than implementing the full type universe; access/protection checks are not
implemented. Timer checks assume a single previously validated clock domain and
non-overlapping authenticated pauses; a list of durations cannot prove either.
Ownership checks assume the live-owner set is genuine and receiver acceptance
has occurred. Retry checks assume prior attempt/fault/adapter facts are authentic.
The probe does not connect eligibility to an atomic commit or update history.
Do not pass untrusted runtime inputs to it as an enforcement API.

The AST preflight bounds depth (64) and node visits (10,000), then checks all
branches before execution. Timer slices are capped at 10,000 entries. These are
regression checks, not allocation measurements, hostile-input fuzzing or scale
evidence for large processes. No new third-party dependency is introduced.
