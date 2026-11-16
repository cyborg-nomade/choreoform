<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Readable retained requests and bounded native integration: frozen plan

**Date:** 2026-11-08<br>
**Baseline:** `30b3f228097692d5e63db418f570b9470f8acd2d`, draft #46 after PR #69<br>
**Authority:** ADR-0018; declarations/conditional proofs are not live storage or work

Use a dedicated stacked dependency PR and ADR-0055. Add explicit source 0.11.0 /
companion 0.10.0 selecting existing exact core 0.8/IR 0.4 and all seven bases.
Preserve all earlier grammar/schema/core/wire/resource/hash/report meanings and
current committed RP-01 bytes. Copy source 0.10 grammar, adding one optional nested
scope clause after its interface/history clause and before declarations:
`Retain the original request from "Request Payment" and record in "Original request".`
Use the delimiter to separate normally qualified references containing `in`.
Root process has no clause. Mixed history/retention syntax may parse but must
retain the SDK's incompatible-owner/handler refusal, not invent mixed semantics.

Resolve the origin normally as a visible node and result as local data. Lower an
empty required retainedRequests map on every scope, or one exact result/origin pair
for the explicit clause; preserve nullable faultInput and capture fields. Do not
supply a value, initializer or private endpoint exception. Keep exact resources /
normal visibility/type/readonly/creator/provider/capture/access/provenance/invalidation
and all other raw refusals. CLI embeds core 0.8 only for the new profile, with all
inherited bases; libraries require exact artifacts. No new resource kind/nominal
profile or implicit upgrade. Inherit source 0.10's allowed resource kinds explicitly.

Keep closed known-profile checks, 16,384 whole-program tokens / 1,000,000 parser
visits, shared 100,000 dependency work and 1 MiB transport/report bounds. Unknown
1.0/fragments and older quotas remain fixed. New package framing domain is source
0.11. Formatting/reparse/coordinated aliases preserve within-profile identities/IR;
explicit version/core/wire promotion changes semantic digests. Map retainedRequests
semantic paths to the original clause's UTF-8 byte range without changing public
declaration/generated span inventories or existing fault-input mapping.

Reuse the already composed raw owned proof, including legacy reconciliation
separation; do not rerun admission for reporting. All eleven checking/advisory views
disclose nonempty declaredRetainedRequests rows from the static report: exact
revision/scope/origin/result and mandatory live premises. Origin/result identify the
exact input-type equality already validated in that same revision; the SDK exposes
the exact typed descriptor. Do not duplicate full type schemas per row or introduce
an unbounded type serializer. Add optional originalRequest relation to existing
owned follow-up rows only when present. Final reconciliation availability discloses
nonempty retainedRequestScopes so legacy uncovered bodies remain understandable.
All older/default/empty-profile output fields remain omitted and byte-identical.

Use bounded row serialization and the shared static cumulative allowance, reserving
all inherited output before adding new declarations. Retain final complete 1 MiB
writer bound before stdout; no truncation, dropping rows or partial JSON. Preserve
capture/source/history/retry/historical/isolated/owned and conditional-history fields.
A valid bounded stress package can compose inherited long descriptor declarations
with a small valid retained-request owner/follow-up; it must reach semantic checks
then refuse report-budget in every checking view with empty stdout. Catalogue
sharing and original request relation never share live values or authority.

Test promoted complete RP-01 retention/read/copy/branch through all eleven native /
SDK views and existing capture/retry/handler-history composition. Test new empty
maps and old profiles, grammar/schema-only delta and strict pair/kind/core resources,
normal qualified references, local ownership, type/readonly/creator/capture/source /
access/protection/strict invalidation barriers and declared-unvisited/unproven rows.
Check formatting/aliases/independent package hashing, original UTF-8 diagnostics,
separate ordinary/legacy/owned coverage and cumulative/final output refusal.
Use genuine SDK/native judgments and independent bounded Python checks, not
fabricated requests, host storage, observations, participants or receipts.

Run native/Wasm/all Python/final-head CI, separate completed-diff delegated review,
all unresolved threads/CodeRabbit summaries and signed expected-head merge. Update
progress. Then permitted separately authorized new work inside the surviving
boundary, with retained original request/correlation and explicit cancellation /
closure cutoff/atomic handoff/all-settlement. General tool-managed near-English
editing, complete validator/benchmarks, protected accessible visual tooling and
actual host/browser/user/independent gates remain open. Parent #46 stays draft /
incomplete, all forty scenarios Partial; no Phase 2 or phase exit.
