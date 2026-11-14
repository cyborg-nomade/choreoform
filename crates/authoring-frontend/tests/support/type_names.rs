// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Test-only finite span rewrite of the largest exact repeated outer type.
use choreoform_authoring_frontend::{Syntax, Tree, parse};
use serde_json::{Value, json};
use std::collections::BTreeMap;
fn has_outcome(t: &Tree) -> bool {
    t.rule == "outcome_reference" || t.children.iter().any(has_outcome)
}
fn visit(
    s: &Syntax<'_>,
    t: &Tree,
    inside: bool,
    out: &mut BTreeMap<String, Vec<std::ops::Range<usize>>>,
) {
    let is_type = t.rule == "type";
    if is_type && !inside && !has_outcome(t) {
        let span = s.span(t);
        out.entry(s.source()[span.clone()].into())
            .or_default()
            .push(span);
    }
    for child in &t.children {
        visit(s, child, inside || is_type, out);
    }
}
pub fn factor(source: &str, ledger: &mut Value) -> String {
    let syntax = parse(source.as_bytes()).unwrap();
    let mut groups = BTreeMap::new();
    visit(&syntax, syntax.tree(), false, &mut groups);
    let (ty, spans) = groups
        .into_iter()
        .filter(|(t, v)| t.len() > 60 && v.len() > 1)
        .max_by_key(|(t, v)| t.len() * (v.len() - 1))
        .expect("finite benchmark has repeated compound types");
    let mut out = source.to_owned();
    for span in spans.into_iter().rev() {
        out.replace_range(span, "type \"Shared instruction type\"");
    }
    let end = out.rfind("End process.").unwrap();
    out.insert_str(
        end,
        &format!("Describe type \"Shared instruction type\" as {ty}.\n"),
    );
    let root = ledger["root"].clone();
    ledger["declarations"].as_array_mut().unwrap().push(json!({"id":"shared_instruction_type","kind":"type-alias","scope":root,"name":"Shared instruction type"}));
    out
}
