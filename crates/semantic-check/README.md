<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Static contract checks, not executable admission

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

The accepted suite has no standardized clock-provider format. Every non-null timer
and clock/calendar authoring resource still refuses, even if a host supplied
matching hashes. Unknown kinds/fields/operations and unsupported roles refuse.
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
