<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0090: Typed source-only pure Boolean tests on unchanged C12 IR

**Status:** Accepted under Project Owner delegation; implemented and integrated in PR #105<br>
**Date:** 2026-11-13<br>
**Decider:** Codex under ADR-0018; no personal owner or independent review

## Decision and publication before code

Signed [scope90](../evaluation/0090-pure-predicate-recipes-plan.md) `76edc820`
follows PR #104 merge `48c4968323fdbf0ed5275e089d0c31b12678507f`, successful
final-head CI43452609800 at78ae1c84, automated COMMENT review5893724397 and
fresh full audit. ADR-0089 selects pure recipes before general IR calls/work
sharing; its55-case/two-shape experiment is not a new compiler or user evidence.

Publish/sign the exact [source0.19 grammar](../authoring/candidate-a-v0.19.ebnf),
[profile](../authoring/profile-v0.19.md) and [companion0.18 schema](../../schemas/authoring/bindings-0.18.schema.json)
before production implementation. They retain C12/IR0.4 and every older meaning /
contract/quota. Source19 is **not implemented yet** at publication; parser/schema
registration, compiler, native evidence and tests follow this checkpoint. No
current benchmark adoption, C13 source, new IR call/runtime or phase admission.

| Publication / inherited baseline | SHA256 of exact raw bytes |
| --- | --- |
| `docs/authoring/candidate-a-v0.19.ebnf` | `ac650a242ef264cf7f2bc9164fa5ca31febfa393d58db3a014fdb29bfcd43c5b` |
| `docs/authoring/profile-v0.19.md` | `acc19707055114b497dabc2658830c9fff51d522dedfa5882c76d7e7f6ce4955` |
| `schemas/authoring/bindings-0.18.schema.json` | `c37de0976883cecc3c178a823f960df97656438559dd2c8da819bdec3d513095` |
| `docs/authoring/candidate-a-v0.18.ebnf` | `37a3b2569f623b073499a3c8915bddb46b34f2c819c8463d15e3f383305cf01e` |
| `docs/authoring/profile-v0.18.md` | `b5134f67a63b4793611a07bb1941b778cdecb991903f0ccdfcb0a22365ed2808` |
| `schemas/authoring/bindings-0.17.schema.json` | `92feb8c2a81e38c13d143cf01d13b8d7ec5666eeb24a361ba3337cc1d49f574b` |

Select Candidate A from the [wording/type-boundary critique](../evaluation/0090-pure-predicate-design.md):
`Define test NAME with inputs ... . It holds when (expression). End test.` and
`test reference using ...`. A makes the Boolean purpose explicit; B's “Say” can
suggest output/action. This is a controlled-English design judgment under owner
delegation, not representative-user evidence or final general-language completion.

## Typed meaning and managed identities

Tests are pure truth-value definitions with explicit complete typed inputs.
Companion source-only kind `predicate` and symbol role `predicate-input` bind exact
stable declaration/input IDs, names and wires. Require all inputs used, exact
arity/unique labels, lexical kind/visibility/provenance, forward/unused/nested
validation, acyclic definitions, stable rename/reorder/fresh copy and tombstones.
No ordinary expression parameter/fanout rule is relaxed, no implicit argument /
context/capture/default/nominal membership and no work/storage/endpoint sharing.

Expand literal-opaque ASTs hygienically into the real caller. Preserve complete
13/14 balanced shapes, all nominal unwrap/payload/tag dispatch, exact complete
current C12 documents/revisions/wires, every semantic/occurrence/permission ID and
the legitimate external-region refusal in finite variants. Arguments occur where
the expanded AST places them, not strict/evaluate-once function-call semantics.
Type-check every declared input even in unused/nested definitions; no ambient
cells/facts/authority. Explicit static literal/type references keep exact normal
scope/pin provenance at definitions and uses.

## Native argument proof and new-profile staging

Checking the expanded Boolean alone is insufficient: a helper declared with text
input x and body `x equals x` would still expand to a valid integer equality when
called incorrectly. Likewise a field subset cannot prove a complete record or
nominal input contract. This is a deduction from existing C12 type rules, not a
claimed new helper implementation. Enforce exact native argument types at every
actual use context, including all contexts of reused policies and nested calls
inside unused definitions. Every call also has a result assertion, including
zero-input calls; missing/unvisited/unknown/non-expression sites refuse.

Use bounded source-only typing assertions outside wire IR, reusing native pure
expression inference and actual policy-use contexts. Assertions can only add
rejection to ordinary checking. A native entry with no assertions must preserve
existing reports/diagnostics/acceptance exactly. No synthetic context, guessed
provider result type, Python proof or embedded wire cast supplies this obligation.
Caches/metadata remain private, immutable and budgeted, and are not host authority.

Source19 packages **with test uses** require native partial-static source-contract
preflight before a candidate is emitted. This may expose an unrelated ordinary
static defect earlier; it is explicit new-profile staging. No-call source19 and
all older profiles retain inherited staging. The preflight does not imply complete
semantic/control/protection/concurrency/lifetime/host admission. Later views stay
separate and non-executable; ordinary raw C12 meaning is unchanged.

## Bounds, alternatives and evidence required

Preflight/cache/use/argument duplication and source-only typing metadata share
existing depth/node/work/compact-byte/output bounds. Charge all unused definitions
and repeated uses, combine alias/recipe/structure depth, and reserve costs before
cloning growth. Do not reset native budgets per definition. Avoid repeated deep
parent clones/quadratic scans; no quota increase or partial candidate. Private
failures identify actionable definition/input/use spans without source disclosure.

Rejected for this scope: new runtime/IR functions and hygienic whole-work templates,
which require broader accepted contracts; untyped substitution, which cannot honor
the explicit input contract; implicit caller-fact capture and generic/widened
nominal matching; bigger quotas as a readability substitute. Protected accessible
inspection/editing remains necessary alongside factoring, not an alternative proof.

Require actual native positive/negative typing/context/identity/purity/visibility /
nominal/format/integrity/budget tests, including otherwise-valid expanded bodies
with wrong declared input types. Produce finite whole RP-01/03 variants with exact
current C12 IR/revision/wire and actual full source measurements including all
definition/use costs. Do not invent prospective savings, native helper evidence,
actual users or a complete RP-08. Preserve every current/historical artifact.

Run required final-head native/Wasm/Python checks/CI, separately review completed
diff, audit all unresolved comments/CodeRabbit summaries and expected-head signed
merge before integration. All40 scenarios remain Partial, G1–G4 conditional and
parent46 draft. Full RP-08, managed protected accessible inspection/editing/export /
text-visual, actual users/hosts/browsers/independent and every complete semantic /
region/phase criterion remain open; no Phase2.

## Implemented evidence before integration

The [native implementation evidence](../evaluation/0090-pure-predicates.md) records
688 Rust tests plus6 compile-fail tests, required native/Wasm checks and85 Python
commands. The complete experimental RP-01/03 variants retain exact current C12
documents/revisions/wire at54172 /291309 source bytes. Current benchmark adoption
remains a separate archive-first scope. Initial quota failures led to bounded
declaration collection and compact cached-cost lookup; every actual descriptor
copy remains fully reserved and all published quotas remain unchanged. Final-head
CI, separate automated review, full comment audit and signed merge remain required.

PR #105 integrated as `63988a63dc826dff6776251dbe747d947e17b2aa` after successful
CI43476905305 atb7057abda1a472a3a80df413f7557ec4ddcf1760, separate automated
COMMENT review5895660433 and a fresh full-body/thread/head audit. CodeRabbit
skipped the non-default target and supplies no review evidence. All local/CI
checks pass; this closes only the contracted implementation slice.
