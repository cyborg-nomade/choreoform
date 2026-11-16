<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Owned local follow-up proof: frozen dependency plan

**Date:** 2026-11-07<br>
**Baseline:** `9a4a39f7bf9fa1080dca697d5f66b94fad3e86cf`, draft #46 after PR #60<br>
**Authority:** ADR-0018; raw capture/source support is not a lifetime proof

Use a dedicated stacked dependency PR and ADR-0047. Preserve wire/core/source
artifact bytes and all old data-free isolated-follow-up behavior. Prove a new
explicitly separate conditional subset, not actual allocated/durable storage:
a covered synchronous invoke/repeat's reconcile closure points to its direct child
follow-up, with no nested scopes, no ports and terminal closure. Its entry is an
explicit core-0.5 timerless capturing wait. All declared body nodes, including
unvisited ones, are flat wait/compute/decision/finish; all waits are timerless,
and every complete node read/write is confined to that follow-up's own local cells.
No ancestor snapshots, foreign writes, activities/effects, nested calls/forks,
reopening or independent concurrent creator is admitted. Preserve required normal
static visibility/provider/type/protection checks and at least one reachable finish.

Retain existing data-free evidence as its original type. Give the new subset a
separate private-constructor immutable revision-bound report with creator/body /
closure/follow-up/local-cell/capture-site declarations, conditional coverage,
required access/invalidation judgments and explicit live P6 premises. Share cached
immutable proof templates by body, not live occurrences/values/authority; charge
indexing, walking, copies and report rows to existing stage work allowances.

Storage isolation alone is insufficient. Before returning the new proof, check
all candidate follow-up entries together with zero inherited values and fresh
local initialization, using existing bounded control analysis. Reuse the protected
flow/dependency engine on those explicit entries with empty inherited origins,
retaining every source/control envelope and actual access reference. Require strict
conditional local invalidation, with ordinary and conditional coverage separately
named; never promote advisory missing links. Do not call public composed checkers
recursively or rerun static admission to obtain disclosure. Preserve all ordinary
covered/uncovered sets and reports; exception/observable-channel/general lifetime
and all actual current access/atomic acceptance obligations remain explicit.

Expose the new evidence through concurrency and all composed later native views.
Omit new keys when absent, preserving legacy output. Bound per-row accumulation
and final combined 1 MiB JSON before stdout; never return partial rows/JSON on work
or output exhaustion. Existing capture/retry/historical/isolated views remain.
No report grants actual host ownership, activation, retained ancestor values,
accepted observations, fault-data access, dispatch or terminal settlement.

Test raw and readable capture->local-read/branch/compute->finish, true source /
control no-declassification, source/destination access reads and strict missing
conditional invalidation, read-before-capture, initializer/ancestor dependency
refusal, ordinary-unvisited hazards, foreign readers/writers, timers, nested scopes,
ports, effects/forks and ordinary resumption. Keep fresh initialized noncapture
storage outside this subset and preserve old refusal tests. Exercise repeated
creators/shared templates, cumulative work/report limits and exact revisions.
Use finite independent Python isolation/initialization/protection/invalidation
vectors, not generated events or a reference engine.

Explicitly test a promoted full RP-01 copy with a local terminal result, retaining
all real nominal identities and original guards; any newly passing stage still
carries full P6 host/source/correlation/current authority/cutoff/settlement premises.
Replace historical lifetime refusal expectations only where this exact new proof
applies, and retain adjacent negative lifetime cases. Checked-in package stays
source 0.7/core 0.4/IR 0.1 until separate reviewed adoption and remaining contracts.

Run relevant native/Wasm/all Python/final-head CI checks, separate completed-diff
agent review, every unresolved thread/CodeRabbit summary and signed expected-head
merge into draft #46. Update durable progress with actual next fault-data/owned
continuation/cutoff/all-settlement dependencies. Full validator/benchmarks,
plain-English/tool-managed/visual/protected editing, actual host/browser/user and
independent review gates stay open; no scenario or phase exit is inferred.
