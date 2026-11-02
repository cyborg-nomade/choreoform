// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

/// An exact-byte artifact identity, not evidence of host enforcement.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pin {
    pub id: String,
    pub revision: String,
}
impl Pin {
    /// Decode the closed immutable-reference shape without fetching anything.
    pub fn decode(v: &Value) -> Result<Self> {
        fields(v, &["id", "revision"])?;
        let revision = text(&v["revision"])?;
        require(
            revision.len() == 71
                && revision.starts_with("sha256:")
                && revision[7..]
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
            "artifact-pin",
        )?;
        Ok(Self {
            id: name(&v["id"])?.into(),
            revision: revision.into(),
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Nominal {
    pub contract: Pin,
    pub name: String,
}

/// Exact suite types. Named equality uses the pin/name, never representation shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Boolean,
    Text,
    Integer,
    Decimal(u8),
    Record(BTreeMap<String, Type>),
    Variant(BTreeMap<String, Type>),
    List(Box<Type>),
    Option(Box<Type>),
    Named(Nominal),
}
impl Type {
    pub(crate) fn empty() -> Self {
        Self::Record(BTreeMap::new())
    }
    pub(crate) fn numeric(&self) -> bool {
        matches!(self, Self::Integer | Self::Decimal(_))
    }
}

impl Resources<'_> {
    /// Resolve a closed type descriptor with finite exact nominal dependencies.
    pub fn type_descriptor(&mut self, v: &Value) -> Result<Type> {
        choreoform_ir_probe_core::transport::admit_value(v).map_err(|_| err("type-budget"))?;
        let ty = self.ty(v, 0)?;
        self.check_named_resources()?;
        Ok(ty)
    }
    pub(crate) fn ty(&mut self, v: &Value, depth: usize) -> Result<Type> {
        self.budget.tick(depth)?;
        let kind = text(&v["kind"])?;
        let keys: &[&str] = match kind {
            "boolean" | "text" | "integer" => &["kind"],
            "decimal" => &["kind", "scale"],
            "record" => &["kind", "fields"],
            "variant" => &["kind", "cases"],
            "list" | "option" => &["kind", "element"],
            "named" => &["kind", "contract", "name"],
            _ => return Err(err("type-kind")),
        };
        fields(v, keys)?;
        Ok(match kind {
            "boolean" => Type::Boolean,
            "text" => Type::Text,
            "integer" => Type::Integer,
            "decimal" => Type::Decimal(scale(&v["scale"])?),
            "record" | "variant" => {
                let m = object(&v[if kind == "record" { "fields" } else { "cases" }])?;
                require(
                    m.len() <= MAX_ITEMS && (kind != "variant" || !m.is_empty()),
                    "type-members",
                )?;
                let map = m
                    .iter()
                    .map(|(k, t)| {
                        name(&Value::from(k.as_str()))?;
                        Ok((k.clone(), self.ty(t, depth + 1)?))
                    })
                    .collect::<Result<_>>()?;
                if kind == "record" {
                    Type::Record(map)
                } else {
                    Type::Variant(map)
                }
            }
            "list" => Type::List(Box::new(self.ty(&v["element"], depth + 1)?)),
            "option" => Type::Option(Box::new(self.ty(&v["element"], depth + 1)?)),
            "named" => {
                let key = Nominal {
                    contract: Pin::decode(&v["contract"])?,
                    name: name(&v["name"])?.into(),
                };
                self.nominal(&key, depth + 1)?;
                Type::Named(key)
            }
            _ => unreachable!("closed type kind"),
        })
    }

    /// Admit a literal value, including all nominal invariants. No host reads.
    pub fn literal(&mut self, ty: &Type, v: &Value) -> Result<()> {
        self.valid_type(ty, 0)?;
        choreoform_ir_probe_core::transport::admit_value(v).map_err(|_| err("value-budget"))?;
        self.value(ty, v, 0)?;
        self.check_named_resources()
    }
    pub(crate) fn valid_type(&mut self, ty: &Type, depth: usize) -> Result<()> {
        self.budget.tick(depth)?;
        match ty {
            Type::Decimal(s) => require(*s <= 18, "decimal-scale")?,
            Type::Record(m) | Type::Variant(m) => {
                require(
                    m.len() <= MAX_ITEMS && (!matches!(ty, Type::Variant(_)) || !m.is_empty()),
                    "type-members",
                )?;
                for (k, t) in m {
                    name(&Value::from(k.as_str()))?;
                    self.valid_type(t, depth + 1)?;
                }
            }
            Type::List(t) | Type::Option(t) => self.valid_type(t, depth + 1)?,
            Type::Named(key) => {
                name(&Value::from(key.name.as_str()))?;
                self.nominal(key, depth + 1)?;
            }
            _ => {}
        }
        Ok(())
    }
    pub(crate) fn value(&mut self, ty: &Type, v: &Value, depth: usize) -> Result<()> {
        self.budget.tick(depth)?;
        match ty {
            Type::Boolean => require(v.is_boolean(), "value-type")?,
            Type::Text => {
                text(v)?;
            }
            Type::Integer | Type::Decimal(_) => {
                integer(v)?;
            }
            Type::Record(m) => {
                let a = object(v)?;
                require(
                    a.len() == m.len() && m.keys().all(|k| a.contains_key(k)),
                    "record-value",
                )?;
                for (k, t) in m {
                    self.value(t, &v[k], depth + 1)?;
                }
            }
            Type::Variant(m) => {
                fields(v, &["tag", "value"])?;
                let t = m.get(text(&v["tag"])?).ok_or_else(|| err("variant-tag"))?;
                self.value(t, &v["value"], depth + 1)?;
            }
            Type::List(t) => {
                for value in items(v)? {
                    self.value(t, value, depth + 1)?;
                }
            }
            Type::Option(t) => match text(&v["tag"])? {
                "none" => fields(v, &["tag"])?,
                "some" => {
                    fields(v, &["tag", "value"])?;
                    self.value(t, &v["value"], depth + 1)?;
                }
                _ => return Err(err("option-tag")),
            },
            Type::Named(key) => {
                let d = self.nominal(key, depth + 1)?;
                self.value(&d.representation, v, depth + 1)?;
                let context = Context {
                    parameters: BTreeMap::from([("value".into(), d.representation.clone())]),
                    ..Context::default()
                };
                let value = self.evaluate(
                    &d.invariant,
                    &BTreeMap::from([("value".into(), v.clone())]),
                    &context,
                    depth + 1,
                )?;
                require(value == true, "invalid-nominal-value")?;
            }
        }
        Ok(())
    }
}
