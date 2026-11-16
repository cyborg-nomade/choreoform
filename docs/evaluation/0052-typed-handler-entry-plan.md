<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Typed handler-local entry: frozen raw SDK plan

**Date:** 2026-11-07<br>
**Baseline:** `5d7443ee1b60ba49fbfc73c643d905294615fa36`, draft #46 after PR #66<br>
**Authority:** ADR-0018; no actual entry, parent/scenario/phase admission

Use this dedicated stacked branch/PR and ADR-0052. Publish Definition IR 0.3 and
core 0.7 with immutable snapshots, leaving every old wire/schema/core/source/hash
pair fixed. IR 0.3 inherits IR 0.2 wait capture and changes only scope records by
adding required nullable `faultInput:null|{history:Ref,result:DataID}`. Typed parsing
checks exact roles/pins and local ownership; no generic ordinary input port. Core
0.7 selects IR 0.3, requires all six inherited core artifacts, and defines the new
binding. Keep earlier selected cores on their previous supported wires; no implicit
migration, unknown-core fallback or quota increase. Extend only explicit known-core
families and preserve inherited verification/refusal ordering.

History pins the exact ADR-0050 provider-fault-history descriptor and all transitive
profile/source/provider/fixed nominal bytes. Result is a cell owned by that handler
scope of exactly the fixed nominal type. Non-null binding is only for a local-child
fault handler named by a faults policy, never root, ordinary invocation/iteration /
parallel/member body or P6 follow-up. Inspect every declared creator, including
unvisited ones. Preserve zero handler ports. Forbid any ordinary initializer,
ordinary declared writer, competing entry binding or ordinary input/output use of
the reserved history cell. Validate all declarations, even unvisited ones, without
claiming activation or initializing values. Verify source-provider protection refs
with normal handler-context visibility; no private-ancestor exception.

Retain bounded immutable static binding rows with definition revision/handler/cell /
history descriptor/source/profile/provider/type and host premises. Share immutable
metadata/revision rather than copying it per use. They supply no event, actual
history value, receipt, current access, accepted selection or dispatch handle.
The ordinary literal/limited-claim APIs must never promote a fabricated value into
an initialized entry. No new source grammar/companion kind/native command in this
raw SDK slice; explicit readable tool-managed entry integration follows separately.

Add a separate conditional `check_fault_entries` report over one retained
check_fault_origins pass. Preserve existing ordinary and legacy exceptional coverage;
new conditional body sets must be explicitly separate. Do not rerun admission to
construct output or infer values from metadata. Cover only bindings with actual
*declared* incoming supported-source route cases: every incoming route selecting
the handler must have an exact supported general-source use matching its descriptor,
ordinary syntactically reachable source site in ordinary control coverage, and empty
active-before stack. Refuse unsupported human/null/legacy, wrong-source, unvisited
and nonordinary/active-handler contexts as unproved; never assume they cannot fault.
No incoming route/explored context means uncovered binding, not vacuous initialization.
Preserve full before/after context evidence and every nearest-selection/exclusion /
cycle rule. This first subset intentionally refuses nested exceptional source-control
contexts rather than borrowing an ordinary provenance snapshot for them.

For covered bindings, restrict body declarations to portless/childless terminal
flat compute/decision/finish work, all reads/writes local, with a reachable finish.
No activity, wait/timer, invoke/repeat/split/fanout, inherited values, ancestor storage,
parent result or ordinary continuation is permitted by this initial proof. The
history target is immutable; other fresh locals may use inherited initializer rules.
Check history-write access dependencies against the pre-entry empty value set,
before seeding the conditional history cell; reject circular/self/ancestor-dependent
access prechecks. Then check fresh local initialization and all node access/read /
write/finish availability with only the conditionally initialized history cell.

Retain bounded ordinary source-scope protection provenance whenever entry bindings
exist (legacy collection remains off unless capture already requires it). Scope
unions are conservative label upper bounds, not fault-site value snapshots. Require
such provenance for each supported incoming source and include its exact activity
provider/source envelope. Convert source cell/historical origins into immutable
retained labels, never live ancestor values or current mutable dependencies. Preserve
all source-input, fault-existence/control and creator/owner labels; zero inherited
values never implies zero provenance. Reuse or clearly generalize the existing P6
retained-label mechanism without changing its old behavior or serialized evidence.
Missing contexts refuse, and nonordinary source contexts stay unsupported here.

Before returning conditional proof, compare complete retained source/control labels
to history initialization and every subsequent publication/branch, check destination
access dependencies, collect explicit current access obligations and run strict local
invalidation. Reuse bounded graph/flow/strict helpers without relabeling advisory
requirements as proof. History/local read dependencies retain inherited strict
invalidates requirements. Factor error labeling for conditional kinds without
changing old P6 diagnostics. Keep source provenance label requirements separate
from local initialized values, access grants and actual events in the result.

Host premises must explicitly require actual authenticated non-engine emission,
exact definition/instance/scope/node/attempt/cause/input/request/key/effect correlation,
exact live nearest handler/exclusion, current source/work/access/management authority,
fresh single-occurrence read-only initialization, complete provenance retention and
atomic lifecycle enforcement. Actual history immutability, cancellation monotonicity,
owned lifetime/handoff/all-settlement remain mandatory. Handler finish cannot clear
cancellation, resume a parent or supply an activity result. No host/user/browser /
independent execution evidence is generated or claimed.

Test positive exact-source micro-fixtures and explicit promoted copies, known/unknown
wire/core boundaries, all pin/resource/role/ownership/creator/port/writer/initializer
refusals, every incoming unsupported/source/active/reachability case, no-incoming
uncovered bindings, nominal versus fabricated-event limits, zero inherited values,
pre-entry access cycles, restricted-source/public history and branch/initializer
leaks, local invalidation and cumulative metadata/provenance/entry budgets. Retain
old capture/owned-local/general-source/claim/origin/retry/report/digest evidence.
Add licensed finite vectors and independent schema/type/context/protection oracles.
Run native/Wasm/all Python/final-head CI, separate completed-diff automated review,
all threads/CodeRabbit summaries and signed expected-head merge; record progress.

Then explicit readable source/companion/native selection, followed by permitted
owned continuation/cutoff/all-settlement and separate benchmark adoption. The full
RP-01 source 0.7/core 0.4/IR 0.1 package is unchanged; a refused promoted snapshot or
pre-entry access case must be recorded honestly. Parent #46 stays draft/incomplete,
all forty scenarios Partial, complete validator/benchmark/general authoring/visual /
protected editing and actual host/user/browser/independent/phase gates remain open.
