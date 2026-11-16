// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

pub(super) struct Bindings {
    pub context: Context,
    pub values: BTreeMap<String, Value>,
    used: BTreeSet<String>,
}
impl Bindings {
    pub fn new(
        r: &mut Resources<'_>,
        context: &Context,
        values: &Value,
        case: &Value,
    ) -> Result<Self> {
        let mut result = Self {
            context: Context::default(),
            values: BTreeMap::new(),
            used: BTreeSet::new(),
        };
        for (set, types, prefix) in [
            ("cells", &context.cells, "cell"),
            ("facts", &context.facts, "fact"),
        ] {
            for (id, key) in object(&case[set])? {
                r.budget.tick(0)?;
                let ty = types.get(id).ok_or_else(|| err("snapshot-binding"))?;
                let value = values
                    .get(name(key)?)
                    .ok_or_else(|| err("snapshot-value"))?;
                r.literal(ty, value)?;
                let parameter = format!("{prefix}:{id}");
                r.budget.charge(
                    parameter.len()
                        + serde_json::to_vec(value)
                            .map_err(|_| err("snapshot-budget"))?
                            .len(),
                )?;
                result
                    .context
                    .parameters
                    .insert(parameter.clone(), ty.clone());
                result.values.insert(parameter, value.clone());
            }
        }
        Ok(result)
    }
    pub fn complete(&self) -> Result<()> {
        require(
            self.values.keys().cloned().collect::<BTreeSet<_>>() == self.used,
            "snapshot-unused-binding",
        )
    }
    pub fn rebind(&mut self, ast: &Value, r: &mut Resources<'_>, depth: usize) -> Result<Value> {
        r.budget.tick(depth)?;
        let op = text(&ast["op"])?;
        if matches!(op, "read" | "fact") {
            let parameter = format!(
                "{}:{}",
                if op == "read" { "cell" } else { "fact" },
                text(&ast[if op == "read" { "cell" } else { "name" }])?
            );
            require(self.values.contains_key(&parameter), "snapshot-binding")?;
            self.used.insert(parameter.clone());
            return Ok(json!({"op":"param","name":parameter}));
        }
        // Literal values and type descriptors are opaque data, even when they
        // happen to contain an object whose op is read/fact.
        let mut result = ast.clone();
        match op {
            "literal" | "none" => {}
            "record" => {
                for (key, child) in object(&ast["fields"])? {
                    result["fields"][key] = self.rebind(child, r, depth + 1)?;
                }
            }
            "list" => {
                for (i, child) in items(&ast["items"])?.iter().enumerate() {
                    result["items"][i] = self.rebind(child, r, depth + 1)?;
                }
            }
            "if" => {
                for field in ["condition", "then", "else"] {
                    result[field] = self.rebind(&ast[field], r, depth + 1)?;
                }
            }
            "and" | "or" | "eq" | "lt" | "add" | "sub" | "mul" => {
                for field in ["left", "right"] {
                    result[field] = self.rebind(&ast[field], r, depth + 1)?;
                }
            }
            "contains" => {
                for field in ["list", "value"] {
                    result[field] = self.rebind(&ast[field], r, depth + 1)?;
                }
            }
            "index" => {
                for field in ["value", "index"] {
                    result[field] = self.rebind(&ast[field], r, depth + 1)?;
                }
            }
            "field" | "variant" | "some" | "isSome" | "unwrap" | "tag" | "not" | "length"
            | "wrap" | "unwrapNamed" | "payload" | "rescale" => {
                result["value"] = self.rebind(&ast["value"], r, depth + 1)?
            }
            _ => return Err(err("snapshot-operation")),
        }
        // Account for the copied AST, including opaque literals/descriptors.
        r.budget.charge(
            serde_json::to_vec(&result)
                .map_err(|_| err("snapshot-budget"))?
                .len(),
        )?;
        Ok(result)
    }
}
