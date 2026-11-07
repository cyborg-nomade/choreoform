// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Versioned closed structural projection only, not semantic capture evidence.
use choreoform_ir_probe_core::{
    DefinitionEnvelope, Error, digest, model::NodeKind, semantic_bytes,
};
use serde_json::{Value, json};
use std::collections::BTreeSet;

const RP: &[u8] = include_bytes!("../../../docs/benchmarks/rp01/candidate.ir.json");
fn body() -> Value {
    let d: Value = serde_json::from_slice(RP).unwrap();
    promote(d["body"].clone())
}
fn promote(mut b: Value) -> Value {
    for n in b["nodes"].as_object_mut().unwrap().values_mut() {
        if n["kind"] == "wait" {
            n["capture"] = Value::Null;
        }
    }
    b
}
fn env(b: Value) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.2.0", b, json!({})).unwrap()
}
fn capture() -> Value {
    json!({"source":{"id":"urn:test:declared-source","revision":format!("sha256:{}","1".repeat(64))},"origin":"payment_request_step","result":"payment_result"})
}

#[test]
fn explicit_constructor_keeps_old_version_and_hashes_separate() {
    let old: Value = serde_json::from_slice(RP).unwrap();
    let a =
        DefinitionEnvelope::from_parts(old["body"].clone(), old["annotations"].clone()).unwrap();
    assert_eq!(a.document(), &old);
    let b = env(old["body"].clone());
    assert_ne!(a.document()["revision"], b.document()["revision"]);
    assert_eq!(b.document()["revision"], digest(b.canonical().as_bytes()));
    let raw = serde_json::to_vec(b.document()).unwrap();
    assert_eq!(
        DefinitionEnvelope::decode(&raw).unwrap().document(),
        b.document()
    );
    assert_eq!(semantic_bytes(&raw).unwrap(), b.canonical());
    assert_eq!(
        DefinitionEnvelope::from_parts_versioned("0.3.0", json!({}), json!({})).unwrap_err(),
        Error::Version
    );
    // Explicit construction is transport only: old wait records still miss required capture.
    assert_eq!(b.check_structure().unwrap_err(), Error::Shape);
}

#[test]
fn null_capture_promotion_preserves_all_ten_typed_variants() {
    let mut kinds = BTreeSet::new();
    for raw in [
        RP,
        include_bytes!("../../../examples/ir/01-reimbursement.json").as_slice(),
        include_bytes!("../../../examples/ir/03-order.json").as_slice(),
        include_bytes!("../../../examples/ir/08-incident.json").as_slice(),
    ] {
        let old: Value = serde_json::from_slice(raw).unwrap();
        let old_env = DefinitionEnvelope::decode(raw).unwrap();
        old_env.check_structure().unwrap();
        let new = env(promote(old["body"].clone()));
        let graph = new.check_structure().unwrap();
        for n in graph.nodes.values() {
            kinds.insert(n.kind.name());
            if let NodeKind::Wait { capture, .. } = &n.kind {
                assert!(capture.is_none());
            }
        }
    }
    assert_eq!(kinds.len(), 10);
}

#[test]
fn role_typed_capture_preserves_exact_external_ref_without_claiming_support() {
    let mut b = body();
    b["nodes"]["payment_observe"]["capture"] = capture();
    let e = env(b);
    let graph = e.check_structure().unwrap();
    let wait = graph
        .nodes
        .iter()
        .find(|(id, _)| id.as_str() == "payment_observe")
        .unwrap()
        .1;
    let NodeKind::Wait {
        capture: Some(c), ..
    } = &wait.kind
    else {
        panic!("typed capture")
    };
    assert_eq!(c.origin.as_str(), "payment_request_step");
    assert_eq!(c.result.as_str(), "payment_result");
    assert_eq!(c.source.id(), "urn:test:declared-source");
    assert_eq!(c.source.revision(), format!("sha256:{}", "1".repeat(64)));
    // Metadata origin does not add an ordinary flow or activation edge.
    assert_eq!(
        e.document()["body"]["flows"],
        serde_json::from_slice::<Value>(RP).unwrap()["body"]["flows"]
    );
}

#[test]
fn missing_extra_null_and_old_version_fields_fail_closed() {
    for mutation in [
        "missing",
        "extra",
        "incomplete",
        "bad_revision",
        "empty_source",
        "source_extra",
        "wrong_null",
    ] {
        let mut b = body();
        b["nodes"]["payment_observe"]["capture"] = capture();
        let n = &mut b["nodes"]["payment_observe"];
        match mutation {
            "missing" => {
                n.as_object_mut().unwrap().remove("capture");
            }
            "extra" => n["extra"] = true.into(),
            "incomplete" => {
                n["capture"].as_object_mut().unwrap().remove("origin");
            }
            "bad_revision" => n["capture"]["source"]["revision"] = "wrong".into(),
            "empty_source" => n["capture"]["source"]["id"] = "".into(),
            "source_extra" => n["capture"]["source"]["trusted"] = true.into(),
            _ => n["capture"] = json!({"source":null,"origin":null,"result":null}),
        }
        assert_eq!(
            env(b).check_structure().unwrap_err(),
            Error::Shape,
            "{mutation}"
        );
    }
    for c in [Value::Null, capture()] {
        let mut b = body();
        b["nodes"]["payment_observe"]["capture"] = c;
        assert_eq!(
            DefinitionEnvelope::from_parts(b, json!({}))
                .unwrap()
                .check_structure()
                .unwrap_err(),
            Error::Shape
        );
    }
}

#[test]
fn origin_and_result_must_resolve_in_the_correct_declaration_maps() {
    for (field, value) in [
        ("origin", "current_claim"),
        ("origin", "missing"),
        ("result", "payment_request_step"),
        ("result", "missing"),
    ] {
        let mut b = body();
        b["nodes"]["payment_observe"]["capture"] = capture();
        b["nodes"]["payment_observe"]["capture"][field] = value.into();
        assert_eq!(env(b).check_structure().unwrap_err(), Error::Reference);
    }
}
