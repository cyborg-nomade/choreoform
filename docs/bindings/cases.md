<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Worked binding and interface judgments

**Status:** Paper evidence for proposed [ADR-0012](../decisions/0012-binding-type-foundations.md).
These are reasoned judgments against [the numbered rules](README.md), not
executed tests, valid source files, complete processes, or executable contracts.
Names, type abbreviations and arrows below are explanatory notation only.

Assume each symbolic contract revision K is pinned to verified supported bytes
in the supplied registry. `K.Amount`, `K.Approval`, `K.Receipt`, `K.Order` and
`K.Asset` are **distinct nominal types**; their concrete value contracts remain
the next deliverable. No assertion here establishes domain correctness. Named
examples use synthetic data and never grant real operational authority.

## RP-01: Reimbursement — value, evidence and permission are different

Suppose a local review child has input `claim: K.Claim`, output
`decision: K.Approval`, and outcomes `approved` / `declined`. Its approved and
declined paths must both initialize the declared output; the later Approval
contract must be able to represent the relevant decision evidence. Alternatively
the author explicitly chooses an optional output type and publishes absence;
omitting a declared output is not a third interpretation.

The caller passes an expression reading claim revision c7, and receives the
result in its `approval` cell. All inputs are captured together. A correction
creates c8 but does not alter the transferred c7 or rewrite the approval's
provenance. An invalidation policy determines whether that prior approval
remains sufficient; binding alone does not decide it.

The subsequent payment capability requires its declared input type, actor
authority and effect policy. An approval record is not a payment receipt even
if both contain similarly shaped fields. Relabeling `approval` as “payment
receipt” cannot satisfy the receipt type. Nor may an explicitly converted value
bypass payment permission or invent a confirmed effect.

| Case | Change | Judgment | Rule |
| --- | --- | --- | --- |
| B01 | Rename source `claim` to `expense claim`, retain ID and rewrite references | Same resolved declaration; semantic body may be unchanged | N1–N3 |
| B02 | Relabel approval as “receipt”; connect `K.Approval` to `K.Receipt` target | Reject unequal nominal identities | T2 |
| B03 | Supply exactly `claim` with result type `K.Claim` | Interface type check succeeds, subject to protection/value checks | P2, T3 |
| B04 | Omit claim or add undeclared `manager` input | Reject argument-key mismatch, even if a caller variable has that name | P1–P2 |
| B05 | Declined path finishes without initializing `decision` | Reject when statically provable; otherwise fault before ordinary output publication | T4, P3 |
| B06 | Import reusable review that reads an unbound caller `bank account` | Reject ambient capture; declare a permitted input/requirement instead | S3 |
| B07 | Claim changes from c7 to c8 during review | Existing transferred value stays c7; sufficiency needs explicit invalidation contract | S2 |
| B08 | Withdrawal races with completion | Binding/type checks do not select a winner; accepted race/cancellation policy governs | P3, ADR-0008 |

For an ordinary **local**, non-exported child, reading an ancestor claim can be
legal instead of using a port, provided visibility, dependencies and protection
are satisfied. B06 rejects capture specifically across the reusable boundary.
No payment/reconciliation/withdrawal trace is implemented by these examples.

## RP-03: Order fulfillment — independent children, atomic interface writes

Two local children perform stock and payment work. Their locals may both use
the name `result` because they are siblings with separate lexical tables. A
parent cannot read either private result directly by guessing a qualified path;
exposure requires an interface (or an explicitly declared writable ancestor cell
under existing lexical semantics). A completed stock reservation is not a
completed payment authorization.

Consider a reusable reservation template returning two outputs,
`reservation: K.Reservation` and `status: K.ReservationStatus`. The caller supplies
distinct compatible targets. After an ordinary outcome, the invocation validates
both available values and transfer permissions, then publishes both together.
If a transfer fails, neither ordinary output is published. This does not undo a
provider reservation already made inside the child; reconciliation remains owed.

| Case | Change | Judgment | Rule |
| --- | --- | --- | --- |
| B09 | Same binding `result` in independent sibling scopes | Allowed; no collision in either visible ancestor chain | N2 |
| B10 | Child declares `order` when an ancestor already binds `order` | Reject implicit shadowing; choose another binding | N2 |
| B11 | Parent reads `stock.result` directly | Reject inaccessible child data despite explicit qualification | N2, S1 |
| B12 | Actual record adds an extra field to expected anonymous shape | Reject width mismatch; construct exact shape explicitly | T1–T2 |
| B13 | Two output ports of the same type target one caller cell | Reject aliasing even when values happen to be equal | P3 |
| B14 | Input reads cell X and single output targets X | Allowed if type/protection checks pass; snapshot input then new output revision | P2–P3 |
| B15 | Child is cancelled after provider request | No ordinary output bundle; retain effects and settlement obligations | P3, M3 |
| B16 | Same-shaped payment type comes from a different contract revision | Reject implicit compatibility; explicit reviewed conversion if provided | T2 |

Existing IR split bodies have no ports. The interface illustration uses invoke,
not invented split-output fields. A later authoring form cannot hide this
restriction by drawing output connectors that current IR cannot represent.
Stock/payment join predicates and compensation still need executable policies.

## RP-08: Incident response — closed reusable response, stable dynamic items

Suppose an incident fan-out binds each asset to its child item cell. The key
expression has exactly the accepted `item` parameter and no process-data reads;
its immutable input is the same value used to initialize the child. Changing a
display label cannot change that stable item key. An authorized new asset adds
a child under the declared change policy; removal cannot erase an existing one.

A reusable containment process takes an asset value and declares a capability
requirement for isolation. A passed host name is data, not authorization to
contact it. The caller's more powerful adapter does not automatically satisfy
the module's least-authority policy. Until the required policy contract and
planner checks exist, no containment execution can be admitted.

| Case | Change | Judgment | Rule |
| --- | --- | --- | --- |
| B17 | Bind every required asset argument at exactly `K.Asset` | Interface check succeeds, not authority admission | P2, P4 |
| B18 | Imported template relies on caller's undeclared isolation adapter | Reject ambient capture/authority; bind declared requirements explicitly later | S3, P4 |
| B19 | Root has no asset value but caller substitutes `None` automatically | Reject; uninitialized is not initialized `Option<K.Asset>` absence | T4 |
| B20 | Definition pins module r1; registry also contains r2 | Resolve r1 only; missing r1 is an error, never fallback | M1, M4 |
| B21 | Qualified type refers to private module declaration | Reject non-exported target | M2 |
| B22 | Imported module A depends on B, which depends on A | Reject cycle with dependency path | M2 |
| B23 | Reusable process calls itself or mutually recurses | Reject composition cycle; repeat/fan-out are separate constructs | M3 |
| B24 | Two aliases reach identical contract revision and type ID | Same nominal type identity, not a duplicate new type | M2, T2 |

These cases do not implement late-event reconciliation, offline evidence import,
isolation authority or dynamic membership. They ensure the foundation cannot
silently reinterpret those obligations as name resolution.

## Cross-cutting counterexamples

| Case | Inputs | Judgment | Rule |
| --- | --- | --- | --- |
| B25 | A data reference resolves to an actor with the requested spelling | Reject wrong kind; do not search elsewhere | N2 |
| B26 | `Record{a: Text, b: Boolean}` vs same fields in reverse order | Equal if primitive contract identities match | T2 |
| B27 | `List<Text>` vs `Option<Text>` | Unequal constructors | T2 |
| B28 | Named domain type versus its anonymous underlying shape | Unequal; a transparent alias would instead expand | T1–T2 |
| B29 | Composed versus decomposed Unicode spelling in a binding | Distinct names; no implicit normalization; frontend should expose potential confusion | N2 |
| B30 | Registry entry has correct ID but wrong digest or kind | Reject before using declarations | M1 |
| B31 | Direct module exists but its transitive dependency is absent | Reject closure; do not fetch from a URI | M1 |
| B32 | Alias A refers to itself as a type, or a record recursively contains itself | Reject cyclic descriptor/alias | T1 |
| B33 | Present `Option<T>` value is explicitly absent, on a permitted input | Initialized typed value; not a missing argument | T4, P1 |
| B34 | Same label, different stable declaration ID | Different identity; never merge based on display text | N1 |
| B35 | Imported type revision changes while an instance runs | Existing binding unchanged; upgrade requires new validated definition, not instance migration | M4 |
| B36 | Known type, but an unenforceable protection policy on its transfer | Refuse admission/transfer; type success cannot waive policy | T2, P3 |

## Perturbation accounting and future test obligations

Late/duplicate observations, partial failure and concurrent cancellation retain
their accepted policies and identity boundaries; no name or type rule chooses
their operational result. Changed evidence retains exact revisions (B07).
Long-running upgrades retain pins (B20/B35). Large fan-out shares one template
but creates separate occurrences (S2), with explicit resource refusal allowed;
no scale result is claimed. Complete RP-01/RP-03/RP-08 processes remain missing.

When implementing, turn each B01–B36 judgment into fixtures with actual supported
contracts, source/IR mappings and expected stable diagnostics. Add adversarial
dependency graphs, deterministic name resolution under declaration reordering,
output-publication fault injection and protected diagnostic tests. Passing
today's parser/IR regressions cannot count as executing these proposed cases.
