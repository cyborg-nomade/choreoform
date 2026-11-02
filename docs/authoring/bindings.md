<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Portable authoring binding package 0.1.0

**Status:** Accepted provisionally in [ADR-0015](../decisions/0015-authoring-grammar-bindings.md),
owner approved 2026-10-04, effective upon merge of PR #19.
The [schema](../../schemas/authoring/bindings-0.1.schema.json) is normative for
structure; the invariants below are additional requirements. This is the concrete
format for [ADR-0014's identity requirements](mapping.md), not an implemented
editor or an accepted source compatibility promise.

## Files and integrity

A package consists of one `NAME.choreo` source and one `NAME.bindings.json`
companion. Both must be explicitly supplied together; the basename is a viewing
convention, never identity or file-discovery authority. Pinned dependency bytes
come from an explicit local registry keyed by `(contract, digest)`. Source imports,
automatic URI fetching and reusable-module formats are outside this profile.
Resources are visible in the companion but maintained by tools; authors select
their readable aliases in source, rather than copying hashes into sentences.

The source uses profile 0.2.0 and the companion has exact fields `format`,
`version`, `profile`, `definition`, `root`, `contracts`, `declarations`, `generated`, `symbols`,
`retired`, `resources`, `annotations`. No additional top-level/record fields;
unknown extension payloads belong in `annotations` and must survive losslessly.
All identity records and arrays must be present, including empty ones. The root
is an existing scope declaration whose parent is null. `definition` is a
different permanent lineage ID. All live IDs (including internal symbol/resource
binding IDs) are globally unique, follow the IR identifier grammar, and are
disjoint from retired IDs. Root is a reference to its declaration, not another
allocated ID.

Read strict UTF-8 JSON with no BOM, duplicate object keys, non-finite/floating
numeric tokens, negative numeric zero, unsafe numeric integers, trailing data or
unpaired surrogates. Arbitrary annotation strings may contain escaped controls;
binding names cannot. Each input is at most 1 MiB, nested containers at most 64.
Schema array/name bounds apply in addition. Canonicalization is RFC 8785 JCS
over the entire admitted companion, including unknown annotations. Array order
is retained: reordering rows changes package integrity even if IR meaning does
not change. Object-key order and insignificant JSON whitespace do not.

Three integrity values have an exact hash domain, each printed `sha256:` plus
64 lowercase hex digits, using the existing IR spelling:

1. Source digest: SHA-256 of **every stored source byte**, including comments,
   whitespace and any final newline.
2. Bindings digest: SHA-256 of the companion's UTF-8 JCS bytes, without a final LF.
3. Package digest: SHA-256 of ASCII `choreoform-authoring-package:0.2.0`, one
   zero byte, an unsigned 64-bit **big-endian** source-byte length, source bytes,
   an unsigned 64-bit big-endian canonical-companion length, canonical bytes.

Hashes are derived outside the companion to avoid self-reference. A digest is
integrity evidence, not a signature or authorization. Pinned resources use their
separate exact-byte hash domain, without JCS or newline removal. A package digest
incorporates resource pins, not proof that their bytes or implementation exist.
Changing the source/companion/pin changes joint integrity. The semantic revision
is still ADR-0009's hash of the lowered IR projection; it must never be replaced
by any of these three hashes. There is no claim that equivalent English has
identical meaning until binding, lowering and validation succeed.

For the checked-in [terminal source](examples/terminal.choreo) and
[companion](examples/terminal.bindings.json), the package digest is
`sha256:9d0328a4cd9266221ac31393567bc23ef6c513bfe8bb24be573047e0601b44d5`.
The test suite fixes this cross-implementation vector. Derived source maps are
separate, disposable files; they must carry source digest, bindings digest and
validated semantic revision, with UTF-8 half-open byte spans. Any mismatch makes
all spans stale. Whitespace edits invalidate spans/package digest, not ledger IDs.

## Persistent records

| Record | Meaning |
| --- | --- |
| Contracts | Exact semantic/core-suite pins and the local core dialect ID; this source profile targets the accepted ADR-0008/0013 snapshots only |
| Declaration `{id, kind, scope, name}` | Exact current binding spelling and owning scope; scope declarations use their parent here; resource aliases have kind `resource` |
| Symbol `{id, owner, role, name, wire}` | Outcome/branch/input/output/purpose alias owned by a scope, node or data cell, with independent internal ID and fixed wire spelling |
| Generated `{id, owner, role, key}` | Existing opaque IR ID associated with a semantic slot, never recomputed from name, source offset or traversal order |
| Retired ID | Tombstone reserved forever in this definition lineage |
| Resource `{binding, kind, contract, digest}` | Exactly one pin for each resource alias declaration; kind is provider, named-type, clock or calendar |
| Annotations object | Entire preserved IR annotation object, including unknown extension namespaces and identity-indexed metadata |

Declaration scopes form one connected acyclic lexical tree, maximum depth 64.
Exactly the root has null parent. Duplicate names in one scope and ancestor
shadowing across all kinds (including resource aliases) are errors; sibling-local
names may repeat. The root process name is a qualification label, not a local
declaration in its own scope. A scope name is declared in its parent. Validate
the full ledger before lookup, then resolve exact spelling and expected kind;
wrong-kind matches cannot fall through.

An outcome symbol can belong to a scope or node, a branch to a split node,
input/output ports to scopes, and purposes to data cells. Name and wire spelling
are each unique within `(owner, role)`. All five wire symbol roles follow the
existing IR pattern `[A-Za-z][A-Za-z0-9_-]{0,63}`; aliases may be multiword Unicode
names. Symbol IDs do not enter the IR's
maps; they let generated slots and refactoring track a symbol through alias edits.
Only these broad owner kinds are structurally checked by the oracle. Admission must
check node kind, exact declared symbol inventory and all use sites against source.

Source `On "correction needed"` selects the containing node's outcome symbol.
Scope `Outcomes`, `Finish`, decision guards/defaults, work result variants,
join predicates and wait-policy outcomes all resolve through their respective
owning symbols. A plain quoted variant tag is an exact literal name; `outcome
"correction needed" of "Review"` explicitly selects the node's fixed wire token.
Provider-owned outcomes retain the provider's exact contract tokens. Tool-created
aliases may improve readability but may never change a pinned provider contract.
For input/output ports, invocation keys select the child scope's symbols; binding
values retain their context-specific expression/data references. Splits use their
branch symbols. Data `Purposes` uses its purpose symbols; cross-cell protection
checks compare fixed wire purposes, not aliases. No slug-generation algorithm
is part of lookup or later editing.

Expression atoms `outcome "alias" of "owner"` and `purpose "alias" of "cell"`
produce the selected wire spelling as a Text literal. Use these when comparing
a variant tag or a policy purpose fact; bare quoted strings stay exact literal
text. This keeps comparisons readable through alias renames without teaching
authors generated wire names.

Generated slots are this profile's complete list:

| Role | Owner kind | Key | Generated IR record |
| --- | --- | --- | --- |
| `initial` | data | null | Expression for the optional initializer |
| `assignment` | compute node | Target data ID | Expression for that assignment |
| `flow` | ordinary node | Its outcome symbol ID | Successor flow |
| `provider-policy` | capability | null | Capability-policy wrapper for its provider pin |

Each `(owner, role, key)` has exactly one ID when the slot exists in source.
Policy predicate/operator ASTs and type descriptors are embedded and have no
individual IR IDs. All other expression/policy declarations are explicit named
source declarations. Assignment keys use target IDs; flow keys use symbol IDs,
so reordering assignments and renaming outcomes cannot reallocate their records.
Changing an existing slot's expression or destination keeps the record ID but
changes IR meaning/revision. Removing a slot retires its ID. Creating it later
allocates a fresh ID; the old tombstone cannot be recovered implicitly.

Resource kind selects the required validator, not an executable guarantee.
Exact bytes for both core contracts and resources must be locally supplied and supported; hashing opaque data proves
only equality. Resolve contract IDs/digests into semantic IR references. Pins
cannot live solely in annotations. Clock/calendar resources remain subject to
the accepted bounded timer contracts, with unsupported implementations refused.

## Transactions and plain-editor reconciliation

Tools must stage source and companion edits together, validate the complete
result, and publish the pair atomically or publish neither. The specification
does not mandate an editor UI or filesystem atomicity protocol. The oracle's
`rename_ledger` is only a pure model of the **companion half**; it deep-copies,
validates before and after, and leaves inputs untouched on failure. It is not
a safe source refactoring command.

- **Create:** an explicit create operation allocates a fresh definition lineage
  and opaque unique IDs. Neither names, hashes nor source positions choose IDs.
  Missing companion for an existing lineage is an error. An explicit new-lineage
  copy cannot claim to restore lost identity or migrate live instances.
- **Rename:** identify the existing declaration/symbol by ID; change its name and
  all resolved source references, including qualified scope parts, result tags
  and join/wait references. Keep ID, wire symbols, slots and unknown annotations.
  Reject collisions, shadowing and unresolved use sites before committing.
- **Raw editor changes:** parse independently, compare complete inventories and
  reject stale/missing/extra ledger entries. Offer explicit user reconciliation
  mapping each changed declaration to an old ID or to fresh creation; never
  match by similar spelling, line number or traversal. Confirm references and
  ownership with semantic validation. Ordinary formatting needs no reconciliation.
- **Copy:** allocate fresh IDs for copied declarations, their owned symbols and
  generated slots; remap every internal reference/annotation target, retain
  approved external pins/ancestors, and select collision-free names. An opaque
  unknown annotation that cannot safely remap blocks copy; retaining stale
  internal annotation targets is not lossless editing.
- **Delete/recreate:** remove all dependent references or refuse; retire every
  removed identity, including symbols/generated records. Fresh work must not reuse
  any retired ID, even when its name and action match the deleted declaration.
- **Move:** preserve ID only through an explicit ownership-changing transaction;
  revalidate visibility, types, protection, ports, generated records and resource
  context. This can change semantics despite identical spelling.
- **Merge:** reconcile both source and companion against their common ancestor.
  Conflicting names, wire spellings, ownership, slot IDs, tombstones or pins are
  explicit conflicts; never last-write-wins or automatic ID regeneration.

The whole pair, pins, instructions, names, comments and annotations are protected
source, as already required by [mapping.md](mapping.md). A redacted projection
must be marked incomplete and cannot replace/approve the whole definition.
The ledger is neither an authorization bypass nor a source of credentials.
Live instances remain bound to their recorded definition revision; editing or
merging a package does not migrate them.

## Full admission requirements

After strict transport and ledger checks, a Rust frontend must parse a bounded
AST, compare source inventories exactly, resolve all symbols/resources, preserve
conservative reads and annotations, lower deterministically, and validate the
accepted IR/dialect contracts. Re-export of an admitted IR must retain all IDs,
existing wire symbols, AST distinctions and unknown annotations. Imported IR
must use explicit import, not generate a replacement lineage. Unsupported
payloads must refuse export or produce a clearly incomplete inert view; a
well-formed companion alone never grants admission.

The specification PR's tests cover grammar recognition and companion invariants, not those
full admission/editing requirements. [Evaluation and remaining gates](../evaluation/0015-authoring-grammar-bindings.md)
keep the distinction explicit.

The subsequent [Rust implementation slice](../../crates/authoring-frontend/README.md)
adds exact inventory/reference checks and supported candidate lowering, with
source-package format/reparse tests. It remains short of full semantic admission,
general identity-preserving IR export and transactional editing; see
[its evidence](../evaluation/0015-rust-authoring-frontend.md).
