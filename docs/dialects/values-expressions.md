<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Value and pure-expression contracts

**Status:** Accepted with ADR-0013 upon merge of PR #17; publication and common notation in [the suite](README.md).

## V1. Type descriptors and values

All descriptors are finite objects with exactly the listed fields. Their identity
includes the exact suite artifact binding. Field/tag names are `Name`; map order
does not affect equality. Types are invariant: ADR-0012 exact equality applies.

| Descriptor body | Admitted value |
| --- | --- |
| `{kind:"boolean"}` | JSON true or false |
| `{kind:"text"}` | JSON string; exact Unicode spelling, including empty string |
| `{kind:"integer"}` | Canonical decimal **string** encoding signed 64-bit integer |
| `{kind:"decimal",scale:s}` | Canonical signed-64 coefficient string; value is coefficient × 10^(-s), s a JSON integer 0..18 |
| `{kind:"record",fields:Map<Name,T>}` | JSON object with exactly those fields, recursively admitted values |
| `{kind:"variant",cases:Map<Name,T>}` | `{tag:Name,value:V}` with exactly one declared tag and admitted payload; descriptor cases nonempty |
| `{kind:"list",element:T}` | Finite JSON array of admitted T values; order significant |
| `{kind:"option",element:T}` | `{tag:"none"}` or `{tag:"some",value:V}`; no bare null |
| `{kind:"named",contract:Ref,name:Name}` | Value admitted by the supported immutable named-type declaration below |

Here T is another descriptor in the same suite; nested `named` may point to an
explicitly supported contract. Signed-64 canonical strings match `0` or
`-?[1-9][0-9]*` and lie in [-9223372036854775808,9223372036854775807]. No plus sign,
leading zeros, negative zero, whitespace, decimal point or exponent. A numeric
JSON token is not an integer/decimal value under this profile. Different scales
are different types, even if two coefficients express the same mathematical value.

Ordinary JSON numeric tokens remain usable for bounded schema metadata such as
scale; their transport still follows ADR-0009's safe-integer limit. Payload
values must not be parsed through binary floating point. Arithmetic is specified
mathematically and checked against the result type's coefficient range before
publication; overflow is a fault, not wrapping or saturation.

## V2. Named types and exact operations

A supported nominal-contract resource has exact fields
`{kind:"named-types",version:"0.1.0",id:Name,definitions:Map<Name,D>}` where each
D is `{representation:T,invariant:E}`. Its external Ref revision hashes every
stored UTF-8 byte, without reformatting; strict JSON admission precedes use.
`invariant` is a Boolean pure expression with only parameter `value` of the
representation type and no process-data reads or host facts. Its transitive type
and expression dependency graph is finite and acyclic. Its identity is the exact
resource ID, byte digest and definition name, not its underlying shape.
The same exact-byte resource must be supplied through the local registry; an
unrecognized implementation must refuse rather than ignore an invariant.

A named value uses the representation's value encoding; its static/runtime type
tag is retained separately. Admission checks representation and invariant; false
is `invalid-nominal-value`, an evaluation fault when constructed at runtime or a
static error for a literal. A type-checker never infers nominal membership merely
from matching bytes. A domain type like Amount can require nonnegative Decimal(2),
while its currency must be an explicit field/nominal distinction rather than an
engine default. This is not a claim of real financial/domain sufficiency.

Equality compares admitted values of **the same exact type**: Boolean/text as
exact values; integer/decimal coefficients; records by matching fields; lists by
ordered elements; variants by tag then payload; options by tag then optional
payload; named values by their representation, preserving nominal type equality.
There is no type-directed overload between different nominal types or revisions.

Ordering (`lt`) applies only to Integer or identical Decimal scale and compares
coefficients. Text ordering, locale collation and nominal ordering are not
implicit. Arithmetic `add`/`sub` requires identical Integer or Decimal types and
returns that type. `mul` requires two Integers, returns Integer. Decimal products,
division, rounding, arbitrary precision and host math functions are unsupported.
Exact scale conversion is explicit: multiply or divide the coefficient by the
appropriate power of ten; division must have zero remainder and result must fit.
No implicit rounding is available.

## E1. Closed expression AST

Each row gives the exact object keys. E means another AST; T a descriptor.

| AST body | Type / evaluation |
| --- | --- |
| `{op:"literal",type:T,value:V}` | Admit V under T before execution; return typed V |
| `{op:"read",cell:ID}` | Cell's declared type; return current immutable revision in the supplied snapshot, or unavailable fault |
| `{op:"param",name:Name}` | Declared parameter type and bound value; no unbound parameter |
| `{op:"field",value:E,name:Name}` | Named field of a record; reject absent field or non-record type statically |
| `{op:"record",fields:Map<Name,E>}` | Anonymous record with the inferred field types |
| `{op:"list",element:T,items:List<E>}` | Every item's type exactly T, including explicit T for empty list |
| `{op:"variant",type:T,tag:Name,value:E}` | T is variant, tag declared, payload exact type |
| `{op:"none",element:T}` | Option(T) absent value |
| `{op:"some",value:E}` | Option(type(E)) present value |
| `{op:"isSome",value:E}` | Option only, returns Boolean |
| `{op:"unwrap",value:E}` | Option(T) to T; absent faults, never supplies a default |
| `{op:"tag",value:E}` | Variant only, returns Text tag |
| `{op:"payload",value:E,tag:Name}` | Declared variant tag's T; runtime tag mismatch faults |
| `{op:"if",condition:E,then:E,else:E}` | Boolean condition, exactly equal branch types; evaluate only selected branch |
| `{op:"not",value:E}` | Boolean negation |
| `{op:"and" or "or",left:E,right:E}` | Boolean operands; left-to-right short-circuit |
| `{op:"eq",left:E,right:E}` | Exactly equal operand types; Boolean equality per V2 |
| `{op:"lt" or "add" or "sub" or "mul",left:E,right:E}` | V2 operand rules and overflow behavior |
| `{op:"length",value:E}` | List only; Integer canonical string count |
| `{op:"contains",list:E,value:E}` | List(T) and exactly T; Boolean using V2 equality |
| `{op:"index",value:E,index:E}` | List(T) and Integer index; T or bounds fault (negative indices fault) |
| `{op:"rescale",value:E,scale:s}` | Decimal to Decimal(s), exact V2 conversion or fault |
| `{op:"wrap",type:T,value:E}` | T named; operand exactly its representation, check invariant |
| `{op:"unwrapNamed",value:E}` | Named T to its declared representation; explicit nominal erasure only |

No function calls, effect requests, random values, current clock, filesystem,
network, environment, reflection, unknown operators, implicit casts or loops.
An integer-to-decimal conversion can explicitly `wrap` into an appropriate
domain representation only if types match; it is not a magical coercion.
Named unwrap/wrap never relaxes protection or authority.

## E2. Static dependencies and binding

Define R(E): `read(c)` contributes `{c}`; all other operations contribute the
union of child expressions, including both branches of `if` and the right side
of short-circuit operators. Literals contribute none after validating their
types/invariants; parameters contribute none. The expression record's `reads`
must equal R(E). Reject hidden dependencies and surplus expression reads; node
read sets may conservatively include additional cells under the suite rules.

Every expression is type-checked in full, including unreachable branches. Never
execute a branch to discover its type. Parameter references must resolve exactly
to the declared map; extra declared but unused parameters are permitted only if
the existing binding site supplies them. IR 0.1.0 only binds `item` for fan-out
key expressions; all other process-expression records have empty parameter maps.
The `value` invariant parameter and policy contexts are **contract-internal**
bindings, not a new IR parameter-call feature or ambient name lookup.

Evaluate pure expressions against one revision snapshot. A read accesses no
external system. Before evaluation, validate availability and read permission
for the full declared read set, even if a branch will short-circuit. Actual
branch evaluation can avoid arithmetic faults in unselected branches, but cannot
hide data dependencies from admission or protection checks. Record consumed
snapshot revisions; compute assignments use the same pre-transition snapshot
and publish only after every result/value/permission check succeeds.

If an evaluation budget is exhausted or a declared operation faults, publish no
partial writes or effects. A missing value is not Option absence. Literals in
definitions must not embed credentials or sensitive live data; diagnostics must
not expose protected values, including in errors from nominal invariants.

## E3. Policy-only facts

Policy predicates use the same AST plus `{op:"fact",name:Name}`. Only the fixed
context names/types listed for the policy kind exist. No dynamic path or host
dictionary lookup is allowed. Type-check all fact uses and reject unknown facts
or fact usage in process expressions/nominal invariants. Facts are immutable
typed host inputs tied to the accepted occurrence/revision context, not mutable
process reads; any process-derived fact must retain its declared dependency and
protection provenance. Predicates using process cells still contribute R(E).

Host authentication, freshness and correlation precede predicate evaluation;
the predicate cannot certify arbitrary caller-supplied claims. False denies the
requested operation. Predicate faults deny it and raise a policy-evaluation
fault for explicit handling; missing facts never become permissive defaults.
