<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Owned continuation, cutoff and settlement: frozen contract audit plan

**Date:** 2026-11-08<br>
**Baseline:** `e5a975af1ac1a5dad2a4cf6a5be506dc331e36f6`, draft #46 after PR #70<br>
**Authority:** ADR-0018; no actual new work, current grant or host evidence

Use a dedicated stacked audit PR and ADR-0056 before extending accepted contracts.
No production behavior/wire/core/source/artifact/package changes in this audit.
Original request retention is now raw/readable and conditionally proven, but the
owned subset is flat, terminal and wait/compute/decision/finish only. Every actual
capability use requires its own designated reconciliation child in later link
checking. Relaxing just the node-kind test would bypass that dependency and the
live transitive lifetime/access/cutoff/settlement requirements.

Read accepted Manifest/P1–P7/IR/core contracts and existing static/control /
protection/invalidation/concurrency/timing/fault/reconciliation implementations.
Trace declaration versus ordinary/conditional coverage, request value identity,
source/control/provider provenance, current authority, cancellation propagation,
registration/receiver acceptance, ordinary closure versus terminal disposal,
subscription/effect cardinality, duplicate/conflict handling and all-settlement.
Do not treat definition IDs, metadata hashes or reports as live receipts or grants.

Build small genuine SDK/native declaration probes for a late-capture follow-up
attempting new read-only capability work: terminal flat owner; nested designated
recovery child; missing original request; ancestor-value/ordinary-parent shortcut;
command/irreversible variant. Record actual admitted stages/refusal codes and source
locations. A successful earlier stage does not bypass later contracts or authorize
execution. Probes are synthetic definitions with explicitly declared protection /
provider metadata, not requests, observations, host storage or generated receipts.
Do not modify accepted resource bytes or relabel RP-01 to make a probe pass.

Compare alternatives: flat same-scope settlement with new activation rules;
explicit finite nested surviving ownership; staged read-only queries before effects;
ordinary-parent resumption; treating a true guard or finish as all-settlement.
Reject routes that drop source/control protection, invent inherited values, reuse
old grants, clear cancellation, hide outstanding obligations, duplicate ownership,
or claim a callback can reopen a terminal parent. Explain whether a new wire is
actually necessary or existing explicit scope/closure records suffice under a new
versioned core; preserve old contracts either way. Avoid speculative implementation.

Publish a concrete critique/audit and durable ADR-0056 decision under delegation,
with supported next subset, exact dependencies, mandatory negative cases, budgets,
proof/report separation, live host premises and remaining gaps. Define a reviewable
implementation plan separately and sign/freeze it before production changes; the
next PR may implement it after this audit merges. Initial preference is read-only
owned work with explicit finite ownership before command/compensation generality,
subject to actual contract/probe findings, not an inferred approval to broaden roles.

Run relevant probes, unchanged package checks and required final-head CI; separately
review the completed diff, audit every unresolved thread/CodeRabbit summary, and
merge at the expected signed head. Update progress and the draft parent. Continue
independent current-phase work after the audit without per-item approval. Complete
validator/benchmarks/general tool-managed English/protected accessible visual editing
and real host/browser/user/independent evidence remain open. Parent #46 draft /
incomplete, all forty scenarios Partial; no Phase 2 or phase exit.
