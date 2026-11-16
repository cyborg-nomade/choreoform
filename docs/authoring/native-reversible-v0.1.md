<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

**Status:** Normative binding accepted under ADR-0095 / owner delegation; native implementation pending.
**Date:** 2026-11-14. PR #109 dependency verified: signed merge23f97d66bb39dc31b4e962c857326d977f5bff96 after successful CI43587788867 at e7231bd33244c552b5677a339baee8306da70b1e, automated COMMENT review5903963064 and fresh complete audit.


# Native reversible authoring binding 0.1.0

Binds the exact document/change/layout0.1.0 contract to the existing Rust authoring-frontend crate and explicit CLI. Does not change source19/companion18/C12/IR4 or any old API. The trusted integration authorizes artifact read/edit/export before invocation; a pure library cannot authenticate its caller. These APIs do not confer permission or executable admission.

## SDK

New exported types:

```rust
pub struct AuthoringDocument { /* private checked state */ }
pub enum AuthoringComponent { Document, Source, Bindings, Layout, Change, Resources }
pub struct AuthoringDiagnostic {
    pub code: &'static str,
    pub component: AuthoringComponent,
    pub path: &'static str,
    pub span: std::ops::Range<usize>,
    pub entity: Option<String>,
}
pub type AuthoringResult<T> = std::result::Result<T, AuthoringDiagnostic>;
```

Exact methods (Resource is choreoform_ir_probe_core::Resource):

```rust
impl AuthoringDocument {
    pub fn admit(raw: &[u8], resources: &[Resource<'_>]) -> AuthoringResult<Self>;
    pub fn from_package(source: &[u8], bindings: &[u8], layout: &[u8], resources: &[Resource<'_>]) -> AuthoringResult<Self>;
    pub fn document(&self) -> &serde_json::Value;
    pub fn candidate(&self) -> &Candidate;
    pub fn authoring_digest(&self) -> &str;
    pub fn source(&self) -> &str;
    pub fn bindings(&self) -> &str;
    pub fn export(&self) -> AuthoringResult<Vec<u8>>;
    pub fn apply(&self, request: &[u8], resources: &[Resource<'_>]) -> AuthoringResult<Self>;
}
```

admit verifies strict document/schema/invariants/native resources/lowering and projection. from_package derives its integrity/projection rather than accepting old claims; layout input is strict JSON at most1MiB raw and256KiB JCS. Both return immutable projection-matched state. document/candidate are shared inspection borrows, with no mutable/unchecked constructor/metadata replacement. No public interface can serialize an unadmitted value as a successful document. Immutable checked state carries a static premise, not live actor/provider authority. export emits bounded JCS without LF; reopening that JSON always requires admit. No executable/ValidatedDefinition type.

apply accepts only the closed0.1.0 change request. It validates request/base, supplies exact resource premises, applies format/rename/layout atomically and verifies complete unchanged IR including annotations. Base object is immutable, no disk mutation or partial-success output. Same-name rename returns identical document values/digest. Other successful output follows94's deterministic source/companion/layout rules. Unsupported copy/retire/structural/semantic operations refuse; no permissive Value mutation API. Changed resource bytes cannot reuse an earlier static premise.

## Exact local resources

The new document layer supplies immutable compile-time bytes for accepted semantics ADR8 and C1–C12, matching the current source19 CLI behavior. They are contract descriptions, not providers or host permission. Caller resources remain explicit, unique by(contract,digest), hash-exact, each at most1MiB, at most128 rows and8MiB total including unused extras. Check all caller bytes. Coalesce an exact caller duplicate of a compiled-in pin only after hash/byte agreement; never override it, fetch a URI or install unknown semantics. Compiled-in constants do not consume the external registry allowance. Existing Package/raw SDK resource behavior stays unchanged. No C13/source20 addition.

## CLI

Explicit commands on choreoform-authoring-frontend:

- document-inspect DOCUMENT [--resource ID REVISION PATH]...: re-admit; emit the closed authoring-inspection0.1.0 non-executable view schema.
- document-export DOCUMENT [--resource ...]...: re-admit; emit only document JCS.
- document-change DOCUMENT CHANGE [--resource ...]...: admit, apply; emit only the complete new document JCS.
- document-import SOURCE COMPANION LAYOUT [--resource ...]...: from_package; emit only derived complete document JCS.

All paths explicit, no discovery/fetch. Bound file reads at allowed+1 before refusal. Buffer output until all checks pass. No stdout artifact/view on failure; stderr contains fixed code/component/static path, optional validated managed ID and source UTF-8 span. No echo of source/names/payloads/file paths/resource contents. Generic filesystem failures use fixed private read/arguments codes. Existing command parsing/stages/output remain unchanged. The library has no IO/network/execution.

The closed inspection schema has exactly format=choreoform-authoring-inspection,version=0.1.0,status=projection-matched,executable=false,definition,semanticRevision,authoringDigest,sourceBytes,bindingsBytes,layoutEntries. Definition uses the existing ID grammar; revisions/digests exact lowercase sha256. Counts within existing component/layout limits. This is an inspection of an admitted authoring projection, not full validation or permission. No source preview or sensitive names are emitted by default.

## Shared bounded JSON infrastructure

The IR probe core additionally exports transport::DecodeLimits and transport::decode_with_limits(raw: &[u8], limits: DecodeLimits) -> Result<serde_json::Value>. DecodeLimits has public usize fields max_bytes, max_containers, max_items and bool reject_negative_zero. Each numeric bound must be nonzero and no larger than16MiB /64 /262144 respectively; larger or zero configuration refuses. Items count every JSON value and object key before allocating it. Container depth counts the root container as1, strings as scalars. Numeric negative zero refuses when selected. Document/change/layout use reject_negative_zero=true and the exact respective bound above. The result is a raw JSON value, never an admitted IR or document. Existing transport::decode/decode_canonical/DefinitionEnvelope admission defaults and legacy numeric/depth behavior remain unchanged; the new function cannot relax those entry points. Reuse the strict parser and bounded canonical writer, no second JSON grammar. This extra raw decoder grants no semantic/structural/execution authority.

## Native operation bounds and diagnostics

All94 bounds remain, including raw/JCS document16MiB, decodedsource/bindings1MiB,64 outer containers,262144 JSON values+keys, layout256KiB/10000 entries, request1MiB. New decoder keeps old transport defaults/legacy -0 behavior unchanged. New authoring path rejects -0, duplicate decoded keys/floats/NaN/surrogates/BOM/trailing data. Existing component/container meanings remain their native exact contracts.

Bounded canonical writer refuses before oversize output growth. Native operation source-walk has one cumulative1M work counter:1 pervisited syntax tree/ledger row/splice,1 per qualification component,64 per indexed lexical lookup (safe upper bound for max64 scopes). Charge in advance before lookup/traversal/recording. Pending walk stack charged32 logical bytes per entry, max1MiB live; source splice metadata charged16 logical bytes per recorded span, max1MiB cumulatively; replacement bytes count against the same splice budget. These fixed logical charges make boundary decisions portable across pointer sizes. Bound output source length before allocation. Native parse/lower/expansion/semantic checks retain their own exact inherited caps; no per-occurrence reset or quota increases. Oversize work/metadata refuses, not truncation. One scope/owner-aware indexed walk replaces whole-source/per-entity rescans; opaque literal/type payloads are never walked as symbol uses.

Diagnostics use fixed codes/components/static paths and validated IDs only. Native source diagnostics preserve actual byte spans; other spans0..0. Path strings identify known format fields, never concatenate untrusted names or payload keys. Admission mismatch codes distinguish document-format,component-profile,integrity-mismatch,projection-mismatch,layout-reference/resource-integrity/registry-limit; request codes distinguish change-format,stale-base,unsupported-target,retired-target,name-conflict. Bounds/IO have private fixed codes. Actual inherited parser/binder/type errors retain native fixed codes and correct component/span. Exact whole error precedence among independently malformed fields is not promised; no error returns a successful partial artifact.

## Evidence

This binding, the closed inspection schema and ADR-0095 must be signed/pinned before production code; the verified PR #109 dependency is satisfied. Native tests must execute all pinned94 before/request/after pairs, additional nontrivial lexical/type/input/Unicode/opaque cases, boundary/refusal/immutability and whole current RP01/RP03 operation cycles with exact checking outcomes. Python contract pairs are no replacement for real native edits. Full final-head native/Wasm/Python CI, separate automated review/full comment audit/expected-head signed merge and progress update. No benchmark/publication byte changes, protected UI/security enforcement/user/host/browser/independent evidence or phase completion claim.
