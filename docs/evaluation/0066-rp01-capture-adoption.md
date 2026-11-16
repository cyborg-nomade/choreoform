<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Full RP-01 typed late-capture checkpoint evidence

**Date:** 2026-11-10<br>
**Baseline:** `47bf1c5e907bbeedde1856a27d76835e9c7e287d`, draft #46 after #80<br>
**Frozen plan:** `262fdc0`, [plan](0066-rp01-capture-adoption-plan.md)<br>
**Decision:** [ADR-0066](../decisions/0066-rp01-capture-adoption.md), owner delegation

This publishes the demonstrated full-case integration from audit 0065 using existing
source 0.14/companion 0.13/core 0.11/IR 0.4. Production rules, accepted artifacts,
grammar productions and quotas are unchanged. Current source is 61,211 UTF-8 bytes /
10,223 whole-source lexed tokens, companion 52,736 stored / 37,464 canonical bytes,
canonical IR 108,565 bytes. Semantic revision:
`sha256:55413bc5be721c103f56bd61a30bbf829e3cc97b1f867157e554baec4470e285`;
canonical wire digest:
`sha256:90c5ee32e45c97adf746355ac0337111fd408071ac622317874e8f0efe942fd6`.
All eleven actual checking/advisory views pass; the final native report is 32,481
bytes including newline (41,869 stored pretty bytes). All remain executable:false.

The exact [historical checkpoint](../benchmarks/rp01/history/source-0.7/README.md)
retains source 56,874 bytes / 9,470 tokens, revision
`sha256:97e5c346442d5b051e40bc91d2c3014be1bd990688324be846739c5d35a265c4`,
wire digest `sha256:c012c1493ad7b04abc5e5f81529eea6231b720a54ee6a8db867fd2e207fd546a`
and original ten-stage manifest digest
`sha256:fd1ca83a022d7ccd8e3f21cf85305d662725e2ac5172cd201615ded0c7471ffb`.
A direct pre-adoption-parent comparison found all 53 archived non-README files
byte-identical. The separate contextual README replaces old relative document links.
Original resource/policy/clock/input/fault-source bytes stay identical at the root.
Historical tests explicitly select old source/IR/manifest/reports instead of silently
weakening old-profile expectations; current coverage has its own required oracle.

## Exact integrated behavior and delta

| Part | Current checkpoint | Boundary preserved |
| --- | --- | --- |
| Profile/wire | Explicit source 0.14/companion 0.13/core 0.11/IR 0.4; accepted general retry fault-source sentence | Same exact payment fault provider/classification binding, ordinary retry budget/key/clock |
| Source descriptor | Exact payment.terminal-source.json pins unchanged payment provider and terminal normalization profile | No authenticated observation or truth inferred from descriptor/pin |
| Capture/request | Actual payment_observe captures payment_request_step; retained_payment_request is its separate readonly full request | Fresh local storage only; original ordinary ports and ancestor current_claim dependence remain |
| Local classifier | classify_late_payment_assignment reuses original payment completion predicate against captured confirmed payload + retained request | Every claim/revision/amount/currency comparison preserved; no current mutable ancestor value |
| Late record | paid/problem only after matching receipt; mismatched-evidence/noeffect/rejected/cancelled stay distinct | No ordinary payment result/parent resumption/cancellation clear/new dispatch; raw capture retained for conflict review |
| Reports | Two isolated declarations + separate conditional flat payment local proof; retainedRequestScopes payment_recovery | Legacy uncovered payment wait/classifier remains explicit; no continuation tree or live handoff claim |

Complete body-delta tests remove only the selected version/core/nullable fields and
new capture/three data cells/compute/assignment/flow, restoring the one wait edge;
the result equals the entire old body. Original data, ordinary control/approval /
commit/withdrawal guards, expressions, protected recipient envelopes, ports and all
IDs remain. All new locals clone unchanged Expense access/purpose/participant /
capability envelopes and declare exact wait/classifier/finish invalidations.
The complete original guard catalogue content hash excluding its two candidate pins
remains `sha256:b0becff0544aa8a5a0610c0b0c7c1ae4ddda21211d77d57566f4364adff5897d`:
44 cases, 46 values, every subject/binding/expectation unchanged. Formatting is also
preserved; only candidateRevision/candidateDigest bytes change.

The thirteen separate synthetic late cases are unchanged. An explicit audit-to-
official cell mapping must equal the actual compiled classifier. The production
pure evaluator checks actual types, complete two-cell dependency binding and all
thirteen values, including the genuine tag-only wrong-amount counterexample. It
loads current wire explicitly and supplies all exact inherited cores/profiles and
the new descriptor. No public evaluator, actual event or accepted history is added.

Seven bounded Python groups reproduce exact generator/history bytes, full body and
guard deltas, all eleven current native reports, conditional protection provenance /
coverage, compiled classifier/corpus equality, genuine wrong-provider/readonly-seed /
missing-invalidation refusals, current format/lower cycle and quota/license/CI checks.
Two initial assertions incorrectly expected absent report/data keys to be explicit
empty/null and were corrected to the actual closed wire/report shapes. Historical
README comparison was explicitly separated from code/data/report equality; an old
unused-terminal-source regression now selects the archive so it does not duplicate
an already-present current profile artifact. No production validation was weakened.

## Verification and remaining evidence

The current workspace Rust suite passes 548 tests plus six compile-fail doctests,
including the actual current guard and late-corpus evaluator. Native workspace and
five library Wasm clippy checks, release portability/frontend Wasm builds and native
portability pass. All 61 required Python commands pass at the final local tree. Separate
completed-diff delegated agent review checked full body/resource/guard/history deltas,
receipt matching, closed transport, bounded generation and honest conditional proof;
no remaining actionable finding. Required final-head CI/comment audit/signed merge
follow before integration. Temporary
local verification first pointed the portability oracle at an unrelated old report;
its missing-vector refusal was corrected by passing the freshly generated report,
without changing the oracle or production code.

All RP-01-A/B/C/D and forty corpus scenarios remain Partial. A matched local category
is not full reconciliation/executable admission: actual source truth/correlation,
current authority, immutable request/capture storage, activation/cutoff, receiver
acceptance/atomic surviving handoff/all-transitive settlement and accountable
contradictory-evidence review remain open. A mismatch text/local finish settles no
conflict; unknown/acknowledged are not terminal and new work is not technical retry.
Current provenance labels grant no live ancestor value or permission. The integrated
host contract names these boundaries explicitly.

Next dependency: reassess remaining actual original-scenario and complete-validator /
RP-03/08/general near-English/protected accessible text-visual/round-trip gaps from
this checkpoint, rather than another speculative permission increment. Actual
host/browser/representative-user/independent evidence cannot be generated here; finish
independent work before reporting a genuine external blocker. G1–G4 conditional,
parent #46 draft, no Phase 1 exit/report approval boundary or Phase 2. Logs:
`/private/tmp/rp01-capture-adoption-{workspace,verify,oracles,python}.log` (historical
failed local harness attempts are retained separately). A skipped CodeRabbit review
supplies no evidence; delegated agent review is not independent/human/owner approval.
