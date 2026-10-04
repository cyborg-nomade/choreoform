<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.2.0

**Status:** Accepted provisionally in [ADR-0015](../decisions/0015-authoring-grammar-bindings.md),
owner approved 2026-10-04, effective upon merge of PR #19.
This completes a concrete grammar specification for the accepted working
direction, not the authoring deliverable or a supported compiler. The Rust
JSON-record profile 0.1.0 remains unchanged. The version numbers identify
different source profiles, not IR versions; both target IR 0.1.0 and the
[accepted executable contracts](../dialects/README.md).

The normative token grammar is [candidate-a.ebnf](candidate-a.ebnf). Its
metasyntax uses quoted exact terminals, sequence, `|`, parentheses and postfix
`?`, `*`, `+`. Uppercase identifiers are lexical classes. There is no empty
alternative, precedence convention, callback, JSON escape clause or free-form
executable sentence. Prose here supplies lexical, binding and lowering rules
that a syntax recognizer alone cannot enforce.

## Reading and writing a file

Each file starts `Choreoform authoring "0.2.0".` and contains exactly one root
`Process` block. Each root or nested `Scope` starts with its complete interface:
entry, inputs, outputs, outcomes and four policies, in that order. Declarations
then appear in any order; all are collected before binding. A scope is physically
nested in its parent and its `within` reference must resolve to that same parent.
Steps need one action followed by optional conservative reads and ordinary
outcome destinations. Declaration order and indentation never imply execution.

The [terminal example](examples/terminal.choreo) and its
[companion](examples/terminal.bindings.json) are a small complete syntax package.
The permissive actor predicate exists only to keep this example small; it is
not production authorization guidance. This package has no executable IR or
runtime evidence. [Benchmark walkthroughs](benchmarks.md) remain paper evidence.

```text
Step "Manager review".
  Ask "Manager" to decide using "Current request" under "Review rules"
    and record in "Manager decision".
  On "approved", continue with "Finance review".
  On "correction needed", continue with "Correction".
End step.
```

This fragment needs the referenced declarations. The companion associates its
second human-facing outcome with, for example, `correction_needed`; authors do
not write that token. A result type uses `outcome "correction needed" of "Manager
review"` as its case name when it means this specific outcome. A literal quoted
case name instead means exactly that string, without alias conversion.

## Tokens and bounds

- UTF-8 only; reject malformed encoding, BOM and unpaired surrogates. Preserve
  Unicode bytes/spelling without normalization or case folding.
- Outside quotes, only space, tab, CR and LF are whitespace. `#` comments end
  at LF or EOF. Punctuation is `. , ; [ ] ( )`; quoted punctuation is ordinary
  string content. Keywords are exact case-sensitive ASCII words, including
  `read-only` and `stable-key`. Unknown words are refused.
- `TEXT` is a double-quoted string with only `\"` and `\\` escapes. C0/C1
  controls, Unicode surrogate code points and Unicode line/paragraph separators
  are forbidden inside.
  `NAME` is the same class with a nonempty decoded value. Text literals may be
  empty. Names that are canonically equivalent remain distinct. A production
  frontend must diagnose confusables with both source locations; the oracle
  establishes exact distinction, not comprehensive confusable detection.
- `INTEGER` spells `0` or `-?[1-9][0-9]*`; `NAT` spells `0` or `[1-9][0-9]*`.
  `NUMBER` permits the same optional minus plus an optional decimal point and
  at least one fractional digit. Leading zeros, plus signs, exponents and
  separators are absent. Type checking additionally rejects negative decimal
  zero, coefficients outside signed-64 bounds, scales outside 0..18 and
  nonpositive join thresholds. Integer literals have signed-64 bounds too.
- A decimal literal is `decimal 1.20 with 2 places`. Its scale must match the
  exact number of fractional digits; scale zero has no decimal point. Conversion
  to a coefficient uses integer digits only, never host floats. `1.20` without
  its decimal marker is not a literal. There is no inferred currency or rounding.
- Input source and companion are each bounded to 1 MiB, nesting to 64.
  Companion arrays are each bounded to 10,000 entries by schema. The joint
  canonical package is bounded to 2 MiB plus its framing. Lowered IR and contract
  structures must independently meet their accepted limits. No truncation is
  permitted. The test recognizer deliberately adds smaller limits of 4,096 tokens
  and 100,000 memoized states, reporting refusal; these are oracle limits, not
  proposed compiler capacity or performance evidence.

## Maps, types and expressions

Nonempty maps use brackets and comma-separated named entries. `empty` is the
explicit empty form where permitted, never `[]`. Reject duplicate resolved keys,
including aliases mapping to the same wire key. True lists retain order; maps
and sets have no execution priority. Names resolve exactly across declaration
kinds without ancestor shadowing, according to [ADR-0012](../bindings/README.md).
An optional qualifier lists the **entire owning scope chain**, nearest to root:
`"Decision" in "Review" in "Order"`. This proposed stricter source spelling makes
qualification deterministic without changing IR access rights. Child-template,
child-item and descendant-invalidation exceptions still need use-site validation;
qualification never opens arbitrary child/sibling data access.

The grammar covers the nine closed type descriptors. Record fields are
`["amount" as decimal with 2 places, "note" as text]`. Variant cases accept
literal names or explicit outcome references. Named types select a pinned
`named-type` resource plus exact definition name; an alias is not a nominal
type contract. `with parameters empty` is mandatory on ordinary expressions;
only a fan-out key expression may declare `["item" as T]` in this profile.

Every compound operand is parenthesized; no precedence or natural-language
scope is inferred. `Let` declares a named expression. Compute assignments and
data initialization also admit inline expressions; these get stable generated
expression IDs. Policy predicates embed their AST directly. The grammar's
individual operation productions map to the closed
[expression table](../dialects/values-expressions.md#e1-closed-expression-ast):
`read`, `param`, `field`, `record`, `list`, `variant`, `none`, `some`, `isSome`,
`unwrap`, `tag`, `payload`, `if`, `not`, `and`, `or`, `eq`, `lt`, `add`, `sub`,
`mul`, `length`, `contains`, `index`, `rescale`, `wrap`, `unwrapNamed`. `fact` is
policy-only. The AST's types, fault behavior and all-branch dependency rules
remain unchanged; grammatical acceptance does not type-check these operations.

Primitive spellings produce typed `literal` ASTs. `outcome "correction needed"
of "Review"` and `purpose "Expense review" of "Request"` also produce Text
literals with the resolved wire symbol, so authors can compare tags/policy facts
without writing technical wire spellings. Quoted text remains literal text,
even when it resembles an alias; it is never guessed or converted.
`literal of T with value V`
also represents an existing literal AST with a composite value without turning
it into a constructor AST. The separate `value` production admits records,
lists, tagged choices, `absent` and `present (V)` recursively. Under the declared
type these become exact closed JSON values; absent/present map to option tags,
and a named value uses its representation encoding with no extra wrapper.
Decimal value markers must match the declared decimal scale. This distinction
allows later IR export to preserve an original literal versus constructor AST.

## Action and policy lowering

The [paper comparison](syntax.md#3-candidate-a-structured-steps) remains the
historical candidate description. This grammar supplies its missing collections
and adds explicit parameters, conservative reads and outcome qualification.
No accepted IR field or executable contract is extended. Scope outcomes and data
purposes must be nonempty even though their shared collection production admits
`empty` in other contexts.

| Source field | IR obligation |
| --- | --- |
| Root/scope interface | `parent`, `entry`, all port maps, outcome set and four policy IDs; `within` agrees with nesting |
| `Keep` | Exact type and all five protection fields; invalidates set; optional generated initial expression |
| `Let` | Exact type/parameters, suite pin, AST and exactly inferred all-branch reads |
| Actor/capability | Requirement policy; provider wrapper, exact input/output types, authority and effect policy |
| `On` clauses | Node outcome set and one generated flow per outcome; no ordinary flows for split/fanout/finish |
| `Ask`/`Request` | Activity mode, target, input expression, result/null and work policy |
| `Set together` | Nonempty cell-to-generated-expression map, atomic writes and one ordinary outcome |
| `Run` | Direct child; input `uses` references expressions, output `uses` references caller cells; exact child port coverage |
| `Choose` | Outcome-to-expression guards plus default/null, with no first-match ordering |
| `Start together`/`Gather` | Branch symbol-to-child map, reciprocal paired join and explicit settlement |
| `Wait`/`While` | Wait policy; Boolean condition and direct child repeat, with accepted cardinality restrictions |
| `For each` | Collection/key/seal expressions, direct child, its named input port resolving to the item cell, membership rule and paired join |
| `Finish` | Containing scope's outcome symbol; node outcome set empty |
| `Also read` | Additional conservative node reads; union with required transitive reads, never replace or reduce them |

Scope ports use locally owned data references. The same `uses` syntax in `Run`
has context-specific kinds as specified above. Every declared ordinary outcome
must have exactly one successor; human result tags and provider observations
must match the resolved outcome tokens. Flow cycles, visibility, availability,
protection, ownership and effect checks are mandatory subsequent admission.

Join predicate outcome sets contain **qualified outcome references**, not bare
spelling guesses. Resolve each reference to its scope's declared outcome symbol
and lower its wire token into the IR predicate set. Referenced scopes must belong
to the paired split/fan-out population. Different symbols may share a token;
the predicate still tests that token across the entire population, not just the
named child. Consequently binding must reject a token shared by an unlisted
member outcome unless that outcome is explicitly listed too. Deduplicate the
resulting token set only after these checks. This prevents a child-qualified
sentence from disguising the IR's population-wide token predicate.

All twelve policies have mandatory, ordered clauses in the grammar. Their
mapping is exactly [P0–P8](../dialects/policies.md): permit predicates; provider
pins; class/idempotency/reconciliation; work instructions/authority/retry/timeout/
completion/compensation; fault handlers; wait type/match/authority/timer/outcomes;
cancel authority/children; settlement operation/owner; closure mode/follow-up;
race mode/invariant; membership removal/changed invalidation. Where a selected
mode has a null field, emit explicit null (retain/cancel owner, terminal follow-up,
acceptance-order invariant). Infer exact policy reads from embedded predicates,
timer basis cells and due expressions, plus transitive referenced policy reads
(including actor/capability references), respecting each policy's typed fact
context. Reject policy cycles and incomplete/surplus dependency declarations.

Timers spell clock, basis cell, due expression, calendar or `none`, and pause
authority. Delays spell exact nonnegative integer microseconds and must fit
signed-64. Attempts are 1..1000 and delay count is attempts minus one. Observation
and timer outcomes explicitly name their owning wait step; attaching the policy
to another step cannot silently remap its outcomes. Settlement transfer still
requires a live ancestor and atomic acceptance. Clock/calendar/provider pins
identify resources; recognition or a matching hash does not prove enforcement.
Unsupported resources and policies must refuse admission.

## Evidence and next implementation boundary

Run `.tools/ir-check/bin/python tools/check_authoring_profile.py` after the
existing hash-locked oracle dependencies are installed. The Python interpreter
is test infrastructure for the EBNF and companion schema, not a second semantic
implementation language. Rust remains the shared implementation language.

The oracle recognizes complete token sequences and checks companion structure,
identity invariants, exact resource bytes and joint hashes. It does not build a
typed source AST, compare source declarations with companion entries, rewrite
source references, lower to IR, enforce policies or authorize execution. A source
and ledger can each pass these checks while disagreeing. Full package admission
must compare every declaration/symbol/generated slot, resolve all references,
validate all accepted contracts and compute the existing IR semantic revision.
See [the evaluation](../evaluation/0015-authoring-grammar-bindings.md) for the
bounded observations and remaining work.

A subsequent [Rust frontend slice](../../crates/authoring-frontend/README.md)
now compares exact source/ledger inventories and lowers a supported subset to
revision-bearing **unvalidated candidates**. It does not supply full admission,
external-resource readers, confusable diagnostics, general IR export or a user
study. [Separate implementation evidence](../evaluation/0015-rust-authoring-frontend.md)
does not retroactively upgrade the original specification observations.
