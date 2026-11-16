<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0084: Explicit named data protection with unchanged expanded semantics

**Status:** Accepted under Project Owner delegation; effective on integration<br>
**Date:** 2026-11-13<br>
**Decider:** Codex under ADR-0018; no personal owner or independent approval

## Context and decision

Signed [scope84](../evaluation/0084-named-data-protection-plan.md) `c76073f`
follows ADR-0083's measured critique. PR #98 merged as
`c06480f51fc0a3f993e7bf67ceae951b053aca86` after successful final-head CI
`43401288608` at `634520845124ec6f406572e3c7979d05921ea362`, automated COMMENT
review `5888768916` and fresh full audit. Its first CI run exposed a measurement
Cargo path assumption; the signed follow-up fixes it, and the failed run is not
passing evidence. This contract is frozen/signed before compiler work.

Choose `Protect data as "Name". ... End protection.` and an explicit
`Keep "Value" as type, protected as "Name".` use. The verb-led definition makes
its data purpose visible; the short use reads as a qualification of the kept data.
A bare `Protection "Name".` is shorter but less explicit; ambient defaults hide
exposure and are rejected. Wording is provisional Candidate A, not user-tested.

Publish source 0.17 / companion 0.16 on unchanged C12 / IR 0.4. The exact profile
specifies complete envelopes, lexical definition/member/use visibility, validation
of unused definitions, source-only managed identity and per-data purpose symbols
whose complete wire set must match the definition. Retaining per-data purpose
identities preserves existing purpose literals and captions without inventing
symbols. Definition references never widen permissions or rewrite providers.

| Frozen bytes | Digest |
| --- | --- |
| [candidate-a-v0.17.ebnf](../../docs/authoring/candidate-a-v0.17.ebnf) | `sha256:81f18a144ea9d826a7368525d42036e23c8d4131d558d387e62eb3e8e802b4b9` |
| [profile-v0.17.md](../../docs/authoring/profile-v0.17.md) | `sha256:a2ad9ea91f72f33b63e9cd83488e760cd95e7ee242863dbacd2eda4afa6e8c81` |
| [bindings-0.16.schema.json](../../schemas/authoring/bindings-0.16.schema.json) | `sha256:cb855732f393e0b16288f0f7808e6537bc4d7f6748b6d55f84d0f11386df9fa8` |

These source-publication digests do not create or replace semantic core artifacts.
Old publications and current/historical benchmark bytes stay fixed. Production
uses the Rust tree/resolver; Python is an independent test harness only. Cumulative
work/expanded-byte/record bounds precede copies/insertion, with unchanged final
wire limits. Define once, resolve immutably, expand exactly; no recursive groups.

## Consequences and validation

Readable reuse adds companion identities and can hide exposure. Later protected
accessible inspection/editing must expose the complete envelope; this change does
not claim that editor or general export. Explicit rename/copy/tombstone operations
remain mandatory. Full positive/negative source, ledger, purpose, resource, privacy,
old-version and bound controls must pass, including formatting/integrity cycles.

Actually lower and check new whole-case RP-01/RP-03 variants, comparing exact
expanded bodies/revisions and measuring genuine RP-03 source-byte reduction >=15%.
Preserve the existing external-effects refusal, guards, receipts and resource pins.
Benchmark adoption/history, type/service/predicate reuse and C13 source selection
remain separate signed scopes. Required final-head native/Wasm/Python CI, separate
automated completed-diff review, full comment audit and expected-head signed merge
precede progress. No real-user, browser, host or independent evidence is fabricated.
All unresolved semantic/visual/exit gates remain open; parent #46 stays draft and
Phase 2 cannot start.
