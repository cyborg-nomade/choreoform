#!/bin/sh
# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
set -eu
cd "$(dirname "$0")/../.."
if [ -x "$PWD/.tools/cargo/bin/cargo" ]; then
  export CARGO_HOME="$PWD/.tools/cargo" RUSTUP_HOME="$PWD/.tools/rustup"
  export PATH="$CARGO_HOME/bin:$PATH"
fi
test "$(wasm-bindgen --version)" = 'wasm-bindgen 0.2.127'
cargo build -p choreoform-authoring-frontend -p choreoform-authoring-prototype --locked
cargo build -p choreoform-authoring-prototype --lib --release --target wasm32-unknown-unknown --locked
mkdir -p tools/authoring-prototype/web/generated
wasm-bindgen --target web --out-dir tools/authoring-prototype/web/generated target/wasm32-unknown-unknown/release/choreoform_authoring_prototype.wasm
node --input-type=module - <<'JS'
import {readFileSync,writeFileSync} from 'node:fs';
const root='tools/authoring-prototype/';
const expected=JSON.parse(readFileSync(root+'imports-0.1.json','utf8'));
const module=new WebAssembly.Module(readFileSync(root+'web/generated/choreoform_authoring_prototype_bg.wasm'));
const actual=WebAssembly.Module.imports(module);
const normalized=a=>JSON.stringify(a.map(x=>[x.module,x.name,x.kind]).sort());
if(normalized(actual)!==normalized(expected))throw new Error('Unreviewed Wasm imports');
writeFileSync(root+'web/generated/imports.json',JSON.stringify(expected));
JS
