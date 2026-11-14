<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0094: Versioned reversible authoring documents and atomic changes

**Status:** Contract accepted under Project Owner delegation; integrated in PR #109; native implementation pending<br>
**Date:** 2026-11-14<br>
**Decider:** Codex under ADR-0018; no personal owner or independent review

Signed [scope94](../evaluation/0094-reversible-authoring-contract-plan.md)
`d3ab333ff7726f009ebe832e925af0bb9fb5dc3f` precedes contract publication.
PR #108 dependency integrated as7471559493a5b97d4a3bac5377d1a7b6a94a7e47,
CI43584392413 at a43da6566804d7be51b01e5a3da493f65e8f3509, separate automated
COMMENT review5903706053 and full fresh audit. Signed
[comparison](../evaluation/0094-reversible-authoring-design.md)
58d1104 selects a separate versioned document rather than unchecked IR annotations
or IR-only reconstruction. This preserves names/source-only definitions/managed
identities while keeping one checked semantic meaning and original component APIs.

Accept exact document/change/layout0.1.0 with source19/companion18/C12/IR4 only.
Original source and companion strings plus their native integrity, expected
semantic binding and sparse identity-indexed layout are carried together. Admit
only after strict component checks, exact resources and real native lowering agree.
Whole-document integrity is separate from semantic revision and native source/ledger
integrity. Reject unknown/stale/inconsistent state, no fallback or partial success.

The first change matrix is format, rename of explicit source-only predicate/type/
protection aliases or predicate-input captions, and replace-layout. Resolve all
uses structurally, preserve fixed wires/IDs/opaque literals/annotations, check
whole-base conflicts and re-lower to exact same IR. General copy/retirement/
structural/semantic edits and IR-only export explicitly refuse until a later full
mapping profile. This bounded foundation does not complete managed editing.
Trusted integration separately authorizes artifact read/edit/export; neither JSON
claims nor process value labels grant access. Faithful accessible matched views,
invalid-draft separation, resource and cumulative transport/native bounds are
normative obligations. No live host authority or protected UI implementation claim.

Normative raw-byte publication pins (not semantic revisions):

- `docs/authoring/reversible-v0.1.md` — `sha256:3063196c90643c182e0c8394c653af834523cee3273038eb214dafd715d41ee2`
- `schemas/authoring/document-0.1.schema.json` — `sha256:ab0dea852d3744e1505e746e9a3b196f256094e42d9f65c6b9522fbc741c741e`
- `schemas/authoring/layout-0.1.schema.json` — `sha256:83618506fe0d4d7d8feaba179863b5b238d49b75e847389195380e3efa063a4f`
- `schemas/authoring/change-0.1.schema.json` — `sha256:48317385bfed0e43aad8102d835886192b04c454762589589b8f017b3395faec`
- `docs/authoring/reversible-0.1/vectors.json` — `sha256:635cc725a4a8da57b5b176b471ffe6ef440fce69221f770c10445addd1c85c28`

The vector manifest pins every schema and finite worked positive/negative state or
request. Finite after-states are authored examples, not generated native managed
operations. A later read-only contract oracle checks schema/consistency and existing
native projections. Publish and sign these artifacts before that support code and
before later production implementation. No existing grammar/profile/schema/contract,
compiler/SDK/API/quota,965 benchmark bytes or corpus/report captures change.

Separate subsequent signed scopes implement native admission/operations then paired
accessible prototype and real cycles. Full RP-08, actionable diagnostics, complete
validator/conformance/negative examples, three-process both-form cycles, actual
representative users and external/independent evidence stay within Phase1. All40
scenarios Partial, G1–G4 conditional, parent46 draft; no Phase2. Required final-head
checks/CI, separate completed-diff automated COMMENT review, all review/comment
assessment and expected-head signed merge remain prerequisites for integration.

PR #109 integration23f97d66bb39dc31b4e962c857326d977f5bff96 follows successful
CI43587788867 at e7231bd33244c552b5677a339baee8306da70b1e, automated COMMENT
review5903963064 and fresh full audit. All eight commits/merge have sign-off;
CodeRabbit skips, no substantive findings or independent/human evidence. Final-head
688 Rust + six compile-fail/native/Wasm/all89 Python commands pass. Previous pending
statements are historical; all native/visual/full Phase1 obligations remain open.
