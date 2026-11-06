<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Language conformance evidence

This is a growing executable catalogue, not certification of a complete process
validator, host, interpreter or authoring product. The current slice checks
V1/V2/E1 value-expression judgments from the accepted core contracts. See the
[remaining-validator audit](../evaluation/0028-validator-gap-audit.md).

The [69 JSON vectors](../../crates/semantic-check/tests/conformance/value-expressions.json)
contain hand-authored expected types/values and static/evaluation refusal codes.
They cover all 29 E1 operations at catalogue level: read and fact have explicit
invariant-context refusals; the other 27 occur in the private evaluator's supported
pure contexts. This is not operational process-read/policy-fact coverage.

The exact top-level fields are format `choreoform-value-conformance`, version
`0.1.0`, contract, base, namedResource and cases. Each of the first three pins
uses the existing `{id,revision}` shape and explicitly available bytes. The named
resource is the existing Nonnegative fixture; no new contract is published.
Each case has exactly id, rule, binding, expression and expect. Binding is null
or a `{type,value}` for the contract-internal parameter `value`, not a new IR
parameter-call feature. Expect is one of:

- `{phase:"value",type,value}`: full static checking succeeds and evaluation
  returns exactly this canonical value/type; the result is admitted again.
- `{phase:"static",code}`: the full expression is rejected before evaluation,
  including invalid unselected branches and forbidden invariant contexts.
- `{phase:"evaluation",type,code}`: full static checking succeeds with this type,
  then the selected operation faults with this code.

The [Rust harness](../../crates/semantic-check/src/expression/conformance.rs)
runs the production static checker and existing private nominal-invariant
evaluator. It introduces no public evaluator or host reads. A fresh bounded
registry is used for each vector. Three test groups execute 42 values, 16 static
refusals and 11 selected faults; a vector is not counted as a separate Rust test.
The [Python oracle](../../tools/check_value_conformance.py) independently checks
strict transport, exact pins, field/identity/phase catalogues, normative operator
accountability and licensing/links. It does not independently evaluate semantics.

```sh
export CARGO_HOME="$PWD/.tools/cargo"
export RUSTUP_HOME="$PWD/.tools/rustup"
export PATH="$CARGO_HOME/bin:$PATH"
cargo test -p choreoform-semantic-check --lib conformance --locked
.tools/ir-check/bin/python tools/check_value_conformance.py
```

Branch short-circuiting suppresses unselected arithmetic/absence faults; it does
not skip static type/dependency checks. Integer/decimal coefficients remain exact
signed-64 strings. Equality keeps record-field, list-order, Unicode and nominal
distinctions. Bounds, overflow, inexact rescaling, absent option, wrong variant
tag and false nominal invariants have explicit outcomes. Secret-marker negatives
check sanitized diagnostics rather than exposing a real credential.

Next expand process snapshot/permission/revision and policy/lifecycle judgments,
complete RP-01/RP-03/RP-08 packages and all forty scenario mappings. Add full
source/IR/text/visual and negative diagnostic evidence without replacing old
immutable artifacts or presenting synthetic inputs as actual host/user evidence.
