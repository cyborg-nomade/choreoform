<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Reconciliation entry availability: frozen plan

**Date:** 2026-11-06<br>
**Baseline:** `1c6718c5c2afd23f5e7609ba34f77cafb44a06db` (PR #41)<br>
**Authority:** ADR-0018 delegation; core P6, ADR-0030/0031.

The pinned P6 rule creates the designated no-port follow-up on first obligation
registration, before disposal of its current owner, and keeps it as a separate
surviving ownership boundary. Neither a parent's ordinary completion snapshot nor
its input/initializer values are thereby guaranteed retained throughout follow-up
work. Do not reuse the handler's live-ancestor-input premises for this boundary.

Publish ADR-0032 and a conditional reconciliation-body availability stage after
declaration links. For each actual scope reconcile-closure use, check its declared
follow-up body with an empty inherited cell set, fresh locals, existing initializers
and ordinary child rules. Never assume parent inputs, results, owner initializers,
metadata, other follow-up locals or actual transferred obligations are available.
This intentionally conservative supported subset may refuse valid richer snapshots;
it is not a language-wide prohibition of ancestor access. Later retention/snapshot
proofs need an explicit contract and evidence, not a safe flag.

Keep one bounded graph/initializer index/budget for these entries. Generalize the
private entry-checker's diagnostic parameter so handler refusals remain unchanged
and reconciliation failures use reconciliation-availability-unproved at actual
declarations. Expose immutable owner/closure/follow-up references, conditional
checked nodes/scopes and total uncovered nodes separately from ordinary/handler
coverage. Check declared follow-up bodies without claiming actual activation,
ordinary reachability, lifetime or authority. Reports and native expanded output
remain bounded and non-executable; old APIs/contracts/source versions stay intact.

Tests: no-port self-contained body success; parent input/initializer/ordinary-result
and peer-initializer refusals after declaration-link success; fresh local constants
and ordinary child calls; ordinary proof of the same template cannot substitute
for reconciliation entry; uncalled/nested/shared role accounting; immutable exact
refs, meaningful budget bounds, and owned source format/report cycles. Preserve
existing handler diagnostic tests and source timer/retry behavior; a follow-up
timer requiring an unproved parent basis may refuse at this later stage honestly.

Run relevant native/Wasm and artifact/source/stage checks, review the completed
diff separately, evaluate all PR comments and record delegated acceptance, then
require passing final-head CI/expected-head merge. Keep exceptional/follow-up
protection/invalidation/concurrency, actual storage/retention/current authority,
accepted handoff/correlation/settlement and full validator/benchmark/text/visual/
protected accessible editing/actual user evidence open. No phase exit or engine.
