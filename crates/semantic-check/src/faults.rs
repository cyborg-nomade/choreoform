// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Conservative possible handler selection, not fault dispatch or data proof.
use super::*;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};
mod analysis;
mod origins;
pub use origins::{FaultOriginReport, FaultOriginRoute, FaultOriginSite};

#[derive(Debug)]
pub struct FaultSelection {
    owner: String,
    policy: String,
    fault: String,
    handler: String,
}
impl FaultSelection {
    pub fn owner(&self) -> &str {
        &self.owner
    }
    pub fn policy(&self) -> &str {
        &self.policy
    }
    pub fn fault(&self) -> &str {
        &self.fault
    }
    pub fn handler(&self) -> &str {
        &self.handler
    }
}

/// Borrowed indexed metadata; no selection strings are duplicated per edge.
#[derive(Debug)]
pub struct FaultRoute<'a> {
    origin: &'a str,
    from: Option<&'a FaultSelection>,
    selected: &'a FaultSelection,
}
impl FaultRoute<'_> {
    pub fn origin(&self) -> &str {
        self.origin
    }
    pub fn from(&self) -> Option<&FaultSelection> {
        self.from
    }
    pub fn selected(&self) -> &FaultSelection {
        self.selected
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Route {
    origin: usize,
    from: Option<usize>,
    selected: usize,
}
#[derive(Debug)]
pub struct FaultReport {
    timing: TimingReport,
    scopes: Vec<String>,
    selections: Vec<FaultSelection>,
    routes: BTreeSet<Route>,
    covered: BTreeSet<String>,
    uncovered: BTreeSet<String>,
    selected: BTreeSet<usize>,
    contexts: usize,
    unhandled: usize,
}
impl FaultReport {
    pub fn revision(&self) -> &str {
        self.timing.revision()
    }
    pub fn timing(&self) -> &TimingReport {
        &self.timing
    }
    pub fn static_report(&self) -> &Report {
        static_report(&self.timing)
    }
    /// Abstract route contexts, not initialized or authorized scope occurrences.
    pub fn checked_scopes(&self) -> &BTreeSet<String> {
        &self.covered
    }
    pub fn uncovered_scopes(&self) -> &BTreeSet<String> {
        &self.uncovered
    }
    pub fn contexts(&self) -> usize {
        self.contexts
    }
    /// Known-name/context lookups with no selection; not observed runtime faults.
    pub fn unhandled_cases(&self) -> usize {
        self.unhandled
    }
    pub fn covered_handlers(&self) -> impl Iterator<Item = &FaultSelection> {
        self.selected.iter().map(|i| &self.selections[*i])
    }
    pub fn uncovered_handlers(&self) -> impl Iterator<Item = &FaultSelection> {
        self.selections
            .iter()
            .enumerate()
            .filter(|(i, _)| !self.selected.contains(i))
            .map(|(_, s)| s)
    }
    pub fn routes(&self) -> impl Iterator<Item = FaultRoute<'_>> {
        self.routes.iter().map(|r| FaultRoute {
            origin: &self.scopes[r.origin],
            from: r.from.map(|i| &self.selections[i]),
            selected: &self.selections[r.selected],
        })
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "occurrence-bound-ordinary-closure-and-snapshot-enforcement",
            "all-prior-timing-stage-obligations",
            "exceptional-handler-entry-availability-protection-and-invalidation",
            "concurrent-handler-activation-and-occurrence-lifetime-proofs",
            "truthful-business-versus-engine-fault-classification",
            "authenticated-fault-source-occurrence-cause-and-input-revisions",
            "runtime-selection-exclusion-and-cancellation-monotonicity",
            "live-handler-reconciliation-ownership-and-atomic-settlement",
            "full-benchmark-cross-form-and-user-evidence",
        ]
    }
}

/// Overapproximate declared business-fault names; never infer emitted faults
/// from retry eligibility, authenticate a fault or create a live handler.
pub fn check_fault_routes(
    envelope: &DefinitionEnvelope,
    supplied: &[Resource<'_>],
) -> Result<FaultReport> {
    let timing = check_timing(envelope, supplied)?;
    analysis::Analysis::new(&envelope.document()["body"], supplied, None)?
        .run(timing)
        .map(|(faults, _)| faults)
}

/// Declared emitting-site/context relations, not actual faults or entry values.
pub fn check_fault_origins(
    envelope: &DefinitionEnvelope,
    supplied: &[Resource<'_>],
) -> Result<FaultOriginReport> {
    let timing = check_timing(envelope, supplied)?;
    let analysis = analysis::Analysis::new(
        &envelope.document()["body"],
        supplied,
        Some(static_report(&timing)),
    )?;
    let (faults, records) = analysis.run(timing)?;
    Ok(FaultOriginReport::new(
        faults,
        records.expect("origin collection explicitly requested"),
    ))
}
fn static_report(timing: &TimingReport) -> &Report {
    timing
        .concurrency()
        .invalidation()
        .protection()
        .control()
        .static_report()
}
