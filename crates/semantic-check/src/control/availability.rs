// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
type Cells = BTreeSet<String>;

pub(super) struct Analysis<'a> {
    b: &'a Value,
    graph: Graph,
    budget: Budget,
    pub nodes: BTreeSet<String>,
    pub scopes: BTreeSet<String>,
}
impl<'a> Analysis<'a> {
    pub fn new(b: &'a Value, graph: Graph, budget: Budget) -> Self {
        Self {
            b,
            graph,
            budget,
            nodes: BTreeSet::new(),
            scopes: BTreeSet::new(),
        }
    }
    fn ancestors(&self, scope: &str) -> Result<Cells> {
        let mut set = Cells::new();
        let mut cursor = self.b["scopes"][scope]["parent"].as_str();
        while let Some(id) = cursor {
            if !set.insert(id.into()) || set.len() > MAX_DEPTH {
                return Err(err("scope-cycle"));
            }
            cursor = self.b["scopes"][id]["parent"].as_str();
        }
        Ok(set)
    }
    fn available(
        &mut self,
        required: &Cells,
        state: &Cells,
        map: &str,
        id: &str,
        code: &'static str,
    ) -> Result<()> {
        self.budget.charge(required.len())?;
        if !required.is_subset(state) {
            return Err(at(code, map, id));
        }
        Ok(())
    }
    pub fn scope(&mut self, scope: &str, parent: &Cells, depth: usize) -> Result<Cells> {
        self.budget.tick(depth)?;
        self.scopes.insert(scope.into());
        let s = &self.b["scopes"][scope];
        let ancestors = self.ancestors(scope)?;
        self.budget.charge(parent.len())?;
        // Every invocation/iteration has fresh locals, even for the same template.
        let mut initial: Cells = parent
            .iter()
            .filter(|id| {
                self.b["data"][*id]["scope"]
                    .as_str()
                    .is_some_and(|owner| ancestors.contains(owner))
            })
            .cloned()
            .collect();
        for cell in object(&s["inputs"])?.values() {
            initial.insert(text(cell)?.into());
        }
        self.budget.charge(initial.len())?;
        let entry = initial.clone();
        for (id, data) in object(&self.b["data"])? {
            self.budget.tick(depth)?;
            if data["scope"] == scope
                && let Some(expr) = data.get("initial")
            {
                let reads = names(&self.b["expressions"][text(expr)?]["reads"])?;
                self.available(
                    &reads,
                    &entry,
                    "data",
                    id,
                    "initializer-availability-unproved",
                )?;
                initial.insert(id.clone());
            }
        }
        let mut after: BTreeMap<String, Cells> = BTreeMap::new();
        let mut finishes: Option<Cells> = None;
        let order = self
            .graph
            .order
            .get(scope)
            .ok_or_else(|| err("scope-entry"))?
            .clone();
        self.budget.charge(order.len())?;
        for id in order {
            self.budget.tick(depth)?;
            let mut before = if s["entry"] == id {
                self.budget.charge(initial.len())?;
                Some(initial.clone())
            } else {
                None
            };
            if before.is_none() {
                for predecessor in &self.graph.predecessors[&id] {
                    if let Some(state) = after.get(predecessor) {
                        self.budget.charge(state.len())?;
                        before = Some(match before {
                            None => state.clone(),
                            Some(current) => current.intersection(state).cloned().collect(),
                        });
                    }
                }
            }
            let Some(mut state) = before else {
                continue;
            };
            self.nodes.insert(id.clone());
            let n = &self.b["nodes"][&id];
            let reads = names(&n["reads"])?;
            self.available(&reads, &state, "nodes", &id, "data-availability-unproved")?;
            match text(&n["kind"])? {
                "invoke" => {
                    let child = text(&n["body"])?;
                    let completed = self.scope(child, &state, depth + 1)?;
                    // Retain proven ancestor availability, never export child locals.
                    self.budget.charge(completed.len())?;
                    for cell in &completed {
                        if self.b["data"][cell]["scope"] != child {
                            state.insert(cell.clone());
                        }
                    }
                    for (port, target) in object(&n["outputs"])? {
                        let source = text(&self.b["scopes"][child]["outputs"][port])?;
                        if !completed.contains(source) {
                            return Err(at("output-availability-unproved", "nodes", &id));
                        }
                        state.insert(text(target)?.into());
                    }
                }
                "repeat" => {
                    self.scope(text(&n["body"])?, &state, depth + 1)?;
                }
                "split" => {
                    for child in object(&n["children"])?.values() {
                        self.scope(text(child)?, &state, depth + 1)?;
                    }
                }
                "fanout" => {
                    self.scope(text(&n["body"])?, &state, depth + 1)?;
                }
                "finish" => {
                    let outputs = object(&s["outputs"])?
                        .values()
                        .map(|v| text(v).map(str::to_owned))
                        .collect::<Result<Cells>>()?;
                    self.available(
                        &outputs,
                        &state,
                        "nodes",
                        &id,
                        "output-availability-unproved",
                    )?;
                    self.budget.charge(state.len())?;
                    finishes = Some(match finishes {
                        None => state.clone(),
                        Some(current) => current.intersection(&state).cloned().collect(),
                    });
                }
                _ => {
                    state.extend(names(&n["writes"])?);
                }
            }
            self.budget.charge(state.len())?;
            after.insert(id, state);
        }
        finishes.ok_or_else(|| at("ordinary-completion-unproved", "scopes", scope))
    }
}
