<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Scope creation read footprints: evidence

**Date:** 2026-11-05<br>
**Decision:** Existing ADR-0013/0017/0019 dependency and availability rules<br>
**Plan:** [frozen scope](0019-creation-access-plan.md), `324faa2`<br>
**Baseline:** `20a94148a09ee9e5d75ddbe2d4c1c9aa4f50dec7` (PR #27)

## Result and boundary

Every invoke/repeat/split/fanout declares the lexical ancestor reads needed by
its child initialization. The footprint includes initializer expression reads,
input/initializer destination access policies, source access policies, and their
transitive data-policy dependencies. Fresh local values remain occurrence-local
and do not become caller reads. Availability still checks input creation against
the ancestor snapshot and initializers against entry inputs/ancestors; no guessed
local initialization order or shared child storage is introduced.

Static checking enforces declarations even on uncalled creators. Source lowering
derives them, without requiring authors to add `Also read` statements. Footprints
are indexed per scope with one data scan and a visited metadata closure, then
copied to each creator under work charges. Static checking uses validated cached
protection references and the existing cumulative budget. The new source pass
charges metadata members, closure visits and creator copies against `MAX_WORK`.
No public API, wire, syntax, dependency or contract version change.

This prevents a dependency omission before conflict analysis; it does not prove
any conflict safe, make a creation transaction atomic, authenticate a principal,
authorize a read or settle work. Current reports remain partial and non-executable.
Clocks, exceptional availability, full benchmarks/conformance, source/visual
editing and actual user evaluation remain open at the Phase 1 checkpoint.

## Regressions and verification

Four new control test groups cover all four creator kinds; missing/available/
unavailable ancestor initializer reads; source access metadata; transitive
destination access metadata; an uncalled creator; and a child input feeding a
local initializer without a false caller-local read. One new source group checks
both value and destination-policy dependencies, missing direct-IR declarations,
and stable candidate revisions/reports after formatting and reparsing.

The existing source input fixture is shared by the two source tests. Historical
examples, frozen snapshots, original artifact revisions and existing checks are
unchanged. Separate automated self-review checks closure completeness, occurrence
identity, diagnostics, cache lifecycle and bounded work; no independent human or
runtime evidence is claimed. Required final-head CI remains a merge prerequisite.

Local verification passed 138 Rust tests and five compile-fail doctests, format,
native lint, semantic/frontend Wasm lint and release frontend Wasm compilation,
plus authoring evidence/frontend, static, control, protection and lifecycle Python
oracles. The completed-diff self-review confirmed per-scope cache validity,
fresh-local exclusion with child entry checking retained, direct-IR declaration
refusal, source parity, and no runtime authority claim. Work accounting includes
both initializer dependency copies, metadata members before set expansion, and
every footprint copy into a creator's read set. No open review finding remains.

Reproduce with workspace tests/format/lint, semantic/frontend Wasm lint and release
frontend build, and the existing authoring/static/control/protection/lifecycle
Python checks in the [CI workflow](../../.github/workflows/rust-portability.yml).
Continue with concurrency/invalidation after merge; no phase gate closes here.
