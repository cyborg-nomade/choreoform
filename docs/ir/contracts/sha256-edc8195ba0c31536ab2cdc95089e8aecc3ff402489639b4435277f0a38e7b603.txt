<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Core profile 0.8.0: immutable original requests in owned follow-ups

**Contract ID:** `urn:choreoform:contracts:core-profile:0.8.0`

These exact bytes form the profile artifact. Inherit exact core 0.7
`sha256:e286e54441b3d785ae823eef359c12d64fb8405829c70709227282e5af62f54b`,
0.6 `sha256:27bce3fde9a0393abb6bf760a8588af9268de65069b73f4cb946144f5d30cb85`,
0.5 `sha256:50007ac05e5e71df91bb9997a40a06c7a8993df04b1a673421e4a987014299d7`,
0.4 `sha256:0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3`,
0.3 `sha256:30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3`,
0.2 `sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623`
and 0.1 `sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58`.
Require all inherited artifacts. Select Definition IR 0.4 explicitly. Preserve
all earlier contracts/wires/source profiles, quotas and inherited metadata/source /
capture/retry/history/type/expression/protection/lifecycle rules.

## Declaration and immutable value identity

Every scope adds required retainedRequests, a closed map from local data IDs to
origin activity node IDs. Empty means no retained request. This bounded profile
admits at most one entry per scope. Result belongs to that scope, has exactly the
origin capability input type (including nominal identity), no initializer and no
writer, including unvisited declarations. It is not an ordinary input or port.
Only a direct designated reconciliation child may declare it. It has no nested
scopes/ports/ordinary creator/fault-handler assignment; its terminal closure and
capturing entry wait remain subject to the fresh-local owned-body proof. Origin
is a non-human capability activity declared directly in its owning parent. The
entry wait captures exactly that origin; all capturing waits in this initial body
must use that origin. An ordinary/fault result cannot impersonate this snapshot.

The snapshot is the one immutable typed value actually accepted for the original
logical request, not re-evaluation of its expression at retry, closure, observation
or handoff. A host must bind exact definition/instance/owner/source occurrence,
input/value revisions and digest, original request, logical effect/key/attempt
identity and provenance. Technical retries retain that value and identity. Changed
business input is new work, not replacement of this snapshot or a reset budget.
A declaration or matching value digest authenticates none of those host facts.

## Conditional owned proof

Extend the fresh-local owned-follow-up proof, keeping its old empty-map behavior
and separate ordinary/conditional sets. Supported synchronous creator and exact
origin must have ordinary coverage; an unvisited, exceptional or unexplored origin
cannot supply the proof. Keep every declared body node flat wait/compute/decision /
finish and every complete read/write local, all waits timerless and a reachable
finish. No new effect, nested work, arbitrary ancestor snapshot, foreign write,
ordinary resumption or current value sharing is admitted.

Check target write access against empty pre-entry values before making the snapshot
available. Then check fresh local initialization, capture and every body access /
read/write/finish. No self/peer/ancestor-dependent pre-entry permission is supplied
by retained values. Ordinary analysis does not seed these cells. Conditional
protection retains complete conservative origin/owner/source/control/provider /
input-policy provenance from the one ordinary chain. Convert source cell influences
to retained label references, never current values or grants. Require the exact
activity-provider origin in retained context, compare all initializer/constant /
branch/publication destinations and require strict local invalidation. Snapshot
identifiers cannot make ancestor cells available. No advisory requirement is proof.

Publish immutable revision-bound static origin/result/type declarations and explicit
conditional owned relations/obligations, sharing immutable evidence and charging
indexes, work, copies and rows cumulatively. Later legacy reconciliation availability
keeps these already proven scopes separate, retaining their prior owned proof and
leaving their nodes outside its legacy checked sets. It must neither erase an
explicit snapshot nor invent one by re-running legacy fresh-local availability.
Failure returns no partial proof. No
report takes actual request values, creates storage, dispatches work or accepts a
handoff. Source/CLI selection follows a separate explicit authoring version.

## Mandatory live premises and open boundaries

Fresh occurrence-owned immutable storage is installed at original request acceptance
before owner disposal, with authenticated complete source/input/control provenance.
Retention/transfer requires current source/target data and management authority,
receiver acceptance and atomic single-owner handoff. Preserve original evidence,
duplicates/stale/contradictory observations, cancellation monotonicity, exact live
selection and all subscriptions/effects/obligations until settled. A failed or
unaccepted handoff leaves the old owner accountable and closure pending. Current
access and capability/work authority must be checked again for any later new work.

No parent resume, activity success, cancellation clearing, irreversible rollback,
subscription disposal, terminal settlement or actual live allocation follows.
Permitted effectful continuation and cancellation/closure cutoff/all-settlement
need separate contracts and evidence. General exceptional/observable-channel /
lifetime protection, actual host/browser/user/independent evidence, complete
validator/benchmarks/editing and all phase gates remain open.
