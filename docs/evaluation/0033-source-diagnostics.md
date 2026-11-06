<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Semantic source location evidence

**Date:** 2026-11-06<br>
**Plan:** [0033](0033-source-diagnostics-plan.md), `15aa0c2`<br>
**Decision:** [ADR-0033](../decisions/0033-semantic-source-locations.md)

Candidate source lookup now requires a known record table and existing row before
returning its immutable original declaration/generated span. Borrowed path parts
and indexed maps avoid source scanning, alias searches and literal copies. The
native adapter uses one formatter for all ten semantic commands, preserving code/
IR-path prefixes and appending a known byte range only. Global/external/missing
record failures have no guessed fallback. No error meaning, report authority,
source/core version, dependency, hash or original span storage is changed.

Four Rust groups cover actual static/control errors; generated initializer,
assignment and flow constructs; missing/global/resource/wrong-table paths and
existing-record missing suffixes; and multibyte original offsets with fresh
locations after format/reparse of identical IR/error. Five Python groups cover
native static/control UTF-8 locations, lifecycle/follow-up scopes and steps,
generated-expression assignment errors without leaking a marker value,
format/reparse locations and documentation/licenses.

Passed 246 Rust tests plus five compile-fail doctests, native format/lint,
frontend Wasm lint/release build and all relevant artifact/source/stage oracles.
Logs: `/private/tmp/source-diagnostic-rust.log`,
`/private/tmp/source-diagnostic-clippy.log`,
`/private/tmp/source-diagnostic-oracles.log`.

Separate completed-diff automated self-review checked table/row isolation,
record-prefix semantics, original UTF-8 range integrity, generated constructs,
unlocated errors, shared native formatting, stable codes/paths, literal privacy,
indexed bounded work and unchanged revision/profile semantics. No actionable
finding remains. Accepted under owner delegation; require passing final-head CI
and expected-head merge. No independent or individual owner review is claimed.

The [guide](../authoring/diagnostics.md) is a selected interpretation aid, not a
complete catalogue. Locations identify declarations, not exact erroneous fields
or tokens; caller-supplied Diagnostic provenance is not authenticated. Reparse
after edits. Bare semantic scope/data/actor/capability validation errors still
need more precise IR paths before source lookup can locate them. Address that
concrete diagnostic gap next, retaining all full validator/exceptional/follow-up
proof, benchmark, text/visual/protected editing and actual user gates.
