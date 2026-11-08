// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Conditional surviving body availability, without inherited value assumptions.
use super::*;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReconciliationEntry {
    owner: String,
    closure: String,
    followup: String,
}
impl ReconciliationEntry {
    pub fn owner(&self) -> &str {
        &self.owner
    }
    pub fn closure(&self) -> &str {
        &self.closure
    }
    pub fn followup(&self) -> &str {
        &self.followup
    }
    pub fn host_requirements(&self) -> &'static [&'static str] {
        &[
            "successful-correlated-obligation-registration-and-followup-activation",
            "create-followup-before-current-owner-disposal",
            "live-required-storage-and-fresh-local-initialization",
            "current-read-write-policy-and-management-authority",
            "accepted-authorized-atomic-handoff-and-subscription-ownership",
            "exact-request-key-observation-correlation-and-owned-settlement",
        ]
    }
}

#[derive(Debug)]
pub struct ReconciliationEntryReport {
    links: ReconciliationReport,
    entries: BTreeSet<ReconciliationEntry>,
    nodes: BTreeSet<String>,
    scopes: BTreeSet<String>,
    uncovered: BTreeSet<String>,
    retained_request_scopes: BTreeSet<String>,
}
impl ReconciliationEntryReport {
    pub fn revision(&self) -> &str {
        self.links.revision()
    }
    pub fn links(&self) -> &ReconciliationReport {
        &self.links
    }
    /// Declared conditional entries, not observed occurrences or handoffs.
    pub fn entries(&self) -> &BTreeSet<ReconciliationEntry> {
        &self.entries
    }
    /// Body data availability with zero inherited cells and fresh locals only.
    pub fn checked_nodes(&self) -> &BTreeSet<String> {
        &self.nodes
    }
    pub fn checked_scopes(&self) -> &BTreeSet<String> {
        &self.scopes
    }
    /// Owned request proof remains in the prior concurrency report, separate from
    /// legacy fresh-local reconciliation coverage. These are proven scope IDs.
    pub fn retained_request_scopes(&self) -> &BTreeSet<String> {
        &self.retained_request_scopes
    }
    /// Remaining after ordinary, handler and legacy reconciliation bodies.
    pub fn uncovered_nodes(&self) -> &BTreeSet<String> {
        &self.uncovered
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "occurrence-bound-ordinary-closure-and-snapshot-enforcement",
            "all-prior-reconciliation-declaration-and-host-obligations",
            "actual-followup-activation-and-live-storage-lifetime",
            "richer-ancestor-retention-snapshot-and-value-provenance-proofs",
            "exceptional-and-reconciliation-protection-invalidation-concurrency",
            "current-authority-accepted-handoff-and-atomic-owned-settlement",
            "provider-truth-and-exact-obligation-observation-correlation",
            "full-validator-benchmark-cross-form-and-user-evidence",
        ]
    }
}

pub fn check_reconciliation_availability(
    envelope: &DefinitionEnvelope,
    supplied: &[Resource<'_>],
) -> Result<ReconciliationEntryReport> {
    let links = check_reconciliation_links(envelope, supplied)?;
    let b = &envelope.document()["body"];
    let mut budget = Budget::default();
    let mut entries = BTreeSet::new();
    let mut assumptions = BTreeMap::new();
    let concurrency = links.exceptional().faults().timing().concurrency();
    let mut retained_request_scopes = BTreeSet::new();
    if !concurrency
        .invalidation()
        .protection()
        .control()
        .static_report()
        .retained_requests()
        .is_empty()
        && let Some(owned) = concurrency.owned_local()
    {
        for row in owned.followups() {
            budget.tick(0)?;
            if row.request_snapshot().is_some() {
                budget.charge(1)?;
                retained_request_scopes.insert(row.followup().into());
            }
        }
    }
    for (owner, scope) in object(&b["scopes"])? {
        budget.tick(0)?;
        let closure = text(&scope["closure"])?;
        let p = &b["policies"][closure]["body"];
        if p["mode"] != "reconcile" {
            continue;
        }
        let followup = text(&p["followup"])?;
        budget.charge(4)?;
        entries.insert(ReconciliationEntry {
            owner: owner.clone(),
            closure: closure.into(),
            followup: followup.into(),
        });
        // P6 has no follow-up ports. A surviving boundary does not establish
        // retention of any ancestor input, initializer, result or prior local.
        // Its existing owned proof already checked the explicit immutable seed;
        // legacy availability must neither erase it nor invent another one.
        if !retained_request_scopes.contains(followup) {
            assumptions.insert(followup.into(), BTreeSet::new());
        }
    }
    let coverage = control::check_entries(
        b,
        &assumptions,
        budget,
        "reconciliation-availability-unproved",
    )?;
    let uncovered = links
        .exceptional()
        .uncovered_nodes()
        .difference(&coverage.nodes)
        .cloned()
        .collect();
    Ok(ReconciliationEntryReport {
        links,
        entries,
        nodes: coverage.nodes,
        scopes: coverage.scopes,
        uncovered,
        retained_request_scopes,
    })
}
