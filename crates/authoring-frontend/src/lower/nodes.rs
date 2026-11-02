// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

use super::*;

impl<'a, 's> Lower<'a, 's> {
    pub(super) fn step(&mut self, tree: &Tree, scope: &str) -> Result<()> {
        let id = self.declare(tree, Some(scope), "node")?;
        let action = tree.one("action")?.first()?;
        let kind = match action.rule {
            "human" | "request" => "activity",
            "invoke" => "invoke",
            other => other,
        };
        let mut node = json!({"kind":kind, "scope":scope, "outcomes":{}, "reads":{}, "writes":{}});
        if let Some(reads) = tree.all("references").first() {
            node["reads"] = self.references(reads, scope, "data")?;
        }
        let mut outcomes = Vec::new();
        for successor in tree.all("successor") {
            let name = self.syntax.text(successor.one("NAME")?)?.to_owned();
            let (symbol, wire) = self.declared_symbol(&id, "outcome", &name)?;
            let flow = self.slot(successor, &id, "flow", Some(&symbol))?;
            self.put(
                "flows",
                flow,
                json!({"source":id, "outcome":wire, "target":self.r(successor, scope, "node", 0)?}),
            )?;
            outcomes.push(wire);
        }
        node["outcomes"] = unique_set(outcomes)?;
        match action.rule {
            "human" | "request" => {
                node["mode"] = if action.rule == "human" {
                    "human"
                } else {
                    "capability"
                }
                .into();
                node["target"] = self
                    .r(
                        action,
                        scope,
                        if action.rule == "human" {
                            "actor"
                        } else {
                            "capability"
                        },
                        0,
                    )?
                    .into();
                node["input"] = self.r(action, scope, "expression", 1)?.into();
                node["policy"] = self.r(action, scope, "policy", 2)?.into();
                let result = action.one("result")?;
                node["result"] = if let Some(reference) = result.all("reference").first() {
                    self.reference(reference, scope, "data")?.into()
                } else {
                    Value::Null
                };
                if let Some(cell) = node["result"].as_str() {
                    node["writes"] = set([cell.into()]);
                }
            }
            "compute" => {
                let mut assignments = Map::new();
                for assignment in action.all("assignment") {
                    let cell = self.r(assignment, scope, "data", 0)?;
                    let expr = self.slot(assignment, &id, "assignment", Some(&cell))?;
                    let ast = self.expression(assignment.one("expression")?, scope, false, 0)?;
                    let ty = self.body["data"][&cell]["type"].clone();
                    self.expression_record(expr.clone(), scope, ast, ty, Map::new())?;
                    insert(&mut assignments, cell, expr.into())?;
                }
                node["writes"] = set(assignments.keys().cloned());
                node["assignments"] = assignments.into();
            }
            "invoke" => {
                let child = self.r(action, scope, "scope", 0)?;
                let ports = action.all("ports");
                node["body"] = child.clone().into();
                node["inputs"] = self.ports(ports[0], scope, &child, "input", "expression")?;
                node["outputs"] = self.ports(ports[1], scope, &child, "output", "data")?;
                node["writes"] = set(node["outputs"]
                    .as_object()
                    .expect("map")
                    .values()
                    .map(|v| v.as_str().expect("data ID").into()));
            }
            "decision" => {
                let mut guards = Map::new();
                for (name, reference) in
                    self.names(action)?.into_iter().zip(action.all("reference"))
                {
                    let wire = self.symbol(&id, "outcome", &name)?.1;
                    insert(
                        &mut guards,
                        wire,
                        self.reference(reference, scope, "expression")?.into(),
                    )?;
                }
                node["guards"] = guards.into();
                node["default"] = if self.names(action)?.len() > action.all("reference").len() {
                    let name = self.names(action)?.pop().expect("extra default");
                    self.symbol(&id, "outcome", &name)?.1.into()
                } else {
                    Value::Null
                };
            }
            "split" => {
                let mut children = Map::new();
                for (name, reference) in
                    self.names(action)?.into_iter().zip(action.all("reference"))
                {
                    let wire = self.declared_symbol(&id, "branch", &name)?.1;
                    insert(
                        &mut children,
                        wire,
                        self.reference(reference, scope, "scope")?.into(),
                    )?;
                }
                node["children"] = children.into();
                node["join"] = self
                    .r(action, scope, "node", action.all("reference").len() - 1)?
                    .into();
            }
            "join" => {
                node["source"] = self.r(action, scope, "node", 0)?.into();
                let mut selected = Vec::new();
                node["predicate"] =
                    self.join(action.one("join_condition")?, scope, 0, &mut selected)?;
                self.join_selections.insert(id.clone(), selected);
                node["remaining"] = self.r(action, scope, "policy", 1)?.into();
            }
            "wait" => node["policy"] = self.r(action, scope, "policy", 0)?.into(),
            "repeat" => {
                node["condition"] = self.r(action, scope, "expression", 0)?.into();
                node["body"] = self.r(action, scope, "scope", 1)?.into();
            }
            "fanout" => {
                node["collection"] = self.r(action, scope, "expression", 0)?.into();
                node["itemKey"] = self.r(action, scope, "expression", 1)?.into();
                let child = self.r(action, scope, "scope", 2)?;
                node["body"] = child.clone().into();
                let port = self
                    .symbol(&child, "input", self.syntax.text(action.one("NAME")?)?)?
                    .1;
                node["item"] = self.body["scopes"][&child]["inputs"]
                    .get(&port)
                    .cloned()
                    .ok_or_else(|| error("fanout-item-port", self.syntax.span(action)))?;
                node["seal"] = self.r(action, scope, "expression", 3)?.into();
                node["changes"] = self.r(action, scope, "policy", 4)?.into();
                node["join"] = self.r(action, scope, "node", 5)?.into();
            }
            "finish" => {
                node["outcome"] = self
                    .symbol(scope, "outcome", self.syntax.text(action.one("NAME")?)?)?
                    .1
                    .into()
            }
            _ => return Err(error("action-unsupported", self.syntax.span(action))),
        }
        self.put("nodes", id, node)
    }
    pub(super) fn join(
        &mut self,
        tree: &Tree,
        scope: &str,
        depth: usize,
        selected: &mut Vec<BTreeSet<String>>,
    ) -> Result<Value> {
        if depth > MAX_DEPTH {
            return Err(error("depth", self.syntax.span(tree)));
        }
        let word = self.syntax.word(tree, 0)?;
        if word == "both" || word == "either" {
            let terms = tree.all("join_condition");
            return Ok(
                json!({"op":if word == "both" {"and"} else {"or"}, "terms":{"left":self.join(terms[0], scope, depth + 1, selected)?, "right":self.join(terms[1], scope, depth + 1, selected)?}}),
            );
        }
        let mut tokens = Vec::new();
        let mut symbols = BTreeSet::new();
        for reference in tree.one("outcome_set")?.all("outcome_reference") {
            let owner = self.r(reference, scope, "scope", 0)?;
            let name = self.syntax.text(reference.one("NAME")?)?.to_owned();
            let (id, wire) = self.symbol(&owner, "outcome", &name)?;
            if !symbols.insert(id) {
                return Err(error("duplicate-join-outcome", self.syntax.span(reference)));
            }
            tokens.push(wire);
        }
        selected.push(symbols);
        let mut predicate =
            json!({"op":if word == "at" {"atLeast"} else {word}, "outcomes":set(tokens)});
        if word == "at" {
            let count = self
                .syntax
                .text(tree.one("NAT")?)?
                .parse::<u32>()
                .ok()
                .filter(|n| *n > 0)
                .ok_or_else(|| error("join-count", self.syntax.span(tree)))?;
            predicate["count"] = count.into();
        }
        Ok(predicate)
    }
}
