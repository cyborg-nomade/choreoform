<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Definition IR 0.2.0 — explicit wait-capture representation

**Status:** Accepted under [ADR-0045](../decisions/0045-explicit-timerless-wait-capture.md) and owner delegation; effective on parent/main integration.

Inherit [Definition IR 0.1](definition-v0.1.md), preserving its exact schema/bytes,
except the version-specific closed wait shape below. Use envelope version `0.2.0`.
All other envelope/body/core records, identities, scopes, graph rules, transport
limits, annotations and exact external references retain their inherited meaning.
No defaulted, unknown or behavior-bearing annotation fields are introduced.

```
{kind:"wait",scope:ScopeID,outcomes:Set<Name>,reads:Set<DataID>,
 writes:Set<DataID>,policy:PolicyID,
 capture:null|{source:Ref,origin:NodeID,result:DataID}}
```

Capture is required and nullable only in IR 0.2 wait records. Its source Ref is
exactly `{id:Name,revision:Digest}`; origin/result resolve to existing node/data
identities. The capture object is closed, with all three fields required. Every
other node shape is unchanged. IR 0.1 rejects capture, even null; IR 0.2 rejects
omission or additional fields. Unknown wire versions refuse rather than selecting
this shape. The existing in-memory constructor stays fixed at 0.1; callers must
explicitly select the new version through the versioned constructor.

Canonical semantic projection remains JCS of format/version/kind/body; version
therefore participates in the revision. Preserve all original hashes/fixtures.
This is not a runtime checkpoint or provider observation. Capture references are
declaration links, not control edges or implicit activity activation/resumption.

Structure validates only shape, reference roles and exact ref/digest syntax.
It does not authorize behavior, source support, correlation or stored values.
[Core profile 0.5](../dialects/core-wait-capture-v0.5.md) supplies the initial
semantic subset: explicit timerless accepted-payload capture, exact source/provider/
origin/type/protection and complete dependencies. Legacy wait policies may use
only null capture. Unsupported semantic bindings or host judgments still refuse.

Text/visual authoring, supported metadata resources, full semantic/protection/
lifetime checks and all phase evidence remain separate. No source profile is
implicitly migrated, no existing wait gains a result, and no closed/cancelled
scope is reopened. Ordinary and terminal closure retain their prior distinction.

The typed SDK projects legacy waits with `capture: None`; a new `WaitCapture`
contains distinct NodeId/DataId roles and an unverified ArtifactRef. Constructing
an envelope explicitly at 0.2 still does not repair missing fields or validate a
source, policy or executable definition. Semantic checks remain separately required.
