<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Reversible authoring document and changes 0.1.0

**Status:** Normative contract accepted under ADR-0094 / owner delegation.
Native document admission, managed changes and visual editing are not implemented.
Schemas constrain structure; every invariant below is also required. This is a
bounded foundation, not full validation, editing completion or execution permission.

## One meaning, preserved authoring state

A complete document has the closed [document schema](../../schemas/authoring/document-0.1.schema.json).
`source` and `bindings` are strings preserving the exact original UTF-8 source and
companion bytes after JSON decoding and UTF-8 encoding, including final newlines.
They are not filesystem paths, imports or URIs to fetch. Keeping both strings
preserves companion whitespace as well as all source-only definitions, names,
annotations and explicit live/retired identities. No automatic IR-to-source
reconstruction is implied. The sole exact component matrix is:

| Component | Selected contract |
| --- | --- |
| Document, change and layout | 0.1.0, their respective closed schemas |
| Source | Candidate A 0.19.0 / [complete grammar](candidate-a-v0.19.ebnf) |
| Companion | bindings0.18.0, profile0.19.0 / [schema](../../schemas/authoring/bindings-0.18.schema.json) |
| Semantic projection | C12 / IR0.4.0, exact inherited semantic/core/resource pins |

No version ranges, future dialects or fallback to older components. Existing
source/SDK APIs and their quotas remain unchanged. Native C13 availability does
not add source support. A renderer cannot treat a self-asserted revision as proof.

The source and strict companion together are authoritative authoring state. The
IR is a derived execution projection; layout has no process meaning. Required
human instructions remain in the semantic body. Opaque literal values, field/type
payloads and annotations are never reinterpreted as authoring references. No IDs
are recovered from names, coordinates, positions or occurrence order.

## Admission and independent digests

Read the outer document as strict UTF-8 JSON without BOM, duplicate keys, trailing
data, floats, non-finite numbers, negative numeric zero, unsafe integers or unpaired
surrogates. These restrictions apply at every level and again to decoded bindings.
No normalization of original source or companion strings. Check schemas and all
bounds before allocating derived state. Resolve schema URNs only to the exact
local publications; no network fetch or dynamic dialect installation.

The `integrity` object's `source`, `bindings`, `package` are exactly the existing
source19 native package integrity values: source SHA-256 over raw bytes, bindings
SHA-256 over UTF-8 RFC8785 JCS (no LF), and the source19 joint digest with domain
ASCII `choreoform-authoring-package:0.19.0`, zero byte, unsigned64 big-endian
source length, source bytes, unsigned64 big-endian canonical bindings length,
canonical bindings. No authoring-document/layout state enters those domains.

Resolve the full strict companion/source inventory and lexical ownership. Supply
exact resource bytes separately by `(contract, digest)` in an explicit local
registry; every declared resource must be present, raw SHA-256 must match, and
native inherited resource validation must succeed. Never auto-fetch. Do not trust
an unrelated supplied resource or a pin alone as provider/host truth. Existing
built-in accepted semantic/core contracts retain their native exact pins.

Run real native source19 lowering, including its inherited prerequisites, and
recompute the IR semantic revision from the native semantic projection. The
`projection` must match IR format/version, body definition ID and revision exactly.
This binds the whole semantic body, not annotations. The document is admitted only
as **projection-matched**; no full-definition/static/control/protection/concurrency
or executable judgment follows. Report later views at their own stages. A document
whose later concurrency check refuses may still preserve authoring state; it cannot
claim that check passed. Invalid source/ledger/resource/projection refuses atomically.
No silent repair, preferred-component fallback or successful partial export.

The separate **authoring digest** is `sha256:` plus lowercase SHA-256 over ASCII
`choreoform-authoring-document:0.1.0`, zero byte, then RFC8785 JCS bytes of the
entire admitted document (no LF). It is derived outside the document, preventing
self-reference. It includes original source/bindings strings, all integrity and
projection fields and layout. Outer key order/JSON whitespace/escape spelling do
not affect it; decoded source or companion whitespace does. Thus companion raw
whitespace can alter authoring integrity while leaving its canonical/joint native
integrity and semantic revision unchanged. Layout changes alter only authoring
integrity. Semantically equivalent authored definitions can have different source,
companion, package and authoring digests. None of these is a signature or authority.

## Non-semantic layout and faithful representation

The [layout schema](../../schemas/authoring/layout-0.1.schema.json) has exactly
format/version/entries. Each entry is `{entity,x,y,collapsed}`. Coordinates are
signed integral abstract layout units, bounded inclusively to ±1000000; they are
not pixels, ordering, host addresses or semantic inputs. Entries must be unique
and strictly ascending by ASCII `entity`. Targets are existing declaration,
symbol or generated IDs; definition lineage and retired/unknown IDs refuse.
An entry for a symbol is an attachment to its resolved owner, not another process
occurrence. Layout creates no storage, work, permission, alias or control link.

Layout is sparse. Missing placement means deterministic frontend default placement
ordered by explicit IDs within real lexical ownership; missing placement never
means a missing entity. Default placements must not be serialized as semantic
identity. Collapsed state affects only presentation; the model remains complete.
Render every selected construct, including all source-only predicates/inputs,
type/protection aliases, policies, resources, symbols and generated slots. Derive
edges/ownership/lexical visibility from actual bound source/IR; do not infer meaning
from proximity or allow user-drawn links to override semantics. Lists/inspectors
may faithfully represent detailed constructs; a generic unsupported placeholder
cannot count as visual parity. Full visual notation is subsequent work.

Both forms must expose names, kinds, owners, types, human instructions, references,
source locations and relevant validation stage with equivalent meaning. Preserve
opaque annotations, fixed wires, explicit nominal/type/resource pins and distinctions
between occurrences, scopes, storage cells, work identities and authority boundaries.
Do not merge identical-looking work or discard unused source-only definitions.
Provide keyboard and assistive-technology inspection/navigation/edit/export, explicit
collapse/expand state and readable descriptions without relying solely on geometry
or color. These are prototype obligations, not current accessibility/user evidence.

## Atomic changes and conflicts

A [change request](../../schemas/authoring/change-0.1.schema.json) contains its own
format/version, `base` authoring digest and exactly one operation. The complete
base document and exact resource registry are separate explicit inputs, not paths
or implicit global state. Authorize first, admit the complete base, validate the
request, then compare `base` to its authoring digest before mutation. A mismatch
refuses as `stale-base`, even if semantic revision is unchanged. No implicit merge,
last-write-wins, replay against a different lineage or best-effort patch.

| Operation | Exact admitted behavior |
| --- | --- |
| `format` | Existing native source19 formatter; replace source and companion strings with its deterministic output; preserve every managed ID, wire, tombstone, pin, annotation and layout; re-admit and require the same complete native IR |
| `rename` | Target explicit ID of a declaration with kind `predicate`, `type-alias` or `data-protection`, or symbol role `predicate-input`; replace its display name and all and only resolved declaration/use-name tokens and that one companion row's `name` |
| `replace-layout` | Replace the complete layout with the supplied closed layout after reference/order/bounds checks; preserve source and companion strings byte-for-byte and preserve native IR |

Rename validates the inherited name grammar, complete kind/owner/lexical inventory,
no shadowing and no owner-role name conflict at every use, including unused,
forward/nested/qualified uses. A predicate-input rename updates its declaration,
parameter references and argument keys at every resolved use; other predicates'
parameters, ordinary expression parameters and cell names stay unchanged. Qualified
references update only the resolved target's token, keeping qualification valid.
No change of fixed symbol wires, IDs, generated slots, resource pins, required human
instructions, annotations or opaque literals/type field labels. A collision refuses
`name-conflict`; retired IDs refuse `retired-target`; other live kinds or missing
IDs refuse `unsupported-target`. Same name is a deterministic no-op after admission.
Source splices preserve all bytes outside resolved name tokens; emit replacement
names with native canonical quoted-string spelling. The companion is emitted as
JCS without final LF for rename, retaining array order and every other value.
Do not use global text substitution. Reparse/rebind/re-lower the complete result,
require the exact same native IR including annotations, preserve ledger inventories
and verify layout references again. Recompute all affected integrity values.
A wrong proof or stale source map refuses, never changes semantic meaning silently.

Every success is a complete re-admitted document plus its newly derived authoring
digest and explicit projection-matched/non-executable stage. No execution-capable
result/report type or serialized response envelope is introduced here. Deterministic
operation output includes deterministic strings and layout order; transport uses
JCS without LF. The original input is immutable; commit a result only after all
checks. Failure emits no partial document/candidate/source/ledger/resource. Diagnostic
codes and structural locations/managed IDs/UTF-8 half-open source spans may be
reported without echoing confidential values. Never echo names, source, opaque
payloads or untrusted schema error strings in a refusal. General diagnostic naming
and full field/argument actionability remain a separate Phase1 obligation.

Copy, retire/delete, arbitrary source/structural edits, semantic rewrites, resource
updates, ordinary semantic-entity renames, partial/redacted editing and IR-only
reverse export are **unsupported** by this exact change schema. Future copy needs
explicit fresh IDs for every copied declaration/symbol/generated slot, no live or
retired collisions, complete internal-reference rewrite and explicit external
captures/resources; identical text never merges occurrences. Future retirement
must reject live references and retain tombstones permanently. No implementation
may invent these mappings or silently extend0.1.0. General source import can create
a new complete document from separately authorized source+companion/resources by
native lowering and deriving fresh integrity/projection, never by trusting old pins.
It is distinct from an admitted change. Original-package export preserves exact
stored strings; formatted export uses `format`. IR-only export has no preservation
claim and needs a separate profile. Re-import must check the whole document anew.
Draft invalid text may be held separately by a future editor, labeled invalid; it
is never a projection-matched document or successful visual/export result.

## Protection, bounds and evidence

A full document is a complete artifact, not an audience view. The trusted integration
must separately authorize artifact read/inspect, edit and export before disclosure
or any operation; do not derive authorization from this untrusted JSON, a digest,
an actor name, a process-data protection label or annotations/layout. This contract
adds no authorizing field or policy language. Keep metadata privacy/authenticity
separate from runtime-value protection. An audience view identifies itself outside
this schema as a view; it cannot be admitted as the complete model or used to infer
hidden content. Unsupported redacted editing/export refuses. A future integration
must define its actual access model and test enforcement before protected UI claims.
A report, revision or package grants no live actor/provider/host/runtime permission.

All limits are inclusive, refuse before oversize allocation/expansion, and apply to
input and generated output. No truncated success. This layer adds transport bounds
without raising existing native limits:

| Bound | Maximum |
| --- | --- |
| Outer document raw UTF-8 and JCS separately | 16 MiB each |
| Decoded source and decoded companion UTF-8 separately | 1 MiB each, plus every inherited component limit |
| Outer JSON nesting / JSON values and object keys visited | 64 containers / 262144 items cumulatively per artifact; string bytes charged to byte bounds |
| Layout JCS / entries | 256 KiB / 10000, also within outer limits |
| Change request raw UTF-8 and JCS separately | 1 MiB each; same depth/item bounds |
| Explicit supplied resource registry | 128 distinct pins / 8 MiB total raw bytes; individual native resource limits still apply |
| New document transport | 16 MiB JCS, bounded before committing; request/base/output and resources remain separately bounded |

JSON strings count as scalars for outer depth; decoded components have their own
native depth64 checks. Cap lexical byte scans by the raw/decoded byte bounds and
structural walks by cumulative item bounds; no repeated per-entity rescans or reset
expansion budgets. Native source19 still has131072 tokens/parser1M/lowering100k /
cumulative expansion1MiB/shared semantic work1M and inherited IR/output limits.
Resource rows can exceed the smaller registry limit structurally but admission then
refuses; schema alone never grants admission. Whole-transport16MiB accommodates
escaping two separately bounded1MiB components (up to6x each) and bounded layout;
it does not make either component larger. Layout/item limits may further constrain
otherwise schema-valid artifacts. RP-01/03's actual source/companion sizes fit; their
full transport/registry measurements must be observed, not extrapolated savings.

[Worked vectors](reversible-0.1/vectors.json) pin schemas and positive/negative
artifacts byte-for-byte. They include preserved source-only structures with equal
semantic revisions, layout stability, explicit name/input/type/protection renames,
opaque text, tombstones, stale bases and private refusals. Paired after-states are
finite authored fixtures, **not** a managed-operation implementation. A read-only
schema/native projection oracle may verify them using existing native APIs; it
cannot prove native change behavior, visual roundtrips, protection enforcement,
users, browser/host execution or independent review. Later signed scopes implement
admission/operations and paired prototype. Complete RP-08, full diagnostics/validator/
conformance, three-process cross-form cycles and actual representative users remain
Phase1 requirements; G1–G4 stay conditional and Phase2 cannot begin.
