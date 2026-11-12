<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Explicit interference binding: publication before implementation

**Date:** 2026-11-12<br>
**Frozen scope:** `aa4d58f`, [plan](0081-interference-definition-binding-plan.md)<br>
**Dependency:** #95 signed merge `78f5d706756642932c57c33672903a7b9ccdcacc`,
CI `43233888435`, automated COMMENT review/full audit `5875818566`<br>
**Decision:** [ADR-0081](../decisions/0081-interference-definition-binding.md),
following separately signed [critique](0081-interference-binding-critique.md)

Core identity: `urn:choreoform:contracts:core-profile:0.13.0`.<br>
Revision: `sha256:dd8667135d1c801683d48d8410867b37d5d6f71c7016b8b716016bce92e77428`; 8270 exact UTF-8 bytes.<br>
Artifact: [sha256-dd8667135d1c801683d48d8410867b37d5d6f71c7016b8b716016bce92e77428.txt](../ir/contracts/sha256-dd8667135d1c801683d48d8410867b37d5d6f71c7016b8b716016bce92e77428.txt).

This signed publication checkpoint precedes core reader changes. Existing IR0.4
is retained; only a new dialect version gains the mandatory nullable descriptor
Ref in capability-contract policies. All wrappers of one exact provider agree on
null or the exact same descriptor. The artifact requires all twelve old cores
plus standalone profile `sha256:0b10eb4dbb19caace45b93f35dec44aa5f26458d631393932caddb5589210250`.
Old profiles/artifacts/source/report and benchmark/history/generator bytes stay
fixed. Immutable resource closure and declared selection are not generic external
independence or actual authority/host truth. New-core effectful controls must
still refuse concurrency. Source and generic proof need separate signed scopes.

Raw SDK implementation, resource/selection/revision/old-new/inheritance/budget /
privacy controls, full required final-head native/Wasm/Python checks and automated
completed-diff review/full comment audit remain before signed parent merge. No
personal owner/independent/human approval, external observations or Phase 2.
