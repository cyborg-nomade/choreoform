<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Evaluation: Binding and type foundations

**Status:** Plan frozen; results pending<br>
**Framework and corpus revision:** `39cebcc2401f12552a034c75a3cba3e075c8ce50`<br>
**Plan frozen:** 2026-09-15<br>
**Owner:** Proposal author; selection authority: Project Owner

## Frozen plan

Question: which binding, type-identity, scope, import, parameter, and composition
rules should constrain the next executable-contract deliverable? Produce a
paper specification and worked judgments, not a product validator or syntax.
The initial parser, IR 0.1.0, and frozen illustrative contracts remain unchanged.

Priorities: stable meaning under edits and dependency changes; explicit data and
authority boundaries; deterministic resolution and checkable type judgments;
composition across domains; then authoring convenience. No weighted score.
The user's Rust/Python experience gives no candidate a preference advantage.

Compare explicit lexical binding and closed, pinned reusable templates against
ambient/dynamic lookup and implicit structural compatibility. Consider a fully
nominal alternative for all types and a more permissive generic/subtyping model.
All alternatives have paper maturity. Only the recommended combination receives
worked cases; do not invent equivalent test or human-study scores.

Roles/tasks: domain reader identifies what a name refers to; author changes a
label, composes a child, or upgrades an import; implementer derives resolution,
type and visibility judgments; operator distinguishes unavailable data from an
optional value and follows revision provenance. No representative users or
independent evaluator are available. All human/accessibility claims remain
unverified. The same author proposes and evaluates; owner review is separate.

Benchmark pressures: RP-01 reimbursement (Level 1), RP-03 order fulfillment
(Level 3), RP-08 incident response (Level 4), from three domains at the revision
above. Supply worked interface/binding fragments, including rejection cases;
they are not complete executable processes. Retain the forty individual
scenario rows and gaps in ADR-0011's evaluation without upgrading their status.

Perturbations: duplicate/ambiguous names; wrong-kind references; shadowing;
same-shape different-domain types; omitted/extra/wrong-type arguments;
output aliasing; sibling access and ambient capture; import identity/digest
mismatch, missing transitive resources and cycles; label versus identity edit;
long-running dependency upgrade; late/duplicate observation, partial failure,
concurrent cancellation, changed evidence and large fan-out. Operational
perturbations receive boundary analysis, not fabricated execution traces.

Required artifacts: proposed ADR-0012, representation-neutral specification,
positive/negative worked cases with rule references, compatibility map to
existing IR and explicit future-wire obligations, gate review and twelve-axis
profile. Each deferred question names a follow-up deliverable. Prose only; no
new package, dependency, wire fields, registry access, runtime, or new parser.
Use repository content as primary evidence; no external technology comparison
is needed. Check Markdown links, scenario accountability, and consistency with
accepted ADRs; run existing regressions to ensure unchanged code still passes.

Evidence is at most B (worked) for proposed semantic judgments. Link/test success
does not make these judgments reproduced execution evidence. G1–G4 remain
conditional; any recommendation is a working foundation, not final language
selection or permission to execute. Owner approval is required before acceptance,
merge, and starting the executable-contract deliverable.
