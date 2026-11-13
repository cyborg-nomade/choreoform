// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Source-only immutable envelopes; never cached access decisions.
use super::*;
use std::io::{self, Write};

pub(super) struct ResolvedProtection {
    value: Value,
    bytes: usize,
    work: usize,
}

struct Size(usize);
impl Write for Size {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0 = self
            .0
            .checked_add(bytes.len())
            .filter(|n| *n <= crate::MAX_BYTES)
            .ok_or_else(|| io::Error::other("size"))?;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
pub(super) fn encoded_size(value: &Value) -> Result<usize> {
    let mut size = Size(0);
    serde_json::to_writer(&mut size, value).map_err(|_| error("size", 0..0))?;
    Ok(size.0)
}
fn envelope_work(value: &Value) -> usize {
    // This closed envelope has only one object level of Boolean member sets.
    1 + value
        .as_object()
        .expect("envelope")
        .iter()
        .map(|(_, v)| 2 + v.as_object().map_or(0, |m| m.len() * 2))
        .sum::<usize>()
}

impl<'a, 's> Lower<'a, 's> {
    pub(super) fn expansion_charge(
        &mut self,
        work: usize,
        bytes: usize,
        span: std::ops::Range<usize>,
    ) -> Result<()> {
        creation::charge(&mut self.expansion_work, work).map_err(|mut e| {
            e.span = span.clone();
            e
        })?;
        self.expansion_bytes = self
            .expansion_bytes
            .checked_add(bytes)
            .filter(|n| *n <= crate::MAX_BYTES)
            .ok_or_else(|| error("size", span))?;
        Ok(())
    }
    pub(super) fn collect_protections(
        &mut self,
        tree: &'a Tree,
        parent: Option<&str>,
        depth: usize,
    ) -> Result<()> {
        if depth > MAX_DEPTH {
            return Err(error("depth", self.syntax.span(tree)));
        }
        self.expansion_charge(1, 0, self.syntax.span(tree))?;
        let name = self.syntax.text(tree.all("NAME")[0])?;
        let scope = self.ledger.declaration(parent, name, "scope")?.id.clone();
        for declaration in tree.all("declaration") {
            self.expansion_charge(1, 0, self.syntax.span(declaration))?;
            let node = declaration.first()?;
            match node.rule {
                "scope" => self.collect_protections(node, Some(&scope), depth + 1)?,
                "data_protection" => {
                    let id = self.declare(node, Some(&scope), "data-protection")?;
                    let lists = node.all("references");
                    let count = lists
                        .iter()
                        .map(|t| t.all("reference").len())
                        .sum::<usize>()
                        + node.one("names")?.all("NAME").len();
                    self.expansion_charge(count + 1, 0, self.syntax.span(node))?;
                    let purposes =
                        self.protection_purposes(node.one("names")?, &id)
                            .map_err(|mut e| {
                                if e.span == (0..0) {
                                    e.span =
                                        self.syntax.span(node.one("names").expect("grammar names"));
                                }
                                e
                            })?;
                    let value = json!({"policy":self.r(node, &scope, "policy", 0)?, "sensitivity":self.syntax.text(node.all("NAME")[1])?, "purposes":purposes, "participants":self.references(lists[0], &scope, "actor").map_err(|mut e|{if e.span == (0..0){e.span=self.syntax.span(lists[0]);}e})?, "capabilities":self.references(lists[1], &scope, "capability").map_err(|mut e|{if e.span == (0..0){e.span=self.syntax.span(lists[1]);}e})?});
                    let bytes = encoded_size(&value).map_err(|mut e| {
                        e.span = self.syntax.span(node);
                        e
                    })?;
                    let work = envelope_work(&value);
                    self.expansion_charge(work, bytes, self.syntax.span(node))?;
                    self.protections
                        .insert(id, ResolvedProtection { value, bytes, work });
                }
                _ => {}
            }
        }
        Ok(())
    }
    pub(super) fn protection_purposes(&mut self, names: &Tree, owner: &str) -> Result<Value> {
        let mut purposes = Vec::new();
        for name in self.names(names)? {
            purposes.push(self.declared_symbol(owner, "purpose", &name)?.1);
        }
        if purposes.is_empty() {
            return Err(error("data-purposes", self.syntax.span(names)));
        }
        unique_set(purposes)
    }
    pub(super) fn named_protection(
        &mut self,
        tree: &Tree,
        scope: &str,
        data: &str,
    ) -> Result<Value> {
        let reference = tree.one("reference")?;
        let id = self.reference(reference, scope, "data-protection")?;
        let span = self.syntax.span(reference);
        let resolved = self
            .protections
            .get(&id)
            .ok_or_else(|| error("protection-definition", span.clone()))?;
        let (bytes, work) = (resolved.bytes, resolved.work);
        self.expansion_charge(work, bytes, span.clone())?;
        // Keep local purpose identities and captions, including purpose literals.
        let count = self.ledger.purpose_symbols(data).count();
        self.expansion_charge(count, 0, span.clone())?;
        let purposes: Vec<_> = self
            .ledger
            .purpose_symbols(data)
            .map(|s| (s.id.clone(), s.wire.clone()))
            .collect();
        let actual = set(purposes.iter().map(|(_, wire)| wire.clone()));
        let resolved = &self.protections[&id].value;
        if actual != resolved["purposes"] {
            return Err(error("protection-purpose-bindings", span));
        }
        for (field, kind) in [("participants", "actor"), ("capabilities", "capability")] {
            for target in resolved[field].as_object().expect("set").keys() {
                let row = &self.ledger.declarations[target];
                if self
                    .ledger
                    .resolve(scope, &[&row.name], kind)
                    .map(|r| &r.id)
                    != Ok(target)
                {
                    return Err(error("protection-member-visibility", span));
                }
            }
        }
        let policy = &self.ledger.declarations[resolved["policy"].as_str().expect("policy")];
        if self
            .ledger
            .resolve(scope, &[&policy.name], "policy")
            .map(|r| &r.id)
            != Ok(&policy.id)
        {
            return Err(error("protection-member-visibility", span));
        }
        for (id, _) in purposes {
            self.symbols.insert(id);
        }
        // Cumulative work/bytes were charged before this only expansion copy.
        Ok(self.protections[&id].value.clone())
    }
    pub(super) fn validate_protections(&self) -> Result<()> {
        for (id, resolved) in &self.protections {
            let policy = resolved.value["policy"].as_str().expect("policy");
            if self.body["policies"][policy]["body"]["kind"] != "protection" {
                return Err(error("protection-policy-kind", self.spans[id].clone()));
            }
        }
        Ok(())
    }
}
