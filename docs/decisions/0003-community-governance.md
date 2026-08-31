<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0003: Establish lightweight community governance and contribution policy

**Status:** Accepted<br>
**Date:** 2026-08-31<br>
**Decider:** Project owner

**Later decision:** [ADR-0004](0004-lightweight-adr-process.md) standardizes the
ADR process anticipated by this record.

## Context

Choreoform needs an understandable way for people to participate before design
and implementation work expands. The policy must match the project's current
reality—a single Project Owner and no appointed maintainers—without leaving
authority implicit or prematurely constructing a foundation-scale voting
system.

The Manifest also creates a structural governance risk: the same product family
may include proprietary process bundles. Commercial participation should be
possible, but private bundle ownership must not create privileged influence,
interfaces, or dependencies in the free-software foundation.

The contribution policy must preserve ADR-0002's inbound-equals-outbound
licensing model, establish contribution provenance, provide safe reporting
channels, and define expectations for collaboration.

## Decision criteria

The policy should:

1. describe who can decide and merge work today;
2. preserve the agreed branch, pull-request, and owner-approval workflow for
   Roadmap deliverables;
3. invite participation without requiring copyright assignment or a broad CLA;
4. provide recognizable conduct and contribution-provenance standards;
5. support confidential security and conduct reports;
6. make commercial conflicts and the open/proprietary boundary explicit;
7. allow authority to be delegated as the contributor community grows; and
8. remain simple enough to operate during Phase 0.

## Decision

Adopt the following community policy set:

- `GOVERNANCE.md`: a consensus-seeking Project Owner model with scoped,
  delegated maintainers;
- `CONTRIBUTING.md`: the development workflow, review expectations, licensing
  rules, and contributor entry points;
- `DCO.txt`: the unmodified Developer Certificate of Origin 1.1, certified by a
  `Signed-off-by` trailer on every commit;
- `CODE_OF_CONDUCT.md`: an adapted Contributor Covenant 3.0 with a private
  reporting channel and a recusal path;
- `SECURITY.md`: private vulnerability reporting, coordinated-disclosure
  expectations, and an explicit statement that no release is supported yet;
  and
- GitHub pull-request and issue templates that put these expectations at the
  point of contribution.

No contributor license agreement or copyright assignment is required. The DCO
attests provenance and authority to contribute; it does not change the
inbound-equals-outbound licenses selected in ADR-0002.

The Project Owner initially holds final authority and serves as Community
Moderator. Authority may be delegated publicly to maintainers with defined
scope. Roadmap deliverables require a dedicated branch, pull request, and
recorded Project Owner approval before merge.

Durable decisions require a public ADR, but the mechanics for ADR lifecycle and
supersession remain the next, separate Phase 0 deliverable.

## Open and commercial governance boundary

Maintainer authority is earned through relevant public stewardship. It is not
granted by employment, payment, commercial partnership, or access to
proprietary bundle repositories.

Decision makers disclose relevant commercial or personal conflicts and recuse
when another authorized reviewer is available. Proprietary bundles receive no
private foundation APIs, exemptions, or closed build/test dependencies.

This policy cannot eliminate the Project Owner's potential conflict while the
project has only one authorized decision maker. It therefore requires public
disclosure and rationale when recusal is impossible, and creates a path to
delegate authority as trusted maintainers emerge.

## Reporting channels

Security vulnerabilities use GitHub private vulnerability reporting, with the
Project Owner's publicly listed email as fallback. Code of Conduct concerns use
that email directly. If a conduct report involves the sole moderator, an
independent reviewer acceptable to the reporter controls the findings.

The seven-day acknowledgement targets in both policies are operational goals,
not guaranteed remediation deadlines or service-level agreements.

## Licensing of adopted texts

Contributor Covenant 3.0 is adapted under CC BY-SA 4.0 and is identified as
third-party material in its SPDX notice and attribution. This is a documented
exception to the repository's CC-BY-4.0 default for project-authored prose.

The DCO is reproduced verbatim under its own permission to copy and distribute
unchanged copies. It has no project SPDX header because adding one would alter
the canonical text; its provenance and treatment are recorded here and in
`CONTRIBUTING.md`.

## Options considered

| Option | Transparency | Works with one owner | Scales by delegation | Administrative cost | Outcome |
| --- | --- | --- | --- | --- | --- |
| **Project Owner with delegated maintainers** | High when decisions are recorded | Yes | Yes | Low | Adopt |
| Informal owner discretion | Low | Yes | Poorly | Lowest | Reject: authority and escalation remain unclear |
| Maintainer voting council | High | No council exists | Yes | Medium | Defer until a real maintainer community exists |
| Membership foundation | High | Disproportionate | Yes | High | Defer until stewardship requires a legal institution |

For contribution provenance, a DCO was selected over a CLA because the project
does not need copyright assignment or special relicensing rights. Requiring no
attestation would be simpler but would leave contributor provenance implicit.

Contributor Covenant was selected over a custom code of conduct because it is a
maintained, recognizable baseline with reporting and enforcement guidance. The
project-specific adaptation adds concrete channels, response targets, and
recusal handling.

## Consequences

- Contributors can see how to participate, license, sign off, and escalate
  work.
- Final authority is explicit rather than hidden, and can be delegated without
  replacing the model immediately.
- Every commit requires a public identity sign-off that is retained
  indefinitely in repository history.
- Conduct and vulnerability reports have private channels, creating a duty to
  monitor and respond to them.
- A solo Project Owner remains a concentration of authority and a continuity
  risk until trusted maintainers are appointed.
- The Code of Conduct adds CC-BY-SA-4.0 third-party material alongside the
  project's two default licenses.
- Automated DCO and policy checks may be added when CI is introduced; manual
  review applies initially.

## Sources

- [Contributor Covenant 3.0](https://www.contributor-covenant.org/version/3/0/)
- [Developer Certificate of Origin 1.1](https://developercertificate.org/)
- [GitHub private vulnerability reporting](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/report-privately)
- [GitHub coordinated vulnerability disclosure](https://docs.github.com/en/code-security/concepts/vulnerability-reporting-and-management/coordinated-disclosure)

## Acceptance and action items

The Project Owner approved this ADR on 2026-08-31. The policy becomes effective
when pull request #4 is merged.

1. [x] Obtain Project Owner approval.
2. [x] Change this ADR's status to Accepted and record the approval.
3. [x] Enable GitHub private vulnerability reporting and confirm the reporting
   link works.
4. [ ] Monitor the published security and conduct channels.
5. [ ] Add automated DCO and policy checks when continuous integration is
   introduced.
6. [ ] Revisit the governance model when at least three active maintainers could
   sustain shared decision-making, or sooner if continuity requires it.
