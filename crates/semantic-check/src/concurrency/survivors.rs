// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use crate::control::graph::Graph;

/// Declared isolation only, conditional on live P6 enforcement. No occurrence
/// owner, accepted transfer or actual terminal settlement is asserted.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IsolatedFollowup {
    creator: String,
    body: String,
    closure: String,
    followup: String,
    waits: BTreeSet<String>,
}

impl IsolatedFollowup {
    pub fn creator(&self) -> &str {
        &self.creator
    }
    pub fn body(&self) -> &str {
        &self.body
    }
    pub fn closure(&self) -> &str {
        &self.closure
    }
    pub fn followup(&self) -> &str {
        &self.followup
    }
    pub fn waits(&self) -> &BTreeSet<String> {
        &self.waits
    }
    pub fn host_requirements(&self) -> &'static [&'static str] {
        &[
            "occurrence-bound-followup-created-before-owner-disposal",
            "exact-authenticated-obligation-request-and-observation-correlation",
            "fresh-authority-and-accepted-atomic-single-owner-handoff",
            "retain-all-transitive-obligations-and-subscriptions-until-settled",
            "durable-late-fact-deduplication-and-contradiction-handling",
            "all-owned-obligations-settled-before-terminal-disposal",
        ]
    }
}

struct Proof {
    closure: String,
    followup: String,
    waits: BTreeSet<String>,
}
pub(super) struct Index {
    graph: Graph,
    storage: BTreeSet<String>,
    parents: BTreeSet<String>,
    // Cache proof templates, never values, occurrence identity or authority.
    proofs: BTreeMap<String, Option<Proof>>,
}
impl Index {
    pub fn build(b: &Value, budget: &mut Budget) -> Result<Self> {
        let graph = Graph::build(b, budget)?;
        let mut storage = BTreeSet::new();
        let mut parents = BTreeSet::new();
        for d in object(&b["data"])?.values() {
            budget.tick(0)?;
            storage.insert(text(&d["scope"])?.to_owned());
        }
        for s in object(&b["scopes"])?.values() {
            budget.tick(0)?;
            if let Some(parent) = s["parent"].as_str() {
                parents.insert(parent.to_owned());
            }
        }
        Ok(Self {
            graph,
            storage,
            parents,
            proofs: BTreeMap::new(),
        })
    }
    pub(super) fn graph(&self) -> &Graph {
        &self.graph
    }
    pub(super) fn has_storage_followup(&self, b: &Value, body: &str) -> Result<bool> {
        let closure = text(&b["scopes"][body]["closure"])?;
        let p = &b["policies"][closure]["body"];
        Ok(p["mode"] == "reconcile" && self.storage.contains(text(&p["followup"])?))
    }
    pub fn check(
        &mut self,
        b: &Value,
        body: &str,
        creator: &str,
        budget: &mut Budget,
    ) -> Result<IsolatedFollowup> {
        if !self.proofs.contains_key(body) {
            let proof = self.prove(b, body, budget)?;
            self.proofs.insert(body.to_owned(), proof);
        }
        let p = self.proofs[body]
            .as_ref()
            .ok_or_else(|| at("scope-lifetime-unproved", creator))?;
        budget.charge(p.waits.len() + 5)?;
        Ok(IsolatedFollowup {
            creator: creator.into(),
            body: body.into(),
            closure: p.closure.clone(),
            followup: p.followup.clone(),
            waits: p.waits.clone(),
        })
    }
    fn prove(&self, b: &Value, body: &str, budget: &mut Budget) -> Result<Option<Proof>> {
        budget.tick(0)?;
        let closure = text(&b["scopes"][body]["closure"])?;
        let p = &b["policies"][closure]["body"];
        if p["mode"] != "reconcile" || !object(&p["reads"])?.is_empty() {
            return Ok(None);
        }
        let followup = text(&p["followup"])?;
        let s = &b["scopes"][followup];
        if s["parent"] != body
            || self.storage.contains(followup)
            || self.parents.contains(followup)
            || !object(&s["inputs"])?.is_empty()
            || !object(&s["outputs"])?.is_empty()
            || b["policies"][text(&s["closure"])?]["body"]["mode"] != "terminal"
        {
            return Ok(None);
        }
        for field in ["cancellation", "faults", "closure", "race"] {
            budget.tick(0)?;
            if !object(&b["policies"][text(&s[field])?]["body"]["reads"])?.is_empty() {
                return Ok(None);
            }
        }
        let entry = text(&s["entry"])?;
        if b["nodes"][entry]["kind"] != "wait" {
            return Ok(None);
        }
        let mut reached = BTreeSet::new();
        let mut waits = BTreeSet::new();
        let mut finish = false;
        for id in &self.graph.order[followup] {
            budget.tick(0)?;
            let n = &b["nodes"][id];
            // Check even unvisited declarations; no hypothetical guarded pruning.
            if !object(&n["reads"])?.is_empty() || !object(&n["writes"])?.is_empty() {
                return Ok(None);
            }
            let kind = text(&n["kind"])?;
            if !matches!(kind, "wait" | "finish") {
                return Ok(None);
            }
            if kind == "wait" {
                let wait = &b["policies"][text(&n["policy"])?]["body"];
                if !wait["timer"].is_null() || !object(&wait["reads"])?.is_empty() {
                    return Ok(None);
                }
            }
            let mut possible = id == entry;
            for prior in &self.graph.predecessors[id] {
                budget.tick(0)?;
                possible |= reached.contains(prior);
            }
            if possible {
                reached.insert(id.clone());
                if kind == "wait" {
                    waits.insert(id.clone());
                } else {
                    finish = true;
                }
            }
        }
        if !finish || waits.is_empty() {
            return Ok(None);
        }
        Ok(Some(Proof {
            closure: closure.into(),
            followup: followup.into(),
            waits,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn indexing_and_cached_evidence_copying_charge_the_same_total_budget() {
        let v: Value = serde_json::from_slice(include_bytes!(
            "../../../../docs/benchmarks/rp01/candidate.ir.json"
        ))
        .unwrap();
        let b = &v["body"];
        let mut spent = Budget { work: 1_000_000 };
        assert!(matches!(Index::build(b, &mut spent), Err(e) if e.code == "budget"));
        let mut budget = Budget::default();
        let mut index = Index::build(b, &mut budget).unwrap();
        index
            .check(b, "checks_service", "run_checks", &mut budget)
            .unwrap();
        let mut spent = Budget { work: 999_999 };
        assert_eq!(
            index
                .check(b, "checks_service", "second_call", &mut spent)
                .unwrap_err()
                .code,
            "budget"
        );
    }
}
