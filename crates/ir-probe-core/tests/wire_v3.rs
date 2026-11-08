// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Structural history roles only, never actual handler initialization evidence.
use choreoform_ir_probe_core::{DefinitionEnvelope, Error, model::NodeKind};
use serde_json::{Value, json};
fn body() -> Value {
    let v: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/benchmarks/rp01/candidate.ir.json"
    ))
    .unwrap();
    let mut b = v["body"].clone();
    for s in b["scopes"].as_object_mut().unwrap().values_mut() {
        s["faultInput"] = Value::Null;
    }
    for n in b["nodes"].as_object_mut().unwrap().values_mut() {
        if n["kind"] == "wait" {
            n["capture"] = Value::Null;
        }
    }
    b
}
fn env(b: Value) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.3.0", b, json!({})).unwrap()
}
fn binding() -> Value {
    json!({"history":{"id":"urn:test:history","revision":format!("sha256:{}","1".repeat(64))},"result":"notice_result"})
}
#[test]
fn explicit_new_wire_roundtrips_null_entries_without_changing_old_constructor() {
    let e = env(body());
    assert_eq!(e.document()["version"], "0.3.0");
    let graph = e.check_structure().unwrap();
    assert!(graph.fault_inputs.is_empty());
    assert!(
        graph
            .nodes
            .values()
            .any(|n| matches!(n.kind, NodeKind::Wait { capture: None, .. }))
    );
    let bytes = serde_json::to_vec(e.document()).unwrap();
    assert_eq!(
        DefinitionEnvelope::decode(&bytes).unwrap().document(),
        e.document()
    );
    let old = DefinitionEnvelope::from_parts(body(), json!({})).unwrap();
    assert_eq!(old.document()["version"], "0.1.0");
    assert_ne!(old.document()["revision"], e.document()["revision"]);
}
#[test]
fn role_typed_local_history_preserves_exact_external_pin_without_semantic_authority() {
    let mut b = body();
    b["scopes"]["notice_service"]["faultInput"] = binding();
    let graph = env(b).check_structure().unwrap();
    let entry = graph.fault_inputs.iter().next().unwrap();
    assert_eq!(entry.0.as_str(), "notice_service");
    assert_eq!(entry.1.result.as_str(), "notice_result");
    assert_eq!(entry.1.history.id(), "urn:test:history");
    assert_eq!(
        entry.1.history.revision(),
        format!("sha256:{}", "1".repeat(64))
    );
}
#[test]
fn required_nullable_field_and_closed_reference_record_are_enforced() {
    for mutation in [
        "missing",
        "extra",
        "history_missing",
        "bad_ref",
        "unknown_binding_field",
    ] {
        let mut b = body();
        b["scopes"]["notice_service"]["faultInput"] = binding();
        let s = &mut b["scopes"]["notice_service"];
        match mutation {
            "missing" => {
                s.as_object_mut().unwrap().remove("faultInput");
            }
            "extra" => s["approved"] = true.into(),
            "history_missing" => {
                s["faultInput"].as_object_mut().unwrap().remove("history");
            }
            "bad_ref" => s["faultInput"]["history"]["revision"] = "invalid".into(),
            _ => s["faultInput"]["trusted"] = true.into(),
        }
        assert_eq!(
            env(b).check_structure().unwrap_err(),
            Error::Shape,
            "{mutation}"
        );
    }
}
#[test]
fn history_result_requires_data_role_and_same_scope_ownership() {
    for (result, expected) in [
        ("payment_request_step", Error::Reference),
        ("current_claim", Error::Scope),
    ] {
        let mut b = body();
        b["scopes"]["notice_service"]["faultInput"] = binding();
        b["scopes"]["notice_service"]["faultInput"]["result"] = result.into();
        assert_eq!(env(b).check_structure().unwrap_err(), expected);
    }
}
#[test]
fn old_wires_still_refuse_even_null_history_fields_and_unknown_wire_stays_closed() {
    for version in ["0.1.0", "0.2.0"] {
        let mut b = body();
        if version == "0.1.0" {
            for n in b["nodes"].as_object_mut().unwrap().values_mut() {
                n.as_object_mut().unwrap().remove("capture");
            }
        }
        assert_eq!(
            DefinitionEnvelope::from_parts_versioned(version, b, json!({}))
                .unwrap()
                .check_structure()
                .unwrap_err(),
            Error::Shape
        );
    }
    assert_eq!(
        DefinitionEnvelope::from_parts_versioned("0.4.0", body(), json!({})).unwrap_err(),
        Error::Version
    );
}

#[test]
fn original_illustrative_inspector_does_not_claim_the_new_entry_contract() {
    use choreoform_ir_probe_core::{Resource, SUPPORTED_CONTRACTS, inspect};
    let v: Value =
        serde_json::from_slice(include_bytes!("../../../examples/ir/01-reimbursement.json"))
            .unwrap();
    let mut b = v["body"].clone();
    for s in b["scopes"].as_object_mut().unwrap().values_mut() {
        s["faultInput"] = Value::Null;
    }
    for n in b["nodes"].as_object_mut().unwrap().values_mut() {
        if n["kind"] == "wait" {
            n["capture"] = Value::Null;
        }
    }
    let e = env(b);
    e.check_structure().unwrap();
    let resources = [
        Resource {
            id: SUPPORTED_CONTRACTS[0].0,
            revision: SUPPORTED_CONTRACTS[0].1,
            bytes: include_bytes!(
                "../../../docs/ir/contracts/sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt"
            ),
        },
        Resource {
            id: SUPPORTED_CONTRACTS[1].0,
            revision: SUPPORTED_CONTRACTS[1].1,
            bytes: include_bytes!(
                "../../../docs/ir/contracts/sha256-282dd3c02983141da13e8f700e852d5472ffc4af176a53ca3578c418482a09b9.txt"
            ),
        },
    ];
    assert_eq!(
        inspect(&serde_json::to_vec(e.document()).unwrap(), &resources).unwrap_err(),
        Error::ContractUnsupported
    );
}
