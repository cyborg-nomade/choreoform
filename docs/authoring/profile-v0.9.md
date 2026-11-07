<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.9.0

**Status:** Accepted under [ADR-0049](../decisions/0049-general-fault-source-authoring.md)
and owner delegation; effective on parent/main integration. Candidate/partial reports
supply no actual fault, history data, current grant or execution admission.

Inherit [source 0.8](profile-v0.8.md) at parent
`af62713386b4b5c6d437fa4ad87bee43f4e4b7f5`, except below. Use header
`Choreoform authoring "0.9.0".`, companion version `0.8.0` and profile `0.9.0`,
[closed schema](../../schemas/authoring/bindings-0.8.schema.json) and
[versioned EBNF](candidate-a-v0.9.ebnf). Every earlier grammar/schema/profile/core /
wire/hash domain stays fixed. No implicit migration or new resource kind exists.

## General source and retry sentence

A work policy may declare its general provider emission source after authority:

```
Rule for work "Send notice".
Instructions "Notify the intended recipient".
Authority "Payment operator".
Recognise faults from "Notice faults".
Attempts 1. Delays empty. Retry faults empty.
Timeout none. Complete when (true). Compensates none.
End policy.
```

Resolve the name as a normally visible resource of kind provider-fault-source.
Single attempts may omit the sentence, lowering outer source null, while their
inner retry source/clock/basis stay null and eligible faults empty. Multiple attempts
require the general source and retain explicit clock/basis binding:

```
Recognise faults from "Payment faults".
For retries use clock "Expense clock", basis "Payable request".
```

The one explicit source populates both outer/general and inner/retry source fields.
No name is repeated, guessed or made eligible automatically. The old extra
`, fault source "…"` belongs to source 0.7/0.8 and is not part of the new retry
production. Parsing/lowering still produces candidate IR; actual capability/provider
identity, human/unvisited/unused contexts and all raw rules are semantically checked.
Existing endpoint validation may refuse a converted human use before static checking.

Select exact [core 0.6](../dialects/core-emission-source-v0.6.md) /
`sha256:27bce3fde9a0393abb6bf760a8588af9268de65069b73f4cb946144f5d30cb85`
on Definition IR 0.2, retaining required nullable wait capture and all inherited
capture/retry/metadata/protection/lifetime rules. Libraries require every exact
core 0.6/0.5/0.4/0.3/0.2/0.1 and semantics artifact. CLI embeds those known bytes;
supply actual source/profile/provider/type and other resources explicitly.

Derive source-provider protection dependencies from the general source, even on
unused policies. Reuse that chosen source for multiple-attempt metadata rather
than walking both fields. Keep normal visibility, strong/finite closure and complete
read derivation/revalidation. Known whole programs retain 16,384 tokens and
1,000,000 recognizer pattern visits; fragments/unknown headers keep legacy bounds.
The shared 100,000 dependency allowance remains; unknown 1.0 selects no new quota.

## Native declarations and integrity

All ten checking stages plus advisory invalidation requirements conditionally add:

- `declaredFaultSourceUses`: exact revision/site/work policy/source/profile/provider
  and live host requirements for every declared non-null use, including unvisited
  sites without an activation claim. Unused policies have no use row.
- `declaredFaultSourceCatalogues`: one all-name evidence/observation map per exact
  source pin used in those rows. Catalogue order is deterministic by pin. All names
  remain distinct from eligible retry names in existing later-stage bindings.

These fields come from the retained typed static report, without a second admission.
They contain no actual emission/receipt/history value, authenticated source, current
permission or handler entry snapshot. Correlation/classification, fresh authority,
actual eligibility/delay/cutoff, exact selection/exclusion, cancellation monotonicity,
retained provenance and owned settlement remain required. Null source is not proof
of no possible faults. Shared metadata does not share live occurrences or grants.

Capture/use/catalogue row accumulation shares the existing bound and the final whole
JSON is limited to 1 MiB before stdout. Overlarge output refuses report-budget with
no partial JSON. Empty fields remain omitted, preserving old report structure;
existing retry/historical/isolated/conditional local evidence remains.

Use package domain ASCII choreoform-authoring-package:0.9.0, zero byte and inherited
exact length framing/source/JCS companion. Core/work/profile promotion changes
semantic/source/package digests; within-profile formatting/reparse and coordinated
aliases preserve IR/revision when identities and meaning stay fixed. Readable names
remain the surface; tool-managed identity/resource editing, complete authoring and
representative-user evaluation remain open roadmap work.

[Evidence](../evaluation/0049-general-fault-source-authoring.md) uses explicit
promoted/static fixtures only. Committed RP-01 source 0.7/core 0.4/IR 0.1 bytes stay
unchanged. Typed fault history, handler-local entry, permitted owned continuation /
cutoff/all-settlement, complete validator/benchmarks, visual/protected accessible
editing and all actual user/host/browser/independent/scenario/phase gates remain open.
