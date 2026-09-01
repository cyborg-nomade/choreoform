<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Commercial-boundary legal-review gate

**Status:** Deferred; not currently triggered<br>
**Policy:** [ADR-0002](../decisions/0002-licensing-policy.md)<br>
**Owner:** Project owner

This gate records when Choreoform must obtain qualified legal review of the
open/commercial boundary. It lets work on the entirely free-software foundation
proceed without treating a hypothetical future commercial product as a current
legal expense.

This document is project governance, not legal advice. It does not conclude
that a particular bundle, extension, generated artifact, distribution, or
business model is legally independent of Choreoform's MPL-covered software.

## Current posture

Phases 1–4 develop the language, canonical model, engine, Studio, SDK, package
format, conformance tools, and free reference material in public under the
licensing policy. That work does not itself begin the proprietary bundle
product line described in Phase 5.

Qualified review remains required before proprietary bundle development. The
review is deliberately deferred until the relevant interfaces, generated-output
behavior, packaging, jurisdictions, and distribution model are concrete enough
for useful advice.

Deferral is not approval. Until the review is complete, the project must not
cross any trigger below.

## Trigger conditions

The legal-review gate becomes blocking before the first of these actions:

1. creating proprietary bundle source, configuration, test data, integration,
   or release infrastructure;
2. implementing a private bundle-only capability, adapter, SDK extension, or
   modification to the open foundation;
3. combining Choreoform software and proprietary bundle material for delivery
   to another person or organization;
4. publishing proprietary bundle terms, an end-user license agreement,
   certification terms, or a paid bundle offer;
5. accepting a paid bundle pilot or another obligation to deliver proprietary
   bundle functionality;
6. releasing a generator that copies MPL-covered templates or runtime source
   into generated artifacts; or
7. using Choreoform names or marks to distinguish a commercial bundle in a way
   that requires a trademark or endorsement policy.

Researching customer problems, publishing the open bundle format, implementing
public extension points, and creating free reference bundles do not trigger the
gate when they remain public, follow ADR-0002, and create no proprietary
delivery obligation.

Start preparing the review before a triggering action becomes the next planned
deliverable. Do not wait until code has already crossed the boundary.

## Safeguards while review is deferred

Until the gate is satisfied:

- keep foundation development and its required interfaces in the public
  repository;
- apply MPL-2.0 to software and code-like assets and CC-BY-4.0 to project prose
  according to ADR-0002;
- maintain file-level SPDX notices and add automated REUSE compliance when
  implementation files are introduced;
- record third-party dependencies, their licenses, notices, and copied or
  generated material before adoption;
- keep free reference bundles and examples clearly licensed;
- make generated scaffolding original and reference separately distributed
  runtime libraries rather than copying covered implementation code;
- require a separate ADR before releasing a backend that copies covered
  templates or runtime code; and
- do not create private-only interfaces or modifications required to build,
  test, or operate the open foundation.

Repository separation supports review and access control but is not treated as
a legal conclusion about whether two works are independent.

## Review scope when triggered

The Project Owner should seek counsel experienced in software copyright and
free/open-source licensing in the jurisdictions relevant to the first
commercial offering. Give the reviewer concrete facts and artifacts, including:

- the legal person or people stewarding the project and commercial offering;
- ADR-0002, LICENSE.md, contribution terms, and file-level licensing practice;
- the implemented language, canonical model, SDK, capability, adapter, and
  bundle boundaries;
- representative open and proposed proprietary artifacts, including any copied
  or generated material;
- source, binary, browser, container, hosted-service, and combined-distribution
  models that are actually planned;
- generated-code and runtime-linking behavior;
- third-party dependency and notice practices;
- proposed proprietary, support, certification, and trademark terms; and
- the countries in which development, distribution, hosting, and sales will
  occur.

Ask for written conclusions about required source availability and notices,
the treatment of bundle and extension files, generated output, combined
distribution, contributor provenance, trademarks, commercial terms, and any
applicable product regulation. Ask the reviewer to identify assumptions and
events that require renewed review.

## Completion evidence

The gate is satisfied only when:

1. a qualified reviewer has assessed the concrete boundary and planned first
   commercial distribution;
2. the review identifies its factual and jurisdictional assumptions;
3. every required change that affects the open foundation or the first
   proprietary product is completed before the triggering action;
4. deferred questions have explicit owners and triggers; and
5. a non-confidential repository note records the review date, scope, overall
   outcome, implemented changes, and remaining triggers without publishing
   confidential advice.

The underlying legal advice should remain outside the public repository unless
the reviewer and Project Owner deliberately approve publication. Checking an
action item or writing a self-assessment does not substitute for the qualified
review required by ADR-0002.

## Reopening the gate

Complete review of one offering does not permanently approve every future
model. Reopen the gate before a material change to licensing, copied generated
content, component boundaries, distribution channels, relevant jurisdictions,
commercial terms, or the relationship between the public foundation and
proprietary products.
