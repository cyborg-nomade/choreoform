<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Immutable local contract artifacts

These content-addressed UTF-8 byte snapshots preserve the contracts referenced
by the three IR fixtures, accepted ADR-0013 specification and later external clock
profile. They are distributed in this repository; no network
resolver or registry service is required. The fixture checker's explicit local
registry binds each exact `(id, revision)` pair to the corresponding file.
The separate [suite publication registry](../../../tools/check_contract_suite.py)
registers ADR-0013 for integrity verification only. It does not add executable
support to the Rust probe, fixture checker or a runtime.

| Contract ID | Artifact (SHA-256 is the filename suffix) | Source and provenance |
| --- | --- | --- |
| `urn:choreoform:contracts:core-profile:0.4.0` | [Explicit retry fault-source binding](sha256-0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3.txt) | Exact `docs/dialects/core-fault-source-v0.4.md` bytes; requires exact 0.3/0.2/0.1 bases; ADR-0042 raw-core support pending parent/main integration; source 0.7 explicitly selects this core under ADR-0043; older profiles retain their cores |
| `urn:choreoform:fault-source-profile:provider-observations:0.1.0` | [Provider-observation fault-source](sha256-f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0.txt) | Exact `docs/dialects/provider-fault-sources.md` bytes; ADR-0041 metadata reader, pending parent/main integration; no implicit core/retry/source binding |
| `urn:choreoform:observation-source-profile:provider-terminal-facts:0.1.0` | [Provider terminal-observation source](sha256-a224fc2b8318af2ff8dfc9955b55af03f4d5b7bd0de719de0384421698d20ead.txt) | Exact `docs/dialects/provider-terminal-observation-sources.md` bytes; ADR-0044 metadata only, pending parent/main integration; no implicit wait/capture/permission |
| `urn:choreoform:semantics:adr-0008` | [Semantic contract snapshot](sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt) | `docs/decisions/0008-core-process-semantics.md` at `eeddfb43547ac766334e6700d7fb02ef33bf8dde` |
| `urn:choreoform:examples:illustrative-dialect` | [Illustrative dialect snapshot](sha256-282dd3c02983141da13e8f700e852d5472ffc4af176a53ca3578c418482a09b9.txt) | `docs/ir/examples.md` at `102f796dec0bc51c4d15e4dfd3e9bed4bc0d47af` |
| `urn:choreoform:contracts:core-profile:0.1.0` | [Accepted executable-contract specification](sha256-26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58.txt) | Three `docs/dialects/` sources at `8ac89587090bc57145da615e4add53548cd661bc`, framed in the order specified below |
| `urn:choreoform:clock-profile:instance-microseconds:0.1.0` | [Instance-clock profile](sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt) | Exact versioned `docs/clocks/instance-microseconds-v0.1.md` bytes; ADR-0023 delegated acceptance, effective upon merge |
| `urn:choreoform:calendar-profile:instance-intervals:0.1.0` | [Instance calendar construction](sha256-2ab8a6a78f2829ababa2d023f6743ac82d7a31acbfca1fb7a03340b7c929f35f.txt) | Exact versioned `docs/clocks/instance-calendar-v0.1.md` bytes; ADR-0025 delegated acceptance, effective upon merge |
| `urn:choreoform:contracts:core-profile:0.2.0` | [Explicit retry clock extension](sha256-5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623.txt) | Exact `docs/dialects/core-retry-v0.2.md` bytes; requires the exact 0.1.0 base artifact; ADR-0026 |

| `urn:choreoform:contracts:core-profile:0.3.0` | [Finite provider metadata closure](sha256-30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3.txt) | Exact `docs/dialects/core-metadata-v0.3.md` bytes; requires exact 0.2.0/0.1.0 artifacts; ADR-0035, effective on parent/main integration |

Hash every byte, including the original notices and final newline. The local
`.gitattributes` disables line-ending conversion for these snapshots. The `.txt`
files preserve the original Markdown verbatim (the suite includes framing); relative links inside them are
historical text interpreted in the original source location, not rebased links.
Do not reformat, update links, or change status metadata inside a snapshot.
Editorial changes belong in the source documents, not these artifacts. A new
contract requires a new snapshot and explicit binding; existing artifacts must
remain available. Tests reject changed bytes and unknown identity/digest pairs.

This preserves the existing contract IDs, digests, and definition revisions.
It does not turn the illustrative dialect into an executable contract, approve
ADR-0009, or provide artifact authenticity/signing. Future accepted contracts
must use the same immutable-binding discipline rather than a live document.

The clock profile is checked by `tools/check_clock_contract.py` and the semantic
crate's metadata reader. Its publication does not enable definition/source timers
or certify an actual host clock. It leaves existing artifact pins/revisions intact.
The calendar construction profile is checked by `tools/check_instance_calendar.py`;
its known mapping and pure interval membership do not approve or bind a live
calendar instance. New profiles never reinterpret the earlier frozen artifacts.

## ADR-0013 publication

The suite snapshot contains, in order, `docs/dialects/README.md`,
`docs/dialects/values-expressions.md`, and `docs/dialects/policies.md`.
Each frame is the UTF-8 path, LF, unsigned decimal byte length, LF, and the exact
source bytes; no extra separator follows the source bytes. SHA-256 covers the
entire framed snapshot. The embedded source commit above is provenance, not a
moving dependency. Relative links are interpreted from each original file.

Publication follows the Project Owner's 2026-09-21 approval of ADR-0013 and PR #17;
merge makes acceptance effective. Integrity and framing are checked in CI with
`python3 tools/check_contract_suite.py`. This check intentionally reads frozen
bytes, not today's editorial sources. It tests corruption, unknown pins and
malformed framing, but cannot certify type/policy enforcement. The existing
illustrative artifacts, registry and fixture revisions remain unchanged.

The earlier pre-merge [publication candidate](sha256-323e6a1333efe412088e52e2fda5cf37c4651dfff3f36b284d95020ea2753bab.txt)
is retained byte-for-byte for provenance but is not the accepted registered pin.
Its source commit was `8f71da41233dfc0834118c54bf1ab8b7110ccc79`. Final review
identified a misleading integer-to-decimal sentence; the registered snapshot
clarifies the already-closed AST: integer-to-decimal conversion is unsupported,
and nominal wrapping requires an exact representation-type match. No operation
or coercion was added. Changed source bytes require the new digest above.
