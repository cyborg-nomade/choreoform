<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Explicit retry fault-source authoring evidence

**Date:** 2026-11-06<br>
**Plan:** [frozen scope](0043-retry-fault-authoring-plan.md), `eb16001`<br>
**Decision:** [ADR-0043](../decisions/0043-retry-fault-source-authoring.md)

Source 0.7/companion 0.6 explicitly selects core 0.4 and all inherited artifacts.
Its new EBNF changes only the retry-binding production (and version comment):
multiple attempts explicitly name a fault source after clock/basis; single attempts
omit the sentence and lower null bindings. Only the new closed companion admits
provider-fault-source resources. Libraries require exact caller-supplied bases /
source/profile/provider/type bytes; the native CLI embeds the known four cores.
Metadata is decoded before lowering; actual eligible fault/provider/use validation
is a subsequent static check. Lowering alone does not admit a retry.

Source-provider protection references contribute exact metadata reads, including
an unused policy whose distinct source provider references a boolean protection
read absent from its ordinary authority/basis. Strong references, visibility,
finite closure, actual-use endpoints and existing work/token/output limits remain.
Unknown negative sentinels now use unsupported 0.8; old schemas/EBNF/core pairs are
unchanged. No quota or implicit migration is introduced.

Actual promoted copies of RP-01 measure **56,874 bytes / 9,470 tokens** and pass
all composed partial stages through reconciliation availability. The fixture's
semantic revision is
`sha256:b9cc8c77f2ed3dba350b2ea72435aeaf83348f1346d996dae3330600017b8ca4`;
this synthetic test source uses `urn:test:authoring-payment-faults`, not an adopted
benchmark resource. The explicit promotion changes core/retry/source and revision.
Format/reparse and coordinated source-resource alias changes preserve candidate
IR/revision; independent Python framing/JCS/hash checks match native integrity.

All five native reports from timing onward disclose one identical typed
`retryFaultBindings` row: exact revision, payment use/source/profile/provider,
eligible confirmed-no-effect declaration and `no-effect` observation name, plus
actual evidence/correlation/authority/retry/ownership requirements. Noneligible
`provider unknown` also enters abstract unhandled-name accounting. There is no
actual provider observation or fault. Old source 0.6 reports, even with unrelated
valid extra metadata, gain no row and retain their exact checked-in structure.

Five new Rust groups exercise the actual full-source pipeline/typed evidence,
format/alias/revision behavior, all inherited/source/provider/type pins and corrupt
bytes, strict pairs/resource-kind/versioned grammar, known/unknown/fragment quotas
and refusal of stale generated reads. Seven Python groups independently cover
EBNF/schema/IR revisions and package digests, all five disclosures, actual human
and exact-provider mismatch (including unvisited sites), source-only metadata in
unused work, legacy compatibility, missing/corrupt source/profile/provider bytes
and bounded native output. Refusals print generic code/span, not secret test names.
Raw-core evidence/class/cycle and cumulative-work tests remain in ADR-0042.

The actual report-budget fixture has 130 sequential read-only declared requests,
29,091 source bytes / 3,976 tokens, a 4,000-character eligible fault and matching
4,000-character observation declaration. Candidate lowering succeeds; **every one
of the five native report commands** refuses `report-budget` with empty stdout.
Its owned direct-child recovery contains a real declared typed wait, so later
stages reach disclosure rather than failing an earlier reconciliation premise.
This is static generated stress evidence, not host execution or a user study.

Passed 320 Rust tests plus six compile-fail doctests; format/native workspace lint;
all five library Wasm lints and release portability/frontend builds; native oracle
(89 cases, 15 JCS/hash comparisons, three unchanged fixtures, ten node variants,
seven native CLI checks); all required Python oracles and unchanged package exact
regeneration/historical probes. The expanded five-command stress case passed its
focused rerun after the full oracle run. Logs:
`/private/tmp/fault-authoring-{workspace,clippy,wasm,native,oracles,tests}.log`.
Final-head CI/comment audit/signed merge remain required.

Separate completed-diff automated self-review checked closed profile selection,
unchanged old paths/bytes, all explicit/inherited pins, candidate versus admission,
nullable fields, lexical resource kind, exact use/provider identity, metadata-only
read closure and shared budgets, new/old native structure, output bounds/privacy,
revision/alias semantics, licensing and remaining gates. Initial synthetic tests
were corrected to use accepted request/result syntax and explicit invalidations;
the expanded stress fixture was given its required owned reconciliation wait so
all five paths test the output boundary. No production contract was weakened for
the fixture. No remaining actionable issue. Review is delegated self-review, not
independent, human or individual owner approval; audit every PR/CodeRabbit finding.

Checked-in RP-01 source 0.6/core 0.3/candidate/resources/reports are unchanged.
Next freeze separate adoption, publish the exact synthetic payment fault source,
regenerate all reports and repin guard-vector revision without changing outcomes.
Then address remaining C/D fault-data/late-observation/continuation requirements.
Actual host/user/browser/independent evidence, full validator/benchmarks, protected
accessible editing and all scenario/phase gates remain open. Parent #46 remains
draft/incomplete/unmerged; no Phase 2 work starts.
