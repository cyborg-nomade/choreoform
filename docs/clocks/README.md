<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Clock resources and host boundary

[ADR-0023](../decisions/0023-clock-resource-contract.md) defines the first supported
clock metadata profile, [instance microsecond observations](instance-microseconds-v0.1.md).
The [immutable profile snapshot](../ir/contracts/sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt)
is separately pinned by each clock resource. It preserves P5's explicit domain,
origin, microsecond, trust-source and monotonicity obligations without choosing
an ambient machine clock or civil-time convention.

`Resources::clock` decodes exact metadata and its read-only Integer tick source;
it does not authenticate that source, read time, dispatch a capability or admit
a timer. Non-null definition timers and source clock/calendar resources remain
unsupported until typed dependencies, information flow and host obligations are
integrated. See [evidence and follow-ups](../evaluation/0023-clock-resource-contract.md).
The Phase 2 reference engine and real host support remain separate deliveries.
