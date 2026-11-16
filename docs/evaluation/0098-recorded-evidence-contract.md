<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Recorded-evidence contract publication and reference checks

**Date:** 2026-11-16. Contract/reference evidence only; production reader pending.

Signed scope d72c286 and comparison cdbf4e3 follow ADR-0097. Verified dependency
PR112 merged asa8fd287fa7a10e34fc20fecfd65bf40d466ee2c7 after successful CI43836632627
atc9f395dde5830791e6ba5ced3558a0184d3c27cf, automated COMMENT5917447049 and fresh
full audit. All six commits/merge signed; CodeRabbit skipped, no independent approval.

Exact [profile](../evidence/recorded-v0.1.md), schemas, immutable future SDK/CLI/bridge
binding and complete authored [vectors](../evidence/recorded-0.1/README.md) are signed
in `8f4cd2243ff70381d10904b76ed1f02565d81a5f`, before committed reference tooling and
all future production reader code. [ADR-0098](../decisions/0098-recorded-evidence-typed-claims.md)
records acceptance under delegation, raw pins and the preliminary draft-check
chronology. Preliminary uncommitted sanity checks occurred earlier than the plan's
post-publication reference sequence; formal checks were repeated after signing.
There is no hidden new native admission or event result in those draft checks.

The separately versioned1MiB artifact retains literal original text, typed documentary
claims, delivery witnesses, explicit asserted order/dispute links and unknown boundary
references. Equal source/event cores can repeat; conflicting reuse refuses. Claim
hashes bind the complete resolved reported clock basis, not accidental internal IDs.
All clocks/source/context/authority-looking values remain claims. No timestamp-derived
edge, authenticated/accepted history, current decision/effect grant, dispatch/replay,
terminal-history mutation, generic importer or redacted complete model is supplied.

Two draft corrections preceded the frozen publication: complete clock basis binding
was added to claim digests, with ID-rename-stable/origin-change-sensitive positives;
and global versus within-artifact identity was made explicit. A standalone valid
artifact cannot check an ambient database. All actual acceptance/correlation/authority /
protected-audience and whole RP08 obligations remain external/later supported layers.

## Actual checks and their limits

| Check | Observed result / boundary |
| --- | --- |
| Exact publication | Profile raw SHA c01a6aaf6291ece728425f83eda5e3edaa64b6fc6b964a0b5bf8151561bba30c; schema/vector/ADR pins checked after signing |
| Authored worked model | Five documentary witnesses/four local asserted identities, three sources/clocks, one unknown boundary and three asserted relations. These are fixture counts, not five received events |
| Explicit resource inputs |15 exact resources,153593 raw bytes: profile, semantics/cores1–12 and small named-type descriptor. Header contiguous framing/pins verified; paths are developer fixture inputs, not a production resolver |
| Reference checks | Three test groups pass: raw pins/shapes/expected hashes, reference identity/order and cumulative framing, actual existing native payload/transport APIs and unchanged benchmarks |
| Digest identity properties | Equal redelivery core and consistently renamed local clock ID preserve claim digest; changed claimed origin changes it. Artifact hash covers all retained witnesses/deliveries/metadata |
| Structured negatives | Unknown authority/profile/core/type, wrong original digest, incompatible identity reuse, hidden alias self-order, cycle, missing reference, wrong clock owner and a local identity disguised as an outside boundary refuse in the reference mathematics/native baseline at their actual layer |
| Cumulative framing | A generated1024-row valid-shape candidate below1MiB raw exceeds cumulative8MiB frame work through repeated long namespace/source context and refuses; no partial reference result |
| Time/order separation | Reversed reported times on one claimed basis do not manufacture/validate an ordering edge; explicit asserted links alone define the checked graph |
| Existing native value API | Real Resources/type_descriptor/literal admits the five worked payloads and refuses wrong value/unknown descriptor cases under one resource budget; no new artifact reader |
| Existing native transport API | Eight malformed raw inputs rejected under selected1MiB/64-container/262144-item/no-negative-zero limits; generic valid `[]` still passes transport, demonstrating it is not artifact admission |
| Preserved benchmarks | All965 files retain inventory a25c6de850fdaa51436023fffe011c8f96695e9d8660191e7598cd4f4bf2e9f7 |

The Python tool is explicitly test-only shape/identity/order mathematics. It does
not implement C12 payload semantics; the existing Rust example supplies those
limited actual literal facts. Schema shape/reference success is not a new native
RecordedEvidence result, verified source/normalizer, clock observation, runtime
history, actual provider/host, browser or user evidence. Expected report/digests are
authored mathematical after-states. There is no new production SDK/CLI/Wasm reader yet.

Reference commands:

```sh
.tools/ir-check/bin/python tools/check_recorded_evidence_contract.py
cargo run -p choreoform-semantic-check --example recorded_payload_baseline --locked
```

The workflow retains all92 previous Python commands and adds this checker as93,
with the same15m job caps. Native704 Rust/eight compile-fail and native/Wasm/full93
final-head CI plus separate completed-diff COMMENT review/full comment audit remain
required before expected-head signed merge. Do not claim the earlier112 run is CI
for this new publication.

Native reader/IO/bridge/private refusals/budget/conformance and actual new browser
execution are the next separately scoped implementation after verified integration.
Full RP08 batch/result/ad hoc-work/effect/audience, broader managed editing, full
validator/actionable diagnostics/conformance/negative examples, three-process cycles,
actual representative users/independent/provider/host/physical-alias/original-snapshot /
accepted-surviving-work evidence remain Phase1 obligations. All40 scenarios Partial,
G1–G4 conditional, parent46 draft; no notation standard/Phase2/phase completion.

## Completed-diff review correction

The full local704 Rust + eight compile-fail/native/Wasm/all93-command suite passed
atcf9fc06. A separate review found that the developer registry check zipped input
and header rows without asserting equal cardinality. The oracle now validates the
closed header schema, exact row count/unique pins and every contiguous range, with
extra-row/gap/forged-authority mutations refused. All three affected groups pass
again after correction. Every frozen profile/schema/vector byte remains exactly
8f4cd22; no production reader behavior is invented. Required final-head CI repeats
the full suite at the corrected head before merge.
