<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Provider terminal-observation metadata: frozen implementation plan

**Date:** 2026-11-07<br>
**Baseline:** `dca191306dc748bea0725cb163756a0f49084fc0`, draft #46 after audit #57<br>
**Authority:** ADR-0018; audit identifies missing source/type/protection premises

Use a dedicated dependency branch/PR. Record ADR-0044 accepted under delegation,
and publish an independently pinned provider terminal-observation-source 0.1
profile with exact immutable artifact bytes. The closed resource is exactly
kind/version/id/profile/provider. It neither supplies types/protection nor lets
an author override states, normalized tags, trust or host support. Existing
provider formats, core/source/wire/schema bytes, package resources/IR/reports and
all old meanings remain unchanged.

Decode only caller-supplied exact source/profile/provider/type artifacts. Derive
terminal observations by declared provider state: succeeded -> confirmed, rejected
-> rejected, cancelled -> cancelled, no-effect -> noeffect; omit acknowledged and
unknown regardless of their names. Multiple succeeded names may share confirmed;
retain exact original names/states without Unicode/name inference. Build the exact
normalized Variant of only present terminal cases: confirmed has provider output
Type; other cases have the existing correlation record of logical effect, request
digest and observation Text fields. Require at least one terminal case (provider
already requires supported declared successes). Preserve exact nominal identity
and verify all referenced type bytes; do not accept nominal representation equality
or arbitrary user-provided normalized payload/protection declarations.

Expose immutable private-constructor metadata with exact pins, provider contract,
terminal-name/state/tag map and normalized type; source protection is precisely
that provider's resultProtection, still needing actual definition-context/access/
no-declassification checks. Cache verified immutable metadata by exact pin with
Arc; budget every derived name/tag/type copy under the existing resource allowance.
Use bounded depth/work for copying the output Type, including nominal references,
not an uncharged deep clone or independent quota. Cached lookup is read-only and
never materializes an accepted value, fact receipt, permission or execution handle.

Document live requirements: truthful supported normalizer/provider source, exact
attempt/occurrence/logical-effect/request/key/input/observation identity, current
authority/protection, acceptance order, retained duplicates/contradictions, cutoff,
owned storage/subscription/obligation and all-settlement gates. An unknown label
is not terminal; a succeeded business problem is not proof of physical payment.
This metadata does not bind any existing wait/retry, introduce wait capture/fault
ports, decide retry eligibility, change ordinary flow or reopen a cancelled owner.
Those require the subsequent explicit representation/owned-storage design.

Tests: exact real RP-01 provider/nominal-output metadata, every resource/profile/type
pin and corrupt bytes; minimal success-only and full terminal-state type cases;
ack/unknown exclusion even misleading names; multiple successes; closed fields and
wrong versions/profile/provider shape; exact nominal revision identity and output
resource/type validation; immutable cached metadata and cumulative work/depth/copy
refusal. Add an independent finite source-state/type/protection oracle/corpus and
required CI; demonstrate current native reports are identical with unrelated
valid new metadata supplied (no implicit binding). Keep metadata versus live truth
and all scenario/phase gaps explicit.

Run native/Wasm/all Python checks and final-head CI, separate completed-diff
self-review, inspect all comments/CodeRabbit findings and signed expected-head
merge into draft #46. Then freeze the reviewed capture representation and its
wire/policy compatibility, P2 provenance, timerless initialization and owned-local
lifetime requirements before code. Parent stays draft/unmerged; all forty scenarios
Partial. No per-item owner approval or Phase 2 work is introduced.

Completed-diff review of the initial implementation identified repeated derived
name/type copies when several distinct source pins name the same exact provider.
Before refining code, share one private immutable terminal schema by provider pin
inside Resources, while retaining exact per-source pin/profile/provider metadata
and source-pin caches. This is declaration sharing only, never a host/value cache.
Test shared schema addresses across source pins and no per-lookup fresh budget;
keep direct cumulative copy/depth tests and verify failed derivation is not cached.
No public contract, quota, source bytes or runtime meaning changes.
