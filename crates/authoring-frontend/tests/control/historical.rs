// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

pub(super) fn closed_call() -> Value {
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "basis", ROOT, true);
    cell(&mut b, "local", "child", false);
    cell(&mut b, "copy", ROOT, false);
    read(&mut b, "copy_basis", "basis");
    b["data"]["local"]["initial"] = "copy_basis".into();
    b["data"]["local"]["invalidates"] = json!({"child_observe":true});
    b["data"]["basis"]["invalidates"] = json!({"child_observe":true,"after_child":true});
    b["data"]["copy"]["invalidates"] = json!({"after_child":true});
    compute(
        &mut b,
        "child_observe",
        "child",
        json!({}),
        json!({"local":true}),
    );
    b["scopes"]["child"]["entry"] = "child_observe".into();
    b["scopes"]["child"]["outputs"] = json!({"result":"local"});
    flow(&mut b, "child_next", "child_observe", "next", "child_done");
    invoke(&mut b, "call", "child", json!({"result":"copy"}));
    b["nodes"]["call"]["reads"] = json!({"basis":true});
    compute(&mut b, "after_child", ROOT, json!({}), json!({"copy":true}));
    b["flows"]["flow_call"]["target"] = "after_child".into();
    flow(&mut b, "after_done", "after_child", "next", "node_done");
    protected(b)
}

#[test]
fn historical_records_bind_exact_call_owner_and_keep_mutable_ancestor_and_output_cells() {
    let b = closed_call();
    let e = envelope(b.clone());
    let report = check_invalidation(&e, &resources()).unwrap();
    assert_eq!(report.revision(), e.document()["revision"]);
    let h = report.historical_origins().first().unwrap();
    assert_eq!(
        (h.site(), h.cell(), h.boundary(), h.scope(), h.closure()),
        ("after_child", "local", "call", "child", "policy_close")
    );
    assert_eq!(
        h.condition(),
        "ordinary-completion-with-no-declared-descendant-writer"
    );
    assert!(
        report
            .obligations()
            .iter()
            .any(|o| o.site() == "after_child" && o.cell() == "basis")
    );
    assert!(
        report
            .obligations()
            .iter()
            .any(|o| o.site() == "after_child" && o.cell() == "copy")
    );
    assert!(!format!("{h:?}").contains(INTEGER_MARKER));
    let mut missing = b;
    missing["data"]["basis"]["invalidates"]
        .as_object_mut()
        .unwrap()
        .remove("after_child");
    let e = check_invalidation(&envelope(missing), &resources()).unwrap_err();
    assert_eq!(e.code, "invalidation-coverage");
    assert_eq!(e.path, "/body/data/basis/invalidates");
}

#[test]
fn every_declared_descendant_writer_blocks_history_even_if_unvisited_and_terminal() {
    let mut b = closed_call();
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
    flow(&mut b, "mutate_done", "mutate", "next", "writer_done");
    let e = envelope(b);
    check_protection(&e, &resources()).unwrap();
    assert_eq!(
        check_invalidation(&e, &resources()).unwrap_err().code,
        "invalidation-context-unproved"
    );
    let mut forged = e.document()["body"].clone();
    forged["nodes"]["mutate"]["writes"] = json!({});
    assert_eq!(
        check(&envelope(forged), &resources()).unwrap_err().code,
        "compute-interface"
    );
}

#[test]
fn readonly_surviving_closure_is_data_evidence_without_a_lifetime_permission() {
    let mut b = closed_call();
    child(&mut b, "recovery");
    b["scopes"]["recovery"]["parent"] = "child".into();
    b["policies"]["keep"] = json!({"scope":"child","dialect":"core","body":{"kind":"closure","reads":{},"mode":"reconcile","followup":"recovery"}});
    b["scopes"]["child"]["closure"] = "keep".into();
    let e = envelope(b);
    let report = check_invalidation(&e, &resources()).unwrap();
    assert!(
        report
            .historical_origins()
            .iter()
            .any(|h| h.closure() == "keep")
    );
    let error = check_concurrency(&e, &resources()).unwrap_err();
    assert_eq!(error.code, "scope-lifetime-unproved");
    assert_eq!(error.path, "/body/nodes/call");
}

#[test]
fn historical_control_keeps_its_original_protection_label() {
    let mut b = closed_call();
    b["actors"]["other"] = json!({"scope":ROOT,"requirement":"policy_actor"});
    b["data"]["basis"]["protection"]["participants"]["other"] = true.into();
    b["scopes"]["child"]["outputs"] = json!({});
    b["nodes"]["call"]["outputs"] = json!({});
    b["nodes"]["call"]["writes"] = json!({});
    cell(&mut b, "public", ROOT, false);
    b["data"]["public"]["protection"]["participants"] = json!({"actor_reviewer":true,"other":true});
    compute(
        &mut b,
        "after_child",
        ROOT,
        json!({"public":"zero"}),
        json!({}),
    );
    let e = envelope(b);
    let ordinary = check_protection(&e, &resources()).unwrap_err();
    let refined = check_invalidation(&e, &resources()).unwrap_err();
    assert_eq!(ordinary.code, "control-information-flow");
    assert_eq!(refined, ordinary);
}

#[test]
fn fork_fanout_and_an_inherited_live_template_are_not_frozen_by_a_later_call() {
    for fan in [false, true] {
        let mut b = if fan { fanout() } else { parallel() };
        if !fan {
            cell(&mut b, "local", "child_a", true);
            child_compute(&mut b, "child_a", json!({}), json!({"local":true}));
            b["data"]["local"]["invalidates"] = json!({"child_a_work":true});
            b["nodes"]["barrier"]["predicate"] = json!({"op":"any","outcomes":{"complete":true}});
        }
        compute(&mut b, "after", ROOT, json!({}), json!({}));
        b["flows"]["barrier_done"]["target"] = "after".into();
        flow(&mut b, "after_done", "after", "next", "node_done");
        invalidation_refuse(b.clone(), "invalidation-context-unproved");
        if !fan {
            invoke(&mut b, "call_again", "child_a", json!({}));
            entry(&mut b, "fork");
            b["flows"]["barrier_done"]["target"] = "call_again".into();
            b["flows"]["flow_call_again"]["target"] = "after".into();
            invalidation_refuse(b, "invalidation-context-unproved");
        }
    }
}

#[test]
fn repeated_calls_keep_distinct_conditional_boundaries_without_caching_values() {
    let mut b = closed_call();
    invoke(&mut b, "second_call", "child", json!({"result":"copy"}));
    b["data"]["copy"]["invalidates"]["child_observe"] = true.into();
    b["nodes"]["second_call"]["reads"] = json!({"basis":true});
    b["flows"]["flow_call"]["target"] = "second_call".into();
    b["flows"]["flow_second_call"]["target"] = "after_child".into();
    entry(&mut b, "call");
    let report = check_invalidation(&envelope(b), &resources()).unwrap();
    let boundaries: std::collections::BTreeSet<_> = report
        .historical_origins()
        .iter()
        .filter(|h| h.site() == "after_child" && h.cell() == "local")
        .map(|h| h.boundary())
        .collect();
    assert_eq!(
        boundaries,
        std::collections::BTreeSet::from(["call", "second_call"])
    );
}

#[test]
fn dense_historical_contexts_charge_the_existing_total_work_budget() {
    let mut b = base();
    child(&mut b, "child");
    let mut reads = serde_json::Map::new();
    for i in 0..1000 {
        let id = format!("local_{i:04}");
        cell(&mut b, &id, "child", true);
        b["data"][&id]["invalidates"] = json!({"child_observe":true});
        reads.insert(id, true.into());
    }
    compute(
        &mut b,
        "child_observe",
        "child",
        json!({}),
        Value::Object(reads),
    );
    b["scopes"]["child"]["entry"] = "child_observe".into();
    flow(&mut b, "child_next", "child_observe", "next", "child_done");
    invoke(&mut b, "call", "child", json!({}));
    b["flows"]["flow_call"]["target"] = "site_0000".into();
    for i in 0..1000 {
        let id = format!("site_{i:04}");
        compute(&mut b, &id, ROOT, json!({}), json!({}));
        let next = if i == 999 {
            "node_done".into()
        } else {
            format!("site_{:04}", i + 1)
        };
        flow(&mut b, &format!("edge_{i:04}"), &id, "next", &next);
    }
    let e = envelope(protected(b));
    check_control(&e, &resources()).unwrap();
    assert_eq!(
        check_invalidation(&e, &resources()).unwrap_err().code,
        "budget"
    );
}
