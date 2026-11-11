<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0069: Version compensation identity separately from lexical access

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-11<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

The full temporary RP-03 source-0.15 promotion refuses a qualified prior activity
inside a sibling scope. Signed [audit plan](../evaluation/0069-compensation-reference-audit-plan.md)
`e86e2fc` precedes the bounded tests. Existing P1 requires lexical visibility and a
capability activity, with actual eligible occurrence, authority and new logical
effect identity separately required. Raw C11 enforces that visibility too. A
source-only resolver exception would produce raw-invalid definitions.

## Decision

Keep all production behavior fixed in this audit. The next dependency is a separately
planned, versioned raw contract for **only the P1 `compensates` activity identity**:
an exact activity in the same definition may be named independently of lexical
value visibility. Then provide a separately planned explicit readable profile with
an exact qualified path for a nonlexical target, ordinary lexical resolution for
ordinary references, and rejection of ambiguous/missing/wrong-kind/foreign targets.
Neither step grants private data, outcome, control or policy access.

Preserve all old core/source artifacts and refusals. Keep C11's finite owned-work
proof restricted to its immediate incoming source, including its self/unrelated /
other-ancestor refusal. A broader compensation identity cannot widen that proof or
admit RP-03's aggregate surviving root. Any later ownership/correlation extension
needs its own evidence and decision. No full-case IR bypass/adoption here.

An accepted static identity is not an eligible dynamic prior effect: the host must
bind exact prior occurrence/effect/input/basis, verify current authority and any
required accepted capture, create a new logical effect/key, preserve provenance,
retain ownership and obtain actual settlement. Same-site static acceptance can
refer to different dynamic occurrences; it establishes neither temporal precedence
nor a valid self-compensation dispatch. Own effect class remains independent of
compensation purpose. Do not infer restoration, retry identity or erased effects.

## Options and consequences

| Option | Assessment |
| --- | --- |
| Change only the readable resolver | Rejected: raw C11 still refuses the link |
| Hoist private activities/values or omit the compensation clause | Rejected: changes the benchmark's scope or meaning |
| Broaden generic scope visibility or accept all global IDs everywhere | Rejected: exposes unrelated values/control and destroys lexical contracts |
| Version a purpose-only raw identity rule, then its readable binding | Selected: explicit narrow semantics, preserves ordinary privacy and old evidence |
| Implement general live compensation/aggregate ownership now | Deferred: separate requirements and proofs, not demonstrated by this audit |

Five raw SDK and five native/Python groups record actual first failures and positive
baselines; [the critique](../evaluation/0069-compensation-reference-audit.md) states
their limits. No production change, benchmark byte change or live permission occurs.
Parent #46 remains draft, forty scenarios Partial and G1–G4 conditional. Full
validator/benchmark/near-English factoring/protected accessible identity-preserving
text-visual and actual user/host/browser/independent/phase gates remain open.
