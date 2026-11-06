// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Surviving declaration links only, never live ownership or fact acceptance.
use super::*;
use crate::control::graph::Graph;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReconciliationUse {
    site: String,
    scope: String,
    capability: String,
    effects: String,
    closure: String,
    followup: String,
    wait_policy: String,
    wait_node: String,
}
impl ReconciliationUse {
    pub fn site(&self) -> &str {
        &self.site
    }
    pub fn scope(&self) -> &str {
        &self.scope
    }
    pub fn capability(&self) -> &str {
        &self.capability
    }
    pub fn effects(&self) -> &str {
        &self.effects
    }
    pub fn closure(&self) -> &str {
        &self.closure
    }
    pub fn followup(&self) -> &str {
        &self.followup
    }
    pub fn wait_policy(&self) -> &str {
        &self.wait_policy
    }
    pub fn wait_node(&self) -> &str {
        &self.wait_node
    }
    pub fn host_requirements(&self) -> &'static [&'static str] {
        &[
            "create-followup-before-disposing-the-current-obligation-owner",
            "live-followup-entry-snapshot-availability-and-current-authority",
            "exact-effect-request-input-and-observation-correlation",
            "accepted-authorized-atomic-obligation-and-subscription-handoff",
            "durable-late-fact-deduplication-and-conflict-reconciliation",
            "all-owned-obligations-settled-before-followup-terminal-closure",
        ]
    }
}
#[derive(Debug)]
pub struct ReconciliationReport {
    exceptional: ExceptionalReport,
    uses: BTreeSet<ReconciliationUse>,
    unvisited: BTreeSet<String>,
    unused_effects: BTreeSet<String>,
}
impl ReconciliationReport {
    pub fn revision(&self) -> &str {
        self.exceptional.revision()
    }
    pub fn exceptional(&self) -> &ExceptionalReport {
        &self.exceptional
    }
    pub fn uses(&self) -> &BTreeSet<ReconciliationUse> {
        &self.uses
    }
    /// Declaration links checked without claiming an ordinary live entry.
    pub fn ordinary_unvisited_sites(&self) -> &BTreeSet<String> {
        &self.unvisited
    }
    pub fn unused_effects(&self) -> &BTreeSet<String> {
        &self.unused_effects
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "occurrence-bound-ordinary-closure-and-snapshot-enforcement",
            "all-prior-exceptional-fault-route-and-timing-obligations",
            "reconciliation-entry-data-protection-and-invalidation",
            "live-obligation-subscription-and-occurrence-ownership",
            "current-authority-accepted-transfer-and-atomic-settlement",
            "exact-provider-truth-request-key-and-observation-correlation",
            "multiple-obligation-wait-progress-and-terminal-disposal",
            "full-validator-benchmark-cross-form-and-user-evidence",
        ]
    }
}
fn refusal(code: &'static str, map: &str, id: &str, field: &str) -> Diagnostic {
    Diagnostic {
        code,
        path: format!("/body/{map}/{id}{field}"),
    }
}
pub fn check_reconciliation_links(
    envelope: &DefinitionEnvelope,
    supplied: &[Resource<'_>],
) -> Result<ReconciliationReport> {
    let exceptional = check_exceptional_availability(envelope, supplied)?;
    let b = &envelope.document()["body"];
    let mut budget = Budget::default();
    let graph = Graph::build(b, &mut budget)?;
    let reachable = graph.reachable(b, &mut budget)?;
    let ordinary = exceptional
        .faults()
        .timing()
        .concurrency()
        .invalidation()
        .protection()
        .control()
        .covered_nodes();
    let mut wait_sites = BTreeMap::new();
    for (id, n) in object(&b["nodes"])? {
        budget.tick(0)?;
        if n["kind"] == "wait" {
            wait_sites
                .entry(text(&n["policy"])?)
                .or_insert_with(Vec::new)
                .push(id.as_str());
        }
    }
    let mut uses = BTreeSet::new();
    let mut used_effects = BTreeSet::new();
    let mut unvisited = BTreeSet::new();
    for (id, n) in object(&b["nodes"])? {
        budget.tick(0)?;
        if n["kind"] != "activity" || n["mode"] != "capability" {
            continue;
        }
        let scope = text(&n["scope"])?;
        let capability = text(&n["target"])?;
        let effects = text(&b["capabilities"][capability]["effects"])?;
        let wait_policy = text(&b["policies"][effects]["body"]["reconciliation"])?;
        let sites = wait_sites.get(wait_policy).ok_or_else(|| {
            refusal(
                "reconciliation-owner-missing",
                "policies",
                effects,
                "/body/reconciliation",
            )
        })?;
        if sites.len() != 1 {
            return Err(refusal(
                "reconciliation-owner-unproved",
                "policies",
                effects,
                "/body/reconciliation",
            ));
        }
        let wait_node = sites[0];
        let closure = text(&b["scopes"][scope]["closure"])?;
        let p = &b["policies"][closure]["body"];
        if p["mode"] != "reconcile" {
            return Err(refusal(
                "reconciliation-followup-unproved",
                "scopes",
                scope,
                "/closure",
            ));
        }
        let followup = text(&p["followup"])?;
        if b["nodes"][wait_node]["scope"] != followup {
            return Err(refusal(
                "reconciliation-context-unproved",
                "policies",
                effects,
                "/body/reconciliation",
            ));
        }
        if !reachable
            .get(followup)
            .is_some_and(|nodes| nodes.contains(wait_node))
        {
            return Err(refusal(
                "reconciliation-route-unproved",
                "nodes",
                wait_node,
                "",
            ));
        }
        budget.charge(8)?;
        uses.insert(ReconciliationUse {
            site: id.clone(),
            scope: scope.into(),
            capability: capability.into(),
            effects: effects.into(),
            closure: closure.into(),
            followup: followup.into(),
            wait_policy: wait_policy.into(),
            wait_node: wait_node.into(),
        });
        used_effects.insert(effects.to_owned());
        if !ordinary.contains(id) {
            unvisited.insert(id.clone());
        }
    }
    let mut unused_effects = BTreeSet::new();
    for (id, p) in object(&b["policies"])? {
        budget.tick(0)?;
        if p["body"]["kind"] == "effect" && !used_effects.contains(id) {
            unused_effects.insert(id.clone());
        }
    }
    Ok(ReconciliationReport {
        exceptional,
        uses,
        unvisited,
        unused_effects,
    })
}
