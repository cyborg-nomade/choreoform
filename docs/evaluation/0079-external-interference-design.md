<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Inactive external-interference design and pure model evidence

**Date:** 2026-11-12<br>
**Frozen plan:** `023cd58`, [scope](0079-external-interference-design-plan.md)<br>
**Dependency:** #93 signed merge `30ed6f75ec91954d2a807625a486603318f94bdf`,
CI `43055059855`, automated COMMENT review `5861732179`<br>
**Critique before selection:** `3f4b91a`, [options](0079-external-interference-design-critique.md)<br>
**Decision:** [ADR-0079](../decisions/0079-external-interference-draft.md)

## Selected artifact and boundary

The [inactive draft](../dialects/drafts/external-interference.md) and two closed
schemas select an exact-provider/exact-profile supplement, finite typed original-
request selectors and exhaustive activity/terminal footprints. Namespace resources
bind a common canonicalization authority and exact component types. Explicit alias
mappings, not provider IDs, decide region equality. Read/write and write/write
intersection conflicts; read/read overlap is permitted. Unsupported/opaque inputs
remain unproved. Runtime locks are not an admission alternative in this design.

Exact versions, UTF-8 byte pins, C12/P3 binding, 16-region footprints, four-component
keys, eight-step paths, type/option/nominal boundaries, shared 100000-work/1MiB/depth64
limits and private 1024-byte reports are specified. No existing resource/schema/
profile/core meaning, production Rust, source, diagnostic, accepted contract or
benchmark/generator/report bytes change. The draft is neither registered nor
consumed by the current checker. Ordinary authors will use managed readable
bindings in a future version; hash maintenance is not an authoring flow.

The comparison is conditional on actual validated provider/input invariants,
truthful exhaustive behavior, common canonical authority and original-request
correlation. The pure model verifies representations and exact nominal resource
bytes but **does not execute nominal invariants**. A deliberate invariant-invalid
representation test still returns a result with the assumption prominently present;
this documents the limitation, not an accepted nominal value or provider request.
It is not a trust/admission API, process run, canonicalizer or host monitor.

## Thirteen executable model test groups

| Test group | Actual finite result |
| --- | --- |
| Closed schema/profile/example bytes | Exact draft/profile/provider/namespace pins and reproducible synthetic files |
| Read/write rule | Disjoint writes pass in model, same write/read or write/write overlaps; shared reads pass |
| Aliases and authority | Different keys/namespaces can conflict through one canonical target; unknown mappings, different authority and duplicate bindings remain unproved |
| Record/Option/integer paths | Present admitted leaf resolves; absent/missing/null/noncanonical/overflow refuses |
| Actual pinned RP-03 named types | Explicit representation paths use unchanged real type/provider bytes; missing/repinned nominal bytes or implicit nominal erasure refuse; invariants remain assumed |
| Original/revised/retry/terminal/compensation identity | Attempt-only changes preserve logical request; changed input/revision/cohort on same occurrence refuses; new revised work is compared against the old key; current-request field refuses; compensation has new identity |
| Closed shape/class rules | Unknown versions/profiles/fields/coverage/steps/leaf kinds, duplicate JSON keys, provider/namespace byte substitution and read-only/terminal writes refuse |
| Privacy and limits | Bounds/depth/cardinality failures publish only fixed unproved; reports expose no keys/domain/request digests or exception text |
| Existing source/evidence | Actual effectful parallel source still refuses; all benchmark file hashes unchanged; document links and CI command checked |
| Composite text/integer keys | Exact Unicode spellings stay distinct until the common assumed canonicalizer maps them together; wrong arity refuses |
| Namespace and unselected-channel validation | Unknown namespace versions/profile/fields/type mismatch refuse; invalid terminal selector is checked even in activity mode; opaque terminal remains unproved when used |
| Unique descriptor selection | One exact provider may be reused at different occurrences with one descriptor; a conflicting descriptor revision refuses |
| Shared work exhaustion | Enough work for setup and first branch cannot publish partial success when second branch exhausts; path/footprint maxima refuse |

Run `.tools/ir-check/bin/python tools/check_external_interference_draft.py`.
Synthetic fixtures are mathematical inputs, not generated accepted observations.
Existing complete fresh source controls are checked through the real native CLI;
no new core acceptance or whole RP-03 progress beyond its prior refusal is claimed.

## Next implementation sequence

1. Freeze a separate contract/resource-integration scope. Specify unique semantic
   descriptor selection and exact dependency closure under a new core version,
   closed namespace/authority/descriptor readers, unknown/old-version refusals and
   immutable normative artifact publication. Do not mutate C12 or old P3 bytes.
2. Implement bounded Rust raw checking/resource/type plumbing with native/Wasm
   parity, pin/substitution/unknown/counterexample/privacy evidence. Validation of
   a descriptor still does not prove external independence or grant execution.
3. Separately prove generic original-request separation through supported source
   construction and nominal invariants for every overlapping/surviving occurrence,
   or retain the current refusal. A concrete sample table cannot fill this gate.
4. Version source/companion managed bindings and source/raw equivalence evidence;
   only then evaluate honest benchmark descriptor adoption, preserving history.
   Obtain real provider/authority/host support/truth evidence before execution.

Cross-revision lifetime and capture/terminal correlation, current protection/actor
permissions, actual quiescence of every possible attempt, durable settlement and
atomic ownership transfer remain required. A cutoff/lease/pin/result never proves
these alone. No runtime execution/registration/user/browser/independent review is
claimed. Parent #46 draft, forty/all four Partial, G1–G4/full/optional/RP-08/validator/
near-English/protected accessible text-visual/actual external evidence and Phase 1
exit remain open. No Phase 2.

## Verification and review

All 602 Rust tests and six compile-fail doctests pass, with format, native and all
five library Wasm lints, release portability/frontend Wasm builds, native portability
and all 75 Python commands. Logs: `/private/tmp/interference-draft-workspace.log`
and `/private/tmp/interference-draft-verify.log`. The targeted final thirteen-group
oracle passes after the review corrections below; final-head CI reruns the full set.

A separate completed-diff automated review checked correctness, privacy/security,
bounds/performance, scope, licensing and the assumptions/evidence boundary. It found
and fixed two gaps: two work items could select different descriptor revisions for
one exact provider; and a renamed logical effect could evade original-request
consistency for the same actual activity occurrence. Both now refuse with regression
controls. The review also confirmed unselected-channel type checking, common-authority
alias handling, all-or-nothing budget failure and the deliberate absence of nominal
invariant/provider/host validation. Production code, accepted normative bytes and
current/historical benchmark files remain untouched. This is automated self-review,
not an independent human review or GitHub approval.

Final-head CI and full review/thread/CodeRabbit audit must pass before the signed
expected-head parent merge. A skipped bot supplies no review evidence.
