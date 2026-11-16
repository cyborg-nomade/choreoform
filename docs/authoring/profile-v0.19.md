<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.19.0

**Status:** Contract accepted under ADR-0090 / owner delegation; compiler support is not yet implemented.

Inherit the normative language/identity/visibility/quotas of [source0.18](profile-v0.18.md)
except the explicit additions/staging below. Header `Choreoform authoring "0.19.0".`
pairs only with companion0.18/profile0.19 and its [closed schema](../../schemas/authoring/bindings-0.18.schema.json).
The [complete grammar](candidate-a-v0.19.ebnf) is normative. Select unchanged
C12/IR0.4 and inherited exact resources. No source selection of raw-SDK-only C13.
All older profiles retain their exact syntax, staging, meanings and quotas.

## Explicit typed pure Boolean tests

```text
Define test "Same reference" with inputs ["Expected" as text, "Observed" as text].
It holds when ((the parameter "Expected") equals (the parameter "Observed")).
End test.
```

This is a complete declaration, not a complete process/companion or permission.
Use `test reference using arguments` as an expression, for example:

```text
test "Same reference" using ["Expected" is ("claim-v1"), "Observed" is ("claim-v1")]
```

Inputs/arguments use `empty` or explicit bracketed named lists. A test returns only
truth value. Every input has an explicit complete type and must occur in the body;
unused inputs refuse. Input names/wires are unique within the test. All uses supply
exactly one argument for each input, with no duplicate, missing, extra, positional,
inferred or default argument. Argument order does not choose identity or meaning.
There are no overloads, type variables or runtime/general IR calls.

Bodies are pure and closed over their typed parameters and explicit static literals /
types with normal lexical provenance. Ambient cell reads, policy facts, actor /
authority/storage captures and implicit imports refuse even in unused definitions.
Explicit pinned types/resources and static outcome/purpose literals retain normal
kind/visibility rules; they do not import runtime values. Callers supply ordinary
existing expressions in their actual contexts. Nominal unwrap/payload/tag dispatch
stays explicit; equal representations never confer another nominal membership.

## Managed identity and complete preflight

The companion adds source-only declaration kind `predicate`. Each input has one
source-only symbol role `predicate-input`, owned by that predicate, with explicit
stable ID, display name and wire. The source input/argument names resolve through
that strict ownership/caption map; body parameters bind to those stable wires.
No source-only definition/input becomes a semantic expression/call/parameter record.
Existing ordinary expression parameters and fanout-item binding retain their meaning.

Collect all definitions before uses. Enforce inherited exact kind, qualification,
lexical visibility/no shadowing, stale/surplus/retired inventory rules. Every unused,
forward and nested definition must be typed; cycles refuse. Resource/outcome/purpose
references resolve in the definition's lexical scope and remain exact managed
provenance checked again at uses. Hoisting a test cannot expose a private sibling
resource/node/data binding. Managed rename/reorder preserves IDs/wires; copying
requires fresh source-only IDs, deletion retains tombstones. No name/position ID
recovery. Format/reparse preserves identities, annotations and package integrity.

Validate every expanded pure definition (including native exact argument assertions
for nested calls) with native semantic types, empty ambient
cells/facts and its exact declared parameter context. Require truth-value result,
complete exact structural/nominal input descriptors and every input used. Neither
Python measurements nor inferred runtime values supply this proof.

## Exact substitution and native caller typing

Substitute ASTs hygienically, never strings. Literal payloads/type descriptors and
field/payload labels are opaque. A substituted argument is not rebound recursively
as if it belonged to the callee. Nested calls must preserve caller parameter wires.
Arguments occur where expanded: this is not strict/evaluate-once runtime function
semantics. Existing conditional/short-circuit/fault behavior is that of the exact
expanded C12 AST. No new evaluation, dispatch or authorization happens.

Native exact argument types must be checked at **every actual caller context**,
including all uses of a reused policy. Expanded-body typing alone is insufficient:
a text-typed `x equals x` would otherwise accept an integer substitution. No width
subtyping, nominal pin relaxation or implicit cast. Every call has a native truth-value result assertion as well as its argument
assertions, including zero-input calls; all sites must be visited in a real context.
Source-only assertions may add rejection to native checking but cannot weaken raw checks or change wire IR.
A missing/unvisited/non-expression/unknown caller context refuses; literal data is
never an expression site. Bound all assertion/type-cache/coverage metadata as well
as the expanded AST. Actual reads/dependencies/invalidation/protection derive from
the expanded caller and undergo the existing native checks.

For packages **with test uses**, lowering requires native partial-static checking
of the expanded candidate plus all source-only argument assertions before emitting
it. An unrelated ordinary static defect can therefore surface earlier in this new
profile. With no uses, inherited candidate staging remains. Older profiles are
unchanged. This partial prerequisite proves no control/protection/concurrency /
lifetime/host sufficiency, full definition admission or executable permission.
Subsequent checking/advisory views retain their existing non-executable meaning.

## Shared bounds, diagnostics and required evidence

Precompute/check DAG, structure and argument-duplication costs before allocating or
cloning expanded values. Charge unused definitions, cached types/templates, every
use/duplicated argument and source-only typing metadata cumulatively to existing
shared expansion work/bytes/depth/node/output limits. Combined alias/recipe/structure
depth cannot multiply separately reset limits. Native definition typing shares its
budget across definitions, never one fresh budget per definition. Reuse immutable
cached descriptors; avoid deep parent copies/quadratic scans. No live value cache.

Retain source/companion1MiB, depth64, tokens131072, parser1M, lowering100k, cumulative
expansion1MiB and existing semantic/wire limits. No quota increases or partial /
truncated success. Diagnostics locate meaningful definition/input/use UTF-8 spans
without echoing confidential source/companion data. Unknown/future versions refuse.

Require native positive/negative whole cases for unused/forward/cyclic typing,
arity/names/result/nominal/complete input mismatches (including otherwise-valid
expanded bodies), caller-context coverage, pure closure, literal opacity, managed
rename/reorder/copy/tombstones, lexical/resource provenance, format/reparse/integrity,
combined depth/exponential/duplicated argument/work/output boundaries and older
profiles. Actual finite RP-01/03 variants must retain complete current C12 IR /
revision/wire, every semantic/work/storage/capability/endpoint/capture/compensation
identity and legitimate concurrency refusal. Preserve complete13/14 balanced shapes.
Measure real whole source/definition/use overhead; no predicted savings or user study.

Actual current benchmark adoption, general managed protected accessible inspection /
editing/export/text-visual, full RP-08, representative users/hosts/browsers /
independent evidence and all semantic/external-region/G1–G4/phase gates remain open.
All scenarios Partial, parent46 draft; no execution, phase completion or Phase2.
