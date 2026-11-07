// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

fn case(f: &Fixture, id: &str) -> Value {
    f.catalogue["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == id)
        .unwrap()
        .clone()
}

#[test]
fn full_original_ast_is_checked_before_unselected_branches_or_binding_lookup() {
    let mut f = Fixture::new();
    let c = case(&f, "current_approvals");
    f.candidate["body"]["expressions"]["may_pay"]["body"] = json!({"op":"if","condition":{"op":"literal","type":{"kind":"boolean"},"value":true},"then":{"op":"literal","type":{"kind":"boolean"},"value":true},"else":{"op":"read","cell":"undeclared_cell"}});
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    let result = evaluate_case(&f, &c, &mut r).unwrap_err();
    assert_eq!(result.0, "static");
    assert_eq!(result.1.code, "cell-reference");
}

#[test]
fn literal_operation_like_data_is_opaque_and_short_circuit_faults_are_preserved() {
    let f = Fixture::new();
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    let context = Context::default();
    let empty_case = json!({"cells":{},"facts":{}});
    let mut bindings = bindings::Bindings::new(&mut r, &context, &json!({}), &empty_case).unwrap();
    let ast = json!({"op":"literal","type":{"kind":"record","fields":{"op":{"kind":"text"},"cell":{"kind":"text"}}},"value":{"op":"read","cell":"SECRET-LITERAL-MARKER"}});
    r.expression(&ast, &context).unwrap();
    let bound = bindings.rebind(&ast, &mut r, 0).unwrap();
    assert_eq!(bound, ast);
    bindings.complete().unwrap();
    let value = r
        .evaluate(&bound, &bindings.values, &bindings.context, 0)
        .unwrap();
    assert_eq!(value, ast["value"]);
    for id in [
        "absent_manager_short_circuits",
        "absent_finance_short_circuits",
        "finance_no_manager",
    ] {
        let c = case(&f, id);
        assert_eq!(outcome(evaluate_case(&f, &c, &mut r)), c["expect"]);
    }
}

#[test]
fn missing_unknown_unused_and_ill_typed_bindings_do_not_supply_guessed_values() {
    let f = Fixture::new();
    for mutation in ["missing", "unknown", "unused", "wrong_type"] {
        let mut c = case(&f, "authority_assigned_role");
        match mutation {
            "missing" => c["facts"] = json!({}),
            "unknown" => c["facts"]["not_a_fact"] = "manager_roles".into(),
            "unused" => c["facts"]["principal"] = "manager".into(),
            _ => c["facts"]["roles"] = "manager".into(),
        }
        let supplied = f.supplied();
        let mut r = Resources::new(&supplied).unwrap();
        let result = evaluate_case(&f, &c, &mut r).unwrap_err();
        assert_eq!(result.0, "binding");
        assert!(!format!("{result:?}").contains("fictional-manager"));
        assert_eq!(
            result.1.code,
            match mutation {
                "missing" | "unknown" => "snapshot-binding",
                "unused" => "snapshot-unused-binding",
                _ => "collection",
            }
        );
    }
}

#[test]
fn catalogue_revision_selector_and_resource_boundaries_fail_closed() {
    let mut f = Fixture::new();
    f.catalogue["candidateRevision"] = "sha256:wrong".into();
    assert_eq!(
        f.validate_catalogue().unwrap_err().code,
        "snapshot-revision"
    );
    f.catalogue["candidateRevision"] = f.candidate["revision"].clone();
    f.catalogue["extra"] = true.into();
    assert_eq!(f.validate_catalogue().unwrap_err().code, "fields");
    f.catalogue.as_object_mut().unwrap().remove("extra");
    for mutation in ["id", "field", "site"] {
        let mut c = case(&f, "manager_approved");
        c["subject"][mutation] = "wrong-subject".into();
        let supplied = f.supplied();
        let mut r = Resources::new(&supplied).unwrap();
        assert_eq!(
            outcome(evaluate_case(&f, &c, &mut r)),
            json!({"phase":"catalogue","code":"snapshot-subject"})
        );
    }
    let c = case(&f, "manager_approved");
    let mut r = Resources::new(&[]).unwrap();
    assert!(evaluate_case(&f, &c, &mut r).is_err());
    let mut supplied = f.supplied();
    supplied
        .iter_mut()
        .find(|r| r.id == "urn:choreoform:benchmarks:rp01:types:0.1.0")
        .unwrap()
        .bytes = b"{}";
    let mut r = Resources::new(&supplied).unwrap();
    assert!(matches!(
        evaluate_case(&f, &c, &mut r),
        Err((
            _,
            Diagnostic {
                code: "resource-digest",
                ..
            }
        ))
    ));
}

#[test]
fn reification_and_value_copying_obey_the_existing_total_work_and_depth_bounds() {
    let f = Fixture::new();
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    let mut bindings = bindings::Bindings::new(
        &mut r,
        &Context::default(),
        &json!({}),
        &json!({"cells":{},"facts":{}}),
    )
    .unwrap();
    let ast = json!({"op":"literal","type":{"kind":"text"},"value":"bounded copy"});
    r.budget.work = 999_999;
    assert_eq!(bindings.rebind(&ast, &mut r, 0).unwrap_err().code, "budget");
    r.budget.work = 0;
    assert_eq!(
        bindings
            .rebind(&ast, &mut r, MAX_DEPTH + 1)
            .unwrap_err()
            .code,
        "budget"
    );
    r.budget.work = 999_990;
    let mut context = Context::default();
    context.facts.insert("principal".into(), Type::Text);
    assert!(
        matches!(bindings::Bindings::new(&mut r, &context, &json!({"name":"fictional-budget-marker"}), &json!({"cells":{},"facts":{"principal":"name"}})), Err(e) if e.code == "budget")
    );
}
