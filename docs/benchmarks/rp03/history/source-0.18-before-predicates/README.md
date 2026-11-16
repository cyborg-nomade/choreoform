<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-03: factored two-line order draft

This fictional bounded package uses source **0.18** / companion **0.17** on unchanged
core **0.12** / IR **0.4**. Explicit **Order information** protection and **Order
instruction** type descriptions replace repeated complete clauses without changing
any semantic ID, type/protection value, provider pin, guard or receipt meaning.
The finite fixture producer is not a general editor, exporter or runtime.

Actual native parsing/lowering verifies **410752 UTF-8 bytes / 66783 tokens /
840430 parser visits**, versus 576759 / 84051 before factoring: **28.78% fewer source
bytes**. Whole-program native recognition remains within the unchanged 1M parser
work / 131072 tokens / 1 MiB limits. Real native formatting/reparse preserves the exact
complete canonical candidate, byte-identical to the pre-factoring C12 package:

- Revision: `sha256:a3b8c0f07cfc63d745376c3eb11b67690c98edc3e216d262c1542f00a05798aa`.
- Wire digest: `sha256:6550ac457539457b6e48a452d17d95060db1a2f36fbcf74f8b3c7c1e7c11cfb4`.
- Source: `sha256:2e2be8154b94791d5af9e5e77c87895d205e2455af37063e66b98efbcb645695`.
- Canonical companion: `sha256:6e0274d9c2fff47cbf1fecdda9fd8920dd5ada6e647a457d34b6e2c0af52cfbe`.
- Joint package: `sha256:21d5c4c815ea3139e10f35e47cd6509f5a1c0761800881a96810738188ad24a0`.

Static, control, protection and strict invalidation checks plus advisory pass;
all five emitted report files are byte-identical. Strict invalidation has 312
required obligations, 0 missing, 849 historical origins, 45 uncovered and 34 analyzed
nodes. These are separate declared coverage sets. Six later views still refuse
`parallel-effects-unproved` at `/body/nodes/start_acceptance`, now measured source
bytes 72263..72450. Opaque external effectfulness is not made independent by readable
names, guarded true, provider descriptors or a smaller source.

All 25 logical service/capability/effect/wait instances remain distinct, with 26
invocations and 7 split children. Immutable resource payloads, 23 provider contracts and 23
terminal sources, exact contextual request/retention/capture/compensation links,
22 pure receipt cases and 8 pure guard cases remain unchanged. Three receipt cases
are true / 19 false; they are synthetic declaration inputs, not accepted provider
observations, actual duplicate suppression or a process trace. None is not no-effect.

The exact old current 126-file source 0.16 package was signed as `b8d73a7` **before**
adoption code in [this archive](history/source-0.16-before-factoring/ARCHIVE.md).
Every old file, including its original README, is preserved byte-for-byte; earlier
source 0.14, pre-instance and instance checkpoints remain untouched. Historical
loaders resolve local resources inside their own archive and distinguish then-
current observations from present rechecking. Source profile promotion does not
rewrite those observations or their links/pins.

All RP-03 scenarios remain **Partial**. Reports are non-executable declarations,
not real permission, physical warehouse independence, authenticated current values,
accepted original snapshots, surviving-work provenance, host execution or completed
Phase 1. Canonicalization/physical aliases and admissible revised-surviving overlap,
full validator/near-English/reusable receipt/work/managed protected accessible
text-visual/RP-08/actual user and independent evidence remain open. No Phase 2.
See [ADR-0087](../../decisions/0087-rp03-factored-authoring.md) for adoption authority.
