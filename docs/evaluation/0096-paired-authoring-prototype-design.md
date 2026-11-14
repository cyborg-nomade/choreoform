<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Paired prototype: integration and representation comparison

**Date:** 2026-11-14<br>
**Status:** design preparation only; normative publication/code await verified PR #110 integration<br>
**Authority:** signed [scope96](0096-paired-authoring-prototype-plan.md)
9174cb6ef706eade0c1a3a023f342473e3de4796

## Requirements and alternatives

The [Manifest](../../MANIFEST.md#one-model-two-representations) requires equivalent
forms, progressive disclosure and real semantic preservation. Its non-goals explicitly
exclude **standardizing a visual notation before usability testing**. Therefore
publish an exact **experimental prototype mapping**, not a standardized or user-
validated language notation. Keep representative-user evaluation and later notation
revision explicit. A versioned implementation contract is necessary to test one
prototype; it does not establish usability or freeze a future visual standard.

[Mapping obligations](../authoring/mapping.md) require complete identities, all eight
IR maps, source-only structure, human instructions, policies/protection and exact
pins. Original source and semantic IR cannot be competing unchecked models. The
new native AuthoringDocument can preserve them together; its projection-matched
stage supplies no executable admission, artifact authorization or actual host truth.

| Complete approach | Strengths | Costs / decision |
| --- | --- | --- |
| A. Rust-derived inspection/model and native operations shared through a small browser Wasm bridge; a separate local integration authorizes artifact access | One semantic/refactoring implementation; real browser/Wasm versus native parity; no Python/JS compiler | Needs exact bounded resource/bridge contracts, actual Wasm-import inspection, real browser execution and a separately enforced artifact/session boundary; prefer |
| B. Rust native CLI behind a narrowly scoped local adapter with browser-only presentation | Reuses the already tested CLI immediately; simple native proof boundary | Actual browser UI would not execute Wasm semantics; cannot supply that evidence or silently close browser portability requirements; possible explicit fallback deliverable only |
| C. Reimplement parsing, name resolution, edit logic or semantic meaning in browser JavaScript/Python | Easy local UI customization | Creates a second compiler/refactoring truth and incompatible quota/privacy/identity rules; reject |

The existing [portability probe](../../tools/portability/README.md) already uses
pinned wasm-bindgen0.2.127, bounded byte transfers, exact native/browser report
comparison and observed browser execution. The matching local generator is present.
Its old report/cases/import contract remain unchanged. Prefer a dedicated small
adapter/bridge rather than changing that accepted probe into an editor or embedding
a second compiler. Browser interface/resource/view schemas and all exact public
APIs must be published after dependency verification and before production code.
No framework/ambient-network/clock/provider/runtime engine is needed for the prototype.

## Complete representation and controlled changes

Derive one bounded native view from admitted source/ledger/Candidate, with its exact
authoring/semantic/source binding. Every live declaration, symbol and generated ID
needs an inspectable representation and real ownership. Every selected semantic
record/field and source-only definition must be accounted for; refuse unknown or
unmapped state rather than claiming a complete view. Flow links derive from actual
IR, while source-only tests/types/protection/input names retain their distinct roles.
Show detailed policies/types/human instructions and exact dependency meaning through
readable inspectors and source locations; preserve opaque annotations as metadata.
Do not derive IDs, authority or semantics from labels, positions or proximity.

An explicit source-location provenance distinguishes exact declaration/token spans
from a symbol's owner span; never fabricate field precision. Sparse layout remains
nonsemantic, with deterministic defaults and explicit collapsed state. Lists and
keyboard navigation supplement diagrams. Names/owners/types/kinds and complete
details need accessible text labels, focus and alternatives to color/geometry.
Ordinary users select readable names and actions; the adapter manages IDs/pins/
digests/companions without manual JSON bookkeeping. The accepted source grammar
stays exact; an experimental visual mapping is not evidence of near-English usability.

Only native0.1.0 format/rename/layout operations edit admitted state. Both forms
must re-render the same complete result and expose stale-base/native refusal
locations privately. Arbitrary source/semantic/structural/copy/retire/IR-only/redacted
editing stays explicitly unsupported pending later full mapping profiles. Invalid
drafts and audience/scope views cannot masquerade as a complete admitted artifact
or supply hidden content during export. No general editing/Phase1 gate closes merely
because this bounded paired prototype works.

## Trust and evidence

Authorization is a trusted integration decision **outside** the document/annotations/
actor names/process-value labels and before read/edit/export. Wasm is semantic code,
not an authenticator or provider. A local prototype must publish its concrete narrow
session/origin/file/transport boundary and actually enforce it; do not imply production
security, revocation or prevention of copying already disclosed content. Render
untrusted names/source/metadata as text, never executable HTML/scripts/URLs. Exact
resource bytes are supplied explicitly; no URI fetching or implicit host grants.

Require actual native and browser/Wasm output/refusal parity, reviewed bounded ABI
imports, complete construct/view coverage and real whole RP-01/03 cycles. Native
compilation, generated summaries, Node execution or a native-backed browser view
cannot substitute for browser Wasm execution. Browser actions/accessibility mechanics
are implementation evidence, not actual representative-user or independent review.
All broader operation, full RP-08, complete diagnostics/validator/conformance,
three-process both-form/user/host/independent and Phase1/G1–G4/scenario gates stay
open. Parent46 draft, no Phase2/new phase chat or runtime execution claim.
