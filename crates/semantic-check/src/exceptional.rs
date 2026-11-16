// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Separate legacy ancestor-input and explicit conditional history-body evidence.
use super::*;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};
use std::sync::Arc;

#[derive(Debug)]
pub struct ExceptionalReport {
    faults: Arc<FaultReport>,
    history_entries: Option<FaultEntryReport>,
    assumptions: BTreeMap<String, BTreeSet<String>>,
    nodes: BTreeSet<String>,
    scopes: BTreeSet<String>,
    uncovered: BTreeSet<String>,
}
impl ExceptionalReport {
    pub fn revision(&self) -> &str {
        self.faults.revision()
    }
    pub fn faults(&self) -> &FaultReport {
        &self.faults
    }
    /// Separate history initialization/flow proof; no legacy coverage promotion.
    pub fn history_entries(&self) -> Option<&FaultEntryReport> {
        self.history_entries.as_ref()
    }
    /// Required bound input cells of live owning ancestors, not observed facts.
    pub fn entry_assumptions(&self) -> &BTreeMap<String, BTreeSet<String>> {
        &self.assumptions
    }
    /// Conditional handler body and ordinary child proof, not fault-control flow.
    pub fn checked_nodes(&self) -> &BTreeSet<String> {
        &self.nodes
    }
    pub fn checked_scopes(&self) -> &BTreeSet<String> {
        &self.scopes
    }
    pub fn uncovered_nodes(&self) -> &BTreeSet<String> {
        &self.uncovered
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "all-prior-fault-route-and-timing-obligations",
            "live-ancestor-input-revisions-and-successful-activation-premises",
            "fault-site-specific-snapshot-and-reconciliation-entry-proofs",
            "exceptional-fault-control-protection-and-invalidation",
            "current-principal-purpose-capability-and-policy-enforcement",
            "concurrent-handler-occurrence-lifetime-and-owned-settlement",
            "occurrence-bound-ordinary-closure-and-snapshot-enforcement",
            "host-trust-and-atomic-runtime-enforcement",
            "full-benchmark-conformance-cross-form-and-user-evidence",
        ]
    }
}

pub fn check_exceptional_availability(
    envelope: &DefinitionEnvelope,
    supplied: &[Resource<'_>],
) -> Result<ExceptionalReport> {
    let b = &envelope.document()["body"];
    let has_history = b["scopes"]
        .as_object()
        .is_some_and(|scopes| scopes.values().any(|s| !s["faultInput"].is_null()));
    let history_entries = if has_history {
        Some(check_fault_entries(envelope, supplied)?)
    } else {
        None
    };
    let faults = if let Some(entries) = &history_entries {
        entries.origins().shared_faults()
    } else {
        Arc::new(check_fault_routes(envelope, supplied)?)
    };
    let mut budget = Budget::default();
    let mut assumptions = BTreeMap::new();
    let typed: BTreeSet<_> = faults
        .static_report()
        .fault_entries()
        .iter()
        .map(|r| r.handler())
        .collect();
    budget.charge(typed.len())?;
    for selection in faults.covered_handlers() {
        budget.tick(0)?;
        let handler = selection.handler();
        if typed.contains(handler) || assumptions.contains_key(handler) {
            continue;
        }
        let mut inputs = BTreeSet::new();
        let mut cursor = b["scopes"][handler]["parent"].as_str();
        let mut depth = 0;
        while let Some(owner) = cursor {
            budget.tick(depth)?;
            depth += 1;
            for cell in object(&b["scopes"][owner]["inputs"])?.values() {
                budget.tick(0)?;
                inputs.insert(text(cell)?.to_owned());
            }
            cursor = b["scopes"][owner]["parent"].as_str();
        }
        budget.charge(inputs.len() + 1)?;
        assumptions.insert(handler.to_owned(), inputs);
    }
    let coverage =
        control::check_entries(b, &assumptions, budget, "exceptional-availability-unproved")?;
    let ordinary = faults
        .timing()
        .concurrency()
        .invalidation()
        .protection()
        .control()
        .covered_nodes();
    let uncovered = object(&b["nodes"])?
        .keys()
        .filter(|id| !ordinary.contains(*id) && !coverage.nodes.contains(*id))
        .cloned()
        .collect();
    Ok(ExceptionalReport {
        faults,
        history_entries,
        assumptions,
        nodes: coverage.nodes,
        scopes: coverage.scopes,
        uncovered,
    })
}
