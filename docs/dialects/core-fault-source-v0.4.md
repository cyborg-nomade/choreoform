<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Core profile 0.4.0: explicit provider retry fault-source binding

**Contract ID:** `urn:choreoform:contracts:core-profile:0.4.0`

This profile inherits the exact core 0.3.0 artifact
`sha256:30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3`,
its core 0.2.0 base
`sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623`
and core 0.1.0 base
`sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58`.
All inherited bytes and this artifact are explicitly supplied and verified.
All inherited rules apply except the retry/source and metadata-family extensions below. Old
bindings keep their old rules; no source or definition migrates implicitly.
These exact file bytes form the profile artifact; no self-digest is embedded.

## Closed retry record

Work policy retry R is exactly:

```
{maxAttempts:N,delays:List<Duration>,faults:Set<Name>,
 clock:null|Ref,basis:null|ID,faultSource:null|Ref}
```

For maxAttempts 1, faults is empty and clock, basis and faultSource are null.
For maxAttempts greater than 1, faults is nonempty and all three references are
non-null. Attempt count, delay count/coefficients, retry clock/domain/basis,
dependencies and all current P4 acceptance rules retain their inherited meaning.
Unknown/omitted/additional fields refuse; an available registry resource or a
matching fault label is never an implicit binding.

FaultSource pins a provider-fault-source 0.1.0 with exact supported profile
`urn:choreoform:fault-source-profile:provider-observations:0.1.0`, revision
`sha256:f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0`.
Verify exact source/profile/provider/type bytes. Every eligible fault name must
be an emission declared by that source. The source's already-validated observation
names and evidence class remain declarations, not accepted facts.

For an eligible unresolved emission this conservative profile additionally
requires the source provider's declared idempotency to be stable-key. Without it
the retry evidence is unproved, including read-only providers without that support.
A definitive-no-effect emission does not require stable-key metadata, but actual
definitive correlated evidence remains mandatory before a retry without a stable
key. No class, source pin or provider declaration proves implemented support.

## Actual-use identity and dependencies

At every declared activity using a multi-attempt policy of this profile, require
capability mode and exact equality of the source provider ID/revision and the
actual capability provider binding. Same types, labels, names or effect classes
are insufficient. Inspect ordinary-unvisited uses too. Human contexts are outside
this provider-source proof and refuse as unproved. An unused policy may have a
validated source declaration without claiming an actual use, entry or activation.
All emission names of bound sources, including noneligible faults, contribute to
conservative fault-route/unhandled-name accounting. This enumerates declared
possibilities without inferring an actual emission or active handler.

Source-provider result-protection policy/actor/capability references contribute
metadata edges to the work policy's exact read closure, just as inherited
provider/clock metadata does. Core 0.3 and 0.4 records use the same finite metadata
closure semantics, so a mixed 0.3/0.4 component is eligible. Strong cycles and
cyclic components containing older legacy records still refuse. All direct,
authority, basis and transitively derived reads remain exact; no supplied read
set, safe Boolean or receiver claim substitutes for derivation.

## Evidence and live boundaries

Static binding checks prove only the declared source/name/provider relationships.
Library retry-use evidence identifies exact source/profile/provider pins and
eligible fault/observation/evidence declarations, bound to the definition revision
and actual use site. Old core retry uses have no inferred source binding.
All reports remain non-executable and preserve their other residual obligations.

Before accepting/emitting/routing a fault or dispatching any retry, a supported
host must authenticate exact original provider evidence, attempt/occurrence,
request/input/basis revision, logical effect and key; retain duplicates and
contradictions; enforce current authority/protection, actual stable-key or
definitive no-effect evidence, attempt/delay budget and atomic success/cancellation/
ordinary-close cutoff. Provider-source faults are not engine faults; an engine
fault cannot become catchable or retryable through a matching label. P6 owned
reconciliation, accepted handoff and all-obligation settlement remain required.

This contract accepts no observations, grants no authority and implements no
dispatch, retry, fault-data binding, wait-result capture or ordinary continuation.
Authoring support requires a separately explicit compatible source/companion
profile. Full validator, irreversible/domain safety, benchmark and phase evidence
remain separate gates.
