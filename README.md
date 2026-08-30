# Choreoform

Choreoform is a family of tools for describing, executing, and editing
information processes in equivalent textual and visual forms. The name and its
rationale are recorded in
[ADR-0001](docs/decisions/0001-project-and-language-name.md).

The project is currently in its groundwork phase. No language syntax, runtime
architecture, or compatibility promise has been finalized yet.

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

## Status

**Groundwork.** The immediate work is to validate representative use cases,
define the semantic core, settle licensing, and build the first end-to-end
language-to-execution slice.

## Name

“Choreoform” combines *choreo-*—the arrangement of coordinated activity—with
*form*, reflecting one semantic process expressed faithfully in textual and
visual forms. The rationale, alternatives, and preliminary collision research
are recorded in [ADR-0001](docs/decisions/0001-project-and-language-name.md).
