<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Completion access dependency declarations: frozen follow-up plan

**Date:** 2026-11-05<br>
**Baseline:** `89d648266702d13746ffd4a7b9c04c5998994b59` (PR #25)<br>
**Authority:** ADR-0018 delegation, implementing existing ADR-0013/0017/0019 rules.

At an ordinary finish, scope outputs are validated/transferred under their access
metadata. Include their protection/participant/capability policy data dependencies
in that finish node's required read declaration and source-derived fixed point.
Do not add fresh or unrelated value reads or invent another scope's visibility.
Keep value/port transfer checks and host authorization separate.

Use the existing protection-policy reference expansion for scope output cells.
Static checking must enforce this for all declarations, including uncalled child
scopes; ordinary availability analysis then checks declared reads on visited paths.
Source lowering must derive the same metadata closure, including protection on
data read by those predicates. No new syntax, policy truth inference or execution
permission follows.

Required evidence: missing completion read declarations, lexical ancestor and
transitive metadata dependencies, unused-child coverage boundaries, empty-output/
metadata cases, source derivation and format/reparse/revision stability. Preserve
old tests, required native/Wasm/oracle CI and bounded traversal. Review separately,
record disposition/verification and merge at the expected head under delegation.
Then continue lifecycle/race/invalidation and supported-clock work; no phase gate
is closed by this narrow declaration fix.
