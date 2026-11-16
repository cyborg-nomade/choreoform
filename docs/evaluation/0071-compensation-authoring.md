<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Readable compensation identity and the next whole-order binding gap

**Date:** 2026-11-11<br>
**Baseline:** #85 head `b40d42454d55d0165a7e3a788b31f96756b324a3`, merged
as `94024f3c5fba3e56c19a9108f8f7b5e6aa354158` before production work<br>
**Frozen plan:** `30ac848`, [scope](0071-compensation-identity-authoring-plan.md)<br>
**Decision:** [ADR-0071](../decisions/0071-compensation-identity-authoring.md)

## Narrow binding and preserved privacy

Source 0.16/companion 0.15 explicitly selects C12/IR 0.4 with all eleven bases. Grammar
productions/schema structure are inherited; new schema metadata and selected core
are explicit. Every closed inherited feature/builtin family includes the new known
profile. New programs inherit source 0.15's131,072 tokens; all old/other byte/depth /
parser/lowering/semantic/shared/report limits stay fixed. Unknown future tests now
name 0.17 explicitly, rather than treating the newly known0.16 as unsupported.

`Ledger::compensation_identity` is used only by this profile's `Compensates` slot.
It requires complete exact scope paths rooted in the same definition, bounds depth,
walks existing name/declaration maps and requires direct node membership. No caption
scan, nearest-name fallback for qualified paths or generic resolver change occurs.
Unqualified names remain lexical. After nodes bind, the normal dependency pass
charges one additional shared work unit per link and checks capability-activity
kind/mode. A private policy-to-reference span map preserves exact UTF-8 diagnostics.
No separate unbounded pass or independently reset budget is introduced.

Other private data/results/outcomes/policies/actors/capabilities/control stay lexical.
Wait-endpoint actual-use boundaries are inherited unchanged. No target private value,
actual eligible effect or current authority is read or invented. C11's finite owned
proof still requires the immediate incoming source; broader static identity cannot
grant dynamic correlation, capture, new-key, ownership, handoff or settlement.

## Actual bounded evidence

Four public SDK groups and seven Python/native groups exercise real parse/bind/lower
and static/conditional checks using complete synthetic fixture resources and original
predicates. They are not provider/host/user observations. Expanded lexer boundaries
check exact new/old program limits; fragments remain legacy. All twelve selected/base
core artifacts are required by direct SDK lowering.

| Witness | Actual result |
| --- | --- |
| Exact same/ancestor or fully qualified sibling prior | New candidate/static check succeeds |
| Same sibling reference in source 0.15 | Preserved `reference-missing` |
| Unqualified private sibling | `reference-missing` at exact reference |
| Partial/foreign root qualification | `reference-qualification` at exact reference |
| Missing scope or target in a real exact scope | `reference-missing`, no ancestor fallback |
| Wrong intermediate or target declaration kind | `reference-kind` |
| Finish or otherwise valid human target | New-profile `compensation-target` before candidate formation |
| Shadowed target caption | Full path names the exact prior; unqualified nearest finish refuses |
| Exact Unicode scope / different decomposed spelling | Exact identity succeeds / `reference-missing`; no normalization |
| Qualified private value/outcome in completion predicate | `reference-missing` in independently valid grammar |
| Qualified private closure policy | `reference-missing`; purpose-only exception does not apply |
| Same-site adjustment identity | Static succeeds; concurrency `owned-continuation-compensation-source` |
| Wrong pair/core/base/resource/future | Real refusal, empty native candidate output |
| Formatting/reparse of valid sibling package | Exact whole candidate document/revision preserved; source digest changes |
| Eleven old/new small native views | Only exact core/revision identity translation differs; all conditional/report meaning preserved |
| Eleven new sibling native views | All pass with `executable:false` |

The valid human target has its own complete node/outcome/flow declaration and a
passing unlinked static baseline; no predicate or capability was weakened to obtain
the negative. The old capacity/finite-work future-only tests now name 0.17 explicitly.
No earlier source/profile/core artifact or committed benchmark was rewritten.

## Whole RP-03, unchanged source meaning

Only the temporary header/pair/core select this profile, preserving every original
resource, scenario clause, protected label, expression and receipt guard. The actual
frontend recognizes all 505,980 source bytes / 77,135 tokens in 832,138 visits under
the unchanged one-million native work cap. It passes the earlier sibling-private
compensation reference, then genuinely refuses `symbol-mismatch at 0..0`, exit 1,
empty stdout. **No candidate IR, semantic revision or full semantic-stage report
exists.** Later stages are not run. Direct SDK and native CLI agree on this refusal.

The temporary companion is 229,261 pretty UTF-8 bytes / 166,165 JCS bytes. Native and
independent Python integrity agree on these package identities (not semantic proof):

- Source: `sha256:b38f37b7240b4c0fbae2b5aa6f6574f3692ceab6c7cf6f10fde9e60c1370c094`.
- Bindings: `sha256:1e29afd35bc79a0ea32a377887b31b8ebef2b1b8f7ba204c845a3c1c06b7b3da`.
- Package: `sha256:8bd1388d2d1a5a13a3aa3dd0012c6a3989455f794076d136d7a6e56aef5d9800`.

Independent Python full recognition still reaches its unchanged one-million state
bound; no recognition agreement is claimed. Original source 0.14 token failure and
source 0.15 private-reference failure remain covered. All committed RP-01/RP-03 bytes
and historical evidence stay fixed. This is no full-case IR bypass or adoption.

The next dependency is a separately frozen whole-case symbol/companion audit to
identify the actual missing owner/role/name and why it is requested. The current
generic location 0..0 is insufficient to identify that cause. Do not guess a generator
repair or new language permission, drop a clause, mutate receipt guards or fabricate
later semantic results. Choose the correction from real source/ledger evidence.

## Verification and limits

Local verification passed 575 Rust tests plus six compile-fail doctests, pinned
workspace/five-library Wasm lints, release portability/frontend Wasm builds, native
portability and all 67 Python workflow commands. Separate completed-diff delegated
review checked correctness, security/privacy, bounded work, scope and evidence; no
remaining actionable scope issue. Required final-head CI/full thread/summary audit
precedes signed expected-head parent merge. Logs:
`/private/tmp/compensation-authoring-{build,sdk,python,workspace,verify}.log`. A skipped
CodeRabbit review supplies no evidence; agent review is automated, not independent /
human/personal owner approval. Compiled Wasm does not establish browser execution.

Forty scenarios Partial, G1–G4 conditional and parent #46 incomplete. Full validator /
full/optional RP-01/03/08, reusable near-English wording/factoring, managed identities /
protected accessible editing/export/text-visual and actual user/host/browser /
independent/phase exits remain open. All eligible occurrence/provider truth/current
authority/new logical effect/key/immutable storage/registration/receiver/cutoff /
atomic handoff/all-transitive settlement/conflicting-evidence review remain host
obligations. No external blocker or phase report/Phase 2 closure is claimed.
