<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Experimental paired authoring prototype 0.1.0

**Status:** Exact experimental prototype contract accepted under ADR-0096 / owner delegation.
Native view/bridge/UI implementation remains pending; this is not a notation standard. The Manifest
requires usability testing before notation standardization. This version defines
one falsifiable experiment, not an accepted visual language or user-study result.

## Model and native interfaces

Select unchanged document/change/layout0.1.0, native binding0.1.0 and exact
source19/companion18/C12/IR4. One admitted AuthoringDocument is the model; both forms
are projections of it. No JavaScript/Python parser, semantic evaluator, name/ID
recovery or edit engine. New Rust SDK method on the existing immutable type:

```rust
pub fn view(&self) -> AuthoringResult<serde_json::Value>;
```

It returns the closed [view](../../schemas/authoring/view-0.1.schema.json), with
experimental=true, projection-matched and executable=false. Original source occurs
once. Binding includes definition/root and exact authoring/source/bindings/semantic
integrity; derived locations/views are stale on any mismatch. `ir` is the exact
complete immutable Candidate document, including opaque annotations. A view is
neither an authoring document nor a semantic/admission/permission report, and cannot
be imported or exported as the complete artifact. Scope filters/pagination are
labeled views of a complete model; hidden content is never reconstructed from them.

Every declaration/symbol/generated row becomes exactly one entity, sorted by ASCII
ID; every semantic record in all eight IR maps is accounted for. Category/role/owner/
caption/wire/pin must match the real strict companion. The definition lineage is
separate top-level identity, not a layout entity. Unknown/unmapped kinds, missing
owners/locations/references or mismatched coverage refuse the entire view. No generic
unsupported placeholder or silent omission. Source-only unused definitions remain.

| Entity category | Role / semantic mapping | Caption and location |
| --- | --- | --- |
| Declarations | scope/data/expression/actor/capability/policy/node map to their corresponding semantic maps | Exact managed name and native declaration span |
| Source-only declarations | resource/data-protection/type-alias/predicate have no separate execution record | Exact name; resource uses companion location, other definitions native declaration span |
| Symbols | outcome/branch/input/output/purpose/predicate-input; no separate execution record | Exact alias/wire and actual owner span, explicitly identified as owner provenance |
| Generated slots | initial/assignment → expressions; flow → flows; provider-policy → policies | Fixed captions Initial value/Assignment/Outcome route/Provider requirements; slot span for first three, owner span for provider-policy |

Resource aliases have no source declaration in source19. Their exact location is
a companion JSONpointer, not a fabricated source span. Every entity's companion
pointer is its current /declarations/N, /symbols/N or /generated/N row, bound to
binding integrity; row order never determines identity. Source spans are UTF-8
half-open byte offsets. Location basis declaration/slot/owner has actual start/end;
basis companion has null source offsets. A symbol's owner span is never presented
as exact field/token precision. Source-only resource pin kind/contract/digest is
shown as a reference to exact supplied bytes, never automatic URI fetching.

Only semantic nodes carry nodeKind: activity/compute/invoke/decision/split/join/wait/
repeat/fanout/finish. Activity mode human versus capability stays explicit. Flows
are actual IR records; source/outcome/target stay exact. Their readable captions
come from the actual generated slot's explicit outcome-symbol key and fixed wire,
not slugging, matching visible words or inventing an edge. All endpoints exist.

Placement reuses exact sparse layout0.1.0. Explicit rows retain x/y/collapsed;
missing rows use sorted IDs within actual ownership groups, eight columns at
240-unit horizontal and120-unit vertical intervals, collapsed=false. Coordinates
are relative to that owner group and have no meaning/authority. Explicit is a
derived boolean. Generated captions do not repeat potentially huge owner names;
inspectors obtain owner context by ID. Collapsed/paginated views retain every entity.

## Experimental visual and textual mapping

| Meaning | Prototype representation / complete detail |
| --- | --- |
| Scope and nesting | Labeled scope frame and accessible ownership tree; declared interface/entry/ports/outcomes and cancellation/fault/closure/race rules remain in its complete source clause and exact IR |
| Human versus capability activity | Distinct labeled work cards; declared actor/capability/input/result/policy shown through complete source detail, no live authority/completion claim |
| Compute and data | Calculation/data cards, exact assignment/initializer slots and declared types/protection/invalidation; derived reads/writes shown as named static requirements |
| Decision and ordinary flows | Decision card, labeled actual outcome arrows; guard/default semantics retained in complete source detail |
| Split and join | Split/join cards and real child/join/source relationships; branch labels and paired-population/remaining-work predicate remain explicit |
| Invoke/repeat/fanout | Named body-scope links and distinct call/repeat/collection cards; all port/item/key/seal/change/join/condition details retained, no shared occurrence/storage inference |
| Wait and finish | Wait/finish cards; policies, optional exact capture and fixed finish outcome remain explicit; timer expiry is never a human decision |
| Policies, actors and capabilities | Named inspectable rule/requirement cards with full source clauses and actual semantic fields; static declarations never labeled current authorization |
| Expressions and types | Inspectable named expressions/type cards and complete accepted source/IR; nominal pins/wrappers and opaque literal/field text preserved, no inferred membership |
| Pure tests, inputs and protection names | Separate source-only definition/input/rule cards with real owners/types/parameters and all unused definitions; no runtime function or policy default invented |
| Resources and internal slots/symbols | Inspectable aliases, pins, labels and generated semantic slots attached to actual owners; raw IDs/wires/digests maintained by tools, not ordinary authors |
| Format/version/id/root/semantics/dialects/annotations | Exact binding and read-only advanced inspection; annotations retained as non-semantic notes, never substituted for required behavior |

All node-specific/core/type/expression/policy/port/resource/exceptional fields are
available in the complete exact IR and accepted source with location provenance.
Readable inspectors show the owning source clause and actual derived reads/writes /
references; do not reinterpret opaque literal/type payloads as references. No JSON
escape block or generic unknown-kind rendering substitutes for accepted meaning.
The source clause is controlled English, not a second guessed English paraphrase.
This mapping's clarity still needs actual representative users.

Provide labeled text and visual panes, complete explorer/search/owner breadcrumbs,
keyboard selection and focus, collapse/expand, source jumps and inspector details.
Diagrams supplement an accessible text/tree/list alternative; no color-, drag- or
geometry-only operation. Use DOM text, not executable HTML/URLs from source/names/
annotations. Both panes select the same explicit entity ID and update from the
same newly admitted document. Ordinary actions are readable-name rename, format,
placement/collapse and original/formatted complete export. No manual companion/
wire/hash maintenance. Owner/type/meaning changes, arbitrary text/semantic/structural/
copy/retire/IR-only/redacted edits remain unsupported and clear. An invalid draft
is separate from the last checked model; it cannot emit a successful diagram/export.

## Shared native/browser bridge

A separate small Rust cdylib/rlib adapter uses the same SDK, with wasm-bindgen
exact0.2.127 only on Wasm. Existing portability probe/cases/report/import contract
stay unchanged. Shared native/Wasm functions:

```rust
pub fn canonical_document(document: &[u8], registry: &[u8], resources: &[u8]) -> Result<String, String>;
pub fn change_document(document: &[u8], change: &[u8], registry: &[u8], resources: &[u8]) -> Result<String, String>;
pub fn view_document(document: &[u8], registry: &[u8], resources: &[u8]) -> Result<String, String>;
```

Success is respectively native document JCS, new document JCS or deterministic view
JSON. View JSON has no separate hash/canonicalization authority; exact native/Wasm
output is compared. Failure is a bounded private fixed diagnostic code/component/
static path/source span/established ID, never source/name/resource/HTTP data echo.
Library has no IO/network/clock/random/threads/execution. Native adapter CLI:
`canonical|view DOCUMENT REGISTRY BLOB`, or `change DOCUMENT CHANGE REGISTRY BLOB`;
explicit bounded files, buffered output, no stdout result on validation failure.

The closed [registry header](../../schemas/authoring/resource-registry-0.1.schema.json)
selects exact resource bytes from a separately supplied blob. Rows are ordered
contiguous ranges covering the entire blob: start0 then previous start+length,
nonoverlapping/no gaps/no unused trailing bytes. Every row <=1MiB, total<=8MiB,
max128 distinct contract/digest pins, inherited contract/digest bounds. Zero-length
resource bytes are structurally allowed but still undergo native exact validation.
No paths/fetch/discovery or self-authorizing field. Header raw/JCS <=4MiB; decoded
JSON containers<=64/items+keys<=262144. Document<=16MiB, change<=1MiB; all inherited
component/native limits remain. Browser guards byte lengths before wasm-bindgen
copies and Rust checks again. No raw bridge can relax native/IR admission limits.

View output <=16MiB,262144 values+keys,64 containers and30000 entities/10000 flows;
construction work<=1M visits/rows/ownership hops/lookups cumulatively. Refuse before
oversize growth, never truncate. Exact source/IR/components retain their original
limits. Counters do not reset per entity. No owner-name/source-subtree duplication
per child; use IDs and bounded spans. Actual generated Wasm imports must be reviewed
and finitely allowlisted as ABI support only; no WASI/filesystem/network/clock/random/
thread/provider/dispatch imports. Compilation/Node/native does not establish browser
execution. Observe actual browser parity and actions before claiming that evidence.

## Trusted local integration, outside artifacts

A loopback-only local prototype serves whitelisted static assets and a fixed
example/RP-01/RP-03 working catalog. Initial exact components/resources are captured
from explicit approved package/contract roots before serving; no arbitrary path/URI
input, directory listing, repository/tool/cache exposure or original benchmark writes.
Work on isolated temporary complete documents. Static assets contain code, not
catalog artifacts/resource bodies or private tokens. Whitelisted generated Wasm/code
assets are at most32MiB each; this code-asset bound does not raise any data quota. Read/edit/export authorization
precedes disclosure/actions, independent of process actors/protection/annotations.

Use independent ephemeral launcher-created session capabilities, in server memory,
not process JSON. Editor has read/edit/export; viewer read; exporter read/export.
The launcher passes tokens through a URL fragment; the client removes it from the
visible URL and keeps it only in memory. API sends X-Choreoform-Session; the server
checks bounded token/Host/origin against its own session/loopback context every time.
No role-elevation endpoint or artifact `authorized` claim. Deny cross-origin or
malformed requests, no implicit cookies/CORS resource grants. This is a prototype
session boundary, not production identity/revocation/DLP or runtime authority.
It cannot prevent copying content already disclosed by an authorized read.

- GET /api/catalog requires read and returns the closed [catalog](../../schemas/authoring/prototype-catalog-0.1.schema.json); permissions come from trusted session state, not an artifact.
- GET /api/process/KEY requires read, returns admitted document JSON<=16MiB.
- GET /api/registry/KEY and /api/resources/KEY require read, return exact bounded header/blob.
- GET /api/reference/KEY requires read and matching X-Authoring-Base, returns actual native bridge view output<=16MiB for the requested snapshot; stale reference refuses, not a generated browser observation.
- POST /api/change/KEY requires read+edit and change0.1.0 JSON<=1MiB. Run real native atomic change against current authoring base before committing temporary state; no command interpolation or second engine. Return exact new document bytes, which the browser compares with its Wasm result.
- GET /api/export/KEY requires read+export and X-Authoring-Base matching current authoring digest; return complete canonical document snapshot with a fixed catalog-key filename. Stale export refuses instead of silently exporting another view.

KEY is exactly example/rp01/rp03, never a path. Error responses use the closed
[private error](../../schemas/authoring/prototype-error-0.1.schema.json):401 missing/
invalid session,403 denied capability/origin,404 unknown key,400 malformed,409 stale,
413 oversize,422 native refusal,500 fixed IO failure. Native diagnostics remain
private, HTTP logs never echo tokens/payloads/paths. No partial artifact on refusal.
No endpoint dispatches a process or reconstructs hidden/redacted content. API resources
are explicit artifact inputs, not live provider observations or host authorization.

## Evidence and phase limits

This profile, exact schemas, [worked vectors](paired-0.1/vectors.json), raw pins and ADR-0096 must be signed before code.
Require complete native view/field/construct/identity/location coverage, all golden
changes and full RP01/RP03 source/view/edit/layout/export/import cycles with same IR /
revision/all11 checking outcomes and legitimate RP03 refusal. Test resource/framing /
malformed/version/opacity/stale/quota/authorization/XSS boundaries and real keyboard/
focus/labels at their layers. Observe actual browser/Wasm/native byte parity and
review actual imports. Browser automation is not representative-user evaluation;
auth-session tests are not production/provider security or runtime host evidence.

The bounded experimental prototype does not complete general authoring or standardize
notation. Broader operation contracts, full RP-08, full diagnostics/validator/
conformance/negative examples, three-process both-form cycles, actual representative
users/hosts/browsers/independent evidence and all full Phase1/G1–G4/scenario gates
remain open. Parent46 draft, no Phase2/new phase chat; honest phase report still
requires owner approval. Never generate external/user evidence to close these gates.
