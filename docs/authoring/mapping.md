<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Lowering, identity and protected meaning

**Status:** proposed obligations, not an implemented encoder. Both candidates
must produce the same accepted [IR](../ir/definition-v0.1.md) and
[dialect contracts](../dialects/README.md). No new node, policy kind or implicit
host operation is authorized by these English phrases.

## 1. Artifact boundaries

Propose an authored text plus a **tool-managed, portable binding companion**.
The companion travels in the same source-control change/export package; it is
not disposable editor cache. Authors use human names. Tools allocate opaque IDs,
pin supported resources and compute revisions. Both files are accessible plain
text; the UI may fold identity details but must provide keyboard/text access.
No hidden server database may be the only surviving identity record.

The companion must record exact profile version, definition identity, declaration
kind/lexical scope/binding name to ID associations, generated node/flow/policy IDs,
retired IDs, exact supported resource identities/digests, and retained unknown
annotations. A derived source map binds both exact authored-source bytes and
the resulting semantic revision; it is distinct from the persistent identity
ledger. Changing source invalidates old spans, not the declaration identities.

Source outcome/branch/port names also need explicit stable symbol bindings.
For example, the readable outcome `"correction needed"` is an alias for a valid
IR token such as `correction_needed`, not that whitespace-containing wire token.
The compiler must use the same resolved token in the node outcome, flow and
human-result variant tag. Allocate/persist local symbols once; do not derive them
afresh by replacing spaces. Imported provider outcomes are fixed contract tokens:
a source alias cannot rename or invent them. Refuse an incompatible provider
outcome instead of modifying its contract. The missing companion grammar must
cover these associations as well as declaration IDs.

The companion's versioned schema, canonicalization and package-integrity domain
are **not specified by ADR-0012 or this paper sketch**. They must be reviewed and
tested before a compiler relies on them. This prevents claiming that this PR
already solves source/IR round trips. Pins influence semantics and go into the
IR body; name recovery and display data alone cannot substitute for semantic
references. Raw IDs/digests need not be manually maintained by ordinary authors.

Safe first creation is an explicit operation allocating a new definition lineage
and all IDs. It is not a fallback for a missing companion on an existing process.
Imported IR already has IDs: preserve them, never regenerate by traversal order,
name hash, source position or line number. Source-only sharing is an incomplete
editing package, not a portable identity-preserving export.

| Edit | Identity/revision rule |
| --- | --- |
| Whitespace, reorder unrelated declarations, comment | Preserve resolved body/IDs and semantic revision; recompute source digest/spans |
| Rename a binding | Transactionally update its ledger entry and all resolved references; preserve ID; same resolved body is possible |
| Raw name change in a text editor | Show unresolved/stale binding; an explicit reconciliation/rename operation must choose identity; no fuzzy rebinding |
| Change a display title | Presentation only; title cannot carry required instructions or affect lookup |
| Change work instructions, guard, authority, deadline or dependency pin | New semantic revision, even if the displayed process looks similar |
| Copy a step or scope | Fresh IDs for copy and all generated internals; remap internal references; references outside copy remain explicit |
| Delete then create unrelated work with the old name | Retired IDs remain unavailable; fresh identity required |
| Move between scopes | May retain identity, but ownership/visibility changed: revalidate and compute new semantic revision |
| Merge branches | Detect ID collision, incompatible renames, stale pins and conflicting semantic edits; do not resolve by file order |
| Upgrade a definition used by a live instance | Create a new revision; the instance keeps its exact definition/plan; no migration |

Consequently, a plain editor remains a supported editing path, but identity-aware
rename needs a CLI/tool transaction, not a guessed textual search-and-replace.
An alternative is visible inline stable-ID anchors. It simplifies copyable
single-file identity but adds punctuation to every declaration. The user-study
protocol compares this packaging burden separately from A/B control wording.

## 2. Pipeline and complete field obligations

Read bounded bytes → parse exact grammar → resolve names using the ledger and
closed resource registry → construct typed source model → lower explicit records
and derived dependencies → compute semantic revision → full validation and
revision verification → admission.
Diagnostics may stop any stage. There is no fallback to a JSON escape block,
host callback, internet lookup or model interpretation. A syntax tree is not
an admitted definition. Every reference has a source span and expected kind.

| IR field family | Required source contribution / derived fact |
| --- | --- |
| body.id, root, semantics, dialects | Durable identity and explicit supported pins from companion; root from process declaration |
| All eight maps | Emit scopes/data/expressions/actors/capabilities/policies/nodes/flows, including empty maps |
| scope parent/entry/ports/outcomes | Explicit declarations, start clause and exact typed port bindings |
| scope cancellation/faults/closure/race | All four named policies; never engine defaults |
| data scope/type/protection/invalidates/initial | Owner and exact type; full envelope; reviewed invalidation set; initializer or absent |
| expression scope/dialect/resultType/parameters/reads/body | Exact AST and types; all-branch read closure; only accepted binding sites |
| actor scope/requirement | Named actor policy, not a principal embedded in source |
| capability scope/contract/input/output/authority/effects | Explicit provider and policies; supported resource/type equality verified |
| policy scope/dialect/body | Closed variant with every required field; transitive read closure derived |
| node common fields | Owning scope and kind; complete ordinary outcomes, transitive reads and actual writes |
| node-specific fields | Every slot of the ten-kind table in [syntax](syntax.md#3-candidate-a-structured-steps); no inferred behavior from layout |
| flows source/outcome/target | Outcome sentences become exact explicit references with durable generated flow IDs |
| annotations | Preserve unknown data; explicitly non-semantic names/titles/layout; protected handling still required |

Compiler-generated expression, flow and policy records need persistent identities
too. Tying them to `(parent identity, semantic slot)` is an allocation lookup,
not recomputing an ID from changing content; map-key renames need explicit
identity-preserving operations. Retired records are not reused for unrelated
meaning. Lowering a node move does not grant access through lexical boundaries.

No “sensible defaults” for authority or settlement. Authoring tools may offer
visible clause templates; choosing one inserts the complete reviewed values.
Purely mechanical empty maps, exact references, dependencies and the IR envelope
may be generated. Extra existing conservative node reads cannot be silently
discarded during import/export: they affect availability and protection. The
future companion/source form must retain and expose these explicit dependencies.

## 3. Safety distinctions that must survive reading and editing

- A human outcome comes from a typed, attributable decision with current
  authority and exact input/assignment revision checks. A timer expiry routes
  through fault policy, not “approved”, “rejected” or an invented human response.
- Capability acknowledgement is not ordinary completion. Timeout leaves an
  unknown effect with owned reconciliation. Eligible attempts preserve input,
  request and logical effect identity; changed inputs mean new work.
- Cancellation stops new ordinary work monotonically. Already-requested effects
  remain visible; compensation is new authorized work. A surviving reconciliation
  child has a declared, bounded role, not a generic escape from cancellation.
- `all`, `any` and thresholds use only the paired population. Finite fan-out
  seals membership before join evaluation. Removal settles obligations; it never
  erases completed evidence or permits a scheduler to shrink the population.
- Outcome `paid`, `closed` or `done` is not itself a proof of closure. The typed
  result and closure rule must establish evidence and single accountable owners.
- Race acceptance order is not event timestamp order. Guarded consequential
  commits need a supported invariant over every conflicting path; a label such
  as “no duplicate payment” is not that proof.

## 4. Protection and accessibility

Policy folds must retain a textual summary of effect class, required authority,
unknown outcomes, cancellation/settlement ownership and missing evidence. Users
can expand every clause without pointer gestures. Reflow or monochrome display
cannot change meaning. Required human instructions remain semantic work fields.
These are design obligations, not a tested accessible interface.

Source names, comments and metadata can themselves reveal sensitive facts.
The entire package must have an appropriate viewing policy; exclusion from the
semantic digest does not make annotations safe. A restricted projection identifies
omitted protected content without leaking labels/values, and cannot masquerade
as a complete, editable/approvable definition. Editing a permitted field uses
revision-checked structured changes against the authorized complete artifact,
not export/reimport of a redacted source that drops hidden semantics.

The conservative ADR-0013 information-flow rules may reject a desired leadership
summary even with redaction. There is no general declassification operator.
Need-to-know summaries, assistive-tech behavior and granular editing need actual
enforcement/interface evidence and possibly a separately approved contract.

## 5. Known blockers to complete lowering

1. Full grammar and versioned binding-package encoding, with rename/copy/merge
   and deterministic generation tests.
2. Exact supported provider, clock, calendar, nominal and authority resources
   for the complete benchmarks; named prose policies alone are not resources.
3. Complete structural/link/type/policy validation, including refusal diagnostics.
4. A faithful handling of corpus pressures outside current contracts: outage
   import with partial order, arbitrary ad hoc templates, cross-policy summaries,
   rich business-calendar calculation and runtime pause beyond named timers.
5. Text/IR/visual synchronization and protected accessible editing with people.

Do not close these gaps by adding unknown IR fields or treating requirements as
instructions. Future draft tools should accept an explicit unresolved-requirement
marker for editing, but **must refuse lowering the entire definition** until it
is resolved. Such a marker is never a new executable core node.
