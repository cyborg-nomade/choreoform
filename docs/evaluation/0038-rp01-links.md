<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-01 precise source-link checkpoint

**Date:** 2026-11-06<br>
**Plan:** [frozen integration](0038-rp01-links-plan.md), `5beea02`<br>
**Dependency:** ADR-0038 / #51, merged into this draft parent

The fixture now declares exactly the 72 measured missing ordinary site/cell
links. `tools/benchmarks/rp01_invalidation.py` stores the reviewed mapping and
renders qualified readable references using the unchanged ledger. It refuses a
changed/already-patched baseline. This is controlled fixture assembly, not a
general authoring edit facility. Native comparison against the retained unpatched
fixture proves that no old link is removed and no other semantic body field,
binding, resource or input artifact changes. Existing conservative extra links
are retained. The historical metadata fixture explicitly remains unpatched.

The source is 56,185 bytes / 9,327 measured tokens. New semantic revision:
`sha256:d0567026a71a1e7808168f6e1e17f27cb6f8272c1d9b49f189439e12c9e6be05`.
Canonical IR byte digest:
`sha256:76a65e2593be2c5ce372aca4802e7a944ae1eab085e824c917685eff42274213`.
Advisory requirements remain 84 with zero missing and the same 88 conditional
historical records. Fresh strict static/control/protection/invalidation pass;
their reports and revised hashes are generated from actual native commands.
The next six composed stages refuse
`scope-lifetime-unproved at /body/nodes/run_checks; source bytes 40983..41175`.
The source span is the actual Run checks declaration, not a guessed location.

The called service's reconcile closure is preserved. A narrow terminal-only
concurrency prerequisite currently refuses every surviving called boundary.
Passing invalidation neither proves that surviving lifetime safe nor authorizes
later checks/execution. Audit that dependency before changing analysis. No scenario
or phase gate closes; all forty scenarios remain Partial.

Local verification passed 283 Rust tests plus six compile-fail doctests, format,
all CI Python oracles and exact regeneration/historical profile probes. The raw
metadata regression now checks fresh invalidation success plus the separate
lifetime refusal, then explicitly removes disposition links and retains its
original coverage counterexample. No negative is silently discarded.
Logs: `/private/tmp/rp01-links-rust.log`, `/private/tmp/rp01-links-oracles.log`.
Final-head required CI is still required for the checkpoint.

Separate completed-diff automated self-review checked exact body set difference,
stable identity/resource/input preservation, source token measurement after link
rendering, strict status/revision/digest regeneration, original missing-coverage
negatives, qualified ancestor visibility and unchanged lifetime policies. It fixed
an initially stale pre-patch token measurement and made an old raw-metadata test's
missing-link premise explicit. No actionable issue remains in this integration.
All parent comments/threads were inspected: CodeRabbit skipped the draft, with no
substantive findings or inline threads; no independent evidence is claimed. Draft
#46 remains unmerged and incomplete. Next freeze and audit surviving called-scope
lifetime; do not start Phase 2.
