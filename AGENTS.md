<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Project execution instructions

The Project Owner delegated implementation, technical decisions, PR self-review
and merge within each Roadmap phase on 2026-11-05. See
[ADR-0018](docs/decisions/0018-phase-review-delegation.md) and
[the current progress record](docs/evaluation/phase-1-progress.md).

- Continue current-phase work without per-item approval or permission to start.
  Preserve dedicated branches/PRs and the accepted contracts/Manifest.
- Make durable decisions in ADRs. Record acceptance under owner delegation;
  do not claim the owner personally reviewed a future PR or ADR.
- Review the completed diff in a separate pass for correctness, security,
  performance, scope and evidence. Record automated self-review honestly.
  Do not submit a purported independent human or GitHub self-approval.
- Evaluate all unresolved review comments, including CodeRabbit summary findings.
  Fix valid issues and explain rejected ones. A skipped bot review supplies no
  review evidence; perform and record the agent's review.
- Run relevant tests and required CI at the final head. Merge with the expected
  head; sign off every commit and preserve SPDX licensing.
- Update the progress record after merges, including the next dependency and
  open gates. Keep work resumable after interruptions.
- Stop at each phase's report for owner approval. Open the next phase chat only
  after that approval. Do not start Phase 2 while Phase 1 gates are unresolved.
- Never substitute generated observations for real representative-user studies,
  independent review, host/browser execution or other external evidence. Finish
  independent work before reporting a genuine external blocker. An incomplete
  phase report must say which exit criteria remain unmet.

Rust uses the pinned toolchain and locked dependencies. Local tools are under
`.tools/cargo`, `.tools/rustup` and `.tools/ir-check`; the CI workflow documents
the required native/Wasm and Python checks. Reports and digests never confer
execution or host authorization beyond their documented stage.
