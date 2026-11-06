<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0028: Check a conservative active-context handler-selection graph

**Status:** Accepted under Project Owner delegation; effective upon PR merge<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 delegation

## Context and criteria

P4 requires nearest exact-key fault selection, exclusion of the handler selection
that failed, and an acyclic selection/invocation graph. ADR-0020 verifies actual
local child links, but the ordinary stages do not model handler contexts. The
[validator audit](../evaluation/0028-validator-gap-audit.md) identifies this missing
static judgment. The wire contains no business-fault emission declaration;
work.retry.faults lists retry eligibility, not all possible faults.

Supply useful bounded routing evidence without guessing emissions, losing active
handler context or claiming data availability, authenticated events, runtime
dispatch or ownership. Preserve old profiles/bytes and conservative refusal.

## Decision

Add `check_fault_routes` after the prior timing/ordinary stages, with immutable
FaultReport, FaultSelection and borrowed indexed FaultRoute views. Add the source
Candidate/CLI adapter. No contract, payload, source version or engine is changed.

Intern scope/name/selection identities. Index ordinary syntactic reachability and
scope creation (both branches and repeat bodies remain possible). Start at root;
worklist states contain scope plus an active selection stack. Ordinary children
retain the stack; selected handler children add the exact owner/policy/key/target
selection. Deduplicate templates/contexts only for static analysis. No live
occurrences or entry value/permission contexts are fabricated.

Conservatively consider every exact name in declared fault policies as potentially
emitted. An unused policy's names may have no selection at any actual use; names
absent from all maps are necessarily unhandled. Create selections only at actual
scope uses of policies. Do not infer wildcards, prefixes, fault subtypes, emission
from retry eligibility or engine/business classification from a name spelling.
Actual engine failures are not catchable business events; the target must enforce
that class boundary independently of these author-declared names.

For a potential body fault, search exact keys from that scope up its lexical
parents. Local handlers inside an outer handler body retain normal lexical
precedence. If propagation escapes an active handler root, pop/exclude its failed
selection for this propagation before searching its enclosing scope. Outer active
frames remain until propagation escapes them too. This distinguishes a body fault
that can be locally handled from an escaped failure of the handler itself, which
cannot immediately reselect the failed selection. Exclusions are per propagation;
a later different handler may expose a genuine cycle. This abstract algorithm
does not dispatch a real fault or bind its record as ordinary data.

Union possible active-selection → selected-handler edges, retaining origin scope
metadata, and check the graph iteratively for cycles. A cyclic overapproximation
refuses `fault-routing-cycle-unproved` at the owning scope's faults field. This is
not a declaration that every such process is semantically invalid: mutually
exclusive contexts or restricted emissions can require a future explicit proof/
emission profile. A one-key handler is not a self-loop merely because it could
fail; distinct keys selecting the same template retain distinct selection identity.

Bound graph indexing, context/stack copies, lookups, edges and graph traversal by
existing work/depth limits. Expanded states/edges use integer indexes, avoiding
long-name duplication. Public route views borrow one immutable selection table.
Report abstract context count, checked/uncovered scopes and handler selections,
and known-name unhandled lookup count. These are not runtime event counts.

## Options considered

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Active-context overapproximation and explicit unproved refusal | Checks existing P4 routing without invented evidence; bounded and precise about active exclusions | Some valid emission/context cases can over-refuse | Adopt |
| Treat retry faults as an emission bound | Small graph | Unsoundly drops nonretry/business faults | Reject |
| Scope-only graph or permanent failed-handler suppression | Simple | False immediate self-loops, lost outer frames or hidden later reentry | Reject |
| Add an emitted-fault dialect/profile now | More precision | New semantics, source/export and host truth requirements before need is demonstrated | Future precision option; do not smuggle into annotations |
| Build runtime fault dispatch now | Can run examples | Introduces the Phase 2 engine and authenticated ownership problems prematurely | Defer to its proper phase |

## Consequences and confirmation

Route coverage is explicitly separate from ordinary availability/protection
coverage. A handler can appear in checkedFaultScopes while its node still appears
in ordinary uncoveredNodes. Its actual entry availability/protection/invalidation,
fault source/cause/occurrence/input revisions, engine classification, cancellation,
surviving ownership and atomic settlement remain required judgments. Prior timing
obligations are retained separately. Reports never grant executable permission.
The abstract stack describes one lexical propagation/handler-invocation chain;
simultaneous fault events and concurrent handler occurrence activation/lifetime
remain separate unproved concurrency/ownership obligations. Deduplicating a
template context does not prove that two live occurrences are interchangeable.

Follow [frozen plan](../evaluation/0028-fault-routing-plan.md), `c6967c7`, and
[evidence](../evaluation/0028-conservative-fault-routing.md). No actual host,
browser, process event or participant observations are claimed. Full validator,
benchmarks, text/visual/editing/user evidence and Phase 1 exit remain open.

Accepted after separate completed-diff automated self-review under delegation.
215 Rust tests plus five compile-fail doctests, native/Wasm checks and all relevant
artifact/source/stage oracles passed locally. Review explicitly retained concurrent
occurrence/data-context limits and bounded native report expansion. No actionable
finding remains; no independent/owner review or actual host evidence is claimed.
Required final-head CI and expected-head merge remain prerequisites.

1. [x] Finish regressions, native/Wasm checks and separate automated self-review.
2. [x] Record delegated acceptance; require final-head CI/expected-head merge.
3. [ ] Continue exceptional entry/lifetime/protection/invalidation and complete
   validator evidence, with explicit unsupported precision boundaries.
