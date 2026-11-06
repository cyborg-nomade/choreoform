// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

pub(crate) struct Graph {
    pub order: BTreeMap<String, Vec<String>>,
    pub predecessors: BTreeMap<String, BTreeSet<String>>,
}
impl Graph {
    /// Potential ordinary reachability only; no predicate truth or entry values.
    pub(crate) fn reachable<'a>(
        &'a self,
        b: &Value,
        budget: &mut Budget,
    ) -> Result<BTreeMap<&'a str, BTreeSet<&'a str>>> {
        let mut result = BTreeMap::new();
        for (scope, order) in &self.order {
            budget.tick(0)?;
            let mut reached = BTreeSet::new();
            for id in order {
                budget.tick(0)?;
                let mut possible = b["scopes"][scope]["entry"] == *id;
                for prior in &self.predecessors[id] {
                    budget.tick(0)?;
                    possible |= reached.contains(prior.as_str());
                }
                if possible {
                    reached.insert(id.as_str());
                }
            }
            result.insert(scope.as_str(), reached);
        }
        Ok(result)
    }
    pub fn build(b: &Value, budget: &mut Budget) -> Result<Self> {
        let nodes = object(&b["nodes"])?;
        let mut edges: BTreeMap<String, BTreeSet<String>> = nodes
            .keys()
            .map(|id| (id.clone(), BTreeSet::new()))
            .collect();
        let mut predecessors = edges.clone();
        let mut pairs = BTreeSet::new();
        for flow in object(&b["flows"])?.values() {
            budget.tick(0)?;
            let source = text(&flow["source"])?;
            let target = text(&flow["target"])?;
            if !pairs.insert((source.to_owned(), text(&flow["outcome"])?.to_owned())) {
                return Err(at("flow-duplicate", "nodes", source));
            }
            if nodes[target]["kind"] == "join" {
                return Err(at("join-activation", "nodes", target));
            }
            edges
                .get_mut(source)
                .ok_or_else(|| err("reference"))?
                .insert(target.into());
            predecessors
                .get_mut(target)
                .ok_or_else(|| err("reference"))?
                .insert(source.into());
        }
        for (scope, s) in object(&b["scopes"])? {
            if nodes[text(&s["entry"])?]["kind"] == "join" {
                return Err(at("join-activation", "scopes", scope));
            }
            let inputs = object(&s["inputs"])?;
            if inputs
                .values()
                .map(text)
                .collect::<Result<BTreeSet<_>>>()?
                .len()
                != inputs.len()
            {
                return Err(at("input-cell-alias", "scopes", scope));
            }
        }
        for (id, n) in nodes {
            budget.tick(0)?;
            for outcome in names(&n["outcomes"])? {
                budget.tick(0)?;
                if !pairs.contains(&(id.clone(), outcome)) {
                    return Err(at("flow-missing", "nodes", id));
                }
            }
            if n["kind"] == "invoke" {
                let outputs = object(&n["outputs"])?;
                let targets = outputs
                    .values()
                    .map(|v| text(v).map(str::to_owned))
                    .collect::<Result<BTreeSet<_>>>()?;
                if targets.len() != outputs.len() {
                    return Err(at("output-target-alias", "nodes", id));
                }
                if names(&n["writes"])? != targets {
                    return Err(at("invoke-write-set", "nodes", id));
                }
            }
            if n["kind"] == "split" || n["kind"] == "fanout" {
                let join = text(&n["join"])?;
                let j = &nodes[join];
                if j["kind"] != "join" || j["source"] != *id || j["scope"] != n["scope"] {
                    return Err(at("join-pair", "nodes", id));
                }
                edges
                    .get_mut(id)
                    .ok_or_else(|| err("reference"))?
                    .insert(join.into());
                predecessors
                    .get_mut(join)
                    .ok_or_else(|| err("reference"))?
                    .insert(id.clone());
            }
            if n["kind"] == "join" {
                let source = text(&n["source"])?;
                let fork = &nodes[source];
                if !matches!(fork["kind"].as_str(), Some("split" | "fanout"))
                    || fork["join"] != *id
                    || fork["scope"] != n["scope"]
                {
                    return Err(at("join-pair", "nodes", id));
                }
                let population: Vec<_> = if fork["kind"] == "split" {
                    object(&fork["children"])?.values().collect()
                } else {
                    vec![&fork["body"]]
                };
                let mut outcomes = BTreeSet::new();
                for child in population {
                    let set = names(&b["scopes"][text(child)?]["outcomes"])?;
                    budget.charge(set.len())?;
                    outcomes.extend(set);
                }
                predicate(&n["predicate"], &outcomes, budget, 0).map_err(|mut e| {
                    e.path = format!("/body/nodes/{id}/predicate");
                    e
                })?;
            }
        }
        // Graph-edge depth is independent of JSON nesting depth.
        let mut degrees: BTreeMap<_, _> = predecessors
            .iter()
            .map(|(id, p)| (id.clone(), p.len()))
            .collect();
        let mut ready: BTreeSet<_> = degrees
            .iter()
            .filter(|(_, n)| **n == 0)
            .map(|(id, _)| id.clone())
            .collect();
        let mut order: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut count = 0;
        while let Some(id) = ready.pop_first() {
            budget.tick(0)?;
            count += 1;
            order
                .entry(text(&nodes[&id]["scope"])?.into())
                .or_default()
                .push(id.clone());
            for target in &edges[&id] {
                budget.tick(0)?;
                let n = degrees.get_mut(target).ok_or_else(|| err("reference"))?;
                *n -= 1;
                if *n == 0 {
                    ready.insert(target.clone());
                }
            }
        }
        if count != nodes.len() {
            let id = degrees
                .iter()
                .find(|(_, n)| **n > 0)
                .map(|(id, _)| id.as_str())
                .unwrap_or("");
            return Err(at("flow-cycle", "nodes", id));
        }
        Ok(Self {
            order,
            predecessors,
        })
    }
}
fn predicate(
    p: &Value,
    outcomes: &BTreeSet<String>,
    budget: &mut Budget,
    depth: usize,
) -> Result<()> {
    budget.tick(depth)?;
    if matches!(p["op"].as_str(), Some("and" | "or")) {
        for term in object(&p["terms"])?.values() {
            predicate(term, outcomes, budget, depth + 1)?;
        }
    } else {
        let tokens = names(&p["outcomes"])?;
        budget.charge(tokens.len())?;
        require(tokens.is_subset(outcomes), "join-outcome")?;
    }
    Ok(())
}
