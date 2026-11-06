<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Conservative fault-selection graph: frozen plan

**Date:** 2026-11-06<br>
**Baseline:** `8da0ac6c7d632bb05a622fe9e6ec9f27728e8563` (PR #37)<br>
**Authority:** ADR-0018 delegation; ADR-0013 P4 and ADR-0020 actual-use links.

Publish ADR-0028 for a separate conservative static fault-routing stage, following
the timing/ordinary stages. Preserve existing APIs/artifacts/source versions and
partial reports. No business-fault emission declaration exists: analyze every
exact name present in declared handler maps as potentially emitted; names absent
from all maps are unhandled. Retry faults are eligibility, not an emission bound.
Do not infer engine fault class from a name/prefix or route actual engine faults
as business events. No actual fault, invocation, permission or ownership is created.

Index immutable scopes, parent links, handler selections (owner/policy/fault key/
handler child) and ordinary-reachable scope creations. Use the existing ordinary
graph shape checks and syntactic reachability (all branches/repeat bodies possible,
no predicate truth invented). Start at the root. A worklist state is scope plus
its active handler-selection stack; ordinary child creation retains the stack,
handler creation adds its selection. Scopes with no discovered context remain
explicitly uncovered; never give them invented availability/protection inputs.

For a potential body fault, search exact keys from that scope up its lexical
parents. Local handlers can catch faults inside an outer handler body. When
propagation crosses out of an active handler root, pop that selection and exclude
it from this propagation, then continue at its enclosing scope. An escaped handler
failure therefore cannot immediately reselect itself. Keep outer active selections
until propagation leaves them too. Exclusion is per propagation, not a permanent
global suppression: a later different handler can expose a genuine cycle. Record
potential edges from the original active selection (if any) to the selected target.
This is abstract body-fault/escaped-failure analysis, not a runtime dispatcher.

Union the possible handler-selection edges and check that graph for cycles.
Refuse `fault-routing-cycle-unproved` with a stable scope faults path when the
overapproximation has a cycle; do not claim every such definition is invalid.
Multiple fault keys or mutually exclusive contexts may need an explicit future
emission/proof profile for precision. Keep unknown names unhandled, exact lexical
precedence and failed-selection exclusion; do not silently narrow possible faults.

Use integer indexes for scopes, names, selections and context/graph keys, avoiding
repeated copies/comparisons of long names in expanded states. Bound every state,
stack copy, fault lookup, edge and graph traversal with the existing work/depth
limits. Deduplicate contexts and callee templates as static analysis, not runtime
occurrences. Expose immutable declaration/potential-route references, context count,
checked/uncovered scope and selection coverage, and explicit residual host/
exceptional-data/lifecycle judgments. Prefer borrowed indexed route views to
duplicating selection strings per edge. Reports and CLI remain non-executable.

Tests: nearest exact key and absence/no wildcard; one handler avoids immediate
self-reentry; ordinary nested scopes retain active exclusions; nested handlers
retain outer frames; different-key/same-template and mutual handler cycles refuse;
ordinary versus handler use of one template stay distinct; uncalled declarations
stay uncovered; unknown emission never derives from retry eligibility; iterative
wide/deep/context budget behavior; stable paths/no value leaks; source/format/
reparse report identity in both profiles. Verify native/Wasm and artifact/stage
oracles, self-review separately, record delegated ADR acceptance, require final-head
CI and expected-head merge. Full exceptional availability/protection/invalidation,
runtime engine-fault classification/selection/ownership and remaining Phase 1 gates
stay open; no Phase 2 engine or actual-host evidence is supplied.
