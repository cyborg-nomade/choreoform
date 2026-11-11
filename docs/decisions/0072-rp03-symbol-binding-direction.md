<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0072: Correct the RP-03 finish caption, preserving exact symbol semantics

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-11<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

The actual whole RP-03 temporary source 0.16/C12 promotion stops at
`symbol-mismatch at 0..0`, with no full candidate. Signed
[audit plan](../evaluation/0072-rp03-symbol-binding-audit-plan.md) `b2874c5`
precedes probes; #86 passed final-head CI and merged before the audit began.
Test-only refusal capture locates the exact request: owner `accept_payment`, role
`outcome`, name `authorization recorded`. Its scope interface, companion and join
instead name `authorization-recorded`. The source generator's finish literal has
a space where that declared outcome has a hyphen.

## Decision

Preserve production exact-name/owner/role/wire/inventory semantics and all pinned
profiles/contracts. In this audit, change no generator or benchmark bytes. Next
freeze a separate narrow generator/source correction to make that finish name the
already declared `authorization-recorded`, preserving its stable wire identity,
scope/interface/join and all resources, receipt guards and scenario meaning. Retain
the exact original draft/refusal evidence when publishing a corrected checkpoint.
Use the real source frontend to measure each subsequent failure before any further
repair/adoption or candidate/semantic claim.

Five bounded SDK groups and four Python/native groups separate caption identity
from wire identity and reproduce the original failure. Exact matching captions,
including a consistently declared friendly caption with spaces, can keep the same
hyphenated wire ID. Changing only companion name instead fails the original
interface; aliases collide on wire identity, and distinct extra wires fail source
inventory. A generated-action audit finds this one mismatch among 56 direct finish
declarations, with every generated artifact byte unchanged. Other bindings and
later semantic stages remain unproved.

## Alternatives and consequences

Normalizing spaces/hyphens would change exact name semantics and make future names
ambiguous. Renaming the companion alone leaves contradictory source. Adding aliases
or an undeclared outcome violates existing wire/inventory rules. Hoisting values,
removing a clause or guessing a new core permission is unrelated. The source-literal
correction is the narrowest change consistent with the declared scope and join;
plain-English caption improvements can later change all matching display names
consistently while retaining managed stable wire identity.

The audit adds only bounded cfg(test) thread-local refusal capture and tests/docs;
no runtime/public debug surface or source-literal diagnostic is exposed. Full RP-03
still has no candidate/revision/semantic report and its committed bytes remain fixed.
See [critique/evidence](../evaluation/0072-rp03-symbol-binding-audit.md). Parent #46
draft, forty Partial, G1–G4/full/optional/RP-08/validator/near-English/managed protected
accessible text-visual/actual user/host/browser/independent/phase exits remain open.
No eligible effect, authority, execution, ownership/settlement or Phase 2 follows.
