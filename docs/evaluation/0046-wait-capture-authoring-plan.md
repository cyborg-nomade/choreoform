<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Explicit wait-capture authoring: frozen dependency plan

**Date:** 2026-11-07<br>
**Baseline:** `c65cd8faf97d43ba0fd6b095b3bf04452c1e0955`, draft #46 after PR #59; raw ADR-0045 IR 0.2/core 0.5<br>
**Authority:** ADR-0018; no parent/scenario/phase admission

Use a dedicated stacked branch/PR; publish ADR-0046. Source 0.8.0 / companion
0.7.0 selects exact core 0.5 and Definition IR 0.2, requiring every inherited
core artifact explicitly. CLI embeds only the known core/semantics artifacts.
Copy and version source-0.7 EBNF, changing only wait to allow the optional phrase:
`Wait under "Policy" for observations from "Source" about "Origin" and record in "Result".`
Bare waits remain supported and lower required capture:null on the new wire.
The grammar's existing `in` qualification must not consume the origin/result
separator. Use normal lexical resource/node/data roles; companion alone gains
closed resource kind provider-terminal-observation-source in this version only.
Explicit source/profile/provider/type bytes are verified; no source truth or host
normalizer is inferred. Do not change earlier grammar/schema/artifact bytes,
source/core pairs, hashes or quotas. Unknown source sentinel moves from 0.8 to 0.9.

Lower the exact per-use source pin/origin/result and singleton destination writes.
Include source-provider protection references in actual wait node reads plus all
inherited policy, management, control and destination access dependencies. Share
the existing bounded dependency allowance; preserve finite metadata closure and
normal visibility, actual-use endpoints, semantic revalidation and source spans.
Libraries retain all raw type/provider/visibility/timer/protection/budget refusals.
Source 0.8 alone selects the explicit versioned IR constructor; old wire output
and null/absent field distinctions stay fixed. Keep package integrity domain ASCII
choreoform-authoring-package:0.8.0 and inherited framing. No companion editing,
manual IDs or derived read lists become the ordinary-author product goal.

Disclose `declaredWaitCaptures` from the retained typed static report through all
ten native checking stages and advisory invalidation requirements. Include exact
revision/site/source/profile/provider/origin/result and remaining host requirements.
These rows cover all declarations, including unvisited ones; they supply no live
activation, observation, value, fault receipt, current access or execution handle.
Do not rerun validation to rebuild rows. A shared helper bounds row accumulation
and final whole-report serialization at 1 MiB before stdout. Omit the key when
empty to preserve old-profile report JSON; overlarge output refuses report-budget
without partial JSON. Preserve existing retry/historical/isolated evidence.

Test readable ordinary capture->read/branch through actual frontend and all ten
partial checks, exact pins/bases/types/provider roles/visibility, source-only
protection reads, timer refusal, wrong pairs/kinds and every old/unknown header.
Check format/reparse/coordinated aliases with stable IR identity, independent EBNF /
schema/package/semantic digests, null captures/new wire, unchanged old packages and
native disclosure at every stage. Stress per-row/final output bounds with a small
accepted program and long source/provider pins; no higher quota. Add Rust and
independent Python CI evidence. Actual RP-01 promoted-copy local capture must still
refuse stored-recovery lifetime; committed source/package bytes remain unchanged.

Run relevant native/Wasm/all Python checks and final-head CI; completed-diff agent
self-review, all unresolved threads and CodeRabbit summaries, signed expected-head
merge into draft #46. Then freeze owned local-storage/lifetime proof before adding
that capability; fault data, permitted owned continuation/cutoff/settlement remain
separate dependencies. Parent stays draft/incomplete, all forty scenarios stay
Partial and every actual external/user/host/browser/independent and phase gate stays
open. Never substitute generated observations for those requirements.
