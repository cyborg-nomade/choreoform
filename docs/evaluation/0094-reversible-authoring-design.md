<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Reversible authoring contract: representation and operation comparison

**Date:** 2026-11-14<br>
**Status:** design comparison; exact contract not published or implemented.<br>
**Authority:** signed [scope94](0094-reversible-authoring-contract-plan.md)
d3ab333ff7726f009ebe832e925af0bb9fb5dc3f after PR #108 integration
7471559493a5b97d4a3bac5377d1a7b6a94a7e47, successful CI43584392413 at
a43da6566804d7be51b01e5a3da493f65e8f3509, separate automated COMMENT
review5903706053 and fresh full audit. No personal owner or independent review.

## Requirements and choices

The [Manifest](../../MANIFEST.md) requires one process meaning across text and
visual forms. [ADR-0009](../decisions/0009-canonical-versioned-ir.md) keeps required
human instructions and policy/protection meaning in the semantic body, while
layout/formatting stay non-semantic. [ADR-0093](../decisions/0093-authoring-completion-sequence.md)
records a real counterexample: complete source18/source19 packages lower to the
same IR but retain different named authoring structures during native formatting.

| Complete alternative | Strengths | Costs / failure modes | Decision |
| --- | --- | --- | --- |
| A. Separate versioned authoring package containing exact source, strict companion, expected semantic binding and layout | Preserves source-only definitions and managed IDs; explicit admission/consistency boundary; does not enlarge execution IR | Must re-lower and compare semantic binding with exact resources; package admission is not full validation or permission | Prefer as the initial bounded contract |
| B. Versioned authoring payload inside IR annotations | Can travel beside a definition; annotations already outside semantic revision | Existing annotations are deliberately opaque; a frontend could display stale or forged source while treating the IR as authoritative; privacy/transport coupling | Possible future carrier only through the same checked package admission; not an unchecked second model |
| C. IR-only canonical inline source export | Simple semantic target and potentially stable canonical output | Cannot recover named pure tests/type/protection spellings uniquely; may expand readable source; loses authored identity structure without a separately defined mapping | Separate explicit export mode later; does not satisfy the preserved-authoring requirement alone |

Choose A under delegation, subject to exact normative publication and worked
vectors. The source plus strict companion is the authoritative authoring state;
execution IR is its checked semantic projection. Layout and opaque author notes
have no execution authority. A claimed semantic revision never bypasses native
lowering or matching resource bytes. An inconsistent package cannot be used as a
successful visual/text export or silently fall back to whichever component looks
valid. A renderer must not trust a package's own assertion that it is validated.

## Bounded initial profile

Define a standalone package/change format independent of existing grammar and IR
versions. Its first exact component matrix should select source19/companion18 /
C12/IR0.4; older sources remain supported by their existing APIs, with no inferred
range compatibility. Preserve original source and strict companion, including
source-only tests/inputs/type/protection names and annotations. Resource contents
are supplied separately and checked against declared pins; self-contained package
metadata supplies no provider truth or authorizing host identity.

Keep semantic revision distinct from source/companion/joint-package integrity and
whole authoring/layout integrity. A layout-only or caption-only managed change
must preserve the semantic projection; a semantic source change must derive its
new revision, not retain a stale claim. Whole transport needs a separate bounded
limit because source and companion are already separately bounded; no source,
compiler, expansion or semantic-proof quota is increased by bundling them.

The contract should establish faithful representation obligations for every
selected construct. Layout references managed identities, not name/position
recovery. Source-only definitions need their own representation; a collapsed
entity is explicitly collapsed, not absent or silently unsupported. Complete
visual notation and accessible prototype implementation remain subsequent work.

## Managed changes and explicit refusal

The initial operation profile should be small enough to implement and test
completely: inspect/format, identity-preserving caption rename, and non-semantic
layout changes with exact stale-base preconditions. Rename must follow resolved
declarations/symbol uses/qualified references and preserve IDs, wires, required
human instructions and opaque literal/type payloads. It must not globally replace
matching text or infer new IDs from captions. Unsupported target kinds refuse.

Copy, retirement, arbitrary structural edits and IR-only reverse export need
complete explicit identity/reference/resource mapping contracts. They must be
described as unsupported in the first operation profile rather than half-admitted:
copy eventually allocates explicit fresh IDs for all affected identities; retirement
eventually preserves tombstones and rejects live references. A later profile must
define those complete cases before implementation. This is a bounded foundation,
not completion of the full managed editing/export deliverable.

Requests bind the complete current authoring integrity, so source-only/layout
changes can cause a stale-base refusal even when semantic revision is unchanged.
Results are atomic: either a complete new checked package/projection with stage
information or a private refusal, never a successful partial source/ledger update.
Invalid textual drafts may be retained by a future editor as drafts, but cannot
masquerade as a valid package or a fresh semantic/visual projection.

## Protection and admission boundaries

Package possession, hashes, selected actor names and process-data protection labels
do not establish authoring access or live process authority. An integration must
authorize reading/editing/exporting the artifact before invoking operations or
disclosing protected content. That trusted decision cannot come from untrusted
package fields or layout annotations. The contract must distinguish a full artifact
from an audience projection and refuse unsupported redacted editing/export rather
than reconstruct hidden content or use a view as the complete process.

Process-value protection is still defined by the existing semantic contracts;
authoring metadata privacy and artifact authenticity remain separate obligations.
No native static report, semantic revision, authoring digest or layout grants
provider execution, current actor authority or host access. Synthetic contract
vectors establish only format/consistency/refusal behavior, not production security
or actual representative-user/host/browser/independent evidence.

## Publication and implementation sequence

Publish the exact package/change/layout profile and closed schema(s), raw pins,
selected operation/target matrix, limits, consistency/stale-base rules and worked
positive/negative vectors. Compare component/transport bounds explicitly with real
RP-01/03 sizes. Contract oracles check schema/vectors and real existing native
semantic projection; they do not implement managed rename or a visual editor.

Later signed scopes implement native package admission/inspection and the selected
managed operations, then a paired accessible prototype and real roundtrip evidence.
General operations/IR-only export/work blueprints require their own later complete
profiles. Full RP-08, diagnostics, complete validation/conformance, three-process
cross-form cycles and actual representative-user evaluation remain within Phase1.
No headline box or conditional gate closes; no Phase2 or Phase3 studio is claimed.
