<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.8.0

**Status:** Accepted under [ADR-0046](../decisions/0046-wait-capture-authoring.md)
and owner delegation; effective on parent/main integration. Candidate lowering and
partial checking supply no executable admission or accepted observations.

Inherit [source 0.7](profile-v0.7.md) at parent
`c65cd8faf97d43ba0fd6b095b3bf04452c1e0955`, except below. Use header
`Choreoform authoring "0.8.0".`, companion version `0.7.0` and profile `0.8.0`,
[closed companion schema](../../schemas/authoring/bindings-0.7.schema.json) and
[versioned EBNF](candidate-a-v0.8.ebnf). Earlier grammar/schema/artifact bytes,
source/core/wire pairs, integrity domains and refusals stay unchanged.

## Capture a declared terminal payload

A wait optionally spells its exact source, origin and destination:

```
Wait under "Payment reconciliation" for observations from "Payment facts" about "Request payment" and record in "Terminal result".
```

Resolve these as resource, node and data references under normal lexical visibility.
Existing `in "Scope"` qualification still applies; no descendant/private exception
is introduced. The companion resource kind is exactly
`provider-terminal-observation-source`, available only in this version. Bare waits
remain valid and lower capture:null. Capturing waits derive exactly one destination
write and all source-protection, wait/management and destination access reads;
authors do not maintain those derived sets manually.

Select exact [core 0.5](../dialects/core-wait-capture-v0.5.md) /
`sha256:50007ac05e5e71df91bb9997a40a06c7a8993df04b1a673421e4a987014299d7`
and [Definition IR 0.2](../ir/definition-v0.2.md). All older node records remain
unchanged; every new-wire wait has the required nullable capture field. Libraries
require all exact core 0.5/0.4/0.3/0.2/0.1 and semantics bytes. The CLI embeds those
known artifacts; supply actual source/profile/provider/type bytes explicitly.

Lowering produces a candidate, not semantic admission. Static checking verifies
source/provider identity against the actual capability origin, nominal types,
normal visibility, singleton writes and intrinsic source-to-target protection,
including unvisited declarations. The wait must be timerless in this subset.
Ordinary control/protection/invalidation and later partial checks retain their
coverage and residual requirements. The original occurrence is not activated by
naming it, and no prior runtime input values are implicitly retained.

Keep the explicit 16,384-token/1,000,000-recognizer quota for known whole programs,
legacy fragment/unknown-header limits, actual-use wait endpoints, finite metadata
closure and 100,000 lowering dependency allowance. Unknown 0.9 headers do not
select higher quotas. Source-protection metadata walks/copies use the same allowance.

## Native declaration evidence and integrity

Every checking command, including advisory `invalidation-requirements`, includes
`declaredWaitCaptures` when nonempty. Rows are all declared sites, including
unvisited ones, with exact definition revision/site/source/profile/provider/origin/
result and host requirements. They contain no payload/value, accepted observation,
activation, authenticated source, current permission or execution handle. Every
row retains live origin/correlation, access/authority, atomic single acceptance,
duplicate/contradiction history, cutoff and owned storage/settlement requirements.

The API returns the already retained typed static report; no second validation is
used to reconstruct disclosure. Bound row accumulation and final combined JSON
at 1 MiB before stdout; overlarge output fails `report-budget` with no partial JSON.
Reports without captures omit this key and retain their previous structure.
Existing retry-source/historical-origin/isolated-follow-up evidence remains.

Use package domain ASCII `choreoform-authoring-package:0.8.0`, zero byte and the
inherited exact length framing/source/JCS companion. Wire/core promotion changes
semantic revision; formatting/reparse and coordinated aliases within this profile
preserve IR/revision when stable identities and meaning remain unchanged.
Readable names are the authoring surface. Tool-managed identities/resources,
arbitrary export/editing, visual parity and actual representative-user studies
remain open roadmap work; this increment does not establish the final syntax.

[Evidence](../evaluation/0046-wait-capture-authoring.md) uses static source fixtures
and promoted copies. Checked-in RP-01 source 0.7/core 0.4/IR 0.1 package stays
unchanged. Captured local storage needs the separate
[ADR-0047](../decisions/0047-owned-local-followups.md) conditional isolation,
initialization, protection and invalidation proof; actual host storage remains a
live premise. General retention, fault data, permitted continuation/cutoff and
all-settlement need further contracts. Actual host/user/browser/independent evidence and all
full benchmark/scenario/Phase 1 gates remain open.
