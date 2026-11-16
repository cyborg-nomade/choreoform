<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Explicit timerless wait-capture evidence

**Date:** 2026-11-07<br>
**Plan:** [frozen raw-contract scope](0045-wait-capture-plan.md), `0e01746`<br>
**Decision:** [ADR-0045](../decisions/0045-explicit-timerless-wait-capture.md)

Definition IR 0.2 adds only the required nullable wait capture object, with exact
external source, NodeID origin and DataID result roles. The old constructor and
IR 0.1 schema/fixtures remain fixed. The explicit versioned constructor performs
transport admission; structural projection still supplies no semantic admission.
Canonical semantic hashing already includes format/version/kind/body, so selecting
another wire version changes the revision without changing the old algorithm.

Core 0.5 is published as exact bytes at
`sha256:50007ac05e5e71df91bb9997a40a06c7a8993df04b1a673421e4a987014299d7`.
All four inherited core artifacts remain explicitly required. Every declaration,
including ordinary-unvisited ones, checks actual lexical visibility, capability
origin, exact provider ID/revision, verified terminal source/profile/type bytes,
exact normalized/wait/destination types and singleton writes. Intrinsic P2
source-to-destination comparison prevents destination-label declassification even
on an unvisited declaration. Ordinary protection additionally propagates the
provider source/control envelope before publication and dependency snapshots.
Source/destination access references feed inherited conservative complete reads,
writer inventories and strict invalidation; no private visibility exemption exists.

Only timerless sole-onObservation completion can initialize this subset. Existing
ordinary definite availability rejects reads before capture and permits reads
after accepted completion. This is a conditional static proof, not a host accepting
an observation. Metadata origin links add no control edge or live activation.
Actual authenticated source/correlation, current authority and access, atomic
single acceptance, duplicate/contradiction history, cancellation/closure cutoff,
live owned storage and all-obligation settlement remain explicit requirements.

The immutable `DeclaredWaitCapture` API exposes revision/site/source/profile/
provider/origin/result plus those requirements, never an observation/value or
execution handle. Composed reports retain the same static report; its new getter
avoids rerunning validation. Declaration rows share one revision and immutable
source metadata, and all nonempty row construction uses the existing resource
work allowance. Legacy empty rows add zero work; exhaustion cannot return partial
rows. An initial 421-capture/2,500-field probe exhausted the allowance by repeatedly
parsing the same immutable observation type. Memoizing only successful type
equality by policy/cell and exact provider/profile lets that probe pass under the
unchanged quota. Every use still checks actual visibility, provider, writes,
protection and scoped access references; distinct types and nominal pins still
refuse. The cache stores no authority, live value or activation judgment.

Five raw-wire groups cover version selection, unchanged legacy projections/all ten
variants, typed roles, closed fields, cross-version refusal and source/ref shapes.
Fifteen semantic integration groups cover exact pins and inherited contracts,
actual nominal types, exact provider revisions, wrong/private/human origins,
unvisited declarations, missing/extra writes, read-before-capture, invalidation,
source/control laundering, nonempty timers, finite versus legacy metadata cycles,
shared large schemas and all seventeen finite corpus cases. One internal group
checks cumulative declaration-budget refusal and unchanged legacy empty cost.
Four Python groups independently check the schema delta, exact profile/hash,
seventeen finite binding/type/ordinary-DAG/invalidation judgments, closed versions
and the unchanged current package/source refusal. The finite model assumes its
prevalidated noncapture fixture; it is not a complete validator or runtime.

An actual full RP-01 copy promoted only for this raw test preserves nominal payload
identity. Adding a local captured result passes static and strict invalidation,
but concurrency refuses `scope-lifetime-unproved` at `/body/nodes/run_payment`.
This is the expected remaining owned-storage gap, not a completed reconciliation
scenario. The committed benchmark remains source 0.7/core 0.4/IR 0.1, with unchanged
source, IR, resources, guard snapshots and all ten partial-stage reports.

Local verification passed **352 Rust tests plus six compile-fail doctests**,
format/native workspace lint, all five library Wasm lints and release portability/
frontend builds. The shared structural suite now has 97 cases; native independent
checks pass 17 JCS/hash comparisons, all three original fixtures/hashes, ten node
variants and nine CLI operations. All required Python checks, including the new
four groups and exact benchmark regeneration/historical probes, pass. Compiling
Wasm is not actual browser execution evidence. Logs are local transient files:
`/private/tmp/wait-capture-{workspace,clippy,wasm,native,oracles,tests,budget}.log`
and final test/lint logs. Final-head CI and PR comment audit remain required.

Separate completed-diff automated self-review checked version/closed-shape/ref
boundaries, all inherited pins and old hashes, provider identity versus live truth,
nominal/type cache keys and per-use checks, protection/control/invalidation order,
timerless initialization, resource amplification and cumulative budget refusal,
report authority boundaries, licensing and test/evidence scope. The repeated-type
work issue was fixed and stress-tested; no actionable finding remains. This is
delegated agent self-review, not independent or individual owner approval. Inspect
all unresolved threads and CodeRabbit summaries before the signed expected-head
merge; a skipped bot review supplies no evidence.

Next freeze explicit source/companion support and bounded native declaration
output. Owned local storage, fault data, permitted owned continuation/cutoff and
settlement need separate proofs before full benchmark adoption. Parent #46 remains
draft/incomplete; all forty scenarios remain Partial, all phase gates stay open,
and no actual user/host/browser or independent review evidence is supplied here.
