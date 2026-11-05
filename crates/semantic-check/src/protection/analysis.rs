// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use crate::control::graph::Graph;

pub(super) struct Flow<'a> {
    b: &'a Value,
    graph: Graph,
    providers: BTreeMap<String, Arc<Provider>>,
    budget: Budget,
    pub transfers: usize,
    pub accesses: BTreeSet<AccessObligation>,
}
impl<'a> Flow<'a> {
    pub fn new(b: &'a Value, supplied: &[Resource<'_>]) -> Result<Self> {
        let mut budget = Budget::default();
        let graph = Graph::build(b, &mut budget)?;
        let mut registry = Resources::new(supplied)?;
        let mut providers = BTreeMap::new();
        for (id, n) in object(&b["nodes"])? {
            budget.tick(0)?;
            if n["kind"] == "activity" && n["mode"] == "capability" {
                let cap = &b["capabilities"][text(&n["target"])?];
                let pin = Pin::decode(&b["policies"][text(&cap["contract"])?]["body"]["provider"])?;
                providers.insert(id.clone(), registry.provider(&pin)?);
            }
        }
        Ok(Self {
            b,
            graph,
            providers,
            budget,
            transfers: 0,
            accesses: BTreeSet::new(),
        })
    }
    fn label(&self, origin: &Origin) -> &Value {
        match origin {
            Origin::Cell(id) => &self.b["data"][id]["protection"],
            Origin::Provider(id) => self.providers[id].protection(),
        }
    }
    fn dependencies(&mut self, p: &Value) -> Result<BTreeSet<String>> {
        crate::access::policy_reads(self.b, p, &mut self.budget)
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
    pub fn scope(&mut self, scope: &str, inherited: &Origins, depth: usize) -> Result<Origins> {
        self.budget.tick(depth)?;
        self.budget.charge(inherited.len())?;
        let s = &self.b["scopes"][scope];
        let mut entry = inherited.clone();
        for cell in object(&s["inputs"])?.values() {
            let id = text(cell)?;
            let deps = self.access(scope, id, "write")?;
            self.publish(inherited, id, scope)?;
            entry.extend(deps);
        }
        for (id, d) in object(&self.b["data"])? {
            self.budget.tick(depth)?;
            if d["scope"] != scope {
                continue;
            }
            if let Some(initial) = d.get("initial") {
                let mut sources = inherited.clone();
                sources.extend(
                    self.access(id, id, "write")?
                        .into_iter()
                        .filter(|x| *x != Origin::Cell(id.clone())),
                );
                for read in names(&self.b["expressions"][text(initial)?]["reads"])? {
                    sources.extend(self.access(id, &read, "read")?);
                }
                self.publish(&sources, id, id)?;
                entry.extend(sources);
            }
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
            if let Some(provider) = self.providers.get(&id).cloned() {
                pc.insert(Origin::Provider(id.clone()));
                for read in self.dependencies(provider.protection())? {
                    pc.extend(self.access(&id, &read, "read")?);
                }
            }
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
                    pc.extend(self.scope(child, &pc.clone(), depth + 1)?);
                    for (port, target) in object(&n["outputs"])? {
                        let source = text(&self.b["scopes"][child]["outputs"][port])?;
                        let mut sources = pc.clone();
                        sources.extend(self.access(&id, source, "transfer")?);
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
                    pc.extend(self.scope(text(&n["body"])?, &pc.clone(), depth + 1)?);
                }
                "split" => {
                    let snapshot = pc.clone();
                    for child in object(&n["children"])?.values() {
                        pc.extend(self.scope(text(child)?, &snapshot, depth + 1)?);
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
            after.insert(id, pc);
        }
        Ok(completed)
    }
}
