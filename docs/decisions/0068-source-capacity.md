<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0068: Version measured full-program source capacity without wider semantics

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-11<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

The actual full declared RP-03 source has 505,980 bytes / 77,135 tokens. Source 0.14
refuses its fixed token quota before complete recognition. Signed [plan](../evaluation/0068-source-capacity-plan.md)
`ae8a44e` precedes implementation/probes. Old benchmark/contract/profile bytes and
all semantics are fixed; a broad budget increase or private-access exception would
confound the measured dependency.

## Decision

Add only explicit source 0.15 / companion 0.14 selecting existing exact core 0.11 /
IR 0.4, with identical grammar productions/schema structure/features/report meanings.
Allow that known full-program header 131,072 lexer tokens, the next power-of-two
bound above the measured whole case. Keep every old quota and byte/depth/parser /
lowering/semantic/shared/report budget. Reject unknown versions and mismatched pair /
core/artifacts. Legacy fragments stay at 4,096; source 0.5–0.14 stays at 16,384.

The native full promotion now recognizes in 832,138 pattern visits under the old
one-million work cap, but lowering refuses a qualified compensation reference into
a sibling's private scope at 60937..61023. Keep this actual first binding gap;
there is no candidate or semantic-stage evidence. The Python recognizer separately
hits its unchanged state cap; do not raise it or claim agreement on full recognition.
Its lexer now matches a compiled token regex at the current source offset rather
than copying every remaining suffix, avoiding quadratic copying at the new bound
without changing tokens. Small paired programs have identical semantic documents /
reports while exact source-package identities differ.

## Alternatives and consequences

Changing source 0.14 would alter a frozen profile. Truncating the case or dropping
protected labels/receipt clauses would change meaning. Raising all work/transport
limits or allowing private sibling references as a side effect would hide the next
real dependency. Source 0.15 exposes it without doing those things. Any missing
purpose-specific binding/semantic contract needs a separate frozen ADR/profile;
ordinary private data/outcome access stays closed. A later controlled benchmark
adoption must be planned separately; current RP-01/RP-03 bytes remain unchanged.

This capacity decision is not a readability solution. Reusable near-English
formulations, stable identity management/export, protected accessible summaries /
visual round trips and actual representative users remain required in Phase 1.
Complete validator/benchmarks, actual host/browser/independent evidence, G1–G4 and
phase exits remain open; all forty scenarios Partial, parent #46 draft/incomplete.
No accepted event/activation/storage/authority/handoff/settlement or Phase 2 follows.
