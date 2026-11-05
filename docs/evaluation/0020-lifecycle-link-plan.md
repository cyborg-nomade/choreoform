<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Lifecycle policy use sites: frozen implementation plan

**Date:** 2026-11-05<br>
**Baseline:** `be697012c15374ca05082a235d15cd16bd442e45` (PR #26)<br>
**Authority:** Owner delegation, ADR-0018. Commit before implementation.

Implement accepted policy/link judgments at actual definition use sites, before
broader race/invalidation analysis. Fault-handler and reconcile-closure target
scopes must be direct lexical children of the scope using the policy, not merely
children of the policy's declaration scope. Inherited terminal/empty-handler
policies remain reusable; do not blanket-ban inherited policies.

Effect reconciliation is a non-node wait-policy use. Require an attached declared
wait node with matching policy/outcome ownership, not an unattached typed wait
record. Use an indexed map of wait-policy users. Zero owners refuses; multiple
possible definition owners refuse as unproved rather than guessing which owner
the effect means. Ordinary wait-policy reuse without a non-node effect use remains
valid. ADR-0020 will record this conservative inference boundary and alternatives.

No claim of live receiver/occurrence identity, accepted ownership transfer, exact
runtime correlation, authority or atomic settlement. No new wire fields, fake
"safe" flag, source grammar, frozen-resource migration or executable credential.
Existing report residual gates stay open. Bind refusal paths to actual definition
sites and bound index/traversal work using the existing static budget.

Required evidence: positive and negative inherited structural policies, uncalled
scope checking, missing/ambiguous owner, ordinary wait reuse, existing service
reconciliation, source lowering/format/reparse and current CLI stage refusals.
Self-review completed code separately, run required native/Wasm/oracle checks,
record acceptance under delegation and verify final-head CI before merge. Continue
Phase 1; do not pause for item approval or close aggregate gates.
