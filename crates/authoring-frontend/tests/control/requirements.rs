// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_semantic_check::derive_invalidation_requirements;

fn missing_case() -> Value {
    let mut b = base();
    cell(&mut b, "first", ROOT, true);
    cell(&mut b, "second", ROOT, true);
    compute(&mut b, "first_site", ROOT, json!({}), json!({"first":true}));
    compute(
        &mut b,
        "second_site",
        ROOT,
        json!({}),
        json!({"second":true}),
    );
    compute(&mut b, "uncalled", ROOT, json!({}), json!({"first":true}));
    flow(&mut b, "first_next", "first_site", "next", "second_site");
    flow(&mut b, "second_next", "second_site", "next", "node_done");
    flow(&mut b, "uncalled_next", "uncalled", "next", "node_done");
    b["data"]["first"]["invalidates"] = json!({"uncalled":true});
    entry(&mut b, "first_site");
    protected(b)
}

#[test]
fn advisory_reports_exact_difference_without_constructing_a_passed_report_or_entry() {
    let b = missing_case();
    let e = envelope(b.clone());
    let before = e.document().clone();
    let r = derive_invalidation_requirements(&e, &resources()).unwrap();
    assert_eq!(e.document(), &before);
    assert_eq!(r.revision(), e.document()["revision"]);
    assert_eq!(r.obligations().len(), 3);
    assert_eq!(r.missing(), r.obligations());
    assert!(r.uncovered_nodes().contains("uncalled"));
    assert!(!r.analyzed_nodes().contains("uncalled"));
    assert!(r.obligations().iter().all(|o| o.site() != "uncalled"));
    assert_eq!(
        r.required_check(),
        "check-invalidation-at-final-definition-revision"
    );
    assert!(!r.remaining().is_empty());
    let strict = check_invalidation(&e, &resources()).unwrap_err();
    assert_eq!(strict.code, "invalidation-coverage");
    assert_eq!(strict.path, "/body/data/first/invalidates");
    assert_eq!(check_concurrency(&e, &resources()).unwrap_err(), strict);
    assert!(!format!("{r:?}").contains(INTEGER_MARKER));
}

#[test]
fn only_fresh_exact_declarations_pass_and_old_advice_does_not_bypass_rechecks() {
    let mut b = missing_case();
    let old = envelope(b.clone());
    let r = derive_invalidation_requirements(&old, &resources()).unwrap();
    for o in r.missing() {
        b["data"][o.cell()]["invalidates"][o.site()] = true.into();
    }
    let e = envelope(b.clone());
    assert_ne!(r.revision(), e.document()["revision"]);
    let strict = check_invalidation(&e, &resources()).unwrap();
    assert_eq!(strict.obligations(), r.obligations());
    let complete = derive_invalidation_requirements(&e, &resources()).unwrap();
    assert!(complete.missing().is_empty());
    assert!(!complete.required_check().is_empty());
    b["data"]["first"]["invalidates"]
        .as_object_mut()
        .unwrap()
        .remove("second_site");
    assert_eq!(
        check_concurrency(&envelope(b), &resources())
            .unwrap_err()
            .code,
        "invalidation-coverage"
    );
}

#[test]
fn strict_error_precedence_is_preserved_while_advisory_refuses_unproved_context() {
    let mut b = missing_case();
    child(&mut b, "child");
    cell(&mut b, "local", "child", true);
    b["data"]["local"]["invalidates"] = json!({"local_site":true});
    compute(
        &mut b,
        "local_site",
        "child",
        json!({}),
        json!({"local":true}),
    );
    b["scopes"]["child"]["entry"] = "local_site".into();
    flow(&mut b, "local_next", "local_site", "next", "child_done");
    child(&mut b, "writer");
    b["scopes"]["writer"]["parent"] = "child".into();
    compute(
        &mut b,
        "mutate",
        "writer",
        json!({"local":"zero"}),
        json!({}),
    );
    b["scopes"]["writer"]["entry"] = "mutate".into();
    flow(&mut b, "mutate_next", "mutate", "next", "writer_done");
    invoke(&mut b, "call", "child", json!({}));
    entry(&mut b, "first_site");
    b["flows"]["first_next"]["target"] = "call".into();
    b["flows"]["flow_call"]["target"] = "second_site".into();
    let e = envelope(protected(b));
    check_protection(&e, &resources()).unwrap();
    let strict = check_invalidation(&e, &resources()).unwrap_err();
    assert_eq!(strict.code, "invalidation-coverage");
    assert_eq!(strict.path, "/body/data/first/invalidates");
    let advisory = derive_invalidation_requirements(&e, &resources()).unwrap_err();
    assert_eq!(advisory.code, "invalidation-context-unproved");
    assert_eq!(advisory.path, "/body/nodes/second_site");
}

#[test]
fn historical_evidence_and_live_ancestor_obligations_match_strict_analysis() {
    let e = envelope(super::historical::closed_call());
    let requirements = derive_invalidation_requirements(&e, &resources()).unwrap();
    let strict = check_invalidation(&e, &resources()).unwrap();
    assert!(requirements.missing().is_empty());
    assert_eq!(requirements.obligations(), strict.obligations());
    assert_eq!(
        requirements.historical_origins(),
        strict.historical_origins()
    );
    assert!(
        requirements
            .obligations()
            .iter()
            .any(|o| o.cell() == "basis")
    );
    assert!(
        requirements
            .historical_origins()
            .iter()
            .any(|o| o.cell() == "local")
    );
}

#[test]
fn invalid_resources_references_and_work_bounds_never_return_partial_requirements() {
    let e = envelope(missing_case());
    assert_eq!(
        derive_invalidation_requirements(&e, &[]).unwrap_err().code,
        "resource-missing"
    );
    let mut b = missing_case();
    b["data"]["first"]["invalidates"]["missing_site"] = true.into();
    let e = envelope(b);
    assert_eq!(
        derive_invalidation_requirements(&e, &resources())
            .unwrap_err()
            .code,
        "structure"
    );
    let e = envelope(protected(chain(1000, 600)));
    assert_eq!(
        derive_invalidation_requirements(&e, &resources())
            .unwrap_err()
            .code,
        "budget"
    );
}

#[test]
fn candidate_advisory_keeps_source_and_ir_immutable_and_strict_failure_visible() {
    let source = std::str::from_utf8(include_bytes!(
        "../../../../docs/authoring/examples/review.choreo"
    ))
    .unwrap()
    .replace("Invalidate [\"Review\"].", "Invalidate empty.");
    let package = Package::parse(
        source.as_bytes(),
        include_bytes!("../../../../docs/authoring/examples/review.bindings.json"),
    )
    .unwrap();
    let c = package.lower(&resources()).unwrap();
    let before = c.document().clone();
    let r = c.invalidation_requirements(&resources()).unwrap();
    assert!(!r.missing().is_empty());
    assert_eq!(before, *c.document());
    assert_eq!(
        c.check_invalidation(&resources()).unwrap_err().code,
        "invalidation-coverage"
    );
}
