<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Critique: useful owned inquiry before terminal facts and general commands

**Date:** 2026-11-10<br>
**Baseline:** `2cdcfccb30321597fee25528294d49da6850e0cb`, draft #46 after #73<br>
**Authority:** ADR-0018 / [ADR-0059](../decisions/0059-owned-entry-command-direction.md)<br>
**Frozen audit:** `d49a96f`, [plan](0059-owned-entry-command-audit-plan.md)

This changes tests and documentation only. Production behavior, accepted contracts,
source/wire/schema/hash/artifact domains and committed benchmark/report bytes stay
fixed. The probes invoke the actual SDK and readable lowering on synthetic static
declarations. They supply no provider observations, original accepted values,
storage, grants, receipts, accepted handoffs or live ownership transitions.

## Finding: terminal capture is a real entry boundary

The finite core-0.9 family has a timerless capturing entry. The exact terminal
source normalizer deliberately excludes acknowledged/unknown provider observations;
its confirmed tag carries the declared successful result, while definitive no-effect
cases have their own correlation record. Spelling unknown as paid cannot make it
terminal. A successful static ownership report therefore does not mean a query can
execute while its original effect remains unknown: its capturing entry still waits
for an actual permitted terminal fact. Definition coverage is not data availability
in a live instance. This limitation needs to be explicit before claiming useful
preterminal inquiry or widening command permissions.

An original irreversible provider with stable-key metadata followed by the existing
read-only query/capture/request chain passes the entire composed SDK. This confirms
the existing family restricts new work, not the original source's effect class.
The same probe inspects the actual source descriptor: the provider has an unknown
observation, but the terminal map and normalized type contain only confirmed. No
unknown event was generated, accepted or transformed into a result.

## Current evidence and exact first refusals

Seven new raw SDK groups and one readable frontend group establish these results:

| Probe | Observed result | What it establishes |
| --- | --- | --- |
| Matched original irreversible/stable-key source, new read-only work | Complete composed SDK pass | Existing conditional baseline, not actual availability or permission |
| Matched new irreversible or compensatable provider/effect/source/capture | Static pass; owned-continuation-query-effect | Class restriction is deliberate and independent of metadata mismatch |
| Valid compensation reference with otherwise read-only new work | Static pass; owned-continuation-query-policy | Changing the effect-class predicate alone would leave another real barrier |
| Wrong provider/idempotency or nonactivity compensation target | fault-source-provider / capability-effect / compensation-target | Real earlier refusal; no later ownership check is claimed |
| Query entry before the original capturing wait | Existing IR 0.4 and source 0.12 lower; retained-request-capture | Relations are representable; current semantic entry contract refuses them |
| Passive finish or local request-copy compute as entry | retained-request-capture | Dropping or moving the capturing entry is not an implicit capability permission |
| Fresh pure record as new query input | Static pass; owned-continuation-query-input | Input equality/type alone cannot counterfeit the exact accepted original request |
| Same P4 history handler and P6 surviving scope, with original request | fault-entry-creator | Distinct activation/lifetime roles cannot be combined by reusing one scope |
| No explicit request binding, with or without an initializer | Static pass; owned-continuation-request-unproved | An initializer or a caption does not supply an original occurrence-bound seed |

Matching command probes use fresh exact provider/source pins and explicit unchanged
recipient envelopes. The compensation policy is declared in its actual lexical
scope so the prior source activity is visible. It refers to the earlier activity
declaration; actual prior occurrence/effect identity and new authority/key remain
host obligations. True completion is a pure expression, not actual effect evidence.
Accepted benchmark metadata is never broadened or relabeled to obtain a pass.

The separate P4 handler case remains covered by the existing genuine core-0.9
conditional-query fault test: declared query origins stay unexplored by ordinary
fault routing and cannot borrow ordinary history-entry coverage. Same-scope P4/P6
reuse refuses even earlier. Source inspection also shows fault-entry bodies remain
ordinary-source, terminal, flat compute/decision/finish; a new command does not
inherit that proof. These later restrictions are inspected separately, not falsely
reported as executed after the earlier creator refusal.

## Smallest useful next contract

Prefer a new exact core for finite **read-only owned entry before capture** on the
existing IR 0.4 relations. Keep command/compensation generality deferred. The concrete
probe changes only a scope entry and existing flow targets:

```
owned recovery entry: read-only lookup using its original-request seed
lookup done -> wait for the original source's terminal fact
original fact observed -> local branch/finish
lookup's own surviving child -> capture/retain the lookup request separately
```

The original source, its exact request map, actual-use effect reconciliation wait,
P6 child, query source and next child remain explicit. The readable frontend already
lowers this layout with stable portable identities and no new sentence or wire
field. The current static checker correctly refuses it at its entry premise. No
concrete missing serialized relation has been found for this narrow conditional
layout; a new field would currently duplicate existing relations. This conclusion
does not claim an accepted activation occurred or that a runtime ledger exists.

A subsequent signed implementation plan must define the new core's superseded
entry premise explicitly. Retain exactly the source-bound timerless capturing wait
in the designated surviving body, matched to the actual P3 reconciliation use and
reachable from its declared entry. Do not simply allow entry work while omitting
the source subscription or making its wait unreachable. Normal conditional
availability must reject any capture-result read before that wait initializes it.
Seed only the exact readonly original accepted request after empty pre-entry write
access; no mutable ancestor data or substituted initializer is inherited.

This is work in the distinct surviving owner **only after actual accepted P6
registration/handoff**, under fresh current authority and its own open/noncancelled
cutoff. A source declaration, control edge, possible ordinary outcome, query response
or missing terminal fact does not enable it by itself. Registration must precede
the closing/cancelled owner's cutoff and disposal; missing receiver acceptance keeps
the old owner accountable. The original source can remain unresolved and must stay
in the all-actually-activated transitive ownership/subscription ledger, including
through the query's further child. A query success cannot invent the original
source's success, return its parent result, clear cancellation or settle it.

Preserve complete source/request/input/policy/fault-existence/control/provider /
creator-owner provenance and actual capability recipients at every entry and query,
including no-result queries. Queries remain one-attempt, timerless, uncompensated
read-only work with exact original-request input and supported general source.
Maintain local storage, all-declaration/lexical-child inspection, strict invalidation,
bounded graphs/shared templates and cumulative publication. Expose entry/capture
relations separately while preserving old ordinary/legacy/flat sets and old reports.
Do not merge P4 history activation with P6 ownership or infer broader fault coverage.

## Alternatives and remaining debt

Broadening effect class alone would neither enable useful preterminal inquiry nor
prove command policy/current authority, safe key/attempt/budget/conflict handling,
actual prior-effect compensation correlation or complete ownership. A fully terminal
same-scope callback cannot do new work: accepted semantics requires a separate
linked instance after terminal disposal. Ordinary parent work is a different live
path, not late continuation after cancellation/disposal. Combining handler and
survivor roles would alter activation, nearest/active fault selection, retained
inputs and lifetime rules together. A new owned-history/entry field may be useful
later, but its need must be demonstrated before versioning it into the wire.

The next raw increment must leave commands, compensation, arbitrary new request
projection, general handler entry and richer ownership open, with separate later
design/evidence. Exact existing original-request input keeps this increment narrow;
it is not a general authoring or recovery solution. If implementation discovers a
concrete representation/semantic conflict, freeze a signed refinement before
changing scope or serialized relations. Readable selection follows another explicit
profile increment. No production change is authorized merely by this audit document;
the delegated next implementation follows its own signed plan.

Local validation passed: 490 Rust tests plus six compile-fail doctests, native and
all five Wasm library Clippy checks, release Wasm portability/frontend builds,
native portability report and all 55 Python workflow commands including deterministic
benchmark regeneration. Separate completed-diff delegated self-review assessed
correctness/security/performance/scope/evidence, matched metadata and actual first
refusals, terminal/unknown distinction, representability versus current permission,
P4/P6 lifetime and complete ownership/cutoff/handoff gates. No actionable issue
remains. This is agent self-review under ADR-0018, not independent/human or personal
owner approval. Final-head CI/full comment audit/signed expected-head merge remain
before integration. A skipped bot review supplies no evidence. Logs use
`/private/tmp/owned-entry-audit-{tests,frontend,workspace,verify}.log`. No independent
human review or actual host/browser execution is claimed. All forty scenarios
remain Partial, G1–G4 conditional, parent #46 draft/incomplete. Complete validator /
benchmarks/general near-English/protected visual editing and every genuine external /
phase gate remain open; no Phase 2 or phase exit.
