<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Provider-observation fault-source profile, version 0.1.0

**Contract ID:** `urn:choreoform:fault-source-profile:provider-observations:0.1.0`

This independently pinned metadata profile describes declared provider-observation
to fault-name mappings for later explicit definition binding. It does not change
core-profile 0.1/0.2/0.3, provider-contract 0.1.0, retry records or source grammar.
Its exact frozen bytes are a profile artifact; a resource pins their ID/revision.
No digest inside this file binds the file to itself. Changed normative bytes
require a new explicit version/binding. No URI permits network discovery.

## Closed resource and references

```
{kind:"provider-fault-source",version:"0.1.0",id:Name,
 profile:Ref,provider:Ref,emissions:Map<Name,Emission>}
Emission = {observations:Set<Name>,evidence:"definitive-no-effect"|"unresolved"}
```

Ref is exactly `{id:Name,revision:"sha256:" + 64 lowercase hexadecimal digits}`.
Verify exact resource/profile/provider bytes in the bounded explicit registry.
The resource ID equals its supplied identity; profile equals this implemented
profile artifact. Provider must be a supported provider-contract 0.1.0, including
all exact type resources its metadata requires. Unknown fields, kinds, versions,
profiles, malformed pins/set flags or arbitrary hashed profile bytes refuse.
Emissions and each observation set are nonempty, within existing collection/work/
transport limits. Names are exact Unicode strings without normalization, wildcard,
substring, inferred aliases or a spelling-based safety classification.

Each emission key declares one fault name. Every observation name resolves to an
actual entry of the pinned provider's closed observation table. One observation
may occur in at most one emission in this source; ambiguous mappings refuse.
Unmapped observations emit no fault through this source. No fault is emitted by
merely parsing this resource or receiving an unaccepted transport message.

## Declared evidence classes

| Evidence | Permitted provider observation states | Meaning |
| --- | --- | --- |
| definitive-no-effect | rejected, cancelled, no-effect | The supported adapter contract must attest definitive no-effect for the exact logical effect/request/attempt; the mapping alone is not that fact |
| unresolved | unknown | The logical effect remains unresolved; no success, failure or no-effect conclusion follows |

Succeeded and acknowledged observations cannot be mapped as faults in this
profile. A success must retain its ordinary result/outcome; an acknowledgement
alone is not a failed attempt. A mixed/wrong evidence class, missing observation
or unknown evidence spelling refuses. A provider's declared state classification
is metadata, not proof of truthful implementation or authenticated evidence.
Rejected/cancelled have definitive no-effect meaning only under the supported
adapter attestation already required by P3; HTTP responses and cancellation
requests do not acquire that meaning through this profile.

## Separate binding and live enforcement

This resource does not bind itself to an activity, work/retry policy or handler.
Existing core/source versions have no such field and must not infer it from
available registry bytes, names or a matching provider. A later explicit supported
definition binding must pin this source, match its exact provider to the actual
capability use, resolve eligible fault names and check the applicable retry rule.
Fault provenance/protection, handler context and ordinary/exceptional lifecycle
judgments remain separate; source metadata is not an accepted fault-data port.

Before any actual fault emission/routing, a supported host authenticates the
observation and verifies its exact provider contract, source, attempt/occurrence,
original request/input revision, key and logical effect, current authority and
protection. Preserve original evidence, duplicates and contradictions. No string,
digest, source declaration, trusted/safe Boolean or decoder result discharges this.

Retry additionally requires P4's eligible accepted fault, budget, elapsed pinned
delay, unchanged original input/request/basis, current authority and cancellation/
ordinary-close cutoff. Unresolved effects require supported stable-key identity
with identical request digest or definitive correlated no-effect evidence as
applicable. A source may classify unknown even for a provider without stable-key
support; that does not authorize its retry. Success never retries. Ownership,
accepted handoff, all-obligation settlement and terminal disposal remain P6 gates.

The metadata reader performs no IO, observation acceptance, fault emission,
dispatch, retry, cancellation or settlement. It returns immutable declaration
information and outstanding requirements only, never executable authority.
