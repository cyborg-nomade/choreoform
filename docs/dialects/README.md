<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Initial executable-contract proposal

**Status:** Proposed under [ADR-0013](../decisions/0013-executable-dialect-contracts.md).
No artifact in this directory is an accepted immutable contract yet.

“Executable” here means a specified value judgment, evaluation rule or state
transition with checkable positive/negative cases. It does not mean today's
parser/probe can admit these definitions or that a process engine exists.

Read [types and expressions](values-expressions.md), [policies](policies.md),
[worked cases](cases.md) and the [evidence limitations](../evaluation/0013-executable-contracts.md).
The proposed first profile is deliberately restrictive. Unsupported forms fail
closed; they do not become host-language callbacks or quietly defaulted policies.
The JSON shapes below are interchange notation, **not the near-English authoring
language**. Final authoring and visual representations remain separate decisions.

## Contract identity and publication

The proposal is one coordinated suite containing three dialect roles: types,
expressions, and policies. Proposed artifact identity:
`urn:choreoform:contracts:core-profile:0.1.0`. The role is selected by the IR
position and the payload discriminator, not by a suggestive local dialect alias.
Each local alias in `body.dialects` still binds an exact `{id, revision}`. A suite
binding may serve all three roles; mismatched/unsupported role usage fails.

Before acceptance is made effective, publish a content-addressed UTF-8 contract
snapshot containing this README, values-expressions.md and policies.md in that
order, including their full file bytes. The snapshot is the concatenation of,
for each file: its repository-relative path in UTF-8, LF, its byte length as
unsigned decimal ASCII without leading zeros, LF, and its exact bytes. No extra
separator follows the bytes. The byte count disambiguates file boundaries.
Its revision is `sha256:` plus lowercase SHA-256 of the **entire snapshot**.
No digest inside those files binds the snapshot to itself. Cases, ADR, evaluation
and probe code are evidence, not part of the contract hash domain.

Freeze that accepted snapshot, add it to the explicit local artifact registry,
and retain it unchanged thereafter. Subsequent editorial source edits do not
rebind definitions. New semantic rules need a new contract version and snapshot;
new whole-file bytes need a new digest. In this proposal PR the files remain
editable and no final digest is promised. No reader may accept the proposed URN
without the exact approved snapshot and implemented contract support.

This does not replace or reinterpret the existing illustrative dialect artifact.
Old examples remain structural excerpts. The new profile fits existing dialect
slots, not new core fields/kinds; any discovered change to core IR or ADR-0008
meaning requires its own explicit revised version/binding before use. Runtime
context records described here are abstract semantic inputs, not a Phase 2
persistence schema. Module/linkage formats from ADR-0012 are still not defined.

## Admission pipeline and common rules

1. Strict transport/envelope/revision admission remains ADR-0009's responsibility.
2. Verify all exact contract identities/digests against host-supplied resources;
   load no network content or executable code from an identifier.
3. Decode the closed payload schemas, resolve all types/references and reject
   cycles, unknown fields/kinds/operations or unsupported contracts.
4. Type-check expressions and derive dependencies across every branch. Validate
   node interfaces, policy placement, scope visibility and output guarantees.
5. Validate protection, authority, capability and host-context obligations. If
   the target cannot implement a required judgment, refuse planning/execution.
6. Only the later complete validator/planner may grant semantic admission. The
   evidence probe and current parser have no such authority.

Schema notation: `{a:T,b:U}` lists **all required fields**, with no extras;
`Map<K,V>` is an unordered JSON object; `Set<K>` maps each member to literal
`true`; `List<T>` is an ordered JSON array. Empty maps/sets are explicit. `ID`
is an existing valid local declaration token; `Name` is a nonempty exact Unicode
string; `Ref` is an immutable `{id,revision}` checked under its supported artifact
format. All JSON strings obey strict transport; no case-folding or normalization.
Nullable fields are marked `null | T`. No other implicit null/default exists.

Every dialect payload is an object. Its root descriptor/AST/policy is the `body`
inside the existing dialect wrapper or record; do not duplicate the IR envelope.
Type descriptor references to local dialect aliases are resolved before equality.
The illustrative profile's string conventions do not become new-profile types.

## Integration with existing IR positions

| IR position | Required new-profile contract judgment |
| --- | --- |
| data.type, expression.resultType/parameters, capability.input/output | Valid type descriptor; exact equality, finite nominal expansion |
| expression.body | Pure AST; result matches declared type; inferred reads equal expression.reads; parameters match existing permitted binding sites |
| actor.requirement | `actor` policy; host supplies authenticated facts, never a hard-coded live account binding |
| data.protection.policy | `protection` policy, in addition to all existing envelope constraints |
| capability.contract | `capability` policy pinning a separate supported provider contract |
| capability.effects | `effect` policy compatible with that provider contract |
| activity.policy | `work` policy; type/actor/effect obligations depend on activity mode |
| scope.cancellation/faults/closure/race | Corresponding `cancel`, `faults`, `closure`, `race` variants |
| join.remaining | `settlement` policy over the source's unfinished children |
| wait.policy | `wait` policy with typed observation matcher and/or timer |
| fanout.changes | `membership` policy; item key and seal retain existing expression restrictions |

Policy-to-policy references have expected kinds and ordinary lexical visibility.
Their graph is acyclic; policies cannot invoke each other recursively. Each node's
reads must include all transitive policy/expression reads. Conservative additional
node reads remain permitted by IR; they impose the same availability/protection
obligations. References hidden in unsupported payloads cannot be treated as empty.

Runtime host facts must be authenticated, revision-bound, typed and checked at
the specified acceptance point. A parser cannot manufacture them, and the host
cannot waive a false policy. Every result is committed with the input revision
set, actor/authority, occurrence identities and accepted-event position required
by ADR-0008. Pure checks read a single frozen context; the runtime reevaluates
after conflicts rather than committing decisions against stale context.

## Failure classes and bounds

Static invalidity blocks planning: shape/type/reference/contract/dependency or
policy inconsistency. Evaluation faults include unavailable data, overflow,
invalid access, invalid nominal value and policy predicate evaluation failure.
Policy denial produces no protected operation. Waiting/unknown-effect/pending
settlement are explicit nonterminal states, not faults or business outcomes.
Infrastructure inability to preserve the contract is an engine fault; do not
invent a business result or claim an external effect was undone.

Initial admission limits: existing 1 MiB wire and depth 64; at most 10,000 AST
nodes per expression (including all branches); at most 64 nested type/AST/policy
reference levels; at most 10,000 elements in a constructed collection or interval
table. Reject before recursive evaluation exceeds these limits. Host budgets may
be stricter and must produce explicit resource refusal, never silent truncation.
Budgets do not change the population of a fan-out or settle remaining work.
Resource limits are not claims of production performance or memory bounds.
