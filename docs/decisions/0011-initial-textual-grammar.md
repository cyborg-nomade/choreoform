<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0011: Prototype an explicit declaration-oriented textual grammar

**Status:** Proposed<br>
**Date:** 2026-09-07<br>
**Decider:** Project Owner

## Context

ADR-0009 defines a structural IR and ADR-0010 selects Rust. The bounded
native/browser confirmation is accepted. The next Roadmap item is an initial
textual grammar and parser. Expression, type, policy, visual, and full validation
decisions remain open. A parser must not fill those gaps with implicit execution
rules or make a successful parse look like permission to run a process.

Readers need to locate declarations, identities, and protection decisions;
tools need exact preservation, deterministic lowering, and original-source
locations. The existing JSON excerpts provide repeatable evidence, but do not
establish a pleasant authoring experience or complete corpus coverage.

## Decision criteria

1. Preserve all accepted IR fields, stable identities, ordered values, unknown
   annotations, and exact immutable contract bindings.
2. Reject ambiguous transport and duplicate declarations without lossy repair.
3. Separate source syntax, unvalidated IR, semantic admission, and execution.
4. Retain original source and useful byte spans with explicit artifact identity.
5. Improve navigability while keeping this initial experiment easy to replace.

## Decision

Propose **text profile 0.1.0** as an experimental explicit declaration surface,
specified in [the grammar and lowering contract](../text/README.md). Require a
source-version header, five metadata items, and all eight declaration sections.
Within sections, stable IDs label strict JSON object records. Preserve JSON
payloads for all fields rather than inventing a type/expression/policy DSL now.

Use a small bounded hand-written scanner in the safe Rust
`choreoform-text-prototype` crate, reusing the existing strict JSON transport and
canonical revision functions. Keep its source tree separate from IR. Retain
exact source bytes and item/record byte spans; bind extracted spans to both the
source digest and lowered semantic revision. No additional third-party package
is introduced; reuse of a probe crate is temporary, not a production component
boundary commitment.

Lowering synthesizes only the fixed IR envelope and calculated revision.
All semantic body fields and empty sections are explicit. Declaration order,
whitespace, and source comments do not determine behavior. The output is an
**unvalidated IR candidate**; a hash does not certify semantic correctness.

Provide an explicit normalized IR-to-text exporter as evidence. It preserves
the complete IR, including unknown annotations, but does not recreate source
comments or formatting. It must not be represented as a lossless source
formatter or automatically overwrite the source. Exact original-source
preservation is supplied separately by the syntax tree.

This is a working prototype proposal, not final syntax selection or completion
of Phase 1's cross-form round-trip deliverable. G1–G4 remain conditional. Do not
introduce syntax compatibility commitments until the missing evidence is closed.

### Authoring goal clarified during review — 2026-09-08

The Project Owner wants the authoring language as close to plain English as
possible, for broad accessibility. This prototype does not meet that goal and
must not become final syntax by default. The proposed follow-up is controlled
English: familiar sentences with precisely defined constructs, deterministic
parsing, and explicit handling of ambiguous meaning. Ordinary authors should
not need JSON records or manual wire-format bookkeeping. Tool-managed stable
IDs must still survive edits and text/visual transformations; convenient wording
must not hide policy, authority, or consequential effect choices.

The [Roadmap](../../ROADMAP.md#phase-1--semantic-core-and-language-prototype)
now makes near-plain-English design and representative-user evaluation a
dedicated required deliverable before final syntax/Phase 1 exit. Rich authoring
syntax is therefore required follow-up work, not merely an optional alternative.
The exact grammar and parser framework remain a later reviewed decision. No
human-usability evidence is retroactively attributed to this JSON-based probe.

The executable IR completion obligation remains unchecked. It was deliberately
split from structural approval in PR #11, but its placement obscured its later
dependencies. The Roadmap now separates that aggregate gate from its concrete
deliverables. Proposed next work is binding/type foundations, then accepted
executable dialect contracts, before English-like and visual authoring designs.
Neither this clarification nor tentative agreement constitutes owner approval
of PR #14 or permission to start the next deliverable.

## Options considered

| Text option | Advantages | Costs and risks | Proposed disposition |
| --- | --- | --- | --- |
| Continue authoring raw IR JSON | No second notation to learn; complete representation; existing tools | Wire envelope and declaration-map punctuation remain exposed; no source-tree prototype | Retain as interchange and baseline |
| Explicit declaration sections with JSON records | Visible section/ID boundaries; exact mapping; comments between items; small replaceable parser | Still verbose and reference-heavy; two punctuation conventions; no evidence of improved comprehension | Prototype now |
| Near-plain-English behavioral DSL with explicit control flow and expressions | Familiar wording and potentially much less wire-format work for authors | Requires decisions about identity allocation, scope, ambiguity, policy visibility, and expression contracts; attractive examples may hide required semantics | Required dedicated follow-up design and user evaluation; this prototype does not fulfill it |

These alternatives have unequal maturity. Only the proposed surface has a
parser experiment here; no comparative usability ranking is claimed.

| Parser approach | Advantages | Costs and risks | Proposed disposition |
| --- | --- | --- | --- |
| Bounded hand-written scanner plus existing JSON reader | Small dependency-free extension; direct spans and limits; inspectable control flow | Grammar/code drift and boundary bugs are our responsibility; poor foundation for rich recovery without more work | Use for this grammar only |
| Parser combinators | Composable grammar-shaped functions; reusable parsing abstractions | Dependency/API choice and recovery/span integration need separate evidence; little payoff demonstrated for this tiny outer grammar | Reassess as syntax grows |
| Generated grammar | Declarative grammar and generator-level grammar checks, depending on formalism | Generation/build workflow and source-preserving recovery still need design | Reassess before a substantially richer grammar |

## Consequences

The three frozen IR excerpts can be inspected as source with stable IDs and
lowered to the same semantic revisions. No policy is supplied by a parser
default. Original comments survive retaining the source tree, while explicit
IR-only export is clearly a different operation.

This syntax may prove too close to the wire format for domain authors. It is a
baseline to criticize with examples, not a claim that human legibility is solved.
Deep record field spans, multiple-error recovery, incremental reparsing,
format-preserving edits, modules, syntactic sugar, and a language server are
deferred. Whole-payload diagnostic spans may be coarse. Unknown record fields
and dangling references require the later validator; parsed policies remain
opaque. The prototype must never be used for execution admission.

## Confirmation

See the [frozen plan and results](../evaluation/0011-textual-grammar.md).
Tests cover exact excerpt round trips, repeat export stability, original-source
and annotation preservation, spans, ordering, revision changes, malformed input,
and resource refusal. Native tests and a wasm library compile/lint check exercise
the core boundary. Browser execution of this parser has not been demonstrated;
ADR-0010's existing browser probe does not establish that result for new code.

Representative-author comprehension and editing tasks, full benchmark processes,
equivalent visual notation, executable dialects, protected accessible editing,
and stronger diagnostics remain required before final language selection.

## Acceptance and action items

This proposal becomes effective only after Project Owner approval and merge.

1. [ ] Obtain owner review of the explicit/verbose baseline versus richer syntax.
2. [ ] Record approval, mark Accepted, and update the index and Roadmap.
3. [x] Supply the disposable parser, exact specification, excerpts, and tests.
4. [ ] Carry the conditional gates and explicitly deferred work into subsequent
   Phase 1 decisions; approval here does not close them.

Pause before starting the next Roadmap item. The revised sequence proposes
binding/type foundations and executable contracts before authoring/visual design.
