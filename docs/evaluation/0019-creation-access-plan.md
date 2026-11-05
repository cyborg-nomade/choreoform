<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Scope creation read footprints: frozen follow-up plan

**Date:** 2026-11-05<br>
**Baseline:** `20a94148a09ee9e5d75ddbe2d4c1c9aa4f50dec7` (PR #27)<br>
**Authority:** ADR-0018 delegation, implementing ADR-0013/0017/0019 dependency rules.

Before concurrency analysis, make child-creation dependencies visible on the
creating invoke/repeat/split/fanout declaration. Initializer expressions and
initial/input access metadata can read lexical ancestor cells even when no child
node reads them. Missing declarations would hide a shared read from later race
analysis. Require the conservative ancestor read footprint at every creator and
derive it in source lowering. Exclude fresh child-local values from caller reads;
they have distinct storage occurrences and remain checked by child initialization
availability. Do not guess a local initializer order or change snapshot semantics.

Build a per-scope footprint from indexed inputs/initialized cells and their
expression/access dependency closures; avoid scanning all data for every creator.
Reuse metadata closure logic and charge visits/member copies to the static work
budget. Preserve lexical visibility, explicit access obligations and immutable
report boundaries. No new wire fields, dialects, syntax or host permission.

Evidence: missing creator reads for all four creator kinds; positive ancestor
dependencies; initializer source and destination metadata, transitive policy
reads, local input exclusion and uncalled declarations; source derivation plus
format/reparse/revision stability. Run relevant tests and native/Wasm/oracle
checks; self-review separately and verify final-head CI before expected-head
merge. This fixes a prerequisite and does not itself prove concurrency safety,
runtime atomicity, exceptional availability or any aggregate Phase 1 gate.
