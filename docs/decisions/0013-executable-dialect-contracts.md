<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0013: Specify a closed initial type, expression and policy profile

**Status:** Accepted — initial executable contract specification<br>
**Date:** 2026-09-19<br>
**Decider:** Project Owner

## Context

ADR-0008 defines process semantics; ADR-0009 leaves executable dialect payloads
open; ADR-0012 establishes exact type/binding and composition boundaries. Current
fixtures use an illustrative dialect and are not executable. The next Roadmap
deliverable must replace unspecified value/operation/policy labels with concrete
contracts while preserving immutable old artifacts and the later validation gate.

This is not a decision to build the Phase 2 engine. A contract can specify an
evaluation or state transition independently of scheduling, persistence, adapters
and a complete validator. Conversely, a JSON record called “retry” is inadequate
unless it says exactly when retry is eligible and what evidence is required.

## Decision criteria

1. Distinguish exact values, type compatibility, permission, faults and unknown
   outcomes; no implicit widening or authority escalation.
2. Make pure evaluation and all static dependencies deterministic and bounded.
3. Preserve correlation, revision, ownership and late-effect evidence through
   time, retry, cancellation and closure.
4. Specify closed forms with fail-closed unknowns, not a general host-language VM.
5. Keep implementation evidence proportional and label unimplemented rules;
   preserve the near-English authoring goal without selecting syntax here.

## Decision

Adopt the [initial contract suite](../dialects/README.md):

- signed-64 integer coefficients encoded as canonical decimal strings, fixed
  decimal scales 0..18, no floats/implicit rounding;
- exact anonymous record/variant/list/option types and immutable nominal types
  with finite pure invariants;
- a closed pure expression AST with all-branch static checking and dependency
  derivation, one-snapshot evaluation and atomic-result obligations;
- explicit actor/protection/capability/work/effect/fault/time/cancellation/
  settlement/closure/race/membership policy forms, typed context facts and
  specified transition/refusal rules; and
- one immutable suite artifact, with a specified snapshot hash domain, published
  only after owner approval; no mutation or reinterpretation of illustrative
  contracts, old examples or existing instance bindings.

The policy specification names exact shapes, mandatory host checks and state
effects for every existing IR policy-bearing position. Separate provider,
nominal-type and finite-calendar resource shapes have explicit hash rules.
Host clock/identity/provider enforcement is not inferred from a URI: an
unsupported trust/behavior contract prevents execution. New contract support
does not change the IR core's version or turn old payloads into valid new ones.

Accompany the proposal with a small disposable Rust probe of selected pure
judgments. It has no third-party dependencies, parsing, registry or enforcement
authority; its synthetic input facts are not a security API. The full validator,
atomic engine and host adapters remain separate Roadmap work. Accepting this
contract would not certify their existence or close Phase 1's executable-IR gate.

### Resolved review questions — approved 2026-09-21

1. **Exact arithmetic:** start with bounded signed-64 coefficients and explicit
   exact decimal rescaling; reject overflow/inexact results. This is easy to
   reason about, but arbitrary-precision, division and rounding require a later
   explicit extension. Never silently select a money rounding convention.
2. **Information flow:** start conservatively with identical sensitivity/policy
   identity and narrowing access/purpose sets for transfers, including control
   dependencies. No generic declassification. This may reject useful processes;
   authorize later relaxation only with a precise supported contract.
3. **Time and effects:** accept precomputed pinned calendar intervals and explicit
   pause/reschedule rules; require unchanged request identity plus no-effect
   evidence or real stable-key support for uncertain-effect retries. Do not
   equate timeout with provider failure, or timing eligibility with commitment.
4. **Policy expressiveness:** begin with closed policy variants and explicit
   ownership, not user plugins or an embedded general language. Rich calendar
   arithmetic, fault-data ports, general race resolvers/declassification and
   provider-specific extensions need later reviewed contract support.

The Project Owner approved these four decisions. The initial profile is restrictive
and does not claim all corpus paths are executable. The
[worked cases](../dialects/cases.md) separate reproduced functions from paper
transition reasoning and retain explicit gaps.

## Options considered

| Option | Benefits | Costs/risks | Recommendation |
| --- | --- | --- | --- |
| Closed typed/declarative suite | Inspectable shapes; deterministic dependency rules; portable bounded checks | Verbosity and restricted integrations; requires complete validator/host obligations | Adopt initially |
| Embedded host-language code | Familiar rich computation and libraries | Ambient effects, divergent runtimes, weak static visibility and visual parity | Reject as core contract |
| General policy VM/plugin ABI now | More extensibility and reusable policy programs | Sandbox, termination, dependency/authority analysis and compatibility problems before evidence | Defer |
| Keep opaque illustrative payloads | Lowest immediate effort; existing examples unchanged | Does not satisfy executable contract deliverable | Retain only as historical non-executable fixtures |

No measured comparison among alternatives is claimed. Their maturity is paper;
the recommended candidate alone has a bounded subset probe. Implementation
language remains Rust under ADR-0010, not a new language-selection exercise.

## Consequences

The next full-validator work has finite contracts and explicit fail-closed
obligations rather than deciding runtime semantics accidentally. Types and
policies remain domain-neutral; adapters still need exact typed provider and
authority contracts. Names, imports and user-facing syntax remain governed by
their own accepted/proposed boundaries.

Conservative equality, flow and effect rules can refuse otherwise useful work.
Full clinical, financial, security or business-domain correctness is not provided.
Calendar tables can expire and leave timers pending; fault handlers cannot read
new implicit fault-data ports; provider resources are definition-context-specific
in this initial profile. These limits are deliberate and visible, not hidden
defaults. They may require later revised contracts after worked needs are shown.

No new core escape node, arbitrary code evaluator or privilege bypass is added.
Runtime evidence must still record revisions, actors, causal order and ownership.
A boolean test input cannot establish that a provider honors idempotency.

## Confirmation

Follow the [frozen evaluation](../evaluation/0013-executable-contracts.md).
Verify exact values and operations, derive branch dependencies, and inspect
positive/negative authority, transfer, timer, retry and closure cases. Run the
[probe](../../tools/contract-probe/README.md) tests alongside unchanged fixture
oracles. Claims outside the implemented subset remain worked evidence only.

Before execution admission, the later validator must enforce every used rule,
the host must support every pinned contract, and complete benchmark processes
must demonstrate their interactions. Cross-form editing, protected accessibility
and representative-user evidence remain conditional. The proposal does not mark
the aggregate completion gate or final language selection complete.

## Acceptance and action items

The Project Owner approved ADR-0013 and PR #17 on 2026-09-21 with all four
recommended decisions: bounded exact arithmetic; conservative information flow
without generic declassification; explicit pinned time/retry evidence; and a
closed typed policy vocabulary. Merge of
[PR #17](https://github.com/cyborg-nomade/choreoform/pull/17) makes this decision
effective. Approval covers the contract specification and bounded evidence, not
a complete validator, trusted host implementation or engine. G1–G4 and the
aggregate executable-IR completion gate remain open.

1. [x] Obtain approval and resolve the four review questions.
2. [x] Mark Accepted, record approval and update index/Roadmap.
3. [ ] Publish/register the approved content-addressed suite snapshot using the
   exact declared hash domain; never edit old illustrative snapshots.
4. [ ] Convert remaining paper rules into validator/conformance tests and full
   benchmark fixtures in their separately authorized deliverables.
5. [ ] Obtain actual supported host contracts/adapter evidence before execution;
   do not treat the synthetic probe as enforcement or a production engine.

Pause before the next Roadmap item. Final near-English design is still required.
