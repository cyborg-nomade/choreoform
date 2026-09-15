<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0012: Bind names and typed interfaces explicitly across closed composition boundaries

**Status:** Accepted — working binding/type foundation<br>
**Date:** 2026-09-15<br>
**Decider:** Project Owner

## Context

The next Phase 1 deliverable defines names, types, scopes, imports, parameters,
and composition. ADR-0008 establishes typed scoped data and explicit effects;
ADR-0009 defines stable IDs, lexical visibility, child-scope ports and exact
immutable references. ADR-0011 provides only a bounded parser: neither parsing
nor hashing validates type compatibility or grants execution authority.

The missing foundations must let later contracts distinguish a payment receipt
from a same-shaped approval, locate a declaration without consulting runtime
state, and compose reusable work without acquiring the caller's authority by
accident. At the same time, they must leave room for near-plain-English names
and tool-managed identifiers. The project's approved direction is accessible
controlled English, not permanent JSON authoring.

## Decision criteria

1. Preserve stable identity, exact revision binding, and fail-closed admission.
2. Make data dependencies, authority requirements, and scope boundaries explicit.
3. Give finite, deterministic rules for resolution and interface checking.
4. Support reusable domain-neutral processes without hidden runtime code loading.
5. Keep authoring vocabulary separate from wire bookkeeping and defer untested
   language conveniences rather than making implicit compatibility promises.

## Decision

Adopt the [binding and type foundation](../bindings/README.md) as the working
semantic constraints for subsequent Phase 1 deliverables. It defines:

- three separate concepts: semantic declaration identity, source binding name,
  and non-binding presentation label;
- lexical resolution with explicit qualification, no overload resolution by
  expected type, and no implicit shadowing in the initial profile;
- structural equality for finite anonymous value shapes, nominal equality for
  explicitly named domain types, and no implicit coercions or width subtyping;
- typed value inputs/outputs, explicit complete argument maps, one-snapshot
  argument evaluation, and atomic output publication with no output aliasing;
- lexical access for existing local child scopes, but closed interfaces and no
  ambient captures for reusable imported process templates;
- exact immutable import closure supplied by the host, explicit exports,
  acyclic imports/composition, and separate authority checks; and
- explicit boundaries between static invalidity, uninitialized runtime data,
  optional values, faults, business outcomes and effect uncertainty.

This is a **definition deliverable**, not an implementation of executable type,
expression, or policy contracts. The proposed type constructors constrain those
contracts; they do not assign payload encodings or exact scalar operations.
The next deliverable must supply those semantics and worked execution cases.

Do not extend IR 0.1.0 or text profile 0.1.0 in this PR. Existing local references
and ports retain their accepted meaning. Source names, reusable imports, named
types and linked-artifact provenance require explicit versioned mappings before
implementation. No import is hidden in annotations, strings or opaque payloads
and then treated as executable. Unknown or unimplemented contracts still block
semantic admission. Existing frozen fixture revisions remain unchanged.

### Resolved review questions — approved 2026-09-15

1. **Names:** require explicit disambiguation rather than implicit shadowing.
   This catches accidental rebinding during edits, at the cost of more visible
   qualification. Phrase delimiters and Unicode equivalence belong to the
   authoring-language ADR, not this proposal.
2. **Types:** use anonymous structural shapes plus nominal domain types, with
   exact compatibility. This protects domain meaning without naming every
   temporary record. Explicit conversions must later be pure, typed and visible;
   a conversion is never a permission bypass.
3. **Parameters:** begin with required named value inputs and outputs; omit
   implicit defaults, user-defined generics, higher-order process parameters,
   recursive types and recursive calls. Built-in type constructors remain
   parameterized. Introduce richer forms only with a demonstrated use case and
   separately reviewed rules.
4. **Reuse:** use closed, pinned templates and explicit requirements. Local
   children retain lexical visibility; reusable modules cannot silently capture
   caller cells, actors or capabilities. Initial imports and composition are
   acyclic; repetition uses the accepted repeat/fan-out model.

The Project Owner approved these four decisions. Their acceptance does not
select final text/visual notation or claim the rules are implemented.

## Options considered

| Option | Advantages | Costs and risks | Recommendation |
| --- | --- | --- | --- |
| Explicit lexical bindings and closed pinned reuse | Stable reviewable references; finite resolution; no ambient authority | More explicit interface work; requires later linking/IR design | Adopt as working foundation |
| Dynamic lookup and implicit caller capture | Less parameter plumbing; context-sensitive reuse | Same template can change meaning with caller context; hidden dependencies and authority | Reject initially |
| All types structural with implicit width compatibility | Convenient interchange of similar records | Domain distinctions can disappear; policy compatibility still unresolved | Reject implicit compatibility; retain exact anonymous shapes |
| All types nominal | Simple separation of domain identities | Forces names and conversions onto routine intermediate shapes | Retain for named domain types, not every shape |
| Hybrid equality with explicit conversions | Small rules; preserves domain distinctions; supports anonymous values | Some integrations require adapters; aliases and nominal definitions must differ visibly | Recommend |
| General generics, subtyping, recursion and higher-order processes now | More reusable abstractions and expressive interfaces | More resolution, termination, variance, authority and cross-form obligations before evidence | Defer; reject use in the initial profile |

No alternative has been tested with representative users. The
[evaluation](../evaluation/0012-binding-type-foundations.md) uses worked examples
only; it does not claim comparative implementation performance or ease of use.

## Consequences

The executable-contract work gains a precise interface target and cannot infer
permission from type compatibility. Source tools will need a persistent mapping
from friendly names to IDs, explicit rename/copy behavior and qualified lookup.
An imported template can be inspected against its pinned closure, but linking,
provenance serialization and protected editor behavior remain substantial work.

Exact equality is intentionally conservative. It may require explicit record
construction or conversion where other languages permit automatic adaptation.
No-shadowing may burden large processes; a later usability study may justify a
controlled relaxation. No evidence here settles phrasing, completion behavior,
or non-programmer comprehension.

The current Rust probe remains partial and the parser continues to return
unvalidated candidates. No executable-IR gate or Phase 1 exit criterion closes.

## Confirmation

Review the [worked cases](../bindings/cases.md) against the numbered rules and
the existing IR compatibility table. Confirm resolution under renames, duplicate
names, wrong-kind targets, exact nominal types, complete port maps, forbidden
capture, output aliasing, pinned dependency upgrades and cancellation boundaries.

Later implementation must translate these judgments into positive/negative
conformance tests, fail-closed stable diagnostics and text/visual equivalent
operations. A code test passing today is not evidence those proposed rules are
implemented. The evaluation retains all forty corpus gaps and G1–G4 conditions.

## Acceptance and action items

The Project Owner approved ADR-0012 and PR #16 on 2026-09-15 with all four
recommended decisions: explicit disambiguation without implicit shadowing;
exact anonymous structural and nominal domain-type compatibility; required
named value parameters without implicit defaults or general generics; and
closed, pinned, acyclic reusable composition without ambient capture.

Merge of [PR #16](https://github.com/cyborg-nomade/choreoform/pull/16) makes this
working foundation effective. Approval covers the definition deliverable, not
an implemented module system, executable dialect, final authoring syntax or
closure of G1–G4. The immediate documentation and worked-evidence actions are
complete; implementation and versioned linkage remain explicitly deferred below.

1. [x] Resolve the four review questions and obtain approval.
2. [x] Mark the ADR Accepted, record approval and update index/Roadmap.
3. [x] Supply a precise paper specification, worked cases, compatibility map
   and explicitly limited evaluation.
4. [ ] In the separately authorized executable-contract deliverable, specify
   exact scalar values, operations, purity, dependencies and policy judgments.
5. [ ] Before module implementation, review versioned source/IR/linkage formats,
   provenance and diagnostics; do not repurpose IR 0.1.0 annotations.
6. [ ] Carry full benchmark, conformance, near-English and visual/accessibility
   obligations forward without treating this paper design as execution evidence.

Pause before starting the next deliverable.
