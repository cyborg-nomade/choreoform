<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0031: Bind explicitly qualified reconciliation wait endpoints

**Status:** Accepted under Project Owner delegation; effective upon PR merge<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

ADR-0030 checks existing IR links between the activity's surviving direct-child
follow-up and its unique reachable wait. Old source profiles cannot name that
private child wait from the parent's wait policy. A dummy ordinary root endpoint
would contradict the intended relation. General child-data visibility would
unnecessarily weaken lexical privacy. The existing qualified English reference
already supplies the needed words; the missing piece is a narrow binding rule.

## Decision

Publish [source 0.4.0](../authoring/profile-v0.4.md) / companion 0.3.0 with the
unchanged 0.3 token grammar and exact core 0.2.0 pin. Only wait-policy observation/
timer outcome owner resolution gains a designated-child endpoint fallback after
ordinary lookup reports missing. Require its own lexical scope's actual reconcile
closure, direct child owner, an actual wait using this exact policy, same observation/
timer owner, and existing symbol/flow identities. Retain all general lexical rules.

Defer new-profile wait outcome binding until all scope/policy declarations, before
steps need the wires; validate actual endpoint node kind/policy after steps. Use
borrowed syntax trees, existing ledger indexes and bounded package sizes. Preserve
exact byte spans. Strictly match versions/pins in Rust and Python, share the grammar
cache and keep old profile/schema/example/snapshot bytes unchanged. Package hash
domain advances; an otherwise identical 0.3 package keeps its semantic IR revision.

## Options and consequences

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Narrow qualified endpoint binding in a new source profile | Short English references; preserves privacy and existing IR | Adds a versioned binding rule and declaration staging | Adopt |
| Export every child declaration | Simple lookup rule | Exposes unrelated private data/nodes; unnecessary semantic scope | Reject |
| Add proxy root wait | Keeps old binder | Misrepresents actual endpoint ownership | Reject |
| Move all wait policies into children | Ordinary lookup works locally | Parent effect cannot name private child policy without another exception | Reject |
| Invent a new IR field or contract | Can encode explicit public endpoints | Existing IR already represents this relation | Defer until a demonstrated need |

This supplies source declaration evidence, not live follow-up entry, authorization,
obligation transfer, observation correlation, durable progress or terminal settlement.
General export/editing, ordinary tool-managed companion creation, visual parity,
protected accessible editing, benchmarks and actual users remain open. No engine,
individual owner review or independent review is implied.

Follow [frozen plan](../evaluation/0031-qualified-endpoint-plan.md), `4f7b2e7`.
[Evidence](../evaluation/0031-qualified-endpoints.md) records 235 Rust tests plus
five compile-fail doctests, native/Wasm and all relevant artifact/source/stage
checks. Separate completed-diff automated self-review found no remaining actionable
finding. Accepted under delegation; require passing final-head CI and expected-head
merge. No independent or individual owner review is claimed.
