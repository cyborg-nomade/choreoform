<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Whole RP-03 symbol binding critique

**Date:** 2026-11-11<br>
**Baseline:** #86 head `f85d8b884216fd99df54809dfc695bd3fb1c3bbb`, merged
as `bb0b280971519cfe67dc3f9a791e2e33d731a9dc` before probes<br>
**Frozen plan:** `b2874c5`, [scope](0072-rp03-symbol-binding-audit-plan.md)<br>
**Decision:** [ADR-0072](../decisions/0072-rp03-symbol-binding-direction.md)

## Exact cause, not a new permission

The actual frontend's failed lookup is:

| Dimension | Actual request | Existing declaration |
| --- | --- | --- |
| Owner | `accept_payment` | `accept_payment` (Authorize branch) |
| Role | `outcome` | `outcome` |
| Readable name | `authorization recorded` | `authorization-recorded` |
| Stable wire ID | Not resolved | `authorization-recorded` |

`Lower::step` handles `finish` by resolving its name in the current scope's outcome
symbols. The failing source is the `Finish authorization` step,
`Finish as "authorization recorded".`. Its scope interface and the original
acceptance join both name `authorization-recorded`. The companion row
`accept_payment_outcome_authorization-recorded` has that same exact name and wire.
`tools/benchmarks/rp03_graph.py` constructs the scope/outcome/join consistently but
uses a separate inconsistent finish literal. The frontend's refusal is correct;
it is not an identity visibility, type, resource or core-profile permission gap.

The limited Python generator audit captures emitted action metadata while calling
the unchanged generator. Every generated file still equals its committed bytes.
It checks all 56 **direct finish** action names against their owning scope's outcome
symbols and finds only this one mismatch (`accept_payment_done`). This is not a full
source/symbol/reference/type/semantic validator; invoke/join/port/policy/dependency /
ordinary/owned and later full-case obligations are not inferred from the count.

## Actual test boundaries and rejected repairs

Five SDK groups and four Python/native groups retain all production checks and
exercise real source packages with exact resources/pins. Bounded witnesses are new
synthetic declarations, not edited versions of the full order. Matching variants
pass all eleven native checking/advisory views with `executable:false`.

| Witness | Actual result |
| --- | --- |
| Original full promotion | `symbol-mismatch at 0..0`, empty stdout; test-only request `(accept_payment,outcome,authorization recorded)` |
| Full source unchanged, only companion name changed to spaces | Same runtime error, test-only request changes to `(accept_payment,outcome,authorization-recorded)` at interface binding |
| Bounded declared hyphen caption, finish with space | `symbol-mismatch` at finish name lookup |
| Bounded companion-only rename | `symbol-mismatch` at the still-hyphenated interface name |
| Bounded all captions hyphenated | Pass, finish wire `authorization-recorded` |
| Bounded all display captions consistently use spaces | Pass, same finish wire `authorization-recorded` |
| Extra alias caption reusing the same wire | `symbol-duplicate` |
| Extra alias with a distinct wire not declared by the source | `source-ledger-inventory`, whole-source span |
| Root outcome symbol changed to branch role | `symbol-owner` |

The inventory span assertion uses the actual complete bounded source extent; an
initial incorrect fixed byte-count expectation was corrected without changing
production diagnostics. None of these witnesses is an actual observation, execution,
effect, prior eligibility, authority or compensation settlement.

No full source literal or guard has been corrected in this audit. The only temporary
full-case mutations beyond the already accepted header/pair/core promotion are the
explicitly failing companion-only rename witness. Every committed RP-01/RP-03 byte,
generator, resource, guard and manifest remains unchanged. No full-case IR is
constructed directly or inferred; subsequent native semantic stages are not run.

## Instrumentation and runtime privacy

The private `cfg(test)` module records at most one current refusal tuple per test
thread plus a count. It captures the actual real-frontend lookup, avoiding guesses
from the generic 0..0 diagnostic. Threads do not share mutable recorder state; each
audit test clears its own state. Production/native/Wasm builds have no recorder,
new public API, debug endpoint, source-literal diagnostics or changed refusal code /
span. Runtime lookup still uses the same exact name map and `symbol-mismatch` error.
The existing compiler quota doc comment now includes already supported source 0.16;
no quota or production behavior changed.

## Recommendation and next dependency

Freeze a separate correction plan for the generator's one finish literal, naming
the already declared hyphenated outcome. Keep stable wire IDs, interface/join,
source clauses/resources/protected labels/receipt guards/original scenarios fixed
apart from that exact caption correction and any explicitly planned profile adoption.
Preserve the original package/refusal evidence when publishing a new checkpoint.
Run the complete corrected source through the real frontend; record every next
genuine first failure and only run later stages if a real candidate exists.

Do not change exact name semantics, normalize punctuation, add aliases, omit source
clauses or claim the one correction will admit the whole order. A consistently
friendly caption with a stable wire is already representable; broader near-English
factoring/managed identities/protected accessible editing/export/text-visual/user
evaluation remains explicit Phase 1 work. Production diagnostics at generic 0..0 and
the full validator catalogue remain separate completeness work.

## Verification and open gates

Local verification passed 580 Rust tests plus six compile-fail doctests, pinned
workspace/five-library Wasm lints, release portability/frontend Wasm builds, native
portability and all 68 Python workflow checks. Separate completed-diff delegated
review checked correctness, security/privacy, performance, scope and evidence; no
remaining actionable audit-scope issue. Required exact final-head CI/full thread /
summary audit precedes signed expected-head merge. Logs:
`/private/tmp/rp03-symbol-audit-{targeted,python,workspace,verify}.log`. CodeRabbit skips supply no evidence;
agent review is automated, not independent/human/personal owner approval. Compiled
Wasm is not browser execution.

Forty scenarios Partial, G1–G4 conditional and parent #46 incomplete; full/optional
RP-01/03/08/validator/near-English/managed identity/protected accessible text-visual /
actual user/host/browser/independent/phase gates stay open. All live eligible effect /
occurrence/current authority/new key/immutable storage/registration/receiver/cutoff /
atomic handoff/all-transitive settlement/conflicting-evidence obligations remain
host gates. Independent correction work remains; no external blocker or Phase 2.
