<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Readable handler entry and bounded native integration evidence

**Date:** 2026-11-08<br>
**Authority:** ADR-0018; [ADR-0053](../decisions/0053-handler-entry-authoring.md)<br>
**Baseline:** `c7983201e4dd81ce2289a00cf236da740fcefbb2` after PR #67<br>
**Frozen plan:** `d1e1b70`, [plan](0053-handler-entry-authoring-plan.md)

Source 0.10 / companion 0.9 adds one scope clause and provider-fault-history kind,
selecting existing exact core 0.7/IR 0.3. Earlier grammar/schema/core/wire/artifact
bytes remain fixed. Exact descriptor/profile/source/provider/fixed nominal resources
validate through existing SDK readers, including unused declarations. Scope result
ownership is checked during lowering; raw readonly/creator/port/type/protection /
context rules stay in the semantic checker. Every new-wire scope has nullable
faultInput, no invented value/initializer. Normal visibility and qualified references
are preserved. Semantic fault-input paths map to the clause's UTF-8 byte span without
changing existing declaration/generated span inventories.

Exceptional availability now retains the existing conditional typed proof only for
explicit bindings. One prior-stage/origin chain is shared through immutable Arc
FaultReport evidence; later reconciliation keeps it. Legacy assumptions and checked
sets exclude those bodies, while legacy uncovered sets remain unchanged. This is
the explicit integration following the raw-only refusal recorded in ADR-0052; it
is neither a retroactive claim about that checkpoint nor ordinary availability.
No binding preserves the old check path, with one small fixed sharing allocation.

All eleven checking/advisory views disclose static entries and per-exact-descriptor
catalogues. Only the final three add separate conditional sets, access/invalidation /
source-control provenance and incoming site/full-stack context relations. Empty
additions are omitted. Report rows share bounded static/conditional accounting;
inherited content and final JSON are checked before stdout. No admission reruns.

Three frontend Rust groups cover all SDK checking stages and advisory requirements,
conditional/ordinary/legacy separation, formatting/reparse/coordinated UTF-8 names,
package identity, clause mapping, local-owner refusal, every inherited core/history /
nominal missing artifact and corrupt unused history. An existing SDK integration
group now verifies the separate pipeline judgment and pointer equality of shared
prior evidence. The inherited fourteen raw SDK groups and finite corpus keep
human/null/wrong/unvisited/unexplored/active source, ordinary/P6 creator/port/writer,
constant/initializer/branch protection, empty pre-entry access and strict invalidation
barriers in the full suite. New source-level refusals exercise wrong kind/pair/pin,
missing/corrupt/unused resources, wrong owner/type/initializer/source, self-dependent
access, public initializer flow and missing invalidation.

Seven Python groups independently check EBNF/schema delta, closed version pairing,
formatting/aliases/package hash framing, new nullable and old output boundaries,
full promoted RP-01 plus capture/owned-local/retry composition through all eleven
views, per-pin catalogue sharing, declared but uncovered entries and output overflow.
The promoted full-package fixture is 59,407 UTF-8 bytes / 9,843 tokens and lowers to
105,798 bytes (including native newline), without adopting its bytes into RP-01.
The stress fixture has 130 distinct long descriptor pins under one unused owner:
94,651 source bytes / 13,829 tokens, 612,314 companion bytes using ordinary JSON
spacing, and 646,692 lowered output bytes. It is within every admission/transport
bound; all eleven checking views refuse report-budget with no stdout. Two aliases
of one exact descriptor disclose two entries and one catalogue; no-incoming entries
stay uncovered. No fake runtime values, receipts or user/host evidence are generated.

Local required validation: 433 Rust tests plus six compile-fail doctests; native
Clippy with warnings denied; all five Wasm library lints; release Wasm portability /
frontend builds; native portability report and every Python workflow command,
including deterministic RP-01 --check --probe-profile. Detailed local logs are
`/private/tmp/handler-authoring-{workspace,clippy,wasm,native,oracles}.log`.
Final-head CI, unresolved threads/CodeRabbit summary audit and signed expected-head
merge remain required before integration.

A separate completed-diff agent review checked correctness, version/resource/role /
visibility barriers, conditional versus ordinary evidence, single-chain sharing,
source/control/access/invalidation protection, cumulative final output bounds,
compatibility, licensing, scope and honest external gates. No actionable issue
remains after the reviewed changes; the final lint also required collapsing the
clause-span lookup into a let-chain. It is automated delegated
self-review, not independent/human or personal owner approval. A skipped bot review
supplies no evidence; all actual comments and summaries must still be evaluated.

Committed RP-01 source 0.7/core 0.4/IR 0.1 and existing reports/guard vectors remain
byte-identical. Actual fault authentication, request/occurrence/input/effect correlation,
current source/work/access/management authority, complete retained protection,
immutable owned storage and live selection/cutoff remain host premises. Handler
finish grants no parent resume or result publication. Next: permitted owned
continuation with retained request/value identity, cutoff/handoff/all-settlement.
Parent #46 remains draft/incomplete, all forty scenarios Partial, G1–G4 conditional.
Full validator/benchmarks, general tool-managed near-English editing, protected
accessible visual tooling and actual host/browser/user/independent evidence remain
open. No Phase 2 work or phase exit is claimed.
