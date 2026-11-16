<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Core profile 0.3.0: finite provider metadata read closure

Contract identity: `urn:choreoform:contracts:core-profile:0.3.0`.
This separately pinned extension inherits core 0.2.0 at
`urn:choreoform:contracts:core-profile:0.2.0` /
`sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623`,
and its required core 0.1.0 base at
`sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58`.
All three exact artifacts must be explicitly available and verified. Existing
IR/semantics versions, payload fields, retry-clock, type/expression, protection,
identity and lifecycle rules remain unchanged except the policy reference/read
closure rule below. Old core records never silently select this rule.

## Two kinds of policy reference

Every policy reference still resolves to its expected kind and lexical context.
Unknown identities, fields, contracts, private targets and invalid envelopes refuse.
For a policy record explicitly selecting core 0.3, references derived from the
`resultProtection` envelope of a supported provider are **metadata edges**:

- the activity provider named by a capability policy;
- the clock-source provider named by its timeout or wait timer;
- the clock-source provider named by its explicit retry clock.

These edges include the envelope's protection policy, participant requirements,
and permitted capabilities' contract/effect/authority policies, with all their
transitive reads. They describe information and access dependencies; they are not
instructions to recursively evaluate, call or dispatch the recipient capability.
All other policy references remain **strong edges**. If the same target is
referenced in both ways, retain both edges. Predicates remain closed expression
ASTs; no callbacks, recursive predicate operations or executable imports are added.

Core 0.1/0.2 policy records retain their original treatment of every reference as
strong. Validate the complete strong graph as acyclic with at most 64 vertices on
any dependency path. Never remove a strong edge merely because metadata also
names its target.

## Least finite reads

For each policy p, let L(p) be its direct embedded expression reads plus explicit
timer/retry basis reads. Let D(p) contain all strong and metadata targets. Derive:

`R(p) = L(p) union union(R(q) for q in D(p))`.

Choose the **least** solution: initialize direct sets and propagate union until
no set gains a member. All reads are existing finite definition data identities.
Require stored `body.reads` to equal R(p), including every protected dependency;
missing or invented reads refuse. Node dependencies continue to include the
applicable union. This neither freezes mutable values nor authenticates facts.

An implementation may use strongly connected components and an acyclic component
graph to compute the same solution. A cyclic component (including a self edge)
is permitted only when **every** member policy selects core 0.3 and its internal
strong edges are acyclic. A cyclic component containing any legacy record refuses,
even if a new record contributes a metadata edge. The component graph also has
at most 64 vertices on a dependency path. Validate all declarations, including
ordinarily unreachable ones; no ordinary entry is inferred for a metadata cycle.

Core 0.1/0.2 retain their existing cycle refusal. Definitions without core 0.3
retain their existing derivation. There is no fixed-point interpretation of type,
expression, control-flow, scope-parent, fault-selection or actual policy execution
cycles. Existing separate cycle rules for those graphs remain in force.

## Bounds and authority

Retain 10,000-item and 1,000,000-total-work semantic bounds, charging vertices,
edges, component processing and set copying/propagation. Use iterative traversal
or a separately bounded stack; input graph depth cannot exhaust the native stack.
A smaller caller bound may refuse; exceeding a bound never truncates reads or
converts a cycle into permission. Immutable declaration closure may be cached
only for exact definition/resource revisions and lexical context. Predicate truth,
current roles, authority and live host facts must be checked freshly.

This contract enables static read derivation for a capability permitted to consume
its own provider's output or for mutually permitted recipients. Result labels
still govern every explicit and control-dependent transfer; empty or incompatible
recipient sets are not widened. An allowed recipient is no proof of current
authority, implemented provider support, authenticated observation, safe lifetime,
retention or settlement. Static reports remain non-executable; the full validator,
benchmark, protected editing and real host/user gates remain separate.
