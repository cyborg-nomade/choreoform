<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Provider fault-history metadata: frozen plan

**Date:** 2026-11-07<br>
**Baseline:** `6c1d51b1b71c6463950b8aab328bfc8686ac0eb1`, draft #46 after PR #64<br>
**Authority:** ADR-0018; no scenario/parent/phase admission

Use a dedicated stacked dependency PR and ADR-0050. Publish an independently pinned
provider-fault-history metadata profile and a fixed named-types artifact. A closed
resource `{kind:"provider-fault-history",version:"0.1.0",id,profile:Ref,source:Ref}`
references one exact existing provider-fault-source. Require exact descriptor/profile /
source/source-profile/provider/type bytes in the bounded explicit registry; no IO,
ambient discovery, provider/name inference or old contract changes. Its immutable
private-constructor SDK result retains source metadata and the exact fixed nominal
history type, not an event, actual-use binding, grant or handler initializer.

The nominal representation is a closed record with four P4 fields: `type:Text`,
`sourceOccurrence:{definitionRevision:Text,instance:Text,scopeOccurrence:Text,
site:Text,occurrence:Text,attempt:Integer}`, `cause:{observation:Text,record:Text}`,
and `inputRevisions:List<{cell:Text,revision:Text}>`. Record is an opaque original
cause/evidence reference, never raw inputs/results or an assertion of approval.
References/revisions are claims requiring actual host authentication; no evidence
payload, current cell value, inferred no-effect/success or ordinary read is supplied.
The fixed nominal representation uses an always-true invariant; nominal wrapping
alone intentionally does not attest that the history occurred or matches a source.

Expose a distinctly named claim-shape check using existing literal admission plus
exact source fault/observation membership, nonempty identity/reference strings,
canonical definition-revision syntax, attempt 1..1000 and unique input cell IDs.
Charge cumulative resource work for all repeated claim/list/member checks. Success
returns unit only; it does not construct an authenticated event or entry report.
Test that a fabricated but well-formed claim can pass this limited judgment while
ordinary handler ports/facts and scope fault fields still refuse. Later entry must
independently check actual emitting node/source/attempt/cause/input revision and
nearest handler key, initialization, current access and strict invalidation.

Expose the provider's exact declared source protection as a lower bound, separately
from mandatory source inputs, fault-existence/control and creator/owner provenance.
Never label it the complete history envelope or erase control labels because there
are no inherited values. No descriptor can authorize declassification, authentication,
retry, continuation, cancellation reversal, parent resumption, handoff or disposal.

Cache immutable descriptors by exact pin, share source metadata and fixed nominal
type without copying per alias; retain the same cumulative resource budget on cached
calls and claim validation. Validate before publishing cache entries; no partial
history evidence on failure. Keep all previous wire/core/source versions, definition
and package digests, reports, quotas, retry eligibility and checked-in RP-01 bytes
unchanged. This is raw SDK metadata only; no new source role, CLI view or scope field.

Test real provider/source metadata and fixed nominal identity/representation;
missing/corrupt/unknown profile/source/provider/type bytes; closed records and exact
source identity; all claim shape/mapping/attempt/revision/duplicate boundaries;
misleading labels/unresolved evidence; cached sharing and cumulative exhaustion;
ordinary-value versus actual-event distinction and unchanged handler barriers.
Add finite shared claim vectors, independent Python type/claim/profile/hash oracle,
required CI and licensing. Run relevant and full native/Wasm/Python checks, separate
completed-diff agent review, every thread/CodeRabbit summary, final-head CI and signed
expected-head merge. Record progress after merge.

Next freeze reviewed handler-local entry on a new explicit wire, with complete
incoming emitting-site/source/control contexts and conditional initialization/access /
strict invalidation; fail closed on unsupported human/unbound/engine origins. Then
readable authoring and permitted owned continuation/cutoff/all-settlement. Do not
silently adopt promoted fixtures into the package. Parent remains draft/incomplete;
all forty scenarios and complete validator/benchmark/authoring/visual/protected
editing/actual host/user/browser/independent/phase gates remain open.
