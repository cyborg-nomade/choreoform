<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Evaluation: Initial executable dialect contracts

**Status:** Plan frozen; results pending<br>
**Plan frozen:** 2026-09-19<br>
**Framework/corpus baseline:** `3e5f79fe23bb5b9ce2490d50fe9c8db376fd7d7a`<br>
**Owner:** Proposal author; decision authority: Project Owner

## Frozen plan

Specify a bounded initial contract for types/values, pure expressions and policy
decisions, under ADR-0008/0009/0012. The deliverable is a reviewable semantic
contract and positive/negative evidence, not the Phase 2 engine or the later
complete structural/link/semantic validator. Do not reinterpret illustrative
payloads or mutate frozen snapshots/fixture revisions. Near-English and visual
designs remain separate. Approval must precede treating new contracts as accepted.

Compare a closed declarative profile against embedded host-language execution
and a general-purpose extensible policy VM at paper maturity. Priorities:
precise rejection/transition semantics, no ambient authority/effects, exact
values and dependencies, explicit uncertainty/settlement, then extensibility.
No language-familiarity preference or weighted total. Only the recommended
profile receives a small disposable Rust evidence probe; no comparative
implementation or usability scores will be invented.

Required artifacts: proposed ADR-0013; concrete type/value and expression rules;
closed policy shapes with context and decision/state semantics; integration map
to every existing policy-bearing IR position; positive/negative worked cases;
bounded pure Rust functions checking selected numeric, dependency, authorization,
time, retry and settlement boundaries; explicit coverage gaps. Keep probe APIs
away from execution admission. No IO, network, provider credentials, plugin VM,
new external dependencies or production integration. Resource refusal is an
error, not a business outcome or permission to discard obligations.

Benchmark pressures: reimbursement RP-01 (Level 1), order RP-03 (Level 3), incident
RP-08 (Level 4), from three domains. Retain all forty scenario-level gaps from
ADR-0012; neither paper examples nor unit functions are full process execution.
Tasks: validate values/assignments; infer expression types/dependencies; inspect
authority and data-flow refusal; trace timers, unknown effects, retry safety,
cancellation and ownership through closure. Perturbations: overflow, missing
data, hidden branch reads, revoked authority, stale/duplicate facts, calendar
revision changes, unknown effect outcomes, concurrent cancellation, changed
evidence, long-running version upgrades and large fan-out.

Roles: author/reader inspect meaning without assuming programming experience;
implementer reproduces judgments; operator distinguishes denial, fault, unknown
outcome and pending settlement. No representative users or independent evaluator
are available. Author both proposes and tests. Evidence grades: at most C for
the exact tested pure functions, B for worked contract rules; never infer user
accessibility or a complete runtime from them. Native tests, format/lint, wasm
compile of the probe, and unchanged fixture oracles are required. Browser
execution of the new probe is not claimed.

G1–G4 stay conditional until complete benchmark, validation, cross-form and
protected-accessibility evidence exists. Exact contract publication/binding and
any incompatible core/semantic version changes must be explicit before use.
Freeze this plan in Git before drafting or assessing results.
