<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Core profile 0.7.0: explicit immutable handler-local entry

**Contract ID:** `urn:choreoform:contracts:core-profile:0.7.0`

These exact bytes form the profile artifact. Inherit exact core 0.6
`sha256:27bce3fde9a0393abb6bf760a8588af9268de65069b73f4cb946144f5d30cb85`,
0.5 `sha256:50007ac05e5e71df91bb9997a40a06c7a8993df04b1a673421e4a987014299d7`,
0.4 `sha256:0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3`,
0.3 `sha256:30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3`,
0.2 `sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623`
and 0.1 `sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58`.
Require every inherited artifact explicitly. This profile selects Definition IR 0.3;
earlier selected cores retain their prior wires and cannot ignore new scope fields.
Keep all inherited policy/type/expression/capture/retry/source/metadata/protection /
lifecycle rules, with the explicit entry extension below. Core 0.7 joins the finite
metadata-read family and general emission/capture/retry families. No quota increases.

## Scope binding and reserved local cell

IR 0.3 scopes add required nullable faultInput. Null supplies no history binding or
initialization. Non-null is exactly `{history:Ref,result:DataID}`. History pins a
supported provider-fault-history resource under profile
urn:choreoform:fault-history-profile:provider-observations:0.1.0 /
`sha256:2f580f46eb6ef9b7fff42906f6b0b5cafc6c5fa71ae14365828935bfe2da070c`.
Verify all exact descriptor/profile/source/source-profile/provider/type bytes.
Result is owned locally and has exactly the fixed nominal Provider fault history
in urn:choreoform:types:provider-fault-history:0.1.0 /
`sha256:a2e177ce1dd4bad5b9cd68b74e6da7749ec2b04b68a39dae1bd72729e2aae7a7`.
Shape-equivalent or caller-selected nominal types do not satisfy this identity.

Only a local-child scope named by an exact faults handler selection can bind history.
It is not root and has no ordinary ports, ordinary declared creator or P6 follow-up
creator, including unvisited declarations. Forbid an ordinary initializer/writer,
ordinary input/output transfer or competing entry binding for the reserved cell.
Validate all bindings/resources even when no incoming context is explored. Normal
visibility applies to result/source protection references in handler context; no
private-ancestor exception, provider/name inference or ordinary hidden fact exists.

Static declaration rows retain revision/handler/cell/history/source/profile/provider /
nominal metadata and mandatory host requirements only. They do not initialize the
cell. Ordinary nominal wrapping and limited claim-shape checks return no event or
entry receipt and can never activate this binding. P4 history remains immutable.

## Separate conditional handler-body judgment

A supported target must check every incoming declared context for each covered
binding, using exact general-source activity uses and complete before/after active
handler stacks. Source must equal the descriptor's exact source; human/null/legacy,
wrong-source, unvisited, nonordinary or active-handler source contexts are unproved
in this initial subset and refuse. A null source is not proof of no faults. No
incoming context means uncovered binding, not vacuous initialized coverage.

Initial conditional coverage is limited to ordinary covered/reachable provider
source sites with empty active-before stack and available retained ordinary source-
scope protection contexts. It does not reuse ordinary contexts for an exceptional
source activation. Nearest lexical selection, local handling before escape, failed-
selection exclusion and routing acyclicity remain unchanged. Engine faults remain
uncatchable independently of a convenient business-fault name.

The covered handler body is portless, childless and terminal, with a reachable finish
and flat local-only compute/decision/finish nodes. This subset admits no activity,
wait/timer, invocation/iteration/parallel body, ancestor values/storage, parent result
or ordinary continuation. Other fresh local initializers retain inherited rules.
Check the reserved cell's write-access metadata preconditions against the empty
pre-entry value set before conditionally initializing it. Circular/self/ancestor-
dependent access preconditions refuse. Then check initialization and all body access,
read/write/finish availability with only that conditionally initialized history cell.
No caller-supplied history Value is accepted by this static body judgment.

Retain all ordinary source-input, fault-existence/control, provider and creator/owner
protection provenance. Conservative scope label unions may overtaint; they are not
fault-site value snapshots. Convert retained cell/historical labels into immutable
provenance requirements, never live ancestor values, ordinary completion assertions,
mutable dependencies or current grants. Zero inherited values never means zero labels.
Missing contexts refuse. Provider resultProtection alone is only a lower bound.

Before conditional coverage, compare the complete retained envelope to history
initialization and every later publication/control branch, including destination
access dependencies. Enumerate current access obligations and require strict local
invalidation, retaining inherited history/local invalidates requirements. Advisory
requirements are not proof. Keep conditional body sets separate from ordinary or
legacy exceptional coverage. Share bounded graph/flow/metadata/provenance indices
under inherited cumulative allowances; no partial proof or truncated coverage.

## Actual entry and lifecycle premises

A host must authenticate an actual supported non-engine emission and exact definition,
instance/scope/node/attempt/cause/input/request/key/logical-effect correlation; validate
the original evidence reference, mapped name/observation and complete history claim;
select the exact nearest live handler with exclusions; recheck current source/work /
access/management authority; and atomically initialize fresh single-occurrence owned
read-only storage while retaining complete source/control provenance. Static type,
metadata/claim/route checks, digests, labels and Boolean flags discharge none of these.

Preserve immutable history, duplicates/contradictions, monotonic cancellation, actual
occurrence lifetimes, owned handoff and all-obligation settlement. Handler finish
cannot clear cancellation, reopen an activity, resume its parent or produce an
activity result. Permitted owned continuation/cutoff/all-settlement requires separate
support. No resource/report grants IO, dispatch, execution or terminal disposal.

Readable authoring/companion/native selection is a separate explicit versioned
integration. Current source 0.7/core 0.4/IR 0.1 benchmark bytes remain unchanged until
separate adoption. Full validation/benchmarks, general authoring/visual/protected
editing and actual host/user/browser/independent/scenario/phase gates remain open.
