<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Initial access dependency availability: frozen follow-up plan

**Date:** 2026-11-05<br>
**Baseline:** `ecc29c965191504b18bd98c75d6d9f64f44afe3d` (PR #24)<br>
**Authority:** Owner's ADR-0018 delegation; routine implementation of accepted
ADR-0013/0017/0019 rules. Commit this plan before changing code.

Close the ordinary initialization access-policy availability gap exposed by
ADR-0019. Share bounded access-metadata read expansion across protection and
availability analysis. Include protection-policy, participant-requirement and
capability/authority/effect dependency closures without guessing predicate truth.

At a scope's input transfer, require policy dependencies available from initialized
ancestors before seeding fresh inputs. Do not invent an order among input cells.
For local initializers, require expression and all accessed source/destination
metadata dependencies available from scope entry inputs/ancestors, without ordering
local initializers. Emit explicit `unproved` refusal at the actual declaration.

Include child input and fanout item destination access dependencies in caller
static read requirements and source lowering's fixed-point derivation. These are
metadata reads, not a read of the fresh target cell's unavailable value. Preserve
fresh scope visibility; dependencies on fresh child cells cannot be legitimized
by inserting invisible caller reads.

Regression evidence must isolate missing dependencies, uninitialized/transitive
policy reads, unavailable peer input and local ordering; demonstrate available
ancestor/input cases and consistent source lowering/format/reparse. Retain fixed
budget refusals, all old tests, native/Wasm checks and Python stage oracles.

Remove only the now-proved ordinary initial-access-availability residual from the
protection report after verification. Authentication, policy truth/current host
facts, exceptional paths, races/invalidation, clocks, full benchmarks and Phase 1
exit remain open. Record a separate automated self-review, final-head CI and merge,
then continue the phase without an item-level owner pause.
