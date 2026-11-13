// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Preflight source type DAGs before allocating their expanded cached values.
use super::*;

pub(super) struct TypePlan<'a> {
    tree: &'a Tree,
    scope: String,
}
#[derive(Clone)]
pub(super) struct Cost {
    pub work: usize,
    pub bytes: usize,
    depth: usize,
    containers: usize,
    pub(super) refs: BTreeSet<(String, String)>,
}
fn sum(a: usize, b: usize, limit: usize) -> Result<usize> {
    a.checked_add(b)
        .filter(|n| *n <= limit)
        .ok_or_else(|| error("type-expansion", 0..0))
}
fn quote_size(name: &str) -> Result<usize> {
    protection::encoded_size(&Value::String(name.into()))
}
fn value_work(value: &Value) -> usize {
    match value {
        Value::Object(m) => 1 + m.values().map(|v| 1 + value_work(v)).sum::<usize>(),
        _ => 1,
    }
}
impl Cost {
    fn leaf(value: &Value) -> Result<Self> {
        Ok(Self {
            work: value_work(value),
            bytes: protection::encoded_size(value)?,
            depth: 1,
            containers: if value.get("contract").is_some() {
                2
            } else {
                1
            },
            refs: BTreeSet::new(),
        })
    }
    fn checked(self, span: std::ops::Range<usize>) -> Result<Self> {
        if self.work > crate::MAX_WORK || self.bytes > crate::MAX_BYTES {
            return Err(error("type-expansion", span));
        }
        if self.depth > MAX_DEPTH || self.containers > MAX_DEPTH {
            return Err(error("depth", span));
        }
        Ok(self)
    }
}
impl<'a, 's> Lower<'a, 's> {
    pub(super) fn prepare_types(&mut self, root: &'a Tree) -> Result<()> {
        self.collect_types(root, None, 0)?;
        let ids: Vec<_> = self.type_plans.keys().cloned().collect();
        for id in ids {
            self.resolve_type_cost(&id, 0)?;
        }
        // All graph/cost validation and cumulative cache reservation precede values.
        for id in std::mem::take(&mut self.type_order) {
            let (tree, scope) = {
                let p = &self.type_plans[&id];
                (p.tree, p.scope.clone())
            };
            let value = self.type_value_inner(tree, &scope, 0)?;
            debug_assert_eq!(
                protection::encoded_size(&value)?,
                self.type_costs[&id].bytes
            );
            self.type_values.insert(id, value);
        }
        Ok(())
    }
    fn collect_types(&mut self, tree: &'a Tree, parent: Option<&str>, depth: usize) -> Result<()> {
        if depth > MAX_DEPTH {
            return Err(error("depth", self.syntax.span(tree)));
        }
        self.expansion_charge(1, 0, self.syntax.span(tree))?;
        let name = self.syntax.text(tree.all("NAME")[0])?;
        let scope = self.ledger.declaration(parent, name, "scope")?.id.clone();
        for d in tree.all("declaration") {
            self.expansion_charge(1, 0, self.syntax.span(d))?;
            let node = d.first()?;
            match node.rule {
                "scope" => self.collect_types(node, Some(&scope), depth + 1)?,
                "type_declaration" => {
                    let id = self.declare(node, Some(&scope), "type-alias")?;
                    self.type_plans.insert(
                        id,
                        TypePlan {
                            tree: node.one("type")?,
                            scope: scope.clone(),
                        },
                    );
                }
                _ => {}
            }
        }
        Ok(())
    }
    fn resolve_type_cost(&mut self, id: &str, walk_depth: usize) -> Result<Cost> {
        let span = self.spans.get(id).cloned().unwrap_or(0..0);
        if walk_depth > MAX_DEPTH {
            return Err(error("depth", span));
        }
        if let Some(c) = self.type_costs.get(id) {
            // Source19 copies only this compact cost/provenance summary here.
            // Expanded descriptor allocations are reserved separately by
            // type_value (and by cache construction), before every actual copy.
            // Keep source18's published conservative accounting unchanged.
            let charge = if self.syntax.profile() == "0.19.0" {
                1 + c.refs.len()
            } else {
                c.work + c.refs.len()
            };
            self.expansion_charge(charge, 0, span)?;
            return Ok(self.type_costs[id].clone());
        }
        if self.type_visiting.contains(id) {
            return Err(error("type-alias-cycle", span));
        }
        if self.type_visiting.len() >= MAX_DEPTH {
            return Err(error("depth", span));
        }
        let (tree, scope) = self
            .type_plans
            .get(id)
            .map(|p| (p.tree, p.scope.clone()))
            .ok_or_else(|| error("type-definition", span.clone()))?;
        self.type_visiting.insert(id.into());
        let cost = self.type_cost(tree, &scope, walk_depth)?;
        self.type_visiting.remove(id);
        self.expansion_charge(cost.work + cost.refs.len(), cost.bytes, span)?;
        self.type_order.push(id.into());
        self.type_costs.insert(id.into(), cost.clone());
        Ok(cost)
    }
    fn type_visibility(
        &mut self,
        cost: &Cost,
        scope: &str,
        span: std::ops::Range<usize>,
    ) -> Result<()> {
        self.expansion_charge(cost.refs.len(), 0, span.clone())?;
        for (id, kind) in &cost.refs {
            let row = &self.ledger.declarations[id];
            if self
                .ledger
                .resolve(scope, &[&row.name], kind)
                .map(|r| &r.id)
                != Ok(id)
            {
                return Err(error("type-member-visibility", span));
            }
        }
        Ok(())
    }
    pub(super) fn type_cost(&mut self, tree: &Tree, scope: &str, depth: usize) -> Result<Cost> {
        let span = self.syntax.span(tree);
        if depth > MAX_DEPTH {
            return Err(error("depth", span));
        }
        self.expansion_charge(1, 0, span.clone())?;
        let result = (|| -> Result<Cost> {
            Ok(match self.syntax.word(tree, 0)? {
                "truth" => Cost::leaf(&json!({"kind":"boolean"}))?,
                "text" => Cost::leaf(&json!({"kind":"text"}))?,
                "whole" => Cost::leaf(&json!({"kind":"integer"}))?,
                "decimal" => Cost::leaf(
                    &json!({"kind":"decimal","scale":scale(self.syntax.text(tree.one("NAT")?)?)?}),
                )?,
                "type" if !tree.all("local_type").is_empty() => {
                    let r = tree.one("local_type")?.one("reference")?;
                    let id = self.reference(r, scope, "type-alias")?;
                    let mut c = self.resolve_type_cost(&id, depth + 1)?;
                    self.type_visibility(&c, scope, self.syntax.span(r))?;
                    c.refs.insert((id, "type-alias".into()));
                    c
                }
                "type" => {
                    let resource = self.r(tree, scope, "resource", 0)?;
                    let value = json!({"kind":"named","name":self.syntax.text(tree.one("NAME")?)?,"contract":self.ledger.pin(&resource,"named-type")?});
                    self.resources
                        .type_descriptor(&value)
                        .map_err(|e| error(e.code, span.clone()))?;
                    let mut c = Cost::leaf(&value)?;
                    c.refs.insert((resource, "resource".into()));
                    c
                }
                "list" | "optional" => {
                    let child = self.type_cost(tree.one("type")?, scope, depth + 1)?;
                    let kind = if self.syntax.word(tree, 0)? == "list" {
                        "list"
                    } else {
                        "option"
                    };
                    let base = protection::encoded_size(&json!({"kind":kind,"element":null}))? - 4;
                    Cost {
                        bytes: sum(base, child.bytes, crate::MAX_BYTES)?,
                        work: sum(4, child.work, crate::MAX_WORK)?,
                        depth: child.depth + 1,
                        containers: child.containers + 1,
                        refs: child.refs,
                    }
                }
                "record" | "choice" => {
                    let record = self.syntax.word(tree, 0)? == "record";
                    let owner = if record {
                        tree.one("type_fields")?
                    } else {
                        tree
                    };
                    let mut refs = BTreeSet::new();
                    let mut names = BTreeSet::new();
                    let mut bytes = protection::encoded_size(&if record {
                        json!({"kind":"record","fields":{}})
                    } else {
                        json!({"kind":"variant","cases":{}})
                    })?;
                    let mut work = 5;
                    let mut type_depth = 1;
                    let mut containers = 2;
                    let labels = owner.all(if record { "NAME" } else { "tag" });
                    if labels.len() > 10_000 {
                        return Err(error("type-members", self.syntax.span(owner)));
                    }
                    for (i, (label, ty)) in labels.into_iter().zip(owner.all("type")).enumerate() {
                        let name = if record {
                            self.syntax.text(label)?.to_owned()
                        } else {
                            let tag = label;
                            if let Some(outcome) = tag.all("outcome_reference").first() {
                                let id = self.reference(
                                    outcome.one("reference")?,
                                    scope,
                                    "outcome-owner",
                                )?;
                                refs.insert((id, "outcome-owner".into()));
                            }
                            self.tag(tag, scope)?
                        };
                        if name.chars().count() > 4096 {
                            return Err(error("type-name", self.syntax.span(label)));
                        }
                        if !names.insert(name.clone()) {
                            return Err(error("duplicate-map-key", self.syntax.span(owner)));
                        }
                        let child = self.type_cost(ty, scope, depth + 1)?;
                        bytes = sum(
                            bytes,
                            quote_size(&name)? + 1 + usize::from(i > 0),
                            crate::MAX_BYTES,
                        )?;
                        bytes = sum(bytes, child.bytes, crate::MAX_BYTES)?;
                        work = sum(work, 1 + child.work, crate::MAX_WORK)?;
                        type_depth = type_depth.max(child.depth + 1);
                        containers = containers.max(child.containers + 2);
                        refs.extend(child.refs);
                    }
                    Cost {
                        bytes,
                        work,
                        depth: type_depth,
                        containers,
                        refs,
                    }
                }
                _ => return Err(error("type", span.clone())),
            })
        })();
        result
            .and_then(|c| c.checked(span.clone()))
            .map_err(|mut e| {
                if e.span == (0..0) {
                    e.span = span;
                }
                e
            })
    }
}
