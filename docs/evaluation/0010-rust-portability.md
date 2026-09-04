<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Rust native/browser confirmation

**Status:** Bounded evidence passes locally; Project Owner review required<br>
**Baseline:** `7f2091b27484723506ce76b63ac4c105dfb1f86d`<br>
**Plan recorded:** 2026-09-04, before implementation

## Bounded plan

Implement ADR-0010's confirmation, not a language parser, interpreter, Studio,
or production validator. Use one safe Rust core for strict JSON transport,
canonical semantic bytes and revisions, typed node/reference modeling, and
selected structural checks. Native and browser adapters supply bytes and pinned
contract resources explicitly; no ambient host I/O belongs in the core.

Tests will include all three unchanged IR fixtures and fixed expected hashes;
duplicate keys, fractional/exponent numbers, unsafe integers, invalid UTF-8 and
surrogates, depth/size limits, unsupported versions/contracts, corrupt resources,
wrong-kind/dangling references, and unknown node kinds. Include non-BMP key order
and escaped strings, map/annotation changes, ordered arrays, and retained policy
fields. Test variant exhaustiveness and distinct ID types at compile time.

Run the same cases natively and in a real browser using the compiled Wasm core;
compare canonical bytes and error categories, not just counts. Audit Wasm imports
and core source/dependencies for host access. Record toolchain, locks, licenses,
platform, reproducible commands and gaps. Existing Python fixture tests remain
an independent regression check, not a complete semantic oracle.

Pass requires reproducible native/browser evidence; Wasm compilation or Node
execution alone does not pass. Unsupported targets or missing browser execution
must remain open, not inferred successful. No ADR-0009 conformance gate is closed.

## Result and recommendation

The bounded confirmation supports retaining Rust for the selected shared-core
direction. The same core produced identical canonical bytes, revisions and
probe error categories on native macOS ARM64 and actual browser WebAssembly.
This is feasibility evidence, not production validation, language conformance,
performance certification or a Studio decision. The Roadmap checkbox remains
open until owner approval and merge of this deliverable.

Use the [reproduction guide](../../tools/portability/README.md) and the exact
toolchain/lock in this PR. No existing fixture, contract snapshot, schema or
accepted ADR was changed. No parser or interpreter was started.

## Verification record — 2026-09-04 (including review fixes)

| Check | Observed result |
| --- | --- |
| Shared Rust corpus | 89/89 expected outcomes natively and in the browser |
| Complete native/browser reports | Byte-identical; includes canonical strings and error categories |
| Second run in the same Wasm instance | Identical report; no residual state observed |
| JS → Wasm raw-byte boundary | 6/6: three fixtures, invalid UTF-8, duplicate keys, exponent token |
| Independent Python oracle | 15 JCS/SHA-256 comparisons, all three unchanged fixtures, all ten kinds, policy/access/array/map invariants |
| Native CLI checks | Three exact canonical stdout results and four failure/exit-code cases |
| Original Python checker | All 14 test groups pass |
| Rust unit tests | Seven pass: shared corpus, three decoder tests and three CLI output/write/flush tests |
| Compile-fail doctests | Three pass: node/data distinction, node/occurrence distinction, missing enum arms |
| Formatting and Clippy | Pass on native and Wasm code, warnings treated as errors |
| Wasm release build and bindings | Pass with the pinned toolchain and CLI |
| CI | Added native tests/oracle and Wasm compile/lint job; not a browser-execution claim |

The native decoder test examines all 65,536 combinations of two arbitrary bytes
inside a quoted string, checking accepted values against the library decoder;
another tests 144 generated Unicode/control-character string pairs. This is
bounded robustness evidence, not exhaustive parsing coverage or fuzzing.

The shared cases include malformed UTF-8, overlong encodings, encoded/unpaired
surrogates, BOM, duplicate decoded names at multiple nesting sites, fractional
and exponent tokens (also inside annotations/dialects), safe integer endpoints,
overflow, integer negative zero, syntax failures and the exact 1 MiB/depth-64
boundaries. Unsupported contracts, missing/corrupt/ambiguous resource entries for
both semantic and dialect bindings, and reversed registry ordering are tested.
Structural negatives include unknown fields/kinds, wrong-map and dangling
references, root/scope problems and missing protection metadata.

The literal JCS vector exercises UTF-16 key ordering across BMP/non-BMP,
required escaping, no Unicode normalization, integer bounds and negative zero.
Fixed expected fixture revisions are independent constants. Mutated positive
cases have revisions prepared by the Rust helper, so they additionally require
the independent Python `rfc8785`/`hashlib` oracle; self-consistency alone is not
treated as proof. The browser compares complete JSON reports before displaying
their hashes. It does not implement an alternative validator.

### Environment and artifact identities

- Native: `aarch64-apple-darwin`, macOS 26.6.2 (25G83).
- Compiler: `rustc 1.98.1 (48a229cea 2026-09-01)`, full commit
  `48a229ceaefd4985c50990b14116b6d856af0985`, LLVM 22.1.8.
- Cargo: `1.98.1 (797e8a9bc 2026-08-05)`.
- Target: `wasm32-unknown-unknown`; release build, software SHA-256 backend
  explicitly selected in `.cargo/config.toml` for both targets.
- Browser binding crate and CLI: `wasm-bindgen 0.2.127`, CLI installed with
  `--locked` from the published package.
- Browser: actual Codex in-app Chromium session. Reported user agent:
  `Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/152.0.0.0 Safari/537.36`.
  This is a browser-reported compatibility string, not the physical host model.
- Initial setup used repo-local, ignored `.tools/cargo` and `.tools/rustup`,
  with no shell-profile change. Tool download permissions were requested.

Observed SHA-256 values (without the IR `sha256:` prefix):

| Artifact | Bytes | Digest |
| --- | ---: | --- |
| `Cargo.lock` | — | `fd3d27efb8927baeb5bf5160b30f2bcd09301c547dcc85cb817fa5eb9c74656d` |
| Python `requirements.txt` | — | `7585dfe55627ac2a68848a8357a3840dfd287b5e631d3a83fde66c7c0b904c75` |
| Generated browser module | 594772 | `ad42cd379f4418bbcb9a2427d213c061324cf222940628de24cfc7b311d88efc` |
| Native report | 132877 | `0a7f12f49ebda3ef66effcd54a26e5a9e942d0e72583b6abe50ab4ce6044755c` |
| Browser report | 132877 | `0a7f12f49ebda3ef66effcd54a26e5a9e942d0e72583b6abe50ab4ce6044755c` |

These identify the observed artifacts. Locked dependency resolution and repeatable
commands do not establish bit-reproducible binaries across build hosts or paths.
An offline native build in a fresh `target/portability-clean` directory also
produced a report identical to the primary build (`cmp` passed).
The compiler, linker, build scripts, environment and binding generator remain
part of the build trust boundary. No supported-platform or latency budget is
inferred from this one browser run. Safari, Firefox, Windows and other CPU
architectures have not been tested by this local evidence record.

### PR #13 review follow-up

All three unresolved inline findings at the start of this pass were independently
checked and accepted:

- [Python dependency integrity](https://github.com/cyborg-nomade/choreoform/pull/13#discussion_r3935333435):
  the original CI pinned only direct versions. CI and the documented local
  reproduction path now install the same complete hash lock with pip
  `--require-hashes --only-binary=:all:`. Direct versions remain unchanged.
  The lock includes `attrs 26.1.0`, `jsonschema 4.25.1`,
  `jsonschema-specifications 2025.9.1`, `referencing 0.37.0`, `rfc8785 0.1.4`,
  `rpds-py 2026.6.3`, and `typing-extensions 4.16.0` on Python <3.13.
  A fresh Python 3.14.6/pip 26.1.2 environment installed the applicable wheels
  with enforced hashes; `pip check`, both Python checks, and CLI checks passed.
  Package hashes do not replace source review, interpreter trust or a security audit.
- [Contract/artifact lengths](https://github.com/cyborg-nomade/choreoform/pull/13#discussion_r3935333446):
  the current two-element arrays agree, but `zip` would silently truncate a
  future mismatch. A compile-time assertion now rejects different lengths.
  The original zipped construction and content-digest checks remain intact.
- [Output flush failures](https://github.com/cyborg-nomade/choreoform/pull/13#discussion_r3935333463):
  successful writes to buffered stdout do not establish successful flushing.
  The CLI now locks stdout, writes the exact canonical bytes and explicitly
  flushes before returning success. Fault-injection tests cover short writes,
  write failure and a flush failure after successful writes; errors propagate
  through the existing nonzero-exit path.

The separate CodeRabbit 80% docstring-coverage warning is not adopted as a blanket
merge gate: no repository policy establishes that percentage, and the warning
does not identify a specific missing behavioral contract. This is not a claim
that documentation is complete. The resource constructor and output helper have
purpose-specific documentation, and the guide now distinguishes locked evidence
reproduction from convenience inline-script dependency resolution. No bot or
repository configuration was changed to suppress the warning.

The complete native/Wasm checks and actual browser run were repeated after these
changes. All 89 reports, six byte-boundary checks and the import allowlist still
pass. The report hash is unchanged; the rebuilt module's updated hash appears
above (the original `ffa1027` module hash was
`319d8ef2448e324a079de45e2a8d770a51b59f0188b599415fedbf6bc375e05d`).
The scope and all previously documented validation gaps remain unchanged.

## Implementation boundary and known omissions

The core accepts raw byte slices and explicitly supplied resource slices. Its
strict JSON stage runs before map construction discards duplicate names or
number conversion erases token spelling. String escape/surrogate decoding is
delegated to `serde_json` only after identifying a complete string token.
The custom canonicalizer implements the IR's restricted integer-only JCS
domain, not arbitrary floating-point RFC 8785 inputs.

`NodeId`, `DataId`, `ScopeId`, `ExpressionId`, `ActorId`, `CapabilityId`,
`PolicyId` and `OccurrenceId` are distinct types. Graph records are owned and
ID-indexed, not pointer-linked. The ten `NodeKind` variants have an exhaustive
match without a wildcard. Runtime checks resolve untrusted references to their
expected declaration map; static types alone do not admit a wire document.

`Inspected.document` retains the full decoded wire object. The typed `graph`
is intentionally partial: for example, the original join predicate, outcomes
and opaque policy bodies remain in the document rather than being modeled as
executable semantics. Do not serialize the typed view as an IR replacement.

The probe checks the closed envelope/body/declaration/node field names,
required fields, selected field shapes, IDs and reference kinds, connected
acyclic scope parents and scope entries, same-scope ports/flows, dialect
references, required protection fields and closed monotone predicate syntax.
It is **not equivalent to the complete JSON Schema or Python linkage checker**.
In particular, it does not yet establish all cardinalities, lexical visibility,
port-binding coverage, input-initializer restrictions, read/write dependencies,
activity/compute result publication, complete successor coverage, flow-cycle
absence, reciprocal split/fanout joins, fanout item/seal/key rules, or predicate
outcome validity. Those are follow-up validation obligations, not accepted
relaxations of ADR-0009.

Four synthetic `typed-*` cases exercise variant decoding only. They deliberately
do not claim valid complete processes: for example, the invocation/repeat cases
name the root scope and the compute case does not establish assignment/write-set
consistency. Passing these bounded cases must never be reported as admission to
execution. The three original fixtures remain independently schema/link checked.

Opaque illustrative dialect bodies are retained, not interpreted. The code
provides no rewrite, planning, execution, policy enforcement, external effect,
time or runtime identity operation. Empty access maps remain empty and cannot
grant access through this API. Unknown contracts are rejected for inspection;
general inert-view/export compatibility is not a product feature of this probe.

The 1 MiB/depth-64 transport limits are prototype limits. They are not a full
CPU/memory budget, incremental validator, cancellation mechanism, hardened
untrusted-input sandbox or performance benchmark. Resource slices and input
transfers must also be bounded by a real host before allocation/copying. The
native adapter bounds stdin reads; this browser page supplies fixed bounded
inputs. Stable user-facing diagnostics, source spans and recovery remain open.
No G1–G4 conditional gate from ADR-0009 is closed.

## Host/dependency review

Project crates inherit `unsafe_code = "forbid"`. Source inspection found no
ambient filesystem, network, time, randomness, process spawning or thread
creation in the portable core. The graph uses ordered maps/sets, and no process
meaning is selected from iteration order. Native I/O is confined to the test
CLI. Local fetching and UI operations are browser-adapter responsibilities.

The generated Wasm module has exactly one imported function:

```text
./choreoform_portability_bg.js :: __wbindgen_init_externref_table
```

The generated implementation initializes the binding's external-reference table
with JS constants. There are no WASI, network, clock, random or thread imports.
The browser harness checks the exact module/name/kind and import count before
initialization. The adapter transfers bytes through the generated safe binding;
its underlying allocation/ABI machinery is dependency code, not proof of a
wholly unsafe-free stack.

The workspace lock contains 29 third-party packages, all from crates.io with
checksums, plus the two local crates. The complete machine-readable inventory
is regenerated by `prepare.sh` as `generated/dependencies.json`. Inventory:

| Packages and pinned versions | Declared license metadata |
| --- | --- |
| `serde_json 1.0.151`, `serde_core 1.0.229`, `serde 1.0.229`, `serde_derive 1.0.229` | MIT OR Apache-2.0 |
| `sha2 0.11.0`, `digest 0.11.3`, `block-buffer 0.12.1`, `crypto-common 0.2.2`, `hybrid-array 0.4.14`, `typenum 1.20.1`, `const-oid 0.10.2` | MIT OR Apache-2.0 (equivalent order for const-oid) |
| `itoa 1.0.18`, `cfg-if 1.0.4`, `cpufeatures 0.3.1`, `libc 0.2.189` | MIT OR Apache-2.0 |
| `memchr 2.8.3` | Unlicense OR MIT |
| `zmij 1.0.23` | MIT |
| `wasm-bindgen 0.2.127`, `wasm-bindgen-macro 0.2.127`, `wasm-bindgen-macro-support 0.2.127`, `wasm-bindgen-shared 0.2.127` | MIT OR Apache-2.0 |
| `bumpalo 3.20.3`, `once_cell 1.21.4`, `proc-macro2 1.0.107`, `quote 1.0.47`, `rustversion 1.0.23`, `syn 2.0.119`, `syn 3.0.4` | MIT OR Apache-2.0 |
| `unicode-ident 1.0.24` | (MIT OR Apache-2.0) AND Unicode-3.0 |

The lock inventory is larger than either target's linked runtime graph; it
includes optional/build/macro resolution. This is a license/source inventory,
not a legal certification. Preserve upstream notices if distributing artifacts.
The generated probe also embeds project fixtures and attributed frozen contracts.

Targeted source review identified unsafe code in serialization/string handling,
integer formatting, digest buffering/array operations and Wasm ABI conversion.
SHA-256's configured software path excludes its hardware-backend dispatch, but
Cargo still lists native `cpufeatures`/`libc` dependencies. Rust's standard
library, allocator and dependencies remain trusted; this review is not a complete
third-party memory-safety or vulnerability audit.

The inventory identifies ten build-script packages: `libc`, `proc-macro2`,
`quote`, `rustversion`, `serde`, `serde_core`, `serde_json`, `wasm-bindgen`,
`wasm-bindgen-shared`, and `zmij`.
Their targeted host-access review found target/compiler configuration probes,
environment reads, compiler/wrapper invocations, generated build-output files,
and (for `wasm-bindgen-shared`) a source-file read and `git rev-parse HEAD`.
`libc` also contains platform-specific version probes. These are build-host
operations, not portable-core runtime effects. No project-owned build script or
project-owned native C binding was added. Do not treat build scripts as sandboxed
by safe Rust.

The separately installed `wasm-bindgen-cli` has its own published lock and a much
larger host-only dependency tree, including server/network and native build
tooling. It is not linked into the browser core. Its installation succeeded but
reported future-compatibility warnings for `buf_redux 0.8.4` and `multipart
0.18.0`; those are tooling maintenance risks, not silently classified as core
test failures or eliminated dependencies. No release packaging or exhaustive CLI
dependency audit is claimed here.

Primary API references: [serde_json](https://docs.rs/serde_json/1.0.151/serde_json/),
[RustCrypto SHA-2 backends](https://docs.rs/sha2/0.11.0/sha2/), and
[wasm-bindgen](https://docs.rs/wasm-bindgen/0.2.127/wasm_bindgen/).
The concrete inventory and host-access observations above come from the locked
local package sources and generated module, not only their documentation.

## Review questions and recommendations

1. **Accept this bounded confirmation?** Recommend yes: the required native/
   browser feasibility, exact wire/hash parity, typed modeling and explicit host
   boundary have evidence. This is not approval of a production validator.
2. **Promote the probe into a product API now?** Recommend no. Keep the explicit
   probe names and partial-model limitations until complete structural/semantic
   validation and diagnostics have their own reviewed contract and tests.
3. **Require more target coverage before any parser design?** Recommend no for
   this bounded gate. Require the appropriate OS/browser matrix before promising
   support or choosing the Studio delivery architecture. The CI build and one
   actual browser are evidence, not universal portability.
4. **Start the next Roadmap item automatically?** No. This PR is the handoff
   boundary; owner review/approval and further direction come first.
