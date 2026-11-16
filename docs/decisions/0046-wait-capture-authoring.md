<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0046: Author explicit capture with readable references and disclose declarations

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-07<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

ADR-0045 admits explicit raw timerless capture on IR 0.2/core 0.5, while source
0.7 deliberately retains old wire output and refuses that syntax. Ordinary authors
need readable source/origin/result references, with derived dependencies and typed
semantic revalidation. Native checking must expose the declaration's residual
requirements without recreating validation or implying an accepted observation.
Follow the [frozen scope](../evaluation/0046-wait-capture-authoring-plan.md), `b29db0b`.

## Decision

Publish [source 0.8 / companion 0.7](../authoring/profile-v0.8.md), explicitly
selecting core 0.5 and wire 0.2. Change only the versioned grammar's wait production:

```
Wait under "Policy" for observations from "Source" about "Request" and record in "Result".
```

The optional clause names the exact terminal source resource, ordinary visible
capability-origin node and destination data cell. `about … and record in …` leaves
existing `in` qualification unambiguous. Bare waits lower capture:null on this new
wire. Earlier source pairs keep their previous grammars/core/wire/hash behavior.
Keep explicit known whole-program quotas and actual-use wait endpoints; unknown
headers/fragments retain legacy bounds. No new quota is introduced.

The new companion alone permits provider-terminal-observation-source resources.
Verify actual source/profile/provider/type bytes; libraries require every exact
inherited core, while CLI embeds known semantics/core artifacts. Derive singleton
writes and actual wait reads from source protection plus all existing policy,
management and destination access dependencies. Charge the additional metadata
walk/dependency copies before use under the same bounded lowering allowance.
Retain all raw static provider/type/visibility/protection/timer and lifetime limits.

All ten native checking stages plus advisory invalidation requirements conditionally
include `declaredWaitCaptures` from their retained static report. Include every
declared site, even unvisited, exact definition revision/source/profile/provider/
origin/result and explicit host requirements. Do not rerun validation, infer live
activation, return values or confer execution/acceptance authority. Omit the field
when there are no captures, preserving old report structure. Bound accumulated
rows and final combined output at 1 MiB before stdout; refuse rather than truncate.
Keep retry, historical-origin and isolated-follow-up disclosure intact.

## Alternatives and consequences

| Option | Benefit | Cost or limitation | Decision |
| --- | --- | --- | --- |
| Explicit readable source/origin/result clause | Inspectable provenance and destination with normal resolution | A longer wait sentence and a new explicit profile | Adopt |
| Infer source from a type or target name | Shorter spelling | A type or label does not establish provider/origin provenance | Reject |
| Put capture in a shared wait policy | Short per-node phrase | Binds shared ancestor policy to private actual-use context | Reject for this subset |
| Disclose accepted values or reuse a handler output | Looks like an executable reconciliation feature | No host acceptance, owned storage or fault-data proof exists | Reject |

Within-profile formatting/reparse and coordinated aliases preserve semantic identity;
source/package digests still change with source bytes and selected profile domain.
General resource/identity editing must be tool-managed. This increment does not
complete the plain-English authoring goal, arbitrary IR export, visual parity or
representative-user accessibility studies; these remain explicit roadmap work.

The actual full RP-01 promoted-copy capture passes static/invalidation and still
refuses stored-recovery lifetime. Keep checked-in package source 0.7/core 0.4/IR 0.1
unchanged until remaining contracts and separate adoption are reviewed. Next freeze
owned local storage/lifetime proof, followed by explicit fault data and permitted
owned continuation/cutoff/all-settlement. Parent #46 stays draft/incomplete; all
scenario/phase/external gates remain open. See [evidence](../evaluation/0046-wait-capture-authoring.md)
for tests, independent finite Python checks and completed-diff automated self-review;
no independent or individual owner approval is asserted.
