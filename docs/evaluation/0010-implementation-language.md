<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Implementation-language decision evidence

**Status:** Research in progress<br>
**Plan frozen:** 2026-09-04<br>
**Repository baseline:** `9ad094264a10a636ed90a866b061560102518959`<br>
**Evaluator:** Proposal author; Project Owner approval pending

## Question and method

Select the first implementation language for Choreoform's shared semantic core,
parser/validator, CLI, language-service backend, and initial reference interpreter.
Do not select a grammar, parser framework, Studio shell, persistence backend,
distributed runtime, extension ABI, or deployment platform in this decision.

Compare TypeScript, Rust, Go, Kotlin/JVM, and Python using official language and
tool documentation at the same desk-research maturity. No candidate has a
Choreoform implementation in this comparison. Existing Python IR tests validate
the specification fixtures, not Python's superiority as a product language.
Team familiarity and target-user deployment constraints are unknown unless the
owner supplies them. No numerical ranking, performance claims, or aggregate
score will be inferred from ecosystem descriptions.

This is implementation-technology research, not a competing semantic or notation
proposal under ADR-0007. It borrows that framework's evidence discipline without
claiming its twelve-criterion scorecard or five gates have been completed.
ADR-0008/0009 meanings, forty scenario statuses, and conditional gates remain
unchanged. Representative pressures are RP-01 revision-bound payment, RP-03
parallel obligations, and RP-08 dynamic scope/protection; no runnable scenario
support is established by choosing a host language.

## Common criteria, in priority order

1. Preserve explicit state variants, stable IDs, immutable revisions, and effect
   boundaries without host-language behavior becoming process semantics.
2. Make a small, testable Phase 1 language toolchain practical to build and
   maintain; account for unknown team experience rather than assume it.
3. Support recoverable parsing, diagnostics, and LSP integration independently
   of a particular editor, while leaving the grammar/framework choice open.
4. Keep one semantic implementation reusable across native tools and a possible
   browser Studio, without deciding that Studio must run locally in a browser.
5. Provide a credible path to a local reference interpreter, explicit I/O,
   controlled concurrency, and cross-platform distribution.
6. Preserve free-software development, reproducible builds, dependency review,
   and an implementation-independent IR/conformance boundary.

Hard exclusions: dependence on proprietary bundles or a hosted vendor service;
silently changing IR semantics to suit a host; treating host scheduling, type
checking, or memory safety as proof of process correctness. Performance,
startup time, build time, artifact size, usability, and hiring costs remain
unmeasured for every candidate. Any recommendation must identify what changed
priority or evidence would reverse it.

## Findings

Pending primary-source research against the frozen criteria above.
