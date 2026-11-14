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

#[test]
fn qualified_ancestor_uses_rename_and_private_descendants_cannot_be_imported() {
    let d = base();
    let (mut s, mut l) = parts(&d);
    let child = "Scope \"Child\" within \"Terminal example\". Start at \"Child done\". Inputs empty. Outputs empty. Outcomes [\"complete\"]. Use cancellation \"Stop\", faults \"Fault handling\", closure \"Close\" and races \"Commit order\". Step \"Child done\". Finish as \"complete\". End step. End scope.";
    let owner = l["root"].clone();
    for (id, kind, scope, name) in [
        ("child", "scope", owner.as_str().unwrap(), "Child"),
        ("child_done", "node", "child", "Child done"),
    ] {
        l["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"kind":kind,"scope":scope,"name":name}));
    }
    l["symbols"].as_array_mut().unwrap().push(json!({"id":"child_complete","owner":"child","role":"outcome","name":"complete","wire":"complete"}));
    s = s.replace("End process.", &format!("{child} End process."));
    let expression = "Let \"Read condition\" mean (test \"Same\" using [\"Value\" is (the value of \"Value\")]) as truth value with parameters empty.";
    assert_eq!(s.matches(expression).count(), 1);
    let qualified = expression.replace(
        "test \"Same\" using",
        "test \"Same\" in \"Terminal example\" using",
    );
    let nested = s.replace(expression, "").replace(
        "Step \"Child done\"",
        &format!("{qualified} Step \"Child done\""),
    );
    let mut n = l.clone();
    n["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|x| x["id"] == "read_condition")
        .unwrap()["scope"] = "child".into();
    let before = import(&nested, &n);
    let after = before
        .apply(
            &request(
                &before,
                json!({"kind":"rename","target":"same","name":"Equal values"}),
            ),
            &[],
        )
        .unwrap();
    assert!(
        after
            .source()
            .contains("test \"Equal values\" in \"Terminal example\" using")
    );
    assert_eq!(after.candidate().document(), before.candidate().document());
    let definition = "Define test \"Same\" with inputs [\"Value\" as whole number]. It holds when ((the parameter \"Value\") equals (the parameter \"Value\")). End test.";
    assert_eq!(s.matches(definition).count(), 1);
    let hidden = s
        .replace(definition, "")
        .replace(
            "Step \"Child done\"",
            &format!("{definition} Step \"Child done\""),
        )
        .replace(
            "test \"Same\" using",
            "test \"Same\" in \"Child\" in \"Terminal example\" using",
        );
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|x| x["id"] == "same")
        .unwrap()["scope"] = "child".into();
    let error = Document::from_package(
        hidden.as_bytes(),
        &serde_json::to_vec(&l).unwrap(),
        br#"{"format":"choreoform-authoring-layout","version":"0.1.0","entries":[]}"#,
        &[],
    )
    .unwrap_err();
    assert_eq!(error.code, "reference-missing");
    assert_eq!(error.component, AuthoringComponent::Source);
}
#[test]
fn untrusted_formats_registries_and_decoded_component_bytes_refuse_privately() {
    use choreoform_ir_probe_core::{Resource, digest};
    let d = base();
    for raw in [
        b"[]".as_slice(),
        b"null",
        b"{\"SECRET\":0,\"SECRET\":1}",
        b"{\"x\":-0}",
        b"{\"x\":1.0}",
        b"\"\\ud800\"",
        b"\xef\xbb\xbf{}",
        b"{} trailing",
    ] {
        let e = Document::admit(raw, &[]).unwrap_err();
        assert_eq!(e.code, "document-format");
        assert!(!format!("{e:?}").contains("SECRET"));
    }
    for raw in ["[]", "null", "true", "\"SECRET\""] {
        let mut v = d.document().clone();
        v["bindings"] = raw.into();
        let e = Document::admit(&serde_json::to_vec(&v).unwrap(), &[]).unwrap_err();
        assert_eq!(e.component, AuthoringComponent::Bindings);
        assert!(!format!("{e:?}").contains("SECRET"));
    }
    let mut v = d.document().clone();
    v["source"] = "é".repeat(524289).into();
    assert_eq!(
        Document::admit(&serde_json::to_vec(&v).unwrap(), &[])
            .unwrap_err()
            .code,
        "component-limit"
    );
    let mut r: Value = serde_json::from_slice(&request(&d, json!({"kind":"format"}))).unwrap();
    r["base"] = "SECRET".into();
    assert_eq!(
        d.apply(&serde_json::to_vec(&r).unwrap(), &[])
            .unwrap_err()
            .code,
        "change-format"
    );
    let digest = digest(b"x");
    let wrong = [Resource {
        id: "SECRET",
        revision: &digest,
        bytes: b"y",
    }];
    assert_eq!(
        Document::admit(BASE, &wrong).unwrap_err().code,
        "resource-integrity"
    );
    for id in [String::new(), "x".repeat(16385), "β".repeat(4097)] {
        let invalid = [Resource {
            id: &id,
            revision: &digest,
            bytes: b"x",
        }];
        let error = Document::admit(BASE, &invalid).unwrap_err();
        assert_eq!(error.code, "resource-integrity");
        assert!(error.entity.is_none());
    }
    let oversized_revision = "r".repeat(100000);
    let invalid = [Resource {
        id: "a",
        revision: &oversized_revision,
        bytes: b"x",
    }];
    assert_eq!(
        Document::admit(BASE, &invalid).unwrap_err().code,
        "resource-integrity"
    );
    let duplicate = [
        Resource {
            id: "a",
            revision: &digest,
            bytes: b"x",
        },
        Resource {
            id: "a",
            revision: &digest,
            bytes: b"x",
        },
    ];
    assert_eq!(
        Document::admit(BASE, &duplicate).unwrap_err().code,
        "resource-integrity"
    );
    let too_many: Vec<_> = (0..129)
        .map(|_| Resource {
            id: "a",
            revision: &digest,
            bytes: b"x",
        })
        .collect();
    assert_eq!(
        Document::admit(BASE, &too_many).unwrap_err().code,
        "registry-limit"
    );
    assert_eq!(
        d.apply(&vec![b' '; 1048577], &[]).unwrap_err().code,
        "change-format"
    );
}
#[test]
fn source_only_state_stales_requests_and_outer_item_limits_do_not_count_decoded_bindings() {
    let d = base();
    let (mut s, mut l) = parts(&d);
    s = s.replace(
        "End process.",
        "Describe type \"Unused type\" as text. End process.",
    );
    let owner = l["root"].clone();
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"unused_type","kind":"type-alias","scope":owner,"name":"Unused type"}));
    let changed = import(&s, &l);
    assert_eq!(changed.candidate().document(), d.candidate().document());
    assert_ne!(changed.authoring_digest(), d.authoring_digest());
    assert_eq!(
        changed
            .apply(&request(&d, json!({"kind":"format"})), &[])
            .unwrap_err()
            .code,
        "stale-base"
    );
    // Opaque bindings are separately bounded by the ORIGINAL native1MiB contract.
    // Its JSON values are not extra values in the outer document's JSON tree.
    l["annotations"]["example:many-values"] = json!(vec![0; 270000]);
    let many = import(&s, &l);
    assert!(many.bindings().len() < 1048576);
    let reopened = Document::admit(&many.export().unwrap(), &[]).unwrap();
    assert_eq!(reopened.authoring_digest(), many.authoring_digest());
}
#[test]
fn cumulative_rename_output_growth_refuses_instead_of_allocating_partial_source() {
    let d = base();
    let (mut s, mut l) = parts(&d);
    let mut extra = String::new();
    let owner = l["root"].clone();
    for i in 0..500 {
        let name = format!("Repeated {i}");
        extra.push_str(&format!("Let \"{name}\" mean (test \"Same\" using [\"Value\" is (1)]) as truth value with parameters empty.\n"));
        l["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":format!("extra_{i}"),"kind":"expression","scope":owner,"name":name}));
    }
    s = s.replace("End process.", &format!("{extra} End process."));
    let before = import(&s, &l);
    let exact = before.export().unwrap();
    let e = before
        .apply(
            &request(
                &before,
                json!({"kind":"rename","target":"same","name":"X".repeat(4096)}),
            ),
            &[],
        )
        .unwrap_err();
    assert_eq!(e.code, "authoring-size");
    assert_eq!(before.export().unwrap(), exact);
}
