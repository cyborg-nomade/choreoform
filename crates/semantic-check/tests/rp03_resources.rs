// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Pure synthetic resource/value checks, not admitted order graphs or events.
use choreoform_ir_probe_core::{Resource, digest};
use choreoform_semantic_check::{Context, Pin, Resources, Type};
use serde_json::{Value, json};

struct Fixture {
    artifacts: Vec<(Pin, Vec<u8>)>,
    manifest: Value,
    order: Value,
    rules: Value,
}
impl Fixture {
    fn new() -> Self {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let base = root.join("docs/benchmarks/rp03");
        let manifest: Value =
            serde_json::from_slice(&std::fs::read(base.join("manifest.json")).unwrap()).unwrap();
        let artifacts = manifest["resources"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                (
                    Pin {
                        id: r["id"].as_str().unwrap().into(),
                        revision: r["revision"].as_str().unwrap().into(),
                    },
                    std::fs::read(root.join(r["path"].as_str().unwrap())).unwrap(),
                )
            })
            .collect();
        let order = serde_json::from_slice::<Value>(
            &std::fs::read(base.join("launch-values.json")).unwrap(),
        )
        .unwrap()["order"]
            .clone();
        let rules = serde_json::from_slice::<Value>(
            &std::fs::read(base.join("receipt-rules.json")).unwrap(),
        )
        .unwrap()["rules"]
            .clone();
        Self {
            artifacts,
            manifest,
            order,
            rules,
        }
    }
    fn supplied(&self) -> Vec<Resource<'_>> {
        self.artifacts
            .iter()
            .map(|(p, b)| Resource {
                id: &p.id,
                revision: &p.revision,
                bytes: b,
            })
            .collect()
    }
    fn pin(&self, suffix: &str) -> Pin {
        self.artifacts
            .iter()
            .find(|(p, _)| p.id.ends_with(suffix))
            .unwrap()
            .0
            .clone()
    }
}

#[test]
fn actual_order_nominal_invariants_reject_wrong_totals_duplicate_lines_and_invalid_quantities() {
    let f = Fixture::new();
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    let p = f.pin(":types:0.1.0");
    r.named_types(&p).unwrap();
    let ty = r
        .type_descriptor(
            &json!({"kind":"named","contract":{"id":p.id,"revision":p.revision},"name":"Order"}),
        )
        .unwrap();
    r.literal(&ty, &f.order).unwrap();
    for mutation in [
        "total",
        "duplicate",
        "quantity",
        "currency",
        "warehouse",
        "delivery",
        "reference",
    ] {
        let mut value = f.order.clone();
        match mutation {
            "total" => value["amount"] = "19999".into(),
            "duplicate" => value["second line"]["line ID"] = value["first line"]["line ID"].clone(),
            "quantity" => value["second line"]["quantity"] = "0".into(),
            "currency" => value["second line"]["currency"] = "USD".into(),
            "warehouse" => {
                value["second line"]["warehouse"] = value["first line"]["warehouse"].clone()
            }
            "delivery" => value["delivery choice"] = "unknown fulfillment policy".into(),
            _ => value["address reference"]["revision"] = "".into(),
        }
        assert_eq!(
            r.literal(&ty, &value).unwrap_err().code,
            "invalid-nominal-value",
            "{mutation}"
        );
    }
    assert_eq!(f.manifest["canonicalIR"], Value::Null);
    assert_eq!(f.manifest["lowering"]["code"], "tokens");
    assert!(
        f.manifest["semanticStageResults"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[test]
fn all_twenty_three_exact_provider_types_and_terminal_sources_check_without_accepted_observations()
{
    let f = Fixture::new();
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    let mut count = 0;
    for row in f.manifest["resources"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row["path"].as_str().unwrap().ends_with(".provider.json"))
    {
        // Each provider/value family is an independent bounded SDK judgment;
        // unrelated negative evaluations do not share a process-check budget.
        let mut r = Resources::new(&supplied).unwrap();
        let p = Pin {
            id: row["id"].as_str().unwrap().into(),
            revision: row["revision"].as_str().unwrap().into(),
        };
        let provider = r.provider(&p).unwrap();
        let op =
            p.id.strip_prefix("urn:choreoform:benchmarks:rp03:")
                .unwrap()
                .strip_suffix(":0.1.0")
                .unwrap()
                .replace('-', "_");
        let mut context = Context::default();
        context
            .cells
            .insert("Instruction".into(), provider.input().clone());
        context
            .facts
            .insert("result".into(), provider.output().clone());
        let judgment = r.expression(&f.rules[&op], &context).unwrap();
        assert_eq!(judgment.ty, Type::Boolean);
        assert_eq!(
            judgment.reads,
            std::collections::BTreeSet::from(["Instruction".into()])
        );
        let phase = provider
            .outcomes()
            .iter()
            .find(|s| !["unavailable", "declined", "problem", "exception"].contains(&s.as_str()))
            .unwrap();
        let value = json!({"operation":op,"phase":phase,"order ID":"fictional-order","order revision":"order-v1",
            "scope key":"fictional-target","scope revision":"target-v1","request revision":"synthetic-instruction-v1",
            "amount":"20000","currency":"EUR","basis reference":"fictional-prior-resource",
            "reference":"fictional-command-receipt","provider event":"synthetic-event-not-accepted",
            "address revision":"address-v1","expiry":"10000000","reason":"Synthetic pure test value",
            "subject reference":{"tag":"some","value":"fictional-operation-subject"},
            "target quantity":if op.ends_with("_a") || op.ends_with("_b") { json!({"tag":"some","value":"1"}) } else { json!({"tag":"none"}) },
            "target warehouse":if op.ends_with("_a") || op.ends_with("_b") { json!({"tag":"some","value":"fictional-warehouse"}) } else { json!({"tag":"none"}) }});
        let wrapped = json!({"tag":phase,"value":value});
        r.literal(provider.output(), &wrapped).unwrap();
        for mutation in ["operation", "amount", "currency", "reference", "subject"] {
            let mut bad = wrapped.clone();
            match mutation {
                "operation" => bad["value"]["operation"] = "unrelated-operation".into(),
                "amount" => bad["value"]["amount"] = "-1".into(),
                "currency" => bad["value"]["currency"] = "JPY".into(),
                "reference" => bad["value"]["reference"] = "".into(),
                _ => bad["value"]["subject reference"] = json!({"tag":"none"}),
            }
            assert_eq!(
                r.literal(provider.output(), &bad).unwrap_err().code,
                "invalid-nominal-value",
                "{op}/{mutation}"
            );
        }
        let source = f.pin(&format!(":{}-terminal:0.1.0", op.replace('_', "-")));
        let source = r.provider_terminal_source(&source).unwrap();
        assert_eq!(source.provider(), &p);
        let Type::Variant(cases) = source.observation_type() else {
            panic!("terminal envelope")
        };
        assert_eq!(cases.len(), 4);
        assert!(!cases.contains_key("unknown"));
        assert!(!cases.contains_key("acknowledged"));
        count += 1;
    }
    assert_eq!(count, 23);
    let clock = r.clock(&f.pin(":clock:0.1.0")).unwrap();
    assert!(clock.source_contract().observations().contains_key("tick"));
}

#[test]
fn immutable_resource_byte_verification_refuses_a_mismatched_declared_contract() {
    let f = Fixture::new();
    for (p, b) in &f.artifacts {
        assert_eq!(digest(b), p.revision);
    }
    let mut supplied = f.supplied();
    supplied
        .iter_mut()
        .find(|r| r.id.ends_with(":types:0.1.0"))
        .unwrap()
        .bytes = b"{}";
    assert_eq!(
        Resources::new(&supplied)
            .unwrap()
            .named_types(&f.pin(":types:0.1.0"))
            .unwrap_err()
            .code,
        "resource-digest"
    );
}
