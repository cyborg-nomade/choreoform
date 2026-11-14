// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Finite benchmark alternatives using real native spans, not a general exporter.
use choreoform_authoring_frontend::{Syntax, Tree, parse};
use serde_json::{Value, json};
trait FixtureTree {
    fn child(&self, rule: &str) -> Option<&Tree>;
    fn children_named(&self, rule: &str) -> Vec<&Tree>;
}
impl FixtureTree for Tree {
    fn child(&self, rule: &str) -> Option<&Tree> {
        self.children.iter().find(|n| n.rule == rule)
    }
    fn children_named(&self, rule: &str) -> Vec<&Tree> {
        self.children.iter().filter(|n| n.rule == rule).collect()
    }
}
fn caption(s: &Syntax<'_>, tree: &Tree) -> String {
    serde_json::from_str(&s.source()[s.span(tree)]).unwrap()
}
fn q(s: &str) -> String {
    serde_json::to_string(s).unwrap()
}
fn ty(v: &Value, resource: &str, pin: &Value) -> String {
    match v["kind"].as_str().unwrap() {
        "text" => "text".into(),
        "boolean" => "truth value".into(),
        "integer" => "whole number".into(),
        "decimal" => format!("decimal with {} places", v["scale"]),
        "option" => format!("optional {}", ty(&v["element"], resource, pin)),
        "list" => format!("list of {}", ty(&v["element"], resource, pin)),
        "record" | "variant" => {
            let record = v["kind"] == "record";
            let fields = v[if record { "fields" } else { "cases" }]
                .as_object()
                .unwrap();
            let values = fields
                .iter()
                .map(|(k, t)| format!("{} as {}", q(k), ty(t, resource, pin)))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "{} with [{}]",
                if record { "record" } else { "choice" },
                values
            )
        }
        "named" => {
            assert_eq!(&v["contract"], pin);
            format!(
                "type {} from {}",
                q(v["name"].as_str().unwrap()),
                q(resource)
            )
        }
        _ => panic!("finite fixture type"),
    }
}
fn expression(v: &Value, labels: &[(&str, &str)]) -> String {
    let e = |child: &Value| expression(child, labels);
    match v["op"].as_str().unwrap() {
        "param" => format!(
            "the parameter {}",
            q(labels
                .iter()
                .find(|(wire, _)| *wire == v["name"].as_str().unwrap())
                .unwrap()
                .1)
        ),
        "literal" => match v["type"]["kind"].as_str().unwrap() {
            "text" => q(v["value"].as_str().unwrap()),
            "boolean" => v["value"].to_string(),
            _ => panic!("finite fixture literal"),
        },
        "field" => format!(
            "field {} of ({})",
            q(v["name"].as_str().unwrap()),
            e(&v["value"])
        ),
        "unwrapNamed" => format!("the representation of ({})", e(&v["value"])),
        "payload" => format!(
            "the payload of ({}) for {}",
            e(&v["value"]),
            q(v["tag"].as_str().unwrap())
        ),
        "tag" => format!("the case of ({})", e(&v["value"])),
        "not" => format!("not ({})", e(&v["value"])),
        "if" => format!(
            "if ({}) then ({}) otherwise ({})",
            e(&v["condition"]),
            e(&v["then"]),
            e(&v["else"])
        ),
        "eq" | "and" | "or" => format!(
            "({}) {} ({})",
            e(&v["left"]),
            match v["op"].as_str().unwrap() {
                "eq" => "equals",
                "and" => "and",
                _ => "or",
            },
            e(&v["right"])
        ),
        _ => panic!("finite fixture expression"),
    }
}
fn abstract_terms(v: &Value, pairs: &[(Value, &str)]) -> Value {
    if let Some((_, name)) = pairs.iter().find(|(old, _)| old == v) {
        return json!({"op":"param","name":name});
    }
    if v["op"] == "literal" {
        return v.clone();
    }
    match v {
        Value::Object(m) => Value::Object(
            m.iter()
                .map(|(k, v)| (k.clone(), abstract_terms(v, pairs)))
                .collect(),
        ),
        Value::Array(a) => Value::Array(a.iter().map(|v| abstract_terms(v, pairs)).collect()),
        _ => v.clone(),
    }
}
fn declarations<'a>(
    s: &Syntax<'_>,
    tree: &'a Tree,
    l: &Value,
    parent: Option<&str>,
    out: &mut Vec<(String, &'a Tree)>,
) {
    let scope = if matches!(tree.rule, "process" | "scope") {
        let name = caption(s, tree.child("NAME").unwrap());
        Some(
            l["declarations"]
                .as_array()
                .unwrap()
                .iter()
                .find(|d| {
                    d["kind"] == "scope" && d["name"] == name && d["scope"].as_str() == parent
                })
                .unwrap()["id"]
                .as_str()
                .unwrap()
                .to_owned(),
        )
    } else {
        parent.map(str::to_owned)
    };
    if matches!(tree.rule, "work_policy" | "step") {
        out.push((scope.clone().unwrap(), tree));
    }
    for child in &tree.children {
        declarations(s, child, l, scope.as_deref(), out);
    }
}
fn find<'a>(tree: &'a Tree, predicate: &impl Fn(&Tree) -> bool) -> Option<&'a Tree> {
    if predicate(tree) {
        return Some(tree);
    }
    tree.children.iter().find_map(|c| find(c, predicate))
}
fn add_test(l: &mut Value, id: &str, name: &str, inputs: &[(&str, &str, &str)]) {
    let scope = l["root"].clone();
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":id,"kind":"predicate","scope":scope,"name":name}));
    for (key, caption, wire) in inputs {
        l["symbols"].as_array_mut().unwrap().push(json!({"id":format!("{id}_{key}"),"owner":id,"role":"predicate-input","name":caption,"wire":wire}));
    }
}
fn first_leaf(v: &Value) -> &Value {
    if v["op"] == "and" {
        first_leaf(&v["left"])
    } else {
        v
    }
}
fn leaves(v: &Value) -> usize {
    if v["op"] == "and" {
        leaves(&v["left"]) + leaves(&v["right"])
    } else {
        1
    }
}
pub fn factor(name: &str, source: &str, l: &mut Value, doc: &Value, types: &Value) -> String {
    assert_eq!(l["profile"], "0.18.0");
    assert_eq!(l["version"], "0.17.0");
    let syntax = parse(source.as_bytes()).unwrap();
    let mut trees = Vec::new();
    declarations(&syntax, syntax.tree(), l, None, &mut trees);
    let b = &doc["body"];
    let mut edits = Vec::new();
    let mut definitions = String::new();
    // The actual resource pin comes from the canonical IR, not a re-encoding.
    let type_pin = if name == "rp03" {
        b["capabilities"]["reserve_a_capability"]["output"]["body"]["cases"]["reserved"]["contract"]
            .clone()
    } else {
        b["capabilities"]["payment_capability"]["output"]["body"]["cases"]["paid"]["contract"]
            .clone()
    };
    assert_eq!(types["id"], type_pin["id"]);
    if name == "rp03" {
        let resource = "Order types";
        let repr = &types["definitions"]["Reservation A evidence"]["representation"];
        let scope = l["root"].clone();
        l["declarations"].as_array_mut().unwrap().push(json!({"id":"receipt_fields_type","kind":"type-alias","scope":scope,"name":"Receipt fields"}));
        definitions.push_str(&format!(
            "Describe type \"Receipt fields\" as {}.\n",
            ty(repr, resource, &type_pin)
        ));
        let mut templates = std::collections::BTreeMap::new();
        let mut cases = 0;
        for n in b["nodes"]
            .as_object()
            .unwrap()
            .values()
            .filter(|n| n["kind"] == "activity" && n["mode"] == "capability")
        {
            let policy = n["policy"].as_str().unwrap();
            let row = l["declarations"]
                .as_array()
                .unwrap()
                .iter()
                .find(|d| d["id"] == policy)
                .unwrap();
            let work = trees
                .iter()
                .find(|(scope, t)| {
                    t.rule == "work_policy"
                        && scope == row["scope"].as_str().unwrap()
                        && caption(&syntax, t.child("NAME").unwrap())
                            == row["name"].as_str().unwrap()
                })
                .unwrap()
                .1;
            let mut source_case = work.child("expression").unwrap();
            let mut ast = &b["policies"][policy]["body"]["completion"];
            while ast["op"] == "if" {
                let conditional = source_case.child("conditional").unwrap();
                let exprs = conditional.children_named("expression");
                let body = exprs[1];
                let phase = ast["condition"]["right"].clone();
                let tag = phase["value"].as_str().unwrap();
                let operation = first_leaf(&ast["then"])["right"].clone();
                let request = b["expressions"][n["input"].as_str().unwrap()]["body"].clone();
                let evidence = json!({"op":"unwrapNamed","value":{"op":"payload","value":{"op":"fact","name":"result"},"tag":tag}});
                let template = abstract_terms(
                    &ast["then"],
                    &[
                        (evidence, "receipt"),
                        (request, "instruction"),
                        (operation.clone(), "operation"),
                        (phase.clone(), "phase"),
                    ],
                );
                let count = leaves(&ast["then"]);
                if let Some(old) = templates.insert(count, template.clone()) {
                    assert_eq!(old, template);
                }
                let evidence = find(body, &|t| {
                    t.rule == "representation"
                        && t.child("expression").unwrap().children[0].rule == "payload"
                })
                .unwrap();
                let request = find(body, &|t| t.rule == "read").unwrap();
                let use_text = format!(
                    "test {} using [\"Evidence\" is ({}), \"Instruction\" is ({}), \"Operation\" is ({}), \"Phase\" is ({})]",
                    q(if count == 13 {
                        "Receipt agrees"
                    } else {
                        "Reasoned receipt agrees"
                    }),
                    &source[syntax.span(evidence)],
                    &source[syntax.span(request)],
                    q(operation["value"].as_str().unwrap()),
                    q(tag)
                );
                edits.push((syntax.span(body), use_text));
                cases += 1;
                source_case = exprs[2];
                ast = &ast["else"];
            }
        }
        assert_eq!(cases, 55);
        assert_eq!(templates.len(), 2);
        for (count, template) in templates {
            let (id, name) = if count == 13 {
                ("receipt_test", "Receipt agrees")
            } else {
                ("reasoned_receipt_test", "Reasoned receipt agrees")
            };
            let inputs = [
                ("receipt", "Evidence", "receipt"),
                ("instruction", "Instruction", "instruction"),
                ("operation", "Operation", "operation"),
                ("phase", "Phase", "phase"),
            ];
            add_test(l, id, name, &inputs);
            definitions.push_str(&format!("Define test {} with inputs [\"Evidence\" as type \"Receipt fields\", \"Instruction\" as type \"Order instruction\", \"Operation\" as text, \"Phase\" as text]. It holds when ({}). End test.\n",q(name),expression(&template,&[("receipt","Evidence"),("instruction","Instruction"),("operation","Operation"),("phase","Phase")])));
        }
    } else {
        assert_eq!(name, "rp01");
        let template = abstract_terms(
            &b["policies"]["payment_work"]["body"]["completion"],
            &[
                (json!({"op":"fact","name":"result"}), "result"),
                (json!({"op":"read","cell":"payment_request"}), "request"),
            ],
        );
        add_test(
            l,
            "payment_receipt_test",
            "Payment receipt agrees",
            &[
                ("result", "Result", "result"),
                ("request", "Instruction", "request"),
            ],
        );
        definitions.push_str(&format!("Define test \"Payment receipt agrees\" with inputs [\"Result\" as {}, \"Instruction\" as type \"Payable instruction\"]. It holds when ({}). End test.\n",ty(&b["capabilities"]["payment_capability"]["output"]["body"],"Expense types",&type_pin),expression(&template,&[("result","Result"),("request","Instruction")])));
        let work = trees
            .iter()
            .find(|(_, t)| {
                t.rule == "work_policy"
                    && caption(&syntax, t.child("NAME").unwrap()) == "Payment work"
            })
            .unwrap()
            .1;
        edits.push((syntax.span(work.child("expression").unwrap()),"test \"Payment receipt agrees\" using [\"Result\" is (the fact \"result\"), \"Instruction\" is (the value of \"Payable request\")]".into()));
        let step = trees
            .iter()
            .find(|(_, t)| {
                t.rule == "step"
                    && caption(&syntax, t.child("NAME").unwrap()) == "Classify late payment"
            })
            .unwrap()
            .1;
        let assignment = find(step, &|t| t.rule == "assignment")
            .unwrap()
            .child("expression")
            .unwrap();
        let outer = assignment
            .child("conditional")
            .unwrap()
            .children_named("expression");
        let inner = outer[1]
            .child("conditional")
            .unwrap()
            .children_named("expression");
        edits.push((syntax.span(inner[0]),"test \"Payment receipt agrees\" using [\"Result\" is (the payload of (the value of \"Late payment evidence\") for \"confirmed\"), \"Instruction\" is (the value of \"Original payment request\")]".into()));
    }
    edits.sort_by_key(|(span, _)| span.start);
    for pair in edits.windows(2) {
        assert!(pair[0].0.end <= pair[1].0.start);
    }
    let mut result = source.to_owned();
    for (span, value) in edits.into_iter().rev() {
        result.replace_range(span, &value);
    }
    let end = result.rfind("End process.").unwrap();
    result.insert_str(end, &definitions);
    assert_eq!(result.matches("authoring \"0.18.0\"").count(), 1);
    result = result.replacen("authoring \"0.18.0\"", "authoring \"0.19.0\"", 1);
    l["profile"] = "0.19.0".into();
    l["version"] = "0.18.0".into();
    result
}
