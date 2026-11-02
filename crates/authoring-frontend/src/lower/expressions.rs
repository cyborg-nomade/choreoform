// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

use super::*;

impl<'a, 's> Lower<'a, 's> {
    pub(super) fn type_value(&mut self, tree: &Tree, scope: &str, depth: usize) -> Result<Value> {
        if depth > MAX_DEPTH {
            return Err(error("depth", self.syntax.span(tree)));
        }
        Ok(match self.syntax.word(tree, 0)? {
            "truth" => json!({"kind":"boolean"}),
            "text" => json!({"kind":"text"}),
            "whole" => json!({"kind":"integer"}),
            "decimal" => {
                json!({"kind":"decimal", "scale": scale(self.syntax.text(tree.one("NAT")?)?)?})
            }
            "record" => {
                let fields = tree.one("type_fields")?;
                let mut map = Map::new();
                for (name, ty) in self.names(fields)?.into_iter().zip(fields.all("type")) {
                    insert(&mut map, name, self.type_value(ty, scope, depth + 1)?)?;
                }
                json!({"kind":"record", "fields": map})
            }
            "choice" => {
                let mut cases = Map::new();
                for (tag, ty) in tree.all("tag").into_iter().zip(tree.all("type")) {
                    let name = self.tag(tag, scope)?;
                    let ty = self.type_value(ty, scope, depth + 1)?;
                    insert(&mut cases, name, ty)?;
                }
                json!({"kind":"variant", "cases": cases})
            }
            "list" | "optional" => {
                json!({"kind": if self.syntax.word(tree, 0)? == "list" {"list"} else {"option"}, "element": self.type_value(tree.one("type")?, scope, depth + 1)?})
            }
            "type" => {
                let resource = self.r(tree, scope, "resource", 0)?;
                json!({"kind":"named", "name": self.syntax.text(tree.one("NAME")?)?, "contract": self.ledger.pin(&resource, "named-type")?})
            }
            _ => return Err(error("type", self.syntax.span(tree))),
        })
    }
    pub(super) fn wrapped_type(&mut self, tree: &Tree, scope: &str) -> Result<Value> {
        Ok(json!({"dialect": self.ledger.dialect(), "body": self.type_value(tree, scope, 0)?}))
    }
    pub(super) fn tag(&mut self, tree: &Tree, scope: &str) -> Result<String> {
        if let Some(outcome) = tree.all("outcome_reference").first() {
            self.alias(outcome, scope)
        } else {
            Ok(self.syntax.text(tree.one("NAME")?)?.into())
        }
    }
    pub(super) fn alias(&mut self, tree: &Tree, scope: &str) -> Result<String> {
        let reference = tree.one("reference")?;
        let kind = if tree.rule == "purpose_reference" {
            "data"
        } else {
            "outcome-owner"
        };
        let owner = self.reference(reference, scope, kind)?;
        let name = self.syntax.text(tree.one("NAME")?)?.to_owned();
        Ok(self
            .symbol(
                &owner,
                if kind == "data" { "purpose" } else { "outcome" },
                &name,
            )?
            .1)
    }
    pub(super) fn raw_value(&mut self, tree: &Tree, scope: &str, depth: usize) -> Result<Value> {
        if depth > MAX_DEPTH {
            return Err(error("depth", self.syntax.span(tree)));
        }
        if let Some(value) = tree.children.first() {
            match value.rule {
                "TEXT" => return Ok(self.syntax.text(value)?.into()),
                "INTEGER" => return Ok(integer(self.syntax.text(value)?)?.into()),
                "outcome_reference" | "purpose_reference" => {
                    return Ok(self.alias(value, scope)?.into());
                }
                _ => {}
            }
        }
        Ok(match self.syntax.word(tree, 0)? {
            "true" => true.into(),
            "false" => false.into(),
            "decimal" => decimal(
                self.syntax.text(tree.one("NUMBER")?)?,
                scale(self.syntax.text(tree.one("NAT")?)?)?,
            )?
            .into(),
            "record" => {
                let mut fields = Map::new();
                for (name, value) in self.names(tree)?.into_iter().zip(tree.all("value")) {
                    let value = self.raw_value(value, scope, depth + 1)?;
                    insert(&mut fields, name, value)?;
                }
                Value::Object(fields)
            }
            "list" => Value::Array(
                tree.all("value")
                    .into_iter()
                    .map(|v| self.raw_value(v, scope, depth + 1))
                    .collect::<Result<_>>()?,
            ),
            "case" => {
                json!({"tag": self.tag(tree.one("tag")?, scope)?, "value": self.raw_value(tree.one("value")?, scope, depth + 1)?})
            }
            "absent" => json!({"tag":"none"}),
            "present" => {
                json!({"tag":"some", "value": self.raw_value(tree.one("value")?, scope, depth + 1)?})
            }
            _ => return Err(error("literal-value", self.syntax.span(tree))),
        })
    }
    pub(super) fn expression(
        &mut self,
        tree: &Tree,
        scope: &str,
        facts: bool,
        depth: usize,
    ) -> Result<Value> {
        if depth > MAX_DEPTH {
            return Err(error("depth", self.syntax.span(tree)));
        }
        let node = tree.first()?;
        let mut args = Vec::new();
        for expr in node.all("expression") {
            args.push(self.expression(expr, scope, facts, depth + 1)?);
        }
        let unary = |op: &str, args: &[Value]| -> Result<Value> {
            let value = args.first().ok_or_else(|| error("internal-tree", 0..0))?;
            Ok(json!({"op":op, "value":value}))
        };
        Ok(match node.rule {
            "literal" => {
                let (ty, value) = if let Some(ty) = node.all("type").first() {
                    let ty = self.type_value(ty, scope, 0)?;
                    let value = node.one("value")?;
                    self.literal_markers(value, &ty, scope, 0)?;
                    (ty, self.raw_value(value, scope, 0)?)
                } else if let Some(child) = node
                    .children
                    .first()
                    .filter(|n| matches!(n.rule, "outcome_reference" | "purpose_reference"))
                {
                    (json!({"kind":"text"}), self.alias(child, scope)?.into())
                } else if let Some(child) = node.all("TEXT").first() {
                    (json!({"kind":"text"}), self.syntax.text(child)?.into())
                } else if let Some(child) = node.all("INTEGER").first() {
                    (
                        json!({"kind":"integer"}),
                        integer(self.syntax.text(child)?)?.into(),
                    )
                } else if let Some(child) = node.all("NUMBER").first() {
                    let places = scale(self.syntax.text(node.one("NAT")?)?)?;
                    (
                        json!({"kind":"decimal", "scale":places}),
                        decimal(self.syntax.text(child)?, places)?.into(),
                    )
                } else {
                    (
                        json!({"kind":"boolean"}),
                        (self.syntax.word(node, 0)? == "true").into(),
                    )
                };
                json!({"op":"literal", "type":ty, "value":value})
            }
            "read" => json!({"op":"read", "cell":self.r(node, scope, "data", 0)?}),
            "parameter" => json!({"op":"param", "name": self.syntax.text(node.one("NAME")?)?}),
            "fact" => {
                if !facts {
                    return Err(error("policy-fact", self.syntax.span(node)));
                }
                json!({"op":"fact", "name":self.syntax.text(node.one("NAME")?)?})
            }
            "field" => {
                json!({"op":"field", "name":self.syntax.text(node.one("NAME")?)?, "value":args[0]})
            }
            "record" => {
                let mut fields = Map::new();
                for (name, expr) in self.names(node)?.into_iter().zip(args) {
                    insert(&mut fields, name, expr)?;
                }
                json!({"op":"record", "fields":fields})
            }
            "list" => {
                json!({"op":"list", "element":self.type_value(node.one("type")?, scope, 0)?, "items":args})
            }
            "variant" => {
                json!({"op":"variant", "type":self.type_value(node.one("type")?, scope, 0)?, "tag":self.tag(node.one("tag")?, scope)?, "value":args[0]})
            }
            "absent" => {
                json!({"op":"none", "element":self.type_value(node.one("type")?, scope, 0)?})
            }
            "some" => unary("some", &args)?,
            "exists" => unary("isSome", &args)?,
            "present" => unary("unwrap", &args)?,
            "case_name" => unary("tag", &args)?,
            "negation" => unary("not", &args)?,
            "length" => unary("length", &args)?,
            "representation" => unary("unwrapNamed", &args)?,
            "payload" => {
                json!({"op":"payload", "value":args[0], "tag":self.tag(node.one("tag")?, scope)?})
            }
            "conditional" => {
                json!({"op":"if", "condition":args[0], "then":args[1], "else":args[2]})
            }
            "binary" => {
                let op = match self.syntax.word(node.one("operator")?, 0)? {
                    "and" => "and",
                    "or" => "or",
                    "equals" => "eq",
                    "is" => "lt",
                    "plus" => "add",
                    "minus" => "sub",
                    "times" => "mul",
                    "contains" => "contains",
                    _ => return Err(error("operator", self.syntax.span(node))),
                };
                if op == "contains" {
                    json!({"op":op, "list":args[0], "value":args[1]})
                } else {
                    json!({"op":op, "left":args[0], "right":args[1]})
                }
            }
            "index" => json!({"op":"index", "value":args[0], "index":args[1]}),
            "rescale" => {
                json!({"op":"rescale", "value":args[0], "scale":scale(self.syntax.text(node.one("NAT")?)?)?})
            }
            "wrap" => {
                json!({"op":"wrap", "value":args[0], "type":self.type_value(node.one("type")?, scope, 0)?})
            }
            _ => return Err(error("expression-unsupported", self.syntax.span(node))),
        })
    }
    pub(super) fn expression_record(
        &mut self,
        id: String,
        scope: &str,
        ast: Value,
        ty: Value,
        parameters: Map<String, Value>,
    ) -> Result<()> {
        let reads = ast_reads(&ast);
        self.put("expressions", id, json!({"scope":scope, "dialect":self.ledger.dialect(), "resultType":ty, "parameters":parameters, "reads":set(reads), "body":ast}))
    }

    // Removing value markers must not silently reinterpret a decimal coefficient.
    // This checks that representational boundary, not general expression typing.
    fn literal_markers(
        &mut self,
        tree: &Tree,
        ty: &Value,
        scope: &str,
        depth: usize,
    ) -> Result<()> {
        if depth > MAX_DEPTH {
            return Err(error("depth", self.syntax.span(tree)));
        }
        match ty["kind"].as_str() {
            Some("decimal") => {
                let places = tree
                    .all("NAT")
                    .first()
                    .copied()
                    .ok_or_else(|| error("decimal-marker", self.syntax.span(tree)))?;
                if scale(self.syntax.text(places)?)? != ty["scale"] {
                    return Err(error("decimal-scale", self.syntax.span(tree)));
                }
            }
            Some("record") => {
                for (name, value) in self.names(tree)?.into_iter().zip(tree.all("value")) {
                    self.literal_markers(value, &ty["fields"][&name], scope, depth + 1)?;
                }
            }
            Some("list") => {
                for value in tree.all("value") {
                    self.literal_markers(value, &ty["element"], scope, depth + 1)?;
                }
            }
            Some("variant") => {
                if let Some(value) = tree.all("value").first() {
                    let tag = self.tag(tree.one("tag")?, scope)?;
                    self.literal_markers(value, &ty["cases"][&tag], scope, depth + 1)?;
                }
            }
            Some("option") => {
                if let Some(value) = tree.all("value").first() {
                    self.literal_markers(value, &ty["element"], scope, depth + 1)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

pub(super) fn ast_reads(ast: &Value) -> BTreeSet<String> {
    fn walk(value: &Value, reads: &mut BTreeSet<String>) {
        let Some(op) = value["op"].as_str() else {
            return;
        };
        if op == "literal" {
            return;
        }
        if op == "read" {
            if let Some(cell) = value["cell"].as_str() {
                reads.insert(cell.into());
            }
            return;
        }
        for field in [
            "value",
            "left",
            "right",
            "condition",
            "then",
            "else",
            "index",
            "list",
        ] {
            if value[field].is_object() {
                walk(&value[field], reads);
            }
        }
        if let Some(fields) = value["fields"].as_object() {
            for child in fields.values() {
                walk(child, reads);
            }
        }
        if let Some(items) = value["items"].as_array() {
            for child in items {
                walk(child, reads);
            }
        }
    }
    let mut result = BTreeSet::new();
    walk(ast, &mut result);
    result
}
