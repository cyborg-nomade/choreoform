<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Critique: owned continuation, cutoff and all-settlement

**Date:** 2026-11-08<br>
**Baseline:** `e5a975af1ac1a5dad2a4cf6a5be506dc331e36f6`, draft #46 after PR #70<br>
**Frozen plan:** `f734d3e`, [audit plan](0056-owned-continuation-audit-plan.md)<br>
**Evaluator:** Codex under ADR-0018 delegation; author/evaluator are the same agent

## Finding

Original request retention is raw/readable and conditionally checked. New capability
work in a surviving body is still unsupported. Removing the owned checker's flat
node-kind restriction would be incorrect: every capability use also needs its own
reachable designated reconciliation child. That child creates another surviving
storage/source/ownership boundary, which the current terminal flat retention
contract deliberately refuses. Earlier static admission does not close those gates.

The next bounded extension should support explicit finite nested surviving ownership
for read-only capability work first. Preserve every old contract; define a new
exact core on the existing explicit IR 0.4 shape, with a distinct conditional proof.
No production behavior, wire/core/source/artifact/package changes occur in this
audit. [ADR-0056](../decisions/0056-owned-continuation-direction.md) records the decision;
a separate signed implementation plan and profile must precede production changes.

## Actual probes

Seven [SDK probe groups](../../crates/semantic-check/tests/capture/continuation_audit.rs)
run real checks on small synthetic declarations derived from the existing capture
fixture. They use new provider/source IDs and exact pins, with explicitly declared
synthetic protection recipients. They do not modify accepted fixture/artifact or
RP-01 bytes, send requests, instantiate storage, grant authority or generate receipts.
The command variant's exact provider class differs even though its declaration ID
still contains lookup; names and a true completion predicate do not establish safety.

| Probe | Actual SDK result | Implication |
| --- | --- | --- |
| Flat capture + original request, no new work | Complete composed chain passes; retained-request scope remains separate | A real supported baseline, not a fabricated failing input |
| Read-only capability work inside terminal flat follow-up | Static/control/protection/invalidation pass; concurrency refuses `scope-lifetime-unproved` at `/body/nodes/run_owner` | Earlier passes do not supply owned lifetime/protection/dispatch proof |
| Give that follow-up a designated nested recovery child | Static refuses `retained-request-creator` | Current core 0.8 deliberately reserves flat terminal storage; nesting needs a versioned contract |
| Pure local copy with/without original-request binding | Full chain passes with binding; without it concurrency refuses `owned-local-availability-unproved` | A local name or initializer cannot invent the original value |
| Replace retained read with ancestor value | Static passes; concurrency refuses `scope-lifetime-unproved` | Normal visibility is not surviving storage or inherited availability |
| Put a copy on the ordinary parent's success path | Full chain passes and marks it ordinarily covered, with no cross-scope edge | This is different behavior before ordinary closure, not a callback resumption path after cancellation/disposal |
| Exact irreversible/stable-key provider with true completion | Static passes; concurrency still refuses `scope-lifetime-unproved` | Metadata parsing neither authorizes command work nor proves its settlement |

The earlier passes cover their documented declaration/ordinary subsets; the query
node is explicitly outside ordinary coverage. They do not establish its conditional
publication or live access. Provider classifications remain declarations of what
a supported adapter must truthfully implement, not evidence that an adapter ran.

The flat read-only shape has a second independent incompatibility: the existing
[reconciliation link checker](../../crates/semantic-check/src/reconciliation.rs)
requires the activity owner's closure to be reconcile, its wait to be in that
designated child, and a reachable declared route. A same-scope wait under terminal
closure cannot satisfy those predicates. The composed API refuses earlier at
concurrency; this additional constraint is a source inspection finding, not an
invented second observed error.

## Contracts that must stay connected

[Manifest](../../MANIFEST.md), [core semantics](../decisions/0008-core-process-semantics.md)
and [P1–P7](../dialects/policies.md) already require the following distinctions.

| Boundary | Existing rule and required continuation consequence |
| --- | --- |
| P1 current authority | New reconciliation work has its own current actor/work/capability authority. Old request, approval, role label or metadata hash is no current grant |
| P2 protection | Retain original input, policy, source/fault-existence, control, provider and creator/owner labels through every handoff. Converting them into provenance cannot initialize ancestor values, erase recipient restrictions or cache permissions |
| P3 effects | Each new request has a distinct logical effect/key identity and its own exact accepted input; technical retries alone reuse the original request/key. Read-only classification is an exact supported provider claim, not a friendly capability name |
| P4 faults | Keep ordinary/conditional source contexts, nearest live selection and active exclusions distinct. Existing history entry accepts only its supported ordinary contexts; a new conditional query source cannot borrow that coverage or disappear from possible incoming relations |
| P5 observations | Every capturing entry retains its exact provider/source/origin/type and live evidence/correlation requirements. Late, duplicate, stale or contradictory delivery cannot confirm twice, create an ordinary parent token or reopen a terminal instance |
| P6 ownership | Register the designated surviving child before current-owner disposal; receiver acceptance and atomic old/new ownership updates are mandatory. Failed or missing acceptance leaves the original owner accountable and closure pending |
| P6 cancellation | Cancellation is monotonic. A surviving child is a separate declared boundary, not permission to restart its cancelled parent. Its own cancellation/closure cutoff must prevent new ordinary work inside that child, while its designated surviving descendant retains already registered obligations |
| P6 disposal | Ordinary outcome/closure is distinct from terminal disposal. Intermediate reconcile boundaries retain all activated effects/subscriptions/children until accepted handoff; a terminal leaf requires all live transitive obligations and subscriptions settled |
| P7 guards/races | Keep complete local read/write/access/invalidation analysis. A true predicate or source-count table cannot establish absence of an irreversible race or prove a live ownership ledger is empty |

Source requests can fail or remain unknown. A confirmed terminal capture is useful
only under its exact authenticated mapping/correlation/authority premises. It is
not universal evidence that every obligation of the owner disappeared. Declaration
IDs and even exact metadata hashes never supply scope occurrence identity, receiver
acceptance, current permissions or an atomic ownership receipt.

## Alternatives and recommendation

| Option | Benefits | Costs and decision |
| --- | --- | --- |
| Loosen only flat node kinds | Small patch | Omits nested reconciliation, storage, conditional source context and all-settlement; reject |
| Same-scope reconciliation wait under terminal closure | Avoids another lexical boundary | Requires new activation/ownership/cutoff semantics and cannot reuse existing P5/P6 link proof; defer |
| Explicit finite nested surviving ownership, read-only work first | Reuses explicit scope/closure/source relations and makes the transitive owner visible | Requires a new exact core and distinct bounded conditional proof; adopt next |
| General command/compensation work immediately | Broader business coverage | Adds irreversible guards, key/budget/compensation and conflict obligations before the ownership gap is closed; defer explicitly |
| Ordinary-parent continuation | Uses an ordinary success graph | Cannot implement late work after cancelled/disposed parent; reject as a substitute |
| Treat capture or finish as all-settlement | Easy positive result | Hides live transitive obligations/subscriptions and invents host evidence; reject |

IR 0.4 already represents the necessary scope/closure, capability/work/provider,
wait/capture, original-request, actor/protection and dataflow relations. A new wire
field is not justified solely to recognize this narrower ownership family. The
new core must explicitly select the supported existing wire and preserve every
older profile's fixed rules. Add a wire only if the implementation identifies a
concrete missing serialized relation; sign a plan refinement first.

## Concrete next implementation requirements

Use a distinct immutable conditional continuation report; do not broaden the old
flat OwnedLocalReport's meaning or ordinary/legacy checked sets. A covered ordinary
synchronous creator starts the supported family. Each continuation body uses its
explicit exact original request and timerless capturing entry. At most one new
read-only capability activity per intermediate continuation body has its own direct
designated surviving child; terminal leaves remain passive flat local bodies.
Permit no arbitrary nested scopes, ports, ordinary parent outputs/resumption,
human work, commands, forks/fanout/repeat/invoke or hidden ancestor value retention
inside this first continuation family. Check every declaration, including unvisited
ones; unsupported or unproved source contexts must refuse or remain explicitly
uncovered without producing an ownership/availability proof.

Keep normal exact provider/type/visibility and complete read/write rules. Require
new work's exact read-only provider/effect contract, single-attempt bounded work,
explicit supported general fault source and current authority premises. Initially
use the exact retained original request as the query input rather than re-evaluating
ancestor input; a new logical request identity remains separate. Require compatible
recipient protection for the complete source/control envelope, not just the input
cell's apparent type or the new provider's lower bound.

Index the finite ownership tree once. Analyze availability and protected flow in
owner-before-child order: a nested source may rely on a previously proven conditional
owner context, never an empty fallback or an ordinary-coverage claim. Check write
access before each seed with zero inherited live values, then local initialization,
capture/new work/branch/publication and strict invalidation. Preserve actual source
node/provider provenance and declaration-bound labels through each new boundary.
Keep all pending/copying/index/flow/report work cumulative and charge copies before
publication, with bounded depth and no partial proof. Share templates/metadata, not
occurrences, values or grants. The old no-continuation path must retain its cost /
behavior where possible; avoid recursive public admission for each body or output.

Publish declaration-level registration/handoff/closure/terminal-settlement gates
with exact source/query/capture/owner/child relations. They require accounting for
**every actually activated transitive obligation and subscription**, not merely
counting query declarations or a successful join. Finite describes the declared ownership structure, not guaranteed termination or
eventual provider settlement. A host must retain fresh immutable
storage/provenance, correlate exact definition/instance/source/owner/effect/key /
request/value/input revisions, enforce current authority and open/noncancelled
registration cutoffs, and commit receiver acceptance/ownership atomically. A failed
handoff remains pending under the old owner. No report proves that ledger empty.

Required negative cases include terminal-with-new-work, missing/multiple/wrong
child/source/request, conditional-unvisited source, ancestor/peer/self access,
constant/initializer/branch/provider egress leaks, missing invalidation, foreign
writers/creators, ordinary parent resumption, hidden effects, command classification,
retry/timer/nested-ordinary work, unproved fault contexts, shared-occurrence values,
and cumulative graph/context/copy/report exhaustion. Reuse current raw/source/native
compatibility evidence. Readable selection follows a separate reviewed profile.

## Evidence and remaining work

Seven actual probe groups pass. Local validation passed: 458 Rust tests plus six
compile-fail doctests, native Clippy, five Wasm library lints, release Wasm
portability/frontend builds, native report and every Python workflow command,
including deterministic benchmark regeneration. The final ordinary-coverage
assertion also passed the focused probe suite and semantic-check Clippy.
Final-head CI/comment audit/signed expected-head merge remain required. Local logs
use `/private/tmp/owned-continuation-audit-*`. Separate completed-diff delegated self-review checked correctness/security /
performance/scope/evidence and preserved production behavior/artifacts. It clarified
that earlier protection passes are ordinary/declaration-only and a finite structure
is not a liveness guarantee; no actionable issue remains. This is agent self-review,
not independent/human or personal owner approval. A skipped bot supplies no review
evidence; all current threads and summaries still require audit.

No external evidence is needed to define/test this restricted static family, so
continue independent Phase 1 work after the audit. Real original requests, provider
behavior, runtime storage/cutoff/handoff/all-settlement, host/browser execution,
representative-user studies and independent review cannot be fabricated. The next
implementation must leave those live premises explicit. Complete validator /
RP-01/03/08, general tool-managed near-English/protected accessible visual editing
and every scenario/phase gate remain open. Committed RP-01 and all accepted artifacts
stay unchanged; draft #46 remains incomplete, all forty scenarios Partial, G1–G4
conditional. No Phase 2 or phase exit.
