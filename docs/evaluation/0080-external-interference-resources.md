<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Standalone provider-region metadata: implementation evidence

**Date:** 2026-11-12<br>
**Frozen scope:** `cba5e61`, [plan](0080-external-interference-resources-plan.md)<br>
**Publication before code:** `a28608358f394e06504cb1bc1bc3bedbc575717f`,
[exact pin](0080-external-interference-publication.md)<br>
**Decision:** [ADR-0080](../decisions/0080-external-interference-resources.md)

## Actual implementation

`Resources::provider_interference` and `external_region_namespace` decode standalone
metadata under the new exact profile. They use existing strict resource transport,
byte pins, full P3 input/output/observation/result-protection/class checks, V1/V2
nominal/type/invariant-static checks and one cumulative budget. Both finite channel
shapes/paths are validated. Names/Refs, footprint counts, key arity, steps and scalar
leaves are bounded before allocation/traversal; unsupported shapes/profiles/types
and substituted/missing resources refuse. Exact authority bytes must be nonempty
UTF-8; their trust, behavior, registration and actual aliases remain unproved.

Immutable Arc caches share provider/namespace metadata; selector checks borrow
only the path through Type, retaining the nominal definition Arc during recursive
representation traversal. No whole-provider type clone occurs per selector.
A failed parent descriptor is not cached as a successful result, while validated
children remain reusable. Cached calls still consume the same finite budget.

No live request, physical canonicalizer, comparison, independence Boolean, semantic
definition binding, source syntax, new core/IR or existing report/diagnostic behavior
is added. Namespace and descriptor getters retain explicit remaining obligations.
Multiple stored versions are not semantic selection. A future definition contract
must select one descriptor per exact provider and retain its full dependency closure
in semantic revision. Current C12 source behavior and all old accepted artifacts /
benchmarks/history/generators/reports remain exact. Draft79 stays inactive history.

## Fourteen Rust groups and four independent Python groups

Rust groups exercise real complete resource graphs, including unchanged real
RP-03 provider/type bytes. They cover exact metadata and Arc reuse; opaque/finite /
read-only/irreversible forms; effect/class/coverage/terminal write refusals; unknown
fields/profiles/versions; missing/substituted authority/core/profile/namespace/provider
resources; malformed provider observations; composite record/Option selectors;
explicit named representation and implicit-erasure refusal; missing nominal and
invalid unused invariant/deep graph refusals; invalid path/leaf/arity in either
channel; UTF-8/path/region/component limits; failure recovery and multiple stored
versions; duplicate pins/duplicate JSON; cumulative cached-read exhaustion without
reset; and 1024-field input with maximum finite footprints using borrowed paths.
No request value or accepted event is supplied to these metadata APIs.

The independent Python oracle verifies exact source/snapshot/profile publication,
immutable draft79 bytes, closed schema examples and provider/namespace/authority
pins, links/licensing/required CI command and unchanged actual source results.
Adding unused new resource bytes does not select or validate them under C12:
actual lowered IR is identical, static reports remain executable:false and the
same effectful source still refuses parallel-effects-unproved. The actual new
reader evidence is the Rust SDK suite, not those unused resources.

Commands: pinned `cargo test -p choreoform-semantic-check --test interference_resources
--locked` and `.tools/ir-check/bin/python tools/check_external_interference_resources.py`.
No synthetic input is claimed as accepted provider/user/host observations. Static
invariant validation is real here; actual invariant-satisfying runtime inputs still
need their own admission. No real canonicalizer or actor authority is established.

## Next dependency and open gates

Freeze a separate definition-binding/core-version scope. Exact provider wrapper
selection and transitive resource closure must enter semantic definition identity;
old core/source versions keep their refusal and unknown new fields remain invalid.
Resource metadata support alone supplies no generic independence proof. Separately
prove supported original-request region separation for every admissible overlapping /
surviving occurrence or retain current parallel-effects-unproved. Managed readable
bindings, source/raw equivalence, protection/lifetime/ownership/settlement and actual
provider/authority/host evidence remain required; no runtime locking shortcut.

Parent #46 remains draft, forty/all four Partial. G1–G4/full/optional/RP-08/complete
validator/near-English factoring/protected accessible text-visual/actual users /
browsers/hosts/independent and Phase 1 exit remain open where unproved. No Phase 2.

## Verification and separate review

All 616 Rust tests and six compile-fail doctests pass, with format, native and all
five library Wasm lints, release portability/frontend Wasm builds, fresh native
portability and all 76 Python commands. Logs:
`/private/tmp/interference-resources-workspace.log` and
`/private/tmp/interference-resources-verify.log`. The targeted final SDK suite also
checks actual Boolean/Decimal/list/Option/variant input leaves cannot masquerade
as the declared Integer key. Final-head CI reruns the complete required set.

A separate completed-diff automated review checked correctness, security/privacy,
budget/cache/failure/performance boundaries, compatibility, scope/licensing and
all evidence claims. It confirmed exact authority integrity remains separate
from trust, both finite channels and unused nominal declarations are checked,
selectors borrow types, failed parents do not become cached successes, and caches
never reset the cumulative budget. No remaining actionable findings were found;
a Markdown table separator was corrected in the decision index. The review
retains the exact draft/production quota and storage-versus-selection distinctions.
No current source/core/concurrency gate is relaxed, no model enters production,
and old accepted artifact/benchmark/history/generator/report bytes are untouched.

Final-head CI and the fresh full comment/thread/CodeRabbit audit are required
before the signed expected-head parent merge. This is automated self-review, not
independent/human or personal owner approval; a skipped bot supplies no evidence.
