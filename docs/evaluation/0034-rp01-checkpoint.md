<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-01 whole-source candidate checkpoint

**Date:** 2026-11-06<br>
**Plan:** [original package](0034-rp01-package-plan.md), `8304be4`, and
[integration continuation/amendment](0034-rp01-integration-plan.md), `65121dc`.

Draft #46 now explicitly selects source 0.5 / companion 0.4, unchanged core 0.2.
Actual native lowering derives [candidate IR](../benchmarks/rp01/candidate.ir.json)
from the 52,055-byte / 8,945-token source. The generator invokes the native
frontend for lowering and all ten current semantic stages, rather than supplying
invented stage observations. The manifest binds source/companion/package/resource
integrity, canonical IR revision/digest and exact stage code/path/source spans.
All ten stages currently refuse at their initial static prerequisite:
`information-flow at /body/nodes/checks_request_step; source bytes 20791..21147`.
They have not evaluated their later judgments. No semantic admission is claimed.

The attempted package-only fix explicitly adds all three intended fictional
capabilities to provider result protection. Under core 0.2 this reproduces
`policy-cycle`: the contract's result-envelope recipient references lead back to
its own capability wrapper. The initial empty provider capability set versus the
broad data envelope remains unresolved, not silently widened. Removing all
capabilities from every data envelope would deny required provider consumption
and correction/recheck; removing dependency edges would hide real protected reads.
A separately versioned dependency must distinguish finite metadata read closure
from recursive policy execution, preserving the old contract's cycle refusal.

Five new Python groups reproduce generated files and all ten native stage refusals,
wire schema/JCS/revision and package hashes, format/reparse identity, the real
shared-label cycle, unknown profile/resource revision/wrong-role failures and all
four scenario declaration mappings/current revision guards. They do not execute
scenario paths or authenticate roles/results. Existing Rust nominal/provider/clock
checks and profile tests remain. The first variant's sequential checks/omitted
optional calendar, fault classification, surviving ownership and all external
host boundaries stay explicit in the package host contract.

Local verification passed 258 Rust tests plus five compile-fail doctests,
workspace lint, all existing Python CI oracles and the new five groups. Logs:
`/private/tmp/rp01-integration-rust.log`, `/private/tmp/rp01-integration-clippy.log`,
`/private/tmp/rp01-integration-oracles.log`. Native/Wasm implementation is unchanged
since passing dependency PR #47; the parent still requires its own final-head CI
and completed whole-package review before merge. The new oracle is in CI.

Separate automated checkpoint review checked actual subprocess results, exact
resource-byte pinning, reproducible JCS and diagnostic parsing, absence of shell
interpolation, output/report authority, original profile refusals and scenario
accountability. It found the policy-cycle dependency and recorded it without a
semantic workaround. This is a progress checkpoint under delegation, not approval
of the incomplete deliverable or independent/individual owner review. Draft #46
remains unmerged. All forty scenarios remain Partial; no Phase 1 gate closes.
