<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Reading authoring diagnostics

The native frontend reports a fixed code and an IR declaration path. When the
candidate has an original source span for that declaration, it appends:

```text
predicate-type at /body/policies/review_work; source bytes START..END
```

`START..END` is a half-open range of UTF-8 bytes in the exact original source:
START is included, END is excluded. It locates the containing declaration or
generated construct, not necessarily the particular erroneous word. It is not a
line/column, Unicode-character index or editor UTF-16 coordinate. No source literal
is copied into the diagnostic. Frontend syntax/binding errors already carry their
own byte ranges and retain that format.

Library callers can use `Candidate::source_span(&semantic_diagnostic)` with a
diagnostic from that candidate. It maps an existing `/body/{record-map}/{id}`
prefix, even if a field suffix is missing. Global/resource/unknown/missing-record
paths have no guessed source location. IDs in a different table do not match.
The immutable span map is available for tooling. Reparse formatted/edited source
to obtain new locations; an old byte range does not follow an edit. This helper
does not authenticate a caller-supplied diagnostic's revision.

## Interpreting the result

Read the code and stage together. A `*-unproved` refusal usually identifies a
conservative proof gap, not a language-wide prohibition. Type/shape/identity
errors identify inconsistent definitions or packages. Resource errors identify
missing, ambiguous, unsupported or mismatched artifacts. Budget errors mean the
bounded checker refused work; they are not proof that a process is valid or invalid.
No successful report grants planning, execution or host authorization.

Static scope policy/outcome/input, actor requirement, data protection/initializer/
invalidation and capability contract/type/authority/effect failures carry their
record/field paths. Input/output mismatches retain `capability-type` but point to
the specific side. Existing deeper paths are preserved. Global structure/revision/
resource and graph-wide errors remain unlocated when no exact cause is established.

Selected examples, rather than a complete catalogue:

| Code | Meaning and next inspection |
| --- | --- |
| `predicate-type` | A policy condition must produce a truth value. Inspect the located policy's condition and expression types. |
| `data-availability-unproved` | A required read is not established on every modeled ordinary path. Check input binding, initialization and incoming branches at the located step. |
| `exceptional-availability-unproved` | The handler requires a value outside its explicit ancestor-input premise. An ordinary result/initializer is not automatically a fault-entry snapshot. |
| `reconciliation-availability-unproved` | The surviving follow-up requires a value outside its zero-inherited-value premise. Check fresh local initialization; richer ancestor retention needs separate proof. |
| `reconciliation-followup-unproved` | A capability activity's own scope lacks the supported surviving follow-up declaration. Inspect that scope's closure; an ordinary unattached wait is insufficient. |
| `reconciliation-context-unproved` | The unique reconciliation wait is outside the designated direct child. Inspect actual scope ownership rather than the effect policy's declaration location. |
| `fault-routing-cycle-unproved` | The conservative active-handler graph cannot establish acyclicity. Inspect repeated selections and lexical propagation; retry eligibility does not bound emission. |
| `reference-missing` | A name is unavailable in the relevant binding context. Check spelling, lexical visibility and explicit qualifiers; do not make child data public to satisfy an endpoint reference. |
| `type-alias-cycle` | Type descriptions refer back to themselves, including through unused definitions. Replace the cycle with a finite complete description. |
| `type-expansion` | The expanded type exceeds a finite node or byte bound, even if its source is short. Inspect repeated nested type descriptions. |
| `type-member-visibility` | A type description's resource or outcome owner is private at its use. Keep the description within the owning scope; naming cannot expose a private sibling. |
| `protection-purpose-bindings` | A data declaration's managed purpose set differs from its named protection definition. Reconcile the explicit purpose identities; do not invent a purpose from a caption. |
| `protection-policy-kind` | A named protection definition must use a rule for protection. Inspect the definition's `Under` clause, including unused definitions. |
| `protection-member-visibility` | A referenced protection member is unavailable where the data is declared. Inspect scope ownership; a definition cannot expose a private sibling actor, capability or policy. |
| `size` / `work-limit` | Source, expansion, output or analysis exceeds its finite bound. Named wording may be short while its expanded data declarations are large. Failure supplies no partial candidate. |
| `ledger-profile` / `package-profile` | The source and companion do not form a supported version pair. Check the selected profile, not just the JSON shape. |
| `resource-missing-or-ambiguous` / `resource-digest` | Explicit matching resource bytes are absent/duplicated or have the wrong digest. A matching digest still supplies integrity rather than live authority. |
| `budget` / `report-budget` | Analysis or report size exceeds its fixed quota. Isolate the definition/diagnostic; do not treat truncated or partial output as evidence. |

Semantic field/source precision, richer explanations and a complete diagnostic
catalogue remain part of the full-validator/editor work. The current source map
adds actionable declaration locations while keeping those gaps explicit. See
[ADR-0033](../decisions/0033-semantic-source-locations.md).
