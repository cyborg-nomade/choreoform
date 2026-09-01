<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Choreoform

Choreoform is a family of tools for describing, executing, and editing
information processes in equivalent textual and visual forms. The name and its
rationale are recorded in
[ADR-0001](docs/decisions/0001-project-and-language-name.md).

The groundwork decisions are complete, and Phase 1 is next. No language syntax,
runtime architecture, or compatibility promise has been finalized yet.

## Product family

- **Language** — a domain-neutral process language with textual and visual
  representations.
- **Engine** — validates and transforms process definitions into executable
  plans, generated code, or deployable artifacts.
- **Studio** — combines a visual process editor with an IDE for the textual
  language.
- **Bundles** — curated, supported process packages for particular kinds and
  sizes of business.

The language, engine, studio, and their public interfaces will be free
software. Commercial bundles may be proprietary and will live outside this
repository.

## Start here

- Read the [Manifest](MANIFEST.md) for the project’s purpose and principles.
- Read the [Roadmap](ROADMAP.md) for the proposed delivery sequence and exit
  criteria.
- Read [Contributing](CONTRIBUTING.md) and [Governance](GOVERNANCE.md) before
  proposing or implementing a substantial change.
- Consult the [decision-record index and process](docs/decisions/README.md) for
  durable project choices.
- Use the [working glossary](GLOSSARY.md) for core process terminology.
- Explore the
  [representative process corpus](docs/representative-processes/README.md) for
  the cases that will test semantic and notation proposals.
- Use the
  [design-evaluation framework](docs/evaluation/README.md) to compare competing
  semantic and notation designs against common evidence.

## Community and security

Participation is governed by the [Code of Conduct](CODE_OF_CONDUCT.md).
Security vulnerabilities should be reported privately according to the
[Security Policy](SECURITY.md), never through a public issue.

## Status

**Phase 1 next.** Groundwork established the project's identity, policies,
representative evidence, and design-evaluation method. The next work will define
the semantic core and build the first language prototype; it has not begun yet.

## Name

“Choreoform” combines *choreo-*—the arrangement of coordinated activity—with
*form*, reflecting one semantic process expressed faithfully in textual and
visual forms. The rationale, alternatives, and preliminary collision research
are recorded in [ADR-0001](docs/decisions/0001-project-and-language-name.md).

## Licensing

The licensing policy uses the Mozilla Public License 2.0 for software and
executable or code-like project assets, and Creative Commons Attribution 4.0
International for prose specifications and documentation. Process authors
retain the right to choose licenses for their own process definitions,
generated artifacts, extensions, and bundles, subject to the licenses of any
project material they copy or modify.

See [LICENSE.md](LICENSE.md) and
[ADR-0002](docs/decisions/0002-licensing-policy.md) for the boundaries and
rationale. The
[commercial-boundary review gate](docs/legal-review/README.md) defers qualified
legal review until the proprietary product boundary is concrete, while keeping
it mandatory before proprietary bundle development.
