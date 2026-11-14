// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! One scope/owner-aware syntax walk; opaque literals and wires are not names.
use super::*;
use crate::{Ledger, Syntax, Tree};

struct Work(usize);
impl Work {
    fn charge(&mut self, count: usize) -> AuthoringResult<()> {
        self.0 = self
            .0
            .checked_add(count)
            .filter(|n| *n <= 1_000_000)
            .ok_or_else(|| {
                diagnostic("authoring-work", AuthoringComponent::Change, "/operation")
            })?;
        Ok(())
    }
}
fn name(tree: &Tree) -> AuthoringResult<&Tree> {
    tree.children
        .iter()
        .find(|n| n.rule == "NAME")
        .ok_or_else(|| diagnostic("rename-reference", AuthoringComponent::Source, ""))
}
struct Walk<'a, 's> {
    syntax: &'a Syntax<'s>,
    ledger: &'a Ledger,
    target: &'a str,
    spans: Vec<Range<usize>>,
    work: Work,
    replacement_bytes: usize,
}
impl<'a, 's> Walk<'a, 's> {
    fn mark(&mut self, tree: &Tree, id: &str) -> AuthoringResult<()> {
        if id == self.target {
            self.work.charge(1)?;
            if (self.spans.len() + 1)
                .checked_mul(16)
                .and_then(|n| n.checked_add(self.replacement_bytes))
                .is_none_or(|n| n > MAX_BYTES)
            {
                return Err(diagnostic(
                    "authoring-metadata",
                    AuthoringComponent::Change,
                    "/operation",
                ));
            }
            self.spans.push(self.syntax.span(tree));
        }
        Ok(())
    }
    fn text(&self, tree: &'a Tree) -> AuthoringResult<&'a str> {
        self.syntax.text(tree).map_err(source_error)
    }
    fn declaration(
        &mut self,
        tree: &'a Tree,
        scope: Option<&str>,
        kind: &str,
    ) -> AuthoringResult<&'a str> {
        self.work.charge(64)?;
        let row = self
            .ledger
            .declaration(scope, self.text(name(tree)?)?, kind)
            .map_err(source_error)?;
        Ok(&row.id)
    }
    fn symbol(&mut self, token: &'a Tree, owner: &str) -> AuthoringResult<()> {
        self.work.charge(64)?;
        let row = self
            .ledger
            .symbol(owner, "predicate-input", self.text(token)?)
            .map_err(source_error)?;
        self.mark(token, &row.id)
    }
    fn reference(
        &mut self,
        tree: &'a Tree,
        scope: &'a str,
        kind: &str,
    ) -> AuthoringResult<&'a str> {
        let reference = tree.one("reference").map_err(source_error)?;
        let mut parts = Vec::new();
        for token in reference.children.iter().filter(|n| n.rule == "NAME") {
            self.work.charge(1)?;
            parts.push(self.text(token)?);
        }
        self.work.charge(64)?;
        let row = self
            .ledger
            .resolve(scope, &parts, kind)
            .map_err(source_error)?;
        self.mark(name(reference)?, &row.id)?;
        Ok(&row.id)
    }
    fn collect(&mut self) -> AuthoringResult<()> {
        let root = self.syntax.tree().one("process").map_err(source_error)?;
        let mut pending = vec![(root, self.ledger.root(), None)];
        while let Some((tree, mut scope, mut predicate)) = pending.pop() {
            self.work.charge(1)?;
            match tree.rule {
                "process" | "scope" => {
                    scope = self.declaration(
                        tree,
                        if tree.rule == "process" {
                            None
                        } else {
                            Some(scope)
                        },
                        "scope",
                    )?;
                    predicate = None;
                }
                "predicate_declaration" | "type_declaration" | "data_protection" => {
                    let kind = match tree.rule {
                        "predicate_declaration" => "predicate",
                        "type_declaration" => "type-alias",
                        _ => "data-protection",
                    };
                    let id = self.declaration(tree, Some(scope), kind)?;
                    self.mark(name(tree)?, id)?;
                    if kind == "predicate" {
                        predicate = Some(id);
                    }
                }
                "local_type" => {
                    self.reference(tree, scope, "type-alias")?;
                }
                "named_data" => {
                    self.reference(tree, scope, "data-protection")?;
                }
                "predicate_inputs" => {
                    let owner = predicate.ok_or_else(|| {
                        diagnostic("rename-reference", AuthoringComponent::Source, "")
                    })?;
                    for token in tree.children.iter().filter(|n| n.rule == "NAME") {
                        self.symbol(token, owner)?;
                    }
                }
                "parameter" => {
                    if let Some(owner) = predicate {
                        self.symbol(name(tree)?, owner)?;
                    }
                }
                "predicate_use" => {
                    let callee = self.reference(tree, scope, "predicate")?;
                    let arguments = tree.one("predicate_arguments").map_err(source_error)?;
                    for token in arguments.children.iter().filter(|n| n.rule == "NAME") {
                        self.symbol(token, callee)?;
                    }
                    // The expressions below keep the CALLER's predicate context.
                }
                _ => {}
            }
            for child in tree.children.iter().rev() {
                if pending.len() >= MAX_BYTES / 32 {
                    return Err(diagnostic(
                        "authoring-metadata",
                        AuthoringComponent::Change,
                        "/operation",
                    ));
                }
                pending.push((child, scope, predicate));
            }
        }
        Ok(())
    }
}

pub(super) fn validate(operation: &Value) -> AuthoringResult<(&str, &str)> {
    let malformed = || diagnostic("change-format", AuthoringComponent::Change, "/operation");
    let target = operation["target"]
        .as_str()
        .filter(|id| crate::ledger::valid_id(id))
        .ok_or_else(malformed)?;
    let caption = operation["name"]
        .as_str()
        .filter(|s| {
            !s.is_empty()
                && s.chars().count() <= 4096
                && !s
                    .chars()
                    .any(|c| c.is_control() || matches!(c, '\u{2028}' | '\u{2029}'))
        })
        .ok_or_else(malformed)?;
    Ok((target, caption))
}

pub(super) fn rename(
    package: &Package<'_>,
    operation: &Value,
    original_bindings: &str,
) -> AuthoringResult<(String, String)> {
    use AuthoringComponent::Change;
    let (target, caption) = validate(operation)?;
    let ledger = package.ledger();
    if ledger.document()["retired"]
        .as_array()
        .is_some_and(|a| a.iter().any(|id| id == target))
    {
        let mut d = diagnostic("retired-target", Change, "/operation/target");
        d.entity = Some(target.into());
        return Err(d);
    }
    let old = if let Some(d) = ledger.declarations.get(target).filter(|d| {
        matches!(
            d.kind.as_str(),
            "predicate" | "type-alias" | "data-protection"
        )
    }) {
        &d.name
    } else if let Some(s) = ledger
        .symbols
        .get(target)
        .filter(|s| s.role == "predicate-input")
    {
        &s.name
    } else {
        return Err(diagnostic(
            "unsupported-target",
            Change,
            "/operation/target",
        ));
    };
    if old == caption {
        return Ok((package.syntax().source().into(), original_bindings.into()));
    }
    let mut changed = ledger.document().clone();
    let mut work = Work(0);
    for group in ["declarations", "symbols"] {
        for row in changed[group].as_array_mut().expect("admitted ledger") {
            work.charge(1)?;
            if row["id"] == target {
                row["name"] = caption.into();
            }
        }
    }
    // Validate collision/shadowing before collecting any source mutation.
    let raw = bytes(&changed, MAX_BYTES, AuthoringComponent::Bindings)?;
    let changed =
        Ledger::decode(&raw).map_err(|_| diagnostic("name-conflict", Change, "/operation/name"))?;
    let replacement = serde_json::to_string(caption)
        .map_err(|_| diagnostic("change-format", Change, "/operation/name"))?;
    let mut walk = Walk {
        syntax: package.syntax(),
        ledger,
        target,
        spans: Vec::new(),
        work,
        replacement_bytes: replacement.len(),
    };
    walk.collect()?;
    walk.spans.sort_unstable_by_key(|span| span.start);
    if walk.spans.is_empty() {
        return Err(diagnostic("rename-reference", Change, "/operation/target"));
    }
    let source = package.syntax().source();
    let mut end = 0;
    let mut removed = 0usize;
    for span in &walk.spans {
        if span.start < end
            || span.end > source.len()
            || !source.is_char_boundary(span.start)
            || !source.is_char_boundary(span.end)
        {
            return Err(diagnostic("rename-reference", Change, "/operation"));
        }
        removed += span.len();
        end = span.end;
    }
    let length = replacement
        .len()
        .checked_mul(walk.spans.len())
        .and_then(|n| n.checked_add(source.len() - removed))
        .filter(|n| *n <= MAX_BYTES)
        .ok_or_else(|| diagnostic("authoring-size", Change, "/operation"))?;
    let mut result = String::with_capacity(length);
    let mut start = 0;
    for span in walk.spans {
        result.push_str(&source[start..span.start]);
        result.push_str(&replacement);
        start = span.end;
    }
    result.push_str(&source[start..]);
    Ok((result, changed.canonical().into()))
}
