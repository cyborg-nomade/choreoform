<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.18.0

**Status:** Accepted under ADR-0086 / owner delegation; effective on integration.

Inherit [source 0.17](profile-v0.17.md) except the explicit additions below.
Header `Choreoform authoring "0.18.0".` pairs only with companion 0.17/profile 0.18
and [its closed schema](../../schemas/authoring/bindings-0.17.schema.json).
The [grammar](candidate-a-v0.18.ebnf) is normative. Select unchanged core 0.12 /
IR 0.4 and every inherited exact resource contract; no source selection of C13.
All previous profiles keep their syntax, kinds, staging and quotas.

## Explicit transparent type descriptions

```text
Describe type "Reservation instruction" as record with ["Warehouse" as text].
Keep "Instruction" as type "Reservation instruction", protected as "Order information".
  Invalidate empty.
  Initially unavailable.
End data.
```

This is a syntax excerpt, not a complete process or permission grant. A named
protection definition must also be declared. `Describe type NAME as type.` binds
a complete description; `type reference` uses it, with ordinary exact lexical
qualification such as `type "Reservation instruction" in "Order fulfillment"`.
The existing `type "Order" from "Order types"` still selects an explicitly pinned
nominal resource/member; it is a distinct production, tried before the local form.
No bare quoted type, parameterized type, recursive type or unrestricted prose is
accepted. A definition may reference a later finite definition, and all existing
type forms can contain local references at any type position.

Local descriptions are transparent: expand to exactly the existing type AST,
including nested nominal contract ID/revision/member, field names, choice/outcome
tags, integer/decimal scale, list element and optional meaning. They create no
semantic type or nominal membership, erase no named wrapper, merge no logical
work/storage/endpoint and grant no permission. Existing literal typing, wrap /
representation membership and resource validation remain in force. Data,
expression result/parameter/literal, policy observation and capability input/output
positions all use the same meaning; old explicit inline forms stay available.

## Managed scope, references and integrity

The companion declares source-only kind `type-alias`, stable explicit ID, owning
scope and exact caption. No semantic wire record or symbol role is added. Collect
all definitions before uses; validate every definition including unused/forward
references. Enforce exact kind, same-scope duplicate, ancestor no-shadowing and
complete lexical qualification rules. Bind resource and outcome-owner references
in the definition's scope; retain their exact managed declaration IDs as provenance
and check their visibility/kind again at each use. Hoisting a description cannot
expose a private sibling resource or node/outcome. Source order never allocates or
recovers identity. Different captions or positions cannot invent equivalent types.

Renaming updates explicit source/companion captions with stable IDs; reordering
retains IDs; copying requires fresh source IDs; deletion reserves tombstones.
Stale/missing/surplus/wrong-kind/retired bindings refuse. Unknown annotations and
source-only IDs survive format/reparse/integrity. General editing, safe arbitrary
IR export and protected accessible inspection are still separate work.

## Preflight and finite expansion

A finite directed graph of transparent descriptions must be acyclic. Validate the
complete graph, including unused cycles, before building expanded cached values.
Compute expanded type-node/JSON-node work, compact serialized bytes and both type /
JSON-container depth bottom-up with checked arithmetic before allocation or copy.
Depth reflects expanded structure, not merely alias-chain length. A short acyclic
DAG may describe exponentially many fields and must refuse before expansion.

Charge resolution/preflight visits and every complete cached description/use
against the existing cumulative lowering/dependency budget, shared with named
protection. Charge compact bytes of caches and repeated uses against the existing
1 MiB cumulative expansion ceiling; do not reset per alias or per type position.
Immutable descriptions cache no live values/access truth. A repeated reference
still pays its full expanded cost, even if resolving its description was cached.
Preserve emitted-record/final wire bounds and all source/companion 1 MiB, depth64,
tokens131072, parser1M, lowering100k and semantic/shared limits. Final wire framing
can impose a tighter depth/size bound than a standalone type; no partial candidate
or truncated success is emitted. No old quota is raised to admit the new syntax.

Diagnostics identify meaningful definition/use UTF-8 spans without echoing source
captions, confidential literals or companion contents. Format/reparse preserves
exact expanded body/revision and managed identities; integrity separately frames
this new profile. No claim of general semantic admission follows from lowering.

## Required evidence and remaining work

Test complete source/ledger/resource cases: exact structural and nested nominal /
outcome meaning; unused/forward/cyclic/private/wrong-kind/qualified references;
managed rename/reorder/fresh-copy/tombstone/stale inventory; mixed local/inline /
named-protection forms; every old pair; future refusal; annotations/integrity /
format; cumulative cache/use/wide/deep/exponential expansion/work/output bounds.
Actually lower/check/reparse whole RP-01/RP-03 variants on explicit C12 baselines,
compare exact IR/revisions/provider/guard/receipt meanings and measure genuine
source reduction including definition/use costs versus the protection-only
variants (56059 / 448421 bytes). Preserve the legitimate concurrency refusal.
Do not overwrite current benchmarks or invent a full RP-08/user study.

Benchmark adoption/archive, parameterized work/predicate reuse, C13 source selection,
full RP-08, protected accessible managed editing/inspection/export/text-visual,
actual representative users/hosts/browsers/independent evidence and every unproved
semantic/region/phase gate remain Phase 1 work. Parent #46 stays draft; no execution,
phase completion or Phase 2 follows.
