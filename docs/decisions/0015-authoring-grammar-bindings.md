<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0015: Specify Candidate A grammar and portable identity bindings

**Status:** Proposed<br>
**Date:** 2026-10-03<br>
**Decider:** Project Owner

## Context

[ADR-0014](0014-near-english-authoring.md) accepted structured English steps as
a provisional direction. Its paper forms left map/list syntax, complete policy
clauses and the portable identity companion unspecified. Implementing a frontend
against those placeholders would force unreviewed choices about identity,
resource pinning and outcome aliases. Human-facing outcomes such as “correction
needed” also need explicit links to the IR's restricted wire tokens.

The existing Rust parser implements the separate JSON-record profile. Accepted
IR and executable contracts already constrain what a new surface may mean.
Full RP-01/03/08 lowering and representative-user study remain open; this step
supplies a concrete, checkable specification before substantial compiler work.

## Decision criteria

1. Express every existing core action, type/expression operation and policy
   without raw JSON or semantic extensions in ordinary source.
2. Preserve stable IDs, outcome/port/purpose symbols, exact resource pins,
   conservative reads and unknown annotations across explicit edits.
3. Give source and package integrity precise domains distinct from IR meaning.
4. Make syntax and identity invariants reproducibly testable without claiming
   full lowering, host enforcement or measured accessibility.
5. Keep the proposal reversible and the authoring completion gates open.

## Decision

Propose the [Candidate A authoring profile 0.2.0](../authoring/profile.md), with
the [complete token grammar](../authoring/candidate-a.ebnf), and the
[portable binding package 0.1.0](../authoring/bindings.md) with a closed schema.
Require explicit ordered scope interfaces and policy clauses, bracketed maps,
typed decimal literals and parenthesized compound operands. Preserve Candidate
B as the later matched study alternative, without claiming a complete B grammar
in this iteration. Neither profile replaces the accepted JSON-record baseline.

Keep declaration identities and source aliases in the companion; store generated
record IDs by owner plus semantic slot, and preserve retired IDs as tombstones.
Bind wire symbols explicitly, including outcome references in variants, joins
and waits. Fully qualified source references spell the entire owning scope
chain. Qualified join outcomes must not disguise population-wide wire-token
matches; require all matching member outcomes to be explicitly represented.

Pin the accepted semantic/core-suite snapshots in the companion and map those
pins into IR semantics/dialects. Resource aliases select explicit local pins,
with no network discovery. Preserve the full IR annotation object, and expose
extra conservative node reads as `Also read`, rather than silently dropping them.
Compute source, companion and length-framed package hashes separately from the
existing IR semantic revision. Require explicit create/import/rename/reconcile/
copy/delete/move/merge operations; stale identity never repairs itself.

Use a bounded Python EBNF recognizer and ledger oracle as **test infrastructure**.
Do not introduce a production Python compiler, runtime or new dependency. The
shared implementation remains Rust. These tests check only the properties
listed in the [evaluation](../evaluation/0015-authoring-grammar-bindings.md).

## Options considered

| Option | Advantages | Costs and risks | Recommendation |
| --- | --- | --- | --- |
| Complete source grammar plus tool-managed companion | Human source has no opaque IDs; identity/pins survive plain-file export; testable boundary | Two-file transactions and explicit reconciliation need tooling | Adopt provisionally |
| Inline IDs/anchors in every declaration | Single file; identity easier to see during raw edits | Adds technical bookkeeping to ordinary prose; still needs symbol/generated/resource rules | Keep for identity-editing study comparison |
| Regenerate IDs from names or positions | Simple one-file implementation | Renames/reordering alter lineage; creates unsafe apparent continuity after deletion | Reject |
| Implement a full compiler before specifying the format | Immediate end-to-end behavior might reveal more design issues | Much larger unreviewed choices and surface area; no full validator exists yet | Specify and test this boundary first |

## Review questions and recommendations

1. **Source shape:** accept bracketed explicit maps and fixed clause order for
   this iteration? Recommend yes: this makes boundaries inspectable while
   preserving sentence-led actions. Test shorter spellings with users before
   any final compatibility commitment.
2. **Symbol aliases:** use explicit `outcome "name" of "owner"` in typed cases,
   joins and wait rules? Recommend yes: spelling equality cannot prove identity.
   Ordinary `On`/`Finish` remains short because the owner is local and unambiguous.
   Outcome/purpose expression atoms also keep comparisons in human-facing aliases.
3. **Identity and integrity:** accept the separate companion, tombstones and
   framed package digest? Recommend yes, provisionally; it supports ordinary
   text plus VCS, but adds transaction/reconciliation requirements. Run the
   already planned inline-anchor editing comparison before freezing the format.
4. **Next step within this deliverable:** implement the bounded Rust frontend,
   source/ledger consistency and deterministic lowering with round-trip tests?
   Recommend yes after approval, with full-benchmark contract gaps tracked
   explicitly. Visual notation remains a later, separately authorized item.

## Consequences

Implementers have a finite grammar and durable slot/key vocabulary. More explicit
type/outcome syntax increases reading effort; its accessibility is still a
hypothesis. Parenthesized expressions and complete policy clauses can be dense,
so editor presentation and user testing matter. A plain editor is sufficient to
edit source, but identity-changing edits require explicit reconciliation tooling.
Package hashes detect changed artifacts, not malicious or unauthorized changes.

The recognizer can accept ill-typed or unresolved fragments. The ledger checker
cannot establish that source and companion agree. Full validation, safe source
rewrites, IR import/export, merge behavior, protected views and supported host
contracts remain implementation requirements, not evidence supplied here.

## Confirmation

Run `tools/check_authoring_profile.py` in the existing hash-locked Python oracle
environment. It interprets the checked-in EBNF, exercises all eleven action
forms and twelve policy variants, type/expression forms, lexical refusals and
exact decimals. It checks schema/identity/tree/slot invariants, literal Unicode
lookup, transactional ledger-only rename, wire aliases, exact pins and a fixed
joint-hash vector. Existing Rust/native/Wasm and IR/text regressions must remain
unchanged. The [evaluation](../evaluation/0015-authoring-grammar-bindings.md)
records what those checks do and do not establish. No user study, full benchmark
or new semantic engine result may be inferred from passing these tests.

## Acceptance and action items

This proposal becomes effective only after Project Owner approval and merge.
The main authoring Roadmap item and G1–G4 remain open.

1. [ ] Review the four questions and approve or amend this bounded proposal.
2. [ ] Record approval/status/index and merge after the required checks.
3. [ ] In separately authorized continuation, implement Rust source/ledger
   admission, lowering, identity-preserving export and round-trip tests.
4. [ ] Complete full RP-01/03/08 source/IR benchmarks and resolve their recorded
   calendar/aggregation/ad-hoc/import requirements without semantic shortcuts.
5. [ ] Prepare equally complete study tasks for A/B and coordinate representative
   participants; assess evidence before final syntax or next Roadmap work.
