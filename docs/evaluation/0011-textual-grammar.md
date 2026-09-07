<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Evaluation plan: Initial textual grammar and parser

**Status:** Planned<br>
**Framework and corpus revision:** `fe4695fe4c2f3040117d23c98752da4e45a24f0b`<br>
**Plan frozen:** 2026-09-07<br>
**Evaluation owner:** Proposal author; selection authority: Project Owner

## Frozen plan

Evaluate one bounded declaration-oriented textual prototype for exact lowering
to definition IR 0.1.0. Compare JSON-only authoring and a higher-level behavioral
DSL architecturally, without fabricated comparative implementation scores.
The prototype is disposable and the syntax has no released compatibility promise.
Final language selection remains blocked by the existing conditional gates.

Priorities, in order: preserve semantic fields and identities; refuse ambiguous
or lossy input; keep syntax separate from validation and execution; expose source
locations; then assess authoring economy. No weighted total.

The prototype will use explicit versioned text, named declaration sections,
stable IDs, and strict JSON record payloads. All sections, including empty ones,
are explicit. This tests a conservative authoring surface before introducing
expression syntax or control-flow sugar. Alternatives have paper maturity;
the candidate will have prototype maturity. Parser implementation alternatives
are hand-written bounded scanning, parser combinators, and generated grammars;
only the first is planned for this small, replaceable experiment.

Roles: process reader (domain knowledge, no assumed programming experience),
author (explicit ID/reference editing), operator (locating malformed input),
and implementer (lossless transformation). No familiarity advantage is assigned
to any implementation language. Keyboard-readable text is represented; screen
reader, cognitive accessibility, and user comprehension are untested.

Benchmarks: the frozen RP-01 reimbursement (Level 1), RP-03 order (Level 3), and
RP-08 incident (Level 4) IR excerpts from three domains. These remain excerpts,
not complete acceptance scenarios. All forty corpus scenarios remain in scope
for accountability; inherit the individually recorded representations and gaps
in [ADR-0009's evaluation](0009-canonical-ir.md#corpus-traceability), with no
upgrade from Partial merely because a parser can carry a record.

Tasks: export each excerpt; parse and lower back to identical IR and canonical
revision; preserve unknown annotations and ordered arrays; preserve original
source including comments; inspect source spans; edit a policy, rename an ID,
reorder declarations, and edit only comments. Reject malformed UTF-8, duplicate
sections/IDs/JSON keys, unsupported source version, missing sections, malformed
delimiters, float tokens, out-of-range integers, and excessive size/depth.

Operational perturbations (late/duplicate observation, partial failure,
concurrent cancellation, revised evidence, long-running version change, large
fan-out) are inspected only for representation loss. No execution, scheduling,
performance, security-enforcement, or recovery claim will follow from parsing.

Required artifacts: proposed ADR; exact grammar and lowering specification;
Rust source tree distinct from IR; original-source preservation and byte spans;
three source fixtures; reproducible positive/negative tests; native CLI; wasm
compile check. No new third-party dependency is planned. Environment: pinned
Rust 1.98.1 and existing Cargo.lock, one local macOS host, no user study or
independent evaluator. Bound input and lowered wire to 1 MiB and JSON depth to
the existing transport limit; fail-fast syntax diagnostics, no editor recovery
or incremental parsing in this experiment. No wall-clock performance target.

The proposal author both implements and evaluates: evidence cannot exceed C
for reproduced mechanical properties or B for worked mappings. Owner review
is required separately; absence of other ratings is not consensus.

## Results

Pending implementation. G1–G4 remain Conditional with the same owners and
closure conditions as ADR-0009. G5 requires reproducible commands and observed
results before this proposal is submitted for review.
