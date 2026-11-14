// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Closed scope records do not silently admit a fault-data port.
use choreoform_ir_probe_core::DefinitionEnvelope;
use serde_json::{Value, json};
#[test]
fn old_and_new_known_wires_refuse_scope_fault_input_even_null() {
    let original: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/benchmarks/rp01/history/source-0.7/candidate.ir.json"
    ))
    .unwrap();
    for version in ["0.1.0", "0.2.0"] {
        let mut body = original["body"].clone();
        if version == "0.2.0" {
            for n in body["nodes"].as_object_mut().unwrap().values_mut() {
                if n["kind"] == "wait" {
                    n["capture"] = Value::Null;
                }
            }
        }
        DefinitionEnvelope::from_parts_versioned(version, body.clone(), json!({}))
            .unwrap()
            .check_structure()
            .unwrap();
        for field in ["faultInput", "faultRecord"] {
            for value in [
                Value::Null,
                json!("payment_result"),
                json!({"result":"payment_result"}),
            ] {
                let mut b = body.clone();
                b["scopes"]["remaining"][field] = value;
                let envelope =
                    DefinitionEnvelope::from_parts_versioned(version, b, json!({})).unwrap();
                assert!(envelope.check_structure().is_err());
                assert_eq!(
                    choreoform_semantic_check::check(&envelope, &[])
                        .unwrap_err()
                        .code,
                    "structure"
                );
            }
        }
    }
}
