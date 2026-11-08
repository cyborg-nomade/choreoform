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

[ADR-0048 / core 0.6](../decisions/0048-general-provider-fault-source-binding.md)
keeps IR 0.2 unchanged and adds only a versioned outer work emission-source field.
General declared uses stay separate from retry eligibility and actual fault/handler
entry evidence; no unknown scope/wire field becomes admitted implicitly.

[Definition IR 0.3](definition-v0.3.md) adds required nullable scope faultInput,
with exact typed local history reference under [ADR-0052](../decisions/0052-typed-handler-entry.md).
It inherits 0.2 wait capture and changes no earlier wire/hash domain. Structural
projection is separate from exact core-0.7 binding and conditional entry proof;
no actual fault/history storage, current authority or execution is supplied.

[Definition IR 0.4](definition-v0.4.md) adds required explicit original-request
retention maps on scopes, selected only by exact core 0.8. Structural declarations
and conditional owned proof remain separate from actual values/storage/authority.
Earlier source profiles and wire/core/hash domains remain fixed.
