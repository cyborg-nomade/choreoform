<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Policy decisions and transition obligations

**Status:** Accepted with ADR-0013 upon merge of PR #17. [Common notation and admission](README.md) and
[expression rules](values-expressions.md) apply throughout.

Each existing IR policy record still has `scope`, `dialect`, `body`. Its body
has the exact fields of one variant below, including `kind` and `reads:Set<ID>`.
`reads` equals the union of direct embedded expression reads, timer basis cells,
and transitively referenced policy reads, including policies reached through
actor requirements and capability declarations. Derive this closure from the
resolved reference graph, never from author-supplied `reads` alone. A node
includes that union in its declared dependencies.
Policy references have expected kinds and lexical visibility; cycles are invalid.
No variant is a general code string, callback, optional semantic annotation or
permission to execute arbitrary host code.

Predicates are E from the expression contract, with Boolean result unless stated.
All policy-only facts are immutable; absence/type mismatch is a fault and cannot
grant permission. Ordinary process expressions cannot obtain policy facts.
All actions below are requests to the later atomic transition system, not IO
performed by predicate evaluation. Histories retain refusals, old revisions,
unknown outcomes, and ownership; no accepted action rewrites prior evidence.

## P1. Actor authority and human work

`{kind:"actor",reads,permit:E}`

Context facts: `principal:Text`, `roles:List<Text>`, `authorities:List<Text>`,
`relationships:List<Text>`, `priorDecisionPrincipals:List<Text>`, and
`inputRevision:Text`. Identity/role/relationship assertions come from the
explicitly trusted identity provider at the current acceptance point, not from
an activity's untrusted reply. Requirements can combine exact membership,
identity inequality, and revision-specific separation of duties. The authenticated
principal must be nonempty. Authority is checked at assignment and again at
completion against current facts; eligibility never creates a human decision.

`{kind:"work",reads,instructions:Name,authority:ID,retry:R,
  timeout:null|Timer,completion:E,compensates:null|ID}`

`authority` references an actor requirement, not a policy. The target activity's
own human requirement or capability authority must **also** pass; intersection,
not substitution. Instructions defining required human behavior are semantic.
`completion` facts: `principal:Text`, `inputRevision:Text`, `result:T` where T is
the activity's contract-defined result type, `occurrence:Text`, and
`observation:Text`. T must be explicitly derivable from the human work's input/
result declarations or capability output; if no result cell, T is empty Record.
Provider/human result payload is type-checked before acceptance. Observation
identity, occurrence and exact input revision correlation are mandatory checks
before the predicate, not optional tests the author can omit.

Human work states/transitions: offered → assigned (eligible principal chosen
and recorded); assigned → accepted (that principal acknowledges); accepted →
completed (fresh correlated result, current authority, true completion predicate).
Offered/assigned/accepted may become refused by the responsible eligible actor,
withdrawn by an authorized cancellation, or expired by the declared timer.
Reassignment creates a new assignment revision, invalidates earlier assignment
completion authority and retains history. Completed/refused/withdrawn/expired
do not accept another ordinary completion. Late replies are recorded, never
silently applied to another assignment. A timer can expire/escalate technical
work but cannot choose a business/expert outcome. Mapping an accepted result to
a declared ordinary outcome is explicit typed variant tagging: for human work
T must be a Variant whose tags equal the node outcomes; an empty-result human
activity with multiple outcomes is invalid. A single-outcome empty-record result
emits that sole outcome. Faults/refusal/expiry remain lifecycle faults, not invented
ordinary results; the explicit fault policy governs continuation.

Overrides/waivers, when modeled, are separate human activities with semantic
typed result fields `actor`, `authority`, `reason`, `scope`, `revisions`, `expiry`;
they do not mutate this policy or erase the original refusal. The initial
profile has no generic bypass/waiver flag. `compensates` names an earlier effectful
activity declaration; runtime must additionally bind the precise prior effect
occurrence, record new authority and create a new logical effect. It cannot erase
that earlier effect or reuse its idempotency key for inverse work.

## P2. Protection and transfer

`{kind:"protection",reads,permit:E}`

Facts: `principal:Text`, `purpose:Text`, `operation:Text` in
`read|write|transfer`, `cell:Text`, `revision:Text`, `capability:Option<Text>`.
The existing data envelope remains mandatory: current principal satisfies at
least one permitted participant requirement for participant access; a capability
access names a permitted capability and also satisfies its declared authority.
Purpose must be in the nonempty declared set. Empty relevant access sets deny.
Every applicable predicate must be true; unsupported enforcement denies admission.

Initial information-flow rule is conservative, with **no declassification**:
for every source dependency and every destination of a compute/transfer/result,
require identical sensitivity tokens and identical resolved protection-policy
identity; destination purposes, participant-ID set and capability-ID set must
each be subsets of the corresponding source set. Independently check the permit
predicate at each source/destination with the actual operation context. Include
control dependencies and all static expression branches, not only evaluated reads.
Participant-set comparison uses exact requirement identity, not guessed semantic
implication between predicates. Distinct policy identities cannot be assumed equal.

Literals add no process-data provenance; copying or nominal conversion never
removes existing provenance. External capability results require a declared
protection envelope from the provider contract; subject it to the same transfer
rule. A target lacking protected metadata/diagnostic handling must refuse the
plan. This restricts useful cross-policy flows; a future declassification
contract needs separate explicit authority/evidence, not an implicit cast.

## P3. Capability and effects

`{kind:"capability",reads,provider:Ref}`

Provider contract bytes are separate exact-byte-hashed strict JSON resources
with fields `{kind:"provider-contract",version:"0.1.0",id:Name,input:T,
output:T,outcomes:Set<Name>,resultProtection:P,observations:Map<Name,O>,
idempotency:"none"|"stable-key",effectClass:"read-only"|"compensatable"|"irreversible"}`.
P has the existing protection-envelope shape with references resolved in the
definition using this contract; each referenced identity must be supported and
visible, making this initial provider resource definition-context-specific.
O is `{state:"acknowledged"|"succeeded"|"rejected"|"cancelled"|"unknown"|
"no-effect",result:null|T,outcome:null|Name}`. Only succeeded may carry an
ordinary outcome and result; it must name one declared outcome, its result type
must equal output, and every declared outcome must have a succeeded observation.
All other observations have null result/outcome. Rejected/cancelled/no-effect
are definitive no-effect facts only if the supported adapter attests that
contract meaning; an HTTP timeout or transport error is not such a fact.
Capability record input/output must equal provider types; activity outcomes
must equal provider outcomes. The adapter must prove operational support for
the exact contract at planning; a self-asserted string is insufficient.

`{kind:"effect",reads,class:"read-only"|"compensatable"|"irreversible",
idempotency:"none"|"stable-key",reconciliation:ID}`

Fields must match the referenced provider contract. `reconciliation` names a
wait policy accepting correlated provider facts; the planner requires a reachable
declared reconciliation path and owned follow-up, not just a dangling reference.
Idempotency is never an engine guarantee of provider behavior: stable-key mode
requires an adapter whose exact provider contract promises that equal logical key
and request digest cannot create another logical effect. Key = exact instance,
activity occurrence and logical effect identity, stable across technical attempts.
Different request bytes with the same key are invalid. New business work and
compensation get new logical keys. The key is an opaque tuple, not unchecked
string concatenation. Persistence/transport encoding is later engine work.

Effect transitions, each retaining prior evidence:

| Current | Accepted cause | Next |
| --- | --- | --- |
| planned | Authorized dispatch with recorded request/key | requested |
| requested | Correlated provider acknowledgement | acknowledged |
| requested/acknowledged | Supported success observation | succeeded |
| requested/acknowledged | Supported definitive rejection/cancellation | rejected/cancelled |
| requested/acknowledged | Timeout, crash ambiguity or ambiguous provider reply | unknown |
| unknown | Correlated reconciliation success/rejection/cancellation | corresponding confirmed state |
| unknown | Supported definitive no-effect observation | rejected, with no-effect evidence |

Duplicate observation ID records delivery but makes no second transition.
Unknown is not failure. A cancellation request alone never changes an external
effect to cancelled. Confirmed terminal state cannot regress; contradictory
later facts create an explicit reconciliation conflict obligation while retaining
the confirmed record, not a silent overwrite. Dispatching a permitted retry
creates a new attempt for the same logical effect; unresolved attempts remain
correlated and visible. Late success of one attempt settles the logical effect
once and prevents further dispatch; technical duplicates do not create new work.

## P4. Retry and fault routing

Retry record R is `{maxAttempts:N,delays:List<Duration>,faults:Set<Name>}`.
N is JSON integer 1..1000 and counts the initial attempt. Delays has exactly N−1
nonnegative signed-64 coefficient strings in microseconds; element i−1 is the
delay before attempt i+1. No random jitter or implicit exponential algorithm.
Fault names match exact admitted fault identities, not substrings. Pure/internal
faults do not become effect retries automatically.

Permit a new attempt only if the preceding attempt ended in an eligible fault,
attempt count is below N, its delay has elapsed on the pinned timer basis,
current authority still passes, and cancellation/ordinary closure has not blocked
new work. If an effect is unresolved, additionally require stable-key support
with identical request digest, **or** definitive correlated no-effect evidence.
Satisfaction of timing/attempt limits alone is insufficient. Missing evidence
leaves the effect unknown and routes to owned reconciliation; do not guess failure.
Succeeded effects never retry; definitive rejected/cancelled work retries only
where the exact fault and confirmed no-effect evidence satisfy this rule.
Every retry keeps the original request and input revision identity, including
after no-effect confirmation. Changed inputs require a new activity occurrence
and logical effect, not reuse of the old attempt budget or key.

`{kind:"faults",reads,handlers:Map<Name,ID>}`

Each exact fault type maps to a local child scope template with no ports. The
nearest lexically enclosing policy containing that exact key handles it; no
wildcards or subtype matching initially. Handler invocation receives the fault
as an immutable runtime record (type, source occurrence, cause, input revisions)
for history; because there is no IR fault-binding port, this profile's handler
cannot read that record as ordinary data. Policies can select on the exact type
only. A later typed fault-data binding requires a reviewed IR extension.
Unhandled faults fail the owning scope under ADR-0008; completed effects remain.
If a handler faults, search starts at its lexical enclosing scope excluding the
handler selection that just failed, preventing accidental self-reentry. Handler
selection/invocation graph must be acyclic. Engine faults are not catchable
business outcomes. A handler cannot clear parent cancellation to resume work.

## P5. Time, waits and observations

Duration is a nonnegative signed-64 string in microseconds. Clock instants are
signed-64 strings in a single explicitly pinned clock domain; no cross-clock
comparison or machine-local clock read is implicit. A clock resource Ref must
identify an implemented immutable contract specifying origin, microsecond tick
interpretation, trust source and observation monotonicity. This suite does not
standardize a clock provider. All clock facts enter as accepted observations;
backward/stale clock facts are refused rather than silently making time negative.

Timer is `{clock:Ref,basis:ID,due:E,calendar:null|Ref,pauseAuthority:ID}`.
Basis names the data cell holding the versioned calculation basis; due returns
Integer in that clock domain. Its expression reads plus basis are dependencies.
The actor requirement at pauseAuthority controls pause/resume/reschedule; record
reason, principal and basis revision. A timer revision stores calculated due,
clock and calendar pins, basis/data revisions and accumulated pause intervals.
Reschedule creates a new revision, invalidating an old pending fire. At acceptance,
fire only if revision matches, no active pause, clock matches, and observed instant
is at least effective due. Eligibility alone does not commit ahead of cancellation.

An authorized pause stops elapsed measurement only for its named timer revision.
For this profile, completed non-overlapping pause durations shift due later by
their exact sum; overflow faults, overlapping/nested pauses are invalid. Resume
requires the same pause record identity and nonnegative elapsed duration. Active
pause blocks firing. For an absolute deadline that must not shift, use an actor
requirement that denies pause/resume. No global pause or implicit extension.

Calendar resource schema: `{kind:"calendar",version:"0.1.0",id:Name,clock:Ref,
zone:Name,zoneRules:Ref,intervals:List<{start:Integer,end:Integer}>}`; exact stored
bytes hashed. Intervals are sorted, non-overlapping, nonempty half-open ranges
with start < end in the pinned clock domain. A non-null calendar permits firing
only inside one interval at or after effective due. Outside all supplied future
intervals the timer remains pending; it must not invent next-day business hours.
zone/zoneRules record immutable provenance of the precomputed intervals; the
runtime does not convert local times or guess daylight-saving rules. Calendar
construction/trust must be supported by the supplied host contract, not an
arbitrary uploaded schedule. A calendar revision change requires explicit
authorized reschedule, never changes an existing timer implicitly. Rich business
duration arithmetic and calendar-generation algorithms are not implemented here.

`{kind:"wait",reads,observationType:T,match:E,authority:ID,
timer:null|Timer,onObservation:Name,onTimer:null|Name}`

Match facts: `payload:T`, `observation:Text`, `occurrence:Text`,
`inputRevision:Text`, `principal:Text`. The observation's authenticated source,
correlation to the exact waiting occurrence and revision, and one-time identity
acceptance are mandatory before match. authority references an actor requirement
and is rechecked at acceptance. True emits onObservation (a declared node outcome)
once; false records unmatched input and keeps waiting. Timer present iff onTimer
non-null; its outcome must be declared and different. All wait node outcomes
must be these exact names. Late/duplicate/stale messages remain recorded without
emitting again. Cancellation disables ordinary wait completion, but explicitly
owned reconciliation waits remain available per closure/cancellation policy.
At non-node use sites (effect reconciliation), outcome names must be declared
by the owning reconciliation wait node; an unattached wait policy is insufficient.

## P6. Cancellation, settlement and closure

`{kind:"cancel",reads,authority:ID,children:ID}`

Authority references an actor requirement; children references settlement policy.
After an authenticated permitted request, cancellation is monotonic: stop new
ordinary work, propagate to current ordinary children, and ask cancellable work to stop.
The reconciliation child explicitly named by a `reconcile` closure policy is
a separate surviving ownership boundary, not an ordinary child for this rule;
it may process only its declared reconciliation/settlement work with fresh
authority checks. No other child is implicitly shielded from cancellation.
It remains requested while settlement is pending. Requests/late observations are
recorded; external success may still arrive. Reconciliation/compensation work is
new explicitly authorized work in the declared surviving ownership boundary, not
an exception permitting arbitrary ordinary work in the cancelled scope.

`{kind:"settlement",reads,unfinished:"retain"|"cancel"|"transfer",
owner:null|ID}`

Retain keeps each unfinished obligation under its current live owner; owner must
be null. Cancel requests cancellation, retains ownership until acknowledged
settlement, owner null. Transfer requires non-null owner naming a declared live
ancestor scope capable of accepting the obligation under its policy, not the
closing/cancelled scope itself or its cancelled descendant. Transfer commits only
with receiver acceptance and atomic removal/addition from the old/new ownership
sets. No accepted transfer means still pending under the old owner. There is
exactly one accountable owner at every committed state. Owner identity includes
scope occurrence, not merely a definition ID. A vanished/terminal receiver fails.
Receiver acceptance is a correlated observation naming the exact obligation set
and source/target occurrence revisions, authorized by the receiver's cancellation
authority (its scope-management actor requirement). Recheck that authority and
the receiver's live/non-cancelled state at atomic transfer; a source request or
host-supplied target name alone is not acceptance.

`{kind:"closure",reads,mode:"terminal"|"reconcile",followup:null|ID}`

Terminal requires no owned obligations or subscriptions and followup null.
Reconcile requires followup naming a declared child reconciliation scope whose
occurrence remains live and owns every outstanding subscription/effect obligation
at ordinary closure. The instance stays nonterminal; the ordinary scope cannot
claim full disposal. Closure checks reject missing, duplicate or ownerless
obligations; it is not enough for a join predicate to be true. A cancellation
outcome likewise waits for settlement or accepted transfer outside the closing
scope. Finish may publish ordinary outcome only after this ownership check.
Fully terminal instances cannot reopen; later facts start/link another instance.

Create the declared reconciliation child occurrence when the first reconciliation
obligation is registered, before ordinary closure/cancellation can dispose of
its current owner. The follow-up scope has no ports in this initial profile;
its entry and wait policies explicitly describe permitted reconciliation work.
Transfer into it still requires the acceptance and permission checks above;
until accepted, the old owner remains accountable and closure stays pending.
At most one such follow-up occurrence exists per owning scope occurrence.

## P7. Races, invalidation and dynamic membership

`{kind:"race",reads,mode:"acceptance"|"guarded",invariant:null|E}`

Acceptance mode requires null invariant and uses engine acceptance position,
not provider timestamp. It is invalid for a consequential race without explicit
protective evidence; an author cannot certify safety with a Boolean “safe” flag.
For this initial profile use guarded mode for any race that can request an
external effect: invariant is a Boolean expression checked against the current
snapshot immediately before every conflicting commit. Static admission must
establish that the invariant and read/write dependencies cover all conflicting
paths; otherwise refuse. Every conflict forces reevaluation. This does not prove
an arbitrary invariant prevents all unsafe behavior: unproved irreversible-race
safety blocks planning. General pure race resolvers/commutativity proofs require
a later supported policy extension, not fallback to scheduler order.

The existing data.invalidates set has operational force: a new revision marks
the named dependent decisions/work insufficient for future transitions using
the revised evidence. Retain their records and prior effect outcomes. Active
work receives explicit withdrawal/cancellation under its lifecycle; renewed work
uses fresh occurrences. It does not automatically undo effects. Static dependency
analysis requires invalidation coverage for decision/effect-relevant changed
inputs; missing coverage blocks admission rather than assuming approval reuse.

`{kind:"membership",reads,removal:ID,changed:"invalidate"}`

Removal references settlement policy. Before seal, compare stable item keys:
duplicates fault before creating children; new keys create distinct child
occurrences; removed keys invoke removal settlement without deleting evidence;
retained changed values publish new item revisions and apply declared
invalidation. Reordering alone preserves keyed occurrences. Seal is a Boolean
process expression and, once committed true for an occurrence, freezes membership
and allows its join to evaluate. Post-seal additions/removals belong to a new
explicit occurrence, not an implicit reopen. Resource limits may postpone
scheduling but cannot shrink membership or erase pending children.

## P8. Boundaries still requiring evidence

These closed rules are the accepted initial policy vocabulary, not a supported
engine. A target must support every used contract, provider/clock/calendar trust
boundary, transfer and invariant check or refuse. Where a host contract is not
yet supplied, that definition is not executable. The first profile deliberately
omits general declassification, arbitrary calendar arithmetic, generic fault
payload bindings, scheduler-only irreversible races and extensible code hooks.
Full executable benchmark fixtures, comprehensive validators, provider/identity
adapters and runtime traces remain explicit downstream work; policy strings
alone do not discharge them.
