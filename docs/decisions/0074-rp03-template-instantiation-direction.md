<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0074: Instantiate service bodies in their actual caller context

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-11<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

The corrected RP-03 checkpoint refuses `child-template`, without candidate IR.
Signed [plan](../evaluation/0074-rp03-template-binding-audit-plan.md) `9bca249`
precedes probes; #88 passed exact-head CI and merged before the audit. Test-only
capture identifies `authorize_call` (invoke), caller `accept_payment`, body
`authorize_service`, declared body parent `order`. The definition places that
service at root while its caller is inside the authorization branch.

The [IR contract](../ir/definition-v0.1.md) specifies child-body creation, lexical
scope and explicit port binding. Both the frontend and raw SDK enforce exact body
parent/caller scope equality; C12's compensation identity exception does not change
that rule. Direct child calls and repeated creators in the same parent pass bounded
checks. A visible sibling template is not a directly owned child body.

## Decision

Keep all production behavior, contracts and benchmark bytes fixed in this audit.
Choose source-level call-site instantiation under existing child/port contracts as
the next direction, beginning with a separately planned **bounded effectful proof**.
Prove matching capability/effect/reconciliation-wait ownership, typed ports, source /
compensation identities, protected recipient/provenance envelopes and local storage
before applying an instantiation factory to the full benchmark.

Do not simply reparent a shared scope record, duplicate capability IDs, drop a child
or bypass the raw/source guard. Reusing a body under the same declared parent can
remain shared. Different caller parents need explicit context-specific bodies or
another independently justified layout. The full case has 23 parent mismatches
among 26 direct invocations; release_a/release_b each have two caller contexts.
Seven split-child declaration parents match in the finite inventory; other full
judgments are not proved by that inventory.

## Options and consequences

| Option | Assessment |
| --- | --- |
| Remove only the frontend check | Rejected: raw C11/C12 still refuse `child-scope` |
| Treat lexical visibility/global ID presence as invocation authority | Rejected: conflates reference visibility and child creation/ownership |
| Reparent each shared record to the first caller | Rejected: other callers and lexical/protection/lifecycle contexts can change |
| Context-specific source instantiation using existing child/port contracts | Selected for bounded effectful feasibility before full restructuring |
| General cross-parent reusable-template contract | Deferred unless project requirements cannot be met by explicit instantiation; requires a separate version/decision |

Instantiation is not presumed semantics-neutral. Preserve original scenario/receipt /
effect identity and all actually activated transitive obligations. Analyze distinct
definition versus occurrence IDs, retry versus new logical effects, wait-policy
endpoint ownership, immutable request/capture/provenance storage and current
authority/receiver/cutoff/atomic handoff/settlement. Protected metadata may require
explicitly matching instance identities/revisions; no alias or declassification is
invented. Preserve all existing benchmark/history bytes until a later controlled
adoption plan permits a correction.

Five SDK and four Python/native groups supply the bounded evidence; see
[critique](../evaluation/0074-rp03-template-binding-audit.md). Full RP-03 still has
no candidate/semantic report. Parent #46 draft, forty Partial, G1–G4/full/optional /
RP-08/validator/near-English factoring/managed protected accessible text-visual /
actual user/host/browser/independent/phase exits remain open. No Phase 2 or execution.
