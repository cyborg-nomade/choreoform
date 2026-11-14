<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0095: Native reversible authoring binding and immutable operations

**Status:** Exact binding accepted under Project Owner delegation; implementation pending<br>
**Date:** 2026-11-14<br>
**Decider:** Codex under ADR-0018; no personal owner or independent review

Signed [scope95](../evaluation/0095-native-reversible-authoring-plan.md)
5bfea4b93b8144d97b9088ea27079667b26e5576 and signed
[comparison358ed5b](../evaluation/0095-native-reversible-authoring-design.md)
precede this publication. PR #109 dependency is verified: signed merge
23f97d66bb39dc31b4e962c857326d977f5bff96 after successful CI43587788867 at
e7231bd33244c552b5677a339baee8306da70b1e, automated COMMENT review5903963064
and fresh complete-body/thread/head audit. All eight commits/merge signed; bot skip
is no evidence. Final-head688 Rust + six compile-fail/native/Wasm/89 Python pass.

Accept the [exact native binding](../authoring/native-reversible-v0.1.md) for the
published document/change/layout0.1.0 contract. Extend the existing Rust frontend
with a distinct immutable AuthoringDocument, private checked Candidate/state and
shared read-only inspection. Reuse real AST spans, indexed ledger kind/owner/
lexical resolution and native lowering; reject a second Python/JavaScript compiler
or duplicating those rules in an independent editor crate. Original raw strings,
whole authoring digest, semantic projection and nonsemantic layout remain distinct.

The SDK binds complete admission, import, inspection/export and the full published
format/source-only-caption-rename/layout-replacement matrix. Exact new CLI commands
and a closed non-executable inspection schema are specified before code. Caller
read/edit/export authorization remains external to untrusted JSON and pure native
checks. No permission flag, host grant or executable admission follows. No general
copy/retire/structural/semantic edits/IR-only reconstruction are added implicitly.

A separately bounded raw JSON decoder may share the existing strict parser while
leaving old IR transport defaults/number/depth behavior unchanged. New whole bundle
limits do not increase any component/compiler/SDK limit. Fixed logical operation
work/metadata charges retain portable refusal boundaries. Native output must match
real golden vectors and semantic preservation, not just schema or Python after-states.

Normative raw-byte binding pins (not semantic revisions):

- `docs/authoring/native-reversible-v0.1.md` — `sha256:6698be358d22a6711f9f357a78184c42314da7ca6d052a00605b13e8d9cfc07e`
- `schemas/authoring/inspection-0.1.schema.json` — `sha256:1d726a7b1ac5dcecf33a773d9fa9edc5952bca7747a902ff17ce84074949fbde`

Sign this publication before production implementation. Existing source19/companion18/
C12/IR4 and all older APIs/accepted publications/vectors/965 benchmark bytes stay
exact. Relevant native negatives/positive actual operations, immutable compile-fail,
whole current RP01/RP03/all11 checking views, final-head native/Wasm/Python CI,
separate completed-diff automated review/all comment assessment/fresh exact-head
signed merge remain required. Native SDK/API checking cannot substitute for paired
accessible/protected UI, real cycles, complete validation/diagnostics/conformance,
full RP-08, three full representative processes or actual users/hosts/browsers/
independent evidence. All full Phase1/G1–G4/scenario/external gates remain open,
parent46 draft. No Phase2/new phase chat; owner approval still gates the phase report.
