<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.10.0

**Status:** Accepted under [ADR-0053](../decisions/0053-handler-entry-authoring.md)
and owner delegation; effective on parent/main integration. Candidate and
conditional reports supply no actual fault history, current grant or execution.

Inherit [source 0.9](profile-v0.9.md) at baseline
`c7983201e4dd81ce2289a00cf236da740fcefbb2`, except below. Use header
`Choreoform authoring "0.10.0".`, companion version `0.9.0` / profile `0.10.0`,
[closed schema](../../schemas/authoring/bindings-0.9.schema.json) and
[versioned EBNF](candidate-a-v0.10.ebnf). All earlier profiles keep their exact
versions, grammar, schema, resource kinds, cores, wire and integrity domains.

## Local history declaration

A nested scope may add one clause immediately after its interface and before its
contents:

```
Receive fault history from "Payment history" and record in "Fault history".
```

Resolve the first reference normally as a visible provider-fault-history resource;
resolve the second as a local data cell. Qualified names may contain `in`, so
`and record in` is the delimiter. The root process has no such clause. This pair
alone adds the provider-fault-history resource kind. The descriptor pins exact
history/source/profile/provider/fixed nominal bytes from ADR-0050. Libraries require
all seven core artifacts and semantics; CLI embeds only known core/semantics bytes,
while actual history, source, profile, type, provider and other resources stay explicit.
Unused declared resources also validate. There is no inferred source, value or alias.

Select exact [core 0.7](../dialects/core-fault-entry-v0.7.md),
`sha256:e286e54441b3d785ae823eef359c12d64fb8405829c70709227282e5af62f54b`,
on [Definition IR 0.3](../ir/definition-v0.3.md). Every scope has required nullable
faultInput; only an explicit clause produces `{history:Ref,result:DataID}`. The
reserved cell has exact nominal Provider fault history type and no ordinary
initializer/writer/creator/ports. Type/protection/invalidation declarations remain
explicit. Raw semantic checks preserve all source/context/ownership barriers.
Fault-input semantic diagnostic paths map to the clause's original UTF-8 byte span;
other paths keep existing declaration/generated-record spans.

Known whole programs retain 16,384 tokens, 1,000,000 parser visits and the shared
100,000 lowering/dependency allowance. Unknown 1.0 and fragments retain legacy
bounds. Integrity uses the explicit source 0.10 package framing domain. Formatting /
reparse and coordinated readable aliases preserve identity/IR within this version;
promotion deliberately changes core/wire/package semantic hashes.

## Separate native judgments

All ten checking views plus invalidation-requirements disclose nonempty:

- declaredFaultEntries: revision, handler, result, history pin and host requirements;
- declaredFaultHistories: one per exact descriptor pin, with exact profile/source /
  source-profile/provider/fixed nominal identity and remaining host requirements.

Exceptional availability, reconciliation links and reconciliation availability
also disclose conditionalFaultEntries. Covered/uncovered entries, conditional body
sets, transfers, access/invalidation/provenance obligations, supported incoming
site/full selection contexts and host premises remain separate. Typed nodes can
still appear in legacy uncoveredNodes; they are never promoted into ordinary or
legacy checked sets. The retained SDK chain is shared, not recomputed for output.
Unvisited/unexplored declarations remain disclosed; no-incoming entries stay
uncovered. Unsupported possible incoming sources refuse rather than disappear.

Capture/source/history/conditional row accumulation shares a cumulative bounded
allowance, with inherited fields included and a complete final 1 MiB writer bound
before stdout. Overflow returns report-budget without partial output. Catalogue
sharing never shares occurrences, current grants or live values. Actual event and
source/control provenance, current authority, owned storage, nearest live selection,
cutoff and settlement remain mandatory. The clause does not resume an activity or
parent. General tool-managed near-English editing and actual usability/host/browser /
independent evidence remain open; committed RP-01 stays source 0.7/IR 0.1.
