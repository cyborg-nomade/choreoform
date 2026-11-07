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

The [RP-01 guard snapshot catalogue](../../crates/semantic-check/tests/conformance/rp01-guard-snapshots.json)
adds 44 finite cases against actual source-derived expression and actor/work/race
ASTs under [ADR-0040](../decisions/0040-finite-guard-snapshot-conformance.md).
It has 41 Boolean results (12 true, 29 false), one selected absence fault and two
binding refusals. Six additional Rust groups cover the catalogue and harness
boundaries; a separate four-group Python oracle checks accountability, not semantic
evaluation. These are different counts from the original 69 vectors and forty
corpus scenarios; all scenarios remain Partial.

The test-only harness checks the original AST and exact bound types/nominal values,
rebinds read/fact leaves to explicit immutable parameters, checks the rebound AST
and uses the existing pure evaluator. Literal values/type descriptors are opaque.
The closed format has `format`, `version`, `candidateRevision`, `candidateDigest`,
`values` and `cases`. Cases contain `id`, `scenario`, `subject`, `cells`, `facts`,
`expect`. Subjects select map/ID/field/site; cell/fact maps reference named explicit
values. Expected outcomes distinguish Boolean value, binding refusal and selected
evaluation fault. Header pins must match current canonical candidate bytes.
This is a test fixture format, not ordinary authoring or a runtime observation API.

```sh
cargo test -p choreoform-semantic-check --lib rp01_snapshots --locked
.tools/ir-check/bin/python tools/check_rp01_guard_snapshots.py
```

Finite predicates do not prove snapshot reachability, assignment/authentication,
policy permission, mandatory acceptance prechecks, process transitions, current
host facts or irreversible safety. See the [evidence and remaining limits](../evaluation/0040-rp01-guard-snapshots.md).

The [terminal-observation metadata vectors](../../crates/semantic-check/tests/conformance/terminal-observation-sources.json)
add twelve finite provider-state classifications: seven normalized terminal-map/type
cases and five invalid state/outcome cases. Rust validates actual provider/source
bytes; an independent Python oracle checks declared expectations. These are metadata
classifications, not observations, accepted facts, wait bindings or executed scenarios;
see [ADR-0044](../decisions/0044-provider-terminal-observation-metadata.md).

The [wait-capture declaration vectors](../../crates/semantic-check/tests/conformance/wait-capture.json)
add one exact raw-IR-0.2/core-0.5 definition and seventeen finite version/binding/
initialization/invalidation cases. Rust checks the actual SDK judgments; an
independent limited Python schema/type/flow model checks declared expectations.
These static sets are not runtime values or generated observations. Five structural
and fifteen semantic integration groups additionally cover version/pin/role/type/
protection/visibility/timer/nominal/budget boundaries and honest stored-recovery
refusal; see [ADR-0045](../decisions/0045-explicit-timerless-wait-capture.md).

[Source-capture integration fixtures](../../crates/authoring-frontend/tests/capture/ordinary.choreo)
exercise readable IR-0.2/core-0.5 capture through the frontend. Nine Rust source /
one native output-bound group and six Python groups cover exact versions/resources,
normal visibility, derived reads, all declaration disclosures and honest stored-
recovery lifetime refusal. These fixtures and promoted copies supply no actual
observations, host/user execution or scenario completion; see
[ADR-0046](../decisions/0046-wait-capture-authoring.md).

[ADR-0047](../decisions/0047-owned-local-followups.md) adds nine Rust conditional
local-follow-up integration groups, two cumulative budget groups and five Python
groups with six independent finite isolation/availability/protection/invalidation
vectors. Retained creator/owner label provenance is separate from inherited values
or actual receipts. Native disclosures remain conditional, bounded and distinct
from ordinary/data-free evidence; no observations, host execution or scenario exit
are generated. See [evidence](../evaluation/0047-owned-local-followups.md).
