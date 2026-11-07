<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Provider terminal-observation metadata evidence

**Date:** 2026-11-07<br>
**Plan:** [frozen scope](0044-terminal-observation-source-plan.md), `141bd8b`;
sharing refinement `53681cb`<br>
**Decision:** [ADR-0044](../decisions/0044-provider-terminal-observation-metadata.md)

Profile identity is
`urn:choreoform:observation-source-profile:provider-terminal-facts:0.1.0`, exact
revision `sha256:a224fc2b8318af2ff8dfc9955b55af03f4d5b7bd0de719de0384421698d20ead`.
The source resource accepts only kind/version/id/profile/provider. Actual exact
profile/provider/type bytes are caller-supplied and verified. Types, source labels,
state/tag mappings, trusted normalizer and safe flags cannot be overridden.

The immutable reader derives terminal names/states/tags from validated provider
states, not labels: succeeded -> confirmed with exact provider output; rejected,
cancelled and no-effect -> respective terminal tags with the correlation record.
Acknowledged/unknown names stay nonterminal. Only present terminal cases appear in
the normalized Variant. Provider validation already requires declared outcomes to
have success observations. Invalid no-success providers fail provider-outcomes;
they are not advertised as a reachable terminal-source-empty case.

The real RP-01 payment provider produces four case tags, retains exact nested Paid
and Problem nominal identities/revisions, and shares its declared resultProtection
for every case. The independently derived type equals the existing benchmark's
terminal-fact wait type; this is shape compatibility, not a wait/source binding.
An equivalent reserialized type artifact with new exact pins produces a distinct
type despite equal representations. Metadata reports all actual source/correlation/
current authority/protection/acceptance/cutoff/ownership requirements explicitly.

Initial completed-diff review found repeated large type/name copies across source
aliases of the same provider. Before refinement, scope supplement `53681cb` froze
sharing one private immutable schema by exact profile/provider pair, alongside
exact per-source pin caches. All 225 distinct sources of a 2,500-field provider now
share the same type/name-map addresses; each retains its own exact source identity.
This is declaration sharing, not a live value/permission/host cache. Every actual
name, case, nested type, key and nominal pin copy remains charged under the same
resource work/depth bounds. Failed derivations cache no schema/source, and cached
lookups refuse when the original work allowance is exhausted. No public refill API
or independent per-call allowance exists. The extra normalized layer also obeys
the existing depth limit.

Seven Rust integration groups exercise real nominal/protection metadata/cache;
every missing/corrupt source/profile/provider/type pin; closed fields, versions and
unsupported profiles; nonterminal misleading names, multiple success names and
non-normalized Unicode identities; exact nominal revision differences; large
shared derived schemas; and all twelve corpus vectors through the real verified
provider/source reader. Three internal groups cover exact bounded copying, failed
cache derivation/cumulative cached lookup and normalization-layer depth. The twelve
vectors have seven positive metadata classifications and five invalid provider
state/outcome cases. They are declarations, never generated observations.

Four independent Python groups check exact profile publication/registry/licensing/
CI, finite state/name/refusal classification, real source type versus the declared
benchmark shape, and byte-identical current native output through all five stages
from timing onward when unrelated valid metadata bytes are supplied. Rust separately
validates those metadata shapes; extra registry bytes remain inert and do not bind
an existing wait/retry. All current source/IR/resources/reports/guard bytes and
existing core/provider/wire/schema artifacts remain unchanged.

Passed **331 Rust tests plus six compile-fail doctests**, format/native workspace
lint, all five library Wasm lints and release portability/frontend builds. The
native portability oracle still passes 89 cases, 15 JCS/hash comparisons, three
unchanged fixtures, ten node variants and seven native CLI checks. Every required
Python oracle and exact RP-01 regeneration/historical probes pass. Logs:
`/private/tmp/terminal-source-{workspace,clippy,wasm,native,oracles,tests,unit}.log`.
Final-head CI/comment audit/signed merge remain required.

Separate completed-diff automated self-review checked exact version/pin/closed-field
boundaries, provider declaration versus actual observation, source protection versus
permission, nominal equality, all present/absent state cases, correlation envelope
requirements, memory amplification and shared-schema identity, cumulative budgets /
failed-cache behavior, unchanged old paths/artifacts and remaining gates. The
sharing refinement resolves the actionable performance issue; no finding remains.
Review is delegated self-review, not independent or individual owner approval.
Inspect all comments/CodeRabbit summaries and merge only the signed passing head.

Next decide and freeze explicit capture representation, preserving closed wire
admission and every old version. Require exact source/type/protection, accepted
observation-only initialization (initially timerless if appropriate), complete
reads/writes/invalidation/availability and a separate owned-local-storage proof.
Fault data and permitted owned continuation/cutoff/settlement remain later explicit
contracts. This reader supplies no capture, accepted fact, permission, dispatch,
reopened scope or executable definition. Parent #46 remains draft/incomplete and
all forty scenarios/phase gates remain open; Phase 2 does not start.
