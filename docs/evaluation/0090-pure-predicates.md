<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Source0.19 typed pure Boolean tests: implementation evidence

**Status:** integrated by PR #105 after successful final-head CI, automated review and fresh full audit.<br>
**Authority:** [ADR-0018](../decisions/0018-phase-review-delegation.md),
[scope90](0090-pure-predicate-recipes-plan.md) and
[ADR-0090](../decisions/0090-pure-predicate-recipes.md); no personal owner or independent review.

Exact grammar/profile/schema publication `5b1a8c57` precedes production code.
Source0.19 / companion0.18 target unchanged C12 / IR0.4. Native lowering caches
bounded pure templates, substitutes named arguments hygienically, and retains
literal values/type descriptors as opaque data. Source-only definitions/inputs
have managed identities; none becomes an IR function or new execution authority.

Pure definitions, including unused/forward/nested definitions, are checked with
their exact declared parameter types and no ambient facts/cells. Cycles, unused
inputs, wrong complete types and private lexical references refuse. Every actual
call carries extra argument/result assertions checked by native inference in all
actual caller contexts. This catches otherwise-valid expanded comparisons with
wrong declared types. An unused-work placeholder supplies no call-context proof.
No-assertion raw checking and no-use source staging preserve inherited behavior.

The implementation uses move-based parent construction, precomputed expansion
costs and bounded source-only tracing. Trace paths use one mutable escaped buffer;
only retained assertion paths allocate copies, with cumulative metadata limits.
Private diagnostics use source spans and assertion indices, not confidential
values or caller-provided paths. Native type comparisons share the registry budget.

Two implementation bottlenecks were found using the full RP-03 variant:

- Definition collection originally scanned ordinary expression/policy/step trees.
  The grammar permits definitions only at declaration boundaries; the collector
  now visits those boundaries and recursively visits scopes only.
- Cached alias-cost lookup originally charged the full descriptor size in addition
  to the later actual copy. Source19 charges the compact cost/provenance summary
  lookup; cache construction and every actual expanded descriptor copy still reserve
  full work/bytes before copying. Source18's conservative accounting is unchanged.

The first attempts refused at the existing lowering quota and were not counted
as successful reductions. Both variants now pass within unchanged quotas. Source19
also carries dependency work into creation analysis rather than starting a fresh
counter, and strips private typing wrappers before ordinary dependency derivation.
These changes neither raise a limit nor omit any actual expanded allocation.

## Complete finite variants

The [experimental files](0090-predicate-variants/README.md) are checked against the
native test-only transformation and current complete C12 documents, including
semantic revisions. Current and historical benchmark files remain untouched.

| Actual whole source | Current bytes / tokens / native parser work | Experimental bytes / tokens / native parser work | Byte reduction |
| --- | --- | --- | --- |
| RP-01 | 55453 /9455 /114301 | 54172 /9145 /113796 | 1281 (2.31%) |
| RP-03 | 410752 /66783 /840430 | 291309 /41306 /386333 | 119443 (29.08%) |

[Raw native measurements](0090-predicate-native-metrics.json) and
[exact package/source/companion digests](0090-predicate-variant-digests.json) retain
reproducible evidence, including the unchanged778-file benchmark inventory.

Counts include all definitions, type descriptions and calls, not fragments or
predicted savings. Parser work is algorithm-specific visit count, not elapsed
performance or user-readability evidence. Bindings retain explicit source-only
identity rows; these are not hidden work/storage sharing.

Fifteen native frontend groups pass, including both complete variants and exact
format/reparse IR; the native extra-type layer has eleven focused groups. Frontend
coverage includes exact input/record/nominal mismatches, purity/cycles/unused nested
calls, zero-input calls/unused-work contexts, managed Unicode renames/reorder/fresh
copy/retirement, lexical privacy/static pins, literal opacity, read dependencies,
and exponential/cumulative bounds. The full local suite passes688 Rust tests plus6 compile-fail tests, native lint,
five Wasm library lints, both release Wasm builds, native portability and all85
Python CI commands. Final-head CI and separate completed-diff review remain required.
A local script initially read a stale portability report; correcting its path to
the freshly generated report resolved that harness failure without a code change.

The complete RP-01 variant passes all eleven checking/advisory views. RP-03 retains
its four ordinary checking views and advisory invalidation result; concurrency and
later dependent views must retain `parallel-effects-unproved` at
`/body/nodes/start_acceptance`. The finite CLI oracle checks all eleven outcomes
against current packages, including changed source-span attribution.

No runtime function, C13 source selection, nominal cast/membership, general exporter,
managed editor, protected inspection, shared work/service template, benchmark
adoption, complete RP-08 or actual user/host/browser/independent evidence is claimed.
All40 scenarios remain Partial, G1–G4 conditional and parent46 draft; Phase1 remains
open. Current package adoption must archive its exact previous epoch before code.

## Verified integration

PR #105 merged as `63988a63dc826dff6776251dbe747d947e17b2aa` after CI43476905305
atb7057abda1a472a3a80df413f7557ec4ddcf1760 (all required steps successful),
a separate completed-diff automated COMMENT review5895660433 and fresh full
comment-body/thread/head audit. All six commits and merge carry sign-off. The
CodeRabbit skip was read in full; it contains no substantive finding and is no
review evidence. No independent/human approval is claimed. Historical pending
statements above describe the pre-integration checkpoint; this section records
the completed integration. Current benchmark adoption remains separate.
