// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

use super::*;

impl<'a, 's> Lower<'a, 's> {
    pub(super) fn optional_reference(&self, tree: &Tree, scope: &str, kind: &str) -> Result<Value> {
        if let Some(reference) = tree.all("reference").first() {
            Ok(self.reference(reference, scope, kind)?.into())
        } else {
            Ok(Value::Null)
        }
    }
    pub(super) fn timer(&mut self, tree: &Tree, scope: &str) -> Result<Value> {
        if self.syntax.word(tree, 0)? == "none" {
            return Ok(Value::Null);
        }
        let clock = self.r(tree, scope, "resource", 0)?;
        let calendar = tree.one("optional_reference")?;
        let calendar = if let Some(reference) = calendar.all("reference").first() {
            let id = self.reference(reference, scope, "resource")?;
            self.ledger.pin(&id, "calendar")?
        } else {
            Value::Null
        };
        Ok(
            json!({"clock":self.ledger.pin(&clock, "clock")?, "basis":self.r(tree, scope, "data", 1)?, "due":self.expression(tree.one("expression")?, scope, true, 0)?, "calendar":calendar, "pauseAuthority":self.r(tree, scope, "actor", 2)?}),
        )
    }
    pub(super) fn policy(&mut self, tree: &Tree, scope: &str) -> Result<()> {
        let id = self.declare(tree, Some(scope), "policy")?;
        let kind = tree
            .rule
            .strip_suffix("_policy")
            .ok_or_else(|| error("internal-tree", self.syntax.span(tree)))?;
        let mut body = json!({"kind":kind, "reads":{}});
        match kind {
            "actor" | "protection" => {
                body["permit"] = self.expression(tree.one("expression")?, scope, true, 0)?
            }
            "capability" => {
                let provider = self.r(tree, scope, "resource", 0)?;
                body["provider"] = self.ledger.pin(&provider, "provider")?;
            }
            "effect" => {
                body["class"] = self.syntax.word(tree, 7)?.into();
                body["idempotency"] = self.syntax.word(tree, 11)?.into();
                body["reconciliation"] = self.r(tree, scope, "policy", 0)?.into();
            }
            "work" => {
                body["instructions"] = self.syntax.text(tree.all("NAME")[1])?.into();
                body["authority"] = self.r(tree, scope, "actor", 0)?.into();
                let attempts = self
                    .syntax
                    .text(tree.one("NAT")?)?
                    .parse::<u16>()
                    .ok()
                    .filter(|n| *n > 0 && *n <= 1000)
                    .ok_or_else(|| error("retry-attempts", self.syntax.span(tree)))?;
                let delays = tree
                    .one("delays")?
                    .all("NAT")
                    .into_iter()
                    .map(|n| integer(self.syntax.text(n)?))
                    .collect::<Result<Vec<_>>>()?;
                if delays.len() + 1 != usize::from(attempts) {
                    return Err(error("retry-delays", self.syntax.span(tree)));
                }
                body["retry"] = json!({"maxAttempts":attempts, "delays":delays, "faults":unique_set(self.names(tree.one("names")?)?)?});
                body["timeout"] = self.timer(tree.one("timer")?, scope)?;
                body["completion"] = self.expression(tree.one("expression")?, scope, true, 0)?;
                body["compensates"] =
                    self.optional_reference(tree.one("optional_reference")?, scope, "node")?;
            }
            "faults" => {
                let mut handlers = Map::new();
                let names = self.names(tree)?;
                for (name, reference) in names.into_iter().skip(1).zip(tree.all("reference")) {
                    insert(
                        &mut handlers,
                        name,
                        self.reference(reference, scope, "scope")?.into(),
                    )?;
                }
                body["handlers"] = handlers.into();
            }
            "wait" => {
                body["observationType"] = self.type_value(tree.one("type")?, scope, 0)?;
                body["match"] = self.expression(tree.one("expression")?, scope, true, 0)?;
                body["authority"] = self.r(tree, scope, "actor", 0)?.into();
                body["timer"] = self.timer(tree.one("timer")?, scope)?;
                let outcomes = tree.all("outcome_reference");
                let owner = self.r(outcomes[0], scope, "node", 0)?;
                for outcome in &outcomes {
                    if self.r(outcome, scope, "node", 0)? != owner {
                        return Err(error("wait-outcome-owner", self.syntax.span(outcome)));
                    }
                }
                self.wait_owners.insert(id.clone(), owner);
                body["onObservation"] = self.alias(outcomes[0], scope)?.into();
                body["onTimer"] = if outcomes.len() == 2 {
                    self.alias(outcomes[1], scope)?.into()
                } else {
                    Value::Null
                };
                if body["timer"].is_null() != body["onTimer"].is_null() {
                    return Err(error("wait-timer-outcome", self.syntax.span(tree)));
                }
            }
            "cancel" => {
                body["authority"] = self.r(tree, scope, "actor", 0)?.into();
                body["children"] = self.r(tree, scope, "policy", 1)?.into();
            }
            "settlement" => {
                body["unfinished"] = self.syntax.word(tree, 7)?.into();
                body["owner"] = if let Some(reference) = tree.all("reference").first() {
                    self.ledger.ancestor(scope, &self.parts(reference)?)?.into()
                } else {
                    Value::Null
                };
            }
            "closure" => {
                let terminal = self.syntax.word(tree, 5)? == "Fully";
                body["mode"] = if terminal { "terminal" } else { "reconcile" }.into();
                body["followup"] = if terminal {
                    Value::Null
                } else {
                    self.r(tree, scope, "scope", 0)?.into()
                };
            }
            "race" => {
                let guarded = self.syntax.word(tree, 5)? == "Guard";
                body["mode"] = if guarded { "guarded" } else { "acceptance" }.into();
                body["invariant"] = if guarded {
                    self.expression(tree.one("expression")?, scope, true, 0)?
                } else {
                    Value::Null
                };
            }
            "membership" => {
                body["removal"] = self.r(tree, scope, "policy", 0)?.into();
                body["changed"] = "invalidate".into();
            }
            _ => return Err(error("policy-unsupported", self.syntax.span(tree))),
        }
        self.put(
            "policies",
            id,
            json!({"scope":scope, "dialect":self.ledger.dialect(), "body":body}),
        )
    }
}
