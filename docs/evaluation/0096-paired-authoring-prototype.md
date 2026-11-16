<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Experimental paired authoring implementation evidence

**Observed:** 2026-11-16. Accepted within Phase 1 under ADR-0018 delegation;
final PR review/CI/integration is recorded separately, not a personal owner approval.

The signed [scope](0096-paired-authoring-prototype-plan.md) and
[comparison](0096-paired-authoring-prototype-design.md) precede exact publication
`ccc49254cb9813a210e76127b42546053b0ed1b2`. Implementation `8a50215` follows that
publication. The frozen [ADR-0096](../decisions/0096-experimental-paired-prototype.md)
and [profile](../authoring/paired-prototype-v0.1.md) retain their historical
publication-time pending status/bytes. This report supplies subsequent evidence;
it does not change the experimental contract or standardize notation before users.

One immutable Rust AuthoringDocument now produces the bounded complete view.
Every companion declaration, symbol and generated slot is represented once; every
record in all eight semantic maps is accounted for, including exact opaque notes.
Resources use real companion pointers, symbols/provider policies honestly identify
owner spans, and other declarations/slots use native UTF-8 locations. Flows use
explicit outcome identities/wires. Default geometry is nonsemantic. View limits
are checked before fragment growth and IR cloning; oversize output refuses wholly.
The new owned view cannot modify the candidate or confer admission/access authority.

A dedicated native/Wasm bridge reuses exactly those Rust APIs. Its closed range
registry validates contiguous framing and exact supplied pins, with inherited
component bounds. Browser guards run before wasm-bindgen copies; Rust checks again.
Generated imports are exactly the two ABI string/externref functions recorded in
[the allowlist](../../tools/authoring-prototype/imports-0.1.json). The existing
portability probe/import contract is unchanged. Native CLI IO is bounded/private.
No second Python/JavaScript parser, refactoring engine or semantic evaluator exists.

The [local prototype](../../tools/authoring-prototype/README.md) captures three
approved packages, serves only whitelisted code assets on 127.0.0.1, and operates
on isolated temporary snapshots. Independent launcher capabilities gate artifact
read/edit/export before action or disclosure. Artifact labels/claims cannot grant
capabilities; Host/origin/path/body/idle-read guards and private errors bound the
HTTP boundary. No shell interpolation, URI resolution, filesystem browsing, process
execution or provider dispatch is exposed. This is a local session experiment,
not production identity/revocation/DLP or runtime/provider authority. Read permission
necessarily allows copying already disclosed data; no stronger claim is made.

Both forms select the same explicit managed identity. Search/list, scope navigation,
readable/overview diagram scales, keyboard focus, complete native source detail,
read/write/reference facts and honest resource provenance supplement geometry.
All accepted node kinds have explicit labels; current full packages exercise nine,
and a real current-profile fanout document covers the tenth in the native suite.
Format, supported source-only definition/input rename and placement/collapse use
native atomic changes. The browser compares exact native/Wasm document and view
bytes at the same authoring base. Export returns the complete checked canonical
snapshot and readmits it through Rust; a second save action saves those bytes.
General/semantic/structural/copy/retire/IR-only/redacted editing remains unsupported.

## Observed checks

| Evidence | Actual observation / limit |
| --- | --- |
| Native complete views | Example21 entities; RP01 340/53 routes/296968 view bytes; RP03 1442/192/1760748 bytes |
| Native whole-package cycles | Actual rename/layout/format/export/re-admission retain exact complete IR, identity, pins and sparse placement; all eleven existing checking outcomes preserved |
| Native contract/bounds | Four new Rust groups; golden view, registry framing/pins/authority refusal, changes and 270000-value component-versus-view limit; current-profile fanout added to existing package group |
| Schema/publication oracle | Two groups verify raw pins/schemas/vectors, whole source/companion/IR coverage and native locations without becoming an edit engine |
| Real HTTP boundary | Four loopback groups cover missing/invalid sessions, role/origin/Host/path guards, forged claims, actual native edit/stale export/change/refusal, size/method checks; simulated failed-process text tests disclosure only |
| Full local suite | 704 Rust + eight compile-fail checks, six Wasm lints/three release Wasm builds, native lint/build/report and all92 Python commands pass; final UI-only pagination additionally passes syntax and actual browser checks |
| Browser execution | Real Chromium157 on macOS runs the generated Rust Wasm module and matches native bytes for example, full RP01 and full RP03; no Node/native-only substitution |
| Browser actions | Actual Unicode/HTML-looking pure-test rename, input/type/protection renames, layout/collapse, format and prepared complete exports; text is rendered as text, no injected element or console error observed |
| Browser mechanics | Keyboard list/diagram selection retains focus; source click/caret move selects the native declaration; nested scopes, all three RP03 root pages, viewer/exporter/no-session behavior and same-tab session switching observed |
| Final advanced view | All27 pages reconstruct883289 characters and exact RP03 wire SHA6550ac…11cfb4; max32768 characters/page, no scalar split, refresh to page1 after actual format; no truncation |
| Final browser export | Original RP03 formatted document526719 bytes, exact native/browser parity and Rust re-admission; final authoring digest captured below |
| Immutable benchmarks | All965 files retain inventory a25c6de850fdaa51436023fffe011c8f96695e9d8660191e7598cd4f4bf2e9f7 |

The [capture](0096-paired-authoring-observations.json) records exact module/assets,
original snapshots and observed final mechanics. It summarizes real iterative
browser sessions: early rename/layout checks and final paginated-view/session/source
checks share the exact same Wasm module. It is not generated participant evidence.
The temporary launcher was stopped after testing; original packages remain intact.
CI preserves all90 prior Python commands and adds two. Their independent groups
now run in parallel with native/Wasm controls, retaining15-minute job caps and all
locked dependencies. No quota or timeout was raised. Required CI and the separate
completed-diff COMMENT review must pass at final head before expected-head merge.

## Review corrections and open phase gates

Development/review caught same-tab fragment navigation retaining the previous
session; launcher links now reinitialize it. Idle header/socket reads are bounded,
not only POST bodies. A full2.1M-character pretty IR accessibility node caused the
browser inspection to time out; exact advanced IR is now paginated with retained
scalar boundaries, and all27 pages were read and reconstructed in the browser.
Overview/readable scales and pagination explicitly describe filtered geometry.
Review also clarified that format intentionally replaces current source; export
does not secretly retain a second pre-format copy. These are agent findings, not
independent human/CodeRabbit findings or a user-comprehension result.

This slice supplies real browser mechanics and a local artifact boundary. It does
not close near-English accessibility/comprehension, general editing, notation,
three complete process cycles or executable IR/validation gates. RP08 is still only
a brief. All40 scenarios remain Partial, G1–G4 conditional; RP03's external-effects
refusals remain legitimate. Full diagnostics/validator/conformance/negative cases,
RP08, broader operations and real representative users/independent/provider-host /
physical-alias/snapshot/accepted-surviving-work evidence remain Phase1 obligations.
No Phase2, general Studio/runtime, deployment or phase-completion claim follows.
