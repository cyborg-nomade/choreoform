<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Static record diagnostic paths: frozen correction plan

**Date:** 2026-11-06<br>
**Baseline:** `fa27c712eaabd9b218a8dc699d353f6934836797` (PR #43)<br>
**Authority:** ADR-0018 delegation; ADR-0033 actionable source locations.

The static checker still returns bare errors from scope policy/input checks,
actor requirements, data protection/initializer/invalidation links and capability
contract/type/authority/effect checks. Reproduce those missing paths before fixing
them. Attach actual existing record/field paths without changing error codes,
evaluation order, validation meaning, resource trust or report authority.

Add a private missing-path annotator that preserves any existing deeper path and
allocates only on error. Locate scope outcomes/policy fields/input members;
actor requirement; data protection/initializer/invalidation members; capability
contract/input/output/authority/effects; and per-scope ancestry validation. Use
existing ASCII wire identity/port keys; never emit aliases or values. Keep global
structure/revision/resource and graph-wide errors global where a precise cause is
not established. Do not manufacture a cycle member or source token location.

Split capability input/output comparisons in the same short-circuit order so
their existing code can point to the failing field. Preserve all current scope,
data, actor, provider and lifecycle conditions and snapshot/source versions.
Source mapping should work through existing prefixes; no additional source scans,
metadata mutation or success-path allocation is needed.

Tests exercise raw IR failures for each affected record/field, both contract
profiles where applicable, stable codes and real pointer targets. Source/native
cases show known newly located errors at original declarations, including
initialized inputs, capability shape mismatch and protection metadata. Preserve
existing exact path/source/hash tests and global-error behavior. Run relevant
native/Wasm and artifact/source/stage checks, review separately, inspect all PR
comments, record delegated review, and require passing final-head CI/expected-head
merge. Keep full diagnostics/catalogue, validator/proof, benchmark/conformance,
text/visual/protected editing and actual user evidence open.
