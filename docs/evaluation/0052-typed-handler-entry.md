<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Typed handler-local entry declaration and conditional proof evidence

**Date:** 2026-11-07<br>
**Plan:** [frozen raw SDK scope](0052-typed-handler-entry-plan.md), `2cbb375`<br>
**Decision:** [ADR-0052](../decisions/0052-typed-handler-entry.md)

Core 0.7 revision is
`sha256:e286e54441b3d785ae823eef359c12d64fb8405829c70709227282e5af62f54b`.
It requires IR 0.3 and all six inherited core artifacts. Only scope wire records
change by adding required nullable faultInput; wait capture remains as in IR 0.2.
Typed projection enforces exact history Ref/data role and same-scope ownership.
Default construction stays IR 0.1; old scopes reject even null faultInput. Version
participates in the existing semantic hash domain. The original illustrative
inspector refuses the new wire contract while transport/structure remain separate.
Unknown wire sentinel moves from newly supported 0.3 to unsupported 0.4.

The raw semantic stage validates an exact ADR-0050 history descriptor, fixed nominal
type and complete transitive bytes. Reserved history belongs to a local-child
handler with zero ordinary ports, initializer, declared writer, ordinary or P6
creator, including unvisited declarations. Source protection metadata uses normal
visibility. Immutable declaration rows share metadata/revision, never actual values,
grants or initialized storage. Source read closure is validated per context without
retaining an unused extra read-set copy per row. Empty legacy publication adds no
resource work; failure cannot drain/publish partial rows.

The separate check_fault_entries report reuses one retained origin pass. Every
incoming covered relation must have the exact source, an ordinary reachable/covered
emitter and empty active-before stack. Human/null/legacy, wrong-source, unvisited
or exceptional contexts refuse. Unexplored source sites are also checked along
lexical ancestors so an omitted active context cannot silently drop a possible
incoming relation. No incoming context means an uncovered binding. It returns no
ordinary/legacy exceptional coverage promotion, runtime event or history Value.

Conditional bodies are portless/childless/terminal flat local-only compute/decision /
finish work. History-write access preconditions are checked against the empty
pre-entry value set before making the reserved cell available. Then fresh local
initializers and all body access/read/write/finish availability are checked.
Self/circular/ancestor access reads refuse; identifiers in history do not make
ancestor values available. No entry coverage avoids another graph/provider/dependency
walk. The function takes no caller-supplied event value or host callback.

Ordinary protection retains bounded conservative source-scope context unions when
history is declared, preserving input/fault-existence/control/provider/creator-owner
labels. The shared retained-label mechanism does not provide live parent values,
mutable dependencies, ordinary completion or current grants. Exact activity-provider
origins must occur in each retained source context. Missing/nonordinary contexts
refuse. Complete history initialization, later publication/control, destination
access and initializer flow precede strict local invalidation. Old P6 behavior,
diagnostics and serialized evidence stay unchanged. Conditional body sets remain
separate, with explicit current access and provenance obligations.

Six structural groups check version/hash/roundtrip, nullable required/closed fields,
external pin/data roles, same-scope ownership, old/unknown wire boundaries and the
illustrative inspector's refusal. Fourteen SDK integration groups include:

- exact promoted full RP-01 nominal/source declarations and inherited capture /
  general-source composition, plus every required missing/corrupt new/base artifact;
- old core/wire refusal, root/wrong-type/ordinary/P6 creator/port barriers and a
  fabricated correctly typed initializer or unvisited writer failing readonly entry;
- normal visibility of private source protection, without a handler lookup exception;
- conditional history read/copy/branch, real source/provider provenance, write access
  obligations and strict local invalidation; old exceptional availability still
  refuses the uninitialized ordinary history read;
- exact other-source, human/null, unvisited and active exceptional source refusals;
  no-incoming bindings remain uncovered, and an unexplored possible source refuses;
- public history, initializer and branch publication leaks, including a constant
  public initializer that never reads history: fault existence still taints it;
- self/circular and ancestor-dependent pre-entry access, unavailable fresh locals,
  body/storage restrictions and missing strict invalidation;
- all thirteen shared finite conditional cases through actual SDK judgments.

One internal group checks zero-work empty legacy publication, cumulative final
construction refusal before draining, shared revision and metadata. Existing shared
provenance/copy budget tests exercise the retained-context helper used by both
conditional kinds. Five independent Python groups check exact core/base/history /
nominal hashes, schema delta and refusal/hash boundaries, all thirteen finite cases
with explicit prevalidated premises, unchanged current source/default reports and
CI/licensing. No finite fixture is an actual fault, user observation or handler run.

Local verification passes **430 Rust tests plus six compile-fail doctests**, native
format/lint, all five library Wasm lints and release portability/frontend builds.
The native oracle passes 97 structural cases, 17 JCS/hash comparisons, three original
fixtures/hashes, ten node variants and nine CLI operations. Every required Python
workflow command and exact benchmark regeneration/historical probe passes. Final
small no-coverage fast path also passes focused SDK tests and workspace lint.
Wasm compilation supplies no browser execution. Transient logs are under
`/private/tmp/handler-entry-{workspace,clippy,wasm,native,oracles}-final.log` and
`/private/tmp/handler-entry-{all-tests,wire-tests,budget,vectors-tests,python,early-tests}.log`.
Required final-head CI, comment audit and signed expected-head merge remain pending.

Separate completed-diff delegated automated self-review checked closed old/new wire /
core inheritance and artifact identity, exact local readonly roles and all creators,
source/provider/type visibility, complete supported/unsupported/uncovered origin
handling and full contexts, pre-entry access ordering, zero-value/retained-label
separation, source/fault-control/constant/branch/initializer flow, strict invalidation,
cumulative/shared copying, no partial proof, no extra old scope-field allocation,
compatibility, licensing and evidence scope. It found and fixed the omitted-unexplored-
source relation before publication. No actionable issue remains. This is agent
self-review, not independent/human or personal owner approval; inspect every thread
and CodeRabbit summary. A skipped bot review supplies no review evidence.

This is raw SDK support only. Readable source/companion/native entry selection and
explicit conditional disclosure are next; general tool-managed near-English editing
remains open. Actual authenticated non-engine event/evidence, exact occurrence/input /
request/key/effect correlation, current source/work/access/management authority,
nearest live selection, fresh immutable owned storage and complete retained provenance
remain host premises. Handler finish cannot clear cancellation, return an activity
result or resume its parent. Permitted owned continuation/cutoff/handoff/all-settlement
follows separately. Current RP-01 source 0.7/core 0.4/IR 0.1 remains byte-identical;
parent #46 stays draft/incomplete, all forty scenarios Partial, complete validator /
benchmarks/authoring/visual/protected editing and actual host/user/browser/independent /
phase gates open. No Phase 2 work starts.
