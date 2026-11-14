// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Actual native operations; never UI/user/host/authorization evidence.
use choreoform_authoring_frontend::{AuthoringComponent, AuthoringDocument as Document};
use serde_json::{Value, json};
const BASE: &[u8] = include_bytes!("../../../docs/authoring/reversible-0.1/base.json");
fn base() -> Document {
    Document::admit(BASE, &[]).unwrap()
}
fn request(d: &Document, operation: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({"format":"choreoform-authoring-change","version":"0.1.0","base":d.authoring_digest(),"operation":operation})).unwrap()
}
fn import(s: &str, l: &Value) -> Document {
    Document::from_package(
        s.as_bytes(),
        &serde_json::to_vec(l).unwrap(),
        br#"{"format":"choreoform-authoring-layout","version":"0.1.0","entries":[]}"#,
        &[],
    )
    .unwrap()
}
fn parts(d: &Document) -> (String, Value) {
    (
        d.source().into(),
        serde_json::from_str(d.bindings()).unwrap(),
    )
}
#[test]
fn all_published_native_changes_match_exact_authored_goldens_and_digests() {
    let d = base();
    let vectors: Value = serde_json::from_str(include_str!(
        "../../../docs/authoring/reversible-0.1/vectors.json"
    ))
    .unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/authoring/reversible-0.1");
    for row in vectors["pairs"].as_array().unwrap() {
        let req = std::fs::read(root.join(row["request"].as_str().unwrap())).unwrap();
        let expected = Document::admit(
            &std::fs::read(root.join(row["after"].as_str().unwrap())).unwrap(),
            &[],
        )
        .unwrap();
        let actual = d.apply(&req, &[]).unwrap();
        assert_eq!(actual.document(), expected.document(), "{}", row["request"]);
        assert_eq!(actual.export().unwrap(), expected.export().unwrap());
        assert_eq!(actual.authoring_digest(), expected.authoring_digest());
        assert_eq!(actual.candidate().document(), d.candidate().document());
    }
    assert_eq!(
        d.authoring_digest(),
        vectors["authoring"]["base"].as_str().unwrap()
    );
    let reopened = Document::admit(&d.export().unwrap(), &[]).unwrap();
    assert_eq!(reopened.document(), d.document());
}
#[test]
fn all_worked_refusals_are_real_native_failures_without_changing_the_base() {
    let d = base();
    let before = d.export().unwrap();
    let vectors: Value = serde_json::from_str(include_str!(
        "../../../docs/authoring/reversible-0.1/vectors.json"
    ))
    .unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/authoring/reversible-0.1");
    for (name, code) in vectors["refusals"].as_object().unwrap() {
        let raw = std::fs::read(root.join(name)).unwrap();
        let error = if name.ends_with(".change.json") {
            d.apply(&raw, &[]).unwrap_err()
        } else {
            Document::admit(&raw, &[]).unwrap_err()
        };
        assert_eq!(error.code, code.as_str().unwrap(), "{name}");
        assert!(!format!("{error:?}").contains("SECRET"));
    }
    assert_eq!(d.export().unwrap(), before);
}
#[test]
fn same_name_is_a_true_noop_and_companion_whitespace_invalidates_the_full_base() {
    let d = base();
    let req = request(&d, json!({"kind":"rename","target":"same","name":"Same"}));
    let unchanged = d.apply(&req, &[]).unwrap();
    assert_eq!(unchanged.export().unwrap(), d.export().unwrap());
    let mut value = d.document().clone();
    value["bindings"] = format!("{}\n", d.bindings()).into();
    let changed = Document::admit(&serde_json::to_vec(&value).unwrap(), &[]).unwrap();
    assert_eq!(changed.candidate().document(), d.candidate().document());
    assert_ne!(changed.authoring_digest(), d.authoring_digest());
    assert_eq!(changed.apply(&req, &[]).unwrap_err().code, "stale-base");
    let op = request(
        &d,
        json!({"kind":"replace-layout","layout":{"format":"choreoform-authoring-layout","version":"0.1.0","entries":[{"entity":"same","x":-1000000,"y":1000000,"collapsed":true}]}}),
    );
    let layout = d.apply(&op, &[]).unwrap();
    assert_eq!(layout.source(), d.source());
    assert_eq!(layout.bindings(), d.bindings());
    assert_eq!(layout.apply(&req, &[]).unwrap_err().code, "stale-base");
}
#[test]
fn type_uses_are_renamed_but_literal_and_type_field_text_are_opaque() {
    let d = base();
    let (mut s, mut l) = parts(&d);
    s = s
        .replace(
            "Keep \"Value\" as whole number",
            "Keep \"Value\" as type \"Counter\"",
        )
        .replace(
            "with inputs [\"Value\" as whole number]",
            "with inputs [\"Value\" as type \"Counter\"]",
        );
    s=s.replace("End process.","Describe type \"Record type\" as record with [\"Counter\" as whole number]. Let \"Literal type name\" mean (\"Counter\") as text with parameters empty. End process.");
    let owner = l["root"].clone();
    for (id, kind, name) in [
        ("record_type", "type-alias", "Record type"),
        ("literal_type", "expression", "Literal type name"),
    ] {
        l["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"kind":kind,"scope":owner,"name":name}));
    }
    let before = import(&s, &l);
    let after = before
        .apply(
            &request(
                &before,
                json!({"kind":"rename","target":"counter","name":"Count type"}),
            ),
            &[],
        )
        .unwrap();
    assert!(
        after
            .source()
            .contains("Keep \"Value\" as type \"Count type\"")
    );
    assert!(
        after
            .source()
            .contains("with inputs [\"Value\" as type \"Count type\"]")
    );
    assert!(
        after
            .source()
            .contains("record with [\"Counter\" as whole number]")
    );
    assert!(after.source().contains("mean (\"Counter\") as text"));
    assert_eq!(after.candidate().document(), before.candidate().document());
}
#[test]
fn nested_calls_keep_callee_argument_names_separate_from_caller_parameters_and_cells() {
    let d = base();
    let (mut s, mut l) = parts(&d);
    s = s.replace(
        "Let \"Read condition\" mean (test \"Same\" using",
        "Let \"Read condition\" mean (test \"Forward\" using",
    );
    s=s.replace("End process.","Define test \"Forward\" with inputs [\"Value\" as whole number]. It holds when (test \"Same\" using [\"Value\" is (the parameter \"Value\")]). End test. End process.");
    let owner = l["root"].clone();
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"forward","kind":"predicate","scope":owner,"name":"Forward"}));
    l["symbols"].as_array_mut().unwrap().push(json!({"id":"forward_value","owner":"forward","role":"predicate-input","name":"Value","wire":"forward_v"}));
    let before = import(&s, &l);
    let after = before
        .apply(
            &request(
                &before,
                json!({"kind":"rename","target":"same_value","name":"Operand"}),
            ),
            &[],
        )
        .unwrap();
    assert!(
        after
            .source()
            .contains("test \"Same\" using [\"Operand\" is (the parameter \"Value\")]")
    );
    assert!(
        after
            .source()
            .contains("Define test \"Forward\" with inputs [\"Value\" as whole number]")
    );
    assert!(
        after
            .source()
            .contains("test \"Forward\" using [\"Value\" is (the value of \"Value\")]")
    );
    let again = after
        .apply(
            &request(
                &after,
                json!({"kind":"rename","target":"forward_value","name":"Input"}),
            ),
            &[],
        )
        .unwrap();
    assert!(
        again
            .source()
            .contains("test \"Same\" using [\"Operand\" is (the parameter \"Input\")]")
    );
    assert!(
        again
            .source()
            .contains("test \"Forward\" using [\"Input\" is (the value of \"Value\")]")
    );
    assert_eq!(again.candidate().document(), before.candidate().document());
}
#[test]
fn unicode_and_quoted_names_preserve_other_source_bytes_and_opaque_annotations() {
    let d = base();
    let caption = "Equal \"values\"\\β";
    let after = d
        .apply(
            &request(&d, json!({"kind":"rename","target":"same","name":caption})),
            &[],
        )
        .unwrap();
    let quoted = serde_json::to_string(caption).unwrap();
    assert!(after.source().contains(&format!("Define test {quoted}")));
    let expected = d
        .source()
        .replace("Define test \"Same\"", &format!("Define test {quoted}"))
        .replace("test \"Same\" using", &format!("test {quoted} using"));
    assert_eq!(after.source(), expected);
    assert_eq!(after.candidate().document(), d.candidate().document());
    for bad in ["", "bad\nname", "bad\u{2028}name"] {
        let error = d
            .apply(
                &request(&d, json!({"kind":"rename","target":"same","name":bad})),
                &[],
            )
            .unwrap_err();
        assert_eq!(error.code, "change-format");
        assert_eq!(error.component, AuthoringComponent::Change);
    }
}
