<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Core profile 0.5.0: explicit timerless accepted-payload capture

**Contract ID:** `urn:choreoform:contracts:core-profile:0.5.0`

These exact bytes form the profile artifact. Inherit exact core 0.4
`sha256:0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3`,
0.3 `sha256:30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3`,
0.2 `sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623`
and 0.1 `sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58`.
Require all inherited bytes explicitly. This profile requires Definition IR 0.2.
All old core/source/wire versions retain their previous meaning and refusals.
All policy payloads, including retry faultSource and finite metadata closure,
retain core 0.4 shape/meaning; core 0.3/0.4/0.5 share that metadata family.

## Per-use capture declaration

The IR-0.2 wait node has required nullable capture. Null keeps the inherited
noncapturing meaning. Non-null is exactly `{source:Ref,origin:NodeID,result:DataID}`
and requires this core on the node's wait policy. An older wait policy permits
only null capture. The declaration is per actual node use, not a shared ancestor
policy binding into arbitrary private data. Resolve origin and destination by
normal lexical visibility from the wait's actual scope; no sibling/private
exception or occurrence/retained-value inference is allowed.

Source pins supported provider-terminal-observation-source 0.1 metadata with
profile `urn:choreoform:observation-source-profile:provider-terminal-facts:0.1.0`,
revision `sha256:a224fc2b8318af2ff8dfc9955b55af03f4d5b7bd0de719de0384421698d20ead`.
Verify source/profile/provider and all nominal type bytes. Origin must be a
visible capability activity whose exact provider ID/revision equals the source
provider. Human/wrong-kind/type-equivalent other providers are unproved. Check
every declared capture, including ordinary-unvisited nodes; metadata equality
never establishes a live origin occurrence or actual source/attempt correlation.

The source's exact normalized observation type equals both the wait policy's
observationType and destination cell type, preserving nominal identity. Timer is
null for this initial subset; the sole ordinary completion is onObservation.
Nonterminal provider states cannot become accepted terminal payloads through a
label, user type, source override or fabricated success flag.

## Writes, protection and availability

A capturing wait declares exactly the destination cell in writes. Derive complete
node dependencies from wait match/authority, scope management, source provider
protection references and destination access/protection, retaining every inherited
control dependency and read. Metadata origin references do not read prior runtime
input values or create control edges. Do not assume private ancestor snapshots.
Derived dependencies remain complete and budgeted; conservative declared node
reads retain all inherited access/invalidation obligations.

Source envelope is precisely the provider resultProtection. Apply inherited P2
no-declassification transfer to the target for every declared capture, including
unvisited uses, and include ordinary control provenance/access obligations in
ordinary protection analysis. Destination-only labels are insufficient. Actual
current read/write/transfer permission, principal, purpose and authority are
required at acceptance, not granted by static envelope comparison.

Only a supported authenticated observation, exact origin occurrence/attempt /
logical effect/request/key/input/observation correlation, fresh authority and true
match can atomically write the typed payload and emit onObservation once. A
cancelled ordinary wait, unmatched payload, duplicate, stale/contradictory or
nonterminal message cannot initialize or overwrite the cell or complete again.
Timer branches never initialize captures; they are deliberately outside this
subset. Post-wait availability follows from the sole accepted completion; pre-wait
reads still require an independently available value. Every capture participates
in write inventories, information flow, invalidation and conflict/lifetime checks.

## Declared evidence and remaining gates

Static declaration evidence names exact revision/site/source/profile/provider /
origin/result and explicit live acceptance/current protection/correlation/cutoff /
ownership requirements. It supplies no observation/value, accepted fault, retry
permission, host authentication, occurrence owner, execution handle or plan.

Preserve the existing data-free isolated-survivor proof and zero-inherited-value
reconciliation entry bounds. Adding a local capture cell in a surviving body does
not prove its lifetime; retain the appropriate unproved refusal until an explicit
owned-storage proof is implemented. No generic retention, fault-data port, handler
result conversion, parent reopening or ordinary continuation is introduced.
P3/P4/P5/P6 actual source truth, acceptance order, stable-key/no-effect, cancellation /
closure cutoff, duplicate/contradiction history and all-obligation settlement remain.

Authoring and bounded native disclosure require a separately explicit supported
source/companion profile. Current source 0.7/core 0.4/IR 0.1 package bytes stay
unchanged. Full validator/benchmarks, visual/protected accessible editing and
actual user/host/browser/independent evidence remain separate phase gates.
