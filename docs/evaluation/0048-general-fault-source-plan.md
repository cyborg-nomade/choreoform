<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# General provider fault-source use: frozen raw-core plan

**Date:** 2026-11-07<br>
**Baseline:** `a9b08627a3f9db54b69b70fc47228ec4d0918307`, draft #46 after #62<br>
**Authority:** ADR-0018; [fault-entry audit](0048-fault-entry-contract-audit.md)

Use a dedicated stacked branch/PR and ADR-0048. Publish exact core 0.6 inheriting
all five existing core artifacts, IR 0.2 and finite metadata-family semantics.
Change only the versioned work payload: add required nullable outer `faultSource`
as general declared provider-emission metadata, independent of retry count and
eligibility. Reuse the exact existing provider-fault-source profile/reader; no
new trusted flag, payload/normalizer override, emitted event or history value.
All earlier wire/core/source/schema/fixture bytes and behavior remain unchanged.

For N=1 keep inherited empty eligible faults and null retry clock/basis/faultSource;
outer faultSource may be non-null. N>1 keeps every inherited retry rule and requires
outer source to equal the exact inner retry source. Null outer source means no new
general emission-use declaration, not proof of no possible faults. Verify metadata
and source-provider protection read closure even on unused policies. Validate every
actual non-null-source activity use, including unvisited nodes: capability mode,
exact actual provider ID/revision, normal policy/resource visibility and no human
provider-source inference. Preserve all inherited type, effect, clock, stable-key,
source/capture/protection and cycle/refusal rules; core 0.6 inherits capture support.

Expose private-constructor immutable revision/site/work-policy/source/profile /
provider declarations and shared all-name evidence/observation metadata through
the retained static report. Distinguish all declared emissions from eligible retry
names and actual emitted/accepted faults. Include unvisited declared uses with no
activation inference, but no row for an unused policy. Reuse immutable metadata /
revision storage and cumulative resource budget; empty legacy paths add zero work.
No host support, receipt, occurrence, current authority, dispatch, fault-data entry
or execution handle is returned. Actual non-engine emission classification, source /
cause/input correlation, current protection, selection/cancellation/cutoff and owned
settlement remain explicit requirements.

Extend the conservative fault-name catalogue from general source metadata for core
0.6, including single-attempt/noneligible names; retain the exact old path for
cores 0.4/0.5. Deduplicate pins and keep name/resource/graph work bounded under one
existing allowance. Scope/name routing still supplies no actual emitting-site or
handler snapshot. Do not add a handler data binding or typed history value here.

Keep a closed explicit known-core list/helper where needed for correct inheritance;
retain old verification/refusal order and all old hash behavior. No broad unknown
version acceptance or quota increase. Test raw core-0.6 IR-0.2 single/multiple
attempts, exact source agreement/pins/bases/provider identities, human/unvisited /
unused policy contexts, complete source reads/metadata cycles, sharing/work limits,
all inherited capture and retry stages, catalogue/eligible distinctions, legacy
absence/refusal and unchanged current package reports. Add finite independent
Python contract/hash/binding/catalogue vectors, not generated observations/events.

Run relevant native/Wasm/all Python/final-head CI checks, separate completed-diff
agent review, all unresolved threads/CodeRabbit summaries and signed expected-head
merge into draft #46. Then freeze explicit readable source/companion selection and
bounded native use disclosure, followed by exact typed fault-history/source/control
protection and reviewed handler-local entry representation. Current benchmark
package remains source 0.7/core 0.4/IR 0.1; no implicit adoption. Owned continuation /
cutoff/all-settlement, full validator/benchmarks, general authoring/visual/protected
editing and all actual external/scenario/phase gates remain open.
