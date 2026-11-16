<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Measured source-0.15 capacity: frozen implementation plan

**Date:** 2026-11-11<br>
**Baseline:** `3c190c9cf7ae08c792c3b18034a67ec4f0b16006`, draft #46 after #82<br>
**Authority:** ADR-0018 / ADR-0067; no new semantic/core/wire permission

Freeze/sign on a dedicated branch before production changes/new probes. The actual
RP-03 draft is 505,980 bytes / 77,135 complete-line tokens, source digest
`sha256:0b6a4c2b18367f47707003cf93358106c8fe20950a677ed7444e182ead938797`.
Source 0.14 refuses tokens at 108626..108627, empty stdout; it has no candidate or
semantic-stage reports. Preserve every committed RP-01/current/history/guard and
RP-03 draft/resource/pure-case/manifest byte throughout this capacity slice.

Add explicit source 0.15 / companion 0.14 selection of the same exact core 0.11 /
IR 0.4 and all ten inherited core bases. No new core/dialect/IR version, grammar
production or private-reference/ownership permission. Copy the existing grammar
with only header/documentation version changes; new companion schema differs only
in exact profile/version identifiers and explanatory metadata. Native library/CLI,
strict ledger pairing, exact resource closure and limited independent Python
recognition/schema mapping must agree on this explicit known pair. Every prior
profile/pin/meaning stays fixed; unknown future versions still refuse.

Give only full programs explicitly selecting source 0.15 a 131,072-token quota
(the next power-of-two bound above the measured complete case). Preserve source
0.5–0.14's 16,384 and legacy/fragment 4,096 limits, including source 0.14's exact
RP-03 first refusal. Keep 1 MiB source/companion/output limits, depth 64, current
parser work 1,000,000 for later profiles, lowering work 100,000, semantic/shared /
report budgets and every other limit unchanged. A larger lexer allowance confers
no semantic admission, execution, IO, authority or host resource grant. Do not
raise another budget just to make this case pass; record the next actual failure.

Update closed profile families for all inherited features, exact expected core,
versioned envelope, builtin resource selection and report paths. Prefer existing
family conventions over unrequested broad refactoring; no arbitrary numeric-version
comparison or future wildcard. Add precise tests for pairing/core/resource/schema /
grammar equivalence, old quotas, unknown future versions, exact new token boundary
and shared inherited feature behavior. A small paired source-0.14/source-0.15
program must have identical semantic body/version/revision and unchanged report
meaning; source-package identity must still reflect the exact source/ledger bytes.

Use an explicit temporary/in-memory RP-03 promotion: header 0.15, companion pair
0.15/0.14, same C11 and every original declaration/ID/resource/guard/predicate /
recipient/compensation/physical/ownership responsibility. Measure actual whole
recognition/parser work, native lowering/canonical output and each of eleven later
checking/advisory views where the previous step allows. Record genuine first code /
path/UTF-8 span and executable:false output sizes, distinguishing not-run stages
from failed or passed. Do not generate replacement IR through another route,
remove compensation/guards/alternatives, initialize accepted observations, broaden
privacy, weaken provider metadata, truncate obligations or invent a host trace.
Unexpected body deltas require analysis and separate scope, not automatic acceptance.
If a source-only syntax/caption/reference correction is necessary within existing
accepted rules, record the actual failure and exact experimental correction; keep
published draft bytes fixed. A missing binding/semantic contract needs a later
separately frozen ADR/profile; this capacity change must not add that permission.

Only after independently demonstrated lowering may a later controlled adoption plan
update the RP-03 package. This PR leaves its source-0.14 negative checkpoint fixed.
Do not claim that complete-line token counts alone recognize it. Keep root-settlement
aggregate entry/retention/current known-vs-live obligation contexts, qualified prior
compensation, ordinary/exceptional availability/protection/invalidation, physical
races/cutoff/current authority/accepted all-obligation handoff/settlement, optional
variants and actual duplicate/event evidence explicit as remaining judgments.

Run relevant pinned Rust/native/Wasm/all required Python checks and exact final-head
CI. Review completed diff separately for correctness/security/performance/scope /
evidence; audit every unresolved thread/summary including CodeRabbit; skipped bot
supplies no evidence. Merge signed with expected head under delegation, never a
purported independent/human or GitHub self-approval. Update progress/parent draft
with actual result, next dependency and open gates. All forty scenarios Partial,
G1–G4 conditional; parent #46 remains incomplete. No actual browser execution is
inferred from Wasm compilation.

Source verbosity remains a separate Phase 1 requirement: reusable near-English
formulations, identity-managed source/IR editing/export, protected accessible
summaries and text/visual round trips before representative-user evidence. Capacity
cannot satisfy or defer it to Phase 2. Full validator/RP-01/03/08, actual users /
host/browser/independent and all phase exits remain open. Finish independent work
before reporting a genuine external blocker. Stop at the honest phase report for
owner approval; no Phase 2 or new phase chat is authorized here.
