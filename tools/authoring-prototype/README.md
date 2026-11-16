<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Experimental paired authoring

This local experiment implements [ADR-0096](../../docs/decisions/0096-experimental-paired-prototype.md).
One Rust authoring model drives native and browser text/diagram projections. It
supports format, source-only definition/input rename, placement/collapse and complete
snapshot export. Text remains read-only outside those managed operations. This is
not a general editor, executable validator, runtime, accepted notation or user study.

From the repository root, with the pinned toolchain, locked dependencies and exact
wasm-bindgen0.2.127 generator installed:

```sh
sh tools/authoring-prototype/prepare.sh
.tools/ir-check/bin/python tools/authoring-prototype/server.py
```

Open the launcher’s editor/viewer/exporter link in a local browser. Tokens are
private, ephemeral session capabilities; do not publish links or launcher output.
The browser removes the fragment and keeps its capability in memory. Reloading
requires reopening the launcher link. Stop the launcher with Ctrl-C. It binds only
127.0.0.1 and discards temporary edits at shutdown. Original fixtures are never saved.
The launcher captures three fixed approved packages before opening its socket.

Select a process, find items by readable name/kind, select a card/list item, follow
owner/scope links and inspect its complete source clause. Fit overview and readable
scrolling show the same coordinates. Both lists and diagram pages explicitly count
items; off-page routes remain in the full explorer. All keyboard operations have
labeled controls. Source selection jumps back to its native declaration. A resource
has companion provenance, not an invented source span. Advanced IR pages retain every
accepted field and opaque annotation, with bounded rendered text for accessibility. No raw hash/ID/JSON editing is required.

Every load, change and prepared export compares the real Wasm result with the native
adapter’s exact bytes at the same authoring base. “Verify browser parity” repeats
that comparison and a private malformed-input refusal. Export prepares a complete
canonical document and readmits it through Rust; the save link then saves that exact
snapshot. It never exports a partial view. Export contains the current complete source (original until format/rename), IDs,
bindings, layout and notes. Formatting intentionally replaces the source using the
accepted native formatter; it does not preserve an additional pre-format copy. Full validity and permission to
execute remain unestablished.

The adapter enforces read/edit/export independently of artifact claims. Viewer
sessions cannot change or export through this API; already disclosed content can be
copied. No production identity, revocation or DLP is claimed. No URI resolution,
repository browsing, process execution or live provider dispatch exists. Registry
bytes are explicit artifact inputs, not host/provider authority. The finite Wasm
import allowlist admits only reviewed ABI string/externref support; preparation
fails on a changed import set. Generated code is local ignored build output.

Regression commands (after native build):

```sh
cargo test -p choreoform-authoring-prototype --locked
.tools/ir-check/bin/python tools/check_paired_authoring_contract.py
.tools/ir-check/bin/python tools/check_paired_authoring_boundary.py
node --check tools/authoring-prototype/web/app.mjs
```

The boundary tests open a real loopback socket. Their native/HTTP results and CI
compilation are not browser or representative-user evidence. Recorded real browser
observations live separately in docs/evaluation/0096-paired-authoring-prototype.md.
