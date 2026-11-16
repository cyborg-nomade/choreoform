// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Finite test transformation using actual native production spans; no editor API.
use choreoform_authoring_frontend::{Tree, parse};
use serde_json::{Value, json};

pub fn factor(source: &str, ledger: &mut Value) -> String {
    let syntax = parse(source.as_bytes()).unwrap();
    fn data<'a>(tree: &'a Tree, out: &mut Vec<&'a Tree>) {
        if tree.rule == "data" {
            out.push(tree);
        }
        for child in &tree.children {
            data(child, out);
        }
    }
    let mut declarations = Vec::new();
    data(syntax.tree(), &mut declarations);
    let first =
        *declarations
            .iter()
            .find(|t| {
                let name = t.children.iter().find(|n| n.rule == "NAME").unwrap();
                let name: String = serde_json::from_str(&source[syntax.span(name)]).unwrap();
                ledger["declarations"].as_array().unwrap().iter().any(|d| {
                    d["kind"] == "data" && d["name"] == name && d["scope"] == ledger["root"]
                })
            })
            .unwrap();
    let child = |rule| first.children.iter().find(|t| t.rule == rule).unwrap();
    let text = |tree: &Tree| &source[syntax.span(tree)];
    let policy = text(child("reference"));
    let sensitivity = text(
        first
            .children
            .iter()
            .filter(|t| t.rule == "NAME")
            .nth(1)
            .unwrap(),
    );
    let purposes = text(child("names"));
    let lists: Vec<_> = first
        .children
        .iter()
        .filter(|t| t.rule == "references")
        .collect();
    let group = format!(
        "Protect data as \"Shared data protection\". Under {policy}. Sensitivity {sensitivity}. Purposes {purposes}. Participants {}. Capabilities {}. End protection.\n",
        text(lists[0]),
        text(lists[1])
    );
    let root = ledger["root"].clone();
    let first_name: String = serde_json::from_str(text(child("NAME"))).unwrap();
    let owner = ledger["declarations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["name"] == first_name && d["kind"] == "data" && d["scope"] == root)
        .unwrap()["id"]
        .clone();
    let symbols: Vec<_> = ledger["symbols"].as_array().unwrap().iter().filter(|s| s["owner"]==owner && s["role"]=="purpose").enumerate().map(|(i,s)| json!({"id":format!("shared_protection_purpose_{i}"),"owner":"shared_protection","role":"purpose","name":s["name"],"wire":s["wire"]})).collect();
    ledger["declarations"].as_array_mut().unwrap().push(json!({"id":"shared_protection","kind":"data-protection","scope":root,"name":"Shared data protection"}));
    ledger["symbols"].as_array_mut().unwrap().extend(symbols);
    let mut output = source.to_owned();
    for tree in declarations.into_iter().rev() {
        let ty = tree.children.iter().find(|t| t.rule == "type").unwrap();
        let capabilities = tree
            .children
            .iter()
            .filter(|t| t.rule == "references")
            .nth(1)
            .unwrap();
        let start = syntax.span(ty).end;
        let end = syntax.span(capabilities).end;
        assert_eq!(&source[end..end + 1], ".");
        output.replace_range(start..end + 1, ", protected as \"Shared data protection\".");
    }
    let end = output.rfind("End process.").unwrap();
    output.insert_str(end, &group);
    output
}
