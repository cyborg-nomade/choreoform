// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use crate::control::graph::Graph;

pub(super) struct Flow<'a> {
    b: &'a Value,
    graph: Graph,
    providers: BTreeMap<ProviderRole, BTreeMap<String, Arc<Provider>>>,
    budget: Budget,
    initializers: crate::initialization::Index<'a>,
    pub transfers: usize,
    pub accesses: BTreeSet<AccessObligation>,
    pub dependencies: Option<Dependencies>,
    nonlocal_writes: BTreeSet<String>,
    history_entries: BTreeMap<String, String>,
    pub(super) provenance: Option<BTreeMap<String, Origins>>,
    pub(super) required_provenance: BTreeSet<RequiredProtectionProvenance>,
}
impl<'a> Flow<'a> {
    pub fn new(b: &'a Value, supplied: &[Resource<'_>]) -> Result<Self> {
        let mut budget = Budget::default();
        let graph = Graph::build(b, &mut budget)?;
        let initializers = crate::initialization::by_scope(b, &mut budget)?;
        let mut registry = Resources::new(supplied)?;
        let mut providers: BTreeMap<ProviderRole, BTreeMap<String, Arc<Provider>>> =
            BTreeMap::new();
        for (id, n) in object(&b["nodes"])? {
            budget.tick(0)?;
            if n["kind"] == "activity" && n["mode"] == "capability" {
                let cap = &b["capabilities"][text(&n["target"])?];
                let pin = Pin::decode(&b["policies"][text(&cap["contract"])?]["body"]["provider"])?;
                providers
                    .entry(ProviderRole::Activity)
                    .or_default()
                    .insert(id.clone(), registry.provider(&pin)?);
            }
            if n["kind"] == "wait" && !n["capture"].is_null() {
                let source =
                    registry.provider_terminal_source(&Pin::decode(&n["capture"]["source"])?)?;
                providers
                    .entry(ProviderRole::Capture)
                    .or_default()
                    .insert(id.clone(), registry.provider(source.provider())?);
            }
            let timer = match n["kind"].as_str() {
                Some("activity") => &b["policies"][text(&n["policy"])?]["body"]["timeout"],
                Some("wait") => &b["policies"][text(&n["policy"])?]["body"]["timer"],
                _ => &Value::Null,
            };
            if !timer.is_null() {
                let clock = registry.clock(&Pin::decode(&timer["clock"])?)?;
                providers
                    .entry(ProviderRole::Clock)
                    .or_default()
                    .insert(id.clone(), registry.provider(clock.source())?);
            }
            if n["kind"] == "activity" {
                let retry = &b["policies"][text(&n["policy"])?]["body"]["retry"];
                if !retry["clock"].is_null() {
                    let clock = registry.clock(&Pin::decode(&retry["clock"])?)?;
                    providers
                        .entry(ProviderRole::RetryClock)
                        .or_default()
                        .insert(id.clone(), registry.provider(clock.source())?);
                }
            }
        }
        let provenance = (providers.contains_key(&ProviderRole::Capture)
            || object(&b["scopes"])?
                .values()
                .any(|s| !s["faultInput"].is_null()))
        .then(BTreeMap::new);
        Ok(Self {
            b,
            graph,
            providers,
            budget,
            initializers,
            transfers: 0,
            accesses: BTreeSet::new(),
            dependencies: None,
            nonlocal_writes: BTreeSet::new(),
            history_entries: BTreeMap::new(),
            provenance,
            required_provenance: BTreeSet::new(),
        })
    }
    pub(super) fn collect_dependencies(&mut self) -> Result<()> {
        // Static checking has verified complete write inventories and visibility.
        // Count every declaration, not only ordinary/reachable descendants.
        for n in object(&self.b["nodes"])?.values() {
            self.budget.charge(object(&n["writes"])?.len() + 1)?;
            for cell in object(&n["writes"])?.keys() {
                if self.b["data"][cell]["scope"] != n["scope"] {
                    self.nonlocal_writes.insert(cell.clone());
                }
            }
        }
        self.dependencies = Some(Dependencies::default());
        Ok(())
    }
    fn ordinary_completed(
        &mut self,
        origins: Origins,
        scope: &str,
        boundary: &str,
        inherited: &Origins,
    ) -> Result<Origins> {
        if self.dependencies.is_none() {
            return Ok(origins);
        }
        self.budget.charge(origins.len())?;
        let mut result = Origins::new();
        for origin in origins {
            let origin = match origin {
                Origin::Cell(cell)
                    if self.b["data"][&cell]["scope"] == scope
                        && !self.nonlocal_writes.contains(&cell)
                        && !inherited.contains(&Origin::Cell(cell.clone())) =>
                {
                    Origin::Historical {
                        cell,
                        boundary: boundary.into(),
                    }
                }
                other => other,
            };
            result.insert(origin);
        }
        Ok(result)
    }
    fn label(&self, origin: &Origin) -> &Value {
        match origin {
            Origin::Cell(id)
            | Origin::Historical { cell: id, .. }
            | Origin::RetainedProvenance(id) => &self.b["data"][id]["protection"],
            Origin::Provider(site) => self.providers[&site.role][&site.node].protection(),
        }
    }
    fn dependencies(&mut self, p: &Value) -> Result<BTreeSet<String>> {
        crate::access::policy_reads(self.b, p, &mut self.budget)
    }
    fn source(&mut self, node: &str, role: ProviderRole, origins: &mut Origins) -> Result<()> {
        if let Some(provider) = self
            .providers
            .get(&role)
            .and_then(|sites| sites.get(node))
            .cloned()
        {
            self.budget.tick(0)?;
            for read in self.dependencies(provider.protection())? {
                origins.extend(self.access(node, &read, "read")?);
            }
            origins.insert(Origin::Provider(ProviderSite::new(node, role)));
        }
        Ok(())
    }
    fn access(&mut self, site: &str, cell: &str, operation: &'static str) -> Result<Origins> {
        let mut origins = Origins::new();
        let mut queue = vec![cell.to_owned()];
        let mut seen = BTreeSet::new();
        while let Some(id) = queue.pop() {
            self.budget.tick(0)?;
            if !seen.insert(id.clone()) {
                continue;
            }
            let p = &self.b["data"][&id]["protection"];
            if object(&p["participants"])?.is_empty() && object(&p["capabilities"])?.is_empty() {
                return Err(diagnostic("access-context-empty", "data", &id));
            }
            self.accesses.insert(AccessObligation {
                site: site.into(),
                cell: id.clone(),
                operation: if id == cell { operation } else { "read" },
            });
            origins.insert(Origin::Cell(id));
            queue.extend(self.dependencies(p)?);
        }
        self.budget.charge(origins.len())?;
        Ok(origins)
    }
    fn publish(&mut self, sources: &Origins, target: &str, site: &str) -> Result<()> {
        self.budget.charge(sources.len())?;
        let mut sources = sources.clone();
        // A destination's own access predicate can depend on protected data,
        // including at input/item/finish boundaries without an ordinary write.
        for read in self.dependencies(&self.b["data"][target]["protection"])? {
            sources.extend(self.access(site, &read, "read")?);
        }
        for origin in &sources {
            self.budget.charge(transfer_cost(
                self.label(origin),
                &self.b["data"][target]["protection"],
            )?)?;
            transfer(self.label(origin), &self.b["data"][target]["protection"]).map_err(|_| {
                let map = if self.b["nodes"].get(site).is_some() {
                    "nodes"
                } else if self.b["data"].get(site).is_some() {
                    "data"
                } else {
                    "scopes"
                };
                diagnostic("control-information-flow", map, site)
            })?;
            self.transfers += 1;
        }
        Ok(())
    }
    pub fn scope(
        &mut self,
        scope: &str,
        inherited: &Origins,
        depth: usize,
        creator: Option<&str>,
    ) -> Result<Origins> {
        self.budget.tick(depth)?;
        self.budget.charge(inherited.len())?;
        if let Some(dependencies) = &mut self.dependencies {
            self.budget.charge(inherited.len() + 1)?;
            if let Some(site) = creator {
                for origin in inherited {
                    if let Origin::Historical { cell, boundary } = origin {
                        self.budget.charge(6)?;
                        let source_scope = text(&self.b["data"][cell]["scope"])?;
                        dependencies.historical.insert(HistoricalOrigin {
                            site: site.into(),
                            cell: cell.clone(),
                            boundary: boundary.clone(),
                            scope: source_scope.into(),
                            closure: text(&self.b["scopes"][source_scope]["closure"])?.into(),
                            context_scope: Some(scope.into()),
                        });
                    }
                }
            }
            dependencies.scopes.entry(scope.into()).or_default().extend(
                inherited.iter().filter_map(|origin| match origin {
                    Origin::Cell(cell) => Some(cell.clone()),
                    Origin::Provider(_)
                    | Origin::Historical { .. }
                    | Origin::RetainedProvenance(_) => None,
                }),
            );
        }
        let s = &self.b["scopes"][scope];
        let mut entry = inherited.clone();
        for cell in object(&s["inputs"])?.values() {
            let id = text(cell)?;
            let deps = self.access(scope, id, "write")?;
            self.publish(inherited, id, scope)?;
            entry.extend(deps);
        }
        if let Some(target) = self.history_entries.get(scope).cloned() {
            self.budget.charge(inherited.len() + 1)?;
            let mut sources = inherited.clone();
            sources.extend(
                self.access(scope, &target, "write")?
                    .into_iter()
                    .filter(|x| !matches!(x,Origin::Cell(cell) if cell==&target)),
            );
            self.publish(&sources, &target, scope)?;
            entry.extend(sources);
            entry.insert(Origin::Cell(target));
        }
        self.budget
            .charge(self.initializers.get(scope).map_or(0, Vec::len))?;
        let initializers = self.initializers.get(scope).cloned().unwrap_or_default();
        for id in initializers {
            self.budget.tick(depth)?;
            let d = &self.b["data"][id];
            if let Some(initial) = d.get("initial") {
                let mut sources = inherited.clone();
                sources.extend(
                    self.access(id, id, "write")?
                        .into_iter()
                        .filter(|x| !matches!(x, Origin::Cell(cell) if cell == id)),
                );
                for read in names(&self.b["expressions"][text(initial)?]["reads"])? {
                    sources.extend(self.access(id, &read, "read")?);
                }
                self.publish(&sources, id, id)?;
                entry.extend(sources);
            }
        }
        let mut provenance = Origins::new();
        if self.provenance.is_some() {
            self.budget.charge(entry.len())?;
            provenance.extend(entry.iter().cloned());
        }
        let mut after: BTreeMap<String, Origins> = BTreeMap::new();
        let mut completed = Origins::new();
        let order = self.graph.order[scope].clone();
        self.budget.charge(order.len())?;
        for id in order {
            self.budget.tick(depth)?;
            let mut pc = if s["entry"] == id {
                self.budget.charge(entry.len())?;
                Some(entry.clone())
            } else {
                None
            };
            for pred in &self.graph.predecessors[&id] {
                if let Some(sources) = after.get(pred) {
                    self.budget.charge(sources.len())?;
                    pc.get_or_insert_default().extend(sources.iter().cloned());
                }
            }
            let Some(mut pc) = pc else {
                continue;
            };
            let n = &self.b["nodes"][&id];
            let mut reads = Origins::new();
            for cell in names(&n["reads"])? {
                reads.extend(self.access(&id, &cell, "read")?);
            }
            // Timing affects whether a result may publish, not only its later
            // successor. Keep clock and activity providers as distinct origins.
            self.source(&id, ProviderRole::Clock, &mut pc)?;
            self.source(&id, ProviderRole::RetryClock, &mut pc)?;
            self.source(&id, ProviderRole::Capture, &mut pc)?;
            if matches!(text(&n["kind"])?, "decision" | "activity" | "wait")
                && let Some(dependencies) = &mut self.dependencies
            {
                self.budget.charge(pc.len() + reads.len())?;
                // Save pre-result dependencies, unioning repeated call contexts.
                // A site's own freshly produced result cannot invalidate itself
                // merely because publication adds it to the outgoing context.
                let cells = dependencies.nodes.entry(id.clone()).or_default();
                for origin in pc.iter().chain(&reads) {
                    match origin {
                        Origin::Cell(cell) => {
                            cells.insert(cell.clone());
                        }
                        Origin::Historical { cell, boundary } => {
                            self.budget.charge(5)?;
                            let scope = text(&self.b["data"][cell]["scope"])?;
                            dependencies.historical.insert(HistoricalOrigin {
                                site: id.clone(),
                                cell: cell.clone(),
                                boundary: boundary.clone(),
                                scope: scope.into(),
                                closure: text(&self.b["scopes"][scope]["closure"])?.into(),
                                context_scope: None,
                            });
                        }
                        Origin::Provider(_) | Origin::RetainedProvenance(_) => {}
                    }
                }
            }
            for cell in names(&n["writes"])? {
                let deps = self.access(&id, &cell, "write")?;
                let mut sources = pc.clone();
                sources.extend(reads.iter().cloned());
                sources.extend(
                    deps.into_iter()
                        .filter(|x| *x != Origin::Cell(cell.clone())),
                );
                self.publish(&sources, &cell, &id)?;
            }
            pc.extend(reads);
            // Successful completion/outcome selection can disclose input/result facts.
            self.source(&id, ProviderRole::Activity, &mut pc)?;
            match text(&n["kind"])? {
                "invoke" => {
                    let child = text(&n["body"])?;
                    for (port, expr) in object(&n["inputs"])? {
                        let target = text(&self.b["scopes"][child]["inputs"][port])?;
                        let mut sources = pc.clone();
                        for cell in names(&self.b["expressions"][text(expr)?]["reads"])? {
                            sources.extend(self.access(&id, &cell, "transfer")?);
                        }
                        self.access(&id, target, "transfer")?;
                        self.publish(&sources, target, &id)?;
                    }
                    let completed = self.scope(child, &pc.clone(), depth + 1, Some(&id))?;
                    self.child_provenance(child, &mut provenance)?;
                    let closed = self.ordinary_completed(completed, child, &id, &pc)?;
                    pc.extend(closed);
                    for (port, target) in object(&n["outputs"])? {
                        let source = text(&self.b["scopes"][child]["outputs"][port])?;
                        let mut sources = pc.clone();
                        let output = self.access(&id, source, "transfer")?;
                        sources.extend(self.ordinary_completed(output, child, &id, &pc)?);
                        let target = text(target)?;
                        self.access(&id, target, "transfer")?;
                        self.publish(&sources, target, &id)?;
                    }
                }
                "repeat" | "fanout" => {
                    if n["kind"] == "fanout" {
                        let item = text(&n["item"])?;
                        self.access(&id, item, "transfer")?;
                        self.publish(&pc, item, &id)?;
                    }
                    let body = text(&n["body"])?;
                    let completed = self.scope(body, &pc.clone(), depth + 1, Some(&id))?;
                    self.child_provenance(body, &mut provenance)?;
                    let completed = if n["kind"] == "repeat" {
                        self.ordinary_completed(completed, body, &id, &pc)?
                    } else {
                        completed
                    };
                    pc.extend(completed);
                }
                "split" => {
                    let snapshot = pc.clone();
                    for child in object(&n["children"])?.values() {
                        pc.extend(self.scope(text(child)?, &snapshot, depth + 1, Some(&id))?);
                        self.child_provenance(text(child)?, &mut provenance)?;
                    }
                }
                "finish" => {
                    for target in object(&s["outputs"])?.values() {
                        let target = text(target)?;
                        self.access(&id, target, "transfer")?;
                        self.publish(&pc, target, &id)?;
                    }
                    self.budget.charge(pc.len())?;
                    completed.extend(pc.iter().cloned());
                }
                _ => {}
            }
            pc.extend(names(&n["writes"])?.into_iter().map(Origin::Cell));
            self.budget.charge(pc.len())?;
            if self.provenance.is_some() {
                self.budget.charge(pc.len())?;
                provenance.extend(pc.iter().cloned());
            }
            after.insert(id, pc);
        }
        if let Some(contexts) = &mut self.provenance {
            self.budget.charge(provenance.len() + 1)?;
            contexts.entry(scope.into()).or_default().extend(provenance);
        }
        Ok(completed)
    }
    fn child_provenance(&mut self, child: &str, into: &mut Origins) -> Result<()> {
        if let Some(origins) = self
            .provenance
            .as_ref()
            .and_then(|contexts| contexts.get(child))
        {
            self.budget.charge(origins.len())?;
            into.extend(origins.iter().cloned());
        }
        Ok(())
    }
    pub(super) fn conditional_scope(
        &mut self,
        scope: &str,
        owners: &BTreeSet<String>,
        ordinary: &BTreeMap<String, Origins>,
    ) -> Result<()> {
        let inherited =
            self.retained_context(owners, ordinary, "owned-local-provenance-unproved")?;
        self.scope(scope, &inherited, 0, None)?;
        Ok(())
    }
    fn retained_context(
        &mut self,
        owners: &BTreeSet<String>,
        ordinary: &BTreeMap<String, Origins>,
        code: &'static str,
    ) -> Result<Origins> {
        let mut inherited = Origins::new();
        for owner in owners {
            self.budget.tick(0)?;
            let origins = ordinary
                .get(owner)
                .ok_or_else(|| diagnostic(code, "scopes", owner))?;
            self.budget.charge(origins.len())?;
            for origin in origins {
                self.budget.charge(4)?;
                let (kind, site, role) = match origin {
                    Origin::Cell(cell)
                    | Origin::Historical { cell, .. }
                    | Origin::RetainedProvenance(cell) => ("cell", cell.clone(), None),
                    Origin::Provider(site) => (
                        "provider",
                        site.node.clone(),
                        Some(match site.role {
                            ProviderRole::Activity => "activity",
                            ProviderRole::Clock => "clock",
                            ProviderRole::RetryClock => "retry-clock",
                            ProviderRole::Capture => "capture",
                        }),
                    ),
                };
                self.required_provenance
                    .insert(RequiredProtectionProvenance {
                        owner: owner.clone(),
                        kind,
                        site,
                        role,
                    });
                inherited.insert(match origin {
                    Origin::Cell(cell)
                    | Origin::Historical { cell, .. }
                    | Origin::RetainedProvenance(cell) => Origin::RetainedProvenance(cell.clone()),
                    Origin::Provider(site) => Origin::Provider(site.clone()),
                });
            }
        }
        Ok(inherited)
    }
    pub(super) fn history_scope(
        &mut self,
        scope: &str,
        target: &str,
        owners: &BTreeSet<String>,
        sources: &BTreeSet<String>,
        ordinary: &BTreeMap<String, Origins>,
    ) -> Result<()> {
        let inherited =
            self.retained_context(owners, ordinary, "fault-entry-provenance-unproved")?;
        for site in sources {
            self.budget.tick(0)?;
            require(
                inherited.contains(&Origin::Provider(ProviderSite::new(
                    site,
                    ProviderRole::Activity,
                ))),
                "fault-entry-provider-provenance-unproved",
            )?;
        }
        self.budget.charge(2)?;
        self.history_entries.insert(scope.into(), target.into());
        self.scope(scope, &inherited, 0, None)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn provenance_walks_refuse_cumulatively_before_copying_and_legacy_collection_stays_off() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let document: Value = serde_json::from_slice(include_bytes!(
            "../../../../docs/benchmarks/rp01/candidate.ir.json"
        ))
        .unwrap();
        let manifest: Value = serde_json::from_slice(include_bytes!(
            "../../../../docs/benchmarks/rp01/manifest.json"
        ))
        .unwrap();
        let owned: Vec<_> = manifest["resources"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                (
                    r["id"].as_str().unwrap().to_owned(),
                    r["revision"].as_str().unwrap().to_owned(),
                    std::fs::read(root.join(r["path"].as_str().unwrap())).unwrap(),
                )
            })
            .collect();
        let supplied: Vec<_> = owned
            .iter()
            .map(|(id, revision, bytes)| Resource {
                id,
                revision,
                bytes,
            })
            .collect();
        let mut flow = Flow::new(&document["body"], &supplied).unwrap();
        assert!(flow.provenance.is_none());
        let contexts = BTreeMap::from([(
            "payment_service".into(),
            BTreeSet::from([
                Origin::Cell("current_claim".into()),
                Origin::Cell("disposition".into()),
            ]),
        )]);
        flow.provenance = Some(contexts.clone());
        flow.budget.work = 999_999;
        let mut result = Origins::new();
        assert_eq!(
            flow.child_provenance("payment_service", &mut result)
                .unwrap_err()
                .code,
            "budget"
        );
        assert!(result.is_empty());
        flow.budget.work = 999_999;
        assert_eq!(
            flow.conditional_scope(
                "payment_recovery",
                &BTreeSet::from(["payment_service".into()]),
                &contexts
            )
            .unwrap_err()
            .code,
            "budget"
        );
        assert!(flow.required_provenance.is_empty());
    }
}
