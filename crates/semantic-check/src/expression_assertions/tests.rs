// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Actual native type counterexamples, no source19 compiler/evaluator claim.
use super::*;
use serde_json::json;

fn boolean() -> Value {
    json!({"kind":"boolean"})
}
fn text() -> Value {
    json!({"kind":"text"})
}
fn same() -> Value {
    json!({"op":"eq","left":{"op":"param","name":"x"},"right":{"op":"param","name":"x"}})
}
fn context(ty: Type) -> Context {
    Context {
        parameters: BTreeMap::from([("x".into(), ty)]),
        ..Context::default()
    }
}

#[test]
fn otherwise_valid_equality_does_not_erase_declared_argument_type() {
    let mut r = Resources::new(&[]).unwrap();
    let ast = same();
    let expected = text();
    let context = context(Type::Integer);
    assert_eq!(r.expression(&ast, &context).unwrap().ty, Type::Boolean);
    let e = r
        .expression_with_type_assertions(
            &ast,
            &context,
            &[ExpressionTypeAssertion {
                path: "/left",
                expected: &expected,
            }],
        )
        .unwrap_err();
    assert_eq!(
        (e.code, e.path.as_str()),
        ("source-type-mismatch", "/sourceTypes/0")
    );
    // A failed scoped proof must not leave assertions on later unrelated ASTs.
    assert_eq!(r.expression(&ast, &context).unwrap().ty, Type::Boolean);
}
#[test]
fn every_visit_uses_its_real_context_instead_of_a_cached_actual_type() {
    let mut r = Resources::new(&[]).unwrap();
    let ast = same();
    let expected = text();
    r.install_expression_assertions(
        &ast,
        &[ExpressionTypeAssertion {
            path: "/left",
            expected: &expected,
        }],
    )
    .unwrap();
    r.expression(&ast, &context(Type::Text)).unwrap();
    assert_eq!(
        r.expression(&ast, &context(Type::Integer))
            .unwrap_err()
            .code,
        "source-type-mismatch"
    );
}
#[test]
fn complete_structural_type_is_required_even_when_the_body_uses_less() {
    let mut r = Resources::new(&[]).unwrap();
    let ast = same();
    let actual = Type::Record(BTreeMap::from([
        ("first".into(), Type::Text),
        ("second".into(), Type::Text),
    ]));
    let expected = json!({"kind":"record","fields":{"first":{"kind":"text"}}});
    assert_eq!(
        r.expression(&ast, &context(actual.clone())).unwrap().ty,
        Type::Boolean
    );
    assert_eq!(
        r.expression_with_type_assertions(
            &ast,
            &context(actual),
            &[ExpressionTypeAssertion {
                path: "/left",
                expected: &expected
            }]
        )
        .unwrap_err()
        .code,
        "source-type-mismatch"
    );
}
#[test]
fn nominal_invariants_and_pins_are_not_replaced_by_equal_representations() {
    use choreoform_ir_probe_core::{Resource, digest};
    let raw = include_bytes!("../../../../docs/benchmarks/rp03/types.json");
    let revision = digest(raw);
    let id = "urn:choreoform:benchmarks:rp03:types:0.1.0";
    let supplied = [Resource {
        id,
        revision: &revision,
        bytes: raw,
    }];
    let mut r = Resources::new(&supplied).unwrap();
    let named = |name| json!({"kind":"named","contract":{"id":id,"revision":revision},"name":name});
    let a = named("Reservation A evidence");
    let b = named("Reservation B evidence");
    let ty = r.type_descriptor(&b).unwrap();
    let ast = same();
    assert_eq!(
        r.expression(&ast, &context(ty.clone())).unwrap().ty,
        Type::Boolean
    );
    assert_eq!(
        r.expression_with_type_assertions(
            &ast,
            &context(ty),
            &[ExpressionTypeAssertion {
                path: "/left",
                expected: &a
            }]
        )
        .unwrap_err()
        .code,
        "source-type-mismatch"
    );
}
#[test]
fn opaque_literal_data_and_missing_sites_fail_without_echoing_paths() {
    let mut r = Resources::new(&[]).unwrap();
    let expected = text();
    let ast = json!({"op":"literal","type":{"kind":"record","fields":{"op":{"kind":"text"},"name":{"kind":"text"}}},"value":{"op":"param","name":"SECRET"}});
    let e = r
        .expression_with_type_assertions(
            &ast,
            &Context::default(),
            &[ExpressionTypeAssertion {
                path: "/value",
                expected: &expected,
            }],
        )
        .unwrap_err();
    assert_eq!(
        (e.code, e.path.as_str()),
        ("source-type-unvisited", "/sourceTypes/0")
    );
    let e = r
        .expression_with_type_assertions(
            &ast,
            &Context::default(),
            &[ExpressionTypeAssertion {
                path: "/SECRET-MUST-NOT-LEAK",
                expected: &expected,
            }],
        )
        .unwrap_err();
    assert_eq!(
        (e.code, e.path.as_str()),
        ("source-type-site", "/sourceTypes/0")
    );
}
#[test]
fn overlapping_assertions_all_hold_and_result_sites_can_cover_zero_inputs() {
    let mut r = Resources::new(&[]).unwrap();
    let ast = json!({"op":"literal","type":{"kind":"boolean"},"value":true});
    let b = boolean();
    let t = text();
    r.expression_with_type_assertions(
        &ast,
        &Context::default(),
        &[ExpressionTypeAssertion {
            path: "",
            expected: &b,
        }],
    )
    .unwrap();
    let e = r
        .expression_with_type_assertions(
            &ast,
            &Context::default(),
            &[
                ExpressionTypeAssertion {
                    path: "",
                    expected: &b,
                },
                ExpressionTypeAssertion {
                    path: "",
                    expected: &t,
                },
            ],
        )
        .unwrap_err();
    assert_eq!(
        (e.code, e.path.as_str()),
        ("source-type-mismatch", "/sourceTypes/1")
    );
}
#[test]
fn metadata_count_bytes_depth_and_total_work_fail_before_unbounded_growth() {
    let ast = json!({"op":"literal","type":{"kind":"boolean"},"value":true});
    let b = boolean();
    let a = ExpressionTypeAssertion {
        path: "",
        expected: &b,
    };
    assert_eq!(
        Resources::new(&[])
            .unwrap()
            .expression_with_type_assertions(&ast, &Context::default(), &vec![a; MAX_ITEMS + 1])
            .unwrap_err()
            .code,
        "source-type-budget"
    );
    let path = "x".repeat(transport::MAX_BYTES + 1);
    assert_eq!(
        Resources::new(&[])
            .unwrap()
            .expression_with_type_assertions(
                &ast,
                &Context::default(),
                &[ExpressionTypeAssertion {
                    path: &path,
                    expected: &b
                }]
            )
            .unwrap_err()
            .code,
        "source-type-budget"
    );
    let mut deep = b.clone();
    for _ in 0..70 {
        deep = json!({"kind":"option","element":deep});
    }
    assert!(
        Resources::new(&[])
            .unwrap()
            .expression_with_type_assertions(
                &ast,
                &Context::default(),
                &[ExpressionTypeAssertion {
                    path: "",
                    expected: &deep
                }]
            )
            .is_err()
    );
    let mut r = Resources::new(&[]).unwrap();
    let mut failed = false;
    for _ in 0..1_000_001 {
        if let Err(e) = r.expression_with_type_assertions(&ast, &Context::default(), &[a]) {
            assert_eq!(e.code, "budget");
            failed = true;
            break;
        }
    }
    assert!(
        failed,
        "the registry budget must not reset between definitions"
    );
}

fn order() -> (
    choreoform_ir_probe_core::DefinitionEnvelope,
    Vec<(String, String, Vec<u8>)>,
) {
    use choreoform_ir_probe_core::DefinitionEnvelope;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let base = root.join("docs/benchmarks/rp03");
    let e = DefinitionEnvelope::decode(&std::fs::read(base.join("candidate.ir.json")).unwrap())
        .unwrap();
    let mut resources = Vec::new();
    for (id, revision) in [
        SEMANTICS, CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, CORE_V6, CORE_V7, CORE_V8, CORE_V9,
        CORE_V10, CORE_V11, CORE_V12,
    ] {
        resources.push((
            id.into(),
            revision.into(),
            std::fs::read(root.join(format!(
                "docs/ir/contracts/{}.txt",
                revision.replace(':', "-")
            )))
            .unwrap(),
        ));
    }
    let m: Value =
        serde_json::from_slice(&std::fs::read(base.join("manifest.json")).unwrap()).unwrap();
    for r in m["resources"].as_array().unwrap() {
        let id = r["id"].as_str().unwrap();
        let rev = r["revision"].as_str().unwrap();
        if !resources.iter().any(|(i, v, _)| i == id && v == rev) {
            resources.push((
                id.into(),
                rev.into(),
                std::fs::read(root.join(r["path"].as_str().unwrap())).unwrap(),
            ));
        }
    }
    (e, resources)
}
fn supplied(rows: &[(String, String, Vec<u8>)]) -> Vec<choreoform_ir_probe_core::Resource<'_>> {
    rows.iter()
        .map(|(id, revision, bytes)| choreoform_ir_probe_core::Resource {
            id,
            revision,
            bytes,
        })
        .collect()
}
#[test]
fn complete_actual_order_uses_native_policy_context_and_keeps_ordinary_reports_exact() {
    let (e, owned) = order();
    let resources = supplied(&owned);
    let b = boolean();
    let t = text();
    let path = "/body/policies/reserve_a_work/body/completion/then";
    let old = check(&e, &resources).unwrap();
    let typed = check_with_expression_type_assertions(
        &e,
        &resources,
        &[ExpressionTypeAssertion { path, expected: &b }],
    )
    .unwrap();
    assert_eq!(format!("{old:?}"), format!("{typed:?}"));
    let bad = check_with_expression_type_assertions(
        &e,
        &resources,
        &[ExpressionTypeAssertion { path, expected: &t }],
    )
    .unwrap_err();
    assert_eq!(
        (bad.code, bad.path.as_str()),
        ("source-type-mismatch", "/sourceTypes/0")
    );
}
#[test]
fn complete_derived_order_rechecks_the_same_policy_in_both_real_result_contexts() {
    use choreoform_ir_probe_core::DefinitionEnvelope;
    let (e, owned) = order();
    let resources = supplied(&owned);
    let mut b = e.document()["body"].clone();
    let p = &mut b["policies"]["reserve_a_work"];
    p["scope"] = "order".into();
    p["body"]["reads"] = json!({});
    p["body"]["completion"] = json!({"op":"eq","left":{"op":"fact","name":"result"},"right":{"op":"fact","name":"result"}});
    b["nodes"]["reserve_b_activity"]["policy"] = "reserve_a_work".into();
    b["policies"]
        .as_object_mut()
        .unwrap()
        .remove("reserve_b_work");
    let expected = b["capabilities"]["reserve_a_capability"]["output"]["body"].clone();
    let changed = DefinitionEnvelope::from_parts_versioned("0.4.0", b, json!({})).unwrap();
    // Ordinary equality is valid for each variant separately. The additional
    // source contract requires A's exact nominal variant at both uses and fails B.
    check(&changed, &resources).unwrap();
    let bad = check_with_expression_type_assertions(
        &changed,
        &resources,
        &[ExpressionTypeAssertion {
            path: "/body/policies/reserve_a_work/body/completion/left",
            expected: &expected,
        }],
    )
    .unwrap_err();
    assert_eq!(
        (bad.code, bad.path.as_str()),
        ("source-type-mismatch", "/sourceTypes/0")
    );
}

#[test]
fn ordinary_expression_source_diagnostics_keep_the_private_assertion_index() {
    let (e, owned) = order();
    let resources = supplied(&owned);
    let t = text();
    let bad = check_with_expression_type_assertions(
        &e,
        &resources,
        &[ExpressionTypeAssertion {
            path: "/body/expressions/reserve_a_input/body",
            expected: &t,
        }],
    )
    .unwrap_err();
    assert_eq!(
        (bad.code, bad.path.as_str()),
        ("source-type-mismatch", "/sourceTypes/0")
    );
}
#[test]
fn unused_work_placeholder_context_does_not_supply_a_source_call_proof() {
    use choreoform_ir_probe_core::DefinitionEnvelope;
    let (e, owned) = order();
    let resources = supplied(&owned);
    let mut b = e.document()["body"].clone();
    let mut policy = b["policies"]["reserve_a_work"].clone();
    policy["scope"] = "order".into();
    policy["body"]["reads"] = json!({});
    policy["body"]["completion"] = json!({"op":"literal","type":{"kind":"boolean"},"value":true});
    b["policies"]["unused_source_test"] = policy;
    let changed = DefinitionEnvelope::from_parts_versioned("0.4.0", b, json!({})).unwrap();
    check(&changed, &resources).unwrap();
    let expected = boolean();
    let bad = check_with_expression_type_assertions(
        &changed,
        &resources,
        &[ExpressionTypeAssertion {
            path: "/body/policies/unused_source_test/body/completion",
            expected: &expected,
        }],
    )
    .unwrap_err();
    assert_eq!(
        (bad.code, bad.path.as_str()),
        ("source-type-context", "/sourceTypes/0")
    );
}
