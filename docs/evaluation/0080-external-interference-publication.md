<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Standalone interference metadata publication checkpoint

**Date:** 2026-11-12<br>
**Frozen scope:** `cba5e61`, [plan](0080-external-interference-resources-plan.md)<br>
**Dependency:** #94 merge `c6ad39959387f08273921f7ba87dc43516a3f385` after
CI `43229278276` / automated COMMENT review/full audit `5875403282`<br>
**Decision:** [ADR-0080](../decisions/0080-external-interference-resources.md)

Normative identity: `urn:choreoform:interference-profile:provider-regions:0.1.0`.<br>
Exact source: `docs/dialects/provider-interference-regions.md`.<br>
Artifact: [sha256-0b10eb4dbb19caace45b93f35dec44aa5f26458d631393932caddb5589210250.txt](../ir/contracts/sha256-0b10eb4dbb19caace45b93f35dec44aa5f26458d631393932caddb5589210250.txt).<br>
Revision: `sha256:0b10eb4dbb19caace45b93f35dec44aa5f26458d631393932caddb5589210250`; 12140 stored UTF-8 bytes.

This signed publication checkpoint precedes Rust reader implementation. The single
artifact defines all closed shapes/type judgments and conditional trust rules;
schemas are validation aids, not moving normative bindings. Draft79/profile/pure
model remain unchanged. New stable resource versions/profile pins are explicit.
Existing SDK registry/work/depth bounds remain, with no reset, rather than adopting
the disposable model's comparison budget/result format. Metadata decoding is not
semantic selection, independence, actual provider/authority/host support or live
request permission. Multiple stored versions are not conflicting semantic selection.

New metadata SDK implementation, positive/negative resource graphs, required
native/Wasm/Python/final-head CI and separate completed-diff automated review/full
comment audit remain to do before expected-head parent merge. Existing production
checking/current benchmarks/old accepted artifacts and every unproved phase gate
remain fixed. No actual observations, independent/human approval or Phase 2.
