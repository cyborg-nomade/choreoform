<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Recorded evidence: typed claims 0.1.0

**Status:** Exact contract accepted under ADR-0098 / owner delegation; production implementation pending.<br>
**Contract ID:** `urn:choreoform:recorded-evidence:typed-claims:0.1.0`

These exact bytes define a separately versioned inert artifact. P is the Ref to
this contract ID and its raw SHA-256, published in ADR-0098. Select unchanged C12
value/type/resource judgments, including exact semantics and cores1–12 premises;
verify the13 raw premises (semantics plus cores1–12), as well as P
and every transitive named-type dependency from explicit supplied bytes. No C13 /
source20/IR change, provider/clock/definition selection or new process behavior.
Semantics is `urn:choreoform:semantics:adr-0008` at
`sha256:0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141`.
The artifact/schema is not a definition, plan, authoring document or runtime history.

## Closed artifact and vocabulary

```
{format:"choreoform-recorded-evidence",version:"0.1.0",profile:P,core:C12,
 id:ID,namespace:Name,sources:Map<ID,Source>,clocks:Map<ID,ClockClaim>,
 records:Map<ID,Record>,boundaries:Map<ID,Boundary>,relations:Map<ID,Relation>}
Source = {identity:Name,caption:Name}
ClockClaim = {source:ID,basis:Name,unit:"microseconds",origin:Name}
Record = {source:ID,event:Name,claim:Claim,delivery:Delivery}
Claim = {kind:"observation"|"hypothesis"|"decision"|"action",subject:Name,
 context:null|Context,reportedTime:null|Time,payload:{type:T,value:V},
 original:{text:Text,digest:Digest}}
Delivery = {receipt:null|Name,channel:Name,recorder:null|ID,reportedTime:null|Time}
Time = {basis:ID,instant:Integer}
Context = {definitionRevision:Digest,instance:Name,scopeOccurrence:Name,
 site:Name,occurrence:Name,attempt:Integer,inputRevisions:List<{cell:Name,revision:Name}>}
Boundary = {namespace:Name,source:Name,event:Name,digest:Digest}
Relation = {kind:"asserted-precedes"|"asserted-causes"|"asserted-disputes",
 from:ID,to:ID,basis:ID}
```

All shown fields required, all other fields refused; no default or extension bag.
Maps are unordered, arrays retain order. Ref is exactly `{id:Name,revision:Digest}`.
C12 is `urn:choreoform:contracts:core-profile:0.12.0` at
`sha256:73158731c33084fde009abcc86530926447902609fda9535f04daa7c768c2007`.
ID is ASCII `[A-Za-z][A-Za-z0-9_-]{0,63}`. Name is1..4096 Unicode scalars, no C0/C1
controls, surrogate, U2028/U2029; raw metadata <=16384 UTF-8 bytes. Equality is exact,
case-sensitive, no Unicode normalization, URI resolution, caption/position recovery
or physical-identity inference. Digest is `sha256:` plus64 lowercase hex digits.
Integer is the inherited canonical signed64 decimal **string**, no -0/leading zero.
T/V are the unchanged exact C12 type descriptor/value, including nominal identity /
invariants, not a new payload language. Resource IDs/pins use this artifact's Ref/Name bounds even inside T; other
payload-internal names/literals retain inherited rules; arbitrary literal strings/field text are never interpreted as
IDs, permissions or URLs. Text is any valid Unicode-scalar string, including newlines /
controls, within its byte limits. Original text is opaque literal UTF-8 evidence;
no JSON/HTML/script/type/command is executed or inferred from its contents.

Sources.identity is unique across source rows. A source is a claimed documentary
namespace, not authentication or a supported provider normalizer. Clocks.source
references a source; `(source,basis)` is unique. Clock origin/unit are claims,
not P5 clock resources, calibration, current clock readings or timer eligibility.
Namespace/source/event strings identify documented claims only, never an actual
principal, instance, accepted event, physical asset, live value or permission.

## Typed claims, originals and deliveries

Every record source exists. A non-null context is the complete documentary claim
shown above; attempt is1..1000; inputRevisions has unique exact cell names. It
claims no actual definition/instance/assignment/occurrence/attempt/input revision
exists. Context is null when unknown/unrepresented; original bytes remain literal
evidence, not a hidden supported context or normalization proof. Required behavior
cannot be supplied by opaque text instead of an accepted process/work contract.

Validate T with the existing native resource/type descriptor judgment and V with
the existing literal/nominal-invariant judgment using **one** cumulative Resources
instance per admission. All explicit pins/transitive resources must match exactly.
Unknown type/profile/semantic fields refuse; no inert fallback masquerades as a
supported typed payload. This validates a supplied claim value, not the relation
between that value and original text, source truth or a real normalization adapter.
A well-typed decision/action/authorized-looking value is still untrusted data.
Support is finite: later SDK additions cannot expand this version's C12 type,
nominal-invariant/operator/profile acceptance without a new exact contract.

Original.digest is raw SHA-256 of original.text's decoded UTF-8 bytes, not its JSON
spelling or JCS. Retain those bytes exactly, including whitespace/newlines; do not
parse/normalize them. This first profile stores UTF-8 records only. A binary file
reference can remain explicitly typed claim data; it is not embedded binary support,
proof of custody/possession, automatic fetch or understood external semantics.

A claim's asserted identity is `(artifact.namespace, Source.identity, Record.event)`.
Its immutable core is exactly `{namespace,source,event,claim}`, where source is
Source.identity, not the internal source ID/caption. Core.claim equals the input
claim except that non-null reportedTime is projected to the complete object
`{source:Source.identity,basis:ClockClaim.basis,unit:"microseconds",origin:ClockClaim.origin,instant:Integer}`
using its explicit clock reference. Null stays null. This binds the actual claimed
basis/domain while excluding accidental local clock IDs: consistent ID renaming
preserves a claim digest, changed basis/origin does not. It performs no timezone /
coordinate conversion, clock read, trust judgment or original-text normalization.
For the same asserted identity, full JCS core bytes must be equal. Equal repeats are allowed as distinct
record witnesses with their original delivery metadata. Different core bytes refuse
`evidence-identity`; no last-writer, timestamp winner or overwrite. Equality is not
inferred from a hash alone. Do not coalesce/remove input rows or rename their IDs.

Identity consistency is checked within this supplied artifact, not against an
ambient history/database. Independently valid artifacts can assert conflicting
unverified claims; changed core digests expose that distinction, not a global
uniqueness/truth guarantee. There is no cross-artifact merge/acceptance method.

Record map IDs identify documentary witnesses, not original event or receipt IDs.
Delivery.receipt is an optional claimed external receipt reference, channel is
literal provenance, recorder is a source ID or null. Claim.reportedTime's basis
exists and belongs to the record's source. Delivery.reportedTime is null if recorder
is null; otherwise its basis belongs to that recorder. Times do not certify receipt
or occurrence, imply current authority, order different clock bases, or create an
ordering edge even on the same basis. Original unnormalized timestamps can remain
in original text; no timezone conversion or missing coordinate is invented.

## Explicit unknown boundaries and asserted relationships

Record and boundary map IDs are disjoint. A boundary names an unavailable claimed
core by exact `(namespace,source,event,digest)`; digest is the claim digest defined
below. No bytes/authority are thereby supplied, fetched or reconstructed. A boundary
cannot repeat a local asserted identity. Equal external identities require the same
digest; repeated equal boundary aliases remain in the input, differing digests refuse
`evidence-identity`. Unknown content/truth stays unknown. Sources/clocks/relations
use their own explicit map namespaces; only record/boundary endpoints share one.

Relation endpoints must be local records or explicit boundaries; basis is a **local
record witness** documenting the assertion, never a source grant. Kind says asserted,
not authenticated/accepted. Relations need not be proven by parsing original text;
a supported trusted promotion/normalizer would require a separate future contract.

For consistency checking, equal local asserted identities are one event class;
equal boundary identities are one boundary class. Retain every witness, delivery,
relation and explicit reference in export. Project asserted-precedes and asserted-
causes onto these classes. Reject a self-edge or any cycle, including cycles hidden
by duplicate witness aliases. Asserted-disputes requires different classes and is
not an order edge. Repeated compatible relationship witnesses are allowed; charge
all of them and retain their provenance. No new edge comes from basis references,
map/list order, clocks, IDs, display ordering, confidence or equal-looking payloads.

A valid asserted-order DAG is not known historic causality or a total order. Return
no transitive all-pairs order or purported unique chronology. Unrelated/tied/unknown
records remain incomparable unless an explicit asserted path connects them; that
path is still a claim. Explicit unknown boundaries do not make a complete history.
Representation checking cannot accept past decisions, rewind terminal history,
change assignments/effects, authenticate anyone, dispatch, replay, settle obligations
or confer source/runtime/host authority. These are separate actual trusted actions.

Refusal leaves no successful partial wrapper/report/export and does not mutate the
input. A caller may retain refused raw candidates only in a separately authorized
quarantine; this library has no quarantine IO or discard/overwrite operation. Conflicts
between different asserted identities may be retained as disputed claims. Inconsistent
same-identity cores/order cannot be laundered through a successful artifact; retain
those failed originals outside admission rather than silently choosing a winner.

## Integrity, access and limits

Artifact digest is SHA-256 over ASCII `choreoform-recorded-evidence:0.1.0`, zero byte,
and JCS of the **whole** artifact. Claim digest uses ASCII
`choreoform-recorded-claim:0.1.0`, zero byte and JCS of its immutable core above.
Neither digest is stored self-referentially in that object. All fields, original
bytes-as-string, deliveries, aliases and asserted links remain covered. JCS key order /
JSON whitespace/escape spelling do not change an artifact digest; decoded original
text changes do. Neither hash is a signature or authority, nor interchangeable with
IR/authoring/provider/original-input/effect hashes. Canonical export is deterministic
complete input preservation, not a new runtime-history reconstruction.

The trusted caller/integration authorizes complete artifact read/inspect/export
before invoking/disclosing these APIs; a pure library cannot authenticate its caller.
No artifact field, payload Boolean, actor-like name, digest, type or report supplies
that authorization. Read can copy already disclosed data; no DLP claim. This contract
contains no redacted audience view/edit/export or generic declassification. Any later
integration must define and enforce current audience access outside untrusted inputs,
covering original/payload/provenance/context/labels/topology/timing and stale integrity.
A successful representation report is not permission to expose even its counts/hash.

| Inclusive limit | Bound |
| --- | --- |
| Artifact raw UTF-8 / JCS / complete export / report separately | 1 MiB each |
| Outer JSON nesting / values plus keys | 64 containers /262144 items per artifact |
| Sources / clocks / records / boundaries / relations | 128 /128 /1024 /256 /4096 rows |
| Original text per witness | 65536 decoded UTF-8 bytes |
| Context input revisions | 1024 rows, unique cell names; inherited Name bounds |
| Explicit supplied registry | 128 distinct pins;8 MiB total raw;1 MiB each; metadata Name bounds/digest syntax before hashing/indexing/copying |
| Registry wire | Exact resource-registry0.1.0 framing;4 MiB raw/JCS,64 containers/262144 items, contiguous complete blob<=8 MiB |
| Construction steps after bounded decoding | 1M cumulative JSON/member/row/reference/index/graph visits/lookups, never reset per record/alias/cache hit |
| New serialized/hash framing work | 8 MiB sum of artifact hash frame, each witness's original UTF-8 hash input and each witness's generated core hash frame; streamed core encoding counted once, not again for feeding the hash. Charge aliases too, preflight before growth; getters reuse cached admitted hashes |
| Native type/resource work | Existing one cumulative1M Resources budget and all inherited component/type/value/resource/depth/collection limits, separately in force for the whole admission |

Strict transport rejects BOM, duplicate keys (including escaped aliases), invalid
Unicode/surrogates, floats/exponent tokens, -0, JSON integers outside [-9007199254740991,9007199254740991], unknown/missing
fields/trailing data and bounds before information-losing conversion. Logical I64
values remain canonical strings. Check raw and generated bytes independently before
copy/expansion; no quota resets per row/pin/cache. Decode may be repeated only within
the fixed byte/item bounds; native type checks reuse one resource context. Bound
encoded core before allocation and charge all repeated framing, including namespace /
source strings. Do not clone a source/claim subtree or build all-pairs closure merely
to display relations. Fail wholly before oversize growth, never truncate success.

## Exact future native and bridge binding

A new dedicated Rust crate exposes immutable `RecordedEvidence` and:

```rust
pub enum EvidenceComponent { Artifact, Registry, Payload }
pub struct EvidenceDiagnostic {
    pub code: &'static str,
    pub component: EvidenceComponent,
    pub path: &'static str,
}
pub type EvidenceResult<T> = Result<T, EvidenceDiagnostic>;
impl RecordedEvidence {
    pub fn admit(raw: &[u8], resources: &[Resource<'_>]) -> EvidenceResult<Self>;
    pub fn document(&self) -> &serde_json::Value;
    pub fn export(&self) -> EvidenceResult<Vec<u8>>;
    pub fn digest(&self) -> &str;
    pub fn claim_digest(&self, record: &str) -> EvidenceResult<String>;
    pub fn report(&self) -> EvidenceResult<serde_json::Value>;
}
```

No public constructor/mutable Value replacement, append/merge/state/history/permission
or actual-acceptance method. Unknown/oversize query record IDs refuse privately;
claim_digest returns a cached admitted digest, not a fresh validation allowance.
Document borrowing cannot mutate the admitted model. Resources is the unchanged
explicit-byte Resource interface from ir-probe-core; no resolver/callback/IO.

The report is exactly
`{format:"choreoform-recorded-evidence-report",version:"0.1.0",stage:"representation-checked",executable:false,authenticated:false,acceptedHistory:false,integrity:Digest,counts:{sources:N,clocks:N,records:N,claims:N,boundaries:N,relations:N}}`.
Counts are documentary rows, except claims counts distinct **local asserted identity
classes**, not actual events, deliveries, decisions or accepted work. No source text,
current principal, chronology, dispatch handle or successful execution stage.

Dedicated bridge functions (native and wasm-bindgen exact0.2.127):

```rust
pub fn inspect_evidence(raw: &[u8], registry: &[u8], blob: &[u8]) -> Result<String,String>;
pub fn export_evidence(raw: &[u8], registry: &[u8], blob: &[u8]) -> Result<String,String>;
```

Return report JCS or complete artifact JCS, no separate semantic interpreter. Reuse
exact published registry0.1.0 headers/ranges and typed explicit resources. Browser
sizes must be guarded before ABI copies; Rust checks again. Later actual module
imports and browser execution require finite reviewed ABI/import evidence, never
assumed from compilation/Node/native. Existing authoring bridge/CLI remain exact.

New CLI: `inspect|export DOCUMENT REGISTRY BLOB`; no claim-lookup CLI, discovery,
network, event ingestion or implicit file writes. Bound explicit file reads at max+1,
buffer successful output completely, no stdout artifact/report on validation failure.
Errors are at most256 UTF-8 bytes, private fixed `code at component/path`, with
component spelled `artifact|registry|payload`. No arbitrary metadata/index/key /
payload/path/ID echoes. IO/argument errors are fixed `read|write|arguments|size`.
Native codes: evidence-format/profile/limit/registry/resource/payload/identity /
reference/order/integrity, with components/path drawn only from Artifact/Registry/
Payload and fixed `/`, `/profile`, `/core`, `/sources`, `/clocks`, `/records`,
`/records/claim`, `/records/claim/original`, `/records/claim/payload`, `/boundaries`,
`/relations`. Whole precedence among independently malformed fields is not promised.

## Evidence and remaining gates

Publish/sign exact schemas, profile/API, authored example and positive/negative
vectors/raw pins before new production reader/bridge/native code. Schema shape alone
is not full type/pin/graph/budget admission. Contract fixtures are synthetic source
claims, not captured incident observations, authentic principals or accepted history.
Existing native type judgments may establish baseline value facts only; a test-only
read-only vector oracle is not the later production implementation or host proof.

This contract is one representation prerequisite for RP08-D, not its complete trusted
reconciliation or full RP08. Batch/result/ad hoc-work/effect/guard/audience/general
editing/full validator/diagnostics/conformance/three-process cycles and actual users /
independent/provider/host/physical-alias/original-snapshot/accepted-surviving-work
premises remain Phase1 obligations. All40 scenarios Partial, G1–G4 conditional,
parent46 draft. No Phase2, notation standardization or phase completion.
