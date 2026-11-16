<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Provider terminal-observation source profile 0.1.0

**Contract ID:** `urn:choreoform:observation-source-profile:provider-terminal-facts:0.1.0`

These exact file bytes are the immutable profile artifact. A resource declaring
this profile supplies only the following closed shape:

```
{kind:"provider-terminal-observation-source",version:"0.1.0",id:Name,
 profile:Ref,provider:Ref}
```

Verify the exact source/profile/provider bytes and every provider-referenced type
artifact using the existing provider-contract 0.1.0 and accepted type rules.
An ID or matching representation is no substitute for exact revision identity.
Unknown/missing/additional fields or unsupported profiles/versions refuse.
No resource field can override normalized payload types, protection, states,
trusted identities, current permissions or implemented host support.

## Derived declarations

Inspect every already-validated provider observation by its declared state, never
by spelling or Unicode normalization. Preserve exact original observation names.
Derive terminal name/state/tag declarations as follows:

| Provider state | Normalized terminal tag | Payload type |
| --- | --- | --- |
| succeeded | confirmed | Exact declared provider output type T |
| rejected | rejected | Correlation record C |
| cancelled | cancelled | Correlation record C |
| no-effect | noeffect | Correlation record C |
| acknowledged or unknown | No terminal declaration | None |

C is the closed Record of `logical effect:Text`, `request digest:Text` and
`observation:Text`. The normalized observation type is a Variant with exactly the
tags present among derived terminal declarations. Several success observation
names may share confirmed and the same exact provider output type. At least one
terminal case is required. Preserve all nominal pins/names; no representation
unwrapping or fabricated result type occurs. Nonterminal observations remain in
the provider metadata but cannot become terminal by being named paid or safe.
A succeeded business-problem output is still a business result, not proof that
physical payment occurred or that any unknown effect was rejected.

The source protection envelope for every normalized case is precisely the
provider's declared resultProtection, including the correlation record. There is
no destination-supplied label, type-only provenance or implicit declassification.
Definition-context policy/actor/capability visibility, complete dependencies and
current access/transfer permission still require subsequent use-site checks.
Bound all derived names, cases and type copies by the existing resource work /
depth allowance. Cached immutable metadata never creates a fresh allowance.

## Metadata and acceptance boundaries

This profile describes a possible supported normalization interface. It supplies
no actual observations, authenticated provider/normalizer, terminal state, accepted
fault, stable-key implementation, effect settlement or proof of truth. Correlation
field types alone do not validate their values; confirmed payloads also require
the full authenticated correlation envelope before acceptance, even though that
envelope is not embedded in T.

A supported host must authenticate the provider and implemented normalizer against
these exact declarations; validate payload and declared provider observation /
outcome; correlate original effect, attempt, occurrence, request/key, immutable
inputs and observation identity; enforce current authority/protection and engine
acceptance order; retain late/duplicate/contradictory facts; and enforce success,
cancellation and closure cutoffs plus accountable all-obligation settlement.
Rejected/cancelled/no-effect are terminal only with actual definitive correlated
evidence; a network error, timeout, label or unknown observation is insufficient.

No existing definition/wait/retry implicitly binds a supplied resource. A reviewed
explicit use/capture representation and source/protection/type/flow judgments must
precede any retained value. The closed IR 0.1 wait variant gains no new fields.
This resource does not introduce ordinary/fault ports, timer initialization,
surviving storage, handler result conversion, continuation or execution authority.
All existing core/source/provider/wire artifacts and benchmark reports retain
their meaning. Full validator, scenario, host/user/browser and Phase 1 gates remain.
