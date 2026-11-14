<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.7.0

**Status:** Accepted under [ADR-0043](../decisions/0043-retry-fault-source-authoring.md)
and owner delegation; effective on parent/main integration. Candidate lowering and
partial checking supply no executable admission or actual retry permission.

Inherit [source 0.6](profile-v0.6.md) at parent `f6cffd2a88635e84b39b49261d984be11e5ac263`,
except below. Use header `Choreoform authoring "0.7.0".`, companion version `0.6.0`
and profile `0.7.0`, the [closed schema](../../schemas/authoring/bindings-0.6.schema.json)
and [versioned EBNF](candidate-a-v0.7.ebnf). All earlier pairs, grammar/artifact bytes,
core meanings and integrity domains stay unchanged. There is no implicit migration.

## Explicit retry source

A multiple-attempt work policy's required binding sentence is:

```
For retries use clock "Expense clock", basis "Payable request", fault source "Payment faults".
```

Resolve the source using normal lexical resource references; require companion kind
`provider-fault-source`. This resource kind is available only in the new companion.
The grammar changes only the retry-binding production from the earlier 0.3 EBNF.
The clock and basis retain their inherited meanings. Single-attempt work omits the
sentence and lowers null clock/basis/faultSource. Core validation requires empty
eligible faults for single attempts. Candidate lowering is distinct from subsequent
static checking: parsing a name never establishes its declared provider relation.

Select exact [core 0.4](../dialects/core-fault-source-v0.4.md),
`urn:choreoform:contracts:core-profile:0.4.0` /
`sha256:0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3`.
Libraries require exact core 0.4/0.3/0.2/0.1 and semantics bytes; the CLI embeds those
known core artifacts. Supply actual fault source, its supported profile, exact
provider/type bytes and all other resources explicitly. No discovery or host-support
inference occurs. The source reader verifies mapping metadata; static checking
additionally resolves eligible names/classes and exact actual-use provider identity,
including unvisited uses, and refuses human contexts. Unresolved eligible evidence
requires declared stable-key metadata, which still does not prove implemented support.

Source-provider protection references join the inherited provider metadata edges
in complete derived policy reads, even for unused declarations. Preserve strong
references, normal visibility, least finite closure and semantic revalidation.
Initial graph work and shared helper work retain the existing 100,000 dependency
allowance. Keep the explicit known-header 16,384-token/1,000,000-parser quotas,
legacy/fragment bounds and actual-use wait endpoint rules. Unknown 0.8 headers do
not select new quotas. No larger allowance is introduced.

## Native evidence and integrity

The five native commands from `check-timing` through
`check-reconciliation-availability` conditionally include `retryFaultBindings`.
Each row discloses exact definition revision/site/source/profile/provider pins,
eligible fault observation names/evidence classes and live host requirements from
the typed library report. There is no row for an uncovered/unused retry use, and
no inferred binding for a legacy core. Existing old-profile JSON structure remains
unchanged when there are no bindings. These rows are declared relationships, not
actual fault receipts, provider observations, permissions or activation evidence.
Row accumulation and final serialization are bounded before stdout; overlarge
reports fail `report-budget` with no partial JSON. The final output limit is 1 MiB.

Use package domain ASCII `choreoform-authoring-package:0.7.0`, zero byte and existing
length framing/exact source/JCS companion bytes. Promotion changes the selected
core and retry body, therefore semantic revision as well as source/package digests.
Within-profile format/reparse and coordinated alias changes preserve IR/revision
when identities and meaning remain unchanged. Source references use readable names;
general tool-managed resource/identity editing and text/visual parity remain open.
Manual companion JSON or generated-ID bookkeeping is not the ordinary-author goal.

[Evidence](../evaluation/0043-retry-fault-authoring.md) uses promoted synthetic
RP-01 copies only; checked-in source 0.6/package reports remain unchanged until
separate adoption. Actual evidence authentication/correlation/current authority,
retry acceptance/cutoff, owned settlement, fault data, late observation capture and
continuation remain unimplemented gates. No full benchmark, user study, host/browser
execution, scenario or Phase 1 exit follows from this profile.
