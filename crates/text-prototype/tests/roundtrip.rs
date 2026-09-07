// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

use choreoform_ir_probe_core::{digest, semantic_bytes};
use choreoform_text_prototype::{MAX_BYTES, export, parse};
use serde_json::{Value, json};

const FIXTURES: [&[u8]; 3] = [
    include_bytes!("../../../examples/ir/01-reimbursement.json"),
    include_bytes!("../../../examples/ir/03-order.json"),
    include_bytes!("../../../examples/ir/08-incident.json"),
];

const TEXTS: [&[u8]; 3] = [
    include_bytes!("../../../examples/text/01-reimbursement.choreo"),
    include_bytes!("../../../examples/text/03-order.choreo"),
    include_bytes!("../../../examples/text/08-incident.choreo"),
];

#[test]
fn checked_in_sources_match_frozen_ir() {
    for (source, raw) in TEXTS.into_iter().zip(FIXTURES) {
        let expected: Value = serde_json::from_slice(raw).unwrap();
        assert_eq!(parse(source).unwrap().lower().unwrap(), expected);
    }
}

#[test]
fn source_maps_bind_to_both_artifacts() {
    let text = source();
    let commented = format!("// shifted spans\n{text}");
    let a = parse(text.as_bytes()).unwrap().binding().unwrap();
    let b = parse(commented.as_bytes()).unwrap().binding().unwrap();
    assert_eq!(a.semantic_revision, b.semantic_revision);
    assert_ne!(a.source_digest, b.source_digest);
}

#[test]
fn syntax_success_does_not_claim_structural_validity() {
    let text = source().replacen("nodes {", "nodes { unvalidated = {};", 1);
    let lowered = parse(text.as_bytes()).unwrap().lower().unwrap();
    assert_eq!(lowered["body"]["nodes"]["unvalidated"], json!({}));
    // Missing node kind and references are deliberately later validation work.
}

#[test]
fn record_order_preserves_identity_but_id_changes_do_not() {
    let text = source();
    let tree = parse(text.as_bytes()).unwrap();
    let section = tree.items().iter().find(|i| i.name == "nodes").unwrap();
    let mut replacement = String::from("nodes {\n");
    for record in section.records.iter().rev() {
        replacement.push_str(&text[record.span.clone()]);
        replacement.push('\n');
    }
    replacement.push('}');
    let mut reordered = text.clone();
    reordered.replace_range(section.span.clone(), &replacement);
    assert_eq!(
        parse(reordered.as_bytes()).unwrap().lower().unwrap(),
        tree.lower().unwrap()
    );
    let record = &section.records[0];
    let mut renamed = text.clone();
    renamed.replace_range(
        record.span.start..record.span.start + record.id.len(),
        "renamedNode",
    );
    assert_ne!(
        parse(renamed.as_bytes()).unwrap().lower().unwrap()["revision"],
        tree.lower().unwrap()["revision"]
    );
}

fn source() -> String {
    export(FIXTURES[0]).unwrap()
}

fn with_annotations(payload: &str) -> String {
    let mut text = source();
    let span = parse(text.as_bytes())
        .unwrap()
        .items()
        .iter()
        .find(|i| i.name == "annotations")
        .unwrap()
        .span
        .clone();
    text.replace_range(span, &format!("annotations = {payload};"));
    text
}

#[test]
fn payload_failures_report_the_actual_lexical_cause() {
    for (payload, category) in [
        (r#"{"x":1,"\u0078":2}"#, "duplicate-key"),
        (r#"{"x":1e0}"#, "number-token"),
        (r#"{"x":1.5}"#, "number-token"),
        (r#"{"x":9007199254740992}"#, "integer-range"),
        (r#"{"x":"\ud800"}"#, "json"),
        (r#"{"x":true,}"#, "json"),
        (r#"{"x": /* comment */ true}"#, "json"),
        ("[]", "wrong metadata value shape"),
    ] {
        let text = with_annotations(payload);
        let diagnostic = parse(text.as_bytes()).unwrap_err();
        assert!(diagnostic.message.contains(category), "{diagnostic:?}");
        assert!(text.is_char_boundary(diagnostic.span.start));
        assert!(text.is_char_boundary(diagnostic.span.end));
    }
}

#[test]
fn envelope_depth_is_checked_again_after_lowering() {
    let payload = format!("{}0{}", "{\"x\":".repeat(64), "}".repeat(64));
    let text = with_annotations(&payload);
    let tree = parse(text.as_bytes()).unwrap();
    assert!(tree.lower().unwrap_err().message.contains("depth"));
}

#[test]
fn delimiters_escapes_comments_and_identifier_boundaries() {
    let text = with_annotations(r#"{"escaped":"a\\\";]}", "n":-0, "edge":-9007199254740991}"#);
    let value = parse(text.as_bytes()).unwrap().lower().unwrap();
    assert_eq!(value["annotations"]["n"], 0);
    let text = source();
    let max_id = format!("a{}", "b".repeat(63));
    let ok = text.replacen("nodes {", &format!("nodes {{ {max_id} = {{}};"), 1);
    assert!(parse(ok.as_bytes()).is_ok());
    let too_long = text.replacen("nodes {", &format!("nodes {{ {max_id}b = {{}};"), 1);
    assert!(parse(too_long.as_bytes()).is_err());
    let comment = text.replacen("id =", "id // comment\n = // comment\n", 1);
    assert!(parse(comment.as_bytes()).is_ok());
    let mismatch = with_annotations("{\"x\": [1}");
    assert!(parse(mismatch.as_bytes()).is_err());
}

#[test]
fn fixtures_repeat_losslessly_with_identical_canonical_bytes() {
    for raw in FIXTURES {
        let expected: Value = serde_json::from_slice(raw).unwrap();
        let text = export(raw).unwrap();
        let syntax = parse(text.as_bytes()).unwrap();
        let actual = syntax.lower().unwrap();
        assert_eq!(actual, expected);
        let wire = serde_json::to_vec(&actual).unwrap();
        assert_eq!(semantic_bytes(&wire), semantic_bytes(raw));
        assert_eq!(export(&wire).unwrap(), text);
        for item in syntax.items() {
            assert!(syntax.source()[item.span.clone()].starts_with(&item.name));
            for record in &item.records {
                assert!(syntax.source()[record.span.clone()].starts_with(&record.id));
            }
        }
    }
}

#[test]
fn comments_whitespace_and_item_order_are_not_semantic() {
    let text = source();
    let decorated = format!("// λ😀\r\n{text}\n// trailing comment");
    let syntax = parse(decorated.as_bytes()).unwrap();
    assert_eq!(syntax.source(), decorated);
    let expected = parse(text.as_bytes()).unwrap().lower().unwrap();
    assert_eq!(syntax.lower().unwrap(), expected);
    let mut reversed = String::from("choreoform \"0.1.0\";\n");
    for item in syntax.items().iter().rev() {
        reversed.push_str(&syntax.source()[item.span.clone()]);
        reversed.push('\n');
    }
    assert_eq!(
        parse(reversed.as_bytes()).unwrap().lower().unwrap(),
        expected
    );
}

#[test]
fn unknown_annotations_arrays_and_unicode_survive() {
    let mut doc: Value = serde_json::from_slice(FIXTURES[0]).unwrap();
    doc["annotations"]["unknown-tool"] =
        json!({"λ😀": [";}", "//", "\\\"", 4, 3], "é": "e\u{301}"});
    let raw = serde_json::to_vec(&doc).unwrap();
    let text = export(&raw).unwrap();
    assert_eq!(parse(text.as_bytes()).unwrap().lower().unwrap(), doc);
}

#[test]
fn policy_edits_change_revision_without_interpretation() {
    let text = source();
    let before = parse(text.as_bytes()).unwrap().lower().unwrap();
    let mut doc = before.clone();
    doc["body"]["policies"]
        .as_object_mut()
        .unwrap()
        .values_mut()
        .next()
        .unwrap()["body"]["probe"] = json!(["a", "b"]);
    let bytes = serde_json::to_vec(&doc).unwrap();
    doc["revision"] = digest(semantic_bytes(&bytes).unwrap().as_bytes()).into();
    assert_ne!(doc["revision"], before["revision"]);
    let changed = export(&serde_json::to_vec(&doc).unwrap()).unwrap();
    assert_eq!(parse(changed.as_bytes()).unwrap().lower().unwrap(), doc);
}

#[test]
fn malformed_source_is_rejected_without_silent_recovery() {
    let text = source();
    let cases = [
        text.replacen("0.1.0", "0.2.0", 1),
        format!("\u{feff}{text}"),
        format!("{text}\nid = \"duplicate\";"),
        format!("{text}\nother {{}}"),
        text.replacen("id =", "id", 1),
        text.replacen("id =", "unknown =", 1),
        text.replacen(
            "annotations =",
            "annotations = {\"x\":1,\"\\u0078\":2}; ignored =",
            1,
        ),
        text.replacen("annotations =", "annotations = {\"x\":1e0}; ignored =", 1),
        text.replacen(
            "annotations =",
            "annotations = {\"x\":9007199254740992}; ignored =",
            1,
        ),
        text.replacen(
            "annotations =",
            "annotations = {\"x\":\"\\ud800\"}; ignored =",
            1,
        ),
        text.replacen("annotations =", "annotations = []; ignored =", 1),
        text.replacen("scopes {", "scopes { bad = [];", 1),
        text.replacen("scopes {", "scopes { 1bad = {};", 1),
        text.replacen("scopes {", "scopes { duplicate = {}; duplicate = {};", 1),
        text.replacen("scopes {", "scopes { cross = {};", 1)
            .replacen("nodes {", "nodes { cross = {};", 1),
    ];
    for malformed in cases {
        assert!(parse(malformed.as_bytes()).is_err(), "accepted {malformed}");
    }
    assert!(parse(&[0xff]).is_err());
    assert!(parse(&vec![b' '; MAX_BYTES + 1]).is_err());
    for item in parse(text.as_bytes()).unwrap().items() {
        let mut missing = text.clone();
        missing.replace_range(item.span.clone(), "");
        assert!(parse(missing.as_bytes()).is_err());
    }
}

#[test]
fn bounded_depth_and_truncation_are_safe() {
    let text = source();
    let deep = text.replacen(
        "annotations =",
        &format!(
            "annotations = {}0{}; ignored =",
            "[".repeat(65),
            "]".repeat(65)
        ),
        1,
    );
    assert!(parse(deep.as_bytes()).is_err());
    for index in 0..text.len() {
        // The exporter emits ASCII fixtures. Every strict prefix lacks a section
        // terminator or later required item (except trailing whitespace).
        let _ = parse(&text.as_bytes()[..index]);
    }
    for byte in 0..=255 {
        let _ = parse(&[byte]);
    }
}

#[test]
fn export_refuses_mismatch_and_unrepresentable_fields() {
    let mut doc: Value = serde_json::from_slice(FIXTURES[0]).unwrap();
    doc["body"]["id"] = "renamed".into();
    assert!(export(&serde_json::to_vec(&doc).unwrap()).is_err());
    doc["body"]["unknown"] = json!({});
    doc["revision"] = digest(
        semantic_bytes(&serde_json::to_vec(&doc).unwrap())
            .unwrap()
            .as_bytes(),
    )
    .into();
    assert!(export(&serde_json::to_vec(&doc).unwrap()).is_err());
}
