<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.4.0

**Status:** Accepted under [ADR-0031](../decisions/0031-qualified-wait-endpoints.md)
and ADR-0018 owner delegation; effective upon PR merge.
This bounded source binding remains provisional Candidate A, not an executable
admission, complete editor or user-study result.

Inherit [profile 0.3.0](profile-v0.3.md) at commit
`9a530b257ed0fdc2d5b891fea905a1e12ff4dd89`, except for the rules below. The
normative token grammar is the unchanged [0.3 EBNF](candidate-a-v0.3.ebnf).
Use header `Choreoform authoring "0.4.0".`, companion version `0.3.0`, profile
`0.4.0` and the [closed schema](../../schemas/authoring/bindings-0.3.schema.json).
Keep exact core 0.2.0 pin
`urn:choreoform:contracts:core-profile:0.2.0` /
`sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623`
and its inherited core 0.1.0 artifact, plus the unchanged semantics/IR versions.

## Designated endpoint binding

A parent wait policy may explicitly name the wait node in its own designated
reconciliation follow-up, for example:

```text
Rule for closure "Close". Reconcile in "Recovery". End policy.
```

Its wait policy may then use:

```text
On observation outcome "observed" of "Observe service" in "Recovery".
On timer none.
```

The specialized rule applies only to the observation and timer outcome fields of
a wait policy. First resolve the node by ordinary lexical rules. Preserve a
successful ordinary binding, wrong-kind error or wrong-qualification error.
Only when ordinary resolution reports `reference-missing`, and there is an
explicit qualifier, resolve the qualifier as a scope in the policy's own lexical
context. It must be a direct child of that context, and that context's actual
closure policy must designate it as a reconcile follow-up. Resolve the first
name directly inside that child and require the actual node to be a wait using
this exact wait policy. Ancestor fallback cannot export a node through a child.
Observation and timer must retain the same owner and existing outcome symbols.
Existing optional-timer presence, identity, outcome and flow checks still apply.

Scope/policy declaration order does not change this binding. The compiler resolves
these wait outcome fields after declarations and validates actual endpoint nodes
after steps. `wait-endpoint-boundary` identifies a non-designated context;
`wait-endpoint-owner` identifies a non-wait or different-policy node. Reference
and symbol errors retain their own codes and original source spans.

This exposes outcome names at one declared lifecycle endpoint only. Ordinary
data, expression, type, general outcome and arbitrary descendant references keep
their existing lexical privacy. No dummy root wait, public child namespace,
implicit migration or new core meaning is introduced. Source 0.2/0.3 retain their
old refusal of private child endpoints. The [example](examples/reconciliation.choreo)
and [companion](examples/reconciliation.bindings.json) illustrate the new binding;
the separate existing provider/type resources remain explicit library inputs.

## Integrity and evidence boundary

Supported source/companion pairs are exactly 0.2.0/0.1.0, 0.3.0/0.2.0 and
0.4.0/0.3.0; reject mismatches, unknown versions, wrong pins and extra fields.
The CLI embeds semantic/base/new core bytes; library callers supply every artifact.
Package hashing uses ASCII `choreoform-authoring-package:0.4.0`, one zero byte,
the existing big-endian length framing and exact source/canonical companion bytes.
Old package digests remain unchanged. Promoting an otherwise identical 0.3 package
changes package integrity but preserves semantic IR/revision, since both select
the same core. Alias edits preserve permanent symbol/node/flow identities.

The resulting [declaration report](../evaluation/0030-reconciliation-declaration-links.md)
proves neither actual follow-up activation nor entry snapshot, permission,
obligation ownership, provider truth, correlation, multiple-obligation progress
or terminal settlement. A follow-up timer can remain ordinary-data-uncovered;
typing a source sentence does not supply its live timing context. All reports
remain non-executable. General IR export/editing, tool-managed companion creation,
near-English usability, full text/visual cycles, protected accessible editing and
actual representative-user evidence remain explicit Roadmap work.
