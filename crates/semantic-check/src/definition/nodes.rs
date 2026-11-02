// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

impl Check<'_, '_> {
    pub(super) fn child(&self, scope: &str, id: &Value) -> Result<()> {
        let id = text(id)?;
        let child = object(&self.b["scopes"])?
            .get(id)
            .ok_or_else(|| err("scope-reference"))?;
        require(child["parent"] == scope, "child-scope")
    }
    fn expr(&self, scope: &str, id: &Value, item: bool) -> Result<&Judgment> {
        let id = self.visible(scope, "expressions", id)?;
        if !item {
            require(
                object(&self.b["expressions"][&id]["parameters"])?.is_empty(),
                "parameter-binding",
            )?;
        }
        self.expressions
            .get(&id)
            .ok_or_else(|| err("expression-reference"))
    }
    pub(super) fn nodes(&mut self) -> Result<()> {
        for (scope, s) in object(&self.b["scopes"])? {
            require(!names(&s["outcomes"])?.is_empty(), "scope-outcomes")?;
            for (field, kind) in [
                ("cancellation", "cancel"),
                ("faults", "faults"),
                ("closure", "closure"),
                ("race", "race"),
            ] {
                self.policy_kind(scope, &s[field], kind)?;
            }
            for cell in object(&s["inputs"])?.values() {
                require(
                    self.b["data"][text(cell)?].get("initial").is_none(),
                    "input-initializer",
                )?;
            }
        }
        for (id, actor) in object(&self.b["actors"])? {
            self.actor(text(&actor["scope"])?, &Value::from(id.as_str()))?;
        }
        for (id, data) in object(&self.b["data"])? {
            let scope = text(&data["scope"])?;
            self.protection(scope, &data["protection"])?;
            if let Some(initial) = data.get("initial") {
                require(
                    self.expr(scope, initial, false)?.ty == self.cells[id],
                    "initializer-type",
                )?;
            }
            for node in names(&data["invalidates"])? {
                self.visible(
                    text(&self.b["nodes"][&node]["scope"])?,
                    "data",
                    &Value::from(id.as_str()),
                )?;
            }
        }
        for c in object(&self.b["capabilities"])?.values() {
            let scope = text(&c["scope"])?;
            let policy = self.policy_kind(scope, &c["contract"], "capability")?;
            let p = self.resources.provider(&Pin::decode(
                &self.b["policies"][&policy]["body"]["provider"],
            )?)?;
            require(
                self.wrapped(&c["input"])? == p.input && self.wrapped(&c["output"])? == p.output,
                "capability-type",
            )?;
            self.actor(scope, &c["authority"])?;
            let effects = self.policy_kind(scope, &c["effects"], "effect")?;
            let effects = &self.b["policies"][&effects]["body"];
            require(
                effects["class"] == p.effect && effects["idempotency"] == p.idempotency,
                "capability-effect",
            )?;
        }
        for (id, node) in object(&self.b["nodes"])? {
            self.node(node).map_err(|mut e| {
                e.path = format!("/body/nodes/{id}");
                e
            })?;
        }
        Ok(())
    }
    fn node(&mut self, n: &Value) -> Result<()> {
        let scope = text(&n["scope"])?;
        let kind = text(&n["kind"])?;
        let declared = names(&n["reads"])?;
        let writes = names(&n["writes"])?;
        for cell in declared.union(&writes) {
            self.visible(scope, "data", &Value::from(cell.as_str()))?;
        }
        let mut required = BTreeSet::new();
        let mut policies = BTreeSet::new();
        for field in ["cancellation", "faults", "closure", "race"] {
            policies.insert(text(&self.b["scopes"][scope][field])?.to_owned());
        }
        for field in ["input", "condition", "collection", "itemKey", "seal"] {
            if let Some(expr) = n.get(field) {
                required.extend(
                    self.expr(scope, expr, field == "itemKey")?
                        .reads
                        .iter()
                        .cloned(),
                );
            }
        }
        for field in ["assignments", "inputs", "guards"] {
            if let Some(map) = n.get(field) {
                for expr in object(map)?.values() {
                    required.extend(self.expr(scope, expr, false)?.reads.iter().cloned());
                }
            }
        }
        for field in ["policy", "remaining", "changes"] {
            if let Some(p) = n.get(field) {
                policies.insert(self.visible(scope, "policies", p)?);
            }
        }
        let outcomes = names(&n["outcomes"])?;
        match kind {
            "activity" => {
                self.policy_kind(scope, &n["policy"], "work")?;
                let result = self.result_type(n)?;
                let target_type = n["result"].as_str().map_or_else(
                    || Ok(Type::empty()),
                    |id| self.cells.get(id).cloned().ok_or_else(|| err("reference")),
                )?;
                require(result == target_type, "activity-result")?;
                let expected = n["result"]
                    .as_str()
                    .map(|s| BTreeSet::from([s.into()]))
                    .unwrap_or_default();
                require(writes == expected, "activity-writes")?;
                if n["mode"] == "human" {
                    policies.insert(self.actor(scope, &n["target"])?);
                    match result {
                        Type::Variant(cases) => require(
                            cases.keys().cloned().collect::<BTreeSet<_>>() == outcomes,
                            "human-outcomes",
                        )?,
                        t => require(t == Type::empty() && outcomes.len() == 1, "human-outcomes")?,
                    }
                } else {
                    let cap = self.visible(scope, "capabilities", &n["target"])?;
                    let c = &self.b["capabilities"][&cap];
                    policies.extend(self.capability_refs(scope, &n["target"])?);
                    let input = self.wrapped(&c["input"])?;
                    require(
                        self.expr(scope, &n["input"], false)?.ty == input,
                        "activity-input",
                    )?;
                    let p = text(&c["contract"])?;
                    let provider = self
                        .resources
                        .provider(&Pin::decode(&self.b["policies"][p]["body"]["provider"])?)?;
                    require(outcomes == provider.outcomes, "provider-outcomes")?;
                    if let Some(cell) = n["result"].as_str() {
                        self.flow(&provider.protection, &self.b["data"][cell]["protection"])?;
                    }
                }
                require(!outcomes.is_empty(), "node-outcomes")?;
            }
            "compute" => {
                let assignments = object(&n["assignments"])?;
                require(
                    writes == assignments.keys().cloned().collect() && outcomes.len() == 1,
                    "compute-interface",
                )?;
                for (cell, expr) in assignments {
                    self.visible(scope, "data", &Value::from(cell.as_str()))?;
                    require(
                        self.expr(scope, expr, false)?.ty == self.cells[cell],
                        "assignment-type",
                    )?;
                }
            }
            "decision" => {
                let mut expected: BTreeSet<_> = object(&n["guards"])?.keys().cloned().collect();
                for expr in object(&n["guards"])?.values() {
                    require(
                        self.expr(scope, expr, false)?.ty == Type::Boolean,
                        "guard-type",
                    )?;
                }
                if let Some(default) = n["default"].as_str() {
                    require(expected.insert(default.into()), "decision-default")?;
                }
                require(
                    expected == outcomes && !outcomes.is_empty(),
                    "decision-outcomes",
                )?;
            }
            "invoke" => {
                self.child(scope, &n["body"])?;
                let child = &self.b["scopes"][text(&n["body"])?];
                for (ports, bindings, input) in [
                    (&child["inputs"], &n["inputs"], true),
                    (&child["outputs"], &n["outputs"], false),
                ] {
                    let ports = object(ports)?;
                    let bindings = object(bindings)?;
                    require(ports.keys().eq(bindings.keys()), "invoke-ports")?;
                    for (port, cell) in ports {
                        let ty = &self.cells[text(cell)?];
                        let bound = if input {
                            self.expr(scope, &bindings[port], false)?.ty.clone()
                        } else {
                            let id = self.visible(scope, "data", &bindings[port])?;
                            require(writes.contains(&id), "invoke-writes")?;
                            self.cells[&id].clone()
                        };
                        require(&bound == ty, "invoke-type")?;
                        if !input {
                            // Child outputs are not lexical parent reads; check
                            // the actual port transfer, not just reads × writes.
                            self.resources.budget.tick(0)?;
                            self.flow(
                                &self.b["data"][text(cell)?]["protection"],
                                &self.b["data"][text(&bindings[port])?]["protection"],
                            )?;
                        }
                    }
                }
                require(names(&child["outcomes"])? == outcomes, "invoke-outcomes")?;
            }
            "repeat" | "fanout" => {
                self.child(scope, &n["body"])?;
                let child = &self.b["scopes"][text(&n["body"])?];
                if kind == "repeat" {
                    require(
                        self.expr(scope, &n["condition"], false)?.ty == Type::Boolean,
                        "condition-type",
                    )?;
                    require(
                        object(&child["inputs"])?.is_empty()
                            && object(&child["outputs"])?.is_empty()
                            && names(&child["outcomes"])?.len() == 1
                            && outcomes.len() == 1,
                        "repeat-interface",
                    )?;
                } else {
                    let item = text(&n["item"])?;
                    require(
                        self.b["data"][item]["scope"] == n["body"]
                            && self.b["data"][item].get("initial").is_none(),
                        "fanout-item",
                    )?;
                    require(
                        self.expr(scope, &n["collection"], false)?.ty
                            == Type::List(Box::new(self.cells[item].clone())),
                        "fanout-collection",
                    )?;
                    require(
                        self.expr(scope, &n["itemKey"], true)?.ty == Type::Text
                            && self.expr(scope, &n["seal"], false)?.ty == Type::Boolean,
                        "fanout-key-seal",
                    )?;
                    require(
                        child["inputs"] == serde_json::json!({"item":item})
                            && object(&child["outputs"])?.is_empty()
                            && outcomes.is_empty(),
                        "fanout-interface",
                    )?;
                    self.policy_kind(scope, &n["changes"], "membership")?;
                }
            }
            "split" => {
                require(outcomes.is_empty(), "split-outcomes")?;
                for child in object(&n["children"])?.values() {
                    self.child(scope, child)?;
                    let c = &self.b["scopes"][text(child)?];
                    require(
                        object(&c["inputs"])?.is_empty() && object(&c["outputs"])?.is_empty(),
                        "split-ports",
                    )?;
                }
            }
            "join" => {
                self.policy_kind(scope, &n["remaining"], "settlement")?;
                require(outcomes.len() == 1, "join-outcomes")?;
            }
            "wait" => {
                let id = self.policy_kind(scope, &n["policy"], "wait")?;
                let p = &self.b["policies"][&id]["body"];
                require(
                    outcomes == BTreeSet::from([text(&p["onObservation"])?.into()]),
                    "wait-outcomes",
                )?;
            }
            "finish" => require(
                outcomes.is_empty()
                    && names(&self.b["scopes"][scope]["outcomes"])?.contains(text(&n["outcome"])?),
                "finish-outcome",
            )?,
            _ => return Err(err("node-kind")),
        }
        if !matches!(kind, "activity" | "compute" | "invoke") {
            require(writes.is_empty(), "node-writes")?;
        }
        for p in policies {
            required.extend(
                self.policy_reads
                    .get(&p)
                    .ok_or_else(|| err("policy-reference"))?
                    .iter()
                    .cloned(),
            );
        }
        // Account for policies consulted by every data access, including newly
        // introduced policy reads. Conservative declared reads retain obligations.
        let mut queue: Vec<_> = declared.union(&writes).cloned().collect();
        let mut seen = BTreeSet::new();
        while let Some(cell) = queue.pop() {
            if !seen.insert(cell.clone()) {
                continue;
            }
            let data = &self.b["data"][&cell];
            for policy in self.protection(text(&data["scope"])?, &data["protection"])? {
                for read in &self.policy_reads[&policy] {
                    if required.insert(read.clone()) {
                        queue.push(read.clone());
                    }
                }
            }
        }
        require(required.is_subset(&declared), "node-reads")?;
        for source in &declared {
            for target in &writes {
                self.resources.budget.tick(0)?;
                self.flow(
                    &self.b["data"][source]["protection"],
                    &self.b["data"][target]["protection"],
                )?;
            }
        }
        Ok(())
    }
    /// Direct transfer only: retain sensitivity/policy identity and never widen
    /// access sets. This does not establish whole-process information flow.
    fn flow(&self, source: &Value, target: &Value) -> Result<()> {
        require(
            source["sensitivity"] == target["sensitivity"] && source["policy"] == target["policy"],
            "information-flow",
        )?;
        for field in ["purposes", "participants", "capabilities"] {
            require(
                names(&target[field])?.is_subset(&names(&source[field])?),
                "information-flow",
            )?;
        }
        Ok(())
    }
}
