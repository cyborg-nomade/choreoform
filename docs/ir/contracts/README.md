<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Immutable local contract artifacts

These content-addressed UTF-8 byte snapshots preserve the contracts referenced
by the three IR fixtures. They are distributed in this repository; no network
resolver or registry service is required. The fixture checker's explicit local
registry binds each exact `(id, revision)` pair to the corresponding file.

| Contract ID | Artifact (SHA-256 is the filename suffix) | Source and provenance |
| --- | --- | --- |
| `urn:choreoform:semantics:adr-0008` | [Semantic contract snapshot](sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt) | `docs/decisions/0008-core-process-semantics.md` at `eeddfb43547ac766334e6700d7fb02ef33bf8dde` |
| `urn:choreoform:examples:illustrative-dialect` | [Illustrative dialect snapshot](sha256-282dd3c02983141da13e8f700e852d5472ffc4af176a53ca3578c418482a09b9.txt) | `docs/ir/examples.md` at `102f796dec0bc51c4d15e4dfd3e9bed4bc0d47af` |

Hash every byte, including the original notices and final newline. The local
`.gitattributes` disables line-ending conversion for these snapshots. The `.txt`
files preserve the original Markdown verbatim; relative links inside them are
historical text interpreted in the original source location, not rebased links.
Do not reformat, update links, or change status metadata inside a snapshot.
Editorial changes belong in the source documents, not these artifacts. A new
contract requires a new snapshot and explicit binding; existing artifacts must
remain available. Tests reject changed bytes and unknown identity/digest pairs.

This preserves the existing contract IDs, digests, and definition revisions.
It does not turn the illustrative dialect into an executable contract, approve
ADR-0009, or provide artifact authenticity/signing. Future accepted contracts
must use the same immutable-binding discipline rather than a live document.
