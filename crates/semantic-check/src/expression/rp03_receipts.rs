// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Emitted pure P1 declarations, not parsed full source or accepted transitions.
use super::*;
use choreoform_ir_probe_core::Resource;

fn bind(v: &Value) -> Value {
    match v["op"].as_str() {
        Some("literal") => v.clone(),
        Some("read") => {
            assert_eq!(v["cell"], "Instruction");
            json!({"op":"param","name":"request"})
        }
        Some("fact") => {
            assert_eq!(v["name"], "result");
            json!({"op":"param","name":"result"})
        }
        _ => match v {
            Value::Object(m) => {
                Value::Object(m.iter().map(|(k, v)| (k.clone(), bind(v))).collect())
            }
            Value::Array(a) => Value::Array(a.iter().map(bind).collect()),
            _ => v.clone(),
        },
    }
}

#[test]
fn order_receipts_match_exact_request_quantity_warehouse_money_and_revisions() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let base = root.join("docs/benchmarks/rp03");
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(base.join("manifest.json")).unwrap()).unwrap();
    let resources: Vec<(Pin, Vec<u8>)> = manifest["resources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            (
                Pin {
                    id: row["id"].as_str().unwrap().into(),
                    revision: row["revision"].as_str().unwrap().into(),
                },
                std::fs::read(root.join(row["path"].as_str().unwrap())).unwrap(),
            )
        })
        .collect();
    let supplied: Vec<_> = resources
        .iter()
        .map(|(p, b)| Resource {
            id: &p.id,
            revision: &p.revision,
            bytes: b,
        })
        .collect();
    let rules: Value =
        serde_json::from_slice(&std::fs::read(base.join("receipt-rules.json")).unwrap()).unwrap();
    let cases: Value =
        serde_json::from_slice(&std::fs::read(base.join("receipt-cases.json")).unwrap()).unwrap();
    assert_eq!(cases["cases"].as_array().unwrap().len(), 22);
    for case in cases["cases"].as_array().unwrap() {
        // A new budget for each independent finite evaluation, never an
        // artificial reset inside a process/packet validation judgment.
        let mut r = Resources::new(&supplied).unwrap();
        let id = format!(
            "urn:choreoform:benchmarks:rp03:{}:0.1.0",
            case["operation"].as_str().unwrap().replace('_', "-")
        );
        let pin = &resources.iter().find(|(p, _)| p.id == id).unwrap().0;
        let provider = r.provider(pin).unwrap();
        r.literal(provider.input(), &case["request"]).unwrap();
        r.literal(provider.output(), &case["result"]).unwrap();
        let ast = &rules["rules"][case["operation"].as_str().unwrap()];
        let mut context = Context::default();
        context
            .cells
            .insert("Instruction".into(), provider.input().clone());
        context
            .facts
            .insert("result".into(), provider.output().clone());
        let checked = r.expression(ast, &context).unwrap();
        assert_eq!(checked.ty, Type::Boolean);
        assert_eq!(checked.reads, BTreeSet::from(["Instruction".into()]));
        let bound = bind(ast);
        let mut context = Context::default();
        context
            .parameters
            .insert("request".into(), provider.input().clone());
        context
            .parameters
            .insert("result".into(), provider.output().clone());
        let checked = r.expression(&bound, &context).unwrap();
        assert_eq!(checked.ty, Type::Boolean);
        assert!(checked.reads.is_empty());
        let parameters = BTreeMap::from([
            ("request".into(), case["request"].clone()),
            ("result".into(), case["result"].clone()),
        ]);
        assert_eq!(
            r.evaluate(&bound, &parameters, &context, 0).unwrap(),
            case["expected"],
            "{}",
            case["id"]
        );
    }
}

#[test]
fn recovery_guards_do_not_turn_missing_records_into_safe_effects_or_partial_acceptance() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let base = root.join("docs/benchmarks/rp03");
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(base.join("manifest.json")).unwrap()).unwrap();
    let resources: Vec<(Pin, Vec<u8>)> = manifest["resources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            (
                Pin {
                    id: row["id"].as_str().unwrap().into(),
                    revision: row["revision"].as_str().unwrap().into(),
                },
                std::fs::read(root.join(row["path"].as_str().unwrap())).unwrap(),
            )
        })
        .collect();
    let supplied: Vec<_> = resources
        .iter()
        .map(|(p, b)| Resource {
            id: &p.id,
            revision: &p.revision,
            bytes: b,
        })
        .collect();
    let rules: Value =
        serde_json::from_slice(&std::fs::read(base.join("guard-rules.json")).unwrap()).unwrap();
    let cases: Value =
        serde_json::from_slice(&std::fs::read(base.join("guard-cases.json")).unwrap()).unwrap();
    fn bind_cells(v: &Value, names: &BTreeMap<String, String>) -> Value {
        match v["op"].as_str() {
            Some("literal") => v.clone(),
            Some("read") => json!({"op":"param","name":names[v["cell"].as_str().unwrap()]}),
            Some("fact") => panic!("no guessed external facts"),
            _ => match v {
                Value::Object(m) => Value::Object(
                    m.iter()
                        .map(|(k, v)| (k.clone(), bind_cells(v, names)))
                        .collect(),
                ),
                Value::Array(a) => Value::Array(a.iter().map(|v| bind_cells(v, names)).collect()),
                _ => v.clone(),
            },
        }
    }
    assert_eq!(cases["cases"].as_array().unwrap().len(), 8);
    for case in cases["cases"].as_array().unwrap() {
        let mut r = Resources::new(&supplied).unwrap();
        let mut original = Context::default();
        for (name, descriptor) in rules["types"].as_object().unwrap() {
            original
                .cells
                .insert(name.clone(), r.type_descriptor(descriptor).unwrap());
        }
        let ast = &rules["rules"][case["rule"].as_str().unwrap()];
        let checked = r.expression(ast, &original).unwrap();
        assert_eq!(checked.ty, Type::Boolean);
        let mut context = Context::default();
        let mut parameters = BTreeMap::new();
        let mut names = BTreeMap::new();
        for (index, name) in checked.reads.iter().enumerate() {
            let ty = original.cells.get(name).unwrap();
            let value = case["cells"].get(name).unwrap();
            r.literal(ty, value).unwrap();
            let parameter = format!("cell_{index}");
            names.insert(name.clone(), parameter.clone());
            context.parameters.insert(parameter.clone(), ty.clone());
            parameters.insert(parameter, value.clone());
        }
        let bound = bind_cells(ast, &names);
        let checked = r.expression(&bound, &context).unwrap();
        assert_eq!(checked.ty, Type::Boolean);
        assert!(checked.reads.is_empty());
        assert_eq!(
            r.evaluate(&bound, &parameters, &context, 0).unwrap(),
            case["expected"],
            "{}",
            case["id"]
        );
    }
}
