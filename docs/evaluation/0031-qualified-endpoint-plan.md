<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Qualified reconciliation endpoints: frozen plan

**Date:** 2026-11-06<br>
**Baseline:** `9a530b257ed0fdc2d5b891fea905a1e12ff4dd89` (PR #40)<br>
**Authority:** ADR-0018 delegation; ADR-0030 source dependency.

Publish ADR-0031 and source 0.4.0 / companion 0.3.0, retaining the exact core
0.2.0 contract and inherited base. Reuse the unchanged 0.3 EBNF: this revision
changes binding rules for existing qualified outcome references, not tokens or
IR meaning. Preserve old source/profile/schema/example bytes and digest behavior.

Only wait policy observation/timer outcome owners gain a specialized fallback:
after ordinary lexical lookup reports reference-missing, an explicitly qualified
node may resolve directly inside the policy scope's designated direct-child
reconciliation follow-up. Require that actual node to be a wait using this exact
policy. Both outcomes retain the same owner and existing symbol/flow identities.
Wrong-kind/qualification errors and ordinary lexical precedence stay intact.
Never expose arbitrary descendant nodes, data, expressions or type references.

Resolve new-profile wait outcomes after all scope/policy declarations, before
steps need their outcome wires; validate exported endpoint node kind/policy after
step lowering. Retain original byte spans, indexed ledger lookup and existing
package/token/depth/work/IR size limits. Avoid cloning whole syntax trees.
No dummy root proxy, implicit migration, core revision, dependency update or live
authorization follows. New CLI/library/Python profile selection must be strict.
The package hash domain becomes 0.4.0; promotion of an otherwise identical 0.3
package preserves its IR revision because it selects the same core meaning.

Provide an owned service source/companion example and source lowering/reparse/
identity/report evidence. Tests cover correct child endpoint and declaration
order; observation/timer owners; wrong/non-designated/nested/non-wait/other-policy
targets; private data and general outcome references; legacy refusals; strict
versions/pins; retry/clock compatibility; alias edits and bounded inputs. The
independent Python grammar/schema/integrity oracle is not a second semantic
evaluator or actual host/browser/user evidence.

Run native/Wasm and all relevant artifact/source/stage checks. Separately review
the completed diff, inspect all PR comments, record delegated acceptance honestly,
and require passing final-head CI and expected-head merge. Then continue remaining
Phase 1 entry/protection/invalidation/lifetime, validator, full benchmark,
text/visual/protected editing and actual user evidence. No phase gate closes from
this bounded source binding.
