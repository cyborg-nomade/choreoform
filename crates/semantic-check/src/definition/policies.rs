// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

impl Check<'_, '_> {
    pub(super) fn policies(&mut self) -> Result<()> {
        let mut local = BTreeMap::new();
        let mut edges = BTreeMap::new();
        for (id, record) in object(&self.b["policies"])? {
            let result = self.policy(id, record).map_err(|mut e| {
                e.path = format!("/body/policies/{id}");
                e
            })?;
            local.insert(id.clone(), result.0);
            edges.insert(id.clone(), result.1);
        }
        let mut dependents: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut degrees = BTreeMap::new();
        for (id, targets) in &edges {
            degrees.insert(id.clone(), targets.len());
            for target in targets {
                require(edges.contains_key(target), "policy-reference")?;
                dependents
                    .entry(target.clone())
                    .or_default()
                    .push(id.clone());
            }
        }
        let mut ready: Vec<_> = degrees
            .iter()
            .filter(|(_, n)| **n == 0)
            .map(|(id, _)| id.clone())
            .collect();
        let mut depths: BTreeMap<String, usize> = BTreeMap::new();
        while let Some(id) = ready.pop() {
            let depth = 1 + edges[&id]
                .iter()
                .map(|target| depths[target])
                .max()
                .unwrap_or(0);
            require(depth <= MAX_DEPTH, "policy-depth")?;
            depths.insert(id.clone(), depth);
            self.resources.budget.tick(0)?;
            let mut reads = local.remove(&id).ok_or_else(|| err("policy-cycle"))?;
            for target in &edges[&id] {
                reads.extend(self.policy_reads[target].iter().cloned());
            }
            require(
                reads == names(&self.b["policies"][&id]["body"]["reads"])?,
                "policy-reads",
            )?;
            self.policy_reads.insert(id.clone(), reads);
            for user in dependents.get(&id).into_iter().flatten() {
                let degree = degrees
                    .get_mut(user)
                    .ok_or_else(|| err("policy-reference"))?;
                *degree -= 1;
                if *degree == 0 {
                    ready.push(user.clone());
                }
            }
        }
        require(self.policy_reads.len() == degrees.len(), "policy-cycle")
    }
    fn policy(&mut self, id: &str, r: &Value) -> Result<(BTreeSet<String>, BTreeSet<String>)> {
        self.dialect(r)?;
        let scope = text(&r["scope"])?;
        let p = &r["body"];
        let kind = text(&p["kind"])?;
        let extra: &[&str] = match kind {
            "actor" | "protection" => &["permit"],
            "capability" => &["provider"],
            "effect" => &["class", "idempotency", "reconciliation"],
            "work" => &[
                "instructions",
                "authority",
                "retry",
                "timeout",
                "completion",
                "compensates",
            ],
            "faults" => &["handlers"],
            "wait" => &[
                "observationType",
                "match",
                "authority",
                "timer",
                "onObservation",
                "onTimer",
            ],
            "cancel" => &["authority", "children"],
            "settlement" => &["unfinished", "owner"],
            "closure" => &["mode", "followup"],
            "race" => &["mode", "invariant"],
            "membership" => &["removal", "changed"],
            _ => return Err(err("policy-kind")),
        };
        let mut keys = vec!["kind", "reads"];
        keys.extend(extra);
        fields(p, &keys)?;
        let mut reads = BTreeSet::new();
        let mut refs = BTreeSet::new();
        let mut context = self.context(scope)?;
        let text_facts = |context: &mut Context, names: &[&str]| {
            for key in names {
                context.facts.insert((*key).into(), Type::Text);
            }
        };
        let mut predicates: Vec<(&Value, Context)> = Vec::new();
        match kind {
            "actor" => {
                text_facts(&mut context, &["principal", "inputRevision"]);
                for key in [
                    "roles",
                    "authorities",
                    "relationships",
                    "priorDecisionPrincipals",
                ] {
                    context
                        .facts
                        .insert(key.into(), Type::List(Box::new(Type::Text)));
                }
                predicates.push((&p["permit"], context));
            }
            "protection" => {
                text_facts(
                    &mut context,
                    &["principal", "purpose", "operation", "cell", "revision"],
                );
                context
                    .facts
                    .insert("capability".into(), Type::Option(Box::new(Type::Text)));
                predicates.push((&p["permit"], context));
            }
            "capability" => {
                refs.extend(self.provider_protection(scope, &Pin::decode(&p["provider"])?)?);
            }
            "effect" => {
                require(
                    ["read-only", "compensatable", "irreversible"].contains(&text(&p["class"])?),
                    "effect-class",
                )?;
                require(
                    ["none", "stable-key"].contains(&text(&p["idempotency"])?),
                    "idempotency",
                )?;
                refs.insert(self.policy_kind(scope, &p["reconciliation"], "wait")?);
            }
            "work" => {
                name(&p["instructions"])?;
                refs.insert(self.actor(scope, &p["authority"])?);
                let retry = &p["retry"];
                fields(retry, &["maxAttempts", "delays", "faults"])?;
                let attempts = retry["maxAttempts"]
                    .as_u64()
                    .filter(|n| *n > 0 && *n <= 1000)
                    .ok_or_else(|| err("retry-attempts"))?;
                let delays = items(&retry["delays"])?;
                require(delays.len() as u64 + 1 == attempts, "retry-delays")?;
                for delay in delays {
                    require(integer(delay)? >= 0, "retry-delay")?;
                }
                names(&retry["faults"])?;
                let timer = self.timer(scope, &p["timeout"], &context)?;
                reads.extend(timer.0);
                refs.extend(timer.1);
                if !p["compensates"].is_null() {
                    let prior = self.visible(scope, "nodes", &p["compensates"])?;
                    require(
                        self.b["nodes"][&prior]["kind"] == "activity"
                            && self.b["nodes"][&prior]["mode"] == "capability",
                        "compensation-target",
                    )?;
                }
                text_facts(
                    &mut context,
                    &["principal", "inputRevision", "occurrence", "observation"],
                );
                let users: Vec<_> = object(&self.b["nodes"])?
                    .values()
                    .filter(|n| n["kind"] == "activity" && n["policy"] == id)
                    .collect();
                if users.is_empty() {
                    context.facts.insert("result".into(), Type::empty());
                    predicates.push((&p["completion"], context));
                } else {
                    for user in users {
                        let mut c = Context {
                            cells: context.cells.clone(),
                            parameters: BTreeMap::new(),
                            facts: context.facts.clone(),
                        };
                        c.facts.insert("result".into(), self.result_type(user)?);
                        predicates.push((&p["completion"], c));
                    }
                }
            }
            "faults" => {
                for (fault, child) in object(&p["handlers"])? {
                    name(&Value::from(fault.as_str()))?;
                    self.child(scope, child)?;
                    let child = &self.b["scopes"][text(child)?];
                    require(
                        object(&child["inputs"])?.is_empty()
                            && object(&child["outputs"])?.is_empty(),
                        "handler-ports",
                    )?;
                }
            }
            "wait" => {
                let ty = self.resources.type_descriptor(&p["observationType"])?;
                let timer = self.timer(scope, &p["timer"], &context)?;
                reads.extend(timer.0);
                refs.extend(timer.1);
                text_facts(
                    &mut context,
                    &["principal", "observation", "occurrence", "inputRevision"],
                );
                context.facts.insert("payload".into(), ty);
                refs.insert(self.actor(scope, &p["authority"])?);
                name(&p["onObservation"])?;
                require(
                    p["timer"].is_null() == p["onTimer"].is_null(),
                    "wait-timer-outcome",
                )?;
                if !p["onTimer"].is_null() {
                    name(&p["onTimer"])?;
                    require(p["onTimer"] != p["onObservation"], "wait-timer-outcome")?;
                }
                predicates.push((&p["match"], context));
            }
            "cancel" => {
                refs.insert(self.actor(scope, &p["authority"])?);
                refs.insert(self.policy_kind(scope, &p["children"], "settlement")?);
            }
            "settlement" => match text(&p["unfinished"])? {
                "cancel" | "retain" => require(p["owner"].is_null(), "settlement-owner")?,
                "transfer" => require(
                    self.ancestors(scope)?.contains(text(&p["owner"])?) && p["owner"] != scope,
                    "settlement-owner",
                )?,
                _ => return Err(err("settlement-mode")),
            },
            "closure" => match text(&p["mode"])? {
                "terminal" => require(p["followup"].is_null(), "closure-followup")?,
                "reconcile" => {
                    self.child(scope, &p["followup"])?;
                    let child = &self.b["scopes"][text(&p["followup"])?];
                    require(
                        object(&child["inputs"])?.is_empty()
                            && object(&child["outputs"])?.is_empty(),
                        "closure-ports",
                    )?;
                }
                _ => return Err(err("closure-mode")),
            },
            "race" => match text(&p["mode"])? {
                "acceptance" => require(p["invariant"].is_null(), "race-invariant")?,
                "guarded" => predicates.push((&p["invariant"], context)),
                _ => return Err(err("race-mode")),
            },
            "membership" => {
                refs.insert(self.policy_kind(scope, &p["removal"], "settlement")?);
                require(p["changed"] == "invalidate", "membership-change")?;
            }
            _ => unreachable!("closed policy"),
        }
        for (predicate, context) in predicates {
            let j = self.resources.expression(predicate, &context)?;
            require(j.ty == Type::Boolean, "predicate-type")?;
            reads.extend(j.reads);
        }
        Ok((reads, refs))
    }
    fn timer(
        &mut self,
        scope: &str,
        timer: &Value,
        context: &Context,
    ) -> Result<(BTreeSet<String>, BTreeSet<String>)> {
        if timer.is_null() {
            return Ok((BTreeSet::new(), BTreeSet::new()));
        }
        fields(
            timer,
            &["clock", "basis", "due", "calendar", "pauseAuthority"],
        )?;
        let pin = Pin::decode(&timer["clock"])?;
        let clock = self.resources.clock(&pin)?;
        if !timer["calendar"].is_null() {
            let calendar = self.resources.calendar(&Pin::decode(&timer["calendar"])?)?;
            require(calendar.clock() == &pin, "calendar-clock")?;
            return Err(err("calendar-domain-binding-unproved"));
        }
        let due = self.resources.expression(&timer["due"], context)?;
        require(due.ty == Type::Integer, "timer-due-type")?;
        let mut reads = due.reads;
        reads.insert(self.visible(scope, "data", &timer["basis"])?);
        let mut refs = self.provider_protection(scope, clock.source())?;
        refs.insert(self.actor(scope, &timer["pauseAuthority"])?);
        Ok((reads, refs))
    }
    pub(super) fn result_type(&mut self, node: &Value) -> Result<Type> {
        if node["result"].is_null() {
            return Ok(Type::empty());
        }
        if node["mode"] == "capability" {
            let c = &self.b["capabilities"][text(&node["target"])?];
            self.wrapped(&c["output"])
        } else if let Some(cell) = node["result"].as_str() {
            Ok(self.cells[cell].clone())
        } else {
            Ok(Type::empty())
        }
    }
}
