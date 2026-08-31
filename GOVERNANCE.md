<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Choreoform governance

**Model:** Project Owner with delegated maintainers<br>
**Project Owner:** [Uriel Fiori (`@cyborg-nomade`)](https://github.com/cyborg-nomade)

Choreoform is developed in public as a free-software foundation. Its governance
is intentionally lightweight while the community is small, but authority and
the relationship to proprietary bundles are explicit.

## Principles

- Seek reasoned consensus before exercising formal authority.
- Discuss product and technical direction in public unless security, privacy,
  or Code of Conduct confidentiality requires otherwise.
- Record durable decisions and their trade-offs so they can be revisited.
- Give open-foundation users and contributors the same public interfaces and
  information available to proprietary Choreoform bundles.
- Base influence on relevant work, judgment, and sustained stewardship—not
  employment, commercial status, or access to private bundle repositories.
- Preserve contributor attribution and use the least authority needed to
  resolve a problem.

## Roles

### Participants

Anyone who uses Choreoform or takes part in its community spaces. Participants
must follow the Code of Conduct.

### Contributors

Participants whose issue, review, design, documentation, code, or other work is
accepted by the project. Contributors do not gain merge or decision authority
automatically, but their relevant expertise and affected interests should be
weighed in decisions.

### Maintainers

Contributors trusted by the Project Owner to review or merge work in a defined
area. Maintainers are expected to:

- uphold the Manifest, accepted decisions, licenses, and Code of Conduct;
- review impartially and explain material decisions;
- keep the open foundation buildable and usable without proprietary assets;
- disclose conflicts of interest; and
- recuse themselves when they cannot decide fairly.

The Project Owner appoints and removes maintainers in a public repository
record, stating their scope. Removal should follow prior notice and an
opportunity to respond unless immediate action is needed for safety or
security.

### Project Owner

The Project Owner is the final steward of project scope, Roadmap priorities,
releases, maintainer appointments, and repository administration. The Project
Owner may delegate any routine authority but remains accountable for its use.

The current Project Owner is Uriel Fiori (`@cyborg-nomade`). A successor must be
named in a public governance change. If the Project Owner becomes unavailable
without naming one, active maintainers may unanimously designate an interim
owner; if unanimity is impossible, the active maintainer with the longest
tenure serves until a public succession decision is made.

## How decisions are made

### Routine changes

Maintainers decide ordinary issues and pull requests within their scope after
allowing reasonable time for relevant review. Silence is not automatically
consent. A maintainer should escalate a change when it creates a compatibility
promise, changes project-wide policy, or has unresolved, material objections.

### Roadmap deliverables

Each Roadmap deliverable has a dedicated branch and pull request. It is merged
only after the Project Owner records approval. Any accepted decision record in
the pull request is updated to reflect that approval before merge.

### Durable or cross-cutting decisions

Changes to language semantics, public compatibility, architecture, licensing,
governance, or the open/commercial boundary require a public architecture
decision record and Project Owner approval. The
[ADR process](docs/decisions/README.md) defines proposal, discussion,
acceptance, and supersession.

### Resolving disagreement

The decision maker should first summarize the strongest competing positions,
the evidence considered, and the consequences of the decision. If consensus is
not reached, the responsible maintainer may decide routine matters and the
Project Owner may decide escalated matters. The rationale must be recorded in
the issue, pull request, or decision record.

Anyone may ask the Project Owner to reconsider a maintainer decision by adding
new evidence or showing a conflict with accepted project policy. Repetition
without new information does not require reopening a decision.

Security response and Code of Conduct enforcement follow their confidential
processes rather than public design-decision rules.

## Commercial interests and conflicts

Commercial participation is welcome, including work that supports proprietary
bundles. A person has a conflict of interest when their personal, employment,
financial, or private-repository interests could reasonably appear to impair an
open-foundation decision.

A conflicted decision maker must disclose the conflict in the relevant public
discussion and recuse themselves when another authorized reviewer is
available. When the Project Owner is conflicted and no other maintainer can
decide, the Project Owner may act only after documenting the conflict, the
public-foundation benefit, and why delay or delegation is impractical.

No proprietary bundle may receive an undocumented private API, waived
compatibility requirement, advance access unavailable on equal terms, or a
dependency on closed assets in the open project's build, test, or operation.

## Community conduct

The Project Owner initially serves as Community Moderator. Code of Conduct
reports are handled under [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md), separately
from technical authority. If a report involves the sole moderator, that
moderator must recuse themselves and appoint an independent reviewer acceptable
to the reporter. The reporter may withhold incident details until that reviewer
is appointed; enforcement findings belong to the reviewer.

## Amending governance

Material changes to this document require a dedicated pull request, a decision
record, public review, and Project Owner approval. Editorial corrections that
do not change authority or rights may use the routine contribution process.

The repository's free-software licenses remain effective regardless of a
governance change. Contributors retain the right to fork the project under
those licenses.
