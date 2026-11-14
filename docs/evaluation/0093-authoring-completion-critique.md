<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Remaining authoring completion: critique before further implementation

**Date:** 2026-11-14<br>
**Status:** measured critique; direction/integration verification pending.<br>
**Authority:** signed [scope93](0093-authoring-completion-critique-plan.md)c30eb10,
after verified PR #107 integration4042081c3b8ea990ad1d4e93357ad368889cdaf8;
CI43581960139 atdd809f4568bd0642a5ec5376da9360a303339d66, automated COMMENT
review5903518032 and fresh full audit. No personal owner or independent review.

## Recommendation

Prioritize a **reversible authoring-package and managed-change contract**, then
its small native implementation and paired text/visual prototype. This is the
next bounded foundation for the remaining Phase1 authoring work. Keep the complete
RP-08 package, actionable diagnostics and actual representative-user evaluation
explicitly in Phase1. Broader work/service blueprints remain a separately
contracted follow-up, guided by full models and observed author tasks.

The [Manifest](../../MANIFEST.md) requires one meaning across both primary forms,
progressive disclosure and safe transformation. The [Roadmap](../../ROADMAP.md)
requires three representative processes in both forms, stable roundtrips,
actionable invalid constructs and actual representative-user evaluation. More
aliases or larger quotas cannot supply those missing deliverables. This is a
requirements-based design judgment; it is not user-comprehension evidence.

## Actual maintained baseline

Both maintained packages now use source19/companion18 on unchanged C12/IR0.4.
The [raw existing native measurements](0093-current-native-metrics.json) reproduce
these complete sources, with no new measurement code or product implementation:

| Native observation | RP-01 | RP-03 |
| --- | ---: | ---: |
| Source bytes | 54172 | 291309 |
| Companion raw bytes | 53726 | 242843 |
| Candidate IR raw bytes | 108565 | 883289 |
| Tokens / parser visits | 9145 /113796 | 41306 /386333 |
| Nested scopes / steps / work policies | 8 /46 /7 | 59 /198 /29 |
| Pure definitions | 1 | 2 |
| Qualified references | 106 | 340 |
| Outer expression bytes | 21428 | 84431 |
| Exact repeated-expression spelling upper bound | 128 | 7156 |
| Outer type bytes / exact spelling upper bound | 5974 /2223 | 61727 /33754 |

The last two rows are not projected savings. Definitions/references have costs;
type/expression categories overlap and must not be summed as a whole-program
partition. Parser visits are algorithm-specific work counts, not elapsed speed.
RP-03's repeated-expression upper bound is only7156 bytes after factoring, while
the complete source remains291309 bytes. Another pure-predicate pass cannot by
itself solve navigation, editing, reversible structure or missing visual forms.

The complete965-file current/historical benchmark inventory remains
SHA256a25c6de850fdaa51436023fffe011c8f96695e9d8660191e7598cd4f4bf2e9f7.
All accepted contracts, raw evidence, resources, reports and guards remain exact.
RP-01 keeps all11 passing partial views; RP-03 retains its external-region refusal.

## Concrete gaps

**Source-only structure is intentionally erased from execution IR.** A real native
comparison of RP-01's complete archived source18 package and current source19
package gives byte-identical lowered IR. Native formatting preserves different
sources: the new one has `Define test`, the old one does not. Source19 pure tests,
transparent type names and protection spellings must not become runtime calls to
make an editor convenient. Instead, roundtrip design must explicitly retain
versioned authoring information alongside semantic IR and layout metadata.
Preserving original source during `Package::formatted` is useful, but is not an
arbitrary IR-to-text exporter or a visual-to-IR-to-visual cycle.

**Managed bookkeeping is still a developer task.** The native frontend exposes
strict parsing/lowering, original-package formatting and partial checking. It
does not expose managed rename/copy/retire/edit operations or a general exporter.
The companion is about as large as source for RP-01 and242843 bytes for RP-03.
Those bytes have important identity/provenance meaning; asking ordinary authors
to maintain their rows manually does not meet the roadmap's authoring goal.
Private Python benchmark producers are not an authoring product.

**Diagnostics are private, but not yet a complete author explanation.** Replacing
the actual RP-01 Instruction argument with a text literal returns no candidate or
secret echo, and native `expression-type at 8900..9585`. The refusal is sound.
The685-byte work-policy span and technical code do not identify the expected
Payable instruction or its argument directly. That is a design/actionability gap,
not a claimed usability study. Exact argument/field locations, stable diagnostic
catalogue and domain-oriented presentation still need implementation and testing.
Additional source assertions already reject otherwise-valid wrong contracts;
invalid expanded expressions may fail ordinary native typing first.

**RP-08 is still a brief.** The full
[incident-response case](../representative-processes/08-cybersecurity-incident-response.md)
requires evolving scope/work, guarded dangerous actions, confidentiality-appropriate
views and outage reconciliation without inventing total order. There is no complete
lowerable source/companion/IR/report package. RP-01/03 alone cannot satisfy the
three-process requirement. The fictional brief is not operational security advice.

**Software evidence and external evidence are different.** All688 Rust tests plus6
compile-fail tests and87 Python commands passed at #107. Native/Wasm compilation
does not establish actual browser execution, representative-user comprehension,
independent review, live provider truth or production host authorization. A skipped
bot is no review. None may be replaced by generated observations or personas.

## Sequencing alternatives

| Option | Benefit | Main constraint | Recommendation |
| --- | --- | --- | --- |
| A. Reversible package and managed authoring changes, then paired prototype | Addresses identity bookkeeping and enables both-form cycles without changing runtime meaning | Must validate authoring/IR consistency and make protection/authority boundaries explicit | First; publish exact contract before implementation |
| B. General work/service blueprints first | Could reduce repeated complete occurrence construction | Broader typing/provenance/identity/capture/compensation contract; visual representation and editor support also needed | Separate later decision after model/task evidence; no inferred authority or shared work shortcut |
| C. Full RP-08 first, without authoring foundation | Exposes the third domain and remaining semantic pressures | Another hand-maintained large source/ledger would not close editing/visual gates | Begin its complete coverage/design inventory alongside A; build full package as an explicit subsequent deliverable |
| D. More English sugar/aliases first | Can improve specific phrases | Cannot supply managed identities, preserved authoring state, visuals, full RP-08 or user evidence | Targeted later changes based on concrete diagnostics/tasks; no broad syntax round now |

A does not mean implementing the Phase3 studio early. The Phase1 slice is a small
contract and testable prototype sufficient to establish semantic preservation,
managed identities and accessible inspection/editing. Runtime simulation, debugger,
production collaboration and deployment remain their later roadmap deliveries.

## Required completion sequence and gates

| Open Phase1 obligation | Concrete remaining deliverable/evidence |
| --- | --- |
| Near-English authoring and protected accessible inspection/editing/export | Exact reversible package/change/authority contract; native managed operations; task-oriented prototype; explicit exports and refusals; actual representative-user evaluation |
| Visual notation and layout metadata | Initial faithful notation for accepted constructs, versioned non-semantic layout, captions and accessible alternatives; no hidden unsupported construct |
| Text/visual roundtrips | Complete text → model → text and visual → model → visual cycles for RP-01/03/08; stable identities/output; authoring-only definitions preserved; edits validated against semantic IR |
| Complete validator and diagnostics | Consolidated accepted-construct coverage matrix, remaining semantic rules/refusals implemented, private field/argument diagnostics and meaningful negative cases; current partial reports stay explicit |
| Executable and negative examples / three representative processes | Small accepted executable-contract examples and negatives plus full RP-08 package; retain all scenario limitations and no production-engine claim |
| Aggregate IR/G1–G4/scenario gate | Complete contract/validation/conformance/benchmark/cross-form evidence and honest conditional accounting; host/region assumptions cannot be turned into facts by a renderer |
| Actual external evidence | Representative users, independent review and actual required browser/host evidence through documented real methods; schedule/collect without fabrication after appropriate independent preparation |

Immediate next scope: publish the exact reversible authoring-package and managed-
change contract, including source/companion/resource/semantic pins, stable identity
operations, layout/authoring-only representation, consistency checks, unsupported
cases and protection/authority boundaries. Compare alternatives before publication.
Then implement the bounded native operations and paired prototype in subsequent
dedicated scopes. A source annotation, report or digest must never grant live host
permission. Untrusted or inconsistent authoring metadata must not change or hide
the actual semantic model through an unchecked visual projection.

Complete RP-08 design/coverage, general work reuse, diagnostic refinement and full
cross-form/user evidence remain explicit dependencies within Phase1. They are not
silently deferred to Phase2/3. Freeze/publish any new language/IR/API contract before
its production implementation. No headline box or conditional gate closes here.
All40 scenarios remain Partial and parent46 draft; stop only at an honest Phase1
report for owner approval, without starting Phase2.
