<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Implementation-language decision evidence

**Status:** Desk research complete; implementation evidence pending<br>
**Plan frozen:** 2026-09-04<br>
**Repository baseline:** `9ad094264a10a636ed90a866b061560102518959`<br>
**Evaluator:** Proposal author; Project Owner approval pending

## Question and method

Select the first implementation language for Choreoform's shared semantic core,
parser/validator, CLI, language-service backend, and initial reference interpreter.
Do not select a grammar, parser framework, Studio shell, persistence backend,
distributed runtime, extension ABI, or deployment platform in this decision.

Compare TypeScript, Rust, Go, Kotlin/JVM, and Python using official language and
tool documentation at the same desk-research maturity. No candidate has a
Choreoform implementation in this comparison. Existing Python IR tests validate
the specification fixtures, not Python's superiority as a product language.
Team familiarity and target-user deployment constraints are unknown unless the
owner supplies them. No numerical ranking, performance claims, or aggregate
score will be inferred from ecosystem descriptions.

This is implementation-technology research, not a competing semantic or notation
proposal under ADR-0007. It borrows that framework's evidence discipline without
claiming its twelve-criterion scorecard or five gates have been completed.
ADR-0008/0009 meanings, forty scenario statuses, and conditional gates remain
unchanged. Representative pressures are RP-01 revision-bound payment, RP-03
parallel obligations, and RP-08 dynamic scope/protection; no runnable scenario
support is established by choosing a host language.

## Common criteria, in priority order

1. Preserve explicit state variants, stable IDs, immutable revisions, and effect
   boundaries without host-language behavior becoming process semantics.
2. Make a small, testable Phase 1 language toolchain practical to build and
   maintain; account for unknown team experience rather than assume it.
3. Support recoverable parsing, diagnostics, and LSP integration independently
   of a particular editor, while leaving the grammar/framework choice open.
4. Keep one semantic implementation reusable across native tools and a possible
   browser Studio, without deciding that Studio must run locally in a browser.
5. Provide a credible path to a local reference interpreter, explicit I/O,
   controlled concurrency, and cross-platform distribution.
6. Preserve free-software development, reproducible builds, dependency review,
   and an implementation-independent IR/conformance boundary.

Hard exclusions: dependence on proprietary bundles or a hosted vendor service;
silently changing IR semantics to suit a host; treating host scheduling, type
checking, or memory safety as proof of process correctness. Performance,
startup time, build time, artifact size, usability, and hiring costs remain
unmeasured for every candidate. Any recommendation must identify what changed
priority or evidence would reverse it.

## Findings and source register

**Owner clarification, 2026-09-04:** after the plan was frozen, the Project Owner
specified that project requirements alone should determine the implementation
language; personal experience should not influence the choice. References to
unknown familiarity in the frozen plan above are therefore not applied as a
selection criterion. Ongoing contributor onboarding, maintenance burden and
toolchain complexity remain project costs, without assuming this owner's skills.

Sources below were inspected on 2026-09-04. These are living upstream documents,
not pinned dependency selections. Their documented capabilities support a
feasibility argument, not measured Choreoform outcomes. The plan above was
committed as `f3839a3` before recording this synthesis.

| Candidate | Documented capability | Project-specific inference and limitation |
| --- | --- | --- |
| Rust | Enums and exhaustive `match`; ownership checked by the compiler without requiring garbage collection ([match](https://doc.rust-lang.org/book/ch06-02-match.html), [ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)) | Good fit for explicit node/obligation variants and controlled mutation. Does not enforce Choreoform authority, safe retries, logical immutability, or determinism by itself. Graphs should retain ID references instead of borrowing directly across every edge. |
| TypeScript | Discriminated unions and `never` support exhaustive handling; type annotations are erased ([narrowing](https://www.typescriptlang.org/docs/handbook/2/narrowing.html), [basics](https://www.typescriptlang.org/docs/handbook/2/basic-types.html)) | Strong alternative for a shared editor/core prototype. Require strict checking, exhaustive handling, and runtime validation of all external data. Erasure is not unique evidence against TypeScript: every candidate needs wire validation. |
| Go | Structs, interfaces, type switches, goroutines, and target selection are documented ([specification](https://go.dev/ref/spec), [targets](https://go.dev/doc/install/source)) | Credible native tooling and service implementation. Closed state variants require project conventions or extra checking rather than Rust-style exhaustive enum matching. Goroutine scheduling cannot define process transition order. |
| Kotlin/JVM | Sealed hierarchies support exhaustive `when`; Java interoperability exposes JVM libraries ([sealed types](https://kotlinlang.org/docs/sealed-classes.html), [interop](https://kotlinlang.org/docs/java-interop.html)) | Strong typed semantic model and JVM tooling option. Attractive if JVM deployment becomes a requirement. A JVM-first dependency graph is not automatically reusable in a browser. |
| Python | Type annotations support external checking but are not enforced by the runtime ([typing](https://docs.python.org/3/library/typing.html)) | Useful for a readable reference model and experiments, with disciplined static checking and runtime validation. The existing fixture script is continuity evidence only, not comparative development-speed or engine evidence. |

### Parser and language-service feasibility

[LSP](https://microsoft.github.io/language-server-protocol/) separates a language
server from editor clients through a protocol. Using VS Code does not require
the semantic implementation to use TypeScript.

| Candidate | Concrete upstream evidence | What it does not establish |
| --- | --- | --- |
| TypeScript | [Langium](https://langium.org/) documents grammar-based language tooling, LSP, Node.js and browser use; [Microsoft's LSP modules](https://github.com/microsoft/vscode-languageserver-node) provide protocol/server tooling | Neither framework is selected. Generated syntax trees must not replace Choreoform's canonical IR; grammar recovery, trivia, stable IDs, and round trips need tests. |
| Rust | [Rowan](https://github.com/rust-analyzer/rowan) provides lossless syntax trees; [lsp-types](https://github.com/gluon-lang/lsp-types) documents Rust protocol types | Rowan is not a complete parser or language server. A maintained server implementation, supported protocol version, recovery strategy, and integration work still need evaluation. A repository's existence does not certify its current protocol coverage. |
| Go | [Participle](https://github.com/alecthomas/participle) supplies a parser library; LSP permits an independent server | A parser library does not establish incremental editing, a complete language server, or stable formatting. |
| Kotlin/JVM | [ANTLR](https://github.com/antlr/antlr4) supplies a Java target usable through Kotlin's Java interoperability | Not a Kotlin-specific grammar/framework selection; JVM parsing and an LSP transport still need integration and editor-error tests. |
| Python | [Lark](https://github.com/lark-parser/lark) supplies parsing and tree tooling; LSP permits an independent server | No Choreoform parser, incremental-analysis design, or language-service implementation has been evaluated. |

### Host and deployment boundaries

- **Rust:** the documented
  [`wasm32-unknown-unknown` target](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html)
  provides a possible shared-core route, but filesystem operations and thread
  spawning cannot be assumed to work. Keep host I/O outside the portable core.
  Browser bindings, responsiveness, target-compatible dependencies and parity
  are untested. Native release packaging must still cover selected OS/CPU targets.
- **TypeScript:** Langium demonstrates Node/browser language-tooling feasibility.
  A pure shared module can avoid Node-only APIs, but browser workers and host
  adapters still need design. Node's
  [event-loop guidance](https://nodejs.org/en/learn/asynchronous-work/dont-block-the-event-loop)
  explains why long synchronous analysis can block other work; it does not show
  that Choreoform would be too slow in TypeScript. Distribution needs a JS runtime
  or a separately evaluated bundled-runtime strategy.
- **Go:** native and WebAssembly targets exist; browser integration and host
  restrictions still need investigation. Native build support is not a claim
  that every dependency produces one self-contained, cross-compiled binary.
- **Kotlin/JVM:** JVM execution is the candidate here. Kotlin also has
  [Kotlin/Wasm](https://kotlinlang.org/docs/wasm-overview.html); it is not fair to
  claim Kotlin cannot target browsers. Choosing a portable Kotlin source set
  and compatible libraries would be additional work, not automatic Java-library
  reuse. Packaging a runtime is a deployment decision, not an intrinsic blocker.
- **Python:** [WebAssembly builds](https://docs.python.org/3/library/intro.html#webassembly-platforms)
  exist with host-library limitations. Browser use is not impossible, but
  requires a Python runtime and compatible dependencies. Runtime distribution
  and responsiveness remain unmeasured, just as for the other candidates.

### Shared pitfalls: no automatic conformance

Every candidate must independently satisfy ADR-0009's strict decoder and JCS
rules. Parsing JSON into a convenient map before checking duplicate keys or
numeric tokens can erase required evidence. Rust's
[generic JSON value representation](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html)
is not a substitute for a duplicate-aware, bounded transport stage. Python's
[JSON decoder](https://docs.python.org/3/library/json.html) exposes hooks used by
the existing fixture checker; that does not make its default settings the IR
contract. Do not rank languages by whether their default JSON parser happens
to accept a fixture.

Likewise, Rust's [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
uses implementation-dependent ordering; ordered internal maps are not, by
themselves, JCS either. Stable graph IDs are not memory addresses, and neither
iteration order nor async completion order may silently select a process race.
The semantic scheduler and wire encoder need explicit rules and tests in all
five languages.

## Synthesis and sensitivity

Recommend **Rust for the shared semantic implementation and native tools**.
This is an architectural judgment, not a benchmark winner. Its closed variants,
controlled mutation, and native/Wasm compilation path jointly fit the highest
priority requirement: one explicit semantic implementation that can outlive a
particular editor or service. Its costs are ownership-oriented design, more
manual language-tool assembly than a framework such as Langium, and a future
browser boundary. No measured productivity advantage is claimed.

**TypeScript is the strongest alternative**, especially if rapid browser/editor
iteration dominates. Its state modeling is
credible; selecting Rust is not a claim that TypeScript is unsafe or incapable.
Go becomes more attractive if native service deployment dominates; Kotlin/JVM
if JVM integration dominates; Python if an exploratory reference model dominates
over distributing a shared product core.
None is excluded by a demonstrated impossibility.

Personal familiarity is excluded by the owner's clarification. If an important
project deployment constraint emerges, update this synthesis before acceptance;
do not preserve the recommendation by silently changing weights.
If Rust's bounded confirmation work exposes unacceptable integration or
maintenance costs, return to the owner before building the rest of the parser.

## Required implementation evidence, not results

The first implementation PR after language approval must record exact compiler,
dependency, OS/CPU and execution commands, then establish these common checks.
Its owner is the implementation author; closure is reviewed by the Project Owner.

| Check | Observable pass condition | Boundary |
| --- | --- | --- |
| Strict IR transport | All three existing fixtures retain their exact revisions; duplicate keys, unsafe numbers, invalid Unicode, unsupported versions/bindings and altered snapshots are rejected | Compare to frozen expected values as well as the Python harness; neither implementation is automatically a full semantic oracle |
| State-model change | Adding a core node variant requires deliberate handling in relevant operations; unknown node kinds and wrong-kind references have explicit rejection paths | Static exhaustiveness alone does not establish runtime correctness; interpreter transitions remain later work |
| Portable core | Same admitted input produces identical canonical bytes and structural error categories in a native run and a real browser/Wasm run, including non-BMP text | Compilation alone does not pass parity; this is a boundary probe, not a Studio implementation |
| Host isolation | Wire/graph operations run without filesystem, network, wall-clock, random-ID, or thread creation in the portable semantic path | Native adapters may provide explicit observations; do not prohibit legitimate adapter I/O |
| Reproduction | Clean checkout builds and repeats tests with pinned stable Rust and locked dependencies; selected OS targets and dependency exceptions are recorded | Locking alone is not proof of bit-for-bit reproducible binaries |

Parser recovery, trivia/source spans, LSP cancellation/versioned documents, and
three full text/IR round trips remain acceptance evidence for their own later
deliverables, not claimed complete by the portable-core probe. Full effect,
recovery, concurrency, privacy, and performance evidence remains later work.

No candidate compiler, parser, Wasm program, language server, or interpreter was
installed or executed for this desk comparison. The existing Python fixture
suite is run only as a regression check for the documentation PR. These limits
prevent this recommendation from upgrading ADR-0009's conformance status.
