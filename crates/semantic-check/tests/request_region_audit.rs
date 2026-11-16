// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Mathematical V1/V2 and declaration audit; no accepted request/event/overlap trace.
use choreoform_ir_probe_core::{Resource, digest};
use choreoform_semantic_check::{Pin, Resources};
use serde_json::{Value, json};

struct Fixture {
    types: Vec<u8>,
    order: Value,
    body: Value,
}
impl Fixture {
    fn new() -> Self {
        Self {
            types: include_bytes!("../../../docs/benchmarks/rp03/types.json").to_vec(),
            order: serde_json::from_slice::<Value>(include_bytes!(
                "../../../docs/benchmarks/rp03/launch-values.json"
            ))
            .unwrap()["order"]
                .clone(),
            body: serde_json::from_slice::<Value>(include_bytes!(
                "../../../docs/benchmarks/rp03/candidate.ir.json"
            ))
            .unwrap()["body"]
                .clone(),
        }
    }
    fn pin(&self) -> Pin {
        Pin {
            id: "urn:choreoform:benchmarks:rp03:types:0.1.0".into(),
            revision: digest(&self.types),
        }
    }
    fn valid_order(&self, value: &Value) {
        let pin = self.pin();
        let supplied = [Resource {
            id: &pin.id,
            revision: &pin.revision,
            bytes: &self.types,
        }];
        let mut r = Resources::new(&supplied).unwrap();
        let ty=r.type_descriptor(&json!({"kind":"named","contract":{"id":pin.id,"revision":pin.revision},"name":"Order"})).unwrap();
        r.literal(&ty, value).unwrap();
    }
}
fn warehouse<'a>(order: &'a Value, which: &str) -> &'a str {
    order[which]["warehouse"].as_str().unwrap()
}

#[test]
fn actual_request_construction_paths_are_typed_declarations_not_snapshot_identity() {
    let f = Fixture::new();
    for (key, line) in [("reserve_a", "first line"), ("reserve_b", "second line")] {
        let call = &f.body["nodes"][format!("{key}_call")];
        assert_eq!(call["kind"], "invoke");
        assert_eq!(call["body"], format!("{key}_service"));
        assert_eq!(call["inputs"]["request"], format!("{key}_call_instruction"));
        let expression = &f.body["expressions"][format!("{key}_call_instruction")];
        assert_eq!(expression["reads"], json!({"original_order":true}));
        assert_eq!(
            expression["body"]["fields"]["target warehouse"],
            json!({"op":"some","value":{"op":"field","name":"warehouse","value":{"op":"field","name":line,"value":{"op":"unwrapNamed","value":{"op":"read","cell":"original_order"}}}}})
        );
        assert_eq!(
            f.body["scopes"][format!("{key}_service")]["inputs"]["request"],
            format!("{key}_request")
        );
        assert_eq!(
            f.body["expressions"][format!("{key}_input")]["body"],
            json!({"op":"read","cell":format!("{key}_request")})
        );
        assert_eq!(
            f.body["nodes"][format!("{key}_activity")]["input"],
            format!("{key}_input")
        );
    }
    // Same cell ID in these ASTs is not an actual occurrence/revision vector.
}

#[test]
fn terminal_and_retained_request_declarations_name_the_original_activity() {
    let f = Fixture::new();
    for key in ["reserve_a", "reserve_b"] {
        let scope = &f.body["scopes"][format!("{key}_recovery")];
        assert_eq!(
            scope["retainedRequests"][format!("{key}_retained")],
            format!("{key}_activity")
        );
        let wait = &f.body["nodes"][format!("{key}_observe")];
        assert_eq!(wait["capture"]["origin"], format!("{key}_activity"));
        assert_eq!(wait["capture"]["result"], format!("{key}_capture"));
        assert_eq!(
            f.body["data"][format!("{key}_retained")]["type"],
            f.body["data"][format!("{key}_request")]["type"]
        );
    }
    // Declaration identity supplies no retained live value or accepted capture.
}

#[test]
fn two_type_valid_revisions_disprove_cross_revision_raw_disequality_as_a_type_theorem() {
    let f = Fixture::new();
    let old = f.order.clone();
    let mut revised = old.clone();
    revised["revision"] = json!("mathematical-order-v2");
    revised["first line"]["revision"] = json!("mathematical-line-a-v2");
    revised["second line"]["revision"] = json!("mathematical-line-b-v2");
    revised["first line"]["warehouse"] = old["second line"]["warehouse"].clone();
    revised["second line"]["warehouse"] = old["first line"]["warehouse"].clone();
    f.valid_order(&old);
    f.valid_order(&revised);
    assert_ne!(
        warehouse(&old, "first line"),
        warehouse(&old, "second line")
    );
    assert_ne!(
        warehouse(&revised, "first line"),
        warehouse(&revised, "second line")
    );
    assert_eq!(
        warehouse(&old, "first line"),
        warehouse(&revised, "second line")
    );
    // This refutes the input-type proposition only. No accepted edit, permission,
    // dispatch, active old work or admissible simultaneous execution is asserted.
}

#[test]
fn raw_text_type_admits_labels_that_alias_under_a_hypothetical_casefold_model() {
    let f = Fixture::new();
    let mut value = f.order.clone();
    value["first line"]["warehouse"] = json!("DEPOT-A");
    value["second line"]["warehouse"] = json!("depot-a");
    f.valid_order(&value);
    let a = warehouse(&value, "first line");
    let b = warehouse(&value, "second line");
    assert_ne!(a, b);
    assert!(a.eq_ignore_ascii_case(b));
    let types: Value = serde_json::from_slice(&f.types).unwrap();
    assert_eq!(
        types["definitions"]["Order"]["representation"]["fields"]["first line"]["fields"]["warehouse"],
        json!({"kind":"text"})
    );
    // ASCII case folding is ONLY a mathematical counter-interpretation here.
    // It is not implemented/registered as this project's physical normalizer.
}

#[test]
fn both_original_and_changed_requests_can_typecheck_without_being_the_same_request() {
    let f = Fixture::new();
    let original = f.order.clone();
    let mut current = original.clone();
    current["revision"] = json!("mathematical-order-v2");
    current["first line"]["warehouse"] = json!("another-mathematical-warehouse");
    let request = |order: &Value, revision: &str| json!({"order":order,"scope key":order["first line"]["line ID"],"scope revision":order["first line"]["revision"],"request revision":revision,"amount":order["first line"]["amount"],"currency":order["currency"],"basis reference":order["order ID"],"address revision":order["address reference"]["revision"],"target quantity":{"tag":"some","value":order["first line"]["quantity"]},"target warehouse":{"tag":"some","value":order["first line"]["warehouse"]}});
    let old_request = request(&original, "mathematical-original");
    let changed_request = request(&current, "mathematical-revised");
    let raw = include_bytes!("../../../docs/benchmarks/rp03/reserve-a.provider.json");
    let provider: Value = serde_json::from_slice(raw).unwrap();
    let pin = Pin {
        id: provider["id"].as_str().unwrap().into(),
        revision: digest(raw),
    };
    let types = f.pin();
    let supplied = [
        Resource {
            id: &types.id,
            revision: &types.revision,
            bytes: &f.types,
        },
        Resource {
            id: &pin.id,
            revision: &pin.revision,
            bytes: raw,
        },
    ];
    let mut resources = Resources::new(&supplied).unwrap();
    let provider = resources.provider(&pin).unwrap();
    resources.literal(provider.input(), &old_request).unwrap();
    resources
        .literal(provider.input(), &changed_request)
        .unwrap();
    assert_ne!(old_request, changed_request);
    assert_ne!(
        old_request["target warehouse"],
        changed_request["target warehouse"]
    );
    // Full provider-input type admission alone cannot justify treating the new
    // value as the original request, a technical retry or correlated capture.
}
