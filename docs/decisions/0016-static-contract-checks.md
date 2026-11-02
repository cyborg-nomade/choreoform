<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0016: Separate static contract checks from executable admission

**Status:** Accepted — bounded static-check stage<br>
**Date:** 2026-11-02<br>
**Decider:** Project Owner

## Context

PR #20 implements bounded Candidate A lowering under ADR-0015. The owner accepted
its recommendations and authorized the next contract-reader/admission item.
ADR-0013 specifies finite type, expression, policy and external-resource formats,
but a complete validator, planner and trusted host do not yet exist. A new
checker must make useful progress without making its success look executable.

The accepted suite specifies named-type/provider/calendar formats, but explicitly
does not standardize a clock provider. It cannot be implemented by accepting an
arbitrary hashed clock JSON object or a caller-supplied “supported” Boolean.

## Decision criteria

1. Implement accepted closed judgments without changing frozen semantics/artifacts.
2. Preserve exact identity, all-branch dependencies and fail-closed unsupporteds.
3. Keep checked artifacts distinct from permission to plan or execute.
4. Provide bounded native/Wasm evidence and inspectable residual obligations.

## Decision

Add a shared Rust static-check stage, separate from syntax, candidate lowering,
full-definition admission and planning. Its immutable report binds the original
IR revision, counts checked records and lists residual obligations. Name the
result `static-checks-passed`, never “executable”, “validated definition” or an
execution capability. Preserve explicit `lower` versus `check` commands/API calls.

Implement the accepted closed type/value and expression vocabulary, exact nominal
identity/invariants, typed policy predicates, policy/read dependency closure and
selected reference/interface/transfer checks. Only actual fanout item sites bind
process parameters; other process records have none. Check every static branch.
Evaluate only pure nominal invariants; there is no general process evaluator or IO.

Read exact caller-supplied named-type/provider bytes before enabling their candidate
lowering. Provider references include protection, actor and capability dependency
closure; they are not opaque dependency-free attachments. Calendar decoding checks
finite intervals and exact provenance bytes only. **Keep non-null timers and
clock/calendar source lowering unsupported** until a clock contract and host
implementation have explicit approval/evidence. Neither JSON shape nor digest
certifies provider/calendar trust or host operational support.

Reuse the existing envelope and selected structural checks without broadening the
historical `inspect` registry. A transport-only in-memory check and an explicit
envelope structural-check method may be public without manufacturing semantic
authority. No frozen snapshot, wire version or semantic rule changes here.

## Options considered

| Option | Advantages | Costs and risks | Recommendation |
| --- | --- | --- | --- |
| Separate bounded static report | Useful closed checks now; no false execution capability; staged review | Another explicit stage; residual judgments still required | Adopt |
| Call the new subset full admission | Fewer API stages | Implies output/protection/race/host proofs that do not exist | Reject |
| Keep all resources opaque/unsupported | Smallest code surface | Blocks named types/provider dependency evidence and the next benchmarks | Retain only for unsupported formats |
| Invent a clock profile or trusted-host flag now | More examples appear lowerable | Introduces unreviewed semantics/trust; hashes cannot prove enforcement | Defer to a dedicated supported-clock proposal |

## Consequences

The new stage rejects many ill-typed candidates and exposes resource dependencies
before a planner exists. API consumers must still inspect the stage boundary:
a report, revision, typed helper or verified pin is not an execution credential.
The implementation is a bounded prototype, not a compatibility or scale promise.

Complete control/definite-output analysis, control-sensitive information flow and
access authorization, race/invalidation/reconciliation proofs, stable rich source
diagnostics, host support and atomic runtime judgments remain open. Direct transfer
checks do not establish the whole information-flow judgment. No statement of
usability, accessible editing, browser execution or complete benchmark evidence
follows. The near-English item, full validator, G1–G4 and Phase 1 exit stay open.

## Confirmation

Follow the committed [preimplementation plan](../evaluation/0016-static-check-plan.md).
[Evidence and review questions](../evaluation/0016-static-contract-checks.md) identify
positive/negative observations, scope and remaining work. Use all-branch and
nominal/registry mutations, a provider/named-type source package, independent
hash/schema/reparse checks, old regressions and native/Wasm lint/build. Preserve
explicit rejection of validly hashed unknown clock resources.

## Acceptance and action items

The Project Owner reviewed and approved PR #21, including this ADR, on 2026-11-02.
The accepted boundary is a separately named static stage, supported named-type/
provider candidate lowering, and continued refusal of unsupported clocks/timers.
The remaining full-definition/host/benchmark judgments retain their stated priority
and open status. Merge of [PR #21](https://github.com/cyborg-nomade/choreoform/pull/21)
makes this decision effective. The invocation-output protection finding was fixed
in `0568151`; its thread is resolved and required CI passed.

Approval closes this item's review pause, not the broader Phase 1 gates. Starting
another implementation item still requires separate owner authorization.

1. [x] Review the stage boundary, resource/clock policy and follow-up priority.
2. [x] Record owner approval/status/index before merging.
3. [x] Supply bounded implementation and reproducible tests/evidence.
4. [ ] Complete the separately tracked full-definition/host/benchmark gates; this
   prototype does not mark them done or start Phase 2.
