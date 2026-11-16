<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Full RP-08 requirements and contract-gap critique

**Date:** 2026-11-16<br>
**Method:** Same-agent requirements/code audit under signed
[scope8830ae7](0097-rp08-coverage-design-plan.md), after verified PR111 integration
`92b83cf794cf3cca9c94045b41d424190c3f515b`.<br>
**Evidence boundary:** Existing contracts/code/tests, not a new RP08 package,
execution/refusal trace, representative-user study or independent review.

The complete fictional [incident-coordination brief](../representative-processes/08-cybersecurity-incident-response.md)
cannot yet be represented as a claimed complete lowerable benchmark. Existing
stable-key batches, human/capability policies, revision-sensitive work and owned
reconciliation are useful foundations. They do not supply a general offline-history
importer, dynamic result return, arbitrary ad hoc template creation, protected
artifact audiences or whole effectful-parallel proof. These are concrete Phase1
dependencies, not reasons to silently narrow the brief or move it into Phase2.

The [original two-surface treatment](../authoring/benchmarks.md#rp-08--incident-coordination-including-changing-scope-and-outage),
[whole-case requirements](0034-benchmark-readiness.md#rp-08-package-requirements) and
[four-scenario accountability](0034-corpus-accountability.md) remain historical,
unchanged evidence. Every RP08 scenario remains Partial. This critique inventories
the complete case before proposing the smallest adequate new contracts. Fictional
participants/effects below are language requirements, not operational security,
severity, legal-notification or forensic advice.

## What exists, and what it actually establishes

| Existing source | Established capability | Boundary relevant to RP08 |
| --- | --- | --- |
| [ADR0008](../decisions/0008-core-process-semantics.md), [ADR0009](../decisions/0009-canonical-versioned-ir.md) | Stable definitions/occurrences, explicit scope/control/data/obligation/authority and causal distinctions | Paper selection/structural schemas are not full semantic admission or accepted runtime records |
| [Policy P1–P7](../dialects/policies.md) | Human authority rechecked at acceptance, independent work/target authority, exact effect/retry/cancellation/settlement and keyed membership/seal obligations | Authenticated facts, correlation, current revisions, atomic acceptance and accepted live transfers remain host obligations |
| [Current source19](../authoring/profile-v0.19.md), [native lowering](../../crates/authoring-frontend/src/lower/nodes.rs) / [graph validation](../../crates/authoring-frontend/src/lower/graph.rs) | All ten declared node kinds; finite declared child templates, explicit interfaces/IDs/resources and first-profile managed edits | No arbitrary runtime definition creation; repeat has no ports and fanout has one item input/no child outputs. Publication-time pending labels are historical; integrated compiler/test evidence establishes current support |
| [Native static node rules](../../crates/semantic-check/src/definition/nodes.rs) | Fanout collection is a list of the exact item type; key is text, seal Boolean, child item is fresh/uninitialized, changes policy has the membership kind | These judgments do not materialize per-item results into parent data or prove live membership/settlement |
| [Ordinary availability](../../crates/semantic-check/src/control/availability.rs) | Child scopes are checked; parent completion requires actual visible initialized outputs | A child's local writes do not automatically become a parent result bundle |
| [Concurrency rules/report](../../crates/semantic-check/src/concurrency.rs) | All-child lifetime predicate, terminal boundaries and conservative independence; fanout compares a template's shared footprint with itself | General effectful parallelism, live membership, irreversible safety, current permissions and host enforcement remain explicit unproved obligations |
| [Protection report](../../crates/semantic-check/src/protection.rs) | Conservative ordinary transfer constraints and explicit future access obligations | No authenticated grant or proof against all exceptional/observable-channel leakage |
| [Typed terminal observations](../dialects/provider-terminal-observation-sources.md), [wait capture](../dialects/core-wait-capture-v0.5.md), [retained request](../dialects/core-retained-request-v0.8.md), [finite owned work](../dialects/core-owned-work-v0.11.md) | Exact declared origins/types, private local captures, original-request references and finite conditional owned continuation classes | Do not discard these later accepted capabilities because older P4 prose lacked them. None is a generic foreign-history importer, arbitrary work engine or accepted original snapshot/current receiver |
| [Physical request audit](0082-request-region-proof-audit.md) | Actual declaration/port/retention links and mathematical counterexamples to inferred physical independence/snapshot identity | Distinct labels, nominal membership and C13 selection metadata do not prove canonical physical aliases, original accepted revisions or admissible surviving work |
| [Complete authoring document](../authoring/reversible-v0.1.md), [paired prototype evidence](0096-paired-authoring-prototype.md) | Complete source/companion/IR-consistent artifacts, first operation matrix and real native/Wasm/browser mechanics with local read/edit/export enforcement | An authorized prototype reader sees the complete artifact. There is no redacted/audience artifact view or general semantic/structural editor, and no representative-user result |

The current fanout limitations are enforced in both source graph checking and
native semantic checking, not inferred from the absence of a UI button. P7's
keyed membership semantics are already accepted: duplicate keys fault, reordering
preserves identity, changed retained items revise/invalidate, removals settle,
and committed seal requires a new occurrence for later membership changes. The
missing work is complete data/operational/proof/interface evidence, not permission
to redefine those rules. Generic runtime history import is explicitly absent in
the original RP08 authoring treatment. P5 requires authentication/correlation for
its own observation acceptance; a typed wait declaration does not itself supply
that evidence or turn a foreign decision into history.

## Complete requirement inventory

The IDs below are audit references, not new IR/occurrence/type IDs. “Package” means
a required future complete witness; “contract” means exact new semantics/interface
must precede implementation; “external” means real authority/provider/user evidence.
No row records an outcome for a nonexistent RP08 package. Common validation,
privacy, cross-form, invalid-case and external obligations apply to every row.

| Audit ID | Whole-brief responsibility | Existing foundation | Missing concrete evidence / boundary |
| --- | --- | --- | --- |
| R01 | Accepted report identity, source/receipt time, known assets, confidence and handling classification | Exact typed/nominal values and protected data | Package: complete incident/report/evidence resources and revision/correlation; external source authenticity and trusted receipt basis |
| R02 | Observations, hypotheses, decisions and actions remain distinguishable and attributable | Typed variants, immutable revisions, declared provider observations | Package: separate records/links with provenance; do not collapse a recommendation/confidence/verified flag into a decision or fact |
| R03 | Authorized assessment, severity/rationale and initial coordination assignment | P1 human requirements/work, typed decision outcomes | Package: assignment/current input revision and accepted rationale; external identity/roles/current authority |
| R04 | Not-an-incident route retains rationale and resolves already-created obligations | Ordinary branches plus settlement/closure | Package: explicit false-positive graph and every unfinished obligation destination; no deletion-as-settlement |
| R05 | Parallel investigation, evidence preservation, impact and permitted mitigation | Split/join/fanout structure and conservative independence | Contract/proof: exact shared dependencies, physical effects and whole lifetime coverage; do not serialize the required parallel case and relabel it complete |
| R06 | Evidence references, custody, access history and retention remain protected | Protected types/cells and access obligations | Package plus external immutable storage/custody/access/retention evidence; a URI/digest is not proof of possession or permission |
| R07 | Commander-approved objectives and bounded containment against current facts/risk | P1/P2/P7 intersections and invalidation | Package: independently correlated approval/guard/current input; external authority and atomic dispatch cutoff |
| R08 | Dangerous proposal may be rejected, staged, simulated or approved; never dispatch merely on tool request | Recommendation as typed data, human activity and capability/work requirements | Package/contract: explicit decision/staging/refusal/effect paths; a recorded simulation result is not an implemented simulator |
| R09 | Consequential effect identity, confirmation, rollback/contingency and uncertainty remain attributable | Provider/effect/retry/compensation/owned-reconciliation contracts | Package: exact instructions/results/unknown and compensating paths; external correlation/current grant/accepted owner. No guarantee every effect is reversible |
| R10 | Scope/severity/stakeholders/objectives change with rationale | Revisioned cells and explicit decisions/invalidation | Package: revision relationships and exact dependency graph; changes cannot erase old authorization contexts |
| R11 | Verified additions create/link identified work from declared playbooks | Stable-key finite fanout and declared templates | Package/contract: item/work identity, verification boundary, complete new-work/correlation/protection and result requirements; raw distinct asset strings are not physical independence |
| R12 | Retained changed items invalidate only dependent sufficiency while preserving unaffected work/evidence | P7 invalidation and keyed identity | Contract/proof: exact change/reuse/capture/result rules across all admissible revisions and surviving work; external original accepted snapshot |
| R13 | Removed items settle active/uncertain work without erasing history | Membership removal references settlement; owned continuation classes | Package/proof: every transitive child/effect/subscription and accepted receiver; removal is not proof the work never happened |
| R14 | Seal freezes a batch; later changes use a fresh explicit occurrence | P7 seal and repeat/invocation structure | Package/contract: monotone seal/new-batch correlation and retained prior results, with no hidden reopening or output bridge |
| R15 | Authorized ad hoc tasks as well as approved fragments have attributable owners/dependencies | Declared human/capability templates and typed inputs | Contract: what definition-bounded ad hoc work can mean, how required behavior stays semantic, who may create/accept it and which effects remain supported; arbitrary runtime IR injection is absent |
| R16 | Per-asset recovery stages and monitoring coexist; no global resolved flag | Child scopes, exact local data and outcomes | Package/interface: per-key results/history/protected summaries and parent exit evidence; fanout outputs are currently closed |
| R17 | Recurrence during recovery can reopen an objective, change severity or link a new incident by explicit decision | Fresh occurrences, typed decisions and immutable terminal history | Package/contract: objective/incident lineage and accepted new work; terminal history itself does not reopen |
| R18 | Response cadence, evidence/credential lifetime, service objectives and notice pressures use distinct clocks/bases | P5 explicit pinned clock/calendar/basis contracts and timing checks | Package: selected fictional clock bases and deadlines; external trustworthy clocks/current authorized rescheduling. No jurisdictional duty or timer-created human decision |
| R19 | Shift handover changes ownership/context with acceptance | Assignment revisions and explicit settlement/transfer obligations | Package: complete old/new owner/revision/acknowledgement paths; external current authorized receiver and atomic transfer |
| R20 | Delayed/duplicate/contradictory/compromised tool evidence retains uncertainty/provenance | Exact observation identities, terminal-source rules and explicit faults/owned waits | Package/contract: exact duplicate versus conflict handling and resolution; no automatic overwrite, timestamp-winner or confidence-based truth |
| R21 | Restricted communication binds audience, approved content revision, release authority and delivery evidence | Typed inputs/provider outputs, work/capability authority and protection | Package: separately guarded communication effects and outstanding delivery/notification obligations; a delivery receipt does not close response |
| R22 | Different audiences see consistent meaning without sensitive content/label/topology/timing leakage | Runtime-data protection constraints; complete artifact authorization only | Contract/integration: audience projection/permission semantics and negative disclosure tests; no complete-document reconstruction or generic declassification |
| R23 | Outage uses an explicitly approved alternate channel | Explicit observations/authority/trust boundaries | Contract/package: channel/source identities, custody, clocks/known uncertainty and approval basis; available transport alone grants no decision authority |
| R24 | Imported records retain original identity/payload/provenance and deduplicate safely | Immutable identity/correlation principles; typed record machinery | Contract: closed foreign-record envelope, exact equality/conflict rules, bounded references and acceptance authority. No generic history import exists today |
| R25 | Known causal links remain; unrelated/tied/unknown times do not become a total order | Control causality differs from timestamp/acceptance order | Contract: partial-order validity/unknown/conflict rules; missing evidence stays unknown, even if a stable storage order is used |
| R26 | Current import acceptance is distinct from historic occurrence/decision/effect authority | P7 acceptance semantics and current actor/capability obligations | Contract/package: staged evidence acceptance and explicit reconstruction limits; no replay that silently dispatches or rewrites accepted terminal history |
| R27 | Active response closes only after exit criteria and accountable remediation/notification/retention/review destinations | Explicit outcomes, closure/settlement and finite owned continuation contracts | Package/proof: complete outcome predicate, obligation census and accepted destinations, including uncertain effects; no unchecked “closed” flag |
| R28 | Responders see objectives/owners/blocked work/scope/decisions/confidence; reviewers can reconstruct consequential actions | Complete named model and native inspector/first operation matrix | Package/view: truthful instance/evidence views plus broader authoring operations; static diagram is not live status or an authorization decision |
| R29 | Incremental scope/work may be large; limits must not erase semantic membership or pending work | Finite templates and cumulative transport/native budgets | Real complete-package measurements and bounded scale witnesses before changing limits; no current RP08 size/result or million-item performance claim |
| R30 | Both readable forms and representative authors cover the complete case | Shared native/Wasm prototype mechanics for RP01/03 and example | Full RP08 source/companion/resources/IR/check reports, complete both-form cycles/general operations, meaningful diagnostics/negatives, actual users/independent evidence |

This includes the brief's success path, alternatives, capabilities, information,
time/concurrency, invariants and observable outcomes; it is not just four happy
paths. Every result must retain its original decision/assignment/input/effect
context. A new scope revision cannot retroactively make earlier authority current.
External host facts remain separate from mathematical value/type judgments.

## Four scenarios and required future witnesses

| Scenario | Inventory | Required whole-case witness / falsifying negatives |
| --- | --- | --- |
| RP08-A | R01–R09, R16, R18–R19, R21–R22, R27–R30 | Per-service authorized parallel work, staged recovery/monitoring and explicit exit plus accepted follow-up. Negative: one unresolved effect/owner/delivery blocks the claimed complete exit; conflicting physical operations cannot pass on different labels |
| RP08-B | R02, R06, R10–R17, R20, R27–R30 | Stable-key addition/change/removal before seal, unaffected retained evidence and explicit later batch; selected authorized ad hoc work has real semantics. Negative: duplicate keys, unowned removal, changed input reusing old authority, post-seal mutation or invented parent outputs |
| RP08-C | R02–R03, R07–R09, R18–R22, R27–R30 | Untrusted recommendation persists for decision; separate current human/work/capability/guard checks and exact effect correlation. Negative: proposal, timer, stale approval, actor-like label or JSON permission cannot dispatch |
| RP08-D | R02, R06, R18–R20, R22–R26, R28–R30 | Approved alternate records retain known provenance/times/causal links and unresolved conflict; acceptance order is separate. Negative: conflicting duplicate IDs, unsupported record/profile, causal cycle/unknown link without a defined boundary, timestamp sorting as inferred order, silent replay/authority grant or history rewrite |

A useful **planned** finite witness begins with multiple asset keys, adds another
before seal, changes one retained input, removes an item with explicit settlement,
and admits a later new batch after seal. Use at least two independent alternate
sources, a duplicate, a contradictory record, tied/incomparable times and an
explicit causal link. These are test requirements, not generated observations or
published admissible fixtures. Choose exact sizes/types only in a later contract /
package scope, then measure the actual complete package under unchanged quotas.

## Alternatives

| Option | Completeness and benefit | Cost / reason to accept or reject |
| --- | --- | --- |
| A. Declare a fixed pre-expanded graph and call existing providers for everything else | Fastest package skeleton; existing native structure and inspector can expose many paths | Cannot claim dynamic work/result/history/audience semantics by calling them manual/provider work. Useful only as explicitly incomplete construction, not the full RP08 answer |
| B. Definition-bounded whole witness plus separately versioned minimal missing contracts | Retains accepted stable keys/ownership/authority and both-form model; exposes exact gaps before code; can test each new layer without a new general engine | Recommended direction. Requires real work on batch/result/lifetime/guard, ad hoc-work bounds, foreign evidence/partial order, audience views and final complete package; does not itself satisfy any missing gate |
| C. General adaptive graph/code/history engine first | Could express arbitrary runtime definitions and rich imports | Much larger typing/revision/authority/termination/protection/runtime boundary; obscures the concrete benchmark and overlaps Phase2/3. Not justified as the next implementation |
| D. Drop outage/ad hoc work or weaken privacy/concurrency, then label the subset complete | Minimizes immediate implementation | Reject: contradicts the brief/roadmap, hides requirements and manufactures completeness |

B permits no implicit meaning change to existing fanout, alias, history, work or
permission contracts. Required human instructions and behavior remain semantic,
not unchecked editorial notes or a free-form command hidden in a task record.
Reusing wording does not share work/storage/authority. The general-work requirement
must receive its own precise accepted bounds; ordinary incomplete manual work is
not an implementation of it. C remains an option only if specific requirements
cannot be met by the smaller accepted meaning, with explicit cost/evidence review.

## Ordered next dependencies

1. Publish an exact **foreign evidence/history representation and admission
   contract** first. This independently addresses R23–R26, the explicit absent
   contract, without requiring new dispatch or a Phase2 engine. Compare exact
   identity/provenance/correlation, duplicates/conflicts, partial causal order,
   timestamps/unknowns, trust/audience, current acceptance versus historic facts,
   opaque payloads, unsupported profiles and cumulative bounds. Define stages and
   fail-closed claims before implementation; validated representation is not
   authenticated truth, an accepted business decision or automatic replay.
2. Inventory and publish exact missing keyed batch/result/retention/lifetime and
   definition-bounded ad hoc-work contracts, with whole effect/guard dependencies.
   Reuse existing typed captures/owned work where valid. Preserve closed old
   interfaces, changed-input/new-work rules and real original-snapshot/physical /
   accepted-survivor prerequisites. Do not “fix” concurrency by treating strings as
   canonical physical keys or a nominal value as a recorded event.
3. Publish/enforce protected **audience artifact/evidence projections** separately
   from complete documents, runtime data protection and read/edit/export rights.
   Cover absent/forged/current authority, metadata/topology/timing disclosure,
   stale bindings and inability to infer hidden content. No redacted complete-model
   editing/export until an exact supported consistency contract exists.
4. Construct the full synthetic RP08 package, binding every inventory row/scenario
   to actual declarations/resources/tests and explicit remaining host obligations.
   Supply the entire graph, exact original source/companion/IR/pins, all stage
   outcomes/refusals and real measurements; preserve all alternatives. A bounded
   whole witness is not arbitrary unbounded scale or an actual incident trace.
5. Complete broader managed text/visual operations, stable actionable field/argument
   diagnostics, validator/conformance/negative examples and three-process cycles;
   then collect actual representative-user/independent evidence with appropriate
   trusted host/provider inputs. Independent work remains before external blockers
   can be reported. No Phase1 dependency silently moves to Phase2/3.

These dependencies can be prepared independently where their contracts do not
intersect; implementation still follows each exact signed publication. A new ADR
must resolve any incompatibility rather than mutating old C12/source19/IR4/native /
view/document contracts. This critique changes none of them and no benchmark byte.

## Gate accounting and verification

The [signed scope](0097-rp08-coverage-design-plan.md), inspected source links and
whole-case inventory are the deliverables here. No new semantic code, history
schema/importer, benchmark producer, native RP08 result or user study is supplied.
No prose-mirroring test is needed; check local links/SPDX and unchanged accepted /
benchmark bytes, then run required final-head CI and separate automated review /
complete comment audit before expected-head signed integration.

The last verified implementation baseline is PR111:704 Rust + eight compile-fail,
six Wasm lints/three release builds and92 Python commands, all passing at its exact
head. That baseline is not a new test run at this critique's future PR head.
Full RP08 and all40 scenario statuses remain Partial, G1–G4 conditional; the six
open Phase1 headline gates and owner phase-report approval remain. A metadata /
representation contract cannot confer authority, liveness, accepted settlement,
provider truth, executable admission or phase completion.
