<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Dependency merge-message sign-off correction

**Date:** 2026-11-06<br>
**Authority:** AGENTS.md sign-off requirement and ADR-0018 delegated workflow.

The agent omitted an explicit merge-message body when merging dependency PRs
#47/#48/#49 through GitHub. Their generated merge messages have no `Signed-off-by`
trailer. This is a workflow exception, not a claim that those trailers exist:

| PR | Merge commit | Signed feature head |
| --- | --- | --- |
| #47 | `3ba30c1fc7a2a4781c3f7db990922a63f0070669` | `5ba9c84ac0d03ee1375652d46ddfffd1d2c1e510` |
| #48 | `ffe0c2cd38327003b0521a9c3a117afb4644d7ee` | `e4ec167234848e7477fa62bdded0f2f87464d845` |
| #49 | `c759741741922cb08f43793d7ed3f7645ca16f29` | `4dd80eaae237e7e6c6f11e57bfd8567cab9bfa5b` |

Each merge tree equals its signed feature-head tree; the merges introduce no
additional source changes relative to those signed commits. All authored
non-merge commits since the draft's main baseline are signed off. Verify trees
and messages with Git; no independent or personal owner review is asserted.

Preserve published hashes and review/CI references rather than rewriting history.
Subsequent GitHub merges must receive an explicit body file containing the
configured contributor's sign-off trailer, in addition to expected-head checks.
This record does not retroactively amend the three messages or waive the project
rule. Include the exceptions in phase-level workflow accounting.
