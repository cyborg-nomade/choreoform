// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use choreoform_ir_probe_core::{DefinitionEnvelope, Error, semantic_bytes, transport::MAX_BYTES};
use serde_json::{Value, json};

#[test]
fn constructed_envelope_matches_strict_transport_and_borrowed_projection() {
    let body =
        json!({"unicode": {"😀":1,"\u{e000}":2}, "array":["é","e\u{301}",-9007199254740991i64]});
    let env = DefinitionEnvelope::from_parts(body, json!({"unknown":true})).unwrap();
    let raw = serde_json::to_vec(env.document()).unwrap();
    assert_eq!(env.canonical(), semantic_bytes(&raw).unwrap());
    assert_eq!(
        DefinitionEnvelope::decode(&raw).unwrap().document(),
        env.document()
    );
    assert!(env.canonical().contains("\"😀\":1,\"\u{e000}\":2"));
}

#[test]
fn construction_cannot_bypass_integer_or_depth_admission() {
    for n in [json!(1.0), json!(-0.0), json!(1e10)] {
        assert_eq!(
            DefinitionEnvelope::from_parts(n, json!({})).unwrap_err(),
            Error::NumberToken
        );
    }
    for n in [json!(9007199254740992u64), json!(u64::MAX)] {
        assert_eq!(
            DefinitionEnvelope::from_parts(n, json!({})).unwrap_err(),
            Error::IntegerRange
        );
    }
    let mut body = Value::Null;
    for _ in 0..100 {
        body = Value::Array(vec![body]);
    }
    assert_eq!(
        DefinitionEnvelope::from_parts(body, json!({})).unwrap_err(),
        Error::Depth
    );
}

#[test]
fn exact_compact_envelope_byte_budget_includes_annotations_and_escaping() {
    let base = DefinitionEnvelope::from_parts(json!({}), json!({"padding":""})).unwrap();
    let padding = MAX_BYTES - serde_json::to_vec(base.document()).unwrap().len();
    let exact =
        DefinitionEnvelope::from_parts(json!({}), json!({"padding":"x".repeat(padding)})).unwrap();
    assert_eq!(
        serde_json::to_vec(exact.document()).unwrap().len(),
        MAX_BYTES
    );
    assert_eq!(
        DefinitionEnvelope::from_parts(json!({}), json!({"padding":"x".repeat(padding+1)}))
            .unwrap_err(),
        Error::Size
    );
    // Escaping a control character consumes six bytes, not one byte of output.
    assert_eq!(
        DefinitionEnvelope::from_parts(
            json!({}),
            json!({"padding":format!("{}\0", "x".repeat(padding-1))})
        )
        .unwrap_err(),
        Error::Size
    );
}

#[test]
fn decoding_still_rejects_duplicate_keys_and_numeric_token_spellings() {
    for (raw, expected) in [
        (b"{\"x\":1,\"\\u0078\":2}".as_slice(), Error::DuplicateKey),
        (b"1e0", Error::NumberToken),
    ] {
        assert_eq!(DefinitionEnvelope::decode(raw).unwrap_err(), expected);
    }
}
