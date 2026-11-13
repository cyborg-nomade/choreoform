<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.17.0

**Status:** Accepted under ADR-0084 / owner delegation; effective on integration.

Inherit [source 0.16](profile-v0.16.md) except the additions below. Header
`Choreoform authoring "0.17.0".` pairs only with companion 0.16/profile 0.17 and
[its closed schema](../../schemas/authoring/bindings-0.16.schema.json). The
[grammar](candidate-a-v0.17.ebnf) is normative. Select unchanged core 0.12 / IR 0.4
and all inherited exact resource contracts; source does not select core 0.13.
No earlier profile gains new syntax, kinds or identity rules.

## Explicit complete protection

A scoped declaration names the entire existing data protection envelope:

```text
Protect data as "Order information".
  Under "Order access".
  Sensitivity "order".
  Purposes ["Order fulfillment"].
  Participants ["Order clerk"].
  Capabilities empty.
End protection.

Keep "Original order" as text, protected as "Order information".
  Invalidate empty.
  Initially unavailable.
End data.
```

This is a syntax excerpt, not a complete process or permission grant. The access
policy and actor must be declared. Existing inline `Keep ... under ...` remains
available; a declaration cannot combine inline and named protection. No implicit
or inherited default, override, nested group, recursive alias, new semantic policy,
work/template/type/predicate reuse or provider result-protection rewrite exists.
Type, invalidation, initialization and lifetime remain independent.

A definition has companion declaration kind `data-protection`, an explicit stable
ID, owning lexical scope and exact caption. It creates no IR record. Collect and
resolve every definition before data uses, including unused ones and definitions
appearing after a use. Enforce existing duplicate, no-shadowing, kind and entire
qualified-path/visibility rules. Resolve policy/actor/capability members in the
definition's owning scope; policy must be a protection policy. Purposes must be
nonempty. Repeated purpose captions or resolved member IDs refuse. Empty participant
and capability sets retain their existing meaning. Check member visibility again
at each data use, then retain all ordinary use-site semantic checks. A private
sibling declaration cannot be exposed through a named definition.

## Managed identity and purpose references

Definition purpose captions have companion `purpose` symbols owned by the
`data-protection` declaration. Every named data use retains its own existing
`purpose` symbols, including their IDs, captions and wire spellings. Its complete
wire-purpose set must equal the definition's set, without missing, surplus or
invented symbols. This preserves `purpose "caption" of "data"` expressions and
permits different local captions for the same fixed wire purposes. It does not
infer purpose identity from group spelling or generate IDs. Inline data still
binds its own explicit source purpose list as before.

The expanded IR `data.protection` is exactly the definition's resolved policy,
sensitivity, purposes, participants and capabilities. Semantic data/policy/actor /
capability IDs and all resource pins remain fixed. Unknown companion annotations
survive formatting and lowering. Source-only definition/symbol IDs share the global
reserved ID space and tombstones. Rename updates explicit source and companion
captions, reorder retains IDs, copy allocates explicitly fresh source identities,
and removal retires them. Stale/missing/surplus bindings refuse; no matching by
position, normalization or approximate caption. These are package rules, not a
claim that a general safe editor or IR exporter is implemented.

## Bounded lowering and evidence

Retain source/companion/final wire 1 MiB each, depth 64, 131072 tokens, parser
1000000 work, dependency/lowering 100000 work and semantic/shared bounds. For this
profile additionally charge one cumulative lowering budget for definition visits,
resolved envelope JSON nodes/member checks and each named-use expansion, before
copying an envelope; dependency analysis starts with the consumed budget. Charge
the compact JSON bytes of each resolved definition and each named-use copy against
one cumulative 1 MiB expansion bound before copy. Immutable resolved definitions
are reused, never live access truth. Also bound the cumulative compact bytes of
emitted records at 1 MiB before insertion; final envelope framing/annotations remain
subject to the unchanged full wire bound. No truncated or partial success.

Diagnostics retain meaningful definition/use UTF-8 spans and never echo literals.
Formatting/reparsing preserves definitions, explicit identities, annotations and
exact expanded semantic body/revision; integrity still frames the new profile
separately. Source-level factoring alone proves neither accessibility nor access.
Required evidence includes exact unchanged whole RP-01/RP-03 IR, at least 15%
measured RP-03 source-byte reduction in new test variants and the same legitimate
external-effects refusal. Current benchmark adoption/archive remains a separate
signed task. Full RP-08, protected accessible inspection/editing, visual parity,
real users/hosts/browsers/independent evidence and all unproved Phase 1 gates stay
open. No execution or Phase 2 follows.
