<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Code critique: simplify the prototype before extending it

**Date:** 2026-09-11<br>
**Baseline:** `167633e3bc20807cc2dfcff3f5a1e98161bfce94` (merged PR #14)<br>
**Status:** Review proposal only. No implementation changes authorized or made.

## Recommendation

Keep Rust, explicit semantic boundaries, strict transport, and the existing
independent evidence. Before substantially extending the implementation, make a
small, measured refactoring pass: linearize scope checking, bound output during
generation, remove redundant parsing/copying, and simplify the source model.
Do not replace the parser framework, add async infrastructure, or build an
elaborate compiler framework merely to make this small prototype look modern.

The source tree is already flat. The problem is duplicated representations and
string-based distinctions, not excessive tree depth. Simplification should mean
fewer representations and invalid internal states, not fewer semantic safeguards.

The most important product constraint remains unchanged: the authoring language
must approach plain English. Cleaning up this JSON-oriented parser does not
deliver that language. Preserve the explicit Roadmap item and user evaluation;
avoid investing heavily in syntax intended to be replaced.

## Scope and evidence limits

Reviewed the Rust transport, canonicalization, partial graph model, source parser,
lowering/export and CLI adapters; Python fixture/oracle checks; portability
adapter/browser harness; workspace/toolchain configuration and CI. Findings are
against the local baseline above, not a claim to audit subsequently pushed code.

This is an implementation critique, not an ADR accepting a new architecture,
language evaluation, security certification, dependency vulnerability audit, or
authorization to close the executable IR gate. The same assistant contributed
to the implementation; this is not an independent human review.

Evidence labels below distinguish **measured** behavior, **reproduced** failure,
**code-inspected** allocation/control flow, and **design recommendation**. No
allocation profiler or browser performance experiment was completed. Timing
results are diagnostic samples, not performance guarantees or speedup promises.

### Existing verification

On macOS 26.6.2 arm64, pinned Rust 1.98.1, using the existing lockfile:

- Workspace tests pass: 15 parser integration tests, 7 existing Rust unit tests
  (including the 89-case shared suite), and 3 compile-fail tests.
- Formatting and native all-target Clippy pass; parser wasm library Clippy passes.
- Python wire checker passes its 14 groups.
- Text CLI oracle passes 3 IR/schema/JCS comparisons, 3 stable export cycles,
  and 7 refusal cases.
- Optimized native binaries build offline. No source or dependency changes were
  needed. Build products are ignored; this critique is the only new tracked-path
  artifact. No commit, PR, or remote update was made for this review.

## Findings and priority

The scores are rough scheduling estimates, not correctness or language-fitness
scores. Impact/risk/effort are 1–5; score = (impact + risk) × (6 − effort).
Dependencies and safeguards still govern implementation order.

| ID | Finding | Evidence | Impact / risk / effort | Score | Timing |
| --- | --- | --- | --- | ---: | --- |
| F1 | Repeated ancestor traversal scales poorly | Measured + code-inspected | 4 / 4 / 2 | 32 | First refactoring slice |
| F2 | Generated output is bounded only after allocation | Reproduced refusal + code-inspected amplification | 3 / 4 / 2 | 28 | First refactoring slice |
| F3 | Python graph checking hits recursion limits | Reproduced | 3 / 3 / 2 | 24 | First refactoring slice |
| F4 | Repeated parse/serialize/clone/canonicalization work | Code-inspected; stage timings not isolated | 4 / 3 / 3 | 21 | After boundary tests |
| F5 | Source items duplicate section bookkeeping | Code-inspected + design recommendation | 3 / 3 / 2 | 24 | With F4, bounded scope |
| F6 | Probe types could be mistaken for validated product IR | Documented design debt | 4 / 4 / 3 | 24 | Before validator/runtime consumers |
| F7 | Tests do not yet cover scale and all refusal boundaries | Observed coverage gaps | 3 / 3 / 2 | 24 | Alongside every slice |

### F1 — Validate the scope forest once

**Location:** [model.rs](../../crates/ir-probe-core/src/model.rs), lines 245–276;
similar repeated traversal in [check_ir_fixtures.py](../../tools/check_ir_fixtures.py),
lines 132–162.

Every scope independently walks to the root, allocating a new visited set and
cloning/string-looking-up ancestor IDs. A chain of N scopes requires roughly
N(N+1)/2 visits, with ordered-map/set lookup costs on top. JSON nesting limits
do not bound this depth: scopes are flat records linked by IDs.

Optimized native `inspect` timings, milliseconds, median of three runs after
one warm-up per case:

| Scopes | Star bytes | Chain bytes | Star median | Chain median |
| ---: | ---: | ---: | ---: | ---: |
| 128 | 35,517 | 35,661 | 3.089 | 4.468 |
| 512 | 137,661 | 138,573 | 6.097 | 33.977 |
| 1,024 | 273,949 | 275,908 | 11.061 | 123.594 |
| 2,048 | 550,429 | 555,460 | 20.072 | 504.202 |

All inputs fit the 1 MiB transport limit and pass the existing partial Rust
inspection. These synthetic declaration graphs are not evidence of complete
executable process validity. Timings include process startup, IO, parsing,
checking and hashing; they do not isolate CPU attribution. The second run of
the experiment was used here, after concurrent build/lint activity had finished.
Even then this is an ordinary developer machine, not a controlled benchmark host.

**Proposal:** resolve parents once, then use iterative visiting/visited state
and memoized root reachability. Preserve cycle, missing-parent, disconnected-root
and scope-entry checks. With string-keyed ordered maps this can be O(N log N);
dense transient indexes can approach O(N + E), but are not mandatory now.
Do not impose an arbitrary new scope-depth language restriction to mask the cost.

**Acceptance:** positive star/chain fixtures at increasing sizes, deep cycles,
disconnected components and dangling parents; each resolved parent edge processed
a bounded number of times. Keep wall-clock measurements informational in CI.

### F2 — Stop oversized export while writing, not after formatting it

**Location:** [text lib.rs](../../crates/text-prototype/src/lib.rs), lines 320–379.

PR #14 correctly separates generated-source refusal from representability.
However, it first allocates each pretty JSON string, formats another string,
appends to the complete source, and only then checks `source.len()`.

A 303,929-byte IR artifact with an annotation containing 40 nested arrays around
150,000 zeroes was refused correctly with no stdout in about 17.6 ms. It is
below the input and nesting limits. Code inspection of the pinned pretty printer
(two-space indentation) implies over 12 MB of indentation alone before refusal.
That is an allocation-amplification risk, not a measured peak-RSS value, an
unbounded-memory claim, or a reopened error-classification bug.

**Proposal:** serialize into one capped in-memory writer that refuses writes
past the output budget; write outer syntax directly into that writer too. Avoid
whole-record `to_string_pretty` intermediates. Keep the completed buffer private
until export checks succeed, then write stdout: directly streaming to stdout
would violate the existing no-partial-output-on-rejection behavior.

The serializer supports writing to a `Write` destination; a capped buffer can
implement that interface. This does not imply generic JSON serialization is JCS.
[serde_json writer API](https://docs.rs/serde_json/1.0.151/serde_json/fn.to_writer.html).

**Acceptance:** exactly-at/just-over output limits, deep annotation expansion,
writer/flush failures, unchanged normalized output and input-artifact spans.
Test that the writer refuses before buffered bytes exceed its declared budget.

### F3 — The independent checker should not recurse through graph edges

**Location:** [check_ir_fixtures.py](../../tools/check_ir_fixtures.py), lines 318–333.

The `acyclic` function recursively traverses causal edges. A shallow JSON document
containing a linear 1,100-node wait/finish graph (180,166 compact bytes) reaches
`RecursionError` under the existing Python recursion limit of 1,000. It reaches
the cycle-checking stage after earlier shape/link checks. This is a test-tool
scalability failure, not evidence that the Rust parser or a running engine fails.

**Proposal:** use an explicit DFS stack with entry/exit markers or an equivalent
iterative algorithm, preserving cycle detection across split/join edges. Do not
increase the Python recursion limit as the main remedy. Keep this implementation
independent of the Rust checker so it can still detect Rust regressions.

**Acceptance:** long acyclic chains and deep back-edges produce a normal success
or intended validation error, never interpreter recursion failure.

### F4 — Remove repeated representation conversions

**Locations:** [text lib.rs](../../crates/text-prototype/src/lib.rs), lines 76–119
and 320–379; [core lib.rs](../../crates/ir-probe-core/src/lib.rs), lines 150–164;
[transport.rs](../../crates/ir-probe-core/src/transport.rs), lines 174–198.

Current lower path: decoded syntax payloads → cloned body/envelope → serialized
JSON bytes → strict decode again → cloned semantic projection → canonical
strings → digest. Calling `binding()` subsequently performs lowering again.
Export separately decodes input twice, generates text, reparses it and lowers it
for equality. Some of this is deliberate evidence; none is needed merely to
calculate the same hash twice in an editor operation.

Canonicalization recursively builds per-field/per-element strings, collects
vectors, joins them, and wraps the result. Long payloads are copied at successive
levels. These are visible allocation sites, but their individual runtime cost
has not been profiled; prioritize measurement before promising a particular gain.

**Proposal:**

1. Add a private, transport-admitted document/value boundary. Validate integer,
   Unicode, depth and byte-budget invariants once; do not expose arbitrary
   `serde_json::Value` as if it had passed strict admission.
2. Canonicalize a borrowed semantic projection into one writer. Keep explicit
   UTF-16 key sorting, ordered arrays and exact escaping. Rust map iteration or
   generic `to_writer` alone is not a replacement for the JCS contract.
3. Return candidate IR and source binding together from one lowering operation.
   Offer a consuming path if it removes payload clones without losing the
   caller's source/spans. Avoid global caches and stale bindings.
4. Decode export input once. Retain the production self-round-trip guard until
   representability is independently enforced and adversarial tests justify any
   removal. Moving it solely to tests is a separate approval choice.

Removing the current serialize/decode round trip must not remove its *whole
envelope* depth/size checks. Per-payload validity does not establish the final
envelope is within limits. Likewise, ordinary deserialization can erase duplicate
keys and numeric token distinctions before validation can inspect them.

### F5 — Simplify the source tree with explicit alternatives

**Location:** [text lib.rs](../../crates/text-prototype/src/lib.rs), lines 41–63,
245–313. The current tree is flat and useful; no recursive AST collapse is needed.

`Item { name: String, span, records: Vec<RecordSpan>, value: Value }` serves both
metadata and sections. Sections store IDs in both a JSON map and a separate
source-order record list; scalar items carry an empty list. Parser invariants
currently keep these consistent, but the relationship is implicit in the type.

**Preferred shape, illustrative only:**

```text
Syntax { original_source, items: Vec<Item> }
Item = Metadata { kind: MetadataKind, value, span }
     | Section  { kind: SectionKind, records: Vec<Record>, span }
Record { id, payload, span }
```

Keep one source-order record collection and construct semantic maps only at
lowering. Represent the fixed metadata/section vocabulary as enums, borrowing
identifier slices or using source spans where that remains simple. Retain decoded
opaque payload values initially; reparsing raw payload spans on every operation
would trade away F4's improvement. Keep all comments/trivia in original source.

This modest use of enum variants expresses distinct shapes directly; it is a
type-safety/maintenance recommendation, not a demonstrated speed improvement.
[Rust enum guidance](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html).

| Option | Benefit | Cost | Recommendation |
| --- | --- | --- | --- |
| Keep current tree, only rename fields | Minimal churn | Parallel bookkeeping remains | Reasonable if this parser is immediately retired |
| Flat typed items with one record vector | Explicit invariants, preserved order/spans, fewer duplicated IDs | Small API/test adjustment; maps built during lowering | Preferred bounded refactor |
| Lossless token/green-tree infrastructure now | Foundation for incremental tooling | Substantial new design before near-English grammar is chosen | Defer until editing/recovery requirements justify it |

Do not merge the syntax tree with the canonical IR or partial graph model.
Source order and trivia matter to editing; canonical declaration-map order does
not. Tool-managed stable IDs must not be regenerated from labels or vector
positions. A future owned source buffer can address long-lived editor lifetimes;
an `Arc`, arena, interner or self-referential structure is not required today.

### F6 — Prevent accidental promotion of a probe into a semantic validator

**Locations:** [model.rs](../../crates/ir-probe-core/src/model.rs), lines 77–80,
120–131 and 512–517; [core lib.rs](../../crates/ir-probe-core/src/lib.rs), lines
79–86 and 192–217; workspace dependency from text prototype to probe core.

The typed graph intentionally omits information (for example, the join predicate
is checked but not stored in `NodeKind::Join`; flows and complete declaration
maps are not represented by `Graph`). The complete JSON document is retained
separately, so this is **not current serialization loss**. It becomes dangerous
only if later code uses this partial graph as complete executable IR.

The core also embeds a two-contract support list, while accepting supplied
resource bytes. That is a good bounded probe but not yet a general dialect
registry. Comments state these limits; public API names/return types could make
them harder to overlook.

**Proposal:** separate wire/admission/canonical utilities from fixture-specific
contract support and graph inspection, initially as modules rather than many
new crates. Introduce clear private-field stage types such as `CandidateIr` and
`InspectedPrototype`, and add `ValidatedDefinition` only when its actual
validation contract exists. Do not create an empty marker that implies safety.
Keep `NodeId`, `DataId` and occurrence identity distinct. Retain accepted safe
Rust and explicit host IO boundaries.

Complete types/expressions/policies and semantic validation remain Roadmap work,
not incidental cleanup. Any consequential public boundary/format change requires
the normal ADR review; this critique does not accept it.

### F7 — Strengthen tests where refactoring is most likely to break guarantees

**Locations:** [roundtrip.rs](../../crates/text-prototype/tests/roundtrip.rs),
lines 216–283; [.github/workflows/rust-portability.yml](../../.github/workflows/rust-portability.yml);
[browser.mjs](../../tools/portability/browser.mjs).

- The truncation loop asserts only absence of panic. Assert rejection of
  syntactically incomplete prefixes, with intentional exceptions for complete
  documents followed by partial trailing whitespace/comments. Add UTF-8 cuts.
- Some malformed cases append an unrelated invalid `ignored` item, so a failure
  does not prove the intended lexical rule fired. Dedicated lexical tests already
  mitigate this; remove ambiguous duplicate cases or assert the intended error.
- Add scope/graph scale and output-budget tests from F1–F3. Later fuzzing should
  cover arbitrary UTF-8 and nested payloads; arbitrary single bytes and the
  existing two-byte transport corpus are valuable but not a full fuzz campaign.
- Preserve independent Python/RFC 8785 comparisons. Do not DRY away independent
  oracles by routing every expected result through the code under test.
- CI compiles the parser for wasm but does not execute it there. The browser
  harness exercises the older portability core, not the text parser. When shared
  canonical/parser boundaries change, run relevant browser parity evidence;
  automate execution when a supported browser runner is deliberately selected.
- The text CLI handles write/flush errors, but the explicit fault-injection
  writer tests live in the older CLI. Add equivalent coverage if output handling
  is consolidated. Do not infer it from the unrelated tests passing.

## Efficient patterns worth retaining or adopting

- **Keep:** owned typed IDs, closed node variants, Result-based failure, exact
  version admission, no implicit network lookup, deterministic fixture evidence,
  lockfiles, safe Rust, and no-partial-output refusal.
- **Adopt selectively:** borrowed read-only projections, bounded writer APIs,
  iterative graph algorithms, explicit enum alternatives, and stage-specific
  types with constructors that actually enforce their stated invariants.
- **Do not cargo-cult:** async/threads for a synchronous CPU-bound parser,
  pervasive trait objects, a large framework, or `Rc<RefCell<_>>` graph ownership.
  They do not address the observed repeated work.
- **Do not replace every BTreeMap with HashMap:** representation ordering,
  determinism and lookup workload differ. Optimize measured indexes; preserve
  canonical sorting explicitly. The standard library documents the collection
  trade-offs; no map is universally best.
  [Rust collections guidance](https://doc.rust-lang.org/std/collections/index.html).
- **Defer micro-optimizations:** per-byte digest formatting, temporary required-
  field vectors, small reference clones, and repeated tiny registry allocations
  are visible but secondary. The software SHA-256 backend is an explicit probe
  choice; do not override it without native/wasm evidence and decision review.

## Proposed implementation sequence — requires approval

1. **Safety and scale slice:** focused F1/F2/F3 regression tests and minimal
   iterative/bounded implementations. No grammar or wire-format change.
2. **Representation slice:** F4/F5 with before/after stage benchmarks, one private
   admission boundary, borrowed canonical projection and typed flat source items.
   Preserve the runtime export guard initially.
3. **Product-boundary slice:** coordinate F6 with the already planned names,
   types, scopes, imports, parameters and composition deliverable. Do not turn
   this review into an unapproved executable-dialect implementation.
4. **Authoring work:** return to the accepted Roadmap. Near-English language
   design and representative-user studies remain distinct from AST cleanup.

Acceptance for every slice: unchanged frozen canonical revisions, byte-stable
normalized exports where currently promised, exact source/trivia retention,
valid source bindings, strict duplicates/numbers/Unicode/resource refusal,
deterministic errors, tests and independent oracle passes. Compare optimized
scaling, not debug-only wall time. Record any deliberate diagnostic/API changes.

## Reproduction notes

From the repository root, use the existing local toolchain environment:

```sh
export CARGO_HOME="$PWD/.tools/cargo"
export RUSTUP_HOME="$PWD/.tools/rustup"
export PATH="$CARGO_HOME/bin:$PATH"
cargo test --workspace --locked --offline
cargo build --release -p choreoform-text-prototype -p choreoform-portability --locked --offline
```

Scope timing inputs were generated in memory from `01-reimbursement.json`:
retain envelope/contracts/annotations and policy payloads; clear the eight maps
except policies; set every policy scope to `s0`; clone the original scope shape
for `s0`…`sN-1`, empty its ports, set outcomes to `{done: true}` and entry `nI`.
Each `nI` is a finish node in `sI`, with empty reads/writes/outcomes and scope
outcome `done`. Root parent is null. Other parents are `s0` (star) or `sI-1`
(chain). Recompute the semantic digest with the independently installed
`rfc8785.dumps` and SHA-256; serialize compact JSON. Run
`target/release/choreoform-portability inspect` with those stdin bytes and stdout
discarded, timeout 20 seconds. Exclude generation/hash preparation from timing.
Warm once and report the median of the next three calls using `perf_counter`.

Raw measured star/chain samples, milliseconds:

```text
128:  star [3.059, 3.202, 3.089];    chain [4.415, 4.845, 4.468]
512:  star [6.067, 6.208, 6.097];    chain [33.300, 34.431, 33.977]
1024: star [11.061, 10.599, 11.109]; chain [123.594, 122.710, 130.976]
2048: star [19.941, 20.558, 20.072]; chain [504.202, 506.956, 499.643]
```

Export expansion input: keep the complete reimbursement body/revision, replace
annotations with `{"nest": v}`, where `v = [0] * 150000` wrapped in 40 successive
single-element lists. Compact JSON is 303,929 bytes. Call the release text CLI
with `export`; expect exit 1, zero stdout bytes, and
`source size limit at bytes 0..303929`. An empty-annotations control is 3,841
input bytes, exports successfully to 4,733 bytes, observed once at 2.939 ms.
The expanded case was observed once at 17.593 ms; neither is a timing benchmark.

Python graph failure input: retain contracts/policies, clear data/expressions/
actors/capabilities/flows/nodes, create one root `s0` with empty ports, entry `n0`
and outcome `done`, and put all policies in `s0`. Create 1,099 wait nodes with
empty reads/writes, outcome `done`, and an existing policy reference, followed
by one finish node; create one `done` flow between successive nodes. Recompute
revision with `wire.revision`, then call `wire.check` from the existing
`.tools/ir-check` environment. At recursion limit 1,000 this 180,166-byte graph
raises `RecursionError` in recursive graph checking. No claim about executable
wait-policy meaning follows from this structural diagnostic.

## Review choices

Recommended approval scope is the first two refactoring slices, preserving
all accepted semantics and formats, then return to the Roadmap. Alternatively,
approve only safety/scale fixes and retire the current source parser when the
near-English design is ready. The latter minimizes investment in temporary
syntax but leaves representation debt for that future work. A full compiler or
parser-framework rewrite is not justified by the evidence collected here.

No implementation work begins until you choose the scope.
