<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Definition IR 0.1.0 — proposed wire specification

**Status:** Proposed; governed by [ADR-0009](../decisions/0009-canonical-versioned-ir.md).<br>
**Semantic foundation:** [ADR-0008](../decisions/0008-core-process-semantics.md).

The requirements below describe the candidate format, not an already released
language. The [schema](../../schemas/ir/definition-0.1.schema.json) checks record
shape. This prose additionally defines reference, scope, ordering, identity,
and admission rules. If they disagree, the proposal is defective; neither a
successful schema check nor an implementation may silently resolve the conflict.

## 1. Scope and pipeline

Text and visual frontends produce the same **definition IR**. A planner consumes
it and produces a separately identified plan. An instance binds to the exact
definition revision and exact plan, never to a moving branch or latest version.
The definition IR contains declarations, not live data values, observations,
attempt records, worker assignments, or execution history. Section 9 preserves
the identities those future runtime artifacts must reference without choosing
the Phase 2 persistence or checkpoint format.

Every core distinction must be encoded in the semantic body. No frontend may
derive behavior from source order, layout, comments, naming conventions, or
undocumented engine defaults. Loading an unfamiliar artifact for inert display
is distinct from claiming to validate, edit semantically, plan, or execute it.

## 2. Transport and envelope

Use UTF-8 JSON with exactly these envelope fields:

| Field | Required value or meaning |
| --- | --- |
| `format` | `choreoform-ir` |
| `version` | Exact format version `0.1.0` |
| `kind` | `definition`; other artifact kinds need their own schemas |
| `revision` | `sha256:` followed by 64 lowercase hex digits, calculated below |
| `body` | Complete semantic definition body |
| `annotations` | Object containing non-semantic presentation/provenance data |

No unknown envelope or core-record fields are permitted. Mandatory maps are
present even when empty. Missing, empty, and JSON null are distinct; null is
permitted only where the schema explicitly admits it. There are no defaulted
semantic fields. JSON comments, duplicate object keys, a byte-order mark,
non-finite numbers, fractions/exponents as number tokens, and unpaired Unicode
surrogates are invalid. Duplicate keys must be rejected before ordinary map
construction can discard them.

JSON number tokens are limited to integers from -9007199254740991 through
9007199254740991. Domain integers outside that range and exact decimals use
dialect-defined string values; a frontend must not round a money value into a
binary floating-point number. Strings retain their Unicode spelling: no silent
normalization. This restriction applies inside dialect payloads and annotations
as well as core records.

### Semantic revision calculation

Let `P` be the object containing exactly `format`, `version`, `kind`, and `body`.
The revision is `sha256:` plus the lowercase hexadecimal SHA-256 of the RFC 8785
JCS encoding of `P`. JCS produces UTF-8 bytes with no trailing newline. This
avoids a self-referential digest; `revision` and `annotations` are not in `P`.
Readers verify the supplied revision rather than trusting it.

Pretty-printed JSON is permitted for storage and review. Map insertion order,
indentation, and whitespace do not affect revision. Array order always matters;
unordered declarations and sets therefore use maps. JCS is byte canonicalization,
not graph-isomorphism checking, expression simplification, or proof that two
different revisions have different observable behavior. Renaming an ID changes
the revision even when an author believes the behavior is equivalent.

The revision is an integrity check, not a signature, authorization credential,
or confidentiality mechanism. Artifact distribution/signing remains separate.
An annotation-only edit preserves semantic revision but may change a separate
whole-file integrity check. A policy edit always changes semantic revision.

## 3. Identity and references

`body.id` is a stable, nonempty definition identifier; examples use URNs. The
wire format does not grant global uniqueness or establish a registry. Local
IDs match `[A-Za-z][A-Za-z0-9_-]{0,63}` and are unique across every declaration
map and flow map in one definition. Frontends allocate once and preserve IDs
through formatting, moves, label changes, and text/visual transformations.
Copying creates new IDs and rewrites internal references; deleting does not
authorize reuse for unrelated work in the same definition lineage.

Maps keyed by local ID are `scopes`, `data`, `expressions`, `actors`,
`capabilities`, `policies`, `nodes`, and `flows`. A local reference is an exact
ID, never a display label, array position, path inferred from layout, or URL
to fetch. A reference must resolve to the expected map and visible scope.
Scope visibility is self and lexical ancestors; child scopes are entered only
through explicit invocation/fork/repeat declarations. Explicit invalidation
targets may name descendant work that can read the declaring data, and fan-out
item initialization explicitly targets a data cell in its child template.

An external immutable reference is `{id, revision}`. Its revision has the same
SHA-256 spelling, but hashes the referenced artifact's specified bytes; only
definition IR uses the semantic projection from section 2. The resolver must
verify both identity and digest against an explicitly supplied local resource
registry. A URI is an identity, not permission to access the network.

`body.semantics` pins the semantic contract. In these examples it identifies
ADR-0008 and hashes that document's exact repository bytes. `body.dialects` maps
local dialect IDs to immutable contract references. These are independent of
the wire version and a tool's own version.

## 4. Definition records

`body` requires `id`, `semantics`, `dialects`, `root`, and all eight declaration
maps. `root` names the sole scope with null parent. Other scopes have one parent;
the parent relation must be connected and acyclic. Declaration maps are
unordered. Symbol sets are objects mapping a local token to the literal `true`.

| Record | Required fields and meaning |
| --- | --- |
| Scope | `parent`, `entry` node, `inputs`/`outputs` maps from port names to local data cells, declared `outcomes` set, and `cancellation`, `faults`, `closure`, `race` policy references |
| Data | `scope`, `type` dialect value, `protection` envelope, `invalidates` node set; optional `initial` expression |
| Expression | `scope`, `dialect`, `resultType` dialect value, typed `parameters` map, declared `reads` data set, and structured `body` |
| Actor requirement | `scope`, `requirement` policy reference; a predicate, not a live user account |
| Capability | `scope`, immutable-contract policy reference `contract`, dialect `input` and `output` types, `authority` actor requirement, and `effects` policy reference |
| Policy | `scope`, `dialect`, structured `body`; includes cancellation, fault matching, closure/reconciliation, race resolution, authority, clocks/calendars, retry, effect, and collection-change contracts |
| Node | Common fields plus kind-specific fields in section 5 |
| Flow | `source` node, named `outcome`, `target` node; explicit causal successor |

A dialect value is `{dialect, body}` with a structured object payload. The
dialect ID must be declared in `body.dialects`. No host-language code string is
implicitly executable. The expression/type/policy ADRs must supply immutable
contracts defining payload schemas, purity, types, dependencies, effects,
authority, and validation. Until then these slots provide lossless structure,
not executable semantics. A tool that lacks a dialect may preserve its payload
unchanged for inert viewing/export; it must not validate, rewrite, plan, or
execute its meaning. A supposedly optional extension that changes behavior must
be a required dialect, never an annotation.

### Protection envelope

Every data declaration requires `sensitivity` (nonempty string), `purposes`
(nonempty symbol set), `participants` (actor-requirement set), `capabilities`
(capability set), and `policy` (local policy reference). The policy may contain
an external immutable policy reference. Empty access sets mean no member of
that class is permitted, not unrestricted access. Readers retain these fields
even when they cannot enforce them; planning and execution fail closed unless
the required policy is understood and enforceable. Secrets and credentials are
bound outside definition artifacts, not embedded in examples or annotations.

Data revisions at runtime are immutable. `invalidates` records which decisions
or work lose sufficiency when this data changes; an empty set does not grant
permission to reuse a result against a different snapshot. Full dependency and
type validation remains a semantic-validation responsibility.

## 5. Nodes and explicit control

Every node requires `kind`, `scope`, `outcomes`, `reads`, and `writes`. Reads and
writes name data declarations, not current mutable values; evaluation uses one
consistent revision snapshot. Declared dependencies must include those of
referenced expressions and policies. Consumers must validate the declarations
against the dialect contracts, not trust a possibly incomplete read set.

| Kind | Additional required fields |
| --- | --- |
| `activity` | `mode` (`human` or `capability`), `target` actor/capability reference, `input` expression, `result` data cell or null, `policy` work-lifecycle policy |
| `compute` | Nonempty `assignments` map from writable data cells to pure expressions; one outcome |
| `invoke` | `body` child scope template; `inputs` map from child input port to expression; `outputs` map from child output port to writable caller data cell |
| `decision` | `guards` map from outcome token to expression, `default` outcome token or null |
| `split` | Nonempty `children` map from stable branch key to child scope, paired `join` node |
| `join` | `source` split/fanout, closed `predicate` tree, `remaining` settlement policy |
| `wait` | `policy` specifying accepted observation or timer, correlation, revision, cancellation, and stale/duplicate handling |
| `repeat` | `body` child scope, Boolean `condition` expression |
| `fanout` | Collection expression `collection`, stable-key expression `itemKey`, child scope `body`, child data cell `item`, paired `join`, membership-closing Boolean expression `seal`, explicit `changes` policy |
| `finish` | Scope `outcome`; its node `outcomes` set is empty |

The schema rejects additional fields and unknown kinds. It does not define a
stringly typed escape node. Extensions must revise the supported format or use
the declared dialect-bearing records without injecting new core control kinds.

Scope ports name cells owned by that scope. Input cells have no independent
initial expression. Launch supplies each root input; root output ports expose
their final revisions at ordinary completion. Invocation bindings cover every
child port exactly once, and explicit assignment to a caller output cell is in
the invocation's write set. This specifies the IR interface shape, not a
source-language parameter or module system. In this initial profile, split and
repeat body templates have no ports and use lexical data; a fan-out body has
only its `item` input and no output ports. More general composition needs a
later versioned extension rather than invented implicit bindings.

A non-null activity `result` publishes its contract-defined result to that cell
in the completion transition and is included in `writes`. Null means no data
result is exported; attributable decision/effect records still exist. A
`compute` evaluates every assignment against the same pre-transition snapshot,
validates all results, and publishes them atomically, emitting its sole outcome.
Assignment map order does not imply sequential evaluation. These are pure
internal transitions from ADR-0008, not undeclared external capabilities.

Flows stay within one scope. Each ordinary node outcome has exactly one
successor flow; no two flows may share `(source, outcome)`. A decision chooses
one outcome by pure guards, with default only for zero matches; multiple true
guards fault. Forks are explicit, not duplicated outgoing flows. `split` and
`fanout` have empty outcomes and no successor flow: they create their paired
join obligation together with identified child occurrences. `finish` has no
successor. Every scope entry is a node in that scope. A scope invocation
produces its child scope's named outcome without flattening child obligations.
A join declares exactly one outcome and emits it when its predicate holds.
A repeat and its body each declare one ordinary outcome in this initial
profile: the condition is tested before each iteration; false emits the
repeat's outcome, while a completed body causes the condition to be tested
again. Faults and cancellation follow their separate scope policies.

Each split/fanout has exactly one reciprocal join in the same parent scope.
All other nodes in a child template stay lexical children, not runtime sibling
copies drawn into the definition. Repetition creates new occurrence IDs, not
new definition node IDs. General graph cycles are invalid in this initial
profile; loops use `repeat`, so iteration boundaries are explicit.

### Joins and dynamic membership

Join predicates are recursively one of:

- `{op: all, outcomes: set}`: every member has a terminal outcome in the set;
- `{op: any, outcomes: set}`: at least one member has such an outcome;
- `{op: atLeast, count: positive-integer, outcomes: set}`;
- `{op: and|or, terms: nonempty-map-of-predicates}`.

All names above are JSON strings. The member set is the identified child set
of `source`, not unrelated children of the parent. `all` over an empty set is
true; `any` and positive thresholds are false. The `remaining` policy must
settle, cancel, transfer, or retain every unfinished child; a true predicate
never discards an obligation. No negation, equality-to-count test, general
expression, clock access, or effect is admitted in this predicate language.

Fan-out membership may change while its `seal` is false. Stable keys preserve
child identity; added keys create work; removal requires explicit settlement.
Changing an existing item's value does not relabel its old evidence: declared
invalidation dependencies determine what must be redone. Duplicate keys are
invalid before child creation. Once sealed, membership is frozen for this
fan-out occurrence and its join may evaluate. New scope afterward requires a
new occurrence, not silently reopening a completed join. This fixed-member
boundary makes monotonicity well-defined; the seal is an explicit design rule,
not an inference from temporary absence of incoming data.

The key expression declares exactly one typed parameter named `item`. It is
evaluated once per collection item with that argument bound to the item's
immutable value; its result is a stable string key. It cannot depend on other
mutable process data. Each new child's `item` data cell is initialized with the
same value/revision used for its key. That cell belongs directly to the body
scope and has no independent initial expression. Existing keyed items publish
new data revisions rather than overwriting prior evidence. Other expression
uses in this profile have empty parameter maps; general parameterization and
composition remain later work. Seal, guard, and repeat-condition results must
be Boolean under their declared dialects.

### Effects, time, cancellation, and closure

The associated policy dialects must represent all ADR-0008 distinctions, not
collapse them into a generic status string. In particular, capability contracts
must distinguish request from confirmed outcome and unknown result, stable
logical effect identity from attempt identity, idempotency from reconciliation,
and cancellation from compensation. A compensation activity uses ordinary work
with an explicit link to the prior effect in its policy; history is not erased.

Scope race policies must explicitly select acceptance order, a pure resolver,
commutativity, or protective invariants as appropriate; consequential and
irreversible races must meet ADR-0008's additional requirements. Clock/calendar
policies pin basis, revision, time zone, pause intervals, and authority. Scope
closure policies distinguish ordinary closure with owned reconciliation
subscriptions from fully terminal instances that can only be linked by later
work. Merely having these policy references does not prove the policy correct.

## 6. Presentation and round-trip obligations

`annotations` may store labels, source spans, comments, and layout as
namespaced objects keyed by stable IDs. Its internal format is deliberately
not a visual-notation decision. It must not carry sole copies of guard rules,
actor authority, effects, protection, deadlines, or other semantics. Unknown
annotations must survive a lossless editing operation; a lossy export must be
explicitly labeled and must not replace the source artifact automatically.

Moving nodes, changing labels, or reformatting source preserves `body` and its
revision. Changing a semantic dependency produces a new revision. A source map
must identify which semantic revision and source artifact it describes; stale
maps must not be applied to another revision. Layout-only changes do not grant
access to protected labels or data. Editors and exporters still need their own
metadata privacy policy; exclusion from the semantic hash is not a safety claim.

The eventual text and visual frontends must demonstrate projection to the same
semantic body, preservation of IDs and unknown annotations, and stable repeat
round trips. These are obligations, not results of the current JSON fixtures.

## 7. Versioning and admission

Format versions use three nonnegative decimal components without leading zeros.
During 0.x, readers must explicitly support each exact version; matching major
or minor numbers does not authorize execution. A future stable compatibility
policy requires a separate accepted decision. Semantics and each dialect also
require exact identity/digest support; no `latest`, wildcard, or range binding.

| Change | Writer/reader rule |
| --- | --- |
| Formatting or annotation-only change | Same semantic revision; preserve unknown annotations |
| Semantic field or ID change in same format | New definition revision; existing instances unchanged |
| Added/removed/renamed field or changed record meaning | New format version, even if a reader could guess a default |
| Documentation/schema correction preserving both accepted documents and meaning | Patch version allowed, still explicitly admitted during 0.x |
| New semantic or dialect contract | New immutable binding and definition revision |
| Unsupported version, dialect, or core field | Inert view possible; reject semantic validation/planning/execution |
| Offline upgrade of a definition | Explicit conversion creates a new artifact, retains source revision and reports losses; never overwrite an active binding |
| Running-instance migration | Prohibited by ADR-0008; a file converter cannot authorize it |

A schema document is pinned as a local validation resource; its `$id` is not a
network-fetch instruction. Read limits for bytes, nesting, record count, and
payload size must be applied before expensive validation or hashing. Resource
exhaustion is a refusal, never permission to discard policy fields. A format
version label cannot substitute for verifying the actual document and digest.

## 8. Validation phases and failures

1. **Transport:** size/depth limits, UTF-8, duplicate keys, integer and Unicode
   restrictions. No lossy JSON parsing.
2. **Shape:** exact version/schema; closed core records; mandatory envelopes.
3. **Linking:** IDs unique; references exist with expected kinds; scope tree,
   entries, visibility, paired joins, outcome flows, and loop boundaries valid.
4. **Semantic contracts:** exact dialect availability, payload validation,
   types, purity, complete dependencies, authority, safe retry, reconciliation,
   monotone joins, and invariant validation. Unknown is not valid.
5. **Identity:** recompute semantic revision and verify supplied bindings.
6. **Planning:** bind required capabilities and enforcement facilities without
   changing meaning. Create a separately identified plan only after success.

Error categories are transport, shape, reference, scope, semantic-contract,
revision, unsupported-version, and unsupported-dialect. Stable diagnostic codes
and user-facing spans remain the later validation Roadmap item. The included
checker exercises transport, shape, selected reference rules, and digests; it
is not the product validator and cannot complete phases 4 or 6.

## 9. Runtime linkage, not a checkpoint schema

The following is a required correspondence for later runtime formats. It is
not permission to serialize an engine heap as definition IR.

| ADR-0008 component | Linkage required in later plan/instance/history artifacts |
| --- | --- |
| Definition binding | Definition ID plus semantic revision, exact format/semantics/dialects, immutable plan ID/revision |
| Scope tree | Unique scope occurrence ID, template scope ID, parent occurrence, ordinary/terminal lifecycle |
| Obligations | One common envelope: occurrence ID, owner, scope occurrence, kind, lifecycle, settlement; typed payload links to node/policy IDs |
| Data store | Declaration ID, immutable revision ID, exact typed value/artifact reference and protection envelope |
| Work/effect records | Node ID, execution ID, distinct attempt IDs, stable effect ID, actor assignments, policy/calendar revisions |
| Accepted observations | Observation ID, correlation, provider provenance/time, engine acceptance position, consumed/rejected disposition |
| History | Transition ID, causal predecessor IDs, read/write revisions, authority, choices, effects, time, and originating definition/plan binding |

Counters or timestamps alone cannot stand in for occurrence identities. A new
definition revision never remaps existing occurrence IDs. A million-item fan-out
adds runtime records, not a million authoring nodes. Phase 2 must supply the
concrete runtime schema and recovery tests before claiming portable checkpoints.

## 10. Sources

- [RFC 8259 — JSON](https://www.rfc-editor.org/rfc/rfc8259): transport syntax.
- [RFC 8785 — JCS](https://www.rfc-editor.org/rfc/rfc8785): canonical JSON bytes.
- [JSON Schema Draft 2020-12](https://json-schema.org/draft/2020-12/json-schema-core): structural schema dialect.

The graph, projection, limits, and admission rules are Choreoform proposals,
not requirements attributed to those standards.
