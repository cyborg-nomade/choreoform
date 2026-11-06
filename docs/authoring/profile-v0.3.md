<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.3.0

**Status:** Accepted under [ADR-0027](../decisions/0027-explicit-retry-authoring.md), effective upon merge,
and ADR-0018 owner delegation. This is a bounded source/package evolution, not
final syntax, user-study evidence, executable admission or a process engine.

This version inherits the lexical, declaration, identity, visibility, symbol,
generated-slot and lowering rules of [profile 0.2.0](profile.md) and
[bindings 0.1.0](bindings.md) as present at commit
`0eb2b77f0258b8fbe8bc7ec1c23b61cae48a9e43`, except for the explicit changes below.
Its full normative grammar is [candidate-a-v0.3.ebnf](candidate-a-v0.3.ebnf).
Old grammar/profile/schema/example bytes and old package digests stay unchanged.

## Source sentence

The header is exactly `Choreoform authoring "0.3.0".`. In a work policy, after
Retry faults and before Timeout, write an explicit binding for multiple attempts:

```text
Attempts 2.
Delays [1000000 microseconds].
Retry faults ["temporarily unavailable"].
For retries use clock "Elapsed clock", basis "Audit basis".
Timeout none.
```

The clock reference resolves to a visible companion resource alias of kind clock;
the basis resolves to a visible data declaration. The full reference syntax
(including lexical qualifiers) is unchanged. No hashes, wire IDs or JSON fields
appear in this sentence. There is no timeout/scheduler default or zero-delay
exception. With one attempt the sentence must be absent; nullable clock/basis
fields lower to null. Multiple attempts require the sentence, even when delays
are zero or the eligible-fault set is empty. Syntax recognition alone permits
the optional clause; binding/lowering enforces its attempt-count requirement.

Lower the clause to the exact core 0.2.0 retry clock Ref and basis ID. Include
the basis and transitive clock-source protection/actor/capability reads in work
policy and node dependencies. Preserve stable work/data/actor/generated/symbol
identities. Do not infer one domain from equal integer coordinates. Retry and
timeout clocks may differ, and their references/influences must stay separate.

## Companion and contract selection

Use format `choreoform-authoring-bindings`, companion version `0.2.0`, profile
`0.3.0`, and [the separate closed schema](../../schemas/authoring/bindings-0.2.schema.json).
The record inventories and fields retain their earlier shapes. Exact core pin:

- `urn:choreoform:contracts:core-profile:0.2.0`
- `sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623`

The [extension](../dialects/core-retry-v0.2.md) requires the exact core 0.1.0 base
artifact as well. Semantic contract and IR envelope remain their existing
versions/pins. Every lowered type/expression/policy record uses the selected core
dialect alias. No mixed legacy work records are synthesized by this source profile.

Only these source/companion/core combinations are supported:

| Source | Companion | Core |
| --- | --- | --- |
| 0.2.0 | 0.1.0 | 0.1.0 |
| 0.3.0 | 0.2.0 | 0.2.0 |

Reject mismatched source/companion versions, wrong pins, unknown profiles and
extra fields. The parser selects the grammar by explicit header; it never admits
the new sentence in an old file. The fragment API retains its legacy grammar
default; new clauses are tested in full versioned programs.

## Integrity and editing boundary

Keep the existing exact-source, JCS-companion, resource-pin and semantic-revision
hash rules. Only the joint package domain changes to ASCII
`choreoform-authoring-package:0.3.0`, then one zero byte, unsigned 64-bit big-endian
source length, exact source bytes, unsigned 64-bit big-endian canonical-companion
length, canonical companion bytes. Source 0.2.0 keeps its old domain byte for byte.
Changing profile/core meaning deliberately changes the semantic revision; stable
declaration/symbol IDs are not replaced. No automatic migration is provided.

The Rust CLI supplies immutable built-in semantic/base/new core contract bytes
for this profile; callers still explicitly supply clock/profile/source and other
external artifacts. Library callers supply every required artifact, including
the inherited base. Resource digests are integrity, not authority. Formatting an
original package preserves candidate IR and timing uses after reparsing; it does
not export arbitrary IR, restore comments or provide a complete editing product.
Readable resource renames with coordinated companion updates preserve wire pins
and semantic revision; that synthetic operation is not a user-interface study.

## Required live evidence

The [core contract](../dialects/core-retry-v0.2.md) and timing report enumerate
accepted fault correlation/stamp, registered clock domain/origin, checked delay
addition, unchanged request/input/basis revisions, current work/target/source
authority, effect key/no-effect evidence and atomic durable ownership/budget
enforcement. A sentence, matching hash or static report supplies none of those
live judgments. All reports remain non-executable. Full semantic validation,
benchmark coverage, ordinary tool-managed authoring without manual companion
bookkeeping, text/visual round trips, protected accessible editing and actual
representative-user evaluation remain explicit Roadmap work.
