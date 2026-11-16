<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.5.0

**Status:** Accepted under [ADR-0034](../decisions/0034-benchmark-authoring-profile.md)
and ADR-0018 owner delegation; effective upon integration into main through the
parent benchmark PR. This remains provisional candidate lowering.

Inherit [profile 0.4.0](profile-v0.4.md) at commit
`482adf7c2871e2766e0ac645628a1a2c38a872ce`, except below. The normative grammar
remains the unchanged [0.3 EBNF](candidate-a-v0.3.ebnf). Use header
`Choreoform authoring "0.5.0".`, companion version `0.4.0`, profile `0.5.0` and
[closed schema](../../schemas/authoring/bindings-0.4.schema.json). Retain exact core
0.2.0 and its inherited core 0.1.0 bytes, semantics and IR versions. Libraries
must still receive all resource bytes explicitly; the CLI's embedded core bytes
supply no provider, clock or host implementation.

## Bounded whole-program recognition

Only the exact first three lexical tokens `Choreoform`, `authoring`, `"0.5.0"`
of a program select a 16,384-token quota. Whitespace/comments follow normal lexical
rules. A malformed remainder still fails recognition. Unknown versions, wrong
prefixes and fragments keep the 4,096-token limit. Recognizing this supported
program uses at most 1,000,000 pattern visits; older profiles/fragments retain
100,000. `Syntax::token_count()` and `parser_work()` expose deterministic counts,
not performance promises or semantic proof. The independent Python oracle counts
its own states, not native pattern visits.

Keep the 1 MiB source/companion limits, 64-level delimiter/AST/ledger/core limits,
512 recognizer recursion limit, 100,000 lowering work bound and all resource,
semantic and report bounds unchanged. Refuse rather than truncate on exhaustion.
The full RP-01 draft required 8,945 tokens and 101,388 native pattern visits in
the frozen measurement. Quotas have bounded headroom; they are not inferred from
input size or arbitrary version strings.

## Actual-use follow-up endpoints

Keep ordinary name lookup first for wait-policy observation/timer outcome owners.
A successful binding, wrong-kind error or qualification error never falls through.
Only missing lookup with an explicit qualifier uses the specialized rule.

First try the existing ordinary scope lookup. If it is missing, only this profile
may traverse a fully qualified descendant scope chain ending in the exact root
process name, for example:

```text
On observation outcome "observed" of "Observe Claim checks"
  in "Claim checks recovery" in "Claim checks" in "Expense reimbursement".
```

Each qualifier must be an actual scope, and the target must be a strict descendant
of the policy's declaring scope. Look up the node directly inside that scope.
After all steps bind, require an actual wait using this exact policy and require
either:

- the policy's declaring scope has a reconcile closure designating that exact
  direct child (the existing 0.4 relation); or
- there is at least one declared capability activity whose target's effect policy
  names this wait policy, and **every** such activity owner's actual reconcile
  closure designates that exact direct child.

The second condition indexes all declared uses, including ordinarily unvisited
ones. Capability declarations alone do not establish a use. A nested descendant
of a follow-up is not its direct designated boundary. Incompatible owners or an
unused relation refuse with `wait-endpoint-use-unproved`. Non-wait/different-policy
nodes retain `wait-endpoint-owner`; wrong qualifiers/kinds keep reference codes.
Observation and timer must select the same actual owner and supported outcome
symbols. Deferred binding preserves declaration-order independence and original
UTF-8 source spans. One policy-to-owner index avoids scanning every node for each
endpoint. Other data, expressions, general outcomes and arbitrary node references
retain existing lexical privacy. Old profiles keep their narrower rules.

## Integrity and evidence boundary

Strict supported source/companion pairs are now 0.2.0/0.1.0, 0.3.0/0.2.0,
0.4.0/0.3.0 and 0.5.0/0.4.0. Reject unknown/mismatched pairs, pins and extra fields.
Package hashing uses ASCII `choreoform-authoring-package:0.5.0`, one zero byte and
the existing length framing. Old hashes remain unchanged. Promoting an otherwise
identical supported 0.4 package preserves its semantic IR/revision, while changing
source, companion and package integrity.

These are declaration bindings. They establish no live ownership, lifetime,
correlation, effect settlement, parent-value survival or execution authority.
The isolated declaration witness passes supported stages when its activity is
ordinarily unvisited; invoking it still exposes `scope-lifetime-unproved`. The
full draft lowers but separately fails its initial static information-flow check.
Neither result upgrades a reimbursement scenario or any Phase 1 gate. General
editing/export, visual parity, full validator/proofs and actual users remain open.
See [evidence](../evaluation/0034-benchmark-profile.md).
