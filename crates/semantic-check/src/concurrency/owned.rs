// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Conditional fresh-local isolation; never allocated storage or live ownership.
use super::*;
use crate::control::graph::Graph;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Template {
    body: String,
    closure: String,
    followup: String,
    locals: BTreeSet<String>,
    captures: BTreeSet<String>,
    request: Option<OwnedRequestSnapshot>,
}

/// Exact original request relation under the conditional owned proof, never a value.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnedRequestSnapshot {
    origin: String,
    result: String,
}
impl OwnedRequestSnapshot {
    pub fn origin(&self) -> &str {
        &self.origin
    }
    pub fn result(&self) -> &str {
        &self.result
    }
}
/// A declared creator/template relation, conditional on occurrence-bound P6.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnedLocalFollowup {
    creator: String,
    template: Arc<Template>,
}
impl OwnedLocalFollowup {
    pub fn creator(&self) -> &str {
        &self.creator
    }
    pub fn body(&self) -> &str {
        &self.template.body
    }
    pub fn closure(&self) -> &str {
        &self.template.closure
    }
    pub fn followup(&self) -> &str {
        &self.template.followup
    }
    pub fn local_cells(&self) -> &BTreeSet<String> {
        &self.template.locals
    }
    pub fn capture_sites(&self) -> &BTreeSet<String> {
        &self.template.captures
    }
    pub fn request_snapshot(&self) -> Option<&OwnedRequestSnapshot> {
        self.template.request.as_ref()
    }
    pub fn host_requirements(&self) -> &'static [&'static str] {
        &[
            "occurrence-bound-followup-and-fresh-local-storage-created-before-owner-disposal",
            "zero-inherited-values-and-no-sharing-between-creator-occurrences",
            "authenticated-occurrence-bound-source-control-protection-provenance-retained-through-handoff",
            "exact-authenticated-obligation-attempt-request-key-input-observation-correlation",
            "current-access-authority-and-accepted-atomic-single-owner-handoff",
            "single-accepted-capture-and-duplicate-stale-contradictory-input-history",
            "retain-all-owned-storage-subscriptions-and-obligations-until-settled",
            "success-cancellation-closure-cutoffs-and-atomic-all-obligation-settlement",
            "observable-channel-protection-and-actual-host-enforcement",
        ]
    }
}

/// Explicit conditional coverage, separate from ordinary checked/uncovered sets.
#[derive(Debug)]
pub struct OwnedLocalReport {
    revision: String,
    followups: BTreeSet<OwnedLocalFollowup>,
    nodes: BTreeSet<String>,
    scopes: BTreeSet<String>,
    transfers: usize,
    accesses: BTreeSet<AccessObligation>,
    obligations: BTreeSet<InvalidationObligation>,
    provenance: BTreeSet<crate::protection::RequiredProtectionProvenance>,
}
impl OwnedLocalReport {
    pub fn revision(&self) -> &str {
        &self.revision
    }
    pub fn followups(&self) -> &BTreeSet<OwnedLocalFollowup> {
        &self.followups
    }
    pub fn checked_nodes(&self) -> &BTreeSet<String> {
        &self.nodes
    }
    pub fn checked_scopes(&self) -> &BTreeSet<String> {
        &self.scopes
    }
    pub fn transfers(&self) -> usize {
        self.transfers
    }
    pub fn accesses(&self) -> &BTreeSet<AccessObligation> {
        &self.accesses
    }
    pub fn obligations(&self) -> &BTreeSet<InvalidationObligation> {
        &self.obligations
    }
    pub fn required_provenance(
        &self,
    ) -> &BTreeSet<crate::protection::RequiredProtectionProvenance> {
        &self.provenance
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "conditional-entry-only-no-actual-followup-activation-or-owned-storage",
            "all-declared-live-P6-and-capture-host-requirements",
            "current-principal-purpose-capability-policy-and-management-enforcement",
            "observable-channel-and-general-exceptional-protection-invalidation-concurrency",
            "fault-data-owned-continuation-cutoff-and-all-obligation-settlement",
            "full-validator-benchmark-cross-form-user-and-independent-evidence",
        ]
    }
}

pub(super) struct Candidates {
    locals: BTreeMap<String, BTreeSet<String>>,
    parents: BTreeSet<String>,
    created: BTreeSet<String>,
    templates: BTreeMap<String, Arc<Template>>,
    followups: BTreeSet<OwnedLocalFollowup>,
}
impl Candidates {
    pub fn new(b: &Value, budget: &mut Budget) -> Result<Self> {
        let mut locals: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut parents = BTreeSet::new();
        let mut created = BTreeSet::new();
        for (id, d) in object(&b["data"])? {
            budget.charge(2)?;
            locals
                .entry(text(&d["scope"])?.into())
                .or_default()
                .insert(id.clone());
        }
        for s in object(&b["scopes"])?.values() {
            budget.tick(0)?;
            if let Some(parent) = s["parent"].as_str() {
                parents.insert(parent.into());
            }
        }
        for n in object(&b["nodes"])?.values() {
            budget.tick(0)?;
            if let Some(body) = n["body"].as_str() {
                created.insert(body.into());
            }
            if let Some(children) = n["children"].as_object() {
                budget.charge(children.len())?;
                for child in children.values() {
                    created.insert(text(child)?.into());
                }
            }
        }
        Ok(Self {
            locals,
            parents,
            created,
            templates: BTreeMap::new(),
            followups: BTreeSet::new(),
        })
    }
    pub fn consider(
        &mut self,
        b: &Value,
        graph: &Graph,
        body: &str,
        creator: &str,
        budget: &mut Budget,
    ) -> Result<()> {
        if !self.templates.contains_key(body) {
            let template = self
                .prove(b, graph, body, budget)?
                .ok_or_else(|| at("scope-lifetime-unproved", creator))?;
            budget.charge(1)?;
            self.templates.insert(body.into(), Arc::new(template));
        }
        budget.charge(2)?;
        self.followups.insert(OwnedLocalFollowup {
            creator: creator.into(),
            template: self.templates[body].clone(),
        });
        Ok(())
    }
    fn prove(
        &self,
        b: &Value,
        graph: &Graph,
        body: &str,
        budget: &mut Budget,
    ) -> Result<Option<Template>> {
        budget.tick(0)?;
        let closure = text(&b["scopes"][body]["closure"])?;
        let policy = &b["policies"][closure]["body"];
        if policy["mode"] != "reconcile" || !object(&policy["reads"])?.is_empty() {
            return Ok(None);
        }
        let followup = text(&policy["followup"])?;
        let s = &b["scopes"][followup];
        if s["parent"] != body
            || self.parents.contains(followup)
            || self.created.contains(followup)
            || !object(&s["inputs"])?.is_empty()
            || !object(&s["outputs"])?.is_empty()
            || b["policies"][text(&s["closure"])?]["body"]["mode"] != "terminal"
        {
            return Ok(None);
        }
        let Some(locals) = self.locals.get(followup) else {
            return Ok(None);
        };
        let entry = text(&s["entry"])?;
        if b["nodes"][entry]["kind"] != "wait" || b["nodes"][entry]["capture"].is_null() {
            return Ok(None);
        }
        let request =
            if let Some(bindings) = s["retainedRequests"].as_object().filter(|m| !m.is_empty()) {
                budget.charge(3)?;
                let (result, origin) = bindings
                    .iter()
                    .next()
                    .ok_or_else(|| err("retained-request-cardinality"))?;
                Some(OwnedRequestSnapshot {
                    origin: text(origin)?.into(),
                    result: result.clone(),
                })
            } else {
                None
            };
        let mut reached = BTreeSet::new();
        let mut captures = BTreeSet::new();
        let mut finish = false;
        for id in &graph.order[followup] {
            budget.tick(0)?;
            let n = &b["nodes"][id];
            let kind = text(&n["kind"])?;
            // Inspect all declarations, even ordinary/conditionally unvisited ones.
            if !matches!(kind, "wait" | "compute" | "decision" | "finish") {
                return Ok(None);
            }
            for field in ["reads", "writes"] {
                budget.charge(object(&n[field])?.len() + 1)?;
                if object(&n[field])?.keys().any(|cell| !locals.contains(cell)) {
                    return Ok(None);
                }
            }
            if kind == "wait" {
                let wait = &b["policies"][text(&n["policy"])?]["body"];
                if !wait["timer"].is_null() {
                    return Ok(None);
                }
                if !n["capture"].is_null() {
                    budget.tick(0)?;
                    captures.insert(id.clone());
                }
            }
            let mut possible = id == entry;
            for prior in &graph.predecessors[id] {
                budget.tick(0)?;
                possible |= reached.contains(prior);
            }
            if possible {
                reached.insert(id.clone());
                finish |= kind == "finish";
            }
        }
        if !finish {
            return Ok(None);
        }
        budget.charge(locals.len() + captures.len() + 3)?;
        Ok(Some(Template {
            body: body.into(),
            closure: closure.into(),
            followup: followup.into(),
            locals: locals.clone(),
            captures,
            request,
        }))
    }
    pub fn check(
        self,
        envelope: &DefinitionEnvelope,
        supplied: &[Resource<'_>],
        ordinary: &ProtectionReport,
        budget: &mut Budget,
    ) -> Result<OwnedLocalReport> {
        let b = &envelope.document()["body"];
        let mut entries = BTreeMap::new();
        let mut requests = BTreeMap::new();
        let mut request_sources = BTreeMap::new();
        for p in self.templates.values() {
            budget.tick(0)?;
            entries.insert(p.followup.clone(), BTreeSet::new());
            if let Some(request) = &p.request {
                if !ordinary.control().covered_nodes().contains(&request.origin) {
                    return Err(at(
                        "retained-request-source-control-unproved",
                        &request.origin,
                    ));
                }
                budget.charge(5)?;
                requests.insert(p.followup.clone(), request.result.clone());
                request_sources.insert(
                    p.followup.clone(),
                    (request.result.clone(), request.origin.clone()),
                );
            }
        }
        // One cumulative analysis per stage for all entries, not per creator.
        let coverage = crate::control::check_entries_with_requests(
            b,
            &entries,
            &requests,
            Budget::default(),
            "owned-local-availability-unproved",
        )?;
        let mut owners: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for p in self.templates.values() {
            budget.charge(2)?;
            owners
                .entry(p.followup.clone())
                .or_default()
                .insert(p.body.clone());
        }
        let analysis =
            crate::protection::analyze_entries(b, supplied, &owners, ordinary, &request_sources)?;
        let obligations =
            crate::invalidation::require_entries(b, &analysis.dependencies.nodes, budget)?;
        budget.charge(
            coverage.nodes.len()
                + coverage.scopes.len()
                + analysis.accesses.len()
                + obligations.len()
                + analysis.provenance.len()
                + 1,
        )?;
        Ok(OwnedLocalReport {
            revision: text(&envelope.document()["revision"])?.into(),
            followups: self.followups,
            nodes: coverage.nodes,
            scopes: coverage.scopes,
            transfers: analysis.transfers,
            accesses: analysis.accesses,
            provenance: analysis.provenance,
            obligations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn indexing_and_cached_template_rows_use_the_existing_total_budget() {
        let document: Value = serde_json::from_slice(include_bytes!(
            "../../../../docs/benchmarks/rp01/history/source-0.7/candidate.ir.json"
        ))
        .unwrap();
        let mut b = document["body"].clone();
        // This private index test supplies declaration roles only; actual source
        // resource admission and public reports are exercised by integration tests.
        let mut cell = b["data"]["payment_result"].clone();
        cell["scope"] = "payment_recovery".into();
        b["data"]["local_capture"] = cell;
        b["nodes"]["payment_observe"]["capture"] = serde_json::json!({"source":{"id":"urn:test:index-only","revision":format!("sha256:{}","0".repeat(64))},"origin":"payment_request_step","result":"local_capture"});
        b["nodes"]["payment_observe"]["writes"] = serde_json::json!({"local_capture":true});
        let mut exhausted = Budget { work: 1_000_000 };
        assert!(matches!(Candidates::new(&b,&mut exhausted),Err(e) if e.code=="budget"));
        let mut budget = Budget::default();
        let graph = Graph::build(&b, &mut budget).unwrap();
        let mut c = Candidates::new(&b, &mut budget).unwrap();
        c.consider(&b, &graph, "payment_service", "run_payment", &mut budget)
            .unwrap();
        c.consider(
            &b,
            &graph,
            "payment_service",
            "another_declared_creator",
            &mut budget,
        )
        .unwrap();
        let rows: Vec<_> = c.followups.iter().collect();
        assert!(Arc::ptr_eq(&rows[0].template, &rows[1].template));
        let mut exhausted = Budget { work: 999_999 };
        assert_eq!(
            c.consider(
                &b,
                &graph,
                "payment_service",
                "third_creator",
                &mut exhausted
            )
            .unwrap_err()
            .code,
            "budget"
        );
        assert_eq!(c.followups.len(), 2);
    }
}
