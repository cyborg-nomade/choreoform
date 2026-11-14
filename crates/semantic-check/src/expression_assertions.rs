// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Additional source-type obligations. They can reject, never relax native checks.
use super::*;
use choreoform_ir_probe_core::transport;
use std::{io, sync::Arc};

/// An extra exact type requirement at an expression JSON pointer. This is source
/// metadata, not wire IR, a runtime call, a trusted value or execution authority.
#[derive(Clone, Copy, Debug)]
pub struct ExpressionTypeAssertion<'a> {
    pub path: &'a str,
    pub expected: &'a Value,
}
struct Entry {
    index: usize,
    expected: Arc<Type>,
    visited: bool,
}
#[derive(Default)]
pub(crate) struct ExpressionAssertions {
    // Addresses identify borrowed immutable nodes only during the scoped check.
    // They are never dereferenced, persisted, serialized or exposed as identity.
    sites: BTreeMap<usize, Vec<Entry>>,
}
struct Counter(usize);
impl io::Write for Counter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0 = self
            .0
            .checked_add(bytes.len())
            .filter(|n| *n <= transport::MAX_BYTES)
            .ok_or_else(|| io::Error::other("source type metadata bound"))?;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn at(index: usize, code: &'static str) -> Diagnostic {
    err(code).at_path(format!("/sourceTypes/{index}"))
}
fn charge_raw_bytes(bytes: &mut usize, amount: usize, limit: usize) -> Result<()> {
    *bytes = bytes
        .checked_add(amount)
        .filter(|n| *n <= limit)
        .ok_or_else(|| err("source-type-budget"))?;
    Ok(())
}
fn inspect_raw(
    value: &Value,
    depth: usize,
    budget: &mut Budget,
    bytes: &mut usize,
    limit: usize,
) -> Result<()> {
    budget.tick(depth)?;
    match value {
        Value::String(s) => charge_raw_bytes(bytes, s.len(), limit)?,
        Value::Object(map) => {
            for (name, child) in map {
                charge_raw_bytes(bytes, name.len(), limit)?;
                inspect_raw(child, depth + 1, budget, bytes, limit)?;
            }
        }
        Value::Array(items) => {
            for child in items {
                inspect_raw(child, depth + 1, budget, bytes, limit)?;
            }
        }
        _ => {}
    }
    Ok(())
}
impl Resources<'_> {
    /// Type a pure expression with additional source assertions, retaining this
    /// resource registry's cumulative budget. Missing/unvisited sites refuse;
    /// opaque literal data cannot masquerade as expression sites. State is cleared
    /// on either success or failure. This is not a general expression evaluator.
    pub fn expression_with_type_assertions(
        &mut self,
        ast: &Value,
        context: &Context,
        assertions: &[ExpressionTypeAssertion<'_>],
    ) -> Result<Judgment> {
        self.install_expression_assertions(ast, assertions)?;
        let result = self.expression(ast, context);
        let pending = self.expression_assertions.take();
        let judgment = result?;
        Self::finish_assertions(pending)?;
        Ok(judgment)
    }
    pub(crate) fn install_expression_assertions(
        &mut self,
        root: &Value,
        assertions: &[ExpressionTypeAssertion<'_>],
    ) -> Result<()> {
        require(self.expression_assertions.is_none(), "source-type-state")?;
        require(assertions.len() <= MAX_ITEMS, "source-type-budget")?;
        if assertions.is_empty() {
            return Ok(());
        }
        let mut pending = ExpressionAssertions::default();
        let mut types = BTreeMap::<usize, Arc<Type>>::new();
        let mut bytes = Counter(0);
        for (index, assertion) in assertions.iter().enumerate() {
            self.budget.tick(0)?;
            io::Write::write_all(&mut bytes, assertion.path.as_bytes())
                .map_err(|_| at(index, "source-type-budget"))?;
            let node = root
                .pointer(assertion.path)
                .filter(|v| v["op"].is_string())
                .ok_or_else(|| at(index, "source-type-site"))?;
            let key = std::ptr::from_ref(assertion.expected) as usize;
            let expected = if let Some(ty) = types.get(&key) {
                Arc::clone(ty)
            } else {
                inspect_raw(
                    assertion.expected,
                    0,
                    &mut self.budget,
                    &mut 0,
                    transport::MAX_BYTES - bytes.0,
                )
                .map_err(|e| at(index, e.code))?;
                transport::admit_value(assertion.expected)
                    .map_err(|_| at(index, "source-type-budget"))?;
                serde_json::to_writer(&mut bytes, assertion.expected)
                    .map_err(|_| at(index, "source-type-budget"))?;
                let ty = Arc::new(
                    self.type_descriptor(assertion.expected)
                        .map_err(|e| at(index, e.code))?,
                );
                types.insert(key, Arc::clone(&ty));
                ty
            };
            pending
                .sites
                .entry(std::ptr::from_ref(node) as usize)
                .or_default()
                .push(Entry {
                    index,
                    expected,
                    visited: false,
                });
        }
        self.expression_assertions = Some(pending);
        Ok(())
    }
    pub(crate) fn expression_without_source_context(
        &mut self,
        ast: &Value,
        context: &Context,
    ) -> Result<Judgment> {
        let previous = self.expression_source_context;
        self.expression_source_context = false;
        let result = self.expression(ast, context);
        self.expression_source_context = previous;
        result
    }
    pub(crate) fn assert_expression_type(&mut self, ast: &Value, actual: &Type) -> Result<()> {
        let key = std::ptr::from_ref(ast) as usize;
        let count = self
            .expression_assertions
            .as_ref()
            .and_then(|a| a.sites.get(&key))
            .map_or(0, Vec::len);
        if count > 0 && !self.expression_source_context {
            return Err(at(
                self.expression_assertions.as_ref().unwrap().sites[&key][0].index,
                "source-type-context",
            ));
        }
        for position in 0..count {
            let entry = &self.expression_assertions.as_ref().unwrap().sites[&key][position];
            let index = entry.index;
            let expected = Arc::clone(&entry.expected);
            // Charge the complete type comparison on every visit/context. Never
            // cache an actual type across uses of the same policy AST.
            self.valid_type(&expected, 0)
                .map_err(|e| at(index, e.code))?;
            if actual != expected.as_ref() {
                return Err(at(index, "source-type-mismatch"));
            }
            self.expression_assertions
                .as_mut()
                .unwrap()
                .sites
                .get_mut(&key)
                .unwrap()[position]
                .visited = true;
        }
        Ok(())
    }
    fn finish_assertions(pending: Option<ExpressionAssertions>) -> Result<()> {
        if let Some(pending) = pending
            && let Some(index) = pending
                .sites
                .values()
                .flatten()
                .filter(|e| !e.visited)
                .map(|e| e.index)
                .min()
        {
            return Err(at(index, "source-type-unvisited"));
        }
        Ok(())
    }
    pub(crate) fn finish_expression_assertions(&mut self) -> Result<()> {
        Self::finish_assertions(self.expression_assertions.take())
    }
}

#[cfg(test)]
mod tests;
