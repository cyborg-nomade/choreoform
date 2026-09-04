<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0009: Specify a typed graph IR with explicit version and identity boundaries

**Status:** Proposed<br>
**Date:** 2026-09-03<br>
**Decider:** Project owner

## Context

[ADR-0008](0008-core-process-semantics.md) gives Choreoform an accepted working
semantic foundation. The next [Roadmap](../../ROADMAP.md) deliverable is the
canonical, versioned intermediate representation that text, visuals, and
planning will share. Neither a textual grammar, a visual vocabulary, a concrete
type/expression system, nor an implementation language has been chosen.

The IR must preserve scope, obligations, revision-bound data, human authority,
effects, uncertain outcomes, cancellation, and reconciliation without inheriting
editor layout or engine storage. The last review specifically required a common
obligation envelope, fail-closed protection metadata, monotone joins, stable
dynamic item keys, and a distinction between ordinary and terminal closure.
An opaque host-language AST or an engine snapshot would move those commitments
out of the shared contract. Conversely, freezing a full language/type system in
this deliverable would pre-empt later Roadmap decisions.

## Decision criteria

1. Preserve ADR-0008 distinctions and stable identities without hidden semantics.
2. Specify deterministic serialization and exact revision/version admission.
3. Make policy, authority, and unknown semantic content impossible to ignore.
4. Keep definition, plan, runtime records, and presentation metadata distinct.
5. Support local edits and compact templates for large dynamic fan-outs.
6. Provide inspectable schemas, examples, and negative evidence now without
   claiming unimplemented semantic validation or frontend round trips.
7. Remain independent of an implementation language or persistence backend.

## Decision

Propose **Definition IR 0.1.0** as a typed, ID-addressed graph with a strict JSON
wire format, separate non-semantic annotations, and exact artifact bindings.
The [wire specification](../ir/definition-v0.1.md) is the detailed candidate
contract; the [JSON Schema](../../schemas/ir/definition-0.1.schema.json) checks
shape but is deliberately not advertised as an execution validator.

### Canonical meaning and canonical bytes

Declarations and flows are unordered maps keyed by stable local IDs. Array
order is always meaningful. Lexical scope is explicit; causal order is carried
by named outcomes and flows, not map order. Structured nodes cover activity,
pure computation, invocation, decision, split/join, wait, repeat, fan-out, and
finish. Scope ports and invocation bindings make input/output interfaces
explicit; they do not select a source-language parameter or module syntax.

Use RFC 8785 JCS bytes for a precisely defined semantic envelope projection and
SHA-256 for its revision. Exclude only the revision field itself and the
separate annotations object. Data protection, actor requirements, type/policy
payloads, and all semantic/dialect bindings remain inside the digest. A semantic
hash is not a signature, a graph-equivalence proof, or a right to access data.

IDs survive label edits and representation changes. A semantic change creates
a new revision, while annotation-only changes preserve it. Definitions never
implicitly update plans or running instances. Format, semantics, dialect,
definition, plan, and tool versions are distinct; all 0.x formats require exact
support, not guessed forward compatibility. Unknown semantic fields or
unsupported dialects cannot be treated as optional metadata.

### Deferred language systems have explicit, non-executable boundaries

Type, expression, and policy payloads have named, immutable dialect bindings.
Their outer record shape and references are specified here. Their concrete
semantics remain decisions for the later language/type and execution work.
Unknown payloads may be preserved for inert inspection, but semantic editing,
validation, planning, and execution require the complete understood contract.

The three benchmark excerpts use a pinned illustrative dialect. This is a
declared evidence gap, not a back door for executing arbitrary JSON or prose.
The proposal specifies the structural IR boundary; it does not claim the
complete executable language is already specified.

### Explicit refinements of the accepted model

- Each data declaration retains sensitivity, purpose, participant/capability
  access sets, and a policy reference. Unsupported enforcement fails closed.
- Monotone join predicates have a closed tree grammar, not arbitrary expressions.
- A dynamic fan-out binds each immutable item to an explicit child data cell.
  Its key expression receives only that item parameter; duplicate keys are
  invalid. Existing keys retain occurrence identity as scope changes.
- Membership must be explicitly sealed before that fan-out's join evaluates.
  After sealing, added work requires another occurrence. This is a proposed
  refinement of ADR-0008's monotonicity rule, not a claim that open-ended
  membership and an already-settled join commute.
- General control cycles normalize to explicit repeat scopes; this first
  profile restricts a repeat body to one ordinary outcome. Richer composition
  must not be smuggled in through arbitrary cross-scope edges.
- Definition records refer to policies for ordinary closure and owned late
  reconciliation; runtime occurrence records remain separate. The specification
  lists the exact categories of identities that later runtime formats must link.

## Options considered

These are architectural alternatives, not equally implemented candidates or
numerical benchmark results. The costs are design judgments for this project.

| Option | Advantages | Costs and risks | Outcome |
| --- | --- | --- | --- |
| **Typed graph in strict JSON** | Explicit references and scope; small edits preserve unrelated IDs; inspectable with ordinary tools; one projection for semantic hashing | Verbose; linking and scope checks exceed JSON Schema; canonical bytes do not prove semantic equivalence | Propose |
| Nested syntax-shaped tree with references | Natural fit for a structured textual frontend; ownership is visually local; many traversals are simple | Still needs references for joins, invalidation, and shared policies; structural movement risks identity churn; can favor one frontend's decomposition | Do not select as the interchange contract; allow frontend-internal trees |
| Binary IDL-backed graph | Can offer compact transport and generated bindings while retaining the same abstract graph | Adds toolchain and canonical-byte decisions before requirements justify them; harder direct review; unknown-field behavior still needs a safe semantic policy | Defer as a possible transport, never a second meaning |
| Engine-owned serialized object model | Fastest path for one engine to save its own state | Couples definition meaning to runtime implementation and object lifetime; weak independent frontend/backend contract | Reject as canonical definition IR |

JSON is chosen for the initial inspectable transport, not because textual
notation must look like JSON. A future binary transport could be acceptable
only if it preserves this abstract meaning and has its own unambiguous revision
rules. A nested tree can still be a useful frontend projection. These options
are not incompatible implementation techniques; only one is the interchange
authority proposed here.

## Consequences

- Frontends and planners gain a concrete structural contract and exact version
  rejection rules before choosing implementation technology.
- Some validation requires graph/reference analysis and semantic dialects;
  JSON Schema alone cannot certify safety or executable conformance.
- Opaque-but-required dialect payloads preserve undecided language systems at
  the cost of incomplete semantic interoperability until those systems exist.
- Strict 0.x admission makes evolution noisy but prevents accidental compatibility.
- Stable graph IDs improve traceability but require future editors to preserve
  author identity intent; this proposal does not solve textual ID ergonomics.
- Keyed dynamic templates stay compact, but the seal rule needs adaptive-case
  review before becoming binding.
- The wire schema is MPL-2.0; prose is CC-BY-4.0; the Python fixture checker is
  a disposable evidence tool, not a project implementation-language decision.

## Confirmation

- The structural schema and three labeled excerpts validate, with current
  immutable semantic/dialect snapshot digests and reproducible canonical revisions.
  [Published local snapshots](../ir/contracts/README.md) decouple bindings from
  editorial changes to the source documents; identity and digest checks reject
  substitution or artifact corruption.
- Negative checks reject malformed transport, unknown format/core fields,
  wrong-kind/dangling references, cross-scope flows, implicit forks, invalid
  item binding, nonmonotone joins, and revision mismatches.
- Map-order, formatting, and annotation changes preserve semantic revision;
  changing policy, protection, ID, or ordered payload data changes it.
- The [evaluation](../evaluation/0009-canonical-ir.md) maps all forty scenarios
  with explicit gaps instead of inferring executable support from field names.
- The complete semantic-contract validator, full benchmark processes, actual
  text/visual round trips, and independent review close the conditional gates
  before this is treated as a final executable IR or the Phase 1 exit is passed.

## Review questions and deferred choices

1. Is strict JSON plus a semantic JCS projection an appropriate first transport,
   including its integer/string restriction and annotation boundary?
2. Should exact-version admission remain mandatory throughout 0.x, or is an
   explicit capability-negotiation scheme worth specifying before any release?
3. Is the explicit non-executable dialect boundary the right sequencing trade-off,
   or should this deliverable remain incomplete until type/expression policies
   can be specified alongside it?
4. Does the proposed seal-before-join rule fit adaptive cases, or should dynamic
   membership and completion be modeled by a different explicit protocol?

This ADR does not select grammar, symbols/layout, full types/expressions/imports,
semantic policy implementations, production diagnostic codes, an execution
language, checkpoint schema, migration, package registry, or signing protocol.

## Sources

- [RFC 8259 — JSON](https://www.rfc-editor.org/rfc/rfc8259).
- [RFC 8785 — JSON Canonicalization Scheme](https://www.rfc-editor.org/rfc/rfc8785).
- [JSON Schema Draft 2020-12](https://json-schema.org/draft/2020-12/json-schema-core).

## Acceptance and action items

This proposal becomes effective only after Project Owner approval and merge.
It remains a working structural proposal while the documented gates are
Conditional; approval must not be described as final executable conformance.

1. [ ] Obtain Project Owner review and resolve the four review questions.
2. [ ] Change status to Accepted and record approval.
3. [ ] Complete or explicitly defer the structural-versus-executable scope decision.
4. [ ] Mark the Roadmap IR deliverable complete only for the approved scope.
5. [ ] Replace illustrative dialects with accepted type/expression/policy contracts.
6. [ ] Complete semantic validation and full text/visual benchmark evidence.
