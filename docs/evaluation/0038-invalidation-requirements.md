<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Advisory invalidation requirements evidence

**Date:** 2026-11-06<br>
**Plan:** [frozen plan](0038-invalidation-requirements-plan.md), `62fd323`<br>
**Decision:** [ADR-0038](../decisions/0038-invalidation-requirements.md)

The new library/Candidate/native advisory derives ordinary required site/cell
pairs and their exact missing subset from the same private analysis as strict
invalidation. It cannot become a strict report. Existing strict/composed entry
points retain their first coverage error even if advisory derivation encounters a
later unproved context. Advisory derivation instead refuses that context entirely.
No source edit, execution permission, syntax/core change or cached truth follows.

Six Rust regression groups cover multiple cells/sites and retained extra links,
unvisited sites, immutable candidates, exact fresh declarations and subsequent
revocation, old-advice revision separation, strict error precedence, historical
evidence/live ancestors, missing resources, invalid references and work bounds.
A compile-fail doctest enforces the advisory/checked API boundary. Six Python
groups exercise native exact set difference, all relevant strict prerequisites,
empty-advice status, malformed/missing resources, source format/reparse identity,
unvisited declarations, full RP-01 and native report exhaustion without stdout.

Actual unchanged RP-01 has 84 ordinary requirements, 72 missing links across
18 cells, and 88 conditional historical-origin records. Its existing current-claim
links supply the other 12 requirements. The missing counts by cell are:

| Cell | Missing links |
| --- | ---: |
| disposition | 6 |
| disposition_reasons | 6 |
| finance_evidence | 6 |
| manager_evidence | 8 |
| payment_due | 12 |
| review_needed | 12 |
| checks_request | 1 |
| cycle_checks | 5 |
| recorded_payment | 3 |
| correction_reasons | 1 |
| duplicate_result | 3 |
| finance_result | 1 |
| manager_result | 2 |
| notice_request | 1 |
| payable_reference | 2 |
| payment_deadline | 1 |
| payment_request | 1 |
| recorded_notice | 1 |

These are conservative ordinary requirements, not minimal causal semantics.
Benchmark source/IR/revision and existing three passing reports are unchanged.
Strict invalidation and later composed stages still refuse
`invalidation-coverage at /body/data/disposition/invalidates`. The requirements
view is advisory even with an empty missing set; its `requiredNextCheck` retains
strict invalidation at the final definition revision, and `remaining` names later
gates. Unvisited sites and historical conditional premises remain explicit.

Local checks passed 283 Rust tests and six compile-fail doctests (the existing
workspace run plus the new semantic-check doctest), format/native workspace lint,
all five library Wasm lints, release portability/frontend Wasm builds, native
portability report/oracle, every CI Python oracle and historical RP-01 probes.
Logs are `/private/tmp/requirements-{workspace,doctests,clippy,wasm,native,oracles}.log`.
The new Python oracle is in required CI. Final-head CI is required before merge.

Separate completed-diff automated self-review checked strict error ordering,
private report constructors, type separation, final-revision rechecks, immutable
input/resource handling, ancestry and private-context refusal, historical/live
lineage, unvisited-site accounting, copying/work charges, and bounded native
publication. The review added a compile-fail boundary test and documented the
mandatory strict next check separately from later residual gates. No actionable
issue remains. This is delegated automated review, not independent/human/owner
review. Audit all PR comments, including CodeRabbit summaries, and merge only
the passing expected head with an explicit sign-off trailer.

Next return to draft #46 and add only the exact 72 missing valid links while
preserving existing links/identities. Regenerate all stages and freeze the next
real proof dependency. No full benchmark, scenario or Phase 1 gate is closed.
