<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Local transparent type implementation evidence

**Date:** 2026-11-13<br>
**Scope:** signed `96d021e`, [production plan](0086-local-type-names-plan.md)<br>
**Exact contract before compiler:** `7d90b734ac8ba596773782541e3610f9ae843f87`,
[ADR-0086](../decisions/0086-local-type-names.md)

## Actual source and unchanged meaning

Production Rust parsing/resolution/lowering supports source 0.18 / companion 0.17 on
unchanged C12 / IR 0.4. Explicit `Describe type ... as ...` definitions and `type ...`
references expand transparently at every existing type position. The existing
pinned nominal `type ... from ...` form stays distinct. No new semantic type,
nominal membership, work/storage/endpoint, permission, resource pin or runtime is
created. Source-only IDs remain explicit in the managed companion.

All definitions resolve before uses, including unused and forward references.
Preflight rejects cycles and computes full expanded type/JSON-container depth,
JSON work/nodes, bytes and exact resource/outcome provenance before allocating
cached descriptions. One traversal-depth guard spans both structural nesting and
forward alias resolution, preventing independent counters from multiplying call-stack depth. Cached definitions and repeated uses share cumulative bounds
with named protection/dependencies; a short acyclic DAG is not treated as safe
merely because its source is finite. Use-site visibility remains enforced.

## Actual whole-process comparisons

Test-only native span transformations derive new variants without writing any
current or historical benchmark. The largest repeated outer compound type without
outcome references is explicitly named; the test does not claim a general safe
editor/exporter or automatic hoisting algorithm. Resource scope must still bind.

| Finite test source | Protection-only bytes | Combined type/protection bytes | Additional reduction | Reduction from current original |
| --- | ---: | ---: | ---: | ---: |
| RP-01 | 56059 | 55702 | 0.64% | 9.00% of 61211 |
| RP-03 | 448421 | 412288 | 8.06% | 28.52% of 576759 |

Both actually parse/lower/format/reparse with the exact protection-only C12 document
and pass static, control, protection and strict invalidation. RP-01's baseline
explicitly promotes its current C11 source to C12; its revision remains
`sha256:cd0a7b6923ccc8a0afa31a27fca3aff280c3da601089ba12946f2d768fa1a829`.
RP-03 remains exactly its current complete document and revision
`sha256:a3b8c0f07cfc63d745376c3eb11b67690c98edc3e216d262c1542f00a05798aa`,
with `parallel-effects-unproved` unchanged. Separate observation-type alias variants
also preserve exact complete documents/static checking in both whole cases.
Provider/guard/receipt/resource meanings stay fixed. The measurements include
naming overhead and assert exact byte counts, but do not establish comprehension
or actual user accessibility. Current benchmark adoption/archive remains separate.

## Tests and finite boundaries

Twelve Rust groups cover scalar/nested structural/outcome descriptions, forward
use, unused invalid/cyclic/duplicate/scale/missing definitions, exact whole nominal
body/resource preservation, purpose/protection coexistence, parameter/result /
literal/initializer/observation/capability type positions, no manufactured nominal
membership, lexical ancestor/private/sibling/wrong-kind use, explicit rename /
reorder/fresh-copy/tombstone/stale/surplus IDs, annotations/format/integrity, every
old pair and genuinely future refusal, and tiny DAG/deep/wide/cumulative expansion.
A standalone parameterized expression retains its existing `parameter-binding`
refusal; local type spelling does not invent a call context. Four independent
Python groups verify frozen hashes/closed schema/actual grammar, real CLI/all 11
small non-executable views and integrity cycles, cycle/kind/stale/future/private
refusals with no literal echo, and tiny-DAG refusal with empty stdout.

The shared 100000 lowering/dependency budget charges preflight visits/provenance,
complete cache/use costs and protection expansion. Compact expanded cache/use bytes
share 1 MiB, and emitted-record/final wire/input/companion bounds remain 1 MiB each.
Expanded type/container depth and traversal use 64, tokens 131072, parser 1M and all
semantic/shared limits stay fixed. Cached values are built only after all graph /
cache reservations succeed, in dependency order. A tiny acyclic graph cannot
allocate exponential output; <100000-byte sources separately hit repeated-use work
and byte ceilings without candidates. Immutable types cache no live access truth.

Full required final-head workspace/compile-fail/native/Wasm/release/native-
portability/Python validation, separate completed-diff automated review, complete
review/thread/CodeRabbit audit and exact-head signed merge remain required before
integration. A skipped bot supplies no review evidence; no human/independent
self-approval is claimed. Existing accepted artifacts and current/historical
benchmark/corpus/generator/guard/receipt/report bytes stay fixed.

## Remaining Phase 1 obligations

Transparent naming is not parameterized type/work/predicate reuse, C13 source
selection, full RP-08, general safe editing/export or protected accessible visual /
text inspection. Real representative users/hosts/browsers/independent evidence,
physical alias/canonicalization/original snapshot/revised-surviving-work proof and
all remaining semantic/phase gates stay open. Parent #46 remains draft; no actual
permission/execution, full-language/phase completion, Phase 2 or new phase chat.
