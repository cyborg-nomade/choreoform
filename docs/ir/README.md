<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Canonical IR structural foundation

**Status:** Accepted structural foundation under
[ADR-0009](../decisions/0009-canonical-versioned-ir.md).
This is not a complete executable language or a compatibility release.

- [Wire specification](definition-v0.1.md): identities, records, ordering,
  semantic digests, version admission, and validation boundaries.
- [Structural JSON Schema](../../schemas/ir/definition-0.1.schema.json).
- [Worked examples and verification](examples.md).
- [Immutable contract artifacts and provenance](contracts/README.md).
- [Evaluation and remaining gates](../evaluation/0009-canonical-ir.md).

[Definition IR 0.2](definition-v0.2.md) adds an explicit nullable wait capture under
[ADR-0045](../decisions/0045-explicit-timerless-wait-capture.md) and a
[separate closed schema](../../schemas/ir/definition-0.2.schema.json). Core 0.5 supplies
the initial timerless source/origin/result/type/protection subset. The old wire and
constructor remain fixed at 0.1; a new explicit constructor selects 0.2. Structural
projection is not observation acceptance, runtime execution or phase admission.

The IR is the interchange contract between future text/visual frontends and
planning. It is neither a user-facing textual syntax nor a runtime checkpoint.
The examples test serialization boundaries, not executable process behavior.
