// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Source-only typed recipes. No IR calls, values, storage or authority are shared.
use super::*;
use predicate_ast::{Argument, Assertion, BOOLEAN, Expr};
#[derive(Clone)]
struct Input {
    id: String,
    name: String,
    wire: String,
    span: std::ops::Range<usize>,
}
struct Plan<'a> {
    tree: &'a Tree,
    scope: String,
    inputs: Vec<Input>,
    refs: BTreeSet<(String, String)>,
}
struct Cached {
    expression: Expr,
    refs: BTreeSet<(String, String)>,
}
pub(super) struct State<'a> {
    plans: BTreeMap<String, Plan<'a>>,
    cache: BTreeMap<String, Cached>,
    visiting: BTreeSet<String>,
    pub types: BTreeMap<String, Value>,
    pub active: Option<BTreeMap<String, String>>,
    pub has_uses: bool,
}
impl Default for State<'_> {
    fn default() -> Self {
        Self {
            plans: BTreeMap::new(),
            cache: BTreeMap::new(),
            visiting: BTreeSet::new(),
            types: BTreeMap::from([(BOOLEAN.into(), json!({"kind":"boolean"}))]),
            active: None,
            has_uses: false,
        }
    }
}
fn source_error(
    e: choreoform_semantic_check::Diagnostic,
    metadata: &[Assertion],
    fallback: std::ops::Range<usize>,
) -> crate::Diagnostic {
    let span = e
        .path
        .strip_prefix("/sourceTypes/")
        .and_then(|i| i.parse::<usize>().ok())
        .and_then(|i| metadata.get(i))
        .map_or(fallback, |m| m.span.clone());
    error(e.code, span)
}
impl<'a, 's> Lower<'a, 's> {
    pub(super) fn prepare_predicates(&mut self, root: &'a Tree) -> Result<()> {
        self.collect_predicates(root, None, 0)?;
        let ids: Vec<_> = self.predicates.plans.keys().cloned().collect();
        for id in ids {
            self.resolve_predicate(&id, 0).map_err(|mut e| {
                if e.span == (0..0) {
                    e.span = self.syntax.span(self.predicates.plans[&id].tree);
                }
                e
            })?;
        }
        Ok(())
    }
    fn collect_predicates(
        &mut self,
        tree: &'a Tree,
        parent: Option<&str>,
        depth: usize,
    ) -> Result<()> {
        if depth > MAX_DEPTH {
            return Err(error("depth", self.syntax.span(tree)));
        }
        self.expansion_charge(1, 0, self.syntax.span(tree))?;
        if matches!(tree.rule, "process" | "scope") {
            let name = self.syntax.text(tree.one("NAME")?)?;
            let scope = self.ledger.declaration(parent, name, "scope")?.id.clone();
            for child in tree.all("declaration") {
                self.collect_predicates(child.first()?, Some(&scope), depth + 1)?;
            }
        } else if tree.rule == "predicate_declaration" {
            let scope = parent.ok_or_else(|| error("predicate-scope", self.syntax.span(tree)))?;
            let id = self.declare(tree, Some(scope), "predicate")?;
            let group = tree.one("predicate_inputs")?;
            let names = group.all("NAME");
            let types = group.all("type");
            let mut inputs = Vec::new();
            let mut refs = BTreeSet::new();
            for (name, ty) in names.into_iter().zip(types) {
                let caption = self.syntax.text(name)?.to_owned();
                let (symbol, wire) = self.declared_symbol(&id, "predicate-input", &caption)?;
                refs.extend(self.type_cost(ty, scope, 0)?.refs);
                let descriptor = self.type_value(ty, scope, 0)?;
                self.predicates.types.insert(symbol.clone(), descriptor);
                inputs.push(Input {
                    id: symbol,
                    name: caption,
                    wire,
                    span: self.syntax.span(name),
                });
            }
            self.predicates.plans.insert(
                id,
                Plan {
                    tree,
                    scope: scope.into(),
                    inputs,
                    refs,
                },
            );
        }
        // Definitions occur only at declaration boundaries. Ordinary expression,
        // policy and step subtrees cannot contain definitions; do not rescan them.
        Ok(())
    }
    fn resolve_predicate(&mut self, id: &str, depth: usize) -> Result<()> {
        if self.predicates.cache.contains_key(id) {
            return Ok(());
        }
        let (tree, scope, inputs, mut refs) = {
            let p = self
                .predicates
                .plans
                .get(id)
                .ok_or_else(|| error("predicate-definition", 0..0))?;
            (p.tree, p.scope.clone(), p.inputs.clone(), p.refs.clone())
        };
        if depth > MAX_DEPTH {
            return Err(error("depth", self.syntax.span(tree)));
        }
        if !self.predicates.visiting.insert(id.into()) {
            return Err(error("predicate-cycle", self.syntax.span(tree)));
        }
        let previous = self.predicates.active.replace(
            inputs
                .iter()
                .map(|i| (i.name.clone(), i.wire.clone()))
                .collect(),
        );
        let result = self.predicate_expression(tree.one("expression")?, &scope, false, depth + 1);
        self.predicates.active = previous;
        let expression = result?;
        self.predicate_refs(tree.one("expression")?, &scope, &mut refs, depth + 1)?;
        let mut used = BTreeSet::new();
        predicate_ast::parameters(&expression.value, &mut used, 0)?;
        if let Some(input) = inputs.iter().find(|i| !used.contains(&i.wire)) {
            return Err(error("predicate-unused-input", input.span.clone()));
        }
        let span = self.syntax.span(tree);
        let work = expression
            .cost
            .work
            .checked_mul(2)
            .ok_or_else(|| error("predicate-expansion", span.clone()))?;
        let bytes = expression
            .cost
            .bytes
            .checked_mul(2)
            .ok_or_else(|| error("predicate-expansion", span.clone()))?;
        self.expansion_charge(work, bytes, span.clone())?;
        // Reserve both the immutable marked cache and one native proof copy.
        let mut clean = expression.value.clone();
        let mut metadata = Vec::new();
        let metadata_bytes = predicate_ast::strip(
            &mut clean,
            "",
            &mut metadata,
            0,
            crate::MAX_BYTES - self.expansion_bytes,
        )?;
        self.expansion_charge(metadata.len(), metadata_bytes, span.clone())?;
        let mut context = choreoform_semantic_check::Context::default();
        for input in &inputs {
            let ty = self
                .resources
                .type_descriptor(&self.predicates.types[&input.id])
                .map_err(|e| error(e.code, input.span.clone()))?;
            context.parameters.insert(input.wire.clone(), ty);
        }
        let assertions = metadata
            .iter()
            .map(|m| {
                Ok(choreoform_semantic_check::ExpressionTypeAssertion {
                    path: &m.path,
                    expected: self
                        .predicates
                        .types
                        .get(&m.key)
                        .ok_or_else(|| error("predicate-metadata", span.clone()))?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let judgment = self
            .resources
            .expression_with_type_assertions(&clean, &context, &assertions)
            .map_err(|e| source_error(e, &metadata, span.clone()))?;
        if judgment.ty != choreoform_semantic_check::Type::Boolean || !judgment.reads.is_empty() {
            return Err(error("predicate-result", span));
        }
        self.predicates.visiting.remove(id);
        self.predicates
            .cache
            .insert(id.into(), Cached { expression, refs });
        Ok(())
    }
    fn predicate_refs(
        &mut self,
        tree: &Tree,
        scope: &str,
        refs: &mut BTreeSet<(String, String)>,
        depth: usize,
    ) -> Result<()> {
        if depth > MAX_DEPTH {
            return Err(error("depth", self.syntax.span(tree)));
        }
        self.expansion_charge(1, 0, self.syntax.span(tree))?;
        match tree.rule {
            "type" => {
                refs.extend(self.type_cost(tree, scope, 0)?.refs);
                return Ok(());
            }
            "outcome_reference" | "purpose_reference" => {
                let kind = if tree.rule == "outcome_reference" {
                    "outcome-owner"
                } else {
                    "data"
                };
                refs.insert((self.r(tree, scope, kind, 0)?, kind.into()));
            }
            "predicate_use" => {
                let id = self.r(tree, scope, "predicate", 0)?;
                refs.extend(self.predicates.cache[&id].refs.iter().cloned());
                refs.insert((id, "predicate".into()));
            }
            _ => {}
        }
        for child in &tree.children {
            self.predicate_refs(child, scope, refs, depth + 1)?;
        }
        Ok(())
    }
    pub(super) fn apply_predicate(
        &mut self,
        tree: &Tree,
        scope: &str,
        facts: bool,
        depth: usize,
    ) -> Result<Expr> {
        let span = self.syntax.span(tree);
        self.predicates.has_uses = true;
        let id = self.r(tree, scope, "predicate", 0)?;
        self.resolve_predicate(&id, depth + 1)?;
        let count = self.predicates.cache[&id].refs.len();
        self.expansion_charge(count, 0, span.clone())?;
        for (id, kind) in &self.predicates.cache[&id].refs {
            let row = &self.ledger.declarations[id];
            if self
                .ledger
                .resolve(scope, &[&row.name], kind)
                .map(|r| &r.id)
                != Ok(id)
            {
                return Err(error("predicate-member-visibility", span));
            }
        }
        let inputs = self.predicates.plans[&id].inputs.clone();
        let group = tree.one("predicate_arguments")?;
        let names = self.names(group)?;
        let expressions = group.all("expression");
        if names.len() != inputs.len() || names.iter().collect::<BTreeSet<_>>().len() != names.len()
        {
            return Err(error("predicate-arguments", span));
        }
        let by_name: BTreeMap<_, _> = inputs.into_iter().map(|i| (i.name.clone(), i)).collect();
        let mut values = Vec::new();
        for (name, expression) in names.into_iter().zip(expressions) {
            let input = by_name
                .get(&name)
                .cloned()
                .ok_or_else(|| error("predicate-arguments", span.clone()))?;
            values.push((
                input,
                self.predicate_expression(expression, scope, facts, depth + 1)?,
                self.syntax.span(expression),
            ));
        }
        let arguments: BTreeMap<_, _> = values
            .iter()
            .map(|(i, e, s)| {
                (
                    i.wire.clone(),
                    Argument {
                        expr: e,
                        key: &i.id,
                        span: s.clone(),
                    },
                )
            })
            .collect();
        let cost = predicate_ast::substitution_cost(
            &self.predicates.cache[&id].expression.value,
            &arguments,
            &mut BTreeSet::new(),
            &span,
            0,
        )?;
        let cost = predicate_ast::result_cost(cost, &span)?;
        self.expansion_charge(cost.work, cost.bytes, span.clone())?;
        let expression = predicate_ast::substitute(
            &self.predicates.cache[&id].expression.value,
            &arguments,
            &mut BTreeSet::new(),
            &span,
            0,
        )?;
        let result = predicate_ast::expect(expression, BOOLEAN, span)?;
        debug_assert_eq!(
            (result.cost.work, result.cost.bytes, result.cost.depth),
            (cost.work, cost.bytes, cost.depth)
        );
        Ok(result)
    }
    pub(super) fn extract_predicate_assertions(&mut self) -> Result<Vec<Assertion>> {
        let mut metadata = Vec::new();
        // Move the body into a temporary root; only expression wrappers are removed.
        let mut body = Value::Object(std::mem::take(&mut self.body));
        let bytes = predicate_ast::strip(
            &mut body,
            "/body",
            &mut metadata,
            0,
            crate::MAX_BYTES - self.expansion_bytes,
        )?;
        self.expansion_charge(metadata.len(), bytes, 0..0)?;
        self.body = match body {
            Value::Object(map) => map,
            _ => unreachable!("body object"),
        };
        Ok(metadata)
    }
}
pub(super) fn validate_calls(
    candidate: &Candidate,
    resources: &[Resource<'_>],
    types: &BTreeMap<String, Value>,
    metadata: &[Assertion],
) -> std::result::Result<(), crate::Diagnostic> {
    let assertions = metadata
        .iter()
        .map(|m| {
            Ok(choreoform_semantic_check::ExpressionTypeAssertion {
                path: &m.path,
                expected: types
                    .get(&m.key)
                    .ok_or_else(|| error("predicate-metadata", m.span.clone()))?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    choreoform_semantic_check::check_with_expression_type_assertions(
        &candidate.envelope,
        resources,
        &assertions,
    )
    .map_err(|e| {
        let fallback = candidate.source_span(&e).unwrap_or(0..0);
        source_error(e, metadata, fallback)
    })?;
    Ok(())
}
