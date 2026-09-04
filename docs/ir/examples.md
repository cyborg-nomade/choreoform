<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Worked wire examples and verification

These are **non-executable benchmark excerpts**, not complete implementations
of the corpus. Their policies and types use an illustrative dialect pinned to
an [immutable snapshot of this document](contracts/README.md). The checker
recognizes its serialization but does not implement its meaning. No tool may
plan or execute these examples just because their JSON Schema and digest checks
pass.

## Illustrative dialect contract and limitations

The dialect ID is `urn:choreoform:examples:illustrative-dialect`. A type payload
`{name: ...}` names a domain type without defining its value schema. Expression
payloads have `form: read` plus a data reference, or `form: symbol` plus a name
for a pure rule still to be specified. Policy payloads contain a `rule` name
and a `case` corpus identifier. The names describe the intent below; they are
not code, a policy language, or a standards-compliant provider contract.

Required follow-up: replace this illustrative contract with accepted type,
expression, and policy dialects, validate full payload semantics, complete the
process paths and invariants, and run them through both frontends and engines.

## RP-01: Review, payment, unknown outcome

[Expense excerpt](../../examples/ir/01-reimbursement.json) retains the identities
of the expense declaration, manager review, payment capability, and late-result
wait. The approved review flows to payment; rejection flows to an explicit
rejected finish. A payment with unknown outcome waits for reconciliation rather
than issuing a second logical payment. The expense declaration invalidates
review sufficiency after a material revision.

Policy intentions: `authority-at-acceptance` rechecks actor authority;
`approved-revision-payment` binds the effect to the approved expense snapshot;
`reconcile-existing-payment` cannot manufacture success; `withdrawal-policy`
prevents payment after effective withdrawal. Missing: validation and finance
review, full correction/withdrawal flow, real payment contract, timer and
negative paths. This is not evidence that RP-01-A through D execute correctly.

## RP-03: Parallel acceptance

[Order excerpt](../../examples/ir/03-order.json) explicitly forks inventory and
payment-authorization scope templates and pairs them with a join over both
identified outcomes. There is no ordering hidden in the map of child branches.
`settle-remaining-work` must preserve void/expiry obligations if a sibling fails;
`reconcile-effects` must deduplicate provider confirmations and forbid unsafe
retries. Missing: shipments, capture bounds, full cancellation/compensation
handlers, failure branches, and provider contracts. It does not model the full
order or establish any stock or payment guarantee.

## RP-08: Keyed scope growth and protected action

[Incident excerpt](../../examples/ir/08-incident.json) fans out an asset
collection into a declared response template. The stable-key rule and explicit
seal represent identity-preserving growth followed by fixed join membership.
A human authorization precedes an isolation capability, while the capability
retains its own authority requirement. Restricted data retains sensitivity,
purpose, actor/capability access sets, and policy reference in the semantic body.

Policy intentions: `keyed-add-retain-settle` must retain existing occurrences,
settle removed work, and reject duplicate keys; `ordinary-closure-reconcile`
retains owned reconciliation subscriptions rather than reopening terminal
instances; `explicit-consequential-race` prevents incidental scheduling from
authorizing dangerous effects. Missing: actual policies, evidence verification,
all incident objectives, recovery, transfers, provenance import, and execution.

## Frozen perturbations: expected results

| Change or input | Expected result and evidence boundary |
| --- | --- |
| Reverse declaration-map insertion order | Same canonical bytes and revision |
| Reformat JSON or alter annotations only | Same semantic revision |
| Change sensitivity, policy, guard payload, or ID | Different semantic revision |
| Reverse a dialect argument array | Different semantic revision; array order is significant |
| Unknown format version/core field | Refuse supported structural admission |
| Missing/undeclared dialect | Reference failure; a declared but unimplemented dialect remains inert |
| Duplicate key, non-integer token, invalid Unicode, corrupt digest | Reject without repairing or guessing |
| Dangling flow, wrong-kind target, duplicated cross-map ID, scope cycle | Reference/scope failure |
| Source/target crosses scope without explicit invocation | Refuse linking |
| Join references nonreciprocal source or uses negation | Refuse linking/shape |
| Add a runtime asset | Same definition template; occurrence behavior still requires engine tests |
| Late result after ordinary closure | Policy slot retained; post-closure transition behavior untested |
| Concurrent cancellation or ambiguous external effect | Explicit policy references survive serialization; race/effect safety untested |

## Reproducing the checks

Run from the repository root with `uv` available:

```sh
uv run tools/check_ir_fixtures.py
```

The script pins its two verification dependencies in inline metadata. It is a
disposable specification-evidence harness, not a choice of language, package
manager, runtime, or production validator for Choreoform. It performs no network
resolution of schema IDs, semantic references, or dialects. Dependency
installation is the only network need; after caching, `uv run --offline` works.

The script checks the schema itself, the three fixtures, exact pinned local
contract identities and snapshot digests, selected linking rules,
canonicalization invariants, malformed JSON, and negative mutations. Its
`--revisions` option prints recalculated fixture digests for review; it never
rewrites fixtures. It does not implement
semantic-contract validation, execution, full control-flow analysis, performance
limits, or text/visual round trips.
