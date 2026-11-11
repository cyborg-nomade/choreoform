// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Complete bounded baseline for closed all-join versus still-live branch evidence.
use super::*;

fn joined_local(all: bool) -> Value {
    let mut b = parallel();
    cell(&mut b, "local", "child_a", true);
    child_compute(&mut b, "child_a", json!({}), json!({"local":true}));
    b["data"]["local"]["invalidates"] = json!({"child_a_work":true});
    b["nodes"]["barrier"]["predicate"] =
        json!({"op":if all {"all"} else {"any"},"outcomes":{"complete":true}});
    compute(&mut b, "after", ROOT, json!({}), json!({}));
    b["flows"]["barrier_done"]["target"] = "after".into();
    flow(&mut b, "after_done", "after", "next", "node_done");
    protected(b)
}

#[test]
fn fixed_all_join_proves_history_while_partial_any_retains_live_private_context() {
    check_invalidation(&envelope(super::historical::closed_call()), &resources()).unwrap();
    for all in [false, true] {
        let e = envelope(joined_local(all));
        check(&e, &resources()).unwrap();
        check_control(&e, &resources()).unwrap();
        check_protection(&e, &resources()).unwrap();
        if all {
            let report = check_invalidation(&e, &resources()).unwrap();
            let row = report
                .historical_origins()
                .iter()
                .find(|h| h.site() == "after" && h.cell() == "local")
                .unwrap();
            assert_eq!(
                (row.boundary(), row.scope(), row.closure()),
                ("barrier", "child_a", "policy_close")
            );
            assert_eq!(
                row.condition(),
                "ordinary-completion-with-no-declared-descendant-writer"
            );
            assert!(
                !report
                    .obligations()
                    .iter()
                    .any(|o| o.site() == "after" && o.cell() == "local")
            );
            let advice =
                choreoform_semantic_check::derive_invalidation_requirements(&e, &resources())
                    .unwrap();
            assert!(advice.missing().is_empty());
            assert_eq!(advice.historical_origins(), report.historical_origins());
        } else {
            let error = check_invalidation(&e, &resources()).unwrap_err();
            assert_eq!(error.code, "invalidation-context-unproved");
            assert_eq!(error.path, "/body/nodes/after");
            let error =
                choreoform_semantic_check::derive_invalidation_requirements(&e, &resources())
                    .unwrap_err();
            assert_eq!(error.code, "invalidation-context-unproved");
            assert_eq!(error.path, "/body/nodes/after");
        }
    }
}

#[test]
fn unvisited_descendant_writer_remains_live_even_for_all_join() {
    let mut b = joined_local(true);
    child(&mut b, "writer");
    b["scopes"]["writer"]["parent"] = "child_a".into();
    compute(
        &mut b,
        "mutate",
        "writer",
        json!({"local":"zero"}),
        json!({}),
    );
    b["scopes"]["writer"]["entry"] = "mutate".into();
    flow(&mut b, "mutate_done", "mutate", "next", "writer_done");
    let e = envelope(b);
    check_protection(&e, &resources()).unwrap();
    let error = check_invalidation(&e, &resources()).unwrap_err();
    assert_eq!(error.code, "invalidation-context-unproved");
}

#[test]
fn thresholds_composites_and_an_inherited_live_occurrence_are_not_upgraded() {
    for predicate in [
        json!({"op":"atLeast","count":2,"outcomes":{"complete":true}}),
        json!({"op":"and","terms":{"all":{"op":"all","outcomes":{"complete":true}}}}),
        json!({"op":"or","terms":{"all":{"op":"all","outcomes":{"complete":true}}}}),
    ] {
        let mut b = joined_local(true);
        b["nodes"]["barrier"]["predicate"] = predicate;
        invalidation_refuse(b, "invalidation-context-unproved");
    }
    let mut b = joined_local(false);
    b["nodes"]["again"] = b["nodes"]["fork"].clone();
    b["nodes"]["again"]["join"] = "again_join".into();
    b["nodes"]["again_join"] = b["nodes"]["barrier"].clone();
    b["nodes"]["again_join"]["source"] = "again".into();
    b["nodes"]["again_join"]["predicate"] = json!({"op":"all","outcomes":{"complete":true}});
    b["flows"]["barrier_done"]["target"] = "again".into();
    flow(&mut b, "again_done", "again_join", "next", "after");
    invalidation_refuse(b, "invalidation-context-unproved");
}

#[test]
fn all_join_keeps_mutable_ancestor_coverage_and_original_protection_labels() {
    let mut b = joined_local(true);
    cell(&mut b, "basis", ROOT, true);
    b["nodes"]["child_a_work"]["reads"]["basis"] = true.into();
    b["data"]["basis"]["invalidates"] = json!({"child_a_work":true});
    b = protected(b);
    let e = envelope(b.clone());
    assert_eq!(
        check_invalidation(&e, &resources()).unwrap_err().code,
        "invalidation-coverage"
    );
    b["data"]["basis"]["invalidates"]["after"] = true.into();
    let report = check_invalidation(&envelope(b), &resources()).unwrap();
    assert!(
        report
            .obligations()
            .iter()
            .any(|o| o.site() == "after" && o.cell() == "basis")
    );

    let mut b = joined_local(true);
    b["actors"]["other"] = json!({"scope":ROOT,"requirement":"policy_actor"});
    cell(&mut b, "public", ROOT, false);
    b["data"]["public"]["protection"]["participants"] = json!({"actor_reviewer":true,"other":true});
    compute(&mut b, "after", ROOT, json!({"public":"zero"}), json!({}));
    let e = envelope(b);
    let protection = check_protection(&e, &resources()).unwrap_err();
    assert_eq!(protection.code, "control-information-flow");
    assert_eq!(
        check_invalidation(&e, &resources()).unwrap_err(),
        protection
    );
}

#[test]
fn an_outer_all_join_does_not_freeze_unproved_descendant_origins() {
    let mut b = joined_local(true);
    b["nodes"].as_object_mut().unwrap().remove("child_a_work");
    b["flows"].as_object_mut().unwrap().remove("child_a_next");
    b["data"].as_object_mut().unwrap().remove("local");
    for name in ["inner_a", "inner_b"] {
        child(&mut b, name);
        b["scopes"][name]["parent"] = "child_a".into();
    }
    cell(&mut b, "deep", "inner_a", true);
    child_compute(&mut b, "inner_a", json!({}), json!({"deep":true}));
    b["data"]["deep"]["invalidates"] = json!({"inner_a_work":true});
    b["nodes"]["inner_fork"] = json!({"kind":"split","scope":"child_a","children":{"a":"inner_a","b":"inner_b"},"join":"inner_join","reads":{},"writes":{},"outcomes":{}});
    b["nodes"]["inner_join"] = json!({"kind":"join","scope":"child_a","source":"inner_fork","predicate":{"op":"any","outcomes":{"complete":true}},"remaining":"policy_settle","reads":{},"writes":{},"outcomes":{"next":true}});
    b["scopes"]["child_a"]["entry"] = "inner_fork".into();
    flow(&mut b, "inner_done", "inner_join", "next", "child_a_done");
    invalidation_refuse(b, "invalidation-context-unproved");
}

#[test]
fn wide_all_join_history_indexes_members_and_visits_origins_once() {
    let mut b = base();
    let mut children = serde_json::Map::new();
    for index in 0..512 {
        let name = format!("c{index:03}");
        child(&mut b, &name);
        let mut reads = serde_json::Map::new();
        for item in 0..4 {
            let cell_id = format!("{name}_v{item}");
            cell(&mut b, &cell_id, &name, true);
            b["data"][&cell_id]["invalidates"] = json!({format!("{name}_work"):true});
            reads.insert(cell_id, true.into());
        }
        child_compute(&mut b, &name, json!({}), Value::Object(reads));
        children.insert(name.clone(), name.into());
    }
    b["nodes"]["fork"] = json!({"kind":"split","scope":ROOT,"children":children,"join":"barrier","reads":{},"writes":{},"outcomes":{}});
    b["nodes"]["barrier"] = json!({"kind":"join","scope":ROOT,"source":"fork","predicate":{"op":"all","outcomes":{"complete":true}},"remaining":"policy_settle","reads":{},"writes":{},"outcomes":{"next":true}});
    compute(&mut b, "after", ROOT, json!({}), json!({}));
    flow(&mut b, "barrier_done", "barrier", "next", "after");
    flow(&mut b, "after_done", "after", "next", "node_done");
    entry(&mut b, "fork");
    let e = envelope(protected(b));
    check_protection(&e, &resources()).unwrap();
    let report = check_invalidation(&e, &resources()).unwrap();
    assert_eq!(
        report
            .historical_origins()
            .iter()
            .filter(|h| h.site() == "after")
            .count(),
        2048
    );
}

#[test]
fn dense_all_join_history_still_refuses_at_the_unchanged_total_work_budget() {
    let mut b = parallel();
    let mut reads = serde_json::Map::new();
    for index in 0..1000 {
        let id = format!("v{index:04}");
        cell(&mut b, &id, "child_a", true);
        b["data"][&id]["invalidates"] = json!({"child_a_work":true});
        reads.insert(id, true.into());
    }
    child_compute(&mut b, "child_a", json!({}), Value::Object(reads));
    b["flows"]["barrier_done"]["target"] = "site_0000".into();
    for index in 0..1000 {
        let id = format!("site_{index:04}");
        compute(&mut b, &id, ROOT, json!({}), json!({}));
        let next = if index == 999 {
            "node_done".into()
        } else {
            format!("site_{:04}", index + 1)
        };
        flow(&mut b, &format!("edge_{index:04}"), &id, "next", &next);
    }
    let e = envelope(protected(b));
    check_control(&e, &resources()).unwrap();
    assert_eq!(
        check_invalidation(&e, &resources()).unwrap_err().code,
        "budget"
    );
}
