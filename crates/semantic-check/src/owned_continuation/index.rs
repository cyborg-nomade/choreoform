// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use crate::control::graph::Graph;
pub(crate) struct Candidates<'a> {
    b: &'a Value,
    children: BTreeMap<&'a str, BTreeSet<&'a str>>,
    locals: BTreeMap<&'a str, BTreeSet<&'a str>>,
    created: BTreeSet<&'a str>,
    handlers: BTreeSet<&'a str>,
    templates: BTreeMap<String, Arc<Template>>,
    roots: BTreeSet<OwnedContinuation>,
    source_waits: Option<crate::source_wait::SourceWaits<'a>>,
    finite_work: bool,
}

impl<'a> Candidates<'a> {
    pub fn new(b: &'a Value, budget: &mut Budget) -> Result<Self> {
        let mut children: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        let mut locals: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        let mut created = BTreeSet::new();
        let mut handlers = BTreeSet::new();
        for (id, s) in object(&b["scopes"])? {
            budget.tick(0)?;
            if let Some(parent) = s["parent"].as_str() {
                budget.charge(2)?;
                children.entry(parent).or_default().insert(id);
            }
            for target in object(&b["policies"][text(&s["faults"])?]["body"]["handlers"])?.values()
            {
                budget.tick(0)?;
                handlers.insert(text(target)?);
            }
        }
        for (id, d) in object(&b["data"])? {
            budget.charge(2)?;
            locals.entry(text(&d["scope"])?).or_default().insert(id);
        }
        for n in object(&b["nodes"])?.values() {
            budget.tick(0)?;
            if matches!(n["kind"].as_str(), Some("invoke" | "repeat" | "fanout")) {
                created.insert(text(&n["body"])?);
            }
            if n["kind"] == "split" {
                for target in object(&n["children"])?.values() {
                    budget.tick(0)?;
                    created.insert(text(target)?);
                }
            }
        }
        Ok(Self {
            b,
            children,
            locals,
            created,
            handlers,
            templates: BTreeMap::new(),
            roots: BTreeSet::new(),
            finite_work: work_core(b),
            source_waits: if pre_capture_core(b) {
                Some(crate::source_wait::SourceWaits::new(b, budget)?)
            } else {
                None
            },
        })
    }
    pub fn consider(
        &mut self,
        graph: &Graph,
        owner: &str,
        creator: &str,
        budget: &mut Budget,
    ) -> Result<()> {
        if !self.templates.contains_key(owner) {
            let t = self.prove(graph, owner, budget)?;
            budget.charge(1)?;
            self.templates.insert(owner.into(), Arc::new(t));
        }
        budget.charge(2)?;
        self.roots.insert(OwnedContinuation {
            creator: creator.into(),
            template: self.templates[owner].clone(),
        });
        Ok(())
    }
    fn prove(&self, graph: &Graph, root: &str, budget: &mut Budget) -> Result<Template> {
        let b = self.b;
        let mut owner = root;
        let mut levels = Vec::new();
        let mut expected_source: Option<&str> = None;
        let root_child =
            text(&b["policies"][text(&b["scopes"][root]["closure"])?]["body"]["followup"])?;
        require(
            self.children
                .get(root)
                .is_some_and(|children| children.len() == 1 && children.contains(root_child)),
            "owned-continuation-root-child-unproved",
        )
        .map_err(|e| e.at_path(format!("/body/scopes/{root}")))?;
        // The ordinary owner has one external activity and no hidden ordinary work.
        let mut ordinary_source = None;
        for id in &graph.order[root] {
            budget.tick(0)?;
            let n = &b["nodes"][id];
            for field in ["reads", "writes"] {
                budget.charge(object(&n[field])?.len() + 1)?;
                if object(&n[field])?
                    .keys()
                    .any(|cell| b["data"][cell]["scope"] != root)
                {
                    return Err(at("owned-continuation-root-storage-unproved", root));
                }
            }
            match text(&n["kind"])? {
                "compute" | "decision" | "finish" => {}
                "activity" if n["mode"] == "capability" && ordinary_source.is_none() => {
                    ordinary_source = Some(id.as_str())
                }
                _ => return Err(at("owned-continuation-root-unproved", root)),
            }
        }
        let ordinary_source =
            ordinary_source.ok_or_else(|| at("owned-continuation-root-unproved", root))?;
        loop {
            budget.tick(levels.len())?;
            let closure = text(&b["scopes"][owner]["closure"])?;
            let parent_policy = &b["policies"][closure]["body"];
            require(
                parent_policy["mode"] == "reconcile",
                "owned-continuation-child-unproved",
            )?;
            let scope = text(&parent_policy["followup"])?;
            let s = &b["scopes"][scope];
            if s["parent"] != owner
                || self.created.contains(scope)
                || self.handlers.contains(scope)
                || !object(&s["inputs"])?.is_empty()
                || !object(&s["outputs"])?.is_empty()
                || !s["faultInput"].is_null()
            {
                return Err(at("owned-continuation-storage-unproved", scope));
            }
            let requests = object(&s["retainedRequests"])?;
            require(requests.len() == 1, "owned-continuation-request-unproved")
                .map_err(|e| e.at_path(format!("/body/scopes/{scope}/retainedRequests")))?;
            let (request, source) = requests
                .iter()
                .next()
                .ok_or_else(|| at("owned-continuation-request-unproved", scope))?;
            let source = text(source)?;
            require(
                source == expected_source.unwrap_or(ordinary_source),
                "owned-continuation-source-unproved",
            )
            .map_err(|e| e.at_path(format!("/body/scopes/{scope}/retainedRequests")))?;
            let entry = text(&s["entry"])?;
            let e = &b["nodes"][entry];
            let capturing_entry = e["kind"] == "wait" && e["capture"]["origin"] == source;
            let source_wait = if capturing_entry {
                entry
            } else {
                require(
                    self.source_waits.is_some()
                        && b["policies"][text(&s["closure"])?]["body"]["mode"] == "reconcile",
                    "owned-continuation-entry-unproved",
                )?;
                self.source_waits
                    .as_ref()
                    .unwrap()
                    .capture(scope, source, budget)?
            };
            let pre_capture = if capturing_entry {
                None
            } else {
                budget.charge(3)?;
                Some(OwnedPreCaptureEntry {
                    entry: entry.into(),
                    capture_wait: source_wait.into(),
                })
            };
            let local = self
                .locals
                .get(scope)
                .ok_or_else(|| at("owned-continuation-storage-unproved", scope))?;
            let own_closure = &b["policies"][text(&s["closure"])?]["body"];
            let child = if own_closure["mode"] == "reconcile" {
                Some(text(&own_closure["followup"])?)
            } else {
                None
            };
            if let Some(target) = child {
                if !self
                    .children
                    .get(scope)
                    .is_some_and(|cs| cs.len() == 1 && cs.contains(target))
                {
                    return Err(at("owned-continuation-child-unproved", scope));
                }
            } else if self.children.get(scope).is_some_and(|cs| !cs.is_empty()) {
                return Err(at("owned-continuation-child-unproved", scope));
            }
            let mut query = None;
            let mut activity = None;
            let mut work = None;
            let mut reached = BTreeSet::new();
            let mut finish = false;
            for id in &graph.order[scope] {
                budget.tick(0)?;
                let n = &b["nodes"][id];
                let kind = text(&n["kind"])?;
                for field in ["reads", "writes"] {
                    budget.charge(object(&n[field])?.len() + 1)?;
                    if object(&n[field])?
                        .keys()
                        .any(|c| !local.contains(c.as_str()))
                    {
                        return Err(at("owned-continuation-storage-unproved", scope));
                    }
                }
                match kind {
                    "compute" | "decision" | "finish" => {}
                    "wait" => {
                        if !b["policies"][text(&n["policy"])?]["body"]["timer"].is_null()
                            || (child.is_some() && id != source_wait)
                        {
                            return Err(at("owned-continuation-wait-unproved", scope));
                        }
                    }
                    "activity" if child.is_some() && activity.is_none() => {
                        if self.finite_work {
                            let declaration = super::work::check(b, id, n, scope, source, budget)?;
                            if declaration.is_query() {
                                query = Some(id.as_str());
                            } else {
                                require(capturing_entry, "owned-continuation-work-entry")?;
                                require(
                                    self.source_waits
                                        .as_ref()
                                        .unwrap()
                                        .capture(scope, source, budget)?
                                        == entry,
                                    "owned-continuation-work-entry",
                                )?;
                            }
                            work = Some(declaration);
                        } else {
                            self.query(n, request, scope, budget)?;
                            query = Some(id.as_str());
                        }
                        activity = Some(id.as_str());
                    }
                    _ => return Err(at("owned-continuation-body-unproved", scope)),
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
            if !finish || child.is_some() != activity.is_some() {
                return Err(at("owned-continuation-body-unproved", scope));
            }
            if pre_capture.is_some() && !reached.contains(source_wait) {
                return Err(at("owned-continuation-capture-unreachable", scope));
            }
            budget.charge(8)?;
            levels.push(Level {
                entry: Entry {
                    owner: owner.into(),
                    scope: scope.into(),
                    source: source.into(),
                    request: request.clone(),
                    conditional_source: expected_source.is_some(),
                },
                closure: closure.into(),
                query: query.map(str::to_owned),
                child: child.map(str::to_owned),
                pre_capture,
                work,
            });
            if child.is_none() {
                break;
            }
            owner = scope;
            expected_source = activity;
        }
        require(levels.len() > 1, "owned-continuation-body-unproved")?;
        Ok(Template {
            owner: root.into(),
            levels,
        })
    }
    fn query(&self, n: &Value, request: &str, scope: &str, budget: &mut Budget) -> Result<()> {
        let b = self.b;
        budget.tick(0)?;
        if n["mode"] != "capability" {
            return Err(at("owned-continuation-query-unproved", scope));
        }
        let cap = &b["capabilities"][text(&n["target"])?];
        let effect = &b["policies"][text(&cap["effects"])?]["body"];
        require(
            effect["class"] == "read-only",
            "owned-continuation-query-effect",
        )?;
        let p = &b["policies"][text(&n["policy"])?]["body"];
        let retry = &p["retry"];
        require(
            retry["maxAttempts"] == 1
                && p["timeout"].is_null()
                && p["compensates"].is_null()
                && !p["faultSource"].is_null(),
            "owned-continuation-query-policy",
        )?;
        let input = &b["expressions"][text(&n["input"])?]["body"];
        require(
            input["op"] == "read" && input["cell"] == request,
            "owned-continuation-query-input",
        )?;
        let reads = object(&n["reads"])?;
        budget.charge(reads.len())?;
        require(
            reads.len() == 1 && reads.contains_key(request),
            "owned-continuation-query-reads",
        )
    }
    pub fn check(
        self,
        envelope: &DefinitionEnvelope,
        supplied: &[Resource<'_>],
        ordinary: &ProtectionReport,
        budget: &mut Budget,
    ) -> Result<OwnedContinuationReport> {
        let entry_count = self.templates.values().try_fold(0usize, |count, t| {
            budget.tick(0)?;
            count
                .checked_add(t.levels.len())
                .ok_or_else(|| err("budget"))
        })?;
        budget.charge(entry_count)?;
        let entries: Vec<_> = self
            .templates
            .values()
            .flat_map(|t| t.levels.iter().map(|l| &l.entry))
            .collect();
        let coverage = crate::control::check_continuation_entries(
            self.b,
            &entries,
            ordinary.control(),
            Budget::default(),
        )?;
        let analysis =
            crate::protection::analyze_continuation_entries(self.b, supplied, &entries, ordinary)?;
        let obligations = crate::invalidation::require_entries_with_codes(
            self.b,
            &analysis.dependencies.nodes,
            budget,
            "owned-continuation-invalidation-coverage",
            "owned-continuation-invalidation-context-unproved",
        )?;
        let mut gates = BTreeSet::new();
        for t in self.templates.values() {
            for level in &t.levels {
                let e = &level.entry;
                for kind in ["registration", "handoff", "ordinary-closure"] {
                    budget.charge(6)?;
                    let managed = if kind == "handoff" {
                        &e.scope
                    } else {
                        &e.owner
                    };
                    let actor = text(
                        &self.b["policies"][text(&self.b["scopes"][managed]["cancellation"])?]["body"]
                            ["authority"],
                    )?;
                    gates.insert(OwnedContinuationGate {
                        kind,
                        owner: e.owner.clone(),
                        source: Some(e.source.clone()),
                        receiver: Some(e.scope.clone()),
                        management_actor: actor.into(),
                    });
                }
                if level.child.is_none() {
                    budget.charge(4)?;
                    let actor = text(
                        &self.b["policies"][text(&self.b["scopes"][&e.scope]["cancellation"])?]["body"]
                            ["authority"],
                    )?;
                    gates.insert(OwnedContinuationGate {
                        kind: "terminal-settlement",
                        owner: e.scope.clone(),
                        source: None,
                        receiver: None,
                        management_actor: actor.into(),
                    });
                }
            }
        }
        budget.charge(
            coverage.nodes.len()
                + coverage.scopes.len()
                + analysis.accesses.len()
                + analysis.provenance.len()
                + obligations.len()
                + self.roots.len(),
        )?;
        Ok(OwnedContinuationReport {
            revision: text(&envelope.document()["revision"])?.into(),
            continuations: self.roots,
            nodes: coverage.nodes,
            scopes: coverage.scopes,
            gates,
            transfers: analysis.transfers,
            accesses: analysis.accesses,
            obligations,
            provenance: analysis.provenance,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::owned_continuation::tests::{fixture, resources};
    fn template_budget(
        (e, artifacts): (
            DefinitionEnvelope,
            crate::owned_continuation::tests::Artifacts,
        ),
        new_work: bool,
    ) {
        let b = &e.document()["body"];
        let supplied = resources(&artifacts);
        let ordinary = check_protection(&e, &supplied).unwrap();
        let mut budget = Budget::default();
        let graph = Graph::build(b, &mut budget).unwrap();
        let mut c = Candidates::new(b, &mut budget).unwrap();
        let mut measured = Budget::default();
        let template = c.prove(&graph, "root", &mut measured).unwrap();
        assert!(template.levels[0].pre_capture.is_some());
        assert_eq!(template.levels[0].work.is_some(), new_work);
        let mut limited = Budget {
            work: 1_000_000 - measured.work + 1,
        };
        assert_eq!(
            c.consider(&graph, "root", "run_owner", &mut limited)
                .unwrap_err()
                .code,
            "budget"
        );
        assert!(c.templates.is_empty());
        assert!(c.roots.is_empty());
        c.consider(&graph, "root", "run_owner", &mut budget)
            .unwrap();
        c.consider(&graph, "root", "again", &mut budget).unwrap();
        let roots: Vec<_> = c.roots.iter().collect();
        assert!(Arc::ptr_eq(&roots[0].template, &roots[1].template));
        let mut measured = Budget::default();
        c.check(&e, &supplied, &ordinary, &mut measured).unwrap();
        let mut c = Candidates::new(b, &mut Budget::default()).unwrap();
        c.consider(&graph, "root", "run_owner", &mut Budget::default())
            .unwrap();
        c.consider(&graph, "root", "again", &mut Budget::default())
            .unwrap();
        let mut limited = Budget {
            work: 1_000_000 - measured.work + 1,
        };
        assert_eq!(
            c.check(&e, &supplied, &ordinary, &mut limited)
                .unwrap_err()
                .code,
            "budget"
        );
    }
    #[test]
    fn new_entry_templates_and_final_rows_obey_the_shared_remaining_budget() {
        template_budget(
            crate::owned_continuation::tests::pre_capture_fixture(),
            false,
        );
    }
    #[test]
    fn new_work_copies_templates_and_final_rows_obey_the_shared_remaining_budget() {
        template_budget(
            crate::owned_continuation::tests::finite_work_fixture(),
            true,
        );
    }
    #[test]
    fn indexing_templates_and_publication_refuse_cumulatively_without_partial_rows() {
        let (e, artifacts) = fixture();
        let b = &e.document()["body"];
        let supplied = resources(&artifacts);
        let ordinary = check_protection(&e, &supplied).unwrap();
        let mut exhausted = Budget { work: 1_000_000 };
        assert!(matches!(Candidates::new(b, &mut exhausted), Err(e) if e.code == "budget"));
        let mut budget = Budget::default();
        let graph = Graph::build(b, &mut budget).unwrap();
        let mut c = Candidates::new(b, &mut budget).unwrap();
        c.consider(&graph, "root", "run_owner", &mut budget)
            .unwrap();
        c.consider(&graph, "root", "run_again", &mut budget)
            .unwrap();
        let roots: Vec<_> = c.roots.iter().collect();
        assert!(Arc::ptr_eq(&roots[0].template, &roots[1].template));
        let mut exhausted = Budget { work: 999_999 };
        assert_eq!(
            c.consider(&graph, "root", "third", &mut exhausted)
                .unwrap_err()
                .code,
            "budget"
        );
        assert_eq!(c.roots.len(), 2);
        // Measure real publication work, then leave one unit less. This reaches
        // the final aggregate charge after all gates, not just the first loop.
        let mut full = Budget::default();
        c.check(&e, &supplied, &ordinary, &mut full).unwrap();
        for remaining in [0, 3, full.work - 1] {
            let mut c = Candidates::new(b, &mut Budget::default()).unwrap();
            c.consider(&graph, "root", "run_owner", &mut Budget::default())
                .unwrap();
            c.consider(&graph, "root", "run_again", &mut Budget::default())
                .unwrap();
            let mut limited = Budget {
                work: 1_000_000 - remaining,
            };
            assert_eq!(
                c.check(&e, &supplied, &ordinary, &mut limited)
                    .unwrap_err()
                    .code,
                "budget"
            );
        }
    }
    #[test]
    fn conditional_graph_budget_never_returns_partial_coverage() {
        let (e, artifacts) = fixture();
        let b = &e.document()["body"];
        let supplied = resources(&artifacts);
        let ordinary = check_control(&e, &supplied).unwrap();
        let mut budget = Budget::default();
        let graph = Graph::build(b, &mut budget).unwrap();
        let mut c = Candidates::new(b, &mut budget).unwrap();
        c.consider(&graph, "root", "run_owner", &mut budget)
            .unwrap();
        let entries: Vec<_> = c.templates["root"]
            .levels
            .iter()
            .map(|l| &l.entry)
            .collect();
        let limited = Budget { work: 999_997 };
        assert_eq!(
            crate::control::check_continuation_entries(b, &entries, &ordinary, limited)
                .err()
                .unwrap()
                .code,
            "budget"
        );
    }
}
