<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Type naming critique before further syntax

**Date:** 2026-11-13<br>
**Scope:** signed `ff0752339788099bb99abed486ff8209ee091fe0`, [plan](0085-type-naming-critique-plan.md)<br>
**Dependency met:** #99 merge `1bd547425c126500fb8d83d6c0e267a5471982ea`, final-head CI
`43404656137` at `b2b00438c8379c56c954a17071c150abfed2d24b`, automated COMMENT
review `5889074244` / fresh full audit

## Problem and existing evidence

The goal is broad plain-English accessibility with explicit meaning. Candidate A
is provisional; no actual user study is available. Named protection now removes
real repeated wording without changing RP-03's IR: its new test variant is 448421
versus 576759 source bytes, 22.25% smaller. This does not make a 448k-byte process
accessible or supply a full editing/visual representation. Current benchmark files
remain fixed, and the RP-03 concurrency refusal is legitimate.

[ADR-0083's native measurements](0083-near-english-factoring-critique.md) already
establish outer type bytes 6390 / exact redundant upper bound 2491 in RP-01 and
98058 / 70621 in RP-03. Outermost type spans exclude nested double counting within
the category. Type occurrences inside expressions can overlap expression metrics;
never add these categories. Equal spelling alone does not prove the same lexical
binding, and the 70621 bytes are an upper bound before definition/use costs, not a
measured future reduction or comprehension gain. ADR-0084's span transformation
leaves type clauses intact; no additional savings claim follows from that fact.
RP-08 still has a complete intent brief but no full lowerable source. New paper
examples cannot replace that missing benchmark.

## Inspection of actual resolved families

After signed critique checkpoint `7263cf9`, read-only inspection of current complete
IR compares exact serialized type values at the following explicit field positions.
These counts are not a complete source-type inventory, do not count nested types
separately and must not be summed with the source-byte/overlapping categories:

| Actual IR position | RP-01 positions / distinct values | RP-03 positions / distinct values |
| --- | ---: | ---: |
| data.type | 26 / 16 | 155 / 75 |
| expressions.resultType | 49 / 11 | 133 / 29 |
| capabilities.input and output | 6 / 6 | 50 / 24 |
| wait policies.body.observationType | 3 / 3 | 25 / 23 |

RP-03's common complete request record occurs 50 times among data types and 50 times
among expression result types; all 25 capability inputs have one exact shape. It
contains address/basis/request/scope revisions, amount/currency, the nominal Order,
quantity and warehouse. Naming this record must retain every field and nested
nominal pin; it does not merge the 25 effectful service/work occurrences. Output /
observation families include distinct nominal Release A versus Release B evidence;
a shared readable category cannot erase those member identities. RP-01 also has
repeated Claim/list and expression-result families, but its smaller type-byte
upper bound implies less likely source reduction, not less semantic importance.

The [RP-08 brief](../representative-processes/08-cybersecurity-incident-response.md)
requires observations separate from hypotheses/decisions, protected evidence and
custody references, changing asset/scope revisions and accountable dynamic work.
Readable type names may express those concepts, but cannot establish trust, custody,
current access or a complete lowerable process. Dynamic playbook fragments need
separate work identity/hygiene and instance semantics. This is inspection of the
fictional benchmark brief, not incident-response advice or actual user findings.

## Options

| Option | Potential human benefit | Preservation/cost |
| --- | --- | --- |
| Explicit local transparent type names | A readable reusable name can replace complete repeated type descriptions at data, expression, capability and policy positions | Must expand to the exact existing type, including every nominal pin and tag; needs managed source identity, scopes/cycle validation and pre-allocation bounds |
| Existing pinned nominal resource types | Already names durable domain concepts and checks membership | Creating a new nominal resource merely for spelling changes type identity/accepted values; cannot silently convert structural records or erase existing named wrappers |
| Layout/progressive disclosure only | Shows a concise type caption with inspectable detail in an eventual protected editor | Leaves the persisted source repetition and current lack of general editing/visual tooling; actual accessibility still requires user evidence |
| Parameterized type/domain library | May express families of related instructions/results | Adds parameter kinds, hygiene, resource/lexical binding, expansion/version/security obligations; not justified before a finite transparent form |
| Defer types until work/predicate templates | Shared service corridors could remove more repeated wording overall | Direct-child work/storage/endpoint/recipient/retention/compensation identities need separate instantiation proof; type naming can progress without sharing logical work |

Prefer a finite **explicit local transparent type name** as the next production
direction. It can reuse an existing complete structural or nested nominal type
without creating a new semantic type or inventing a work instance. Do not adopt
new nominal resources solely to compress text. Keep pinned domain-type selection
available and explicit. Managed editing, protected inspection and real-user
validation remain required; a transparent alias is not a domain ontology.

## Wording comparison — paper only

All snippets below are unimplemented alternatives, not accepted grammar, complete
processes, lowering evidence or final user-tested language:

```text
Describe type "Reservation instruction" as record with ["Warehouse" as text].
Keep "Instruction" as type "Reservation instruction", protected as "Order information".
```

```text
Call the structure record with ["Warehouse" as text] "Reservation instruction".
Keep "Instruction" as structure "Reservation instruction", protected as "Order information".
```

```text
Define "Reservation instruction" to mean record with ["Warehouse" as text].
Keep "Instruction" as "Reservation instruction", protected as "Order information".
```

The first retains a visible type cue at definition/use, and distinguishes the
existing `type "Order" from "Order types"` resource selection by its explicit
`from`. A local reference would still need exact scoped qualification. The second
uses familiar vocabulary but "structure" could misleadingly exclude primitive,
optional, list or nominal types. The third is shortest but can confuse type names
with data/expression captions. Recommend evaluating the first direction when the
production contract is frozen; this critique does not settle its tokens. Bracketed
record detail remains the current precise syntax, not ordinary unrestricted prose.

## Exact meaning and managed identity requirements

A transparent source name must resolve to a complete immutable description. It
must preserve every nested named-type resource ID/revision/member, wrap/unwrap
membership requirement, record field, choice/outcome tag, integer/decimal scale,
optional/list element and existing expression/type relationship. A shorthand cannot
change accepted values or turn distinct pinned types into structural equivalents.
Resolve resources and outcome references in the definition's scope, then recheck
use visibility; declaration order must not select meaning. Outcome tags can refer
to particular node identities, so "same text" is insufficient for hoisting.

A new source-only declaration kind must use explicit managed companion identity,
lexical ownership, duplicate/no-shadowing/kind/exact-qualification rules and
explicit rename/reorder/copy/tombstone operations. Validate all definitions,
including unused ones and forward references. Source/version selection and exact
closed schema/pins must stay explicit, preserving all old behavior. No text
substitution, production Python compiler, name-based ID recovery or broad editor /
IR-export claim is acceptable. The repeated version-feature lists in the frontend
are an implementation maintenance concern: a later refactor needs an explicit
version matrix and complete old-profile regression evidence, not implicit numeric
"newer means all features" admission.

## Bounded alias graph criteria

A production form should permit finite acyclic references only. Detect cycles
before expansion, validate unused definitions and compute depth/node/compact-byte
costs bottom-up with checked arithmetic. Charge every definition/cache and every
use against cumulative bounds **before** allocation/copy. A small DAG can encode
exponential structure, so checking source size, each alias separately or depth
alone is insufficient. Reusing an immutable description saves resolution, not the
cost of an expanded wire value or live-access evaluation.

Keep input/output 1 MiB, depth 64, tokens 131072, parser 1M, lowering 100k and semantic
shared bounds. A deep alias chain cannot evade the expanded type-depth limit, and
a wide shared graph cannot evade node/work/output limits. Refusal emits no partial
candidate. Preserve useful definition/use spans without echoing confidential
captions or literals. Existing source0.17/C12 behavior and quotas stay fixed here.

## Future validation and limits

Before implementation, publish/sign a separate grammar/profile/companion contract
and hashes. Require exact full native RP-01/RP-03 expanded-body/revision comparisons
on an explicit C12 baseline, real byte measurements after definition/use overhead,
format/reparse/integrity/annotations, lexical scopes, nested nominal pins/outcome
tags, missing/duplicate/wrong-kind/private references, unused invalid definitions,
forward/cyclic graphs, stale/retired IDs and cumulative expanded depth/work/bytes.
No target savings is claimed until that work runs. Current benchmark adoption /
archive needs its own signed task. No new inactive utility is needed merely to
repeat ADR-0083's established type-byte counts.

Full RP-08, reusable receipt/work/predicate formulations, protected accessible
inspection/editing, general safe export, text/visual parity and actual representative
users remain explicit **Phase 1** obligations. The aggregate executable-IR gate
and physical-alias/canonicalization/original-snapshot/revised-surviving-work proof
remain unresolved. Do not use type naming to imply concurrency, execution or phase
completion; no Phase 2 or new phase chat follows.
