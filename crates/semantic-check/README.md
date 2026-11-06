<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Static contract checks, not executable admission

[Core 0.3.0](../../docs/dialects/core-metadata-v0.3.md) adds finite provider-result
metadata read closure under [ADR-0035](../../docs/decisions/0035-provider-metadata-read-closure.md).
The strong graph remains acyclic; cyclic components containing legacy policies
refuse. The pure `policy_read_closure` helper returns sets/work, not validated
policies or authority. Callers validate exact pins/visibility/eligibility and charge
its work against their existing remaining budget. [Source 0.6](../../docs/authoring/profile-v0.6.md)
selects this core explicitly; earlier profiles keep their original pins.

Static scope/input, actor, data protection/initializer/invalidation and capability
errors include record/field paths without changing their codes. Existing deeper
paths are retained; global resource/structure/revision and graph failures are not
assigned guessed declarations. The frontend maps known paths to original source
spans; see the [diagnostic guide](../../docs/authoring/diagnostics.md).

`check_reconciliation_availability` follows declaration links with a conditional
body judgment for each actual reconcile-closure use. It uses zero inherited cells,
fresh locals and one shared graph/index/budget. Parent input/initializer/result
values are not assumed retained across the surviving boundary. Conditional body
coverage remains separate from actual activation, live storage/current authority,
protection/invalidation/concurrency and accepted ownership/correlation/settlement.
See [ADR-0032](../../docs/decisions/0032-conditional-reconciliation-availability.md).

`check_reconciliation_links` follows conditional handler availability and verifies
each declared capability activity's own reconcile closure, direct child follow-up
and reachable unique wait site. It records exact declaration references, unvisited
sites and unused effect contexts; it does not prove activation, accepted ownership,
provider correlation, wait progress or terminal disposal. Read-only provider
metadata supplies no exemption. See
[ADR-0030](../../docs/decisions/0030-reconciliation-declaration-links.md) and
[direct-IR/source-boundary evidence](../../docs/evaluation/0030-reconciliation-declaration-links.md).

`check_exceptional_availability` checks routing-discovered handler bodies under
explicit ancestor-input premises, with fresh locals and no borrowed ordinary
result/initializer snapshot. It separates conditional body coverage from earlier
ordinary/routing coverage; current activation/revisions/authority, fault-control
protection and ownership remain unproved. A private per-scope initializer index
replaces repeated full-data scans in availability/protection without caching values
or policy truth. See [ADR-0029](../../docs/decisions/0029-conditional-handler-availability.md)
and [evidence](../../docs/evaluation/0029-conditional-handler-availability.md).

`check_fault_routes` follows timing/ordinary checks with an indexed conservative
handler-selection graph and active failed-selection exclusion. It considers all
declared fault names, refuses potential cycles as unproved and reports abstract
route/selection coverage. This does not initialize/authenticate handler contexts,
classify actual engine faults, prove concurrent occurrence lifetime or dispatch
work. Route views borrow one immutable metadata table. See
[ADR-0028](../../docs/decisions/0028-conservative-fault-routing.md) and
[evidence](../../docs/evaluation/0028-conservative-fault-routing.md).

The [value-expression conformance corpus](../../docs/conformance/README.md)
exercises exact production static/invariant-evaluation judgments in three Rust
test groups. Its 69 vectors cover all 29 E1 operations at catalogue level, with
explicit invariant-context refusals for process read/policy fact. It introduces
no public process evaluator. The
[gap audit](../../docs/evaluation/0028-validator-gap-audit.md) records what is
implemented, conservatively unproved, missing statically or required of a host.

Settlement transfer dependencies include the receiving ancestor's actual
cancellation/management actor requirement, as required by P6. This does not
include its children-settlement policy or establish current receiver acceptance.
Policy dependency mismatches point to the declaration's `body/reads`; policy
closure charges edge/member copies. See the
[correction evidence](../../docs/evaluation/0019-settlement-authority-dependencies.md).

Core 0.2.0 adds explicit retry clock/basis fields while inheriting the exact core
0.1.0 artifact. Records select their own dialect; old records are not upgraded.
`check_timing().retry_uses()` exposes immutable bindings/attempts/delays and live
host requirements, never dispatch permission. Distinct retry-clock labels apply
before work result writes and survive null results alongside timeout/activity
origins. Source 0.2.0 retains its old payload; the separate
[source 0.3.0 profile](../../docs/authoring/profile-v0.3.md) expresses the new binding.
See [ADR-0026](../../docs/decisions/0026-explicit-retry-clock.md) and
[evidence](../../docs/evaluation/0026-explicit-retry-clock.md).

Accepted [ADR-0016](../../docs/decisions/0016-static-contract-checks.md) implementation
of closed ADR-0013 judgments. Rust remains the shared implementation language.
No IO, network, clock, authentication, dispatch or process interpreter is provided.

## Stage boundaries

`check(&DefinitionEnvelope, explicit_resources)` reuses selected structural checks,
verifies revision and exact semantic/core-role pins, then checks closed type,
expression and policy payloads plus selected links/types/dependencies. The private
`Report` fields expose only revision, record counts and `remaining()` obligations.
It cannot be converted to a plan or executable definition. The original immutable
envelope is neither rewritten nor upgraded to a full-admission capability.

`Resources` is an explicit bounded local registry. `type_descriptor`, `literal`
and `expression` are judgment helpers, not whole-definition admission; `Context`
contains explicit static type bindings, never authenticated host values. Named
types retain exact artifact ID/digest/name identity. All referenced and loaded
named declarations are checked, including unused siblings; cached partial work
does not turn a failed resource into success on retry.

The type/value and expression vocabularies are the accepted closed forms, not
host code. Static checks visit dead branches, exact result types and complete
reads. Nominal literals check representation and Boolean invariant. A private
bounded evaluator exists only for statically checked pure nominal invariants;
it has no data/fact/IO lookup and uses exact checked arithmetic and rescaling.

All twelve policy variants have closed schemas and typed predicate contexts.
Work completion is checked at each activity's result type; null publication uses
empty Record without changing a provider's output declaration. Policy reads equal
the full closure through predicates and referenced policies/actors/capabilities,
including provider result-protection references. Data-access protection can
introduce additional node dependencies; candidate lowering expands these to a
fixed point and static checking verifies the result. Policy cycles/depth refuse.

Selected node checks cover parameter sites, expression/input/result/assignment
types, guard/key/seal types, local/direct-child interfaces, human/provider outcomes,
policy kinds and declared reads/writes. Direct source/destination, invocation-output and provider-result
transfer checks use identical sensitivity/policy identity and narrowing access sets.
They do **not** prove control-sensitive whole-process information flow or access
authorization, reachable reconciliation, definite outputs, race safety or
invalidation completeness. Those residual gates are explicit in the report.

## External readers and unsupporteds

- `named_types`: strict exact-byte format, finite exact nominal dependencies,
  pure typed invariants; no ambient reads or facts.
- `provider`: strict complete observations, total declared outcomes, exact
  input/output/result descriptors and effect/idempotency declarations. Referenced
  protection identities are additionally checked in the consuming definition.
  This does not attest adapter behavior, identity, no-effect facts or stable keys.
- `calendar`: sorted, non-overlapping half-open signed-64 intervals, zone provenance
  and exact supplied clock/zone-rule bytes. It does not interpret those opaque
  provenance bytes, generate intervals, authenticate the calendar or prove clock
  support. Empty tables simply contain no eligible interval.

The accepted suite leaves clock providers external. ADR-0023 defines the first
separately pinned clock profile; ADR-0024 adds supported timer typing and ordinary
clock-origin flow. ADR-0025 types known relative calendar construction and records
required live approval/origin binding. The timing stage still refuses unbound
multiple-attempt retry clocks. Resource hashes establish no live
host trust. Unknown kinds/fields/operations and unsupported roles refuse.
The historic illustrative `inspect` registry is unchanged.

## Prototype budgets

Each artifact/IR/AST/value is at most 1 MiB with existing strict depth-64 transport.
Registry: 256 entries and 8 MiB total supplied bytes; duplicate pins refuse even
when unused. Types/AST/reference traversal is depth-bounded; ASTs and collections
at most 10,000 members/nodes; policy/nominal chains at most 64 traversal levels.
Each registry/check has a cumulative 1,000,000-unit budget (visits plus serialized
nominal-evaluation result bytes). Repeat helper calls consume the same budget;
create a fresh registry for an independent check. Guards may be stricter than a
syntactically legal fragment. These are refusal limits, not performance claims.

Unused registry bytes are not interpreted or promoted to supported contracts.
Every referenced artifact (and every explicit authoring-companion resource) must
pass the appropriate checks; arbitrary extra host bytes grant no authority.

Diagnostics contain fixed codes and declaration-level IR paths, not source values
or invariant payloads. They are prototype-local, not the complete stable diagnostics
contract or source-related/confusable reporting system.

## Try the source integration

```sh
cargo run -p choreoform-authoring-frontend --locked -- check \
  docs/authoring/examples/review.choreo \
  docs/authoring/examples/review.bindings.json
cargo test -p choreoform-semantic-check --locked
cargo build -p choreoform-authoring-frontend --locked
.tools/ir-check/bin/python tools/check_static_contracts.py
```

The report says `static-checks-passed`, `executable:false`, the unchanged IR
revision and remaining obligations. `lower` still emits unvalidated candidates;
checking is explicit. The [service package](../../docs/authoring/examples/service.choreo)
uses [an explicit companion](../../docs/authoring/examples/service.bindings.json),
[provider](../../docs/authoring/examples/service.provider.json) and
[named types](../../docs/authoring/examples/service.named-types.json). Pass each
external file as `--resource ID REVISION PATH` using the companion's exact pins.
The native adapter embeds only the two accepted core snapshots, reads explicit
paths and writes stdout. Formatting still drops comments. No runtime/browser
execution or production authorization is implied by these synthetic examples.

See [the evidence and remaining work](../../docs/evaluation/0016-static-contract-checks.md).

## Conservative ordinary control stage (proposed ADR-0017)

`check_control(&DefinitionEnvelope, explicit_resources)` first runs unchanged
`check`, then verifies ordinary outcome/flow coverage, unique source/outcome pairs,
acyclicity and reciprocal same-scope split/fanout joins. Scope entry and ordinary
flows cannot directly activate a join; its source fork supplies the sole synthetic
edge. Nested predicate tokens must belong to the source population. This proves
neither predicate satisfiability nor progress.

The analysis follows ordinary root/call paths, intersecting initialized-cell sets
at merges. Root input cells are reported launch assumptions. Initial expressions
may depend on entry inputs/available ancestors, with no invented local initializer
order. All declared reads precede writes; every reachable finish must supply its
outputs. Invocations use fresh locals and exact port/write maps, distinct output
targets and summaries of guaranteed ancestor initialization. Repeat's zero case
and parallel joins receive no body-only initialization credit. Fanout supplies its
explicit item input. Unreachable declarations remain static checked and reported
as uncovered. Exceptional work receives no invented ordinary entry context.

`ControlReport` exposes the unchanged revision, covered/uncovered nodes, covered
scopes, required input cells and residual obligations through shared getters.
`unproved` diagnostics can reflect conservative limits rather than semantic
invalidity. Initialization is distinct from current evidence validity, access
authorization, race/invalidation, exceptional-flow, host and atomic enforcement.
The API provides no executable capability and preserves the old `check` boundary.

Control traversal has an independent cumulative 1,000,000-unit visit/state-member
budget and scope/predicate depth 64, in addition to existing transport/static
limits. Graph-edge traversal is iterative. Retained/copied availability membership
is charged; dense sets may refuse. These are prototype bounds, not scale promises.

Try `check-control` instead of `check` on the
[branch source](../../docs/authoring/examples/control.choreo) and
[companion](../../docs/authoring/examples/control.bindings.json). The CLI returns
`control-checks-passed`, `executable:false`, the revision, explicit coverage/input
sets and remaining obligations. See
[ADR-0017](../../docs/decisions/0017-control-availability-checks.md) and
[evidence/review questions](../../docs/evaluation/0017-control-availability-checks.md).

## Ordinary protection constraints (ADR-0019)

`check_protection(&DefinitionEnvelope, explicit_resources)` runs the preceding
stages, propagates cell/provider protection origins through ordinary paths and
checks inherited control origins against writes, initializers and input/output
transfers. Merges/child completions retain origin unions, including conservative
repeat/parallel cases. Provider result labels survive null publication. Access
metadata dependencies are included; no declassification or branch-release proof
is inferred. Cumulative work charges envelope members as well as transfer pairs;
the older static stage shares that accounting improvement.

`ProtectionReport` has immutable revision, transfer count, control coverage/input
assumptions and `AccessObligation` getters (`site`, `cell`, `operation`). Site IDs
resolve to node/data/scope declarations in that revision. Obligations enumerate
future judgments; they do not authenticate access. Both access sets empty refuses.
Actual principal/purpose/capability membership, actor authority, policy truth at
the current snapshot, exceptional/observable
channels, race/invalidation and atomic host enforcement remain explicit residual
work. Passing this stage is not whole-process noninterference or executable
admission. See [ADR-0019](../../docs/decisions/0019-protected-ordinary-transfers.md)
and [evidence](../../docs/evaluation/0019-protected-ordinary-transfers.md).

The [initial access follow-up](../../docs/evaluation/0019-initial-access-evidence.md)
checks input policy reads against the ancestor snapshot before seeding fresh
inputs, and initializer value/access-metadata reads against entry inputs and
ancestors. Peer-input/local initializer ordering is not invented. Shared bounded
dependency expansion includes participant and capability requirements. Caller
read sets include child-input/fanout-item destination metadata without reading
fresh target values; source lowering derives the same dependencies. The resolved
ordinary initialization residual is removed; actual runtime policy enforcement
remains a separate obligation.

The [completion follow-up](../../docs/evaluation/0019-completion-access-evidence.md)
requires scope-output access-policy data dependencies at every finish, including
uncalled child declarations. Source lowering derives the same metadata closure.
Validated data protection reference sets are cached only within the immutable
definition check and copied under a work charge; raw validation charges purpose
members and visits. This is declaration/availability evidence, not current access
permission or atomic publication.

## Lifecycle definition links (ADR-0020)

Static checking validates handler/follow-up child relations at every scope using
the policy, including uncalled scopes. Inherited empty-handler and terminal
policies stay reusable. For effect reconciliation, an indexed map must identify
one declared wait node using the referenced wait policy; zero refuses as missing
and multiple sites refuse as unproved. Ordinary IR wait-policy reuse without a
non-node effect use remains valid. Reports do not bind a live owner or prove
receiver acceptance, occurrence/subscription identity, correlation, authority or
atomic settlement. See [ADR-0020](../../docs/decisions/0020-lifecycle-policy-links.md)
and [evidence](../../docs/evaluation/0020-lifecycle-policy-links.md).

Creator read declarations also include ancestor data consulted by child
initializers and initial access metadata, for invoke/repeat/split/fanout. The
source frontend derives the same per-scope footprint; fresh child-local values
remain outside caller reads. This exposes dependencies to later conflict analysis
without claiming conflict safety, atomic creation or read authorization. See the
[creation evidence](../../docs/evaluation/0019-creation-access-evidence.md).

## Ordinary invalidation declarations (ADR-0021)

`check_invalidation` adds conservative pre-result dependency coverage for visited
decisions, human/capability activities and waits. Visible cell origins must name
the site in `invalidates`; nonvisible completion origins refuse as unproved.
Immutable reports expose checked/uncovered consequential sites and sorted links,
not consumed live revisions, withdrawal, cancellation or effect undo. Earlier
stages retain their boundaries. See [ADR-0021](../../docs/decisions/0021-ordinary-invalidation.md)
and [evidence](../../docs/evaluation/0021-ordinary-invalidation.md).

## Ordinary parallel independence (ADR-0022)

`check_concurrency` follows invalidation, indexes scope/context footprints and
checks ordinary-covered fork declarations. Fresh local storage stays private
across reused/fanout occurrences; ancestor shared hazards and unproved overlapping
effects refuse. Early joins and surviving reconciliation lifetimes refuse as
unproved. Accumulated sets avoid quadratic branch scans. Immutable reports retain
checked/uncovered forks, earlier evidence and residual host/domain obligations;
no arbitrary invariant, actual settlement, live seal or race permission is proved.
See [ADR-0022](../../docs/decisions/0022-ordinary-concurrency.md) and
[evidence](../../docs/evaluation/0022-ordinary-concurrency.md).

## Pinned clock metadata (ADR-0023)

`Resources::clock` returns immutable metadata for the exact `CLOCK_PROFILE` and
its read-only Integer tick provider. Register the clock, profile and source bytes
explicitly; the reader verifies all pins and source constraints. It reads no
time, authenticates no source and grants no timer/dispatch authority. Supported
timer typing/flow is integrated in ADR-0024, with known relative calendars in
ADR-0025. ADR-0026 adds explicit retry metadata; live calendar activation, retry
domain/fault/basis bindings, current facts and actual
host enforcement remain open. See
[clock contracts](../../docs/clocks/README.md) and
[evidence](../../docs/evaluation/0023-clock-resource-contract.md).

## Typed timer uses (ADR-0024)

`check_timing` follows prior stages and exposes immutable TimerUse site/policy,
clock/source pins, basis, pause authority and required live host judgments. Exact
wait outcomes and transitive clock-source metadata dependencies are checked;
clock labels apply before timing-dependent writes and coexist with activity
origins even without result publication. Reports remain non-executable; no
observations, current clock facts or authorization are inputs. Integer typing is
not proof of due/basis domain provenance. See
[ADR-0024](../../docs/decisions/0024-timer-contract-checks.md) and
[evidence](../../docs/evaluation/0024-timer-contract-checks.md).

## Instance calendar construction (ADR-0025)

`Resources::instance_calendar` checks exact known rule/profile/clock pins and
the relative-zone convention, sharing immutable interval metadata. Generic
`calendar` reading still verifies provenance only. `contains_coordinate` is a pure
half-open membership guard with no observed time, live binding or firing authority.
Timing reports expose exact CalendarUse references and required trusted host
construction/origin activation. Initial construction trust is separate from
requested pause/reschedule authority. See
[ADR-0025](../../docs/decisions/0025-instance-calendars.md) and
[evidence](../../docs/evaluation/0025-instance-calendars.md).
