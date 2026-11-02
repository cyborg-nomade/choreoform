<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Rust authoring frontend implementation plan

**Date:** 2026-10-04<br>
**Specification baseline:** `e65d82d15aa2a5f68d8d0ec60edd6a56d126976a` (PR #19)<br>
**Authority:** Project Owner authorized this continuation after approving ADR-0015.

Freeze this engineering check plan before implementing the next bounded slice.
This is not a new notation ranking or representative-user evaluation. The
ADR-0014 frozen comparison and scenario statuses remain unchanged.

## Scope

Implement the accepted Candidate A grammar in a shared Rust library with exact
UTF-8 spans and bounded work, using the checked-in EBNF as the syntax vocabulary.
Read the portable companion through strict transport, validate its identity and
binding invariants, and compare declarations/symbols/generated slots with source.
Never allocate or reconcile identities implicitly. Lower supported source into
revision-bearing **candidate IR**, with preserved IDs, aliases, annotations and
conservative dependencies. Keep semantic admission/execution explicitly separate.

Exercise source-package formatting/reparse/lower cycles. Retaining or formatting
an original parsed package is not general IR import/export or safe refactoring;
those capabilities require their own evidence. Any unsupported lowering must
refuse explicitly, not drop the construct or fall back to technical JSON.
Full semantic validation and provider/clock enforcement remain other deliverables.

## Required observations

- Differential Rust/Python grammar checks across the accepted positive catalogue
  and lexical/structural negatives, including complete input consumption.
- Source/ledger exact inventory mismatch, unknown/duplicate/retired identity,
  wrong-kind/visibility/qualification, shadowing and missing/generated-slot cases.
- No source-name or positional ID generation, preserving unknown annotations and
  fixed wire symbols through declared rename/reordering/formatting examples.
- Exact source/JCS-ledger/framed-package digest parity with the Python oracle;
  strict JSON, limits and UTF-8 byte spans, including multibyte names.
- Positive candidate lowering, stable repeated source-package cycles and semantic
  versus formatting changes; explicitly state any lowering subset or missing
  general IR exporter. Verify wire shape/JCS independently in Python.
- Native tests plus Wasm compilation/lint, with no IO/network in the shared API.
  Native/Wasm compilation is not browser execution or runtime policy evidence.
- Existing Rust, contract/fixture and text baseline checks remain unchanged.

## Review and stop boundary

Record precise supported operations and limitations beside reproducible tests.
Do not claim syntax/binding success is complete semantic admission or a runnable
process. Do not upgrade the three paper benchmarks, forty Partial scenarios,
accessibility claims or G1–G4. Produce a reviewable implementation PR; pause for
owner approval before its merge or the next implementation slice. Do not start
visual notation, recruit participants or change accepted contracts implicitly.
