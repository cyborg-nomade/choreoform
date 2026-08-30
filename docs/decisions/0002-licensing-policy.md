<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0002: License the open foundation without capturing user processes

**Status:** Proposed<br>
**Date:** 2026-08-30<br>
**Decider:** Project owner

## Context

The Manifest commits the language specification, canonical model, engine,
studio, conformance suite, and public extension interfaces to free software.
It also permits proprietary process bundles. The licensing policy must make
both commitments credible without a custom license, ambiguous “open core”
exceptions, or a requirement that users license their own processes to the
project.

The project has several materially different outputs:

- software implementations and development tools;
- a prose language specification and explanatory documentation;
- user-authored process definitions;
- machine-generated execution artifacts;
- third-party extensions and adapters; and
- free or proprietary process bundles.

A single unexplained repository license would leave important questions at
these boundaries unanswered.

This ADR records project intent and standard-license selection. It is not legal
advice, and it cannot determine whether a specific third-party work is legally
derivative. The open/commercial boundary should receive qualified legal review
before proprietary bundle development begins.

## Decision criteria

The policy should:

1. guarantee the freedoms to use, study, modify, and redistribute the open
   foundation;
2. keep distributed modifications to core implementation files available to
   the community;
3. permit independent free or proprietary extensions and bundles through
   stable public interfaces;
4. avoid imposing the project’s license on user-authored process definitions or
   generated output merely because Choreoform processed them;
5. include an explicit contributor patent grant for software;
6. use recognized, standard licenses with established SPDX identifiers;
7. remain compatible with broad free-software reuse; and
8. keep compliance understandable at the file level.

## Decision

Adopt a two-license policy:

1. **Mozilla Public License 2.0 (`MPL-2.0`)** for software source, build
   scripts, tests, schemas, grammars, interface definitions, code generators,
   executable examples, and other code-like assets.
2. **Creative Commons Attribution 4.0 International (`CC-BY-4.0`)** for prose
   specifications, documentation, diagrams, and other non-software explanatory
   material.

Use the standard MPL without the Exhibit B “Incompatible With Secondary
Licenses” notice. This preserves MPL’s secondary-license compatibility
mechanism.

Store canonical texts in `LICENSES/`, summarize the rules in `LICENSE.md`, and
identify each new file with SPDX copyright and license notices.

## Why this boundary

MPL 2.0 applies copyleft at the file level. Distributed changes to covered
files remain available under MPL, while separate files in a larger work may use
other terms. This matches the intended boundary: improvements to Choreoform’s
open implementation stay open, but an independently authored adapter or bundle
does not become MPL-covered merely because it interoperates through a public
interface.

CC BY 4.0 allows anyone to copy, adapt, translate, and commercially reuse the
specification and documentation with attribution. Creative Commons recommends
software-specific licenses for software while recognizing CC licenses as
appropriate for software documentation, which supports the proposed division.

## Artifact policy

| Artifact | Policy |
| --- | --- |
| Engine, studio, SDK, compiler, runtime, and language tooling | MPL-2.0 |
| Canonical schemas, grammars, interface definitions, and code-generation templates | MPL-2.0 |
| Conformance runners, test harnesses, and executable examples | MPL-2.0 |
| Prose language specification, tutorials, ADRs, and diagrams | CC-BY-4.0 |
| A user’s process definition outside this repository | Author chooses |
| Generated code or deployment artifact | Author chooses, except copied covered material retains its license |
| Choreoform-maintained extension or adapter | MPL-2.0 |
| Independent third-party extension or adapter in separate files | Author chooses |
| Modification to an MPL-covered file | MPL-2.0 when distributed |
| Free example bundle committed here | Declared free license; MPL-2.0 by default for executable source |
| Proprietary commercial bundle in a separate repository | Proprietary terms permitted |
| Third-party dependency or vendored material | Its upstream license |

## Generated output rule

Running a compiler or engine does not, by itself, copy copyright from that tool
into its input or output. Choreoform therefore claims no additional rights in a
process definition or generated artifact merely because Choreoform processed
it.

This policy does not erase rights in material actually copied into an output.
If a generator copies covered template or runtime source into a generated file,
that material retains its license. To keep the boundary predictable, execution
backends should generate original scaffolding and reference separately
distributed runtime libraries instead of copying substantial covered code.

If a future backend must copy covered templates, that backend requires a new
ADR addressing output licensing before release. The project will not invent a
custom output exception preemptively.

## Extension and bundle rule

An extension or bundle may be free or proprietary when it is independently
authored in separate files and uses documented public interfaces. MPL-covered
files copied into or modified for that product remain MPL-covered when
distributed.

Proprietary bundles must:

- live outside the public foundation repository;
- operate on an unmodified public Choreoform release unless a modification is
  also made available as MPL-covered source;
- use the same public SDK, bundle format, and compatibility rules available to
  independent developers; and
- carry their own terms without restricting recipients’ rights in the open
  foundation.

Copyright licenses do not replace bundle signing, certification, support,
pricing, or trademark policies.

## Contributions

Use an inbound-equals-outbound model: a contribution is made under the existing
license of the file changed, and a new file uses the repository default for its
artifact type. No copyright assignment or contributor license agreement is
required by this decision.

A later governance deliverable may adopt a Developer Certificate of Origin or
another lightweight provenance mechanism. That process must not silently alter
the licenses selected here.

## Options considered

| Option | Core changes stay open | Proprietary extensions/bundles | Patent grant | Network-hosted modifications | Complexity | Outcome |
| --- | --- | --- | --- | --- | --- | --- |
| **MPL-2.0 + CC-BY-4.0** | At file level on distribution | Permitted in separate files | Yes for covered software contributions | Source release not required solely for hosting | Medium | Recommend |
| **Apache-2.0 + CC-BY-4.0** | Not required | Permitted | Yes | Source release not required | Low | Reject: permits closed core forks |
| **AGPL-3.0 + CC-BY-4.0** | Broadly, including network interaction | Possible but boundary is more complex | Yes | Corresponding source offered to network users | Medium–high | Reject for initial ecosystem |
| **GPL/LGPL split + CC-BY-4.0** | Broad or library-level depending on component | Requires component-specific linking analysis | Yes | Source release not required solely for hosting | High | Reject: too many boundaries |
| **Custom source-available or commercial exception** | Depends on custom text | Depends on custom text | Uncertain | Depends on custom text | Very high | Reject: not standard free software |

### Apache 2.0

Apache 2.0 is permissive, well understood, and includes copyright and patent
grants. It would make independent implementations and commercial adoption easy.
It would also permit a distributor to modify the core, ship the modified
version, and withhold those modifications. That is weaker than the Manifest’s
commitment to a durable free foundation.

### GNU AGPL 3.0

AGPL provides strong copyleft and requires a network-facing modified program to
offer corresponding source to users interacting with it. That can protect a
hosted studio or engine from closed service forks. It also creates a broader
and less intuitive boundary for embedded runtimes, integrations, and commercial
bundle deployments. The project does not yet have enough architecture to apply
that boundary safely.

AGPL may be reconsidered for a clearly separable hosted service in a later ADR;
it should not be the default for the entire initial product family.

### GPL or LGPL component split

Licensing applications under GPL and libraries under LGPL is conventional, but
it requires early agreement about which components are libraries, programs,
plugins, or combined works. Choreoform has not designed those boundaries yet.
MPL’s file-level rule is simpler and applies consistently across the product
family.

## Consequences

- Distributed improvements to existing Choreoform software files must remain
  available under MPL-2.0.
- Separate proprietary extensions and bundles are permitted, supporting the
  commercial model without a proprietary fork of the core.
- Hosted modifications do not trigger source-disclosure obligations merely
  because users interact over a network; this is an intentional compromise and
  may be revisited for a future hosted service.
- Specification reuse requires attribution but not share-alike licensing.
- The repository carries two standard licenses, so every file needs a clear
  classification and SPDX identifier.
- Generated-code architecture must avoid silently copying covered
  implementation code.
- Trademarks, certification, warranties, services, and proprietary bundle terms
  require separate policies.

## Sources

- [MPL 2.0 license](https://www.mozilla.org/en-US/MPL/2.0/)
- [Mozilla’s MPL 2.0 FAQ](https://www.mozilla.org/en-US/MPL/2.0/FAQ/)
- [Creative Commons Attribution 4.0 deed](https://creativecommons.org/licenses/by/4.0/)
- [Creative Commons FAQ on software and documentation](https://creativecommons.org/faq/#can-i-apply-a-creative-commons-license-to-software)
- [Apache guidance for applying Apache 2.0](https://www.apache.org/legal/apply-license)
- [GNU guidance on AGPL network source offers](https://www.gnu.org/licenses/gpl-howto.html#The-Affero-notice)
- [FSF license classification and MPL compatibility notes](https://www.gnu.org/licenses/license-list.html)
- [REUSE Specification 3.0](https://reuse.software/spec-3.0/)

## Acceptance and action items

This ADR remains **Proposed** until the project owner approves it. Before merge,
change its status and `LICENSE.md` to **Accepted**.

1. [ ] Obtain project-owner approval.
2. [ ] Change this ADR and `LICENSE.md` to Accepted.
3. [x] Verify every repository file has an SPDX license declaration or explicit
   coverage.
4. [ ] Add automated REUSE compliance once implementation files are introduced.
5. [ ] Obtain qualified legal review before proprietary bundle development.
6. [ ] Revisit generated-output licensing before releasing a backend that
   copies covered templates or runtime code.
