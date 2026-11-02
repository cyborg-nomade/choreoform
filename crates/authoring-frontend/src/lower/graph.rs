// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

use super::*;

impl<'a, 's> Lower<'a, 's> {
    pub(super) fn dependencies(&mut self) -> Result<()> {
        let mut local = BTreeMap::new();
        let mut edges = BTreeMap::new();
        let mut indegrees = BTreeMap::new();
        let mut dependents: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (id, policy) in self.body["policies"].as_object().expect("map") {
            let p = &policy["body"];
            let mut reads = BTreeSet::new();
            let mut refs = BTreeSet::new();
            for field in ["permit", "completion", "match", "invariant"] {
                reads.extend(ast_reads(&p[field]));
            }
            for field in ["reconciliation", "children", "removal"] {
                if let Some(target) = p[field].as_str() {
                    refs.insert(target.to_owned());
                }
            }
            if let Some(actor) = p["authority"].as_str() {
                refs.insert(
                    self.body["actors"][actor]["requirement"]
                        .as_str()
                        .ok_or_else(|| error("actor-requirement", 0..0))?
                        .into(),
                );
            }
            for field in ["timeout", "timer"] {
                let timer = &p[field];
                if timer.is_object() {
                    reads.extend(ast_reads(&timer["due"]));
                    if let Some(cell) = timer["basis"].as_str() {
                        reads.insert(cell.into());
                    }
                    let actor = timer["pauseAuthority"]
                        .as_str()
                        .ok_or_else(|| error("timer-authority", 0..0))?;
                    refs.insert(
                        self.body["actors"][actor]["requirement"]
                            .as_str()
                            .ok_or_else(|| error("actor-requirement", 0..0))?
                            .into(),
                    );
                }
            }
            for target in &refs {
                if self.body["policies"].get(target).is_none() {
                    return Err(error("policy-reference", 0..0));
                }
                dependents
                    .entry(target.clone())
                    .or_default()
                    .push(id.clone());
            }
            indegrees.insert(id.clone(), refs.len());
            edges.insert(id.clone(), refs);
            local.insert(id.clone(), reads);
        }
        let mut ready: Vec<_> = indegrees
            .iter()
            .filter(|(_, n)| **n == 0)
            .map(|(id, _)| id.clone())
            .collect();
        let mut closure: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        while let Some(id) = ready.pop() {
            let mut reads = local.remove(&id).expect("unique policy visit");
            for dep in &edges[&id] {
                reads.extend(closure[dep].iter().cloned());
            }
            closure.insert(id.clone(), reads);
            if let Some(users) = dependents.get(&id) {
                for user in users {
                    let n = indegrees.get_mut(user).expect("known policy");
                    *n -= 1;
                    if *n == 0 {
                        ready.push(user.clone());
                    }
                }
            }
        }
        if closure.len() != indegrees.len() {
            return Err(error("policy-cycle", 0..0));
        }
        for (id, reads) in &closure {
            self.body["policies"][id]["body"]["reads"] = set(reads.iter().cloned());
        }
        for (id, selection) in &self.join_selections {
            let node = &self.body["nodes"][id];
            let source = node["source"].as_str().expect("join source");
            let source = &self.body["nodes"][source];
            let population: BTreeSet<String> = match source["kind"].as_str() {
                Some("split") => source["children"]
                    .as_object()
                    .expect("children")
                    .values()
                    .map(|v| v.as_str().expect("scope").into())
                    .collect(),
                Some("fanout") => BTreeSet::from([source["body"].as_str().expect("scope").into()]),
                _ => return Err(error("join-source", 0..0)),
            };
            if source["join"] != *id {
                return Err(error("join-pair", 0..0));
            }
            for chosen in selection {
                if chosen
                    .iter()
                    .any(|key| !population.contains(&self.ledger.symbols[key].owner))
                {
                    return Err(error("join-population", 0..0));
                }
                let tokens: BTreeSet<_> = chosen
                    .iter()
                    .map(|key| &self.ledger.symbols[key].wire)
                    .collect();
                let matches: BTreeSet<_> = self
                    .ledger
                    .symbols
                    .values()
                    .filter(|s| {
                        s.role == "outcome"
                            && population.contains(&s.owner)
                            && tokens.contains(&s.wire)
                    })
                    .map(|s| s.id.clone())
                    .collect();
                if matches != *chosen {
                    return Err(error("join-wire-collision", 0..0));
                }
            }
        }
        let ids: Vec<_> = self.body["nodes"]
            .as_object()
            .expect("nodes")
            .keys()
            .cloned()
            .collect();
        for id in ids {
            let node = &self.body["nodes"][&id];
            let mut reads: BTreeSet<String> = node["reads"]
                .as_object()
                .expect("reads")
                .keys()
                .cloned()
                .collect();
            let scope = &self.body["scopes"][node["scope"].as_str().expect("scope")];
            for field in ["cancellation", "faults", "closure", "race"] {
                reads.extend(
                    closure[scope[field].as_str().expect("policy")]
                        .iter()
                        .cloned(),
                );
            }
            let mut expressions = Vec::new();
            for field in ["input", "condition", "collection", "itemKey", "seal"] {
                if let Some(expr) = node[field].as_str() {
                    expressions.push(expr);
                }
            }
            for field in ["assignments", "guards", "inputs"] {
                if let Some(map) = node[field].as_object() {
                    expressions.extend(map.values().filter_map(Value::as_str));
                }
            }
            for expr in expressions {
                reads.extend(
                    self.body["expressions"][expr]["reads"]
                        .as_object()
                        .ok_or_else(|| error("expression-reference", 0..0))?
                        .keys()
                        .cloned(),
                );
            }
            for field in ["policy", "remaining", "changes"] {
                if let Some(policy) = node[field].as_str() {
                    reads.extend(closure[policy].iter().cloned());
                }
            }
            if node["mode"] == "human" {
                let actor = node["target"].as_str().expect("actor");
                let policy = self.body["actors"][actor]["requirement"]
                    .as_str()
                    .ok_or_else(|| error("actor-requirement", 0..0))?;
                reads.extend(closure[policy].iter().cloned());
            }
            self.body["nodes"][&id]["reads"] = set(reads);
        }
        self.check_shape()
    }
    pub(super) fn check_shape(&self) -> Result<()> {
        let policy_kind = |id: &Value, kind: &str| -> Result<()> {
            let id = id.as_str().ok_or_else(|| error("policy-reference", 0..0))?;
            if self.body["policies"][id]["body"]["kind"] != kind {
                return Err(error("policy-kind", 0..0));
            }
            Ok(())
        };
        let same_keys = |a: &Value, b: &Value| {
            a.as_object().map(|m| m.keys().collect::<BTreeSet<_>>())
                == b.as_object().map(|m| m.keys().collect::<BTreeSet<_>>())
        };
        for (id, scope) in self.body["scopes"].as_object().expect("scopes") {
            if self.body["nodes"][scope["entry"].as_str().expect("entry")]["scope"] != *id {
                return Err(error("entry-scope", 0..0));
            }
            for field in ["inputs", "outputs"] {
                for cell in scope[field].as_object().expect("ports").values() {
                    if self.body["data"][cell.as_str().expect("cell")]["scope"] != *id {
                        return Err(error("port-ownership", 0..0));
                    }
                }
            }
            for (field, kind) in [
                ("cancellation", "cancel"),
                ("faults", "faults"),
                ("closure", "closure"),
                ("race", "race"),
            ] {
                policy_kind(&scope[field], kind)?;
            }
        }
        for actor in self.body["actors"].as_object().expect("actors").values() {
            policy_kind(&actor["requirement"], "actor")?;
        }
        for data in self.body["data"].as_object().expect("data").values() {
            policy_kind(&data["protection"]["policy"], "protection")?;
        }
        for policy in self.body["policies"]
            .as_object()
            .expect("policies")
            .values()
        {
            let p = &policy["body"];
            match p["kind"].as_str() {
                Some("cancel") => policy_kind(&p["children"], "settlement")?,
                Some("membership") => policy_kind(&p["removal"], "settlement")?,
                Some("effect") => policy_kind(&p["reconciliation"], "wait")?,
                _ => {}
            }
        }
        let mut graph: BTreeMap<String, BTreeSet<String>> = self.body["nodes"]
            .as_object()
            .expect("nodes")
            .keys()
            .map(|id| (id.clone(), BTreeSet::new()))
            .collect();
        for flow in self.body["flows"].as_object().expect("flows").values() {
            let source = flow["source"].as_str().expect("source");
            let target = flow["target"].as_str().expect("target");
            if self.body["nodes"][source]["scope"] != self.body["nodes"][target]["scope"] {
                return Err(error("flow-scope", 0..0));
            }
            graph.get_mut(source).expect("source").insert(target.into());
        }
        for (id, node) in self.body["nodes"].as_object().expect("nodes") {
            match node["kind"].as_str() {
                Some("activity") => policy_kind(&node["policy"], "work")?,
                Some("wait") => {
                    policy_kind(&node["policy"], "wait")?;
                    let p = node["policy"].as_str().expect("policy");
                    if self.wait_owners.get(p) != Some(id) {
                        return Err(error("wait-outcome-owner", 0..0));
                    }
                    let policy = &self.body["policies"][p]["body"];
                    let expected: BTreeSet<_> =
                        [policy["onObservation"].as_str(), policy["onTimer"].as_str()]
                            .into_iter()
                            .flatten()
                            .collect();
                    if expected
                        != node["outcomes"]
                            .as_object()
                            .expect("outcomes")
                            .keys()
                            .map(String::as_str)
                            .collect()
                    {
                        return Err(error("wait-outcomes", 0..0));
                    }
                }
                Some("join") => policy_kind(&node["remaining"], "settlement")?,
                Some("fanout") => policy_kind(&node["changes"], "membership")?,
                Some("decision") => {
                    let mut expected: BTreeSet<_> = node["guards"]
                        .as_object()
                        .expect("guards")
                        .keys()
                        .map(String::as_str)
                        .collect();
                    if let Some(default) = node["default"].as_str()
                        && !expected.insert(default)
                    {
                        return Err(error("decision-default", 0..0));
                    }
                    if expected
                        != node["outcomes"]
                            .as_object()
                            .expect("outcomes")
                            .keys()
                            .map(String::as_str)
                            .collect()
                    {
                        return Err(error("decision-outcomes", 0..0));
                    }
                }
                _ => {}
            }
            if let Some(child) = node["body"].as_str() {
                let child = &self.body["scopes"][child];
                match node["kind"].as_str() {
                    Some("invoke") => {
                        if !same_keys(&node["inputs"], &child["inputs"])
                            || !same_keys(&node["outputs"], &child["outputs"])
                            || !same_keys(&node["outcomes"], &child["outcomes"])
                        {
                            return Err(error("invoke-interface", 0..0));
                        }
                    }
                    Some("repeat") => {
                        if !child["inputs"].as_object().expect("inputs").is_empty()
                            || !child["outputs"].as_object().expect("outputs").is_empty()
                        {
                            return Err(error("repeat-interface", 0..0));
                        }
                    }
                    Some("fanout")
                        if child["inputs"].as_object().expect("inputs").len() != 1
                            || !child["outputs"].as_object().expect("outputs").is_empty() =>
                    {
                        return Err(error("fanout-interface", 0..0));
                    }
                    _ => {}
                }
            }
            let count = node["outcomes"].as_object().expect("outcomes").len();
            match node["kind"].as_str().expect("kind") {
                "finish" | "split" | "fanout" if count != 0 => {
                    return Err(error("node-outcomes", 0..0));
                }
                "compute" | "join" | "repeat" if count != 1 => {
                    return Err(error("node-outcomes", 0..0));
                }
                "activity" | "invoke" | "decision" | "wait" if count == 0 => {
                    return Err(error("node-outcomes", 0..0));
                }
                _ => {}
            }
            if let Some(child) = node["body"].as_str()
                && self.body["scopes"][child]["parent"] != node["scope"]
            {
                return Err(error("child-template", 0..0));
            }
            if node["kind"] == "split" {
                for child in node["children"]
                    .as_object()
                    .expect("children")
                    .values()
                    .filter_map(Value::as_str)
                {
                    if self.body["scopes"][child]["parent"] != node["scope"] {
                        return Err(error("child-template", 0..0));
                    }
                    if !self.body["scopes"][child]["inputs"]
                        .as_object()
                        .expect("inputs")
                        .is_empty()
                        || !self.body["scopes"][child]["outputs"]
                            .as_object()
                            .expect("outputs")
                            .is_empty()
                    {
                        return Err(error("split-interface", 0..0));
                    }
                }
            }
            if matches!(node["kind"].as_str(), Some("split" | "fanout")) {
                let join = &self.body["nodes"][node["join"].as_str().expect("join")];
                if join["kind"] != "join" || join["source"] != *id || join["scope"] != node["scope"]
                {
                    return Err(error("join-pair", 0..0));
                }
                graph
                    .get_mut(id)
                    .expect("node")
                    .insert(node["join"].as_str().expect("join").into());
            }
        }
        // Iterative topological check, independent of JSON nesting and scope depth.
        let mut indegrees: BTreeMap<_, usize> = graph.keys().map(|id| (id.clone(), 0)).collect();
        for targets in graph.values() {
            for target in targets {
                *indegrees
                    .get_mut(target)
                    .ok_or_else(|| error("flow-reference", 0..0))? += 1;
            }
        }
        let mut ready: Vec<_> = indegrees
            .iter()
            .filter(|(_, n)| **n == 0)
            .map(|(id, _)| id.clone())
            .collect();
        let mut seen = 0;
        while let Some(id) = ready.pop() {
            seen += 1;
            for target in &graph[&id] {
                let n = indegrees.get_mut(target).expect("node");
                *n -= 1;
                if *n == 0 {
                    ready.push(target.clone());
                }
            }
        }
        if seen != graph.len() {
            return Err(error("flow-cycle", 0..0));
        }
        Ok(())
    }
}
