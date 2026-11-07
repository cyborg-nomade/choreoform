// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Conservative ordinary-control proof, not exceptional-flow or execution admission.
use super::*;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};
mod availability;
pub(crate) mod graph;
use availability::Analysis;
use graph::Graph;

/// Proof coverage for one immutable definition revision, never an executable plan.
#[derive(Debug)]
pub struct ControlReport {
    static_report: Report,
    covered_nodes: BTreeSet<String>,
    uncovered_nodes: BTreeSet<String>,
    covered_scopes: BTreeSet<String>,
    required_inputs: BTreeSet<String>,
}
impl ControlReport {
    /// The same checked static report retained by this composed result; no rerun.
    pub fn static_report(&self) -> &Report {
        &self.static_report
    }
    pub fn revision(&self) -> &str {
        self.static_report.revision()
    }
    /// Ordinary-reachable declarations visited under modeled root/call contexts.
    pub fn covered_nodes(&self) -> &BTreeSet<String> {
        &self.covered_nodes
    }
    /// Type/shape checked, but not assigned an invented availability entry context.
    pub fn uncovered_nodes(&self) -> &BTreeSet<String> {
        &self.uncovered_nodes
    }
    pub fn covered_scopes(&self) -> &BTreeSet<String> {
        &self.covered_scopes
    }
    /// Root input cells assumed supplied by launch; no values/authentication claimed.
    pub fn required_inputs(&self) -> &BTreeSet<String> {
        &self.required_inputs
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "parallel-and-exceptional-availability-proofs",
            "complete-information-flow-and-access-authorization",
            "race-and-invalidation-proofs",
            "host-trust-and-atomic-runtime-enforcement",
            "full-benchmark-and-conformance-evidence",
        ]
    }
}

/// Run static payload checks, then conservative ordinary control/availability
/// analysis. Refusal can mean unproved, not necessarily language-invalid.
pub fn check_control(
    envelope: &DefinitionEnvelope,
    resources: &[Resource<'_>],
) -> Result<ControlReport> {
    let static_report = check(envelope, resources)?;
    let b = &envelope.document()["body"];
    let mut budget = Budget::default();
    let graph = Graph::build(b, &mut budget)?;
    let mut analysis = Analysis::new(b, graph, budget)?;
    let root = text(&b["root"])?;
    let required_inputs = object(&b["scopes"][root]["inputs"])?
        .values()
        .map(|v| text(v).map(str::to_owned))
        .collect::<Result<_>>()?;
    analysis.scope(root, &BTreeSet::new(), 0)?;
    let uncovered_nodes = object(&b["nodes"])?
        .keys()
        .filter(|id| !analysis.nodes.contains(*id))
        .cloned()
        .collect();
    Ok(ControlReport {
        static_report,
        covered_nodes: analysis.nodes,
        uncovered_nodes,
        covered_scopes: analysis.scopes,
        required_inputs,
    })
}

pub(crate) struct EntryCoverage {
    pub nodes: BTreeSet<String>,
    pub scopes: BTreeSet<String>,
}
/// Reuse one bounded graph for conditional body entries. These sets are explicit
/// premises (possibly empty), not invented current runtime snapshots.
pub(crate) fn check_entries(
    b: &Value,
    entries: &BTreeMap<String, BTreeSet<String>>,
    mut budget: Budget,
    availability_code: &'static str,
) -> Result<EntryCoverage> {
    if entries.is_empty() {
        return Ok(EntryCoverage {
            nodes: BTreeSet::new(),
            scopes: BTreeSet::new(),
        });
    }
    let graph = Graph::build(b, &mut budget)?;
    let mut analysis = Analysis::new(b, graph, budget)?;
    for (entry, inputs) in entries {
        analysis.scope(entry, inputs, 0).map_err(|mut e| {
            if matches!(
                e.code,
                "data-availability-unproved"
                    | "initial-access-availability-unproved"
                    | "initializer-availability-unproved"
                    | "output-availability-unproved"
            ) {
                e.code = availability_code;
            }
            e
        })?;
    }
    Ok(EntryCoverage {
        nodes: analysis.nodes,
        scopes: analysis.scopes,
    })
}

fn at(code: &'static str, map: &str, id: &str) -> Diagnostic {
    Diagnostic {
        code,
        path: format!("/body/{map}/{id}"),
    }
}
