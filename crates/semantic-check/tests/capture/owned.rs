// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_semantic_check::{check_concurrency, check_timing};

fn branch_fixture() -> Fixture {
    let mut f = actual_rp01_capture();
    let scope = "payment_recovery";
    let mut guard = f.b["expressions"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap()
        .clone();
    guard["scope"] = scope.into();
    guard["dialect"] = "core".into();
    guard["resultType"] = wrap(json!({"kind":"boolean"}));
    guard["parameters"] = json!({});
    guard["reads"] = json!({"late_payment_fact":true});
    guard["body"] = json!({"op":"eq","left":{"op":"tag","value":{"op":"read","cell":"late_payment_fact"}},"right":{"op":"literal","type":{"kind":"text"},"value":"confirmed"}});
    f.b["expressions"]["local_guard"] = guard.clone();
    let mut expr = guard;
    expr["resultType"] = wrap(json!({"kind":"text"}));
    expr["body"] = json!({"op":"tag","value":{"op":"read","cell":"late_payment_fact"}});
    f.b["expressions"]["local_tag"] = expr;
    let mut cell = f.b["data"]["late_payment_fact"].clone();
    cell["type"] = wrap(json!({"kind":"text"}));
    f.b["data"]["local_tag_cell"] = cell;
    f.b["data"]["late_payment_fact"]["invalidates"] = json!({"local_choose":true});
    f.b["nodes"]["local_choose"] = json!({"kind":"decision","scope":scope,"outcomes":{"yes":true,"no":true},"reads":{"late_payment_fact":true},"writes":{},"guards":{"yes":"local_guard"},"default":"no"});
    f.b["nodes"]["local_compute"] = json!({"kind":"compute","scope":scope,"outcomes":{"next":true},"reads":{"late_payment_fact":true},"writes":{"local_tag_cell":true},"assignments":{"local_tag_cell":"local_tag"}});
    let old = f.b["flows"]
        .as_object()
        .unwrap()
        .iter()
        .find(|(_, v)| v["source"] == "payment_observe")
        .map(|(k, v)| (k.clone(), v["target"].clone()))
        .unwrap();
    f.b["flows"][&old.0]["target"] = "local_choose".into();
    for (id, source, outcome, target) in [
        ("local_yes", "local_choose", "yes", json!("local_compute")),
        ("local_no", "local_choose", "no", old.1.clone()),
        ("local_next", "local_compute", "next", old.1),
    ] {
        f.b["flows"][id] = json!({"source":source,"outcome":outcome,"target":target});
    }
    f
}

#[test]
fn owned_capture_read_branch_compute_has_separate_conditional_coverage_and_obligations() {
    let f = branch_fixture();
    let e = f.envelope();
    let resources = f.supplied();
    let ordinary = check_invalidation(&e, &resources).unwrap();
    assert!(ordinary.uncovered_nodes().contains("local_choose"));
    let concurrency = check_concurrency(&e, &resources).unwrap();
    let owned = concurrency.owned_local().unwrap();
    assert_eq!(owned.revision(), e.document()["revision"]);
    assert_eq!(
        owned.checked_scopes(),
        &std::collections::BTreeSet::from(["payment_recovery".into()])
    );
    assert!(owned.checked_nodes().contains("local_choose"));
    assert!(owned.checked_nodes().contains("local_compute"));
    assert!(
        owned
            .obligations()
            .iter()
            .any(|o| o.site() == "local_choose" && o.cell() == "late_payment_fact")
    );
    assert!(owned.accesses().iter().any(|a| a.site() == "local_compute"
        && a.cell() == "local_tag_cell"
        && a.operation() == "write"));
    assert!(owned.transfers() > 0);
    assert!(!owned.remaining().is_empty());
    assert!(
        concurrency
            .invalidation()
            .uncovered_nodes()
            .contains("local_choose")
    );
    assert!(
        concurrency
            .isolated_followups()
            .iter()
            .all(|p| p.followup() != "payment_recovery")
    );
    check_timing(&e, &resources).unwrap();
    check_reconciliation_availability(&e, &resources).unwrap();
}

#[test]
fn strict_conditional_invalidation_is_not_conferred_by_ordinary_or_advisory_passes() {
    let mut f = branch_fixture();
    f.b["data"]["late_payment_fact"]["invalidates"] = json!({});
    let e = f.envelope();
    let resources = f.supplied();
    check_invalidation(&e, &resources).unwrap();
    let error = check_concurrency(&e, &resources).unwrap_err();
    assert_eq!(error.code, "owned-local-invalidation-coverage");
    assert_eq!(error.path, "/body/data/late_payment_fact/invalidates");
}

#[test]
fn zero_inherited_entry_rejects_read_before_capture_and_ancestor_dependencies() {
    let mut f = branch_fixture();
    f.b["nodes"]["payment_observe"]["reads"]["late_payment_fact"] = true.into();
    check(&f.envelope(), &f.supplied()).unwrap();
    assert_eq!(
        check_concurrency(&f.envelope(), &f.supplied())
            .unwrap_err()
            .code,
        "owned-local-availability-unproved"
    );
    let mut f = branch_fixture();
    f.b["nodes"]["payment_observe"]["reads"]["current_claim"] = true.into();
    check(&f.envelope(), &f.supplied()).unwrap();
    assert_eq!(
        check_concurrency(&f.envelope(), &f.supplied())
            .unwrap_err()
            .code,
        "scope-lifetime-unproved"
    );
}

#[test]
fn source_and_control_flow_cannot_launder_a_captured_value_to_a_wider_local_cell() {
    let mut f = branch_fixture();
    f.b["data"]["local_tag_cell"]["protection"]["purposes"]["unrelated"] = true.into();
    // Make the computation a literal; only its inherited captured control remains.
    f.b["expressions"]["local_tag"]["body"] =
        json!({"op":"literal","type":{"kind":"text"},"value":"published"});
    f.b["expressions"]["local_tag"]["reads"] = json!({});
    f.b["nodes"]["local_compute"]["reads"] = json!({});
    check_invalidation(&f.envelope(), &f.supplied()).unwrap();
    let error = check_concurrency(&f.envelope(), &f.supplied()).unwrap_err();
    assert_eq!(error.code, "control-information-flow");
    assert_eq!(error.path, "/body/nodes/local_compute");
}

#[test]
fn unvisited_foreign_write_nested_scope_and_noncapture_storage_remain_unproved() {
    let mut f = branch_fixture();
    let mut literal = f.b["expressions"]["local_tag"].clone();
    literal["reads"] = json!({});
    literal["body"] = json!({"op":"literal","type":{"kind":"text"},"value":"changed"});
    f.b["expressions"]["foreign_literal"] = literal;
    let mut n = f.b["nodes"]["local_compute"].clone();
    n["reads"] = json!({});
    n["writes"] = json!({"disposition":true});
    n["assignments"] = json!({"disposition":"foreign_literal"});
    f.b["nodes"]["unvisited_writer"] = n;
    f.b["flows"]["unvisited_write_flow"] =
        json!({"source":"unvisited_writer","outcome":"next","target":"payment_recovered"});
    check(&f.envelope(), &f.supplied()).unwrap();
    assert_eq!(
        check_concurrency(&f.envelope(), &f.supplied())
            .unwrap_err()
            .code,
        "scope-lifetime-unproved"
    );
    let mut f = branch_fixture();
    let mut s = f.b["scopes"]["remaining"].clone();
    s["parent"] = "payment_recovery".into();
    s["entry"] = "nested_finish".into();
    f.b["scopes"]["nested"] = s;
    let mut n = f.b["nodes"]["remaining_done"].clone();
    n["scope"] = "nested".into();
    f.b["nodes"]["nested_finish"] = n;
    check(&f.envelope(), &f.supplied()).unwrap();
    assert_eq!(
        check_concurrency(&f.envelope(), &f.supplied())
            .unwrap_err()
            .code,
        "scope-lifetime-unproved"
    );
    let mut f = actual_rp01_capture();
    f.b["nodes"]["payment_observe"]["capture"] = Value::Null;
    f.b["nodes"]["payment_observe"]["writes"] = json!({});
    check(&f.envelope(), &f.supplied()).unwrap();
    assert_eq!(
        check_concurrency(&f.envelope(), &f.supplied())
            .unwrap_err()
            .code,
        "scope-lifetime-unproved"
    );
}

#[test]
fn an_independent_unvisited_creator_of_the_followup_is_not_an_owned_entry() {
    let mut f = actual_rp01_capture();
    f.b["nodes"]["unvisited_creator"] = json!({"kind":"invoke","scope":"payment_service","outcomes":{"complete":true},"reads":{},"writes":{},"body":"payment_recovery","inputs":{},"outputs":{}});
    f.b["flows"]["unvisited_create_flow"] =
        json!({"source":"unvisited_creator","outcome":"complete","target":"payment_done"});
    f.b["nodes"]["unvisited_creator"]["reads"] =
        f.b["nodes"]["payment_request_step"]["reads"].clone();
    check(&f.envelope(), &f.supplied()).unwrap();
    assert_eq!(
        check_concurrency(&f.envelope(), &f.supplied())
            .unwrap_err()
            .code,
        "scope-lifetime-unproved"
    );
}

#[test]
fn repeated_covered_creators_share_only_immutable_template_metadata() {
    let mut f = actual_rp01_capture();
    f.b["nodes"]["run_payment_again"] = f.b["nodes"]["run_payment"].clone();
    let (flow, old_target) = f.b["flows"]
        .as_object()
        .unwrap()
        .iter()
        .find(|(_, v)| v["source"] == "run_payment")
        .map(|(k, v)| (k.clone(), v["target"].clone()))
        .unwrap();
    f.b["flows"][&flow]["target"] = "run_payment_again".into();
    f.b["flows"]["again_complete"] =
        json!({"source":"run_payment_again","outcome":"complete","target":old_target});
    let advice =
        choreoform_semantic_check::derive_invalidation_requirements(&f.envelope(), &f.supplied())
            .unwrap();
    let missing: Vec<_> = advice
        .missing()
        .iter()
        .map(|o| (o.cell().to_owned(), o.site().to_owned()))
        .collect();
    for (cell, site) in missing {
        f.b["data"][cell]["invalidates"][site] = true.into();
    }
    let r = check_concurrency(&f.envelope(), &f.supplied()).unwrap();
    let owned = r.owned_local().unwrap();
    let copies: Vec<_> = owned
        .followups()
        .iter()
        .filter(|p| p.body() == "payment_service")
        .collect();
    assert_eq!(copies.len(), 2);
    assert!(std::ptr::eq(
        copies[0].local_cells(),
        copies[1].local_cells()
    ));
    assert_ne!(copies[0].creator(), copies[1].creator());
    assert!(
        copies[0]
            .host_requirements()
            .contains(&"zero-inherited-values-and-no-sharing-between-creator-occurrences")
    );
}

#[test]
fn an_unvisited_activity_cannot_hide_effects_inside_owned_local_evidence() {
    let mut f = actual_rp01_capture();
    let mut n = f.b["nodes"]["payment_request_step"].clone();
    n["scope"] = "payment_recovery".into();
    let mut policy = f.b["policies"]["payment_work"].clone();
    policy["body"]["completion"] = boolean(true);
    f.b["policies"]["unvisited_work"] = policy;
    n["policy"] = "unvisited_work".into();
    n["result"] = Value::Null;
    n["writes"] = json!({});
    f.b["nodes"]["unvisited_activity"] = n;
    for outcome in ["paid", "problem"] {
        f.b["flows"][format!("extra_{outcome}")] =
            json!({"source":"unvisited_activity","outcome":outcome,"target":"payment_recovered"});
    }
    check(&f.envelope(), &f.supplied()).unwrap();
    assert_eq!(
        check_concurrency(&f.envelope(), &f.supplied())
            .unwrap_err()
            .code,
        "scope-lifetime-unproved"
    );
}

#[test]
fn local_initializers_are_fresh_and_cannot_inherit_an_ancestor_value() {
    let mut f = branch_fixture();
    let mut initial = f.b["expressions"]["local_tag"].clone();
    initial["reads"] = json!({});
    initial["body"] = json!({"op":"literal","type":{"kind":"text"},"value":"fresh"});
    f.b["expressions"]["fresh_initial"] = initial;
    f.b["data"]["local_tag_cell"]["initial"] = "fresh_initial".into();
    let r = check_concurrency(&f.envelope(), &f.supplied()).unwrap();
    assert!(
        r.owned_local()
            .unwrap()
            .accesses()
            .iter()
            .any(|a| a.site() == "local_tag_cell" && a.operation() == "write")
    );
    f.b["expressions"]["fresh_initial"]["body"] = json!({"op":"read","cell":"disposition"});
    f.b["expressions"]["fresh_initial"]["reads"] = json!({"disposition":true});
    f.b["nodes"]["run_payment"]["reads"]["disposition"] = true.into();
    let advice =
        choreoform_semantic_check::derive_invalidation_requirements(&f.envelope(), &f.supplied())
            .unwrap();
    let missing: Vec<_> = advice
        .missing()
        .iter()
        .map(|o| (o.cell().to_owned(), o.site().to_owned()))
        .collect();
    for (cell, site) in missing {
        f.b["data"][cell]["invalidates"][site] = true.into();
    }
    check_invalidation(&f.envelope(), &f.supplied()).unwrap();
    assert_eq!(
        check_concurrency(&f.envelope(), &f.supplied())
            .unwrap_err()
            .code,
        "owned-local-availability-unproved"
    );
}
