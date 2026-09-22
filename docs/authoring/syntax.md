<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate syntax and refusal rules

**Maturity:** paper grammar design for ADR-0014, not a supported source profile.
The templates below fix the proposed token boundaries and semantic distinctions.
They are not a complete parser grammar: policy/resource and binding-package
encodings still need an implemented grammar and conformance tests. No example
with omitted declarations or an unresolved requirement can lower successfully.

## 1. Shared lexical and binding rules

- A sentence ends in `.` outside a quoted string. A block has an explicit matching
  ending, such as `End step.`, `End rule.` or `End process.`. Indentation is presentation, not
  control flow. Space, tab, CR and LF separate tokens; wrapped lines are allowed.
- Fixed words have the exact spelling/capitalization shown. No synonyms,
  stemming, inferred articles, pronoun resolution or fuzzy execution. Completion
  may suggest a correction but must not apply it as accepted meaning.
- Every user binding is a double-quoted, nonempty Unicode string. A quoted
  string escapes only quote and backslash as `\"` and `\\`; newline/control
  characters are forbidden inside it. Literal text uses the same string syntax
  and may be empty. Unicode is preserved without normalization or case folding.
  Names differing only by confusable characters get a diagnostic showing both
  spellings; never merge them. Unicode-security handling still needs tests.
- `"Decision" in "Review" in "Order"` means one binding qualified through
  explicit scope names, nearest to outermost. `"Decision in Review"` is one
  name. Quoted punctuation has no qualification meaning. All declarations are
  collected before resolution; no ancestor shadowing, wrong-kind fallback or
  crossing a scope interface (ADR-0012 N1–N3).
- Comment lines start with `#` outside strings and end at LF/EOF. Comments and
  display titles cannot be the sole copy of instructions, authority or policy.
  Blank lines have no meaning. All input must be consumed.
- `none` in a clause requiring an optional reference means explicit null;
  `nothing` in a result clause means no result cell. Empty lists/sets are written
  explicitly as `empty`. Missing required clauses are errors, not these values.
- Lowering must set resource limits before parsing. Initial proposed limits are
  1 MiB UTF-8 source, nesting depth 64 and the accepted contract's AST/collection
  bounds. Also check total package and lowered IR limits. Refusal cannot truncate.

Quoted names are a deliberate initial cost. Unquoted multiword names might read
more naturally, but require reserved-word boundaries and ambiguity experiments
before replacing this rule. Arbitrary user text is data, never executable code.

## 2. Shared declarations

Both candidates use these declaration roles. `N` denotes a quoted name/ref,
`E` a pure expression, `T` a type, and `L` an explicit list. Metavariables are not
literal source. Bracketed descriptions below are grammar explanations, not
accepted free-form statements.

| Template | Role and required content |
| --- | --- |
| `Process N. ... End process.` | Root scope, explicit entry, input/output ports, outcomes and four scope rules |
| `Scope N within N. ... End scope.` | Direct lexical child, same mandatory scope interface |
| `Start at N.` | Scope entry, never inferred from first step |
| `Inputs ... . Outputs ... . Outcomes ... .` | Exact named port maps to typed local data; explicit `empty` where allowed |
| `Use cancellation N, faults N, closure N and races N.` | All four scope policy references; none may be omitted |
| `Keep N as T under N. ... End data.` | Data plus full protection envelope, invalidation targets and optional initializer |
| `Let N mean E as T.` | Named expression; result and all-branch dependencies checked, not a mutable assignment |
| `Require actor N under N.` | Actor requirement linked to an actor policy, not a named live account |
| `Use capability N from N with input T and output T, authority N and effects N.` | Exact provider binding and explicit authority/effect rules |
| `Rule for [policy kind] N. ... End policy.` | One closed policy variant; clauses from section 5 |

Types, policy/resource declarations and ports are part of the proposed full
surface, not implemented module imports. `from N` selects an already pinned local
provider resource; it does not fetch a URI. New reusable-module syntax is refused
until ADR-0012's module and linked-artifact formats exist.

The exact line grammar for maps, ports and generated binding blocks is still an
implementation prerequisite, not permission to insert raw JSON into source.
The [mapping](mapping.md) records the consequences of that open boundary.

## 3. Candidate A: structured steps

Each `Step N. ... End step.` has exactly one action below, followed by one
`On N, continue with N.` clause per ordinary outcome. Outcomes on a split,
fan-out or finish are forbidden. Steps may be reordered without changing the
graph. Sequential behavior comes from named outcome clauses, not prose order.

| Core kind | Action template | Constraints retained |
| --- | --- | --- |
| activity | `Ask N to decide using N under N and record in N.` | Human target; expression input; work rule; typed result cell or `nothing` |
| activity | `Request N using N under N and record in N.` | Capability target; same explicit fields; request is not confirmation |
| compute | `Set together ... .` | Nonempty cell/expression assignment map; all use one pre-transition snapshot |
| invoke | `Run N with inputs ... and outputs ... .` | Direct child; exact port coverage; no ambient parameters |
| decision | `Choose by ... ; otherwise N.` | Named Boolean guard map; `otherwise none` permits a no-match fault; multiple matches fault |
| split | `Start together ... and gather at N.` | Stable branch-name/child-scope map and reciprocal join; no ordinary successor |
| join | `Gather from N when ... ; for unfinished work use N.` | Closed join predicate and explicit settlement; sole ordinary outcome |
| wait | `Wait under N.` | Wait policy fixes correlation, authority, timer and exact outcomes |
| repeat | `While N is true, repeat N.` | Named Boolean condition, tested before each iteration; single-outcome child |
| fanout | `For each item of N, using key N, run N with item N; seal when N is true; for changes use N; gather at N.` | Collection/key/body/item/seal/membership/join all explicit |
| finish | `Finish as N.` | A declared scope outcome, with closure checks still required |

All maps reject duplicate keys. List order is semantic only for actual lists;
branch keys/guard names/sets are unordered. A decision does not mean “first match”.
General backward flows are invalid; the correction loop must use `repeat`.
Fan-out has only its item port; a split/repeat body has no ports. Reusable process
calls do not become available merely by spelling “run”.

Join words are exactly `all report L`, `any report L`, `at least K report L`,
`both (J) and (J)`, or `either (J) or (J)`, recursively; K is positive.
They map to all/any/atLeast/and/or. No “none remaining”, negation, exact count,
timer, global query or arbitrary expression is admitted. The population is the
paired split/fan-out, not every child in the process. Fan-out joins wait for seal
even for `any`; empty-population `all` is true but still needs settlement/closure.

## 4. Candidate B: guarded rules

B replaces each step block by `Rule N. ... End rule.`. It uses the same action
templates with a lowercased initial word after `When entered, then ... .`.
Outcome clauses are `When the result is N, then start N.`. All shared data,
policy, scope and type declarations are identical to A. For example:

```text
Rule "Wait for payment evidence".
  When entered, then wait under "Payment reconciliation".
  When the result is "confirmed", then start "Record paid".
End rule.
```

This is a **local control rule**, not a global reactive rule engine. “Entered”
means activation of that exact node occurrence by its scope entry or incoming
flow; an unrelated event cannot fire it. “Result” is its accepted ordinary
outcome, not a provider packet or work-lifecycle fault. Duplicate result clauses
are invalid. Rule order conveys neither priority nor event acceptance order.
Unknown effects do not emit a business result just to make a rule fire.

This restriction makes B lower to exactly the same graph as A. It also exposes a
usability risk: readers may expect ordinary English “when” to mean an enduring
global condition. The study tests that expectation explicitly. Broader ECA
semantics are an alternative semantic design, not a syntax shortcut in this PR.

## 5. Mandatory policy vocabulary, shared by A and B

Policy sentences must expose the **entire** accepted payload. These clause labels
name fields, not new operations or defaults. The full encoded grammar remains
open; this catalogue is a completeness requirement for its implementation.

| Kind | Required author-facing clauses (besides name and lexical scope) |
| --- | --- |
| actor | `Permit when E.`; trusted current principal/role/authority facts |
| protection | `Permit access when E.`; data also states sensitivity, purposes, participant requirements and capabilities |
| capability | `Provider contract N.`; exact resource pin, typed input/output, outcomes and provider result protection |
| effect | `Effect is ... . Idempotency is ... . Reconcile under N.`; exact class and provider support |
| work | `Instructions Q. Authority N. Attempts K. Delays L. Retry faults L. Timeout ... . Complete when E. Compensates N/none.` |
| faults | Exact fault-name/handler-scope map, or explicit `empty`; no wildcard catch |
| wait | Observation type, match E, authority, timer/none, observation outcome and timer outcome/none |
| cancel | Cancellation authority and unfinished-child settlement rule |
| settlement | Retain, request cancellation, or transfer to an explicitly named live ancestor with acceptance |
| closure | Fully terminal with no follow-up, or ordinary closure with a named surviving reconciliation child |
| race | Acceptance order with no invariant, or guarded commits with E; consequential paths require protective evidence |
| membership | Removal settlement rule and changed-item invalidation, never delete old evidence |

Timers state clock pin, calculation-basis cell, due expression, calendar pin/none
and pause authority. Durations use exact integer microseconds initially; a
friendlier unit literal may be added only with exact checked conversion tests.
“Tomorrow”, “soon”, “three business days” and implicit local time are unsupported
until calendar construction and corresponding surface rules are specified.

An instruction string is semantic text in the work policy. It describes required
human conduct; it is not a hidden interpreter for new control/effect semantics.
Policy review must expose retries as attempt limits plus exact delays/faults;
`retry safely` or `reimburse automatically` alone is not a valid policy.
Facts available inside each predicate are exactly ADR-0013's closed typed context,
not arbitrary English terms. Policy-only facts cannot occur in ordinary expressions.

## 6. Values and expressions, shared by A and B

| Accepted form | Proposed written form / restriction |
| --- | --- |
| Boolean, Text, Integer | `truth value`, `text`, `whole number`; literals `true`, `false`, quoted text or canonical signed integer digits |
| Decimal(s) | `decimal with s places`; a literal spells exactly s fractional digits, no exponent/grouping, signed-64 coefficient checked without float parsing |
| Record, Variant | `record with ...`, `choice with ...`; exact fields/cases, no extra fields; constructors explicitly name their fields/tag/type |
| List, Option | `list of T`, `optional T`; ordered elements; absent is `no value of T`, not unavailable data |
| Named | `type N from N`; exact named-type artifact; an alias does not manufacture a nominal contract |
| literal / read / param / fact | Typed literal / `the value of N` / `the parameter N` / `the fact N`; fact context and param binding sites remain restricted |
| field / record / list | `field Q of (E)` / explicit field-expression map / typed ordered item list |
| variant / none / some | `case Q of T containing (E)` / `no value of T` / `some (E)` |
| isSome / unwrap | `a value exists in (E)` / `the present value of (E)`; absent unwrap faults |
| tag / payload | `the case of (E)` / `the payload of (E) for Q`; wrong tag faults |
| if | `if (E) then (E) otherwise (E)`; equal branch types, both branches checked |
| not / and / or | `not (E)` / `(E) and (E)` / `(E) or (E)`; explicit parentheses, left-to-right short circuit |
| eq / lt | `(E) equals (E)` / `(E) is less than (E)`; exact same type; no text/nominal ordering |
| add / sub / mul | `(E) plus (E)` / `(E) minus (E)` / `(E) times (E)`; multiply Integers only |
| length / contains / index | `the length of (E)` / `(E) contains (E)` / `item (E) at index (E)`; indices are zero-based, negative/out-of-range faults |
| rescale | `(E) expressed with s decimal places`; Decimal only, exact conversion or fault |
| wrap / unwrapNamed | `(E) as named T` / `the representation of (E)`; exact representation and invariant, no Integer-to-Decimal conversion |

For scale zero, a decimal literal needs its explicit Decimal type to distinguish
it from Integer. Negative zero is rejected. Fractional zeros are retained to
identify scale; `1.20` and `1.2` are different types. `s` is 0..18. There is no
money/currency/calendar inference. Literal, map/list constructor and reference
grammar tests remain outstanding; these written forms are not parser results.

Parentheses delimit operands of compound expressions; no precedence guessing or
natural-language quantifier scope. A readable formatter may wrap lines but cannot
rewrite expressions algebraically. Static reads include unselected branches;
the author never supplies an incomplete dependency list as an optimization.

## 7. Required diagnostics, not implemented messages

| Input/problem | Required refusal or distinction |
| --- | --- |
| `Pay it when ready.` | Unsupported sentence; no pronoun/intent inference; offer specific templates |
| Two visible declarations named `"Review"` | Duplicate/shadowing error with both locations, not “nearest plausible” binding |
| `"Decision in Review"` used for a qualified name | Missing exact name; explain explicit `in` qualification |
| Missing `On` outcome; multiple true decision guards | Missing successor at validation; ambiguous decision fault, respectively |
| `Gather ... any ...` before fan-out seal | Join remains unavailable; cannot choose a partial population |
| `Retry after a timeout.` without required policy fields | Missing retry/clock/authority/reconciliation detail; no dispatch |
| Raw rename leaving generated identity data stale | Binding mismatch; require explicit rename/rebinding, no ID regeneration |
| Unknown policy or resource | Inert view only; unsupported-contract refusal for lowering/admission |
| Required protected detail hidden from a reviewer | Incomplete-view status; no whole-definition edit/approval claim |

Stable diagnostic codes are a later validator deliverable. Future diagnostics
must carry exact UTF-8 byte spans, expected clause/kind, related declarations and
actionable text without exposing protected values. Recovering enough structure
for editing must never return a repaired definition as valid.
