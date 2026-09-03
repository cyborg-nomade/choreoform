<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Canonical IR proposal

**Status:** Proposed by [ADR-0009](../decisions/0009-canonical-versioned-ir.md).
Nothing in this directory is an accepted language or compatibility release.

- [Wire specification](definition-v0.1.md): identities, records, ordering,
  semantic digests, version admission, and validation boundaries.
- [Structural JSON Schema](../../schemas/ir/definition-0.1.schema.json).
- [Worked examples and verification](examples.md).
- [Evaluation and remaining gates](../evaluation/0009-canonical-ir.md).

The IR is the interchange contract between future text/visual frontends and
planning. It is neither a user-facing textual syntax nor a runtime checkpoint.
The examples test serialization boundaries, not executable process behavior.
