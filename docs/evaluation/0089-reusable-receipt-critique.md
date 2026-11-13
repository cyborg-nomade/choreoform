<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Reusable receipt and work authoring critique

**Date:** 2026-11-13<br>
**Signed scope:** `0e4c706d3d865ad45cfa159b9e3123c7cd5d346f`<br>
**Verified baseline:** PR #103 merge `46e6566e1f0c90f2d027911b40af466fad97d1e5`,
CI43448852892 at1778130; automated COMMENT review5893351354, fresh complete audit

## Problem and observed baseline

Named protection and transparent types reduce actual RP-01 to55453 bytes/9455
tokens and RP-03 to410752/66783. They leave repeated receipt predicates and work
corridors. Existing native measurement reports22971/204788 bytes in outer expression
spans; exact-spelling redundancy upper bounds are only128/16268. These categories
overlap other spans, exclude helper overhead and do not measure prospective savings.
There are7/29 work policies,49/133 expression records and3/25 capabilities. Python
producer helper reuse has not given ordinary authors reusable language constructs.

RP-03's23 named operation evidence types have the same complete structural
representation but retain separate nominal names and operation-specific invariants.
That is a promising boundary: a future pure predicate could take an explicitly
unwrapped representation and the exact common instruction record, with explicit
operation/phase inputs. Unwrapping already exists at the real call sites; it must
remain explicit and never create membership in another nominal type.
Complete positive/negative-case predicate structure must be inspected before
choosing helper boundaries or claiming equality. Balanced conjunction trees differ
with13 versus14 leaves: appending a reason check to an already-balanced13-leaf
helper would change the old AST. Do not assume algebraic equivalence is exact IR.

Current IR supports typed `param` leaves and finite fanout-item binding, not general
expression calls. Definition checking rejects other nonempty parameter contexts
with parameter-binding; ordinary node expression references also require no
parameters. The closed operation match rejects a call operator. A reusable source
recipe therefore needs its own source-only contract, or a separately accepted
IR/runtime call contract; existing syntax does not already grant either behavior.

## Options and provisional recommendation

| Option | Benefit | Required proof and cost |
| --- | --- | --- |
| A: typed, source-only pure predicate recipes expanded at each use | Can remove repeated receipt text while retaining the complete old AST and wire meaning | Explicit typed inputs; no ambient facts/reads; per-use literal-opaque substitution and native typing/dependency checks; stable source-only IDs; bounded all-unused DAG validation and cumulative expansion |
| B: IR function/call semantics | Reuse remains visible in IR and may avoid expanded storage | New accepted core/schema/expression/runtime/call-context/dependency/visibility contract; changes benchmark IR/revisions; current parameter fields alone are insufficient |
| C: scoped work/service templates | Could shorten whole retention/capture/compensation corridors | Fresh managed scope/work/storage/endpoint/occurrence IDs; direct-child ownership, exact provider claims and per-use lifetime/permission/compensation proof; far broader than pure expressions |
| D: formatting and protected inspection only | Makes existing meaning navigable without semantic expansion | Does not remove receipt repetition; protected accessible inspection/editing is still required alongside any factoring |

Provisionally prefer **A**, with explicit structural representation inputs and
separate complete predicate shapes where necessary. Keep B/C separate. Before a
production contract, perform a finite read-only AST analysis and exact reconstruction
against the actual whole RP-03 predicates, preserving nominal provenance and all
work identities. It is an experiment, not a compiler, working language, runtime,
measured future reduction or user study. Do not select paper wording as final syntax.

RP-01 combines actor/principal/current-policy/approval dependencies with provider
receipts; no order-specific helper should silently replace those checks. RP-08 is
a full fictional brief, not a lowerable package: dynamic scope/work, confidential
custody, contradictory evidence, authorization, outage reconciliation and explicit
handover cannot be replaced by a receipt Boolean. Complete RP-08 remains Phase1 work.

## Required next boundary

A future source-only recipe must require explicit typed parameters, lexical
visibility and source-only identity. Its body must be pure and closed over its
parameters; no ambient policy facts, storage, dispatch, imports or implicit values.
Literal values/type descriptors are opaque to substitution. Validate unused /
forward/cyclic definitions before a candidate, all nominal pins and types, and
bounded total cache/use/depth/node/work/output under existing shared quotas.
At each use, substitute ASTs hygienically, preserve complete old field/tag/operation /
phase checks and exact balanced shape, then run actual native typing/reads /
invalidation/protection checks in the caller's real context. Fail privately with
no partial candidate. Protected summaries must disclose effective meaning within
current authorized visibility; a name/summary is not current actor permission.

General safe managed editing/export/visual round trips/actual representative users,
full RP-01/03/08 and semantic/external-region/G1–G4/phase criteria stay open. All
scenarios remain Partial, no Phase2. Durable direction and measured finite findings
follow before any production language implementation.

## Finite measured findings after the signed critique

[Native current metrics](0089-current-native-metrics.json) reproduce exact source
bytes/tokens/parser spans and digests from the existing native utility. The new
[finite AST inventory](0089-receipt-reuse-ast.json) reads the actual883289-byte
RP-03 candidate and418692-byte type resource, verifies candidate/type provenance,
and reconstructs all55 complete predicates:26 thirteen-leaf cases and29 fourteen-
leaf cases, exactly two template hashes. Twenty-five activity/policy/capability /
request identities stay distinct. Twenty-three nominal outputs share one complete
representation, and all input descriptors match. The two template ASTs are2770 /
2968 canonical JSON bytes; these are structural observations, **not source-size
savings**. Dispatch/tag guards and false fallback remain outside these predicates.

The actual candidate revision remains a3b8c0f0, wire6550ac45; no benchmark bytes
change. The read-only tool bounds inputs at1MiB each, traversal at100000 visits /
64 levels and emitted report at65536 bytes. Substitution stops at explicit whole
AST terms and does not enter literal data/types or labels. The complete two shapes
reconstruct every original case; no runtime evaluation, fact authentication, new
membership or compiler implementation follows. Unknown operators and quota excess
refuse. Parent ASTs use shallow shells plus recursively rebuilt children, avoiding
repeated deep parent clones; literal payloads/explicit replacement terms are copied.

Four oracle groups reproduce native metrics and finite inventory, pin every current /
historical benchmark byte, check nominal/request/shape/identity counts, exercise
literal opacity/depth/work/unknown-op boundaries, rerun both actual native static
checks and preserve the genuine RP-03 concurrency refusal. A real current RP-01
nonempty unused parameter declaration lowers but refuses parameter-binding during
static checking; general call semantics have not been silently enabled.
[ADR-0089](../decisions/0089-reusable-receipt-direction.md) selects A as the separate
next production-contract direction, not accepted syntax or a user study.

Local validation: all four new oracle groups pass, together with the six existing
authoring-evidence groups and all changed-document links. No Rust or production
frontend/SDK file changes; baseline660 Rust+six compile-fail/native/Wasm/83 Python
commands remain recorded at PR #103. Required final-head CI adds the84th Python
command and reruns the full suite before integration. Automated review/full
comments/CodeRabbit audit/expected-head signed merge remain required; no independent
review or scenario/phase closure follows.
