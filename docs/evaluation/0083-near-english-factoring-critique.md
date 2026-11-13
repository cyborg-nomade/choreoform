<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Near-English factoring: critique before inactive probes

**Date:** 2026-11-12<br>
**Frozen scope:** `b81edda`, [plan](0083-near-english-factoring-critique-plan.md)<br>
**Dependency met:** #97 signed merge `7f26305021e28b8598582ac1a02a0888891d65c0`,
CI `43247994354`, automated COMMENT review/fresh full audit `5877161955`

ADR14/15 select structured named steps and explicit sentences provisionally,
with managed identities and no raw JSON in ordinary authoring. The owner prefers
broad accessibility and plain English; quoted names/fixed clauses are a current
precision tradeoff, not final user evidence. Existing grammar represents types,
expressions, policies and scoped work explicitly, with exact private bindings.
The Rust Syntax tree preserves token/byte extents; lowering operates over it with
strict ledger resolution. Fixture Python is a generator, not the authoring editor.

RP-01 source is 61211 bytes/10223 tokens and RP-03 576759 bytes/84051 tokens.
Those previously measured full files contain repeated structural type descriptions,
protected cells, receipt predicates and scoped request/recovery declarations.
RP-08 has a complete case brief and paper walkthrough, not a full current lowerable
source package. A new prose sketch cannot fill that gap. New measurements below
must distinguish repeated text, semantic structure and real comprehension evidence.

## Options to measure before selection

| Option | Human benefit | Semantic/implementation constraints |
| --- | --- | --- |
| Local reusable type/expression/protection names | Familiar names can replace repeated structure; smallest compositional extension | Transparent type aliases must preserve exact nominal identity; predicate parameters/fact contexts need static typing; policy defaults must not hide access or authority |
| Hygienic parameterized scoped blocks | Removes repeated service/retention/receipt corridors while keeping named purposes | Every use needs correct fresh scoped work/storage/endpoints and stable managed IDs; direct-child ownership, lexical privacy, compensation references and exact provider recipient claims cannot be shared or widened |
| Domain vocabulary/libraries | Familiar task words and reuse can aid domain authors | Definitions must expand to inspectable accepted meaning; no guessed verbs or opaque callbacks; domain-specific words cannot become hidden core semantics |
| Formatting/progressive disclosure alone | Lower visual load without semantic change | Does not remove full-source duplication; protected summaries must retain uncertainty/authority and need actual accessibility/user evidence |

Paper-only sketches illustrate intent, not a chosen grammar or working compiler:

```text
Describe a "Reservation instruction" as a record with ... .
Keep "Request" as a "Reservation instruction" under "Order access".
```

```text
Describe "A reservation service" once, with a request, an adapter and recovery rules.
Use "A reservation service" here as "Reserve first line" with ... .
```

Ellipses and parameters are deliberately unfinished. The first could preserve an
anonymous type's exact structure; it must never create/erase nominal membership.
The second needs a real hygienic instantiation contract and cannot be implemented
as unchecked string replacement. Neither is claimed parseable or semantically
complete, and shorter spelling supplies no human accessibility measurement.

## Required evidence before direction

Measure actual token/byte extents and exact repeated outer type/expression spelling
using the existing bounded Rust parser; do not add overlapping category counts
or call redundancy a measured savings figure. Compare actual IR declaration/type /
policy families and the complete process intents, including unknown effects and
recovery. Any inactive expansion probe follows this signed critique checkpoint.
If asserting equivalence, actually lower/check the finite supported comparison;
paper examples remain paper. No representative-user feedback may be generated.

The first selected implementation must have meaningful whole-process benefit and
preserve companion identity, private scope, P3 pins/recipients, actual-use endpoint /
retention/capture/compensation relations and cumulative expansion/output budgets.
It needs a separately signed grammar/companion/version plan before production code.
Current source0.16 remains C12; C13 selection is a distinct explicit version issue.
Full export/editing, protected accessible text/visual parity, RP-08 and actual
representative-user evaluation remain Phase 1 work, not a larger-parser substitute.
Generic region proof and every other unproved phase gate remain open; no Phase 2.

## Measured native baseline and recommended first slice

The read-only `factoring_metrics` example uses the existing Rust parser/byte spans,
not a new grammar. Its [captured JSON](0083-factoring-metrics.json) is reproduced
from the unchanged full sources. Outermost types/expressions avoid nested double
counting within their category; expression/type categories may overlap and must
not be summed. Exact repeated spelling is an upper bound before definition and
reference costs, not measured compression or usability.

| Actual source | Bytes / tokens | Outer type bytes / repeated upper bound | Data protection bytes / repeated upper bound | Outer expression bytes / exact repeated upper bound |
| --- | --- | --- | --- | --- |
| RP-01 | 61211 / 10223 | 6390 / 2491 | 6292 / 5808 | 22971 / 128 |
| RP-03 | 576759 / 84051 | 98058 / 70621 | 134230 / 133364 | 204788 / 16268 |

RP-03 has 155 data declarations repeating the same 866-byte protection clause.
Independent inspection of resolved IR confirms **one exact protection value** across
all 155: order_access, sensitivity order, one purpose, five participants and 25
capabilities. RP-01 has two surface spellings across 26 data declarations but one
resolved protection value. Thus spelling alone must not decide semantic reuse.
The RP-03 protection redundancy upper bound is 23.1% of the full source before
alias costs; repeated types add a separate future opportunity. Most outer
expressions differ literally; their 35.5% source footprint does not prove safe
parameterized predicate reuse or corresponding savings.

Choose **explicit named data protection definitions first**. This removes the
largest measured exact repetition without instantiating/sharing work, callbacks,
ports, capture/retention/compensation or provider resources. Each data use names a
complete definition; do not add an ambient default. Expand to the same complete
IR protection object and recheck use-site visibility/policy/flow normally. Shared
text does not imply shared authorization or broaden a single recipient.

A paper-only direction using existing vocabulary is:

```text
Protect data as "Order information".
  Under "Order access".
  Sensitivity "order".
  Purposes ["Order fulfillment"].
  Participants [the explicitly named participants].
  Capabilities [the explicitly named services].
End protection.

Keep "Original order" as type "Order" from "Order types",
  protected as "Order information".
  Invalidate [the explicit dependent work].
  Initially unavailable.
End data.
```

The bracketed descriptions above are explanatory placeholders, **not grammar or
lowerable source**. Final wording is a separate signed implementation proposal;
real examples must list exact roles/services/dependents and preserve all fields.
Use names with visible definitions and meaningful protected summaries, rather
than hide exposure behind a label. More conversational synonyms need matched
representative-user tasks; there are no such results yet.

Target the first production slice at new source0.17/companion0.16 while retaining
C12/IR0.4, subject to its separately frozen contract. This isolates surface reuse
and permits exact semantic-body/revision comparison against current full packages.
Do not bundle C13 selection into that experiment; its explicit versioned source
adoption remains future work. Existing source0.16 and all quotas stay fixed.
Require real measured whole RP-03 reduction (a proposed 15% byte target), exact
expanded IR/body identity, invalid/ambiguous/private/duplicate/oversize refusals
and stable managed identities before claiming the slice works. An upper bound or
this utility supplies none of that implementation evidence.

Next prioritize transparent type names if measured benefit warrants them, then
hygienic service/predicate reuse with proper typed parameters and unique instance
identity. These remain separately scoped. The first slice alone will not make
576 KB ordinary prose, cover complete RP-08, implement general safe editing or
supply visual/user evidence. Keep the full authoring deliverable open.

The same native tree inventory counts RP-01/RP-03 as 8/59 nested scopes,
46/198 steps, 7/29 work policies, 3/25 wait policies, 1/25 original-request retention
declarations and 106/340 structurally qualified references. RP-03's 25 capability
instances and large expression footprint motivate later scoped/predicate reuse,
but those counts are not semantic equivalence or usability findings. The metric
uses parser productions for qualification, so the word “in” inside a quoted name
is not counted as a qualifier. Source digests bind every captured measurement.

## Whole-case and identity review criteria

The on-disk current source/companion/candidate artifacts are respectively
61211/52736/108565 bytes for RP-01 and 576759/240578/883289 bytes for RP-03.
Companion size is managed identity/resource metadata, not ordinary prose; reducing
it is not a substitute for improving what authors read.

| Complete intent / actual material | Benefit and retained responsibilities |
| --- | --- |
| RP-01 reimbursement: corrections, authority-specific decisions, uncertain payment and follow-up; current partial source exists | One resolved data envelope across 26 cells is a finite first reuse case. Manager/finance revision evidence, original payment request, cancellation and uncertain settlement remain explicit. |
| RP-03 two-line order: reservation/payment, preparation/delivery, cancellation/compensation/late evidence; current partial source exists | One resolved envelope across 155 cells gives a meaningful exact target. Reuse must leave all 25 logical capability instances, unique waits/storage/retention and pinned provider recipient claims unchanged. Actual external concurrency remains unproved. |
| RP-08 incident response: changing asset/task/severity scope, confidential evidence, guarded effects and ownership handover; brief/paper walkthrough only | Different evidence classes may require distinct named protections. No global default or broad inherited recipient set; no executable source, size reduction, round trip or user evidence is claimed. |

Prospective implementation acceptance cases (requirements, **not passing tests**):

- A definition visible in an ancestor can be explicitly selected by data in a
  descendant only when all resolved policy/participant/capability references remain
  visible and valid at the use. Expand the exact IR value and preserve data IDs.
- Undefined/duplicate/mismatched names, sibling/private definitions and wrong-kind
  actor/policy/capability references refuse with useful definition/use locations.
  Same spelling across scopes is never evidence of equal semantic bindings.
- All five fields remain explicit in the definition; neither omission nor a name
  grants current access. Provider result-protection resources are not rewritten.
- Renaming the source-only definition preserves explicit managed identity;
  copying allocates a new identity through the declared operation; deleting leaves
  a tombstone. No ID recovery from position/name or silent reassignment.
- A short source that expands many large protection sets must still obey the
  unchanged shared work/IR/output limits, with no partial candidate on failure.
  The first slice should not allow recursive protection definitions/default chains.

General hygienic work templates are a later contract: direct-child scope ownership,
exact actual-use wait/retention/capture relations, source-only versus semantic
identities, compensation purpose and immutable P3 recipient bindings cannot be
solved by replacing names in text. The current parser Tree already retains source
extents; the next implementation should reuse that structure and ledger resolver.

## Verification checkpoint

**Verified:** 2026-11-13. All 637 Rust tests and six compile-fail doctests pass,
including four native measurement groups. Format, native/all-five-library Wasm
lint, release portability/frontend Wasm, fresh native portability and all 79 Python
commands pass. Logs: `/private/tmp/near-english-factoring-final-workspace.log` and
`/private/tmp/near-english-factoring-verify.log`; final metric/lint/oracle checks pass
on the final helper. The utility is an example/test target, not a production API.

A separate completed-diff automated review checked correctness, privacy/security,
source extents/counting, stable signatures, bounded read/work/output, determinism,
resolved-envelope identity, scope/licensing and paper/evidence distinctions.
No remaining actionable findings. Nested categories are not added; upper bounds
exclude reuse costs; hypothetical examples/prospective acceptance cases are not
called compiler, round-trip or user evidence. All production syntax/library/core /
source/schema/quota and old artifact/benchmark/history/generator/report meanings
and bytes stay fixed. Real source reduction, safe identity-aware editing, RP-08,
visual parity and actual representative-user evaluation remain open.

Final-head CI, fresh full unresolved thread/review/CodeRabbit audit and signed
expected-head merge remain. This is automated self-review, not independent/human
or personal owner approval; a skipped bot supplies no evidence.
