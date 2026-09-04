<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Rust portability probe

This is the bounded confirmation required by
[ADR-0010](../../docs/decisions/0010-initial-implementation-language.md), **not a
production IR validator, parser, interpreter or Studio**. A successful probe
does not establish that a process is semantically valid or safe to execute.
See the [evidence and limits](../../docs/evaluation/0010-rust-portability.md).

## Reproduce

Prerequisites: [rustup](https://rust-lang.org/tools/install/), Python 3.11+,
and `uv` for the independent Python scripts. Cargo reads the committed exact
toolchain from `rust-toolchain.toml`; dependencies are in `Cargo.lock`.
The browser glue generator must exactly match the pinned crate version:

```sh
cargo install wasm-bindgen-cli --version 0.2.127 --locked
sh tools/portability/prepare.sh
uv run tools/check_ir_fixtures.py
uv run tools/portability/check_report.py
python3 -m http.server 8765 --bind 127.0.0.1 --directory tools/portability
```

All commands start at the repository root. Installation and the first dependency
fetch need network access. With dependencies cached, set `CARGO_NET_OFFLINE=true`
and pass `--offline` to `uv run`. The optional local installation used for the
recorded run is ignored under `.tools/`; `prepare.sh` selects it automatically.
For individual Cargo commands with that installation:

```sh
export CARGO_HOME="$PWD/.tools/cargo"
export RUSTUP_HOME="$PWD/.tools/rustup"
export PATH="$CARGO_HOME/bin:$PATH"
```

Open `http://127.0.0.1:8765/` in an actual browser, then click **Run browser
parity checks**. The page must report PASS, exact report parity, repeated-run
parity, byte-boundary parity, and the allowlisted Wasm import. Retain its visible
evidence summary when reviewing changes. Stop the server with Ctrl-C afterward.
If port 8765 is occupied, choose another unused loopback port. The server exposes
only this probe directory, not the repository root or local tool/cache files.

Reload the page after rebuilding: the browser module is cached in memory. A
Wasm compilation, native test run, Node run, or green CI **does not replace this
browser step**. The added CI job covers native tests, compile-fail checks, Wasm
compilation/lints, and the independent Python oracle; browser evidence is manual.

The generated directory contains the JS/Wasm bindings, full native report and
Cargo metadata; these are reproducible outputs, not committed source. The
report is versioned `0.1.0` and deliberately contains full canonical strings,
documents and error categories so comparison is stronger than a count/hash only.
Generated binaries embed the existing MPL fixture and CC-BY contract snapshots
as test data. They are not a release package; any future distribution must retain
the corresponding source, license and attribution notices plus dependency notices.

## Layout and boundaries

- `crates/ir-probe-core`: safe project-owned decoder, restricted-integer JCS,
  SHA-256, supplied-resource checks, and a partial typed graph projection.
- `src/lib.rs`: shared evidence cases, explicit test resources and Wasm exports.
  `run_suite()` returns a JSON report; `inspect_bytes()` receives raw bytes and
  uses the test adapter's two pinned contracts. Rust memory layout is not an ABI.
- `src/main.rs`: native test adapter. `suite [path]` writes the report and exits
  unsuccessfully on failed expectations. `inspect` reads bounded standard input,
  returns canonical semantic bytes without a newline on success, and writes a
  probe error category to stderr with nonzero exit status on failure.
- `browser.mjs`: loads only local test artifacts, audits imports, calls the same
  Wasm core, checks the complete report twice and tests six JS/Wasm byte transfers.
  There is no second JavaScript implementation of validation or hashing rules.
- `check_report.py`: independently checks canonical bytes with `rfc8785`, hashes
  with `hashlib`, frozen fixtures with the existing checker, protection retention,
  annotation/map invariance, array significance and variant coverage.

Example native inspection (using Cargo on PATH):

```sh
cargo run -p choreoform-portability --locked -- inspect < examples/ir/01-reimbursement.json
```

`semantic_bytes` is explicitly transport/envelope-only. `inspect` additionally
checks the supplied contracts and selected structure; neither API grants
permission to rewrite, plan, execute or enforce the opaque dialect payloads.
`Inspected.document` retains all decoded fields, while `graph` is only a partial
typed view. Never serialize that view as a replacement definition.

The error-category names are probe-local, not the future stable diagnostics
contract. The adapter must bound inputs before a Wasm transfer; the core's
1 MiB/depth-64 checks happen after the binding has copied the input into Wasm.
This test page supplies only bounded fixtures and fixed negative byte arrays.
