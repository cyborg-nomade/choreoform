<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Pure-test wording and type-boundary critique

**Date:** 2026-11-13<br>
**Authority:** signed scope `76edc820cc7c002a6e6f913f153efed78d3e99de` after
PR #104 merge `48c4968323fdbf0ed5275e089d0c31b12678507f`, CI43452609800 /
automated COMMENT review5893724397 / fresh full audit

## Complete inactive wording alternatives

Candidate A:

```text
Define test "Same reference" with inputs ["Expected" as text, "Observed" as text].
It holds when ((the parameter "Expected") equals (the parameter "Observed")).
End test.
```

Complete use expression:

```text
test "Same reference" using ["Expected" is ("claim-v1"), "Observed" is ("claim-v1")]
```

Candidate B:

```text
Say "Same reference" using ["Expected" as text, "Observed" as text] means
((the parameter "Expected") equals (the parameter "Observed")). End definition.
```

Complete use expression:

```text
whether "Same reference" with ["Expected" is ("claim-v1"), "Observed" is ("claim-v1")]
```

Prefer A under owner delegation. Its declaration kind and Boolean purpose are
explicit, with stable sentence boundaries and familiar input labels. B is more
conversational, but “Say” can suggest output/action and “with” already has multiple
roles. This is a design judgment, not representative-user comprehension evidence.
These examples alone are not complete processes, companion packages or working
language. The separately published source19 contract selects exact grammar before
compiler implementation; old profiles remain unchanged.

## Why checking only expanded bodies is insufficient

A code-derived counterexample uses the existing C12 equality rule: a pure test
whose declared text input `x` appears only as `x equals x` has truth-value result.
Substituting a whole-number AST produces a well-typed integer equality, also a
truth value. Native checking of the expanded result alone therefore cannot enforce
its declared text-input contract. A record helper inspecting only one field can
likewise fail to enforce its complete declared input shape; nominal pins must not
be inferred from equal representations. These are type-rule deductions, not a
claimed new compiler or native helper experiment.

Require native exact argument-type assertions at every actual caller context, in
addition to native checking of every unused/expanded pure definition. Preserve
normal exact structural/nominal equality; introduce no width subtyping, implicit
cast or dynamic polymorphism. Reuse the SDK's actual expression inference and
ordinary policy-use contexts, including all uses of a reused policy. A missing /
unvisited/unproved context must refuse, never silently skip an assertion.

A coherent implementation may carry bounded source-only assertions on actual
expanded expression sites into native partial-static checking. Assertions can
only add rejection; the ordinary raw check with no assertions stays byte/diagnostic
compatible. Metadata is not wire IR, nominal membership, runtime permission or a
new call operator. Literal data/type descriptors are not expression sites.

For source19 packages containing test uses, successful lowering requires this
native partial-static source-contract preflight before a candidate is emitted.
That may expose an unrelated ordinary static defect earlier in this new profile;
it is explicit new staging, not a change to any older profile. No-call source19
packages keep inherited staging. Subsequent control/protection/invalidation /
concurrency/timing/lifetime/host gates remain separate; no full admission follows.

Every declared test input must actually occur in its body. This catches unused
binding mistakes and avoids silently discarding an argument through substitution.
Expansion is textual-AST meaning, not a strict/evaluate-once runtime function:
arguments occur where substituted; conditional/short-circuit behavior remains
exactly the expanded C12 AST. All argument syntax/types are checked statically.
No runtime calls, evaluation order or effectful parameter execution is introduced.

The implementation must prove argument mismatch even when the expanded Boolean
would otherwise type-check, including identical field subsets with different
complete types, different nominal pins and reused-policy contexts. Nested calls in unused definitions require the same exact argument proof in their
declared pure parameter contexts. Every call also carries a result-type assertion,
including zero-input calls, so unvisited calls cannot silently pass. Source-only
assertion storage/type caches and visited-site coverage must be cumulatively
bounded, immutable and private; no missing assertion may become success.

## Working native proof layer after publication

Exact language publication `5b1a8c57ccfa22cac9951830ecf443986db096a1` precedes
production changes. The new native extra-type API can only reject more: it checks
immutable expression sites against exact native types, charges shared metadata /
comparison budgets and requires all assertions visited. Addresses are scoped
internal node lookups, never dereferenced/persisted/exposed as identity. Expected
types may cache; actual types are checked again in every caller context. Literal
data cannot become a visited expression site. Errors use private assertion indices
instead of echoing caller-provided paths or values. Failed pure checks clear only
the scoped assertion state while retaining the cumulative registry budget.

Eleven native groups pass (`/private/tmp/predicate-native-assertions.log`): the
otherwise-valid text/integer equality counterexample, complete record and distinct
nominal mismatches, actual-context reuse, opaque/missing/overlapping/result sites,
metadata/depth/work bounds, complete current RP-03 with identical ordinary reports,
a genuine derived full RP-03 shared-policy context mismatch, ordinary expression
source diagnostics and unused-work placeholder rejection. The complete SDK suite
also passed before the last two context/diagnostic refinements
(`/private/tmp/predicate-native-suite.log`); final full checks remain necessary.

Inspection found the old raw checker deliberately uses an empty result type for
unused work policies. That placeholder must not supply a source-call proof. The
additional API rejects source assertions there while ordinary raw checking keeps
its previous behavior. Policy/expression/node diagnostic wrappers preserve the
new private assertion index and retain their exact old record paths otherwise.
The full raw benchmark files remain unchanged. Source19 compiler/parser/binding
registration, recipe expansion/cost/provenance/managed identity, complete variants,
format/oracles/full CI/review/audit/merge are **not complete** yet.
