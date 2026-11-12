<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Near-English factoring: critique before inactive probes

**Date:** 2026-11-12<br>
**Frozen scope:** `b81edda`, [plan](0083-near-english-factoring-critique-plan.md)<br>
**Dependency met:** #97 signed merge `7f26305021e28b8598582ac1a02a0888891d65c0`,
CI `43247994354`, automated COMMENT review/fresh full audit `5877161955`

ADR14/15 select structured named steps and explicit sentences provisionally,
with managed identities and no raw JSON in ordinary authoring. The owner prefers
broad accessibility and plain English; quoted names/fixed clauses are a current
precision tradeoff, not final user evidence. Existing grammar represents types,
expressions, policies and scoped work explicitly, with exact private bindings.
The Rust Syntax tree preserves token/byte extents; lowering operates over it with
strict ledger resolution. Fixture Python is a generator, not the authoring editor.

RP-01 source is 61211 bytes/10223 tokens and RP-03 576759 bytes/84051 tokens.
Those previously measured full files contain repeated structural type descriptions,
protected cells, receipt predicates and scoped request/recovery declarations.
RP-08 has a complete case brief and paper walkthrough, not a full current lowerable
source package. A new prose sketch cannot fill that gap. New measurements below
must distinguish repeated text, semantic structure and real comprehension evidence.

## Options to measure before selection

| Option | Human benefit | Semantic/implementation constraints |
| --- | --- | --- |
| Local reusable type/expression/protection names | Familiar names can replace repeated structure; smallest compositional extension | Transparent type aliases must preserve exact nominal identity; predicate parameters/fact contexts need static typing; policy defaults must not hide access or authority |
| Hygienic parameterized scoped blocks | Removes repeated service/retention/receipt corridors while keeping named purposes | Every use needs correct fresh scoped work/storage/endpoints and stable managed IDs; direct-child ownership, lexical privacy, compensation references and exact provider recipient claims cannot be shared or widened |
| Domain vocabulary/libraries | Familiar task words and reuse can aid domain authors | Definitions must expand to inspectable accepted meaning; no guessed verbs or opaque callbacks; domain-specific words cannot become hidden core semantics |
| Formatting/progressive disclosure alone | Lower visual load without semantic change | Does not remove full-source duplication; protected summaries must retain uncertainty/authority and need actual accessibility/user evidence |

Paper-only sketches illustrate intent, not a chosen grammar or working compiler:

```text
Describe a "Reservation instruction" as a record with ... .
Keep "Request" as a "Reservation instruction" under "Order access".
```

```text
Describe "A reservation service" once, with a request, an adapter and recovery rules.
Use "A reservation service" here as "Reserve first line" with ... .
```

Ellipses and parameters are deliberately unfinished. The first could preserve an
anonymous type's exact structure; it must never create/erase nominal membership.
The second needs a real hygienic instantiation contract and cannot be implemented
as unchecked string replacement. Neither is claimed parseable or semantically
complete, and shorter spelling supplies no human accessibility measurement.

## Required evidence before direction

Measure actual token/byte extents and exact repeated outer type/expression spelling
using the existing bounded Rust parser; do not add overlapping category counts
or call redundancy a measured savings figure. Compare actual IR declaration/type /
policy families and the complete process intents, including unknown effects and
recovery. Any inactive expansion probe follows this signed critique checkpoint.
If asserting equivalence, actually lower/check the finite supported comparison;
paper examples remain paper. No representative-user feedback may be generated.

The first selected implementation must have meaningful whole-process benefit and
preserve companion identity, private scope, P3 pins/recipients, actual-use endpoint /
retention/capture/compensation relations and cumulative expansion/output budgets.
It needs a separately signed grammar/companion/version plan before production code.
Current source0.16 remains C12; C13 selection is a distinct explicit version issue.
Full export/editing, protected accessible text/visual parity, RP-08 and actual
representative-user evaluation remain Phase 1 work, not a larger-parser substitute.
Generic region proof and every other unproved phase gate remain open; no Phase 2.
