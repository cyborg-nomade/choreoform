<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Readable original-request retention and native integration evidence

**Date:** 2026-11-08<br>
**Baseline:** `30b3f228097692d5e63db418f570b9470f8acd2d` after PR #69<br>
**Authority:** ADR-0018; [ADR-0055](../decisions/0055-request-retention-authoring.md)<br>
**Frozen plan:** `e9af4dc`, [plan](0055-request-retention-authoring-plan.md)

Source 0.11 / companion 0.10 explicitly selects existing core 0.8/IR 0.4 and all
seven bases. One optional nested-scope original-request clause adds no resource
kind, initializer, value or visibility exception. Normal origin/local result
resolution produces one pair in the required retainedRequests map; other scopes
get empty maps. Nullable faultInput, capture, retry, metadata/actual-use endpoint
families and all earlier versions/quotas/hashes stay fixed. Libraries require exact
artifacts; CLI embeds only known core/semantics bytes. Private clause spans map
semantic retainedRequests paths in original UTF-8 bytes, preserving public span
inventories and fault-input mapping.

All eleven native checking/advisory views add nonempty declaredRetainedRequests
from retained static rows, including unvisited declarations. Exact revision/scope /
origin/result identify the validated input-type relation; the SDK exposes its
actual type descriptor. Reports avoid repeated schemas. Owned rows add optional
originalRequest relations; final reconciliation adds nonempty retainedRequestScopes.
Its already-proven owned bodies remain separate from ordinary/legacy checked sets
and still appear in legacy uncovered sets. No proof/admission rerun is used for
output, and no actual immutable storage or original request is manufactured.

Static accumulation reserves inherited output before new rows and uses bounded
serialization; owned rows now reuse the same bounded writer. Final complete JSON
still has a 1 MiB bound before stdout. Capture/source/history/retry/historical /
isolated/owned and conditional-history fields remain present; older/default empty
fields stay omitted.

Three Rust frontend groups cover promoted full RP-01 retained read/copy/branch with
capture/retry/history through every SDK view, legacy/ordinary/owned separation,
formatting/reparse/coordinated Unicode aliases, qualified references, exact core
artifacts, local-owner/readonly refusals and original UTF-8 clause spans. Small
shared recovery-scope/resource/ledger fragments avoid duplicating the full package.
Six Python groups independently check:

- full composition through all eleven native views, exact declarations/owned
  relations, separate conditional scopes and preserved prior evidence;
- format/qualified aliases/independent package framing; empty new maps and old
  profiles preserving omitted fields/default behavior;
- grammar-only scope delta, known version/pair/core boundaries and unchanged quotas;
- local owner, nonactivity origin, forged initializer, pre-entry self-access,
  public-copy flow and strict missing invalidation refusals;
- unvisited original-request declarations without an owned proof, combined valid
  bounded stress inputs and no partial stdout at every checking view.

The full promoted fixture is 61,162 UTF-8 bytes / 10,095 tokens and lowers to
110,007 output bytes including newline. The stress fixture composes 130 long
history descriptor aliases with one valid declared original-request owner/follow-up:
97,553 source bytes / 14,257 tokens, 615,741 companion bytes using ordinary JSON
spacing, and 654,063 lowered output bytes. Each is within source/transport bounds;
all eleven checking views refuse report-budget with empty stdout. The small
unvisited variant passes and discloses the declaration without claiming retained
request coverage. These are real native semantic/report judgments on synthetic
static declarations, not runtime requests, storage, observations or receipts.

Local validation passed: 451 Rust tests plus six compile-fail doctests, native
Clippy, all five Wasm library lints, release Wasm portability/frontend builds,
native portability report and every Python workflow command, including deterministic
benchmark regeneration. Final-head CI/comment audit/signed expected-head merge
remain required before integration. Local logs use
`/private/tmp/request-retention-authoring-{workspace,clippy,wasm,native,oracles}.log`.
Compilation does not establish browser or host execution.

Separate completed-diff delegated self-review assessed correctness/security /
performance/scope/evidence, version/resource/visibility/local/readonly barriers,
raw source/access/protection/invalidation preservation, proof separation,
cumulative/final output bounds, compatibility and licensing. It corrected the copied
grammar header to point to profile 0.11; no actionable issue remains. It is agent self-review,
not independent/human or personal owner approval. Audit all unresolved threads and
CodeRabbit summaries; skipped bot review supplies no evidence.

Committed RP-01 source 0.7/core 0.4/IR 0.1 and existing package/report/guard bytes
stay unchanged. Actual original accepted value/revision/digest, exact occurrence /
request/key/effect/attempt correlation, fresh immutable owned storage, authenticated
complete provenance, current data/work/capability/management grants, receiver
acceptance and atomic handoff/cutoff/all-obligation settlement remain live premises.
Next permitted separately authorized new work inside the surviving boundary with
explicit cancellation/closure cutoff/handoff/all-settlement. Parent #46 stays draft /
incomplete, all forty scenarios Partial, G1–G4 conditional. Full validator/benchmarks,
general tool-managed near-English/protected accessible visual editing and real
host/browser/user/independent/phase gates remain open. No Phase 2 or phase exit.
