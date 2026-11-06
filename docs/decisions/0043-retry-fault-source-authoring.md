<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0043: Author explicit retry fault-source bindings

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

ADR-0042/core 0.4 checks explicit retry-source/name/provider relationships. Source
0.6 selects core 0.3 and cannot declare them. Altering its old retry sentence or
inferring a source from available bytes would silently reinterpret existing
programs. Native reports must disclose the new typed premises without suggesting
actual fault acceptance or retry permission. Follow the
[frozen plan](../evaluation/0043-retry-fault-authoring-plan.md), `eb16001`.

## Decision

Adopt [source 0.7 / companion 0.6](../authoring/profile-v0.7.md), exact core 0.4
plus all inherited artifacts. A multiple-attempt binding explicitly says
`For retries use clock "Clock", basis "Input", fault source "Source".`
The new companion admits kind `provider-fault-source`; normal lexical names and
exact supplied resource/profile/provider/type bytes resolve it. Single attempts
omit the sentence and lower null bindings. Old pairs/grammar/cores/digests remain
unchanged; a new explicit package integrity domain separates 0.7.

Derive source-provider protection metadata reads alongside existing metadata,
including unused work declarations, with normal visibility, complete finite closure
and the same dependency budget. Preserve all strong edges and actual-use wait
endpoint rules, parser quotas and semantic revalidation. Unknown-version tests
move their sentinel to still-unsupported 0.8 rather than losing negative coverage.

A shared native helper discloses typed revision/site/source/profile/provider and
eligible observation/evidence declarations with live host requirements through all
five stages from timing onward. Limit row accumulation and final JSON serialization
before stdout. Preserve old JSON structure when no binding exists. There is no
observation/fault/permission/dispatch/continuation API or inferred host support.

## Options and consequences

| Option | Benefit | Cost or limitation | Decision |
| --- | --- | --- | --- |
| Explicit new source/core pair and readable source clause | Stable compatibility and checkable actual provider relation | Another provisional maintained version | Adopt |
| Infer resource from eligible name or registry | Shorter source | Ambiguous provider, changed legacy meaning | Reject |
| Accept only manual raw IR/derived read annotations | Less frontend work | Burdens ordinary authors and risks missing dependencies | Reject |
| Treat disclosed metadata as accepted retry evidence | Appears executable | Lacks actual observations, current authority and ownership | Reject |

The versioned clause is a bounded prototype, not final plain-English usability.
Tool-managed authoring/resources/identity and protected accessible text/visual editing
remain explicit roadmap work. Promotion changes semantic revision; formatting and
coordinated aliases within-profile preserve it. Candidate lowering stays separate
from semantic checking. All P3/P4/P6 live requirements and missing late-observation/
fault-data/continuation contracts remain open.

[Evidence](../evaluation/0043-retry-fault-authoring.md) records actual source/check/
disclosure, pin/budget/compatibility cases and completed-diff automated self-review.
Acceptance is delegated, not individual owner or independent approval. Require
passing final-head CI, all comments/CodeRabbit audit and signed exact-head merge
into draft #46. Then freeze separate RP-01 adoption; no scenario/phase gate closes.
