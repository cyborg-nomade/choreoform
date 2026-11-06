<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Settlement receiver authority dependencies: frozen plan

**Date:** 2026-11-06<br>
**Baseline:** `6170c615bda47e985bc03e018e2d9da1e81ecc54` (PR #35)<br>
**Authority:** ADR-0018 delegation; ADR-0013 P6 and ADR-0019 dependency mechanism.

The remaining-validator audit found that settlement transfer checks an ancestor
owner link but omits the receiver's cancellation/management actor requirement
from policy dependency closure. Source lowering also omits it. P6 explicitly
requires current receiver acceptance authorized by that authority; its data reads
must not disappear from declaration/availability/protection/invalidation analysis.

Before changes, reproduce a transfer with a distinct child cancellation actor
and a receiving ancestor management requirement reading ancestor data: omitted
transfer reads currently pass static checking. Then close the receiver actor
requirement edge in static policy checking and source dependency derivation.
Validate receiver cancellation policy kind and actor/requirement visibility in
the receiver's actual lexical context. Include only its management authority,
not the receiver's own children-settlement policy or unrelated lifecycle reads.
Propagate the edge through existing cancellation/membership/join dependencies.

Preserve all old contract/source/schema bytes and both supported source versions;
this corrects an existing rule and needs no new dialect. Reject missing declared
reads and unsupported links; preserve cancel/retain null-owner behavior and
existing ancestor restrictions. Charge normal edge/member work through existing
closures; do not cache current authority, manufacture receiver acceptance or
change ownership. Live occurrence/liveness/atomic transfer evidence remains host
work; no exceptional-flow, executable or Phase 1 claim follows.

Tests cover omitted/exact/transitive reads, distinct child/receiver authorities,
inherited receiver cancellation, receiver kind/visibility errors, unrelated
children-settlement dependencies, one-hop propagation to child nodes and source
format/reparse/report cycles in both profiles. Run relevant native/Wasm/artifact/
stage checks, perform separate automated review, require final-head CI and
expected-head merge, then continue the validator/lifecycle audit and Phase 1 work.
