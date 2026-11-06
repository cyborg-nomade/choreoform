<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0040: Use finite guard snapshots without execution claims

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

The [original RP-01 audit](../evaluation/0040-rp01-scenario-audit.md) demonstrates
that ten passing partial static stages do not establish the intended approval,
revision or effect predicates. The production checker already types full
expressions and validates exact nominal values; its private pure evaluator is
used for invariants and value conformance. A process evaluator or authenticated
host is not available. Generated observations cannot substitute for either.

## Decision

Add a revision/digest-bound finite snapshot catalogue and test-only Rust harness
for actual candidate ordinary expressions and actor/work/race predicates. Select
the original AST by existing ID; derive lexical cell and accepted policy-fact
types, with work result type bound to its real activity site. Statically check the
complete original AST before binding, including unselected branches. Validate
explicit immutable values against production exact types and nominal invariants.

Inside tests only, replace read/fact leaves with distinct named parameters. Keep
literal data, type descriptors, field names and every other operation unchanged.
Check the rebound type/read set and use the existing production pure evaluator;
validate its returned value. Missing, unknown, unused, ill-typed or invalid nominal
bindings refuse. Charge index/traversal/AST/value copying against existing work/
depth bounds. Catalogue selectors, exact revisions/resources and sanitized errors
are checked. An independent Python oracle checks transport, closed fields,
identity/reference/count/expectation accountability and licensing; it does not
independently evaluate predicates.

No public evaluator/runtime API, ambient reads/IO, accepted observation, permission
or process transition is introduced. A Boolean result is only the selected
predicate's value for that synthetic binding. It does not supply mandatory P1
identity/correlation checks or prove that the snapshot is reachable, authentic or
currently authorized. Even a true actor predicate does not authenticate its input
facts. No snapshot count becomes a completed scenario or representative-user study.

The first catalogue run found that `payment_commits` omitted approval/current
policy-revision comparisons. Under the separately frozen
[precise correction](../evaluation/0040-rp01-policy-guard-fix-plan.md), `d0c13bf`,
the current fixture retains six old clauses and adds three equality clauses for
manager/finance evidence and current claim policy revision. Keep the negative
expectation and symmetric regressions. This is a predicate correction, not a
general irreversible-race proof. Old fixture profiles retain their original guard;
the current generator opts into the stronger guard explicitly and obtains a new
semantic revision/digests. Other body fields, IDs and resources are preserved.

## Alternatives and consequences

| Option | Benefit | Cost or limitation | Decision |
| --- | --- | --- | --- |
| Actual ASTs, explicit test parameters, existing pure judgments | Exercises real predicate meaning without introducing execution authority | Finite cases; does not check process transitions or host truth | Adopt |
| Another handwritten Boolean implementation | Easy testing | Can agree with expectations while production predicates differ | Reject |
| Public snapshot/runtime API now | Broader utility | Requires a separate authenticated/protected binding and admission design | Defer |
| Treat static type success as predicate evidence | No harness | Misses the reproduced policy revision gap | Reject |

The [frozen implementation plan](../evaluation/0040-rp01-guard-plan.md), `2826cce`,
and [evidence](../evaluation/0040-rp01-guard-snapshots.md) retain C/D fault-source,
late-observation/continuation, ordering/history, live ownership, irreversible and
all host/user/phase requirements. Acceptance is delegated automated review, not
individual owner or independent review. Require final-head CI, complete comment
audit and signed expected-head merge into draft #46. No phase gate closes.
