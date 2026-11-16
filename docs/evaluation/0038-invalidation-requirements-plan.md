<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Ordinary invalidation requirements: frozen advisory-stage plan

**Date:** 2026-11-06<br>
**Baseline:** `a8fd751f33385cd65d96f5987489b2aead598018`, draft #46 after #50<br>
**Authority:** ADR-0018; actual missing disposition coverage after ADR-0037.

Use a dedicated stacked dependency PR targeting the unmerged RP-01 branch.
The full package now reaches genuine missing invalidation declarations, but the
first-error checker exposes only one cell path. Adding every possible site would
change behavior unnecessarily and hide the exact dependency requirements. Derive
all ordinary live obligations and missing links first, without mutating source.

Publish ADR-0038 and a separate typed `InvalidationRequirements` API plus Candidate
method/native `invalidation-requirements` adapter. Validate the same strict
transport/resources, static/control/protection, historical-origin and ancestry
premises as today. Refuse unresolved private contexts, malformed declarations,
wrong pins and budgets; do not return a guessed partial list. Once these premises
hold, report all required live `(site, cell)` pairs and the exact subset missing
from data.invalidates, alongside analyzed/unvisited sites, historical conditions,
revision/input assumptions and remaining gates. Status is advisory requirements
only, always non-executable, never invalidation-passed or a planning permission.

Share the analysis through private data/mode types so advisory reports cannot be
converted into strict InvalidationReport. Keep check_invalidation and all composed
checks strict, preserving first-error precedence/code/path when coverage is missing.
A missing set may be empty without establishing host/current authority, non-data/
exceptional proofs or execution. No public ignore-coverage flag, accepted Boolean
claim, implicit source edit, new syntax/core pin or changed definition revision.

Run graph/protection work under the existing budgets, charge missing-pair copying
and retain bounded native JSON before stdout. Reuse historical evidence serialization
without requiring a strict report for the advisory view. Do not cache predicate
truth or mutate the immutable candidate/envelope/resources.

Tests: multiple missing cells/sites versus exact required-minus-declared set;
strict check/composed refusal despite advisory success; fresh strict success only
after exact valid links; preserved strict error precedence; unproved/private context
refusal; missing/corrupt resource and invalid references; unvisited declarations
without invented entry; historical/mutable lineage and report/hash/format identity;
work/report bounds. Derive the actual RP-01 requirements/counts from the native
adapter and retain its still-failing strict stages. Do not modify benchmark links
inside this dependency merely to make the advisory appear successful.

Run native/Wasm/all relevant Python checks, separately review completed diff,
inspect every PR comment/CodeRabbit summary, sign off authored and merge messages,
and merge only the passing expected head into the parent. Then return to #46,
use the exact requirements to make reviewable source declarations, regenerate and
recheck all stages, and freeze any next genuine lifetime/proof dependency. Keep
all forty scenarios Partial and all full benchmark/phase/actual host/user gates open.
