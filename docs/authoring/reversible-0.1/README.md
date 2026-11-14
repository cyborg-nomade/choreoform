<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Reversible authoring 0.1.0 worked contract vectors

These synthetic, non-executable artifacts illustrate the
[normative contract](../reversible-v0.1.md). `vectors.json` contains exact raw pins,
independent authoring digests, finite before/request/after pairs, refusal codes and
a stale request against a layout-changed base. Base/renamed/input-renamed/type-renamed/
protection-renamed/layout/formatted/expanded documents retain the same native IR.
The expanded alternative deliberately removes the pure test and adds its IDs to
tombstones; it is an import example, not an admitted0.1.0 structural operation.

After-states are finite authored examples, not output of a native rename engine.
They preserve an opaque literal `Same` and annotation text while changing only
resolved source-only names. The input alias `Value` coexists with a distinct data
cell named `Value`, exercising different identity and fixed wire preservation.
All documents retain a retired ID; layouts cannot target it. The schemas alone
accept some negative vectors which the additional consistency rules must refuse.

Current native lowering/formatting verifies the examples' semantic projections.
No production document/change parser or visual editor is introduced. Authorization
is an external trusted prerequisite, never asserted by these fixtures. No actual
user, browser, independent security/review or host execution evidence is supplied.
