<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Core profile 0.6.0: general declared provider fault-source use

**Contract ID:** `urn:choreoform:contracts:core-profile:0.6.0`

These exact bytes form the profile artifact. Inherit exact core 0.5
`sha256:50007ac05e5e71df91bb9997a40a06c7a8993df04b1a673421e4a987014299d7`,
0.4 `sha256:0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3`,
0.3 `sha256:30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3`,
0.2 `sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623`
and 0.1 `sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58`.
Require all inherited bytes explicitly. This profile requires Definition IR 0.2
and inherits timerless capture, exact retry/source/effect/clock/type/protection and
finite metadata rules. Old wire/core/source versions retain their previous meanings.
Core 0.3/0.4/0.5/0.6 form the same finite metadata-read family; strong or legacy
unsupported cycles still refuse. No new wire field or quota is introduced.

## Work emission source, separate from retry eligibility

Only this profile's work policy adds required nullable outer faultSource:Ref|null.
Every other policy payload and the full inherited retry object retain core 0.5
shape. Outer faultSource pins supported provider-fault-source 0.1 metadata with
profile urn:choreoform:fault-source-profile:provider-observations:0.1.0 /
`sha256:f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0`.
Verify exact source/profile/provider/type bytes, closed metadata shape, normalized
names, declared provider observation states and evidence classifications. No type,
state, host support, protection or safe flag override can stand in for that reader.

For one attempt, inherited eligible faults stay empty and retry clock/basis /
faultSource stay null. Outer faultSource may be non-null, declaring possible
provider emissions even when none is retry-eligible. For multiple attempts, retain
every inherited eligible-name/class, delay/basis/clock/authority/effect/stable-key
rule and require outer source to equal the exact inner retry source. Null outer
source supplies no general use binding; it never proves that no fault can occur.
All declared emissions remain distinct from the subset eligible for retries.

For every activity using non-null outer source, including ordinary-unvisited ones,
require capability mode and exact actual capability provider ID/revision equal to
the source's provider. Type-equivalent other providers, human modes and matching
labels are unproved contexts. Shared work policies must satisfy each actual use;
an unused policy verifies its source metadata but supplies no activity use row.
No activation, occurrence/attempt, actual emitted name, cause, input revision or
current source/access truth follows from declaration equality.

## Dependencies and declaration evidence

Include source-provider protection references in complete work-policy metadata
read closure, even for unused declarations. Preserve normal visibility, all strong
and inherited dependencies and the least bounded finite closure. Actual node reads
retain policy/scope/actor/capability/source access dependencies; immutable metadata
caches cannot cache current permission or source truth.

The conservative possible fault-name catalogue additionally includes all names in
outer source metadata, including one-attempt/noneligible declarations. Deduplicate
source pins and charge resource/name/graph work to the same cumulative allowance.
The scope/name routing analysis remains an overapproximation: a name/handler route
is not an admitted emitting-site relation or authenticated handler entry snapshot.

Immutable static use evidence names exact definition revision/activity/work policy /
source/profile/provider and all declared emission/observation/evidence metadata.
It covers unvisited declarations without implying activation; unused policies have
no use row. Share immutable metadata and one revision per result under cumulative
work/copy limits. The report supplies no observation, accepted fault, retry permit,
current authority, fault-history value, handler-data port or execution handle.

Actual supported non-engine emission truth/classification, authenticated original
occurrence/attempt/logical effect/request/key/input/cause correlation, fresh work /
source/access authority, actual eligibility/delay/cutoff, exact handler selection /
exclusion, cancellation monotonicity, retained provenance and owned settlement
remain required. A misleading fault name cannot make an actual engine fault
catchable or turn a business result/uncertainty/cancellation into another class.

Readable authoring/companion selection and bounded native disclosure need a
separately explicit supported profile. Current source 0.7/core 0.4/IR 0.1 package
stays unchanged. Typed fault history/handler entry, permitted owned continuation /
cutoff/all-settlement, full validator/benchmarks, general authoring/visual/protected
editing and actual user/host/browser/independent/scenario/phase gates remain open.
