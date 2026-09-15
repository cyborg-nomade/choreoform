<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Binding and type foundations

**Status:** Proposed working foundation under [ADR-0012](../decisions/0012-binding-type-foundations.md).
Not a supported source grammar, wire format, executable dialect, or validator.

This document defines semantic constraints for the next contracts. Words such
as “reject” describe required future behavior, not claims about today's parser.
`Record`, `List`, `Option` and qualified names below are mathematical shorthand,
not proposed user-facing keywords. Ordinary authors must eventually work through
the near-plain-English and visual forms without managing wire IDs by hand.

## 1. Names and binding

**N1 — Identity is not spelling.** A declaration has an immutable local ID within
one definition and a kind. Across reusable artifacts its identity is the exact
artifact identity/revision plus declaration ID. A source binding name is how an
author refers to it; a display label is non-binding presentation. Neither a
label nor a runtime occurrence ID is a reference. Existing definition-wide IDs
remain unique across kinds and comply with IR 0.1.0's ID rules.

A rename of a binding preserves the declaration ID and rewrites affected source
references. Its resolved IR body can remain identical. Relabeling changes only
presentation. Copying allocates fresh IDs and rewrites internal references;
deleting/recreating unrelated work must not reuse an ID. Moving between lexical
scopes changes semantic ownership and requires revalidation even if ID survives.
Tools may not silently rebind unresolved names after edits.

**N2 — Lexical, static lookup.** Model a binding name as one exact nonempty
Unicode string. A qualifier is a separate sequence of explicit scope/import
bindings, not punctuation guessed from that string. No case folding, Unicode
normalization, fuzzy matching, singular/plural inference or natural-language
interpretation participates in lookup. The future source grammar must specify
how names/qualifiers are delimited and diagnose confusable spellings; it may
restrict admissible source spellings but must not silently merge distinct names.

For an unqualified reference in scope S:

1. Build symbol tables from all declarations before resolving references; source
   order does not control visibility. Initializers still need dependency checks.
2. Walk S then its lexical ancestors; do not walk runtime callers or siblings.
3. Require exactly one binding of that name in the visible chain. In this initial
   profile, reject declaration-time duplicates and ancestor shadowing across all
   kinds, including import aliases. Separate sibling scopes may reuse a name.
4. Check the resolved declaration's expected kind and access rules. A wrong-kind
   match is an error, not an invitation to search for a different declaration.

An explicit qualifier selects a known lexical scope or import alias and one of
its exposed bindings. Qualification does not grant access: sibling/child data
remains inaccessible except through the defined interface. The explicit
child-template and descendant-invalidation exceptions in IR section 3 remain.
Missing, ambiguous, inaccessible and wrong-kind bindings are distinct failures.

**N3 — Two binding stages.** Source resolution yields identity references;
semantic admission checks those references, not source spellings. A stored
definition has no unresolved lookup at execution time. Symbol/source maps are
bound to both exact source bytes and semantic revision. Maps used only to
recover author spelling may be presentation metadata, but must never substitute
for missing semantic references. Module provenance affecting meaning is semantic.

## 2. Types and compatibility

**T1 — Closed, finite descriptors.** Every value-bearing declaration and port
has a fully resolved type. The proposed initial type-shape vocabulary is:

| Form | Identity and meaning required of its later executable contract |
| --- | --- |
| Primitive reference | Exact immutable contract plus primitive declaration ID, e.g. Boolean or Text; no implicit host-language type |
| `Record{field: T, ...}` | Closed finite map; exact field names and member types; order irrelevant; no undeclared fields |
| `Variant{tag: T, ...}` | Nonempty closed map of alternatives; one explicit tag and its typed payload; order irrelevant |
| `List<T>` | Finite ordered sequence; order retained |
| `Option<T>` | Explicit absent/present alternatives; absence is a value, not an uninitialized cell |
| `Named(K, D)` | Nominal type declaration D in exact immutable type contract K; underlying finite shape and construction rules belong to K |

No `Any`, implicit nullable type, open record, structural width subtyping,
recursive descriptor, user-defined generic declaration, function value or
process value is admitted initially. Empty records are allowed; variants with
no alternatives are not. Built-in `List<T>`/`Option<T>` construction is not
user-defined generic programming. Maps/sets, exact numbers, temporal values,
artifact references and other domain types require explicit contracts rather
than accidental JSON encodings. Type aliases are transparent names for another
descriptor; cycles in aliases or named underlying-shape definitions are invalid.

**T2 — Exact equality.** Resolve contract identities/digests and aliases first.
Primitive and named descriptors are equal only when their contract identity,
revision and declaration ID match. Different nominal declarations remain unequal
even with identical representations. Two anonymous records/variants are equal
iff their field/tag sets and corresponding types are equal. Lists/options are
equal iff their element types are equal. Different constructors are unequal;
`Option<T>` is not an untagged union. Equality is recursive over finite shapes,
independent of declaration order, display labels and source aliases.

An assignment or interface connection requires equal resolved types. There is
no automatic number widening, string parsing, record-field dropping, option
wrapping/unwrapping, nominal erasure or cross-revision compatibility. Conversion
must be an explicit pure typed expression admitted by a later contract, with
its dependencies and failure behavior visible. Type compatibility is necessary
but never sufficient for information-flow or authority permission.

**T3 — Type versus value validity.** The next executable type contract must
define exact value encodings, ranges, units, precision, equality and construction
invariants. Money cannot mean “whatever the host calls a number.” This proposal
does not invent executable money, time or patient/security-domain semantics.
Checking a type descriptor does not prove a runtime value satisfies it.
Unknown type contracts block semantic admission; inert viewing is separate.

**T4 — Availability is separate.** A cell can be uninitialized or hold an
immutable value revision of its declared type. A known `Option<T>` absence is
initialized data. A read of an uninitialized cell is not `None`, null, zero,
false or an automatic wait: reject provably unavailable reads statically;
otherwise the later evaluator must raise a data-unavailable fault before the
transition publishes writes or requests effects. Imported arguments and outputs
must be available at their transfer points. Fault policy remains explicit.

## 3. Scopes and data ownership

**S1 — Lexical ownership.** Every local declaration belongs to its declared
lexical scope; the connected parent tree is fixed in a definition. Local children
may refer to self/ancestor declarations under the existing IR rules. No sibling
cell access, dynamic caller lookup or automatic parent-to-child alias exists.
Visibility alone never permits reading/writing protected data or acting as an
actor. Reads, writes, expression dependencies and policy enforcement still apply.

**S2 — Occurrences are not declarations.** Invoking, repeating or fanning out
creates fresh scope occurrences and local cell revisions, not new declaration
identities. A reference to ancestor data selects the owning ancestor occurrence
in that runtime scope tree; it cannot select a sibling or prior iteration by
name. Repeated calls cannot share locals accidentally. A value transfer retains
source revision provenance; it is not a mutable alias. Later revision changes do
not retroactively change a transferred value or invalidate history by erasure.

**S3 — Local versus reusable boundary.** A local child may use explicit lexical
dependencies as IR 0.1.0 already permits. A reusable process exported by a module
is closed: its free value dependencies must be input ports, its internal
dependencies must be declared locally or pinned imports, and its effects must
use declared capability/actor requirements. It cannot capture caller cells,
credentials, live actors, adapters or authority. This is a check on reuse, not a
retroactive prohibition on ordinary local lexical children.

## 4. Parameters and invocation

**P1 — Required named value interface.** A process template declares finite input
and output port maps and an ordinary outcome set. Each port has one exact type
derived from its owned data cell. Port tokens are interface keys, not global
declaration IDs; renaming a public port changes the interface. No positional
arguments, defaults, varargs, `inout` aliasing or omitted option arguments.
Passing explicit absence still requires `Option<T>`. Expression parameters are
pure value binders, not cells, actors, authority grants or executable callbacks.
They cannot shadow lexical bindings; all uses must be bound exactly once.

**P2 — Input judgment.** Given template input map I and actual expression map A,
require `keys(I) = keys(A)`. For each key p, the actual expression must be pure,
have no unbound parameters, and return exactly `I[p]`. The caller's read set
includes all actual-expression and policy dependencies. Evaluate all arguments
against one pre-invocation snapshot; validate all values and access before
creating the child. Initialize each input cell once with transferred value and
provenance, never by aliasing caller storage. Subsequent permitted child writes
create new revisions. Input cells have no competing initial expression.

**P3 — Output judgment.** Given template output map O and caller target map B,
require `keys(O) = keys(B)`. Targets must be visible writable caller cells of
exactly matching types and appear in the caller's declared write set. Different
output ports must target different cells: map iteration order cannot resolve
conflicting writes. A cell can be both an input-expression dependency and an
output target; input captures the earlier snapshot, output publishes a new
revision, subject to ordinary conflict/policy checks.

On any ordinary child outcome, every declared output must be initialized and
valid. Validate all transfers and publish the output set atomically as part of
completing the invocation, before enabling its successor. Protection and
information-flow checks apply to the transfer itself, not only the child. A
fault or cancellation publishes no ordinary output bundle. Earlier child
effects, lexical writes or reconciliation obligations are not rolled back or
hidden by this rule. Separate policies govern their settlement.

**P4 — No hidden specialization.** The initial profile has value parameters only,
not caller-selected type parameters, process parameters or policy overrides.
Generic type construction is limited to T1's built-in constructors. Actor and
capability requirements are not passed as ordinary values: the later planner
must satisfy their exact contracts and authority policies explicitly. A value
containing an account name or URI grants no authority.

## 5. Imports and reusable composition

**M1 — Imports are immutable resources, not commands.** A proposed module is a
finite set of declarations, explicit exports and exact dependency references.
Imports name a local alias, expected artifact kind and immutable `{id, revision}`.
`revision` is the expected digest itself: `sha256:` followed by 64 lowercase
hex digits, not a version label or a lookup key for another expected digest.
No separate expected content digest is implied by this import reference.

Before using declarations, the resolver must identify a supported artifact
kind and exact format version, check the referenced identity, and recompute
`sha256:` plus SHA-256 of that format's specified hash input. Compare the result
with the **import reference's** `revision`, not merely a registry key or an
artifact's self-reported checksum. The calculation rule comes from the resolver's
explicitly supported format contract, not a rule supplied by untrusted content.

The existing hash inputs are deliberately different:

- Definition IR 0.1.0: UTF-8 JCS bytes of exactly `format`, `version`, `kind`,
  and `body`, without a trailing newline, as specified in
  [IR section 2](../ir/definition-v0.1.md#2-transport-and-envelope). Exclude
  `revision` and `annotations`; retain the existing envelope/revision checks.
- Existing immutable contract snapshots: every stored byte, including notices
  and the final newline, without reformatting or line-ending conversion, as
  specified by [the snapshot contract](../ir/contracts/README.md).

New type/module artifact formats must obtain an approved exact hash-input and
canonicalization rule before they can be supported by a linker. Until then,
reject them as unsupported; do not guess whole-file hashing or reuse the IR
projection. Any future transport-file checksum is a separate explicitly
specified check and cannot replace comparison with the pinned semantic revision.

The host supplies a closed local resource registry. Resolution verifies expected
identity, kind and digest for every transitive dependency. Missing, unsupported,
wrong-kind or mismatched resources fail; a URI never triggers a network fetch.
No `latest`, version range, runtime import, wildcard import or implicit re-export.
Registry publication, signatures, trust and package installation remain Phase 4
work; a verified digest alone is not authorization.

**M2 — Closed export namespace.** Export only declared named types, process
templates and immutable capability/policy contract declarations. Never export
live data cells, actor identities, credentials or runtime effects. An export
map connects exact external binding names to stable declaration IDs/kinds.
References through an import alias can resolve only exported declarations.
Import/export bindings that affect resolution belong to a semantic module
artifact, not annotations. Relabeling a display is non-semantic; changing an
export name or dependency pin changes that artifact's revision.

Two aliases for the same exact artifact retain the same imported declaration
identity. Two revisions of the same artifact may coexist only under distinct
aliases; their nominal types remain different. Transitive dependencies are
resolved in the exporting module's environment, not the caller's. Diamond
imports of an identical revision do not duplicate type identity or mutable
state (modules contain no live state). Imports must form a finite acyclic graph.
Resource budgets may refuse a graph explicitly; truncation is not success.

**M3 — Composition is explicit invocation, not substitution.** Reusable process
calls use P1–P4, preserve named outcomes, explicit cancellation/fault/closure
boundaries and internal obligation ownership. Calls must form an acyclic graph;
self or mutual recursion is rejected. Use repeat/fan-out for the accepted
iteration/dynamic-cardinality model. Code/data supplied at runtime cannot add
new process templates or capabilities. An import alone creates no work.

**M4 — Linking obligations.** Before planning, a later linker must resolve the
complete pinned closure, check exported interfaces and closedness, and produce
a separately specified canonical linked artifact. A process instance binds that
exact result and plan. If lowering materializes local scopes, persist a mapping
from `(composition-site identity, imported artifact identity/revision,
declaration ID)` to fresh definition-local IDs; allocate once, preserve it on
unrelated edits, and reject collisions. Import aliases and traversal order must
not determine allocated IDs. Remapping internal references must include all
supported dialect dependencies; opaque unknown payloads cannot be guessed at.

This mapping's versioned encoding and the linked artifact's digest/provenance
rules are required **before linking is implemented**. This proposal deliberately
does not claim existing JSON can round-trip modules. Imported internal scopes
must still be checked against their closed boundary after materialization;
becoming a local child must not grant new ambient access. A moving dependency
never updates a running instance; upgrades create an explicit new definition
revision and require revalidation. Running-instance migration stays prohibited.

## 6. Compatibility and follow-through

| Topic | Existing IR 0.1.0 | Required follow-through |
| --- | --- | --- |
| Identity/lexical scopes | Exact IDs, expected-map references, parent tree, occurrence distinction already specified | Source binding tables and rename operations in later authoring design |
| Types | Dialect slots are illustrative and opaque | Accepted executable descriptor/value contracts implementing T1–T4; no silent interpretation of old payloads |
| Local invoke | Direct child, exact port maps, outcomes and caller writes already represented | Type, availability, transfer protection and output-target uniqueness validation |
| Expressions | Only fan-out `item` parameter currently has a binding site; other uses have empty maps | Preserve restriction; general expression-call parameters need reviewed binding sites/encoding |
| Split/repeat/fan-out | Split/repeat no ports; fan-out only item input, no outputs | Preserve restrictions; any generalized ports need a versioned core extension |
| Reusable imports | No import/module/linked-provenance core fields | Review module and linked formats plus required contract bindings before implementing M1–M4 |
| Text/visual | Current text is explicit IDs and JSON; no visual design | Controlled-English names, explicit disambiguation and equivalent visual operations with user evidence |
| Runtime/validation | Partial probe only | Full structural/link/type/policy validator, stable diagnostics and later engine |

No current strict reader may accept unknown fields as an implementation shortcut.
If a proposed rule changes accepted semantics or a core-record shape rather than
filling an explicitly open dialect contract, obtain an explicit version/contract
decision and preserve old artifacts; do not rewrite accepted snapshots.

Follow-through owners are the Phase 1 author with Project Owner approval:
executable contracts supply T3's concrete semantics and authority/flow judgments;
validation implements these checks and resource limits; authoring/visual design
defines names and accessibility; versioned linkage must precede reuse tooling.
Stable diagnostic codes and inner-source spans are later API work. For now,
each rejection must identify the conflicting declarations, expected/actual
types or dependency path without exposing protected values.

See [worked judgments](cases.md) and [evaluation limitations](../evaluation/0012-binding-type-foundations.md).
