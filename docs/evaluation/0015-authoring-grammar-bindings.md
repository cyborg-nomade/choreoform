<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Evidence: Candidate A grammar and portable bindings

**Status:** Proposed specification with bounded executable checks<br>
**Date:** 2026-10-03<br>
**Baseline:** `24680d9c8a34013db4551fe2ab391c2c2c403e4d` (accepted PR #18)<br>
**Evaluator:** Proposal author; decision authority: Project Owner

## Question and evidence boundary

Can the accepted Candidate A direction be specified without map/policy/identity
placeholders and exercised through reproducible grammar/ledger checks?
[ADR-0015](../decisions/0015-authoring-grammar-bindings.md) asks for this bounded
specification decision. Candidate revision is the exact review commit containing
this file, the EBNF, schema and tests. This is engineering conformance evidence
within [ADR-0014's frozen plan](0014-authoring-plan.md), not a new candidate ranking
or a retrospective rescore of its human-facing criteria. Its original comparison
and all forty Partial scenario rows remain unchanged.

Author, implementer and evaluator are the same agent. No independent reviewer,
participants, accessibility results or measured A/B preference are invented.
Python serves as test infrastructure only; Rust remains the selected product
implementation language. No runtime contracts or historic IR fixtures change.

## Artifacts and observed checks

| Artifact/check | Evidence supplied | Limit |
| --- | --- | --- |
| [EBNF](../authoring/candidate-a.ebnf) interpreted by [oracle](../../tools/authoring_probe.py) | Closed lexical/production vocabulary, complete consumption, bounded recognition | No typed AST, semantic admission or deterministic lowering |
| [Test catalogue](../../tools/check_authoring_profile.py) | 106 positive fragments across 52 named grammar entries; all eleven action forms, nine type descriptors, expression operations, twelve policy kinds and mode alternatives | Fragments deliberately omit referenced declarations; some are ill-typed |
| [Terminal source](../authoring/examples/terminal.choreo) | Whole-program recognition and whitespace/CRLF/nested-scope variants | One small specification fixture; not a full RP benchmark |
| Grammar refusals | Unknown/trailing/missing/reordered clauses; empty names/maps where forbidden; wrong block endings/profile; unsupported numbers/escapes; resource limits | Recognition errors are oracle exceptions, not stable product diagnostics |
| Exact decimal helper | Scale, negative zero, signed-64 edges and integer-only conversion | Literal conversion only; no expression evaluator |
| [Companion schema](../../schemas/authoring/bindings-0.1.schema.json) plus invariants | Root/tree connectivity, global live/retired IDs, cross-kind shadowing, symbol roles/keys and generated-slot uniqueness | Does not compare companion with source or enforce node subkinds |
| Pure ledger rename/lookup models | IDs/wire symbols/slots/unknown annotations survive alias rename; collisions fail without modifying input; exact case/qualification and no child-data leakage | No source rewrite or full editing transaction implementation |
| Copy/delete/recreate model | Fresh IDs accepted, tombstones prevent reuse | No recursive source/annotation copy or live-instance migration |
| Qualified join token model | Rejects an alias that would silently match an unlisted child outcome sharing its wire token | Caller supplies the template population; no graph pairing/execution validation |
| Pin verification | Accepted snapshot pins plus exact local bytes; missing/corrupt resources and changed contract pins refused | Hash equality does not establish supported provider/clock/calendar enforcement |
| Integrity vector | Fixed length-framed joint hash; JSON formatting preserves bindings hash, source formatting changes source/package hash | No IR semantic revision or round-trip proof |
| Strict JSON | Duplicate keys, float/non-finite/unsafe integers, negative numeric zero, BOM, invalid UTF-8/surrogates, trailing data and depth refused | Protected-source access enforcement is not implemented |

`tools/check_authoring_profile.py` contains 17 test methods, with the positive
catalogue and mutation cases run as subtests/loops. It passes locally in the
existing hash-locked oracle environment. The new check is wired into existing CI.
The test count is not a coverage percentage or a semantic completeness claim.
In particular, grammar recognition cannot establish that a `true` invariant
proves safety, that a provider exists or that a policy is enforceable.

Local regression results: 40 Rust unit/integration tests and 3 compile-fail docs;
16 wire tests, 4 snapshot tests and 6 original authoring-inventory tests; 89 native
report cases with 15 JCS/hash comparisons; and the text oracle's 3 complete
comparisons, 3 stable export cycles and 7 refusals. Rust formatting/native clippy,
all three Wasm library clippy checks and the portability Wasm release build pass.

Reproduce from repository root:

```sh
.tools/ir-check/bin/python tools/check_authoring_profile.py
.tools/ir-check/bin/python tools/check_authoring_evidence.py
.tools/ir-check/bin/python tools/check_contract_suite.py
.tools/ir-check/bin/python tools/check_ir_fixtures.py
# With the pinned Rust toolchain and built text CLI:
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
.tools/ir-check/bin/python tools/check_text_prototype.py
```

The existing CI additionally compiles/lints the Wasm libraries. That is build
regression evidence; the new Python specification oracle is not browser runtime
evidence. Runtime/native-browser parity is not re-evaluated by this change.

## Concrete design findings

The paper syntax needed more than punctuation: expression parameter maps,
composite literal-versus-constructor preservation, extra conservative reads,
exact port/purpose aliases and semantic/core-suite pins needed explicit homes.
The proposed formats supply those homes. They also make policy clauses mandatory
instead of treating natural-language phrases such as “retry safely” as executable.

Outcome aliasing exposes a real mapping hazard. If two split children both report
wire token `complete`, a join sentence naming just one child's “complete” cannot
mean only that child: the accepted IR predicate tests tokens across the paired
population. The proposal refuses that misleading form and requires every member
outcome with that token to be explicitly listed. This is a source-level precision
restriction, not a new IR join operator. The unit model exercises the collision.

Separate package and semantic revisions are necessary. Even adding a source
newline changes source/package hashes while preserving the persistent ledger.
JSON reformatting preserves its JCS hash. Neither observation proves unchanged
IR meaning; that requires a validated frontend. The ledger rename model similarly
establishes ID and annotation retention only, not that source references changed.

## Corpus and conditional gates

All forty scenarios inherit their existing **Partial** status from
[the ADR-0014 traceability table](0014-near-english-authoring.md#corpus-traceability).
No complete RP-01, RP-03 or RP-08 source/IR definition is supplied here. Their
specific remaining work is retained:

- RP-01: full revision-bound payment/retry/reconciliation graph, supported
  providers and precise timer/calendar construction.
- RP-03: accepted exact aggregation/capture operations and item interfaces,
  physical-effect cutoffs, compensation and guarded-commit evidence.
- RP-08: owned follow-up and handover graph, bounded ad-hoc work design and
  partial-order outage import contract; ordinary waits do not implement import.

G1–G4 remain conditional: faithful/protected whole-process meaning, complete
mapping and contracts, round trips, and accessible equivalent editing are
unproven. G5's bounded checkability improves for grammar/ledger properties only;
full benchmark/implementation evidence remains incomplete. No criterion total,
new fitness ranking, usability score or final language selection follows.

## Recommended continuation

Review and accept/amend this proposed boundary before building the Rust frontend.
Then implement typed parsing, exact source/ledger inventory checks, binding and
resource admission, deterministic lowering, preserved IDs/annotations and
import/export round trips. Carry full benchmark contract gaps alongside that
implementation, without silently expanding the accepted contracts. Prepare
matched A/B and inline-anchor study materials, coordinate participants with the
owner, and evaluate results before closing the authoring item. Pause before
visual notation or any next Roadmap deliverable.
