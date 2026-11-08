// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

impl<'a, 's> Lower<'a, 's> {
    fn wait_owner(
        &mut self,
        tree: &Tree,
        scope: &str,
        policy: &str,
        export: bool,
    ) -> Result<String> {
        let reference = tree.one("reference")?;
        let ordinary = self.reference(reference, scope, "node");
        if !export
            || ordinary.is_ok()
            || ordinary
                .as_ref()
                .is_err_and(|e| e.code != "reference-missing")
        {
            return ordinary;
        }
        let parts = self.parts(reference)?;
        if parts.len() < 2 {
            return ordinary;
        }
        let scope_lookup = self.ledger.resolve(scope, &parts[1..], "scope");
        let target = match scope_lookup {
            Err(e)
                if matches!(
                    self.syntax.profile(),
                    "0.5.0" | "0.6.0" | "0.7.0" | "0.8.0" | "0.9.0" | "0.10.0" | "0.11.0"
                ) && e.code == "reference-missing" =>
            {
                self.ledger.endpoint_scope(scope, &parts[1..])
            }
            other => other,
        }
        .map_err(|mut e| {
            e.span = self.syntax.span(reference);
            e
        })?;
        let closure = self.body["scopes"][scope]["closure"]
            .as_str()
            .expect("bound closure");
        let close = &self.body["policies"][closure]["body"];
        if !matches!(
            self.syntax.profile(),
            "0.5.0" | "0.6.0" | "0.7.0" | "0.8.0" | "0.9.0" | "0.10.0" | "0.11.0"
        ) && (target.scope.as_deref() != Some(scope)
            || close["mode"] != "reconcile"
            || close["followup"] != target.id)
        {
            return Err(error("wait-endpoint-boundary", self.syntax.span(reference)));
        }
        let node = self
            .ledger
            .resolve(&target.id, &parts[..1], "node")
            .map_err(|mut e| {
                e.span = self.syntax.span(reference);
                e
            })?;
        if node.scope.as_deref() != Some(target.id.as_str()) {
            return Err(error("wait-endpoint-boundary", self.syntax.span(reference)));
        }
        let id = node.id.clone();
        self.exported_endpoints.push((
            policy.into(),
            id.clone(),
            target.id.clone(),
            self.syntax.span(reference),
        ));
        Ok(id)
    }

    pub(super) fn wait_outcomes(
        &mut self,
        tree: &Tree,
        scope: &str,
        policy: &str,
        export: bool,
    ) -> Result<(Value, Value)> {
        let outcomes = tree.all("outcome_reference");
        let mut owner = None;
        let mut wires = Vec::new();
        for outcome in outcomes {
            let resolved = self.wait_owner(outcome, scope, policy, export)?;
            if owner.as_ref().is_some_and(|id| id != &resolved) {
                return Err(error("wait-outcome-owner", self.syntax.span(outcome)));
            }
            let name = self.syntax.text(outcome.one("NAME")?)?;
            let symbol = self
                .ledger
                .symbol(&resolved, "outcome", name)
                .map_err(|mut e| {
                    e.span = self.syntax.span(outcome);
                    e
                })?;
            wires.push(Value::String(symbol.wire.clone()));
            owner = Some(resolved);
        }
        self.wait_owners
            .insert(policy.into(), owner.expect("grammar requires outcome"));
        Ok((
            wires[0].clone(),
            wires.get(1).cloned().unwrap_or(Value::Null),
        ))
    }

    pub(super) fn resolve_waits(&mut self) -> Result<()> {
        for (tree, scope, policy) in std::mem::take(&mut self.pending_waits) {
            let (observation, timer) = self.wait_outcomes(tree, &scope, &policy, true)?;
            let body = &mut self.body.get_mut("policies").expect("policies")[&policy]["body"];
            if body["timer"].is_null() != timer.is_null() {
                return Err(error("wait-timer-outcome", self.syntax.span(tree)));
            }
            body["onObservation"] = observation;
            body["onTimer"] = timer;
        }
        Ok(())
    }

    pub(super) fn validate_endpoints(&self) -> Result<()> {
        // One immutable declared-use index, not a runtime ownership/permission
        // cache and not a full node scan for every exported observation/timer.
        let mut uses: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        if matches!(
            self.syntax.profile(),
            "0.5.0" | "0.6.0" | "0.7.0" | "0.8.0" | "0.9.0" | "0.10.0" | "0.11.0"
        ) {
            for node in self.body["nodes"].as_object().expect("nodes").values() {
                if node["kind"] != "activity" || node["mode"] != "capability" {
                    continue;
                }
                let Some(capability) = node["target"].as_str() else {
                    continue;
                };
                let Some(effect) = self.body["capabilities"][capability]["effects"].as_str() else {
                    continue;
                };
                if let Some(wait) = self.body["policies"][effect]["body"]["reconciliation"].as_str()
                {
                    uses.entry(wait)
                        .or_default()
                        .insert(node["scope"].as_str().expect("bound node scope"));
                }
            }
        }
        for (policy, node, scope, span) in &self.exported_endpoints {
            let n = &self.body["nodes"][node];
            if n["kind"] != "wait" || n["policy"] != *policy || n["scope"] != *scope {
                return Err(error("wait-endpoint-owner", span.clone()));
            }
            if matches!(
                self.syntax.profile(),
                "0.5.0" | "0.6.0" | "0.7.0" | "0.8.0" | "0.9.0" | "0.10.0" | "0.11.0"
            ) {
                let declaring_scope = self.body["policies"][policy]["scope"]
                    .as_str()
                    .expect("bound policy scope");
                let owns = |owner: &str| {
                    let Some(closure) = self.body["scopes"][owner]["closure"].as_str() else {
                        return false;
                    };
                    let close = &self.body["policies"][closure]["body"];
                    close["mode"] == "reconcile"
                        && close["followup"] == *scope
                        && self.body["scopes"][scope]["parent"] == owner
                };
                if !owns(declaring_scope)
                    && !uses.get(policy.as_str()).is_some_and(|owners| {
                        !owners.is_empty() && owners.iter().all(|owner| owns(owner))
                    })
                {
                    return Err(error("wait-endpoint-use-unproved", span.clone()));
                }
            }
        }
        Ok(())
    }
}
