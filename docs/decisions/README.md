<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Architecture decision records

Architecture decision records (ADRs) explain why Choreoform made durable,
consequential choices. They preserve the context, alternatives, decision, and
consequences for future contributors. In Choreoform, “architecture” includes
institutionally significant decisions such as licensing and governance, not
only software structure.

ADRs complement current documentation: an ADR explains why a choice was made,
while specifications, guides, and code describe the system as it exists now.

## Decision index

| ADR | Status | Decision |
| --- | --- | --- |
| [0001](0001-project-and-language-name.md) | Accepted | Name the project and language Choreoform |
| [0002](0002-licensing-policy.md) | Accepted | License the open foundation without capturing user processes |
| [0003](0003-community-governance.md) | Accepted | Establish lightweight community governance and contribution policy |
| [0004](0004-lightweight-adr-process.md) | Accepted | Adopt a lightweight, repository-native ADR process |
| [0005](0005-pre-semantic-working-glossary.md) | Accepted | Establish a pre-semantic working glossary |
| [0006](0006-representative-process-corpus.md) | Accepted | Adopt a representative process corpus |
| [0007](0007-design-evaluation-framework.md) | Accepted | Evaluate semantic and notation designs with evidence profiles |
| [0008](0008-core-process-semantics.md) | Accepted | Define the core process semantics as explicit state transitions |
| [0009](0009-canonical-versioned-ir.md) | Accepted | Specify a typed graph IR with explicit version and identity boundaries |
| [0010](0010-initial-implementation-language.md) | Accepted | Use Rust for the initial shared semantic implementation |
| [0011](0011-initial-textual-grammar.md) | Accepted | Prototype an explicit declaration-oriented textual grammar |
| [0012](0012-binding-type-foundations.md) | Accepted | Bind names and typed interfaces explicitly across closed composition boundaries |
| [0013](0013-executable-dialect-contracts.md) | Accepted | Specify a closed initial type, expression and policy profile |
| [0014](0014-near-english-authoring.md) | Accepted | Develop structured English sentences as a provisional authoring direction |
| [0015](0015-authoring-grammar-bindings.md) | Accepted | Specify Candidate A grammar and portable identity bindings |
| [0016](0016-static-contract-checks.md) | Accepted | Separate static contract checks from executable admission |
| [0017](0017-control-availability-checks.md) | Accepted | Check ordinary control and initialization conservatively |
| [0018](0018-phase-review-delegation.md) | Accepted | Delegate implementation review within approved phases |
| [0019](0019-protected-ordinary-transfers.md) | Accepted | Track ordinary control protection and access obligations |
| [0020](0020-lifecycle-policy-links.md) | Accepted under delegation | Validate lifecycle policies at their definition use sites |
| [0021](0021-ordinary-invalidation.md) | Accepted under delegation | Check conservative ordinary invalidation declarations |
| [0022](0022-ordinary-concurrency.md) | Accepted under delegation | Check conservative ordinary parallel independence |
| [0023](0023-clock-resource-contract.md) | Accepted under delegation | Define a pinned instance-clock resource and host boundary |
| [0024](0024-timer-contract-checks.md) | Accepted under delegation | Type supported timers and expose their host obligations |
| [0025](0025-instance-calendars.md) | Accepted under delegation | Define instance calendar construction and activation obligations |
| [0026](0026-explicit-retry-clock.md) | Accepted under delegation | Bind retry timing explicitly in a versioned core profile |
| [0027](0027-explicit-retry-authoring.md) | Accepted under delegation | Express retry clock bindings in a versioned English sentence |
| [0028](0028-conservative-fault-routing.md) | Accepted under delegation | Check a conservative active-context handler-selection graph |
| [0029](0029-conditional-handler-availability.md) | Accepted under delegation | Check handler-body availability under explicit input contracts |
| [0030](0030-reconciliation-declaration-links.md) | Accepted under delegation | Require an owned reachable reconciliation declaration route |
| [0031](0031-qualified-wait-endpoints.md) | Accepted under delegation | Bind explicitly qualified reconciliation wait endpoints |
| [0032](0032-conditional-reconciliation-availability.md) | Accepted under delegation | Check surviving follow-up bodies without inherited values |
| [0033](0033-semantic-source-locations.md) | Accepted under delegation | Locate semantic diagnostics in original source declarations |

| [0034](0034-benchmark-authoring-profile.md) | Accepted under delegation | Bound full-benchmark source and bind actual-use wait endpoints |

| [0035](0035-provider-metadata-read-closure.md) | Accepted under delegation | Derive finite read closure through provider recipient metadata |

| [0036](0036-metadata-core-authoring.md) | Accepted under delegation | Select finite metadata closure through an explicit source/core pair |

| [0037](0037-closed-ordinary-provenance.md) | Accepted under delegation | Separate proved historical origins from live invalidation inputs |

| [0038](0038-invalidation-requirements.md) | Accepted under delegation | Derive advisory ordinary requirements without bypassing strict coverage |

| [0039](0039-isolated-surviving-waits.md) | Accepted under delegation | Check declared isolation of synchronous surviving waits under live P6 premises |

| [0040](0040-finite-guard-snapshot-conformance.md) | Accepted under delegation | Check actual guard ASTs with finite synthetic snapshots, without execution claims |

| [0041](0041-provider-fault-source-metadata.md) | Accepted under delegation | Pin declared provider observation-to-fault evidence without implicit retry authority |

| [0042](0042-explicit-retry-fault-source-binding.md) | Accepted under delegation | Bind eligible provider fault sources explicitly in core 0.4 without live permission claims |
| [0043](0043-retry-fault-source-authoring.md) | Accepted under delegation | Explicit source 0.7 retry fault-source clauses and bounded declaration disclosure |
| [0044](0044-provider-terminal-observation-metadata.md) | Accepted under delegation | Exact declared terminal provider observations, normalized types and protection without wait binding |
| [0045](0045-explicit-timerless-wait-capture.md) | Accepted under delegation | Versioned IR-0.2/core-0.5 timerless source/origin/result capture with complete static protection and availability boundaries |
| [0046](0046-wait-capture-authoring.md) | Accepted under delegation | Explicit source 0.8 wait-capture authoring and bounded declaration disclosure |
| [0047](0047-owned-local-followups.md) | Accepted under delegation | Conditional isolated fresh-local storage with retained source/control protection provenance |
| [0048](0048-general-provider-fault-source-binding.md) | Accepted under delegation | General exact provider emission-source declarations independent of retry eligibility |
| [0049](0049-general-fault-source-authoring.md) | Accepted under delegation | Readable source 0.9 general fault-source clauses and shared bounded native catalogues |
| [0050](0050-provider-fault-history-metadata.md) | Accepted under delegation; pending parent/main integration | Separate provider history claims from authenticated handler entry |
| [0051](0051-declared-fault-origin-contexts.md) | Accepted under delegation; pending parent/main integration | Retain declared emitting sites and complete handler contexts |
| [0052](0052-typed-handler-entry.md) | Accepted under delegation; pending parent/main integration | Bind immutable history to explicit conditional handler-local entry |
| [0053](0053-handler-entry-authoring.md) | Accepted under delegation; pending parent/main integration | Integrate readable history entry with separate conditional reports |
| [0054](0054-owned-request-retention.md) | Accepted under delegation; pending parent/main integration | Retain exact immutable original requests in owned follow-ups |
| [0055](0055-request-retention-authoring.md) | Accepted under delegation; pending parent/main integration | Author original-request retention with an explicit readable clause |
| [0056](0056-owned-continuation-direction.md) | Accepted under delegation; pending parent/main integration | Extend owned work through explicit finite surviving ownership |
| [0057](0057-owned-readonly-continuations.md) | Accepted under delegation; pending parent/main integration | Check finite read-only surviving continuations separately |
| [0058](0058-readonly-continuation-authoring.md) | Accepted under delegation; pending parent/main integration | Select and disclose finite continuations explicitly in readable source |
| [0059](0059-owned-entry-command-direction.md) | Accepted under delegation; pending parent/main integration | Enable useful owned read-only entry before broad command work |
| [0060](0060-owned-pre-capture-entry.md) | Accepted under delegation; pending parent/main integration | Check finite read-only owned entry before source capture |
| [0061](0061-pre-capture-authoring.md) | Accepted under delegation; pending parent/main integration | Select and disclose pre-capture entry explicitly in readable source |
| [0062](0062-owned-command-direction.md) | Accepted under delegation; pending parent/main integration | Separate finite owned work from read-only query semantics |
| [0063](0063-finite-owned-work.md) | Accepted under delegation; pending parent/main integration | Check finite owned work and pure local input construction |
| [0064](0064-finite-work-authoring.md) | Accepted under delegation; pending parent/main integration | Select and disclose finite owned work in readable source |
| [0065](0065-rp01-current-contract-direction.md) | Accepted under delegation; pending parent/main integration | Adopt demonstrated RP-01 capture before further generalization |
| [0066](0066-rp01-capture-adoption.md) | Accepted under delegation; pending parent/main integration | Publish the full RP-01 typed late-capture checkpoint |
| [0067](0067-rp03-package-draft.md) | Accepted under delegation; pending parent/main integration | Preserve the whole RP-03 draft and its actual capacity refusal |
| [0068](0068-source-capacity.md) | Accepted under delegation; pending parent/main integration | Version measured source capacity without wider semantics |
| [0069](0069-compensation-reference-direction.md) | Accepted under delegation; pending parent/main integration | Version compensation identity separately from lexical access |
| [0070](0070-compensation-identity-contract.md) | Accepted under delegation; pending parent/main integration | Publish purpose-only compensation activity identity |
| [0071](0071-compensation-identity-authoring.md) | Accepted under delegation; pending parent/main integration | Bind exact readable compensation activity identities |
| [0072](0072-rp03-symbol-binding-direction.md) | Accepted under delegation; pending parent/main integration | Correct the RP-03 finish caption, preserving exact symbol semantics |
| [0073](0073-rp03-corrected-checkpoint.md) | Accepted under delegation; pending parent/main integration | Publish a corrected current-profile RP-03 draft with exact history |
| [0074](0074-rp03-template-instantiation-direction.md) | Accepted under delegation; pending parent/main integration | Instantiate service bodies in their actual caller context |
| [0075](0075-effectful-template-instantiation.md) | Accepted under delegation; pending parent/main integration | Keep effectful instances and their identities in their actual context |
| [0076](0076-rp03-instance-adoption.md) | Accepted under delegation; pending parent/main integration | Instantiate every RP-03 service at its actual caller |
| [0077](0077-fixed-all-join-history.md) | Accepted under delegation; pending parent/main integration | Prove direct fixed all-join history and exact invalidation coverage |
| [0078](0078-external-interference-contract-direction.md) | Accepted under delegation; pending parent/main integration | Require an explicit external-interference contract before extending acceptance |
| [0079](0079-external-interference-draft.md) | Accepted under delegation; inactive draft design only | Model finite exact-provider external regions without execution admission |
| [0080](0080-external-interference-resources.md) | Accepted under delegation; pending parent/main integration | Validate standalone provider-region metadata without selecting it |
| [0081](0081-interference-definition-binding.md) | Accepted under delegation; pending parent/main integration | Select exact provider descriptors explicitly without proving concurrency |
| [0082](0082-request-region-proof-direction.md) | Accepted under delegation; pending parent/main integration | Preserve unproved region safety and progress independent near-English factoring |
| [0083](0083-near-english-factoring-direction.md) | Accepted under delegation; direction only, pending parent/main integration | Factor explicit data protection before scoped work reuse |
| [0084](0084-named-data-protection.md) | Accepted under delegation; pending parent/main integration | Explicit named data protection with unchanged expanded semantics |
| [0085](0085-transparent-type-naming-direction.md) | Accepted under delegation; direction only, pending integration | Prefer explicit transparent type names before parameterized reuse |
| [0086](0086-local-type-names.md) | Accepted under delegation; pending integration | Explicit local type descriptions with bounded transparent expansion |
| [0087](0087-rp03-factored-authoring.md) | Accepted under delegation; pending integration | Adopt factored RP-03 source with an exact pre-adoption archive |

## When an ADR is required

Write an ADR before committing the project to a decision that is expensive to
reverse, constrains later work, or needs durable rationale. Examples include:

- language semantics, syntax foundations, compatibility, or versioning;
- canonical formats, public interfaces, extension boundaries, or execution
  guarantees;
- component boundaries, foundational dependencies, deployment models, or
  security architecture;
- project-wide privacy, reliability, accessibility, or release policy; and
- licensing, governance, or the open/commercial boundary.

An ADR is normally unnecessary for routine implementation, local refactoring,
editorial changes, temporary experiments, or choices that are both inexpensive
and easy to replace. When uncertain, ask in the relevant issue or pull request.
A short ADR is cheaper than repeatedly reconstructing a consequential choice.

Never put secrets, personal data, embargoed vulnerability details, or
confidential Code of Conduct information in an ADR.

## Creating a record

1. Search this index, open issues, and pull requests for an existing decision.
2. Keep the record to one decision. Split independently reversible choices.
3. Copy [the template](template.md).
4. Assign the next four-digit number and a concise kebab-case filename:
   `NNNN-short-decision-title.md`.
5. Set the status to `Proposed`, use the proposal date, and name the decider by
   governance role.
6. Explain the forces and decision criteria before evaluating realistic
   alternatives. Include material disadvantages and unresolved risks.
7. Open a pull request and link relevant issues, prototypes, evidence, and
   Roadmap items.

Numbers increase monotonically and are never reused after a proposal pull
request is opened; gaps are acceptable. If concurrent proposals select the
same number, the later one must be renumbered before merge. A number conveys
identity, not priority.

The ADR and implementation may share a pull request when the implementation is
small, reviewable, and safe to discard. Expensive or difficult-to-reverse work
should wait for a separate decision pull request to be accepted. Prototypes are
welcome as evidence when they do not create a compatibility commitment.

## Review and decision

The authority to decide follows [GOVERNANCE.md](../../GOVERNANCE.md). Anyone may
propose or review an ADR. The author should actively seek input from affected
contributors and summarize material objections; silence is not consent.

There is no fixed review period. The decider allows time proportionate to the
decision’s reach, reversibility, urgency, and participation. The Project Owner
must approve Roadmap deliverables and the durable or cross-cutting decisions
identified by Governance, individually or through an explicit recorded delegation.
[ADR-0018](0018-phase-review-delegation.md) records the current phase workflow:
Codex reviews and accepts scoped decisions under owner delegation, with owner
approval of each phase report before continuing into the next phase. Record this
as automated self-review under delegation, not personal owner or independent
human review of every future proposal.

When the decider approves a proposal:

1. record the approval in the pull request;
2. change the status to `Accepted` and record the approval in the ADR;
3. update the index and any Roadmap item in the same branch;
4. verify that immediate action items are complete or explicitly deferred; and
5. merge the pull request, which makes the decision effective.

A proposal that is not selected may be closed without merge. Mark it `Rejected`
and merge it only when preserving the evaluated alternative and rejection
rationale will help future decisions.

## Statuses

| Status | Meaning |
| --- | --- |
| `Proposed` | Under review and not binding. |
| `Accepted` | Approved and effective once merged. |
| `Rejected` | Considered and explicitly not selected. |
| `Deprecated` | Retained for history but no longer recommended; no single replacement necessarily exists. |
| `Superseded` | Replaced in whole or in part by a linked later ADR. |

## Changing a decision

Accepted ADRs are historical records, not living specifications. Do not rewrite
their original context, choice, or consequences to match hindsight. Minor
corrections, clearer links, action-item updates, and status metadata may be
edited without changing the decision’s meaning.

To reverse or materially change an accepted decision:

1. create a new ADR describing the changed context and prior decision;
2. follow the normal review and approval process;
3. after acceptance, mark the old ADR `Superseded` and link both records; and
4. update current specifications and guides separately.

Use `Deprecated` when a decision is being retired without one clear
replacement. A superseding ADR need not copy the entire old record.

## Urgent and confidential decisions

Urgent safety or security work may be performed before a public ADR when delay
would increase harm. If the result creates a durable decision, document it as
soon as disclosure is safe, omitting protected details. Operational incident
handling and Code of Conduct enforcement remain governed by their confidential
processes.

## Writing guidance

- Write for a future contributor who did not witness the discussion.
- State observable constraints and distinguish evidence from assumptions.
- Use active, decisive language in the Decision section.
- Present the strongest realistic alternatives fairly.
- Record positive, negative, and neutral consequences.
- Link sources instead of copying transient discussion into the record.
- Prefer a short, complete record; add detail only when it preserves important
  evidence or trade-offs.

This process draws on Michael Nygard’s original lightweight ADR format and the
Markdown Architectural Decision Records (MADR) project, while retaining the
house style established by Choreoform’s first records.

## Sources

- [Michael Nygard, “Documenting Architecture Decisions”](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions)
- [Markdown Architectural Decision Records](https://adr.github.io/madr/)

- [ADR-0088: RP-01 factored authoring and explicit C12 promotion](0088-rp01-factored-authoring.md) — accepted under owner delegation; effective on integration.

- [ADR-0089: Typed pure receipt recipe direction](0089-reusable-receipt-direction.md) — accepted under owner delegation; no production syntax yet.

- [ADR-0090: Typed source-only pure Boolean tests](0090-pure-predicate-recipes.md) — contract accepted under owner delegation; native implementation and experimental whole variants integrated in PR #105.

- [ADR-0091: Current RP-03 pure-test adoption](0091-rp03-pure-test-adoption.md) — accepted under owner delegation; signed archive precedes implementation; final checks and integration pending.
