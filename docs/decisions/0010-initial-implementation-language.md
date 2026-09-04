<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0010: Use Rust for the initial shared semantic implementation

**Status:** Accepted<br>
**Date:** 2026-09-04<br>
**Decider:** Project Owner<br>
**Approval:** Project Owner approved the ADR and selected Rust on 2026-09-04;
effective upon merge of [PR #12](https://github.com/cyborg-nomade/choreoform/pull/12).

## Context

[ADR-0008](0008-core-process-semantics.md) defines an explicit transition-based
semantic foundation. [ADR-0009](0009-canonical-versioned-ir.md) accepts a
structural, versioned IR while keeping executable dialects and conformance work
open. The [Roadmap](../../ROADMAP.md) now requires an implementation-language
decision before parser implementation. No product language has been selected;
the Python fixture checker remains a disposable specification-evidence tool.

The implementation must model many explicit state variants, preserve identities
and revisions across graphs, reject invalid input, support language services,
and later implement a local reference interpreter. A Studio may need to reuse
the same semantic code, but its web/desktop architecture is undecided. Choosing
the language of a future UI should not determine the process semantics.

Known constraints are the Manifest, accepted ADRs, free-software boundaries,
and phased delivery. Minimum hardware, offline-browser needs, enterprise runtime
requirements, and performance budgets are not established. On 2026-09-04 the
Project Owner explicitly directed that project requirements, not personal
experience, determine this choice. Personal familiarity is excluded; general
maintenance and contribution costs still matter. This proposal compares feasible
implementation directions, not measured prototypes.

## Decision criteria

The [evidence record](../evaluation/0010-implementation-language.md) freezes the
common criteria, source evidence, limits, and reversal conditions.

1. Represent explicit semantic variants, identities, and effect boundaries.
2. Keep the initial language toolchain small, testable, and maintainable.
3. Support parsing, diagnostics, and editor-independent language services.
4. Preserve a shared-core route across native tools and a possible browser host.
5. Support a local reference interpreter and cross-platform distribution.
6. Keep builds and dependencies inspectable without proprietary infrastructure.

No language's type system, memory model, garbage collector, or concurrency
runtime substitutes for Choreoform's operational semantics or conformance tests.

## Decision

Use **Rust, using a pinned stable toolchain and Cargo**, for the first shared
semantic implementation: IR loading/linking/validation, parser and normalization
logic, native CLI, language-service backend, and the initial reference interpreter.
This selects an implementation direction, not grammar, expression semantics,
an async runtime, a parser generator, or a production deployment architecture.

Maintain one semantic implementation behind explicit host boundaries:

| Component | Responsibility and boundary |
| --- | --- |
| Portable Rust core | Typed IDs and graph records, strict transport/admission, pure analysis and transformations; no hidden host I/O or scheduling semantics |
| Native Rust adapters/tools | CLI and LSP transport; later capability, time, persistence and execution adapters supply explicit inputs and consume explicit effects |
| Possible browser adapter | Invoke the same core through a versioned data boundary, with native/browser parity tests; do not independently reimplement validation in UI code |
| Studio presentation | Language/framework/shell remain a Phase 3 decision; TypeScript is a plausible client, not selected here |
| Existing Python evidence harness | Retain as an independent wire-fixture regression tool; do not turn it into a second product semantic implementation |

This is not a mandate that every future adapter, SDK, generated target, or service
use Rust. The published IR and future conformance suite remain the authority for
independent implementations. Rust structures and object layout must not become
the interchange format, extension ABI, or persistent checkpoint schema.

### Implementation guardrails

- Use enums and exhaustive handling for closed variants, with distinct ID types
  for definition nodes, data, and runtime occurrences. Prefer owned ID-indexed
  records over references tying graph edges to memory lifetimes. Validate scope
  and reference kinds at runtime; static types cannot validate untrusted JSON.
- Keep the project-owned portable core in safe Rust; require explicit review of
  dependency unsafe code, native bindings, build scripts, and host access.
  A safe public API is not a sandbox or a correctness proof.
- Do not derive race resolution from map order, threads, async wakeups, or wall
  clocks. Model observations and effects explicitly. Persistent history and
  external-effect guarantees remain Phase 2 work.
- Implement the strict IR transport stage before information-losing map/value
  conversion. Preserve duplicate-key, Unicode, safe-integer-token, exact-version,
  immutable-contract and JCS requirements. Do not treat ordinary JSON serializer
  output or an ordered map as canonical bytes.
- Keep syntax trees separate from canonical IR. Parser libraries may help with
  lossless text and recovery but cannot dictate semantic IDs or definition meaning.
- Pin an exact stable compiler and commit the Cargo lockfile in the first
  implementation PR. Review dependency licenses and sources under existing
  policy. A lockfile pins resolution; reproducible artifact claims need more
  evidence. No toolchain version or dependency is installed by this ADR PR.
- Test the native/browser boundary early as bounded confirmation. No assumption
  that native filesystem/threading libraries work in browser Wasm is permitted.
  Failure triggers owner review before broad parser implementation, not an
  automatic language switch or a second semantic engine.

## Options considered

All options are desk-researched and technically plausible. Assessments below
are project judgments supported by the linked evidence, not speed or cost scores.

| Option | Advantages for Choreoform | Costs and risks | Recommendation |
| --- | --- | --- | --- |
| **Rust shared core** | Closed variants and exhaustive matching; ownership makes mutation boundaries explicit; native and Wasm targets can reuse core code | Ownership-oriented graph design and contributor learning; parser/LSP components need assembly; browser bindings and dependency compatibility need proof | Adopt for the initial core and native tools |
| **TypeScript core with Node.js tooling** | Discriminated unions; direct JS/browser reuse; concrete integrated language-tooling and LSP options | Static types are erased; strict checking and runtime admission discipline are essential; host APIs and long synchronous analysis require care; distribution includes a JS runtime strategy | Strongest alternative if editor-first delivery dominates |
| **Go core** | Native tooling and explicit service/adaptor code; structs/interfaces, concurrency facilities, parser libraries and Wasm targets | Closed variants/exhaustiveness need conventions or extra checks; browser integration and rich language tooling still need assembly | Prefer if native service operations are the dominant constraint |
| **Kotlin/JVM core** | Sealed state hierarchies and Java tooling interoperability; credible JVM interpreter and language-service route | JVM-oriented dependencies do not directly carry into a browser; runtime packaging and a possible multiplatform split add decisions | Prefer if JVM deployment is required |
| **Python core** | Concise reference-model style; parser tools and continuity with fixture tooling | Static checking needs an explicit tool/policy; runtime and browser packaging need care; existing fixture success is not product scalability evidence | Retain for specification experiments; reconsider if exploratory delivery dominates |

Kotlin and Python also have WebAssembly routes; Go can target WebAssembly.
Browser feasibility is not an exclusive Rust advantage. The judgment favors
Rust's combination of state modeling and a portable native core, not a claim
that competing languages cannot implement the same semantics.

### Why Rust rather than TypeScript first?

TypeScript is the closest alternative: its discriminated unions can represent
the model, and Langium demonstrates a direct language-tooling path. A Node/browser
shared module could also avoid duplicate semantics. If an editor demonstration
were the primary deliverable, choosing it would be reasonable on project
requirements alone.

The current Manifest and Roadmap prioritize a reusable semantic foundation and
local reference execution before the Studio. I therefore favor Rust's explicit
closed variants and ownership boundaries for code expected to serve both native
tools and a later host-independent engine. This accepts extra implementation
and integration work now; it does not assume that Rust makes development faster,
eliminates bugs, or guarantees portable execution without testing.

## Consequences

- One core can be reused by the CLI, language services, and later interpreter;
  a future browser integration must reuse or conform to it, not fork semantics.
- Rust knowledge and tooling become contribution costs. Evaluate onboarding and
  maintenance as project concerns, not as assumptions about the owner's skills.
- A browser Studio would likely introduce another language at its presentation
  boundary. Avoiding duplicate semantics matters more than claiming a single
  language for every product component.
- Graph edits, snapshots, source spans, cancellation and host interaction still
  need explicit design. Memory safety does not prove authorization, correct
  compensation, durable history, or privacy.
- Initial deployment can focus on native tools, but supported OS/CPU targets,
  executable packaging, and dependency maintenance remain follow-up decisions.
- A future replacement is possible through the language-independent IR, but
  rewriting an implementation is costly. Revisit early if the confirmation
  probe or owner constraints change the balance.

## Confirmation

Approval selects the direction, not an already tested implementation. Before
broad parser work, the first implementation PR must provide the bounded evidence
listed in the [comparison](../evaluation/0010-implementation-language.md):

1. Decode the existing three fixtures and reproduce their frozen canonical
   revisions; reject malformed transport and unsupported/corrupt bindings.
2. Demonstrate explicit variant handling and typed reference boundaries, without
   mistaking static exhaustiveness for full semantic validation.
3. Execute the same wire/structural checks natively and through a real browser
   Wasm host; retain identical bytes/error categories and non-BMP Unicode cases.
4. Demonstrate core operations without ambient filesystem, network, time, random
   ID generation, or thread creation; supply host behavior through adapters.
5. Record the exact toolchain, dependency lock, commands, target matrix and
   remaining failures so another contributor can repeat the result.

This probe is not the parser, Studio, production validator, or engine. Parser
recovery, source/trivia preservation, LSP behavior, full processes, and execution
remain separately reviewed deliverables. Nothing here closes G1–G4 from ADR-0009.

### Confirmation result

The Project Owner approved the [bounded evidence](../evaluation/0010-rust-portability.md)
on 2026-09-04, effective upon merge of
[PR #13](https://github.com/cyborg-nomade/choreoform/pull/13). The probe reproduces
the three frozen revisions, passes 89 shared native/browser cases and six
JavaScript/Wasm byte-boundary checks, and records typed reference/variant and
host-boundary evidence. This satisfies the bounded confirmation requirement,
not complete structural/semantic validation, universal target support or
ADR-0009's conditional conformance gates. Subsequent Roadmap work remains a
separate deliverable.

## Review decisions

The Project Owner reviewed and approved the ADR and PR on 2026-09-04, explicitly
selecting Rust. The approved decision is:

1. Adopt Rust on project requirements, accepting the ownership and integration
   costs for the shared-core direction. Personal familiarity remains excluded.
2. Retain the core/native tooling/initial interpreter scope. Studio presentation,
   adapter implementations, parser framework, runtime infrastructure and specific
   deployment targets remain separate decisions.
3. Retain the bounded native/browser confirmation before broad parser work;
   browser evidence is not deferred. It must be produced in a separately reviewed
   implementation PR. Approval is not a claim that this probe has passed.

Minimum hardware and performance budgets remain unestablished. The approval
does not invent these constraints or close any existing conformance gate.

## Sources

Primary sources inspected on 2026-09-04; the evidence record links each factual
comparison to its source. In particular:

- [Rust exhaustive matching](https://doc.rust-lang.org/book/ch06-02-match.html)
  and [ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).
- [Rust's minimal WebAssembly target and limitations](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html).
- [Cargo manifests and lockfiles](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html).
- [TypeScript narrowing and discriminated unions](https://www.typescriptlang.org/docs/handbook/2/narrowing.html)
  and [Langium's language-tooling approach](https://langium.org/).
- [Language Server Protocol](https://microsoft.github.io/language-server-protocol/).
- [Full comparison and source register](../evaluation/0010-implementation-language.md).

## Acceptance and action items

The Project Owner approved this decision on 2026-09-04; merge of PR #12 makes it
effective.
No product tooling, parser, or runtime implementation was added by the
language-decision PR #12; the bounded probe is delivered separately in PR #13.

1. [x] Confirm the requirements-led Rust choice and resolve review questions;
   detailed deployment constraints remain future work.
2. [x] Obtain approval, record Accepted status and update the ADR index/Roadmap.
3. [x] Complete the bounded confirmation in the separately reviewed
   [PR #13](https://github.com/cyborg-nomade/choreoform/pull/13), approved on
   2026-09-04 and effective upon merge.
4. [ ] Record any failed assumption and obtain owner direction before expanding work.
5. [ ] Select grammar/parser approach and implement the next deliverable only
   after the language decision and required confirmation are settled.
