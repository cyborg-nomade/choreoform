// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! The existing closed IR rejects capture fields before semantic interpretation.
use choreoform_ir_probe_core::DefinitionEnvelope;
use serde_json::{Value, json};

#[test]
fn current_wait_wire_cannot_gain_a_capture_field_from_a_core_or_source_change() {
    let current: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/benchmarks/rp01/history/source-0.7/candidate.ir.json"
    ))
    .unwrap();
    let original = DefinitionEnvelope::from_parts(current["body"].clone(), json!({})).unwrap();
    original.check_structure().unwrap();
    let mut body = current["body"].clone();
    body["nodes"]["payment_observe"]["result"] = "payment_result".into();
    let changed = DefinitionEnvelope::from_parts(body, json!({})).unwrap();
    assert!(changed.check_structure().is_err());
    assert_eq!(
        choreoform_semantic_check::check(&changed, &[])
            .unwrap_err()
            .code,
        "structure"
    );
}
