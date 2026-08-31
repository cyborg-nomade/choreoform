<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Contributing to Choreoform

Thank you for helping build Choreoform's free-software foundation. Contributions
of use cases, design critique, documentation, tests, code, accessibility
feedback, security research, and process examples are all welcome.

## Before participating

Participation is governed by the [Code of Conduct](CODE_OF_CONDUCT.md). Report
security vulnerabilities through the private channels in
[SECURITY.md](SECURITY.md), not through a public issue.

Read the [Manifest](MANIFEST.md) and [Roadmap](ROADMAP.md) before proposing a
substantial change. The project is still in its groundwork phase, so opening an
issue before investing in a large implementation can avoid conflicting designs
or premature compatibility commitments.

Use the [working glossary](GLOSSARY.md) when describing processes and design
proposals. If a core term needs a different conceptual boundary, call out the
change explicitly and follow the ADR process.

## Ways to contribute

- Describe a representative process and the hard cases it exposes.
- Improve terminology, examples, specifications, or accessibility.
- Report a reproducible bug or propose a narrowly stated capability.
- Review a design or pull request against the Manifest's principles.
- Implement an agreed Roadmap item, test, tool, or documentation improvement.

Use the repository's issue forms where possible. Questions and small editorial
fixes do not need an issue first.

## Development workflow

The default branch is `main`. Each Roadmap deliverable is developed on a
dedicated branch and reviewed in its own pull request. The Project Owner must
approve a Roadmap deliverable before it is merged.

For other changes:

1. Fork or branch from an up-to-date `main`.
2. Keep the branch focused on one coherent change.
3. Add or update tests and documentation appropriate to the change.
4. Add the correct SPDX notice to every new file, following
   [LICENSE.md](LICENSE.md).
5. Sign off every commit under the Developer Certificate of Origin.
6. Open a pull request and respond to review without rewriting another
   contributor's work without their consent.

Until component-specific build and test commands exist, a pull request should
describe how its claims were checked. The repository will document required
commands as implementation tooling is introduced.

## Developer Certificate of Origin

Choreoform uses the
[Developer Certificate of Origin 1.1](DCO.txt) (DCO), not a contributor license
agreement or copyright assignment. By adding a `Signed-off-by` trailer, you
certify that you have the right to submit the contribution under the licenses
indicated in the affected files.

Sign a commit with:

```shell
git commit -s
```

The trailer must use a name and email address by which you can be identified:

```text
Signed-off-by: Your Name <your.email@example.com>
```

If a commit is missing the trailer, amend it locally or add a follow-up commit
that explicitly signs off the identified commit. Do not sign off another
person's contribution unless DCO clause (c) applies and you preserve its
provenance.

## Licensing contributions

Contributions are made under the existing license of each file changed. New
files use the repository policy for their artifact type: generally MPL-2.0 for
software and code-like material, and CC-BY-4.0 for project-authored prose and
documentation. Third-party material retains its upstream license and must be
clearly identified.

The project uses an inbound-equals-outbound model. It does not require a CLA or
copyright assignment. See [LICENSE.md](LICENSE.md) and
[ADR-0002](docs/decisions/0002-licensing-policy.md) for the complete boundary.

## Pull-request expectations

A reviewable pull request:

- explains the problem and why the change belongs in Choreoform;
- links its issue, Roadmap item, or decision record when one exists;
- states user-visible, semantic, compatibility, licensing, and security
  effects;
- includes proportionate verification and reports what was not tested;
- avoids unrelated formatting or refactoring; and
- leaves the branch mergeable without requiring access to proprietary bundles.

Maintainers may request changes, split an oversized proposal, or close work
that conflicts with an accepted decision. A declined contribution can still be
used under its own terms until it has been incorporated into the repository.

## Decisions and governance

Routine changes are resolved through review. Decisions that establish durable
project direction follow the
[architecture decision record process](docs/decisions/README.md). Use its
[template](docs/decisions/template.md) and seek approval from the decider named
under Governance.

Roles, authority, conflicts of interest, and escalation are defined in
[GOVERNANCE.md](GOVERNANCE.md).
