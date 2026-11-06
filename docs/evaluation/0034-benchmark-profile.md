<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Full-benchmark source profile evidence

**Date:** 2026-11-06<br>
**Plan:** [frozen dependency plan](0034-benchmark-profile-plan.md), `7dc428a`<br>
**Decision:** [ADR-0034](../decisions/0034-benchmark-authoring-profile.md)

The original RP-01 draft remains source 0.4 on its parent branch. Its token and
small endpoint refusals remain reproducible. Tests explicitly promote copies to
source 0.5 / companion 0.4, rather than changing old admission or publishing a
silently migrated draft. Whole-source lowering produces nine scopes and matching
format/reparse IR. Measured native recognition: 8,945 tokens / 101,388 pattern
visits; independent Python grammar: 160,098 states. These are deterministic work
counts for distinct algorithms, not timings. The full draft still fails static
information flow at `checks_request_step`; this dependency does not fix its ACL.

Three additional Rust groups cover whole-draft lowering, immutable resource bytes,
strict pairs/inherited base, semantic identity versus package integrity and format
cycles, legacy/unknown/fragment/wrong-prefix quotas, exact 16,384-token boundary
and adversarial recognizer-depth refusal. Seven Python groups cover full grammar/
schema/JCS/revision/integrity cycles; exact declared root capability/policy and
child activity/follow-up; unused/incompatible sites and wrong qualifiers/kinds;
private data/general outcome refusal; declaration order, coordinated Unicode
alias identity and 201 declared uses; timer/observation owner equality; nested
non-designated and non-wait endpoints. Existing source 0.2/0.3/0.4 regressions pass.

The isolated endpoint witness intentionally has an ordinarily unvisited activity
and explicit compatible protection/invalidation declarations. It passes through
conditional reconciliation availability, reports its unvisited use separately
and remains non-executable. Invoking that child still refuses
`scope-lifetime-unproved`. This distinguishes existing core declaration support
from the still-unproved ordinary surviving-scope composition; no live ownership,
activation, retention or host authorization is asserted.

Passed 258 Rust tests plus five compile-fail doctests, format/native workspace
lint, all five library Wasm lints, release portability/frontend Wasm builds,
native portability report, original draft probes and every CI Python oracle.
The exact-boundary test added during review was rerun with all seven RP-01 Rust
groups. Logs: `/private/tmp/profile5-tests.log`, `/private/tmp/profile5-native.log`,
`/private/tmp/profile5-wasm.log`, `/private/tmp/profile5-report.log`,
`/private/tmp/profile5-oracles.log`, `/private/tmp/profile5-final-focused.log`.
CI also runs the new seven-group oracle at the final head.

Separate completed-diff automated self-review checked explicit known-header quota
selection, bounded recursion/work, strict pins/pairs, ordinary error precedence,
exact descendant/direct-child ownership, all declared uses including unvisited
ones, indexed rather than per-endpoint scans, deferred validation and literal/span
privacy. It corrected the frontend README's previously universal legacy-budget
wording and added exact token-cap evidence. No actionable issue remains. Acceptance
is under owner delegation, not independent or individual owner review. Require
passing final-head CI, comment audit and expected-head merge into draft PR #46;
main effectiveness waits for parent integration.

Next adopt the profile explicitly in that parent, regenerate real canonical IR,
resolve its separate static policy mismatch and record all later stage results or
refusals before package readiness. All forty scenarios remain Partial; full
validator/proof, complete RP-01/03/08, text/visual/protected editing, independent
host/browser and actual representative-user gates remain open. No phase exit.
