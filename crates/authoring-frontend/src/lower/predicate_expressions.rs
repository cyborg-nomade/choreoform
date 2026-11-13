// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Source19 moves expanded children into parents rather than copying subtrees.
use super::*;
use predicate_ast::{Expr, node, scalar};
impl Lower<'_, '_> {
    pub(super) fn predicate_expression(
        &mut self,
        tree: &Tree,
        scope: &str,
        facts: bool,
        depth: usize,
    ) -> Result<Expr> {
        if depth > MAX_DEPTH {
            return Err(error("depth", self.syntax.span(tree)));
        }
        self.expansion_charge(1, 0, self.syntax.span(tree))?;
        let n = tree.first()?;
        if n.rule == "predicate_use" {
            return self.apply_predicate(n, scope, facts, depth + 1);
        }
        let mut args = Vec::new();
        for child in n.all("expression") {
            args.push(self.predicate_expression(child, scope, facts, depth + 1)?);
        }
        let mut args = args.into_iter();
        let mut next = || args.next().ok_or_else(|| error("internal-tree", 0..0));
        Ok(match n.rule {
            "literal" => {
                let (ty, value) = if let Some(ty) = n.all("type").first() {
                    let ty = self.type_value(ty, scope, 0)?;
                    let value = n.one("value")?;
                    self.literal_markers(value, &ty, scope, 0)?;
                    (ty, self.raw_value(value, scope, 0)?)
                } else if let Some(child) = n
                    .children
                    .first()
                    .filter(|c| matches!(c.rule, "outcome_reference" | "purpose_reference"))
                {
                    (json!({"kind":"text"}), self.alias(child, scope)?.into())
                } else if let Some(child) = n.all("TEXT").first() {
                    (json!({"kind":"text"}), self.syntax.text(child)?.into())
                } else if let Some(child) = n.all("INTEGER").first() {
                    (
                        json!({"kind":"integer"}),
                        integer(self.syntax.text(child)?)?.into(),
                    )
                } else if let Some(child) = n.all("NUMBER").first() {
                    let scale = scale(self.syntax.text(n.one("NAT")?)?)?;
                    (
                        json!({"kind":"decimal","scale":scale}),
                        decimal(self.syntax.text(child)?, scale)?.into(),
                    )
                } else {
                    (
                        json!({"kind":"boolean"}),
                        (self.syntax.word(n, 0)? == "true").into(),
                    )
                };
                node(
                    "literal",
                    vec![("type", Expr::raw(ty)?), ("value", Expr::raw(value)?)],
                )?
            }
            "read" => {
                if self.predicates.active.is_some() {
                    return Err(error("predicate-purity", self.syntax.span(n)));
                }
                node(
                    "read",
                    vec![("cell", scalar(self.r(n, scope, "data", 0)?)?)],
                )?
            }
            "parameter" => {
                let caption = self.syntax.text(n.one("NAME")?)?;
                let wire = if let Some(active) = &self.predicates.active {
                    active
                        .get(caption)
                        .cloned()
                        .ok_or_else(|| error("predicate-parameter", self.syntax.span(n)))?
                } else {
                    caption.into()
                };
                node("param", vec![("name", scalar(wire)?)])?
            }
            "fact" => {
                if self.predicates.active.is_some() {
                    return Err(error("predicate-purity", self.syntax.span(n)));
                }
                if !facts {
                    return Err(error("policy-fact", self.syntax.span(n)));
                }
                node(
                    "fact",
                    vec![("name", scalar(self.syntax.text(n.one("NAME")?)?)?)],
                )?
            }
            "field" => node(
                "field",
                vec![
                    ("name", scalar(self.syntax.text(n.one("NAME")?)?)?),
                    ("value", next()?),
                ],
            )?,
            "record" => {
                let fields = self.names(n)?.into_iter().zip(args).collect();
                node("record", vec![("fields", Expr::named_object(fields)?)])?
            }
            "list" => node(
                "list",
                vec![
                    (
                        "element",
                        Expr::raw(self.type_value(n.one("type")?, scope, 0)?)?,
                    ),
                    ("items", Expr::array(args.collect())?),
                ],
            )?,
            "variant" => node(
                "variant",
                vec![
                    (
                        "type",
                        Expr::raw(self.type_value(n.one("type")?, scope, 0)?)?,
                    ),
                    ("tag", scalar(self.tag(n.one("tag")?, scope)?)?),
                    ("value", next()?),
                ],
            )?,
            "absent" => node(
                "none",
                vec![(
                    "element",
                    Expr::raw(self.type_value(n.one("type")?, scope, 0)?)?,
                )],
            )?,
            "some" | "exists" | "present" | "case_name" | "negation" | "length"
            | "representation" => {
                let op = match n.rule {
                    "some" => "some",
                    "exists" => "isSome",
                    "present" => "unwrap",
                    "case_name" => "tag",
                    "negation" => "not",
                    "length" => "length",
                    _ => "unwrapNamed",
                };
                node(op, vec![("value", next()?)])?
            }
            "payload" => node(
                "payload",
                vec![
                    ("value", next()?),
                    ("tag", scalar(self.tag(n.one("tag")?, scope)?)?),
                ],
            )?,
            "conditional" => node(
                "if",
                vec![("condition", next()?), ("then", next()?), ("else", next()?)],
            )?,
            "binary" => {
                let op = match self.syntax.word(n.one("operator")?, 0)? {
                    "and" => "and",
                    "or" => "or",
                    "equals" => "eq",
                    "is" => "lt",
                    "plus" => "add",
                    "minus" => "sub",
                    "times" => "mul",
                    "contains" => "contains",
                    _ => return Err(error("operator", self.syntax.span(n))),
                };
                node(
                    op,
                    if op == "contains" {
                        vec![("list", next()?), ("value", next()?)]
                    } else {
                        vec![("left", next()?), ("right", next()?)]
                    },
                )?
            }
            "index" => node("index", vec![("value", next()?), ("index", next()?)])?,
            "rescale" => node(
                "rescale",
                vec![
                    ("value", next()?),
                    ("scale", scalar(scale(self.syntax.text(n.one("NAT")?)?)?)?),
                ],
            )?,
            "wrap" => node(
                "wrap",
                vec![
                    ("value", next()?),
                    (
                        "type",
                        Expr::raw(self.type_value(n.one("type")?, scope, 0)?)?,
                    ),
                ],
            )?,
            _ => return Err(error("expression-unsupported", self.syntax.span(n))),
        })
    }
}
