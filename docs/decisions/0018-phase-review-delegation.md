<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0018: Delegate implementation review within approved phases

**Status:** Accepted — owner-authorized workflow, effective upon merge<br>
**Date:** 2026-11-05<br>
**Decider:** Project Owner

## Context

The original workflow pauses at every Roadmap deliverable for owner review,
approval and permission to start the next item. On 2026-11-05 the Project Owner
approved PR #22 and explicitly delegated the regular implementation, PR review
and merge flow to Codex. The owner requested a general achievement report at the
end of each phase, approval of that report before continuing, and a new chat for
the next phase. This record changes that review cadence without changing the
Manifest, licenses, technical contracts or evidence requirements.

## Decision criteria

1. Carry out the owner's standing authorization without repeated item-level pauses.
2. Retain inspectable decisions, reviews, attribution and required verification.
3. Keep phase boundaries under owner control and distinguish delegation from
   independent human approval or empirical evidence.

## Decision

Authorize Codex, acting as the owner's automated project agent, to select and
implement work within the current Roadmap phase, evaluate alternatives, propose
and accept scoped ADRs, create PRs, review them and merge them after verification.
The agent may author and review its own changes; record this honestly as automated
self-review under owner delegation, never as independent human review. This is a
delegation of project work, not an appointment of a human maintainer or a change
in ownership. The owner retains authority to steer, narrow or revoke it.

Keep one coherent deliverable per branch/PR, the existing ADR lifecycle, DCO
sign-offs and licensing rules. Review the completed diff separately from writing
it, assess correctness/security/performance/contract boundaries, evaluate every
unresolved human and CodeRabbit finding (including summary comments), fix valid
issues and explain rejected findings. Run proportionate tests and all required
CI at the final head before merge; verify the expected head at merge. Do not
manufacture a GitHub self-approval or treat an unavailable bot as a human review.

Record future acceptance as "accepted under the Project Owner's delegation of
2026-11-05," with the review evidence and PR, rather than claiming personal owner
review of each proposal. No per-item permission or owner approval pause is needed
within the phase. Material contradictions or genuinely unavailable external
evidence must be reported honestly after all independent work can proceed.

At the end of each phase, provide a report covering delivered behavior, decisions,
verification, remaining limits and the explicit exit criteria. Stop for owner
approval. After approval, open a new phase chat and continue the same procedure.
A missing gate is not phase completion: do not fabricate participant studies,
external reviews, runtime/browser observations or benchmark results to reach a
boundary. Preserve the qualified legal-review gate before proprietary work.

Maintain a repository progress record and agent instructions so interrupted chats
can resume from verified commits, PR state and open obligations.

## Options considered

| Option | Advantages | Costs and risks | Outcome |
| --- | --- | --- | --- |
| Delegate within phases; owner approves phase report | Matches owner's request; keeps review history and phase oversight | Self-review lacks independent human scrutiny; evidence and limitations must be explicit | Adopt |
| Pause for every PR/ADR | Frequent owner involvement | Conflicts with the owner's new instruction and interrupts routine work | Superseded cadence |
| Bypass PRs/reviews or close gates automatically | Fewer steps | Loses inspectability and can misstate evidence | Reject |

## Consequences

Implementation can continue across deliverables without waiting for item-level
approval. Public PRs and ADRs still explain why changes were accepted. Automated
self-review cannot supply independent security review, participant observations
or other external evidence required by the Roadmap. Such gaps remain visible and
can block phase completion. Progress messages may describe ongoing work; they do
not become new approval gates.

## Confirmation

Update Governance, Contributing, the ADR guide, Roadmap and root agent instructions
to use the same delegated workflow. Record PR verification/review decisions and
the final head on each merge. Maintain the Phase 1 progress record; stop at the
phase report rather than at the next implementation PR.

## Acceptance and action items

The Project Owner explicitly requested this workflow on 2026-11-05 when approving
PR #22. This ADR records that authorization; it does not infer approval from
silence. Historical item-level pauses remain historical records.

Automated self-review on 2026-11-05 checked the changed governance/contribution/
ADR/Roadmap instructions for consistent authority, preserved phase gates and
honest review attribution. No remaining actionable finding was identified. Local
link checks and existing authoring/control evidence checks passed; final-head
required CI remains a pre-merge check on the workflow PR.

1. [x] Record the owner's standing authorization and phase boundary.
2. [x] Align current contributor/agent workflow and durable progress tracking.
3. [ ] Keep item reviews/verification and phase reports throughout execution.
