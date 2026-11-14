<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0086: Explicit local type descriptions with bounded transparent expansion

**Status:** Accepted under Project Owner delegation; effective on integration<br>
**Date:** 2026-11-13<br>
**Decider:** Codex under ADR-0018; no personal owner or independent approval

## Context and decision

Signed [scope86](../evaluation/0086-local-type-names-plan.md) `96d021e` follows
ADR-0085. PR #100 merged as `e2ead2ae309730bee9729f08dc67ea39037dd176` after
exact-head CI `43406305606` at `17a83ed6840e38a350c5edfcc755f55c04e48b37`,
automated COMMENT review `5889193632` and fresh full audit. This exact production
contract is frozen/signed before compiler implementation.

Choose `Describe type "Name" as TYPE.` and `type "Name"` with existing lexical
qualification. The explicit type cue distinguishes a type description from data
and expressions; existing `type "Order" from "Order types"` remains the pinned
nominal form. "Structure" could misleadingly exclude primitive/list/optional /
nominal types; a bare caption is less explicit. The wording follows the selected
plain-English direction, without claiming actual user testing or final syntax.

Select source 0.18 / companion 0.17 on unchanged C12 / IR 0.4. Each source-only
managed `type-alias` expands to the exact complete existing AST at every type
position, without creating or erasing nominal membership. Definitions are collected
before uses; validate unused/forward definitions and cycles, bind exact scoped
resource/outcome provenance, and preserve use-site visibility and all old behavior.

| Frozen bytes | Digest |
| --- | --- |
| [candidate-a-v0.18.ebnf](../../docs/authoring/candidate-a-v0.18.ebnf) | `sha256:37a3b2569f623b073499a3c8915bddb46b34f2c819c8463d15e3f383305cf01e` |
| [profile-v0.18.md](../../docs/authoring/profile-v0.18.md) | `sha256:b5134f67a63b4793611a07bb1941b778cdecb991903f0ccdfcb0a22365ed2808` |
| [bindings-0.17.schema.json](../../schemas/authoring/bindings-0.17.schema.json) | `sha256:92feb8c2a81e38c13d143cf01d13b8d7ec5666eeb24a361ba3337cc1d49f574b` |

Publication hashes do not replace semantic core artifacts. Before building cache
values, preflight the complete acyclic graph's expanded type/container depth,
node/work and compact-byte costs with checked arithmetic. Charge caches and repeated
uses cumulatively, sharing named protection/lowering/dependency/expansion budgets.
Short acyclic source alone is no bound. Immutable descriptions contain no live
values/access truth; failure emits no partial candidate. Do not raise quotas.

## Consequences and validation

Local descriptions add identity/version/inspection complexity. Preserve explicit
rename/reorder/fresh-copy/tombstones, annotations/integrity and exact nested nominal
pins/tags/fields/scales/list/optional/wrap meanings. All source/binding/resource /
old-version/privacy/cycle/depth/work/output controls and actual whole RP-01/RP-03
C12-body/revision/format checks must pass. Measure genuine source benefit against
protection-only variants including costs; no savings or comprehension is assumed.
No current/historical benchmark/resource/guard/receipt/generator/report bytes change.

Benchmark adoption/archive, parameterized work/predicate reuse, C13 source selection,
general safe editing/export, protected accessible text/visual inspection, full
RP-08 and actual representative users/hosts/browsers/independent evidence remain
open in Phase 1. All semantic/region/exit gates remain explicit. Required final-head
native/Wasm/Python checks, separate automated completed-diff review/full comments /
CodeRabbit audit and expected-head signed merge precede progress. Parent #46 stays
draft; no permission, runtime, phase completion or Phase 2 follows.
