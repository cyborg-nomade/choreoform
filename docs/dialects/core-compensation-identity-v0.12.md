<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Core profile 0.12.0: purpose-only prior activity identity

**Contract ID:** `urn:choreoform:contracts:core-profile:0.12.0`

These exact bytes form the artifact. Require all eleven exact inherited cores:

- `urn:choreoform:contracts:core-profile:0.11.0` / `sha256:246bf3835eef25f8adf76bce39e6b887715e098761e675f821bf745a8111d29e`.
- `urn:choreoform:contracts:core-profile:0.10.0` / `sha256:74c3d5bfe2a880259bc13f521127aa1dd84977ae9e6f83b339e3a0629964e60f`.
- `urn:choreoform:contracts:core-profile:0.9.0` / `sha256:5faaf24217bcc5644a9ce5975b2c6ad7074d0197d39a73ab648d6cb3a9ced87f`.
- `urn:choreoform:contracts:core-profile:0.8.0` / `sha256:edc8195ba0c31536ab2cdc95089e8aecc3ff402489639b4435277f0a38e7b603`.
- `urn:choreoform:contracts:core-profile:0.7.0` / `sha256:e286e54441b3d785ae823eef359c12d64fb8405829c70709227282e5af62f54b`.
- `urn:choreoform:contracts:core-profile:0.6.0` / `sha256:27bce3fde9a0393abb6bf760a8588af9268de65069b73f4cb946144f5d30cb85`.
- `urn:choreoform:contracts:core-profile:0.5.0` / `sha256:50007ac05e5e71df91bb9997a40a06c7a8993df04b1a673421e4a987014299d7`.
- `urn:choreoform:contracts:core-profile:0.4.0` / `sha256:0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3`.
- `urn:choreoform:contracts:core-profile:0.3.0` / `sha256:30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3`.
- `urn:choreoform:contracts:core-profile:0.2.0` / `sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623`.
- `urn:choreoform:contracts:core-profile:0.1.0` / `sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58`.

Select existing IR 0.4 with uniform exact selected core aliases. Inherit all bases,
superseding only P1's lexical visibility for the optional `compensates` identity.
Unknown versions, wrong digests, missing bases and mixed aliases refuse. Earlier
core/source/wire/schema/artifact/hash/report meanings and all budgets stay fixed.

## Static identity, not private access

When non-null, `compensates` is an exact ID in this definition's `nodes` map. The
target must be an existing capability activity (`kind:activity`, `mode:capability`),
independently of the work policy's lexical scope. A global ID from another
definition/resource is not resolvable here. No dynamic/external target lookup,
new field, callback, path syntax or wildcard is introduced.

This exception applies only to that P1 identity slot. It grants no target data,
result, outcome, expression, policy, actor, capability, scope-entry or control-flow
access. All ordinary lexical/type/dependency/protection/recipient/availability /
invalidation rules remain mandatory. Target effect metadata does not change the
compensator's own effect class. Class and compensation purpose remain independent.
Static same-site identity does not prove a prior dynamic occurrence, temporal
precedence, actual effect or valid self-compensation dispatch.

## Existing conditional proofs and live boundary

Inherit core 0.11's finite owned-work proof unchanged: any optional compensation
link in that family must still name exactly its immediate incoming source activity.
Self, unrelated and other-ancestor targets remain refused by that proof, even when
the static identity resolves. Preserve finite acyclic actual-use ownership,
capture-first command/compensator entry, metadata/provider/source/typed-input /
protection/provenance checks, readonly original-request seeds, empty pre-entry
access, local availability/invalidation and all ordinary/legacy/old-flat meanings.
No general aggregate compensation/parent or sibling storage access is admitted.

The host must bind the precise actually eligible prior effect/occurrence and exact
request/input/basis/value revisions; verify current authority and any required
accepted correlated capture; create a NEW logical effect and key rather than reuse
the inverse operation's prior key; preserve immutable authenticated provenance and
actual accepted registration/receiver/cutoff/atomic handoff/all-transitive settlement.
Unknown facts are not absence. Static identity grants no values, provider truth,
accepted event, execution, restoration, erased effect, current authority or settlement.

Cache profile selection once, use the existing bounded node map, and charge each
new-core identity lookup to the same cumulative semantic resource budget. No new
unbounded scan, independent budget reset or partial report publication. This is
raw static/conditional SDK support only; readable/native selection and benchmark
adoption need separately explicit versions/plans. Complete validator, full cases,
near-English/protected accessible text-visual/actual user/host/browser/independent
and phase evidence remain open; no Phase 2 execution follows.
