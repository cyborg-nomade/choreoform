<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0080: Decode standalone provider interference metadata without selecting it

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-12<br>
**Decider:** Codex under ADR-0018; no personal owner or independent review claimed

## Context

Signed [scope80](../evaluation/0080-external-interference-resources-plan.md)
`cba5e61` precedes contract/implementation. #94's inactive draft merged as
`c6ad39959387f08273921f7ba87dc43516a3f385` after CI `43229278276`, automated
COMMENT review/full audit `5875403282`. Its thirteen model groups exercise a
conditional finite rule; they do not implement provider/input/authority truth.
Current RP-03 still refuses the actual reservation pair. Existing resource readers
already validate exact P3/V1/V2 metadata using shared budgets/caches.

## Decision

Publish the [standalone profile](../dialects/provider-interference-regions.md)
under a new exact identity and pin before implementation. Adopt draft79's closed
descriptor/namespace shapes, original-request selector type judgments, exhaustive
or opaque channel claims, exact provider/class/profile/authority constraints and
trust/correlation/uncertainty/ownership boundary. New resource version is 0.1.0;
no draft artifact is reinterpreted or accepted implicitly. Schemas aid validation;
the single frozen profile document fully defines the normative shapes and rules.

Add bounded immutable Rust Resources metadata APIs, not definition binding or
concurrency admission. Reuse the complete existing provider/type/invariant-static
checks and supplied exact authority contract bytes. Verify integrity without
claiming that an arbitrary authority contract is trusted or implemented. Both
finite channels and all their selector paths are checked. Borrow type paths and
share immutable decoded structures rather than cloning the whole provider input
for each selector. Failed parents are not cached as successes.

Make the prototype-to-SDK differences explicit: existing 256-resource/8MiB-registry /
1MiB-per-resource/depth64/1000000 shared-work quotas remain; new field/footprint/key /
selector bounds are retained. No 1024-byte model result or actual comparison API
is added. Registry loading may retain multiple versions; unique exact-provider
descriptor **selection** belongs to the next semantic binding contract. The pure
model enforced uniqueness for one comparison, which is a different operation.

## Alternatives and consequences

| Option | Assessment |
| --- | --- |
| Add fields to P3/core/source now | Rejected: changes accepted meanings and conflates metadata with proof |
| Publish only another assumed model | Insufficient: real V1/V2/provider validation remains unimplemented for these resources |
| Standalone exact resource decoding, then separately version semantic binding | Selected: inspectable actual Rust evidence with explicit limits |
| Introduce a second budget/callback normalizer/runtime lock API | Rejected: reset/portability/trust risks and no actual host enforcement evidence |

These resources add no live values, authority, physical alias mapping, actual
occurrences or generic independence proof. A present-Option type judgment does
not prove the eventual value present. New semantic binding must enter definition
revision/dependency closure under an explicit new core version, preserve old-core
refusals and enforce unique selection. Generic separation and managed near-English
source/raw integration follow separately, or existing refusal remains.

Confirm with full SDK resource graphs, class/profile/pin/type/selector/authority /
nominal/unknown/depth/work/caching/failure cases, native/Wasm parity and independent
artifact/schema/source-refusal checks. Preserve every accepted artifact and
benchmark/history/generator/report byte. Run required final-head CI, separate
automated completed-diff review and full comment audit before signed expected-head
merge. Parent #46 and all unproved Phase 1/real external-evidence gates remain open.
No Phase 2. Actual artifact digest is recorded in the registry and evaluation
publication checkpoint before Rust reader implementation.
