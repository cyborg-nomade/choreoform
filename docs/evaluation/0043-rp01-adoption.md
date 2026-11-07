<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-01 explicit fault-source adoption checkpoint

**Date:** 2026-11-07<br>
**Plan:** [frozen parent scope](0043-rp01-adoption-plan.md), `69b7bf2`<br>
**Dependencies:** ADR-0042/#55 and ADR-0043/#56, pending parent/main integration

The draft now explicitly selects source 0.7 / companion 0.6 / core 0.4 and pins
`payment.fault-source.json` to the unchanged exact payment provider. Its accepted
profile maps confirmed no effect to declared `no-effect` / definitive-no-effect,
and provider unknown to declared `unknown` / unresolved. Only the former remains
eligible. The source names Payment faults explicitly; one new portable resource
declaration/row resolves it. Actual fault acceptance, evidence authentication,
current authority, correlation, retry cutoff and owned settlement stay host gates.

The controlled generator first constructs the historical source-0.6 package and
all 72 explicit invalidation links, then applies only this adoption. It regenerates
actual native IR and all ten reports. The eight-group package oracle proves the
body delta is exactly the core pin and required nullable work retry faultSource
fields; all other body records/guards/links/identities remain equal. The exact
source/ledger adoption recreates the current files from an explicit historical
counterpart. No provider/type/clock/input bytes or old IDs change.

The historical counterpart still has 56,843 bytes / 9,466 tokens and exact semantic
revision `sha256:d0bca38fb470cd0485b9a1cc22767182e6276669ca9ca131ac45d7af268269f3`.
It retains the complete 72-link comparison, old metadata cycle/quota refusals,
unbound retry reports and rejection of new syntax. New-package resources are
supplied once by exact pin; unrelated metadata does not infer a source binding.
The historical core-0.3 metadata fixture explicitly removes the newer nullable
field. The resource-corruption guard test selects its required type artifact by
identity rather than depending on inventory ordering. No production check changes.

Current actual generated evidence:

| Item | Result |
| --- | --- |
| Full source | 56,874 UTF-8 bytes; 9,470 measured tokens |
| Semantic revision | `sha256:97e5c346442d5b051e40bc91d2c3014be1bd990688324be846739c5d35a265c4` |
| Canonical IR byte digest | `sha256:c012c1493ad7b04abc5e5f81529eea6231b720a54ee6a8db867fd2e207fd546a` |
| Existing partial semantic stages | All ten pass, all non-executable |
| Applicable native disclosure | Five reports identify one exact payment retry source/provider/eligible declaration with live requirements |
| Abstract unhandled fault cases | 10: two declared source names across five checked scopes; no actual fault is asserted |
| Historical origins / isolated follow-ups | 88 / three; conditional declarations remain unchanged |
| Guard catalogue | Only candidate revision/digest repinned; all 44 cases, 46 values and expected outcomes unchanged |
| Corpus / phase status | All forty scenarios Partial; G1–G4 and aggregate Phase 1 gate unresolved |

Passed 320 Rust tests plus six compile-fail doctests, format/workspace native lint,
all five library Wasm lints and release portability/frontend Wasm builds. All CI
Python oracles, exact package regeneration/historical profile probes and focused
new/legacy metadata/binding/disclosure checks passed. Logs:
`/private/tmp/rp01-fault-adopt-{tests,clippy,wasm,oracles}.log`.
Final-head parent CI and complete parent comment audit remain required.

Separate completed-checkpoint automated self-review compared exact adoption and
historical deltas, every resource/report/digest, original graph IDs/guards/links,
source/core/schema boundaries, no implicit binding, complete source-name accounting,
retained host requirements and finite snapshot results. Test fixture adaptations
retain old proof/refusal obligations; they do not weaken a product contract. No
remaining actionable checkpoint issue. This is delegated self-review, not independent
or individual owner approval. Keep parent #46 draft/incomplete/unmerged.

Next audit/freeze the actual wait-result/fault-data and owned continuation needs
against RP-01-C/D and accepted P3/P4/P5/P6 contracts before adding fields or syntax.
The current wait can inspect a terminal-fact payload but cannot retain it for
later graph work; the wait-only surviving subset carries no storage or writes.
A bound fault name is no substitute for that data/ownership contract. Continue
independent current-phase work; complete validator/benchmarks, near-English and
visual/protected accessible editing plus actual user/host/browser/independent
requirements remain open. No Phase 2 or phase report approval is implied.
