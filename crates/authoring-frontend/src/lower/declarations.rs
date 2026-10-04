// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

use super::*;

impl<'a, 's> Lower<'a, 's> {
    pub(super) fn references(&self, tree: &Tree, scope: &str, kind: &str) -> Result<Value> {
        unique_set(
            tree.all("reference")
                .into_iter()
                .map(|r| self.reference(r, scope, kind))
                .collect::<Result<Vec<_>>>()?,
        )
    }
    pub(super) fn ports(
        &mut self,
        tree: &Tree,
        scope: &str,
        owner: &str,
        role: &str,
        kind: &str,
    ) -> Result<Value> {
        let mut map = Map::new();
        for (name, reference) in self.names(tree)?.into_iter().zip(tree.all("reference")) {
            let key = if scope == owner {
                self.declared_symbol(owner, role, &name)?
            } else {
                self.symbol(owner, role, &name)?
            }
            .1;
            insert(
                &mut map,
                key,
                self.reference(reference, scope, kind)?.into(),
            )?;
        }
        Ok(map.into())
    }
    pub(super) fn scope(
        &mut self,
        tree: &'a Tree,
        parent: Option<&str>,
        depth: usize,
    ) -> Result<()> {
        if depth > MAX_DEPTH {
            return Err(error("depth", self.syntax.span(tree)));
        }
        let id = self.declare(tree, parent, "scope")?;
        if parent.is_none() && id != self.ledger.root() {
            return Err(error("root-binding", self.syntax.span(tree)));
        }
        if let Some(parent) = parent {
            let reference = tree.one("reference")?;
            let target = self.ledger.declarations.get(parent).expect("bound parent");
            let parts = self.parts(reference)?;
            let mut chain = vec![target.name.as_str()];
            let mut ancestor = target.scope.as_deref();
            while let Some(owner) = ancestor {
                let row = &self.ledger.declarations[owner];
                chain.push(row.name.as_str());
                ancestor = row.scope.as_deref();
            }
            if parts != [target.name.as_str()] && parts != chain {
                return Err(error("scope-within", self.syntax.span(reference)));
            }
        }
        let interface = tree.one("interface")?;
        let ports = interface.all("ports");
        let inputs = self.ports(ports[0], &id, &id, "input", "data")?;
        let outputs = self.ports(ports[1], &id, &id, "output", "data")?;
        let mut outcomes = Vec::new();
        for name in self.names(interface.one("names")?)? {
            outcomes.push(self.declared_symbol(&id, "outcome", &name)?.1);
        }
        if outcomes.is_empty() {
            return Err(error("scope-outcomes", self.syntax.span(interface)));
        }
        self.put("scopes", id.clone(), json!({"parent":parent, "entry":self.r(interface, &id, "node", 0)?, "inputs":inputs, "outputs":outputs, "outcomes":set(outcomes), "cancellation":self.r(interface, &id, "policy", 1)?, "faults":self.r(interface, &id, "policy", 2)?, "closure":self.r(interface, &id, "policy", 3)?, "race":self.r(interface, &id, "policy", 4)?}))?;
        for declaration in tree.all("declaration") {
            let node = declaration.first()?;
            match node.rule {
                "scope" => self.scope(node, Some(&id), depth + 1)?,
                "data" => self.data(node, &id)?,
                "expression_declaration" => {
                    let expr = self.declare(node, Some(&id), "expression")?;
                    let ty = self.wrapped_type(node.one("type")?, &id)?;
                    let ast = self.expression(node.one("expression")?, &id, false, 0)?;
                    let parameters = node.one("parameters")?;
                    let mut map = Map::new();
                    for (name, ty) in self
                        .names(parameters)?
                        .into_iter()
                        .zip(parameters.all("type"))
                    {
                        let ty = self.wrapped_type(ty, &id)?;
                        insert(&mut map, name, ty)?;
                    }
                    self.expression_record(expr, &id, ast, ty, map)?;
                }
                "actor" => {
                    let actor = self.declare(node, Some(&id), "actor")?;
                    self.put(
                        "actors",
                        actor,
                        json!({"scope":id, "requirement":self.r(node, &id, "policy", 0)?}),
                    )?;
                }
                "capability" => self.capability(node, &id)?,
                "policy" => self.policy(node.first()?, &id)?,
                "step" => self.pending_steps.push((node, id.clone())),
                _ => return Err(error("declaration-unsupported", self.syntax.span(node))),
            }
        }
        Ok(())
    }
    pub(super) fn data(&mut self, tree: &Tree, scope: &str) -> Result<()> {
        let id = self.declare(tree, Some(scope), "data")?;
        let ty = self.wrapped_type(tree.one("type")?, scope)?;
        let reference_lists = tree.all("references");
        let mut purposes = Vec::new();
        for name in self.names(tree.one("names")?)? {
            purposes.push(self.declared_symbol(&id, "purpose", &name)?.1);
        }
        if purposes.is_empty() {
            return Err(error("data-purposes", self.syntax.span(tree)));
        }
        let invalidates = unique_set(
            reference_lists[2]
                .all("reference")
                .into_iter()
                .map(|r| self.ledger.invalidation(scope, &self.parts(r)?))
                .collect::<Result<Vec<_>>>()?,
        )?;
        let mut record = json!({"scope":scope, "type":ty, "protection":{"policy":self.r(tree, scope, "policy", 0)?, "sensitivity":self.syntax.text(tree.all("NAME")[1])?, "purposes":set(purposes), "participants":self.references(reference_lists[0], scope, "actor")?, "capabilities":self.references(reference_lists[1], scope, "capability")?}, "invalidates":invalidates});
        if let Some(expr) = tree.all("expression").first() {
            let initial = self.slot(expr, &id, "initial", None)?;
            let ast = self.expression(expr, scope, false, 0)?;
            self.expression_record(initial.clone(), scope, ast, ty, Map::new())?;
            record["initial"] = initial.into();
        }
        self.put("data", id, record)
    }
    pub(super) fn capability(&mut self, tree: &Tree, scope: &str) -> Result<()> {
        let id = self.declare(tree, Some(scope), "capability")?;
        let provider = self.r(tree, scope, "resource", 0)?;
        let policy = self.slot(tree, &id, "provider-policy", None)?;
        self.put("policies", policy.clone(), json!({"scope":scope, "dialect":self.ledger.dialect(), "body":{"kind":"capability", "reads":{}, "provider":self.ledger.pin(&provider, "provider")?}}))?;
        let types = tree.all("type");
        let input = self.wrapped_type(types[0], scope)?;
        let output = self.wrapped_type(types[1], scope)?;
        self.put("capabilities", id, json!({"scope":scope, "contract":policy, "input":input, "output":output, "authority":self.r(tree, scope, "actor", 1)?, "effects":self.r(tree, scope, "policy", 2)?}))
    }
}
