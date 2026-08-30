<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Choreoform licensing policy

**Status:** Accepted<br>
**Decision record:** [ADR-0002](docs/decisions/0002-licensing-policy.md)

This repository uses standard licenses for different kinds of material.

## Default licenses

| Material | Default license |
| --- | --- |
| Software source, build scripts, tests, schemas, grammars, interface definitions, code generators, templates that emit code, and executable process examples | [Mozilla Public License 2.0](LICENSES/MPL-2.0.txt) (`MPL-2.0`) |
| Prose specifications, documentation, diagrams, and other non-software explanatory material | [Creative Commons Attribution 4.0 International](LICENSES/CC-BY-4.0.txt) (`CC-BY-4.0`) |

An individual file may declare a different license with an SPDX identifier.
Third-party material retains its original license and must be identified.

## Source-file notices

New project files should contain an SPDX notice appropriate to their format.

Software:

```text
SPDX-FileCopyrightText: <year> <copyright holder>
SPDX-License-Identifier: MPL-2.0
```

Documentation:

```text
SPDX-FileCopyrightText: <year> <copyright holder>
SPDX-License-Identifier: CC-BY-4.0
```

Do not add the MPL Exhibit B notice. Choreoform software is intended to retain
MPL 2.0’s compatibility mechanism for GNU-family secondary licenses.

## Processes, generated artifacts, extensions, and bundles

Using Choreoform does not require a process author to adopt the project’s
licenses.

- **Process definitions:** Authors choose the license for process definitions
  they create. Process definitions committed to this repository use the
  file’s declared license or, by default, MPL-2.0.
- **Generated artifacts:** The Choreoform project claims no additional
  copyright merely because an artifact was produced by Choreoform software.
  Authors may license generated artifacts as they choose. Material copied from
  MPL-covered templates or runtime sources remains subject to MPL-2.0.
- **Extensions and adapters:** Choreoform-maintained extensions and adapters
  use MPL-2.0. Independently authored files outside this repository may use
  free or proprietary licenses. Files that contain or modify MPL-covered source
  remain covered by MPL-2.0 when distributed.
- **Bundles:** Free and proprietary bundles are both permitted. Proprietary
  bundles must remain in separate files and repositories, use documented public
  interfaces, and may not remove or relicense MPL-covered project code.
- **Runtime dependencies:** Generated programs should call or depend on runtime
  libraries instead of copying covered implementation code into generated
  files. Runtime libraries retain their own licenses.

These statements describe project policy, not a conclusion that any particular
third-party work is or is not legally derivative. Seek qualified legal advice
for a specific distribution or commercial offering.

## Contributions

Unless a file states otherwise, contributions are licensed under the existing
license of the file being modified. New software files use MPL-2.0; new prose
documentation uses CC-BY-4.0. No copyright assignment is required by this
policy.

## Trademarks and commercial terms

The licenses do not grant rights to Choreoform names, logos, or trademarks
beyond uses necessary for attribution and license compliance. Commercial
warranties, support, certification, and proprietary bundle terms are separate
from the copyright licenses of the open foundation.

## Canonical license texts

- [Mozilla Public License 2.0](LICENSES/MPL-2.0.txt)
- [Creative Commons Attribution 4.0 International](LICENSES/CC-BY-4.0.txt)
