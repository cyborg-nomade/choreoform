// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Conservative ordinary protection constraints; no authenticated access grant.
use super::*;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};
use std::sync::Arc;
mod analysis;
use analysis::Flow;

/// A required future access judgment at a declaration site in the bound revision.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccessObligation {
    site: String,
    cell: String,
    operation: &'static str,
}
impl AccessObligation {
    pub fn site(&self) -> &str {
        &self.site
    }
    pub fn cell(&self) -> &str {
        &self.cell
    }
    pub fn operation(&self) -> &str {
        self.operation
    }
}
/// Immutable ordinary-flow evidence and unresolved access obligations, never a plan.
#[derive(Debug)]
pub struct ProtectionReport {
    control: ControlReport,
    transfers: usize,
    accesses: BTreeSet<AccessObligation>,
}
impl ProtectionReport {
    pub fn revision(&self) -> &str {
        self.control.revision()
    }
    pub fn control(&self) -> &ControlReport {
        &self.control
    }
    /// Number of static envelope comparisons, never a count of runtime transfers.
    pub fn transfers(&self) -> usize {
        self.transfers
    }
    pub fn accesses(&self) -> &BTreeSet<AccessObligation> {
        &self.accesses
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "parallel-and-exceptional-availability-proofs",
            "exceptional-and-observable-channel-information-flow",
            "current-principal-purpose-capability-and-policy-enforcement",
            "race-and-invalidation-proofs",
            "host-trust-and-atomic-runtime-enforcement",
            "full-benchmark-and-conformance-evidence",
        ]
    }
}

/// Check ordinary protected transfers and enumerate required access judgments.
/// Runtime facts and authorization are deliberately absent from this interface.
pub fn check_protection(
    envelope: &DefinitionEnvelope,
    supplied: &[Resource<'_>],
) -> Result<ProtectionReport> {
    let control = check_control(envelope, supplied)?;
    let b = &envelope.document()["body"];
    let mut flow = Flow::new(b, supplied)?;
    flow.scope(text(&b["root"])?, &BTreeSet::new(), 0)?;
    Ok(ProtectionReport {
        control,
        transfers: flow.transfers,
        accesses: flow.accesses,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Origin {
    Cell(String),
    Provider(String),
}
type Origins = BTreeSet<Origin>;

fn diagnostic(code: &'static str, map: &str, id: &str) -> Diagnostic {
    Diagnostic {
        code,
        path: format!("/body/{map}/{id}"),
    }
}

// The same no-declassification rule as ADR-0013 direct transfer checking.
pub(crate) fn transfer_cost(source: &Value, target: &Value) -> Result<usize> {
    let mut work = 1;
    for p in [source, target] {
        for field in ["purposes", "participants", "capabilities"] {
            work += object(&p[field])?.len();
        }
    }
    Ok(work)
}
pub(crate) fn transfer(source: &Value, target: &Value) -> Result<()> {
    require(
        source["sensitivity"] == target["sensitivity"] && source["policy"] == target["policy"],
        "information-flow",
    )?;
    for field in ["purposes", "participants", "capabilities"] {
        require(
            names(&target[field])?.is_subset(&names(&source[field])?),
            "information-flow",
        )?;
    }
    Ok(())
}
