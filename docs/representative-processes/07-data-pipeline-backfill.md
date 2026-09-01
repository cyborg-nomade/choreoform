<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-07: Data-pipeline backfill

| Attribute | Value |
| --- | --- |
| Domain | Data engineering |
| Complexity | Level 3 — Long-running |
| Primary participants | Data operator, scheduler, compute platform, source and destination systems, data owner |
| Typical duration | Minutes to several days |
| Automation mix | Automated partition processing with human authorization and exception handling |

## Purpose and corpus role

Recompute a bounded historical data range safely, at scale, without corrupting
current outputs or overwhelming shared systems. This case stresses parameterized
fan-out, bounded concurrency, checkpointing, idempotent writes, transient versus
permanent failure, pause/resume, cancellation, and the identity of code and data
versions.

## Scope and assumptions

The case begins with an authorized backfill request and ends with publication,
cancelled cleanup, or an explicit partial outcome. The transformation already
exists and is referenced by an immutable artifact identity. Data correctness,
privacy classification, retention, and resource policy are organization-specific;
this synthetic case is not a production runbook.

## Participants

- **Data operator:** defines scope, dry-runs, starts, pauses, resumes, cancels,
  and assesses exceptions within authority.
- **Data owner:** approves sensitive or high-impact scope and publication.
- **Scheduler:** creates partitions and enforces dependencies and limits.
- **Compute platform:** executes attempts and reports resource and result facts.
- **Source and destination systems:** provide versioned reads and staged or
  idempotent writes.

## Trigger and preconditions

An approved request identifies time or key range, partition strategy,
transformation artifact, input snapshot or consistency rule, destination,
validation rules, resource budget, and publication mode. Overlap with active
jobs and data sensitivity must be assessed.

## Information and state

- Backfill, partition, attempt, artifact, input, staging, and publication
  identities.
- Partition dependencies and status; checkpoints; retry reason and budget;
  resource use; validation evidence; pause, cancellation, and cleanup state.
- Desired output range kept distinct from material already staged or published.

## Main success path

1. A dry run validates authorization, scope, dependencies, capacity, and output
   plan without publishing data.
2. The scheduler expands the request into deterministic partitions.
3. Eligible partitions execute concurrently within source, destination, and
   compute limits, using stable output identities.
4. Failed transient attempts retry from a safe boundary; successful partitions
   are not repeated unnecessarily.
5. Aggregate and partition validation passes.
6. An authorized publication switches or commits the validated result, and
   temporary material is cleaned according to policy.

## Alternatives and failures

- Rate limiting or transient infrastructure failure backs off and retries
  without multiplying writes.
- A deterministic data or code failure exhausts or bypasses retries and blocks
  only dependent publication while independent diagnostics may continue.
- Pause stops new scheduling and reaches a documented safe condition; in-flight
  work may finish or checkpoint according to provider capability.
- Resume uses the same request definition and validated completed partitions;
  a code or scope change creates a new revision with explicit reuse decisions.
- Cancellation prevents publication and creates cleanup obligations; already
  published data requires a separate correction or restoration effect.
- Worker loss yields an unknown attempt until lease, checkpoint, and destination
  state are reconciled.

## Time, concurrency, and scale

The partition count may range from one to millions. Concurrency is a runtime
policy constrained by quotas and back-pressure, not a semantic claim that all
partitions start together. Partitions may have dependencies and skew. Pauses
and crashes can last longer than credentials, leases, or input availability,
so resume must revalidate assumptions.

## Capabilities and effects

- **Read versioned input:** obtain data under a declared consistency and access
  contract.
- **Execute transformation:** run an immutable artifact with bounded resources
  and observable attempt identity.
- **Write staged partition:** produce repeatable output keyed by backfill and
  partition identity.
- **Validate result:** emit evidence separately from the decision to publish.
- **Publish or restore range:** perform a guarded, reconcilable destination
  effect with an explicit authority boundary.

## Invariants and protections

- Every published value is attributable to one request revision, transformation
  artifact, input rule, partition, and validated publication decision.
- Retrying an attempt does not duplicate logical output.
- Pause and cancellation do not imply that in-flight external work vanished.
- A changed artifact or scope cannot silently resume as the old backfill.
- Restricted data is read, staged, logged, and retained only within its allowed
  purpose and environment.

## Observable outcomes

- Operators can inspect progress, skew, retry reasons, remaining dependencies,
  validation, resource use, and cleanup obligations by partition and aggregate.
- Recovery can distinguish not started, leased, running, checkpointed,
  succeeded, failed, cancelled, and outcome unknown.
- Publication history identifies exactly which validated result is visible.

## Acceptance scenarios

### RP-07-A — Bounded parallel completion

- **Given** a validated request expanded into independent partitions,
- **when** workers complete them under the configured concurrency limit,
- **then** each logical partition has one staged result and publication waits
  for aggregate validation.

### RP-07-B — Retry after worker loss

- **Given** a partition attempt loses its worker after a possible write,
- **when** its lease expires and recovery begins,
- **then** checkpoint and destination state are reconciled before an idempotent
  continuation or retry is scheduled.

### RP-07-C — Pause and version-safe resume

- **Given** a partially complete backfill is paused,
- **when** an operator resumes without changing its artifact or scope,
- **then** valid completed partitions are reused and only eligible incomplete
  work is scheduled.

### RP-07-D — Cancellation after publication

- **Given** a result has already been published,
- **when** cancellation is requested,
- **then** the process does not pretend publication was rolled back and instead
  records the required correction or restoration outcome.

## Semantic pressures exposed

- How is dynamic fan-out expressed without materializing an enormous visual
  graph?
- Which state belongs to the process and which belongs to a compute provider?
- How are pause, resume, cancellation, retry, and replay distinguished?
- How do definition, artifact, input, and output versions constrain recovery?

## Out of scope

- Transformation logic, schema design, data-quality rule design, cluster
  scheduling algorithms, disaster recovery, and privacy-law interpretation.

## Sources and inspiration

- [Google Cloud, Dataflow pipeline lifecycle](https://docs.cloud.google.com/dataflow/docs/pipeline-lifecycle)
- [Google Cloud, Pause a streaming pipeline](https://docs.cloud.google.com/dataflow/docs/guides/pause-job)
- These sources illustrate execution and recovery pressures; the case is
  provider-neutral and does not reproduce Dataflow semantics.
