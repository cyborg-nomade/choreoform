// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Structural local data/node roles, without storage or execution authority.
use choreoform_ir_probe_core::{DefinitionEnvelope, Error};
use serde_json::{Value, json};
fn body() -> Value {
    let mut b: Value = serde_json::from_slice::<Value>(include_bytes!(
        "../../../docs/benchmarks/rp01/candidate.ir.json"
    ))
    .unwrap()["body"]
        .clone();
    for s in b["scopes"].as_object_mut().unwrap().values_mut() {
        s["faultInput"] = Value::Null;
        s["retainedRequests"] = json!({});
    }
    for n in b["nodes"].as_object_mut().unwrap().values_mut() {
        if n["kind"] == "wait" {
            n["capture"] = Value::Null;
        }
    }
    b
}
fn env(b: Value) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.4.0", b, json!({})).unwrap()
}
#[test]
fn new_wire_has_closed_required_map_and_preserves_old_domains() {
    let e = env(body());
    assert!(e.check_structure().unwrap().retained_requests.is_empty());
    assert_eq!(
        DefinitionEnvelope::decode(&serde_json::to_vec(e.document()).unwrap())
            .unwrap()
            .document(),
        e.document()
    );
    assert_ne!(
        DefinitionEnvelope::from_parts_versioned("0.3.0", body(), json!({}))
            .unwrap()
            .document()["revision"],
        e.document()["revision"]
    );
    let mut b = body();
    b["scopes"]["expense"]
        .as_object_mut()
        .unwrap()
        .remove("retainedRequests");
    assert!(env(b).check_structure().is_err());
    let mut b = body();
    b["scopes"]["expense"]["retainedRequests"] = Value::Null;
    assert!(env(b).check_structure().is_err());
    let mut b = body();
    b["scopes"]["expense"]["extra"] = json!({});
    assert!(env(b).check_structure().is_err());
}
#[test]
fn exact_local_data_and_origin_node_roles_are_projected_without_semantic_admission() {
    let mut b = body();
    b["scopes"]["notice_service"]["retainedRequests"] =
        json!({"notice_result":"notice_request_step"});
    let graph = env(b.clone()).check_structure().unwrap();
    let (scope, m) = graph.retained_requests.iter().next().unwrap();
    assert_eq!(scope.as_str(), "notice_service");
    let (result, origin) = m.iter().next().unwrap();
    assert_eq!(
        (result.as_str(), origin.as_str()),
        ("notice_result", "notice_request_step")
    );
    for value in [
        json!({"current_claim":"notice_request_step"}),
        json!({"notice_result":"notice_result"}),
        json!({"notice_result":null}),
    ] {
        b["scopes"]["notice_service"]["retainedRequests"] = value;
        assert!(env(b.clone()).check_structure().is_err());
    }
}
#[test]
fn old_wire_refuses_new_scope_fields_and_unknown_versions_still_refuse() {
    let b = body();
    for wire in ["0.1.0", "0.2.0", "0.3.0"] {
        assert!(
            DefinitionEnvelope::from_parts_versioned(wire, b.clone(), json!({}))
                .unwrap()
                .check_structure()
                .is_err()
        );
    }
    assert_eq!(
        DefinitionEnvelope::from_parts_versioned("0.5.0", b, json!({})).unwrap_err(),
        Error::Version
    );
}
