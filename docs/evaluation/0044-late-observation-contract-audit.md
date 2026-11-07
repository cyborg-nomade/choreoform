<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Late-observation capture and owned continuation audit

**Date:** 2026-11-07<br>
**Baseline:** Draft #46 adoption `a92e472e5f026474277aa8dd25ef32cc0b8ae386`<br>
**Plan:** [frozen audit scope](0044-late-observation-audit-plan.md), `aeb4b34`<br>
**Authority:** ADR-0018 delegated review; parent/scenario/phase gates remain open

RP-01-C needs a correlated reconciliation result before deciding whether another
call is safe; RP-01-D needs defined acceptance order and retained late history
without payment after effective withdrawal. Declared fault mapping now prevents
unmapped eligible names in core 0.4, but it does not retain an accepted wait
payload, provide fault data, resume an activity or establish actual retry safety.
A new wait spelling alone would leave protection, wire and lifecycle gaps.

## Requirements and current boundaries

| Requirement | Accepted contract / actual representation | Missing judgment or representation |
| --- | --- | --- |
| Distinguish provider business result, uncertain effect, fault and engine failure | P3/P4; exact provider states plus explicit core-0.4 source/class/use binding | Actual authenticated evidence and lifecycle transitions, not guessed fault labels |
| Accept one exact correlated observation with fresh authority | P5 requires source, observation/occurrence/input revision correlation before match; current wait sees policy-only payload facts | No actual host observation/acceptance is supplied by static reports |
| Know captured payload type and source protection | Wait declares observationType; provider result metadata supplies a result type/protection envelope | No explicit wait/source/provider relation or typed normalized terminal-fact source. A self-declared type or destination label cannot supply source provenance |
| Retain payload for later scoped work | Current wait has no result target and empty writes | Explicit reviewed capture representation, type/write/protection/dependency derivation and accepted-observation-only initialization |
| Preserve sound timer/cancellation paths | P5 distinguishes onObservation from onTimer and disables ordinary completion after cancellation | Capture cannot initialize on a timer branch or overwrite after cancellation/duplicate. Begin with a timerless capture subset unless branch-sensitive availability is implemented |
| Retain local/ancestor values after ordinary closure | P6 creates a direct surviving child before disposal; current isolated proof accepts only data-free wait/finish bodies | Local storage lifetime, initializers, source/control provenance, parent/current versus frozen values and all declared writers; no inherited values are presumed |
| Finish reconciliation or request another effect | P4/P6 require accepted evidence, same logical request/key, budget/delay/basis, current authority, cutoff and owned settlement | A static declaration cannot decide actual retry eligibility or silently reopen a closed owner. Any continuation must be explicit permitted work under a live owner |
| Read a caught fault and decide a business branch | P4 handler receives a history record, but explicitly has no IR fault-data binding port | Reviewed typed fault-data/protection/context binding; handler finish is not an activity result |
| Show both forms and close the case | Phase 1 aggregate/ADR-0009 G1–G4 | Complete checker/conformance/benchmarks, text/visual/protected accessible authoring and actual required external evidence |

The current terminal-fact wait type is a benchmark declaration: confirmed provider
output versus correlated no-effect/rejected/cancelled records. Its type can be
checked, but a record with those field names does not prove that a provider or
normalizer supports the declared source/state/protection relationship. Success
payload typing does not authenticate the original request, accepted lifecycle
state, stable key or all-obligation settlement. Unknown/acknowledged must never
be normalized into a terminal result just to make a branch convenient.

## Actual native and raw-IR probes

The original source, resource and candidate files were unchanged by these probes.
Temporary copies used the manifest's exact resource registry. Results are static
observations of tools, not generated process histories or host executions.

| Probe | Actual result |
| --- | --- |
| Add `and record in "Recorded payment result"` to Payment reconciliation wait | `lower`: `syntax at 12002..12005`, empty stdout |
| Replace ordinary Another review is needed expression with `the fact "payload"` | `check`: `policy-fact at 42368..42386`, empty stdout; policy-only facts cannot leak through ordinary expressions |
| Give an isolated invoke recovery a fresh initialized local boolean, with valid declaration/purpose/initializer identities | Static/control/invalidation checks pass non-executable; concurrency refuses `scope-lifetime-unproved` at `/body/nodes/run`, source bytes `2948..3103` |
| Route the observation successor directly to parent Record paid status | `lower`: `flow-scope at 0..0`, empty stdout |
| Explicit Root handler for noneligible provider unknown, using existing portless Remaining work | Fault routing and reconciliation availability pass non-executable. Checked selection names `provider_faults`/`remaining`; no result/data port or flow out of handler finish is synthesized |
| Add `result:payment_result` directly to raw payment_observe node | Native library structural check refuses before semantic checking; semantic wrapper returns `structure` even with no supplied resources |

Reproduce the source probes with `tools/check_late_observation_boundaries.py` and
the raw boundary with `crates/semantic-check/tests/wait_capture_boundary.rs`.
Raw local source-probe detail is `/private/tmp/late-observation-probes.json`.
One new Rust group and five Python groups retain these boundaries in required CI.
The existing package and guard-catalogue checks pass unchanged.

## Wire, policy and protection consequences

[Definition IR 0.1](../ir/definition-v0.1.md) and its closed schema admit only
kind/scope/outcomes/reads/writes/policy on wait nodes. The native typed decoder
also requires that exact variant shape. Selecting another policy-core version
cannot authorize an extra node field. A per-use result/source field requires an
explicit reviewed wire extension with old-version refusal and canonical/schema/
typed-model/native/Wasm compatibility evidence. Annotations cannot carry behavior.

A dialect-governed policy payload could instead carry per-use capture bindings,
but that is not a free shortcut: ancestor effect/wait policies currently name a
private follow-up endpoint through a narrow checked exception. Capturing a private
child cell from an ancestor policy would need exact use-site lexical authority,
read/protection closure and complete actual-use validation. Do not add a generic
private-data visibility exception or let shared wait policies capture the wrong
occurrence. A reviewed explicit per-use wire representation is easier to inspect;
its precise version/API shape remains a subsequent design decision.

P2 requires source provenance for every transferred result, with no implicit
declassification. Declaring only the target cell's protection cannot label an
untrusted observation safely. First establish exact declared observation-source
metadata: provider identity, supported terminal observation names/states, normalized
payload type and the provider's actual declared protection envelope. Keep those
premises distinct from actual source truth/current permission. Do not let an
arbitrary emitted type, protection override, normalizer name or safe flag stand
in for that derivation. Exact external bytes and inherited type artifacts remain
explicit and budgeted.

## Options and recommended sequence

| Option | Benefit | Limitation / decision |
| --- | --- | --- |
| Explicit provider terminal-observation metadata first, then reviewed per-use capture | Establishes type/source/protection premises without changing old wire or pretending live acceptance | Recommend next dependency; metadata alone does not bind a wait or retain a value |
| Add a result field or implicit write to today's wait | Short syntax change | Reject silent old-wire extension; no source envelope, timer-branch initialization or surviving-storage proof |
| Put capture bindings in versioned policy payload | Leaves wire variant stable | Viable alternative only with complete actual-use/private-context validation; no decision to bypass that work |
| Immediately resume the original activity/caller from handler or recovery | Appears to complete C quickly | Reject implicit reopening, cancellation bypass and handler-finish/result conflation |
| Return opaque safe/approved/reconciled flags from an adapter | Hides graph complexity | Reject as proof substitute; declared flags do not establish current authority, correlation, evidence or ownership |

Next freeze a narrow independent provider terminal-observation-source profile and
immutable reader. Derive terminal names/tags/type and protection from exact provider
metadata; exclude unknown/acknowledged as terminal. Reject unsupported source/type/
protection/trust overrides. Supply no observation, wait binding, runtime transition,
permission or report admission. Test actual provider states, all pins and types,
nominal output, unknown/success confusion, label independence, immutable/budgeted
metadata and unchanged current reports when unrelated bytes are supplied.

Then freeze and decide the explicit capture representation. Start with timerless
accepted-payload capture if that keeps branch initialization sound; enforce exact
source type, current authority/acceptance premises, no-declassification transfer,
node/data/protection reads/writes and ordinary/conditional availability. Add a
bounded owned-local-storage/lifetime proof before claiming surviving captures.
Finally model explicit fault data and owned continuation/cutoff/settlement for C/D.
Each changes a different premise; none alone completes a scenario. No old contract,
wire, package or source syntax changes in this audit.

Separate completed-audit automated review checked each original C/D clause against
P1–P7, exact actual source/IR/resource shapes, structural decoder/schema, data-free
survivor/zero-inherited-value reports, native refusals and all options. It identified
the wire and source-envelope gates before recommending capture implementation.
No remaining actionable audit issue. Review is delegated self-review, not independent
or individual owner approval. Require final-head CI, all comments/CodeRabbit audit
and signed exact-head merge; parent #46 stays draft and all forty scenarios Partial.
