<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Fault entry, emission binding and owned continuation audit

**Date:** 2026-11-07<br>
**Baseline:** `805d507836c3ba23243339522f63f699f075ab7a`, draft #46 after #61<br>
**Plan:** [frozen audit](0048-fault-entry-audit-plan.md), `2790195`<br>
**Authority:** ADR-0018; no semantic contract, wire, grammar or package change

The narrow fresh-local capture proof now has conditional initialization, protection,
retained creator/owner label provenance and strict invalidation. It supplies no
fault record or permission to reopen ordinary work. Before adding typed handler
data, distinguish three things: declared provider emission metadata, an abstract
handler selection, and an actual authenticated occurrence-bound runtime fault.
Only the last can initialize a future fault-data entry under its accepted contract.

## Contract and implementation findings

[P4](../dialects/policies.md#p4-retry-and-fault-routing) gives a handler an immutable
runtime history record containing type, source occurrence, cause and input revisions,
but explicitly supplies no ordinary data binding. Handler scopes have no ports.
Core 0.5 inherits that exact rule. Both known Definition IR versions keep the same
closed scope shape; IR 0.2 changes only wait capture. Adding a core version cannot
silently authorize a new scope field. Ordinary expressions cannot read policy-only
facts, and a fresh cell does not receive history by a suggestive name or type.

Current `FaultReport` explores name/scope selection and exclusion contexts. Its
route origin is a scope declaration, not an emitting activity occurrence/attempt;
selection names owner/policy/fault/handler only. Names come from exact handler keys
and declared retry-source emissions, including noneligible ones. This is deliberate
overapproximation, not complete emission admission. It cannot initialize an exact
fault snapshot or authenticate a node/provider relation for arbitrary handler names.

Core 0.4/0.5 retry source binding is exact per actual multiple-attempt use, but its
contract deliberately requires null source and empty eligible faults for one attempt.
It is not a general emission-source binding. A noneligible provider uncertainty may
need a handler even when no retry is permitted. Future emission binding must therefore
be independent of retry eligibility, with explicit consistency when both are present.
Unrelated providers or human activities cannot inherit an emission contract merely
because a shared policy or fault spelling matches.

Exceptional availability currently assumes declared live ancestor *input* cells;
this is conditional coverage, not a fault-site history snapshot, retained results,
actual source data, fresh permission or P6 handoff. Fresh-local surviving proofs are
separate, start with explicit timerless capture and supply no handler record. The
new retained protection-provenance requirement is an immutable label premise, not
a value source. A future fault entry needs its own zero-invented-value initialization
and actual fault/control envelope. Reusing ordinary inputs or P6 capture rows would
conflate these different entry events.

## Actual boundary evidence

These are tool judgments over explicit declaration copies, not process histories,
provider observations, handler runs or host/browser executions. The committed
benchmark remains unchanged.

| Probe | Actual result |
| --- | --- |
| Read ordinary `the fact "fault"`, `"cause"`, `"sourceOccurrence"` or `"inputRevisions"` | Static checking refuses `policy-fact`, empty stdout |
| Give the selected Root handler a fresh unavailable text cell and branch on it | Fault routing passes non-executable; exceptional availability refuses `exceptional-availability-unproved` at `/body/nodes/remaining_done` |
| Add an ordinary input port for that handler cell | Static checking refuses `handler-ports`; the copy uses terminal Root closure to isolate this gate from the independently forbidden P6 follow-up ports |
| Replace the exact handler key with an unrelated declared fault name | Abstract fault routing passes, naming only owner/policy/fault/handler and retaining source/occurrence/cause/input-authentication obligations; no emission receipt appears |
| Add a retry source sentence to single-attempt Notice work | Lowering refuses `retry-binding`, empty stdout; eligibility binding is not general emission binding |
| Add `faultInput`/`faultRecord` to a scope, including null, on IR 0.1 or 0.2 | Native typed structure and semantic wrapper refuse; wrapper returns `structure` before resource admission. Independent schemas likewise reject `faultInput` |
| Treat the declared `provider unknown` emission as terminal wait data | Actual metadata remains unresolved; its mapped provider `unknown` state is nonterminal. Normalized terminal cases exclude both labels, while confirmed business `paid`/`problem` remain success payload cases |
| Jump from recovery observation directly into parent Record paid status | Existing native boundary probe still refuses `flow-scope`; handler finish still supplies no activity result or outgoing ordinary flow |

One Rust group tests both known wires and closed field/value variants. Six Python
groups retain the native source/port/entry/routing/retry/schema/classification
boundaries. The existing continuation/handler-finish and source-normalization checks
remain in the required suite. Reproduce with `tools/check_fault_entry_boundaries.py`,
`crates/semantic-check/tests/fault_entry_boundary.rs` and the prior
`tools/check_late_observation_boundaries.py`.

## Options and recommendation

| Option | Benefit | Missing requirement / recommendation |
| --- | --- | --- |
| Independent versioned work emission-source field, separate from retry | Covers single-attempt and noneligible provider faults with actual-use identity | Recommend first: exact source/profile/provider/class/name, all actual/unvisited uses and complete protection/read closure; preserve old retry semantics |
| Explicit handler-local typed entry binding on a new wire | Local target role and auditable initialization without private-owner lookup exceptions | Recommend subsequent design: exact history type/source/control envelope and complete incoming origin contexts, not an ordinary input port |
| Per-handler policy binding table into private child data | Can leave the outer wire shape stable | Requires narrow actual-use/private-context semantics like wait endpoints, plus all source and initialization proofs; adds complexity without removing any gate |
| Hidden fault facts or a suggestive initialized text/record cell | Short syntax | Reject: no exact source binding or authenticated entry, ordinary fact leakage and easy fabricated values |
| Treat existing retry binding or abstract route as a general fault receipt | Reuses current API | Reject: single-attempt/eligible distinctions and scope versus activity-occurrence identity are different contracts |
| Treat handler finish as an activity result or resume the parent | Appears to finish C/D | Reject: no result/correlation/cancellation/lifetime proof and no implicit reopening |

Freeze a separate ADR/design before code. A work-policy emission-source field can
use a new explicit core payload because policy bodies are dialect-governed. It
should apply independently of attempt count, validate exact actual capability
provider at every use, expose all declared emission classes separately from eligible
retry names and require consistency with the existing retry source when both apply.
No matching label can turn a real engine fault into a catchable provider event.
Actual emission truth and occurrence/cause/input correlation remain host premises.

Then specify the smallest exact immutable fault-history value shape and its source /
control protection. Relevant revisions/identity fields are claims to authenticate,
not source proof supplied by strings or a record type. Avoid embedding unbounded
raw inputs/results or arbitrary safe/approved flags. Do not make successful business
problems, uncertainty, cancellation and engine faults interchangeable. Keep untrusted
nominal values distinct from authenticated entry-binding evidence.

A scope-local fault-entry cell requires an explicitly reviewed wire extension if
represented in the scope record. Require normal local ownership, exact type and
source/control transfer, no ordinary initializer or declared ordinary writer that
could impersonate entry, actual incoming emitting-site/source contexts, conditional
fresh initialization/current access and strict invalidation. Preserve every old
wire/core/source pair and fail closed on unsupported incoming cases. Retained label
provenance cannot satisfy the record's value initialization. Readable authoring and
bounded native declaration disclosure follow explicitly, with tool-managed identities;
manual JSON or hidden bindings are not the ordinary-author goal.

Permitted owned continuation remains a distinct contract after that entry work.
Cancellation is monotonic: handlers cannot clear it, a closed activity does not
restart, and surviving work needs its own explicitly authorized occurrence, exact
logical request/key/input identity, evidence, budget/delay/current authority, cutoff
and accountable owner. Terminal disposal still requires all obligations/subscriptions
settled; capturing a terminal result or finishing a handler alone does not prove it.
Keep the original full RP-01 guards and synthetic package until separate reviewed
adoption. No opaque reconciled flag or fake observation can substitute for those
requirements or actual scenario/host evidence.

## Verification, review and remaining gates

Local 374 Rust tests plus six doctests, workspace lint and every required Python oracle, including exact
benchmark regeneration/historical probes, pass. Production code/artifacts are
unchanged; required final-head CI repeats native/Wasm/Python checks. Logs:
`/private/tmp/fault-entry-{workspace,clippy,oracles,python,rust}.log`.
Final-head CI/comment audit/signed merge remain required.

Separate completed-audit automated self-review checked P3/P4/P6, exact closed
scope/wait/policy shapes, scope/name routing versus actual emission, policy facts,
handler/closure ports, initialization/protection/invalidation provenance and all
native/refusal outcomes. Reviewed compatibility, licensing, finite test scope and
recommended version boundaries. No actionable audit issue remains. This is delegated
self-review, not independent or individual owner approval; inspect all comments /
CodeRabbit summaries and require the signed passing head. Parent #46 remains draft /
incomplete. All forty scenarios, complete validator/benchmarks, general authoring /
visual/protected editing and actual user/host/browser/independent/phase gates stay
open; Phase 2 does not start.
