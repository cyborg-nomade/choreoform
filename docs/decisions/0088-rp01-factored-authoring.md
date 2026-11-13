<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0088: Adopt factored RP-01 source and explicitly promote core 0.12

**Status:** Accepted under Project Owner delegation; effective on integration<br>
**Date:** 2026-11-13<br>
**Decider:** Codex under ADR-0018; no personal owner or independent approval

## Decision

Follow signed [scope88](../evaluation/0088-rp01-factored-authoring-plan.md)
`7604997` after PR #102 merge `6e7bb9c57fcde127eae5f763f5ec92f023dab4fc`, successful
final-head CI `43444154169`, automated COMMENT review `5892888321` and fresh full
audit. Signed archive checkpoint `2fd3cd24519f36c4bd0dfaf876d2be9baa8ef886` precedes
adoption code. Actual inventory corrects the plan's count: **59 current files**
plus **54 existing historical files**, **113 total**, rather than113 non-history
files. The59 are copied exactly, including the original README, to
[the new archive](../benchmarks/rp01/history/source-0.14-before-factoring/ARCHIVE.md);
the54 remain in place. No recursive history copy or rewritten historical evidence.

Adopt accepted source0.18/companion0.17 on **C12/IR0.4** in the actual finite RP-01
producer. Name the complete protection **Expense information**, stable source-only
ID `expense_data_protection`, with its own explicit reimbursement purpose symbol.
Name the complete payment input record **Payable instruction**, stable source-only
ID `payable_instruction_type`. All26 data records retain their distinct semantic
IDs and purpose symbols; five exact payment input/result-type positions use the
transparent description. Nested nominal contracts and every expanded field stay
fixed. Changed requested protection refuses before registering a data declaration.
Historical/default producer modes remain explicit and byte-exact. This fixture
renderer supplies native Rust, not a production Python compiler/general editor.

RP-01 selected C11 before this adoption. Explicitly accept promotion to published
C12, without selecting raw-SDK-only C13. Actual native old/new lowering changes
only `/body/dialects/core/id`, `/body/dialects/core/revision` and `/revision`.
All other body/envelope fields are exactly equal. The full documents and report
bytes therefore **change**; shorter spelling does not itself change the semantics.
No dialect contract, resource/provider/clock/receipt, guard predicate, nominal
membership, logical work, attempt, capture or permission changes.

## Evidence and consequences

Current source is **55453 UTF-8 bytes /9455 tokens /114301 native parser visits**,
versus61211/10223 before adoption: 5758 bytes (**9.41%**) smaller, including both
definitions and every use. The earlier generic combined test variant was55702;
domain naming and direct rendering save249 more bytes. Neither size reduction nor
native parsing supplies actual user comprehension or accessibility evidence.

| Attribution | Old | Current |
| --- | --- | --- |
| Source profile / companion / core | 0.14 /0.13 /0.11 | 0.18 /0.17 /0.12 |
| Semantic revision | `55413bc5be721c103f56bd61a30bbf829e3cc97b1f867157e554baec4470e285` | `cd0a7b6923ccc8a0afa31a27fca3aff280c3da601089ba12946f2d768fa1a829` |
| Canonical wire SHA256 | `90c5ee32e45c97adf746355ac0337111fd408071ac622317874e8f0efe942fd6` | `d96b6398e6856a1e34916046a6ca61ee0e09339f59c5c3664c524a5b43a801de` |

Source SHA256 `95d223f9bef902cc93edbf830642e8af499e2e77071a80d40f01377c4ece6735`,
canonical bindings `e2757e1f6f3201a67de13aa16ec35a379180aa424a6805c0728e281957939360`,
joint package `c26c9689b479566d07ed244ededcf9b3d6d1cda7fcbe1409a1e3be56442d2a7a`.
All eleven actual checking/advisory views pass, still executable:false. Explicit
field-level comparisons allow only revision attribution at documented report
pointers; they never blindly replace matching strings. All prior conditional
lifetime/provenance/coverage limitations remain. [Evidence](../evaluation/0088-rp01-factored-authoring.md)
and the [exact old/new digest table](../evaluation/0088-rp01-adoption-digests.json)
record the finite allowed changes. The44-case/46-value guard catalogue changes
only its two candidate pins. Every immutable input/resource/guard payload remains
exact, and the complete adopted RP-03/all prior archives remain byte-identical.

Final-head tests/CI, separate completed-diff automated COMMENT review, full review /
CodeRabbit audit and expected-head signed merge are mandatory. A bot skip supplies
no evidence. All scenarios remain Partial, G1–G4 conditional and parent #46 draft.
C13 source, reusable receipt/work/predicate language, full RP-08, general protected
accessible editing/export/text-visual, actual users/hosts/browsers/independent and
all semantic/region/phase gates stay open; no Phase 2.
