// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use serde_json::json;

#[cfg(test)]
mod conformance;
#[cfg(test)]
mod rp01_snapshots;

/// Explicit static bindings; empty facts means ordinary expressions/invariants.
#[derive(Default)]
pub struct Context {
    pub cells: BTreeMap<String, Type>,
    pub parameters: BTreeMap<String, Type>,
    pub facts: BTreeMap<String, Type>,
}
#[derive(Debug)]
pub struct Judgment {
    pub ty: Type,
    pub reads: BTreeSet<String>,
}

impl Resources<'_> {
    /// Check the complete AST and all branches, never discovering types by execution.
    pub fn expression(&mut self, ast: &Value, context: &Context) -> Result<Judgment> {
        choreoform_ir_probe_core::transport::admit_value(ast).map_err(|_| err("ast-budget"))?;
        for ty in context
            .cells
            .values()
            .chain(context.parameters.values())
            .chain(context.facts.values())
        {
            self.valid_type(ty, 0)?;
        }
        let mut reads = BTreeSet::new();
        let ty = self.infer(ast, context, &mut reads, 0, &mut 0)?;
        Ok(Judgment { ty, reads })
    }
    fn infer(
        &mut self,
        v: &Value,
        c: &Context,
        reads: &mut BTreeSet<String>,
        depth: usize,
        nodes: &mut usize,
    ) -> Result<Type> {
        self.budget.tick(depth)?;
        *nodes += 1;
        require(*nodes <= MAX_ITEMS, "ast-budget")?;
        let op = text(&v["op"])?;
        let extra: &[&str] = match op {
            "literal" => &["type", "value"],
            "read" => &["cell"],
            "param" | "fact" => &["name"],
            "field" => &["name", "value"],
            "record" => &["fields"],
            "list" => &["element", "items"],
            "variant" => &["type", "tag", "value"],
            "none" => &["element"],
            "some" | "isSome" | "unwrap" | "tag" | "not" | "length" | "unwrapNamed" => &["value"],
            "payload" => &["value", "tag"],
            "if" => &["condition", "then", "else"],
            "and" | "or" | "eq" | "lt" | "add" | "sub" | "mul" => &["left", "right"],
            "contains" => &["list", "value"],
            "index" => &["value", "index"],
            "rescale" => &["value", "scale"],
            "wrap" => &["type", "value"],
            _ => return Err(err("expression-op")),
        };
        let mut keys = vec!["op"];
        keys.extend(extra);
        fields(v, &keys)?;
        let mut child =
            |this: &mut Self, key: &str| this.infer(&v[key], c, reads, depth + 1, nodes);
        Ok(match op {
            "literal" => {
                let t = self.type_descriptor(&v["type"])?;
                self.value(&t, &v["value"], 0)?;
                t
            }
            "read" => {
                let cell = name(&v["cell"])?;
                reads.insert(cell.into());
                c.cells
                    .get(cell)
                    .cloned()
                    .ok_or_else(|| err("cell-reference"))?
            }
            "param" => c
                .parameters
                .get(name(&v["name"])?)
                .cloned()
                .ok_or_else(|| err("parameter"))?,
            "fact" => c
                .facts
                .get(name(&v["name"])?)
                .cloned()
                .ok_or_else(|| err("policy-fact"))?,
            "field" => {
                let Type::Record(m) = child(self, "value")? else {
                    return Err(err("expression-type"));
                };
                m.get(name(&v["name"])?)
                    .cloned()
                    .ok_or_else(|| err("field-reference"))?
            }
            "record" => {
                let m = object(&v["fields"])?;
                require(m.len() <= MAX_ITEMS, "collection")?;
                let mut result = BTreeMap::new();
                for (k, value) in m {
                    name(&Value::from(k.as_str()))?;
                    result.insert(k.clone(), self.infer(value, c, reads, depth + 1, nodes)?);
                }
                Type::Record(result)
            }
            "list" => {
                let element = self.type_descriptor(&v["element"])?;
                for value in items(&v["items"])? {
                    require(
                        self.infer(value, c, reads, depth + 1, nodes)? == element,
                        "expression-type",
                    )?;
                }
                Type::List(Box::new(element))
            }
            "variant" => {
                let t = self.type_descriptor(&v["type"])?;
                let Type::Variant(m) = &t else {
                    return Err(err("expression-type"));
                };
                require(
                    m.get(name(&v["tag"])?) == Some(&child(self, "value")?),
                    "variant-type",
                )?;
                t
            }
            "none" => Type::Option(Box::new(self.type_descriptor(&v["element"])?)),
            "some" => Type::Option(Box::new(child(self, "value")?)),
            "isSome" | "unwrap" => {
                let Type::Option(t) = child(self, "value")? else {
                    return Err(err("expression-type"));
                };
                if op == "isSome" { Type::Boolean } else { *t }
            }
            "tag" | "payload" => {
                let Type::Variant(m) = child(self, "value")? else {
                    return Err(err("expression-type"));
                };
                if op == "tag" {
                    Type::Text
                } else {
                    m.get(name(&v["tag"])?)
                        .cloned()
                        .ok_or_else(|| err("variant-tag"))?
                }
            }
            "if" => {
                require(
                    child(self, "condition")? == Type::Boolean,
                    "expression-type",
                )?;
                let yes = child(self, "then")?;
                let no = child(self, "else")?;
                require(yes == no, "expression-type")?;
                yes
            }
            "not" => {
                require(child(self, "value")? == Type::Boolean, "expression-type")?;
                Type::Boolean
            }
            "and" | "or" | "eq" | "lt" | "add" | "sub" | "mul" => {
                let left = child(self, "left")?;
                let right = child(self, "right")?;
                require(left == right, "expression-type")?;
                match op {
                    "and" | "or" => require(left == Type::Boolean, "expression-type")?,
                    "lt" | "add" | "sub" => require(left.numeric(), "expression-type")?,
                    "mul" => require(left == Type::Integer, "expression-type")?,
                    _ => {}
                }
                if matches!(op, "add" | "sub" | "mul") {
                    left
                } else {
                    Type::Boolean
                }
            }
            "length" | "contains" | "index" => {
                let t = child(self, if op == "contains" { "list" } else { "value" })?;
                let Type::List(element) = t else {
                    return Err(err("expression-type"));
                };
                match op {
                    "length" => Type::Integer,
                    "contains" => {
                        require(child(self, "value")? == *element, "expression-type")?;
                        Type::Boolean
                    }
                    _ => {
                        require(child(self, "index")? == Type::Integer, "expression-type")?;
                        *element
                    }
                }
            }
            "rescale" => {
                require(
                    matches!(child(self, "value")?, Type::Decimal(_)),
                    "expression-type",
                )?;
                Type::Decimal(scale(&v["scale"])?)
            }
            "wrap" => {
                let t = self.type_descriptor(&v["type"])?;
                let Type::Named(key) = &t else {
                    return Err(err("expression-type"));
                };
                let d = self.nominal(key, depth + 1)?;
                require(child(self, "value")? == d.representation, "expression-type")?;
                t
            }
            "unwrapNamed" => {
                let Type::Named(key) = child(self, "value")? else {
                    return Err(err("expression-type"));
                };
                self.nominal(&key, depth + 1)?.representation.clone()
            }
            _ => unreachable!("closed operation"),
        })
    }

    // Only called for statically checked nominal invariants. There is no general
    // process evaluator or ambient facts/read interface in the public API.
    pub(crate) fn evaluate(
        &mut self,
        v: &Value,
        params: &BTreeMap<String, Value>,
        context: &Context,
        depth: usize,
    ) -> Result<Value> {
        self.budget.tick(depth)?;
        let op = text(&v["op"])?;
        let child = |this: &mut Self, key: &str| this.evaluate(&v[key], params, context, depth + 1);
        let value = match op {
            "literal" => v["value"].clone(),
            "param" => params
                .get(text(&v["name"])?)
                .cloned()
                .ok_or_else(|| err("parameter"))?,
            "if" => {
                let c = child(self, "condition")?
                    .as_bool()
                    .ok_or_else(|| err("evaluation-type"))?;
                child(self, if c { "then" } else { "else" })?
            }
            "and" | "or" => {
                let a = child(self, "left")?
                    .as_bool()
                    .ok_or_else(|| err("evaluation-type"))?;
                if (op == "and" && !a) || (op == "or" && a) {
                    a.into()
                } else {
                    child(self, "right")?
                }
            }
            "not" => (!child(self, "value")?
                .as_bool()
                .ok_or_else(|| err("evaluation-type"))?)
            .into(),
            "field" => child(self, "value")?
                .get(text(&v["name"])?)
                .cloned()
                .ok_or_else(|| err("evaluation-field"))?,
            "record" => {
                let mut map = Map::new();
                for (k, e) in object(&v["fields"])? {
                    map.insert(k.clone(), self.evaluate(e, params, context, depth + 1)?);
                }
                Value::Object(map)
            }
            "list" => Value::Array(
                items(&v["items"])?
                    .iter()
                    .map(|e| self.evaluate(e, params, context, depth + 1))
                    .collect::<Result<_>>()?,
            ),
            "variant" | "some" => {
                json!({"tag":if op=="some" {Value::from("some")} else {v["tag"].clone()},"value":child(self,"value")?})
            }
            "none" => json!({"tag":"none"}),
            "isSome" => (child(self, "value")?["tag"] == "some").into(),
            "unwrap" | "payload" => {
                let a = child(self, "value")?;
                require(
                    a["tag"]
                        == if op == "unwrap" {
                            Value::from("some")
                        } else {
                            v["tag"].clone()
                        },
                    "evaluation-tag",
                )?;
                a.get("value")
                    .cloned()
                    .ok_or_else(|| err("evaluation-tag"))?
            }
            "tag" => child(self, "value")?["tag"].clone(),
            "eq" => (child(self, "left")? == child(self, "right")?).into(),
            "lt" | "add" | "sub" | "mul" => {
                let a = integer(&child(self, "left")?)?;
                let b = integer(&child(self, "right")?)?;
                if op == "lt" {
                    (a < b).into()
                } else {
                    let n = match op {
                        "add" => a.checked_add(b),
                        "sub" => a.checked_sub(b),
                        _ => a.checked_mul(b),
                    }
                    .ok_or_else(|| err("overflow"))?;
                    n.to_string().into()
                }
            }
            "length" => items(&child(self, "value")?)?.len().to_string().into(),
            "contains" => {
                let list = child(self, "list")?;
                let value = child(self, "value")?;
                items(&list)?.contains(&value).into()
            }
            "index" => {
                let list = child(self, "value")?;
                let i = integer(&child(self, "index")?)?;
                usize::try_from(i)
                    .ok()
                    .and_then(|i| list.as_array()?.get(i))
                    .cloned()
                    .ok_or_else(|| err("index-bounds"))?
            }
            "rescale" => {
                let Type::Decimal(from) = self.expression(&v["value"], context)?.ty else {
                    return Err(err("evaluation-type"));
                };
                let to = scale(&v["scale"])?;
                let value = integer(&child(self, "value")?)?;
                let factor = 10i64.pow(u32::from(from.abs_diff(to)));
                let coefficient = if to >= from {
                    value.checked_mul(factor).ok_or_else(|| err("overflow"))?
                } else {
                    require(value % factor == 0, "inexact-rescale")?;
                    value / factor
                };
                coefficient.to_string().into()
            }
            "wrap" => {
                let t = self.type_descriptor(&v["type"])?;
                let value = child(self, "value")?;
                self.value(&t, &value, depth + 1)?;
                value
            }
            "unwrapNamed" => child(self, "value")?,
            _ => return Err(err("invariant-operation")),
        };
        choreoform_ir_probe_core::transport::admit_value(&value)
            .map_err(|_| err("evaluation-budget"))?;
        let cost = serde_json::to_vec(&value)
            .map_err(|_| err("evaluation-budget"))?
            .len();
        self.budget.work = self
            .budget
            .work
            .checked_add(cost)
            .ok_or_else(|| err("budget"))?;
        self.budget.tick(depth)?;
        Ok(value)
    }
}
