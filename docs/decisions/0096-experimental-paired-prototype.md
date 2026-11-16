<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0096: Experimental paired prototype over one Rust authoring model

**Status:** Exact experimental mapping/interfaces accepted under Project Owner delegation; implementation pending<br>
**Date:** 2026-11-14<br>
**Decider:** Codex under ADR-0018; no personal owner or independent review

Signed [scope96](../evaluation/0096-paired-authoring-prototype-plan.md)
9174cb6ef706eade0c1a3a023f342473e3de4796 and signed
[comparison604337c](../evaluation/0096-paired-authoring-prototype-design.md) precede
this publication. PR #110 dependency verified: signed merge
aebca078354c847602c937e131d6b899edf4e368 after successful CI43613364792 at
1978088c3fd23d2c2154c8c0f2c1aa08ab7c4f58, automated COMMENT review5905656042
and fresh complete comment/thread/head audit. All eight commits/merge signed;
700 Rust + eight compile-fail/native/Wasm/all90 Python commands pass. Bot skip
is no evidence. No independent/human approval or actual user observation.

Accept one exact [experimental prototype mapping](../authoring/paired-prototype-v0.1.md),
not a standardized visual language. The Manifest explicitly forbids standardizing
notation before usability testing. Versioning a falsifiable prototype is necessary
to compare real results; clarity/general usability still requires representative
users and later revision. No full authoring/visual/phase gate closes here.

Choose a dedicated small Rust native/Wasm adapter over the existing immutable
AuthoringDocument, plus a separately enforced narrow local artifact/session boundary.
Do not rewrite semantics/refactoring in JavaScript/Python or turn the accepted old
portability probe into an editor. The matching pinned wasm-bindgen0.2.127 generator
is present. Publish exact SDK view, complete entity/source/IR/field mapping, resource
byte registry, native/browser bridge, HTTP/access/output bounds and closed schemas
before code. Native/Wasm parity and actual browser execution are distinct required
observations, never inferred from compilation or generated expectations.

All live managed rows and semantic records are represented with explicit ownership,
readable captions, full accepted source/IR and actual flows/details. Unused pure
source-only definitions remain. Resource aliases live only in the companion and get
companion locations; symbols get honestly labeled owner spans, generated slots their
real source provenance. Never infer IDs/spans/meaning from names, order or positions.
Sparse layout stays nonsemantic. All first-profile edits use native atomic operations;
unsupported general/structural/semantic/copy/retire/redacted/IR-only edits stay explicit.
Ordinary users do not maintain JSON/wires/hashes. Accessibility requires meaningful
labels/focus/keyboard/tree alternatives to geometry or color, plus later real users.

A trusted local launcher/session context gates read/edit/export outside artifacts
before disclosure/actions. No actor/process label/annotation/digest/JSON grant becomes
authority. Working copies and exact pinned byte registries are isolated; no arbitrary
paths/URI fetching/command interpolation/original benchmark writes. No DLP/revocation/
production identity or runtime/provider-host guarantee is claimed. Process execution
is outside this prototype. The view and session catalog are not admitted documents.

Normative raw-byte publication pins (not semantic revisions):

- `docs/authoring/paired-prototype-v0.1.md` — `sha256:c7239cd5d9987bf4c9baf01e4bb501899f8fcaeec74983afbfc8dee738f4b993`
- `schemas/authoring/view-0.1.schema.json` — `sha256:41fa0195f72f6737d004b1cd0c231374b000f5d342e260ea10e1fb43d7c17623`
- `schemas/authoring/resource-registry-0.1.schema.json` — `sha256:de3c6b7c54531178385a9a35b7cc1b1ec982a751b6d3d507f02b5a1898115d56`
- `schemas/authoring/prototype-catalog-0.1.schema.json` — `sha256:69753de7de2e14f2ca3f73d03df30aa93e20bbed17bc088df65e670a86e26439`
- `schemas/authoring/prototype-error-0.1.schema.json` — `sha256:173784d46e1768f1a766bd2f58d9a8fa98742f462f2285124c8b98b9adf02079`
- `docs/authoring/paired-0.1/vectors.json` — `sha256:3ca9396a49ca1839146b7e3fbf9e72fecfea59f9b8cd641f2c60df384c21db40`

The vector manifest raw-pins every schema/worked positive/negative artifact. Existing
PR110 native APIs actually supply source/IR/integrity/spans for the finite example;
its view/default placement is authored contract data, not the proposed view/bridge/UI
implementation or browser/user/access-enforcement evidence. Source-only resource
metadata is deliberate; no unused pin grants provider truth or nominal membership.
Sign this publication before production implementation.

Preserve document/change/layout0.1.0, native binding0.1.0, source19/companion18/C12 /
IR4, old APIs/portability probe and all accepted publication/vector/965 benchmark
bytes. No source20/core promotion/generic work/runtime grant or quota relaxation.
Require complete construct/field/identity/location coverage, real full RP01/RP03
cycles/all checking outcomes, malformed/stale/opaque/resource/authorization/XSS/bounds,
actual browser/Wasm/native parity and honest accessibility mechanics. Review actual
Wasm imports finitely; no ambient host capabilities. Final-head native/Wasm/Python
CI, separate completed-diff automated COMMENT review/all comment assessment/fresh
expected-head signed merge precede progress update. Native tests or a browser UI
backed only by native code do not prove browser Wasm execution.

Full RP-08, general operation profiles, full diagnostics/validator/conformance/
negative examples, three-process both-form cycles and actual representative users /
hosts/browsers/independent evidence remain Phase1 obligations. All40 scenarios Partial,
G1–G4 conditional, parent46 draft. No Phase2/new phase chat or phase completion;
honest phase report still requires owner approval. Never generate external evidence.
