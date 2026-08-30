<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0001: Name the project and language Choreoform

**Status:** Accepted<br>
**Date:** 2026-08-30<br>
**Decider:** Project owner

## Context

The project needs a permanent name before its language syntax, package names,
command-line interface, file conventions, and product family begin to create
switching costs.

The initial working name, **General Process Language (GPL)**, accurately
describes the ambition but is not a viable long-term identity:

- GPL is overwhelmingly associated with the GNU General Public License, making
  conversation about the project and its free-software licenses ambiguous.
- *A General Process Language and Its Categorical Models* used the exact phrase
  for an academic process language in 1999.
- The name is generic and therefore difficult to search for, protect, or use as
  a coherent family identity.

The name should cover the language and the wider product family without tying
the semantics to conventional flowcharts, business-only workflows, AI agents,
or a particular execution architecture.

## Decision criteria

The selected name should be:

1. **Distinctive:** discoverable without competing directly with an active
   process, workflow, language, or developer-tool product.
2. **Semantically relevant:** suggest coordinated activity, executable meaning,
   or equivalent representations without narrowing the project prematurely.
3. **Usable:** pronounceable, spellable, and compact enough for documentation,
   commands, and package names.
4. **Extensible:** work naturally across the language, engine, studio, SDK, and
   commercial bundles.
5. **Neutral:** avoid a particular industry, vendor, implementation technology,
   or product tier.
6. **Legally screenable:** have no obvious collision in this preliminary search;
   formal trademark clearance remains a separate legal task.

## Decision

Adopt **Choreoform** as the name of the project and language, subject to formal
name clearance before the first public product release.

The name combines:

- **choreo-**, from choreography: the arrangement and coordination of actions;
  and
- **form**, representing the equivalent textual and visual forms backed by one
  canonical semantic model.

The product family will use:

| Product | Name |
| --- | --- |
| Language and open project | Choreoform |
| Execution engine | Choreoform Engine |
| Visual editor and textual IDE | Choreoform Studio |
| Public extension toolkit | Choreoform SDK |
| Process packages | Choreoform Bundles |

`choreoform` is reserved as the preferred stem for repository, command, and
package names. This decision does **not** choose a source-file extension,
Internet domain, logo, visual identity, or legal entity name.

## Options considered

| Option | Distinctiveness | Semantic fit | Product-family fit | Collision risk | Outcome |
| --- | --- | --- | --- | --- | --- |
| **Choreoform** | High | High | High | Lowest observed | Recommend |
| **Enacture** | Medium–high | High for execution; medium for representation | High | `.com` identity occupied | Keep as fallback |
| **General Process Language / GPL** | Low | High | Medium | Very high | Reject |
| **Praxis** | Low | High | High | Very high in adjacent software | Reject |
| **Praxform** | Medium | High | High | Existing repository and commercial uses | Reject |

### Choreoform

**Advantages**

- Connects coordination and representation, the two central ideas in the
  Manifest.
- Supports the whole product family without an acronym.
- Preliminary exact-name searches found no active software product or company
  using the name.
- Exact `choreoform` names were unclaimed on GitHub, npm, and PyPI when checked.

**Costs and risks**

- It is coined, so its pronunciation and meaning initially require explanation.
- “Choreography” has a narrower technical meaning in some distributed-systems
  literature; the project must consistently explain its broader use.
- Search and package availability are not trademark clearance.

### Enacture

**Advantages**

- An existing, if obsolete, English word meaning enactment or resolution.
- Directly conveys the move from definition to execution.
- Exact package and GitHub names appeared unclaimed in the preliminary search.

**Costs and risks**

- `enacture.com` is already in use.
- The word is unfamiliar and may be heard as “enactor” or “enactment.”
- It emphasizes execution but does not express the text/visual duality as well
  as Choreoform.

### General Process Language / GPL

**Advantages**

- Immediately communicates the intended category and scope.
- Already used throughout the initial groundwork documents.

**Costs and risks**

- Conflicts in ordinary technical conversation with the GNU GPL license family.
- The exact phrase already names prior academic work.
- Generic search terms make the project hard to distinguish.

### Praxis and Praxform

Both connect naturally to action and practice. Praxis, however, is already used
by multiple active developer tools, an AI workflow language/runtime, and
workflow-oriented commercial products. Praxform has existing repository names
and prior commercial uses. Neither offers enough differentiation.

## Preliminary collision research

Research was performed on 2026-08-30. It is evidence for naming, not a legal
opinion.

| Check | Choreoform result |
| --- | --- |
| General web search for exact name plus software/language/workflow terms | No active competing software identity surfaced |
| GitHub repository-name search | 0 repositories returned |
| npm registry package endpoint | Not found (HTTP 404) |
| PyPI project endpoint | Not found (HTTP 404) |
| crates.io package endpoint | Inconclusive (registry rejected the automated probe) |
| Trademark clearance | Not performed; required before public product release |
| Domains and social handles | Not reserved; select only after this ADR is accepted |

Key sources:

- [A General Process Language and Its Categorical Models](https://www.brics.dk/RS/99/36/BRICS-RS-99-36.pdf)
- [GitHub search for `choreoform`](https://github.com/search?q=choreoform&type=repositories)
- [npm package page for `choreoform`](https://www.npmjs.com/package/choreoform)
- [PyPI project page for `choreoform`](https://pypi.org/project/choreoform/)
- [Merriam-Webster definition of “enacture”](https://www.merriam-webster.com/dictionary/enacture)
- [Praxis language/runtime repository](https://github.com/cssmith615/praxis)
- [Processa approval-workflow product](https://getprocessa.com/about)

## Consequences

- Public prose will refer to the project and language as Choreoform.
- Future component names should use the family names in this ADR.
- The GitHub repository is named `cyborg-nomade/choreoform`.
- Package names, domains, handles, file extensions, and visual identity remain
  uncommitted until separately selected or reserved.
- If formal clearance finds a material conflict, Enacture is the documented
  fallback, but adopting it requires a superseding ADR.

## Acceptance and action items

The project owner approved this ADR on 2026-08-30. Package, domain, and public
handle reservation remains a later rollout action.

1. [x] Obtain project-owner approval.
2. [x] Change this ADR’s status to Accepted.
3. [x] Rename the GitHub repository after the decision is merged.
4. [ ] Reserve required package namespaces, domains, and handles before they are
   advertised.
5. [ ] Arrange formal trademark clearance before the first public product
   release or commercial bundle offering.
