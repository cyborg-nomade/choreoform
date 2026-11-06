<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0027: Express retry clock bindings in a versioned English sentence

**Status:** Accepted under Project Owner delegation; effective upon PR merge<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 delegation

## Context and criteria

ADR-0026 introduces explicit core retry-clock/basis fields while preserving old
records. Authors need to express those requirements through readable aliases,
without JSON/wire bookkeeping in sentences or silent changes to existing files.
Preserve identity, exact contract/version boundaries, bounded native/Wasm parsing,
independent grammar/schema/hash checks and the static/live distinction. Actual
nonprogrammer comprehension remains an external evaluation requirement.

## Decision

Adopt [source profile 0.3.0](../authoring/profile-v0.3.md), its separate full EBNF
and companion 0.2.0 schema. The only production change is an optional work clause
before Timeout: `For retries use clock "Elapsed clock", basis "Audit basis".`
Require it for multiple attempts; forbid it for one attempt, which lowers null
fields. Use exact core 0.2.0 and its inherited base, preserving the semantic/IR
versions, permanent declarations, generated slots, symbols and tombstones.

Select grammar and integrity domain by explicit source profile; require exact
source/companion/core combinations. Keep source 0.2.0, companion 0.1.0, old grammar,
schema, examples and digests intact. Fragment parsing keeps its legacy default.
No union grammar, implicit clock, annotations, automatic migration or general
IR exporter is introduced. The CLI includes the new immutable core artifact for
new packages; libraries still require explicit bytes. Derive retry basis and
transitive source authority/protection reads before node footprints.

## Options considered

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Explicit English clause plus new profile/schema | Readable aliases; precise version/identity boundary | One more versioned grammar/schema | Adopt |
| Extend source 0.2.0 or its old companion silently | Less file duplication | Changes meaning/digests and old closed grammar | Reject |
| Infer from timeout or require raw JSON fields | Small compiler work | Implicit domain or inaccessible author workflow | Reject |
| Separate standalone retry declaration | Reusable syntax | Adds identity/link machinery for one work-local binding | Defer unless reuse is required |

The new companion version records the changed core/profile semantics even though
its inventory shapes stay the same. A grammar-delta oracle limits duplication
drift: exactly one existing production changes and one new production appears.
Rust remains selected on project requirements, without familiarity advantage.

## Consequences and confirmation

Known retry bindings become expressible and survive source/format/reparse and
schema/JCS/revision/report cycles. Tool-managed companion allocation/editing,
arbitrary-IR export, user comprehension, full validator/lifecycle/host enforcement
and aggregate Phase 1 gates stay open. Timing reports grant no retry permission.
No runtime clock/fault/key evidence, representative user or browser execution is
claimed. Follow [frozen plan](../evaluation/0027-retry-authoring-plan.md), `474cc35`,
and [evidence](../evaluation/0027-explicit-retry-authoring.md).

Accepted after separate completed-diff automated self-review under delegation.
Local checks passed 195 Rust tests plus five compile-fail doctests, native/Wasm
checks and all relevant artifact/source/stage oracles. No independent human/bot
approval or individual owner review is claimed. Final-head required CI remains
a merge prerequisite.

1. [x] Finish tests and separate automated review; record delegated acceptance.
2. [x] Require final-head CI and expected-head merge.
3. [ ] Continue remaining full-validator/lifecycle and aggregate Phase 1 evidence.
