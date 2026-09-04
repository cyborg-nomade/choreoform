#!/bin/sh
# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
set -eu
cd "$(dirname "$0")/../.."
# Optional repository-local rustup installation; otherwise use the user's PATH.
if [ -x "$PWD/.tools/cargo/bin/cargo" ]; then
    export CARGO_HOME="$PWD/.tools/cargo"
    export RUSTUP_HOME="$PWD/.tools/rustup"
    export PATH="$CARGO_HOME/bin:$PATH"
fi
test "$(wasm-bindgen --version)" = 'wasm-bindgen 0.2.127'
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy -p choreoform-portability --lib --target wasm32-unknown-unknown --locked -- -D warnings
cargo build -p choreoform-portability --lib --release --target wasm32-unknown-unknown --locked
mkdir -p tools/portability/generated
wasm-bindgen --target web --out-dir tools/portability/generated target/wasm32-unknown-unknown/release/choreoform_portability.wasm
cargo run -p choreoform-portability --locked -- suite tools/portability/generated/native.json
cargo metadata --locked --format-version 1 > tools/portability/generated/dependencies.json
printf '%s\n' 'Ready: serve tools/portability on loopback and run the browser page.'
