// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_semantic_check::{check_reconciliation_availability, check_reconciliation_links};

fn body() -> Value {
    let mut b = base();
    child(&mut b, "followup");
    b["policies"]["reconcile_close"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"closure","reads":{},"mode":"reconcile","followup":"followup"}});
    b["scopes"][ROOT]["closure"] = "reconcile_close".into();
    b
}
fn copy(b: &mut Value, source: &str) {
    cell(b, "result", "followup", false);
    expr(
        b,
        "copy_expr",
        json!({"kind":"integer"}),
        json!({"op":"read","cell":source}),
        json!({source:true}),
    );
    b["expressions"]["copy_expr"]["scope"] = "followup".into();
    compute(
        b,
        "copy",
        "followup",
        json!({"result":"copy_expr"}),
        json!({source:true}),
    );
    b["scopes"]["followup"]["entry"] = "copy".into();
    flow(b, "copied", "copy", "next", "followup_done");
}
fn refusal(b: Value, path: &str) {
    let e = envelope(protected(b));
    check_reconciliation_links(&e, &resources()).unwrap();
    let error = check_reconciliation_availability(&e, &resources()).unwrap_err();
    assert_eq!(error.code, "reconciliation-availability-unproved");
    assert_eq!(error.path, path);
    assert!(!format!("{error:?}").contains(INTEGER_MARKER));
}

#[test]
fn empty_entry_has_exact_links_no_inherited_values_and_separate_coverage() {
    let e = envelope(body());
    let r = check_reconciliation_availability(&e, &resources()).unwrap();
    let entry = r.entries().first().unwrap();
    assert_eq!(entry.owner(), ROOT);
    assert_eq!(entry.closure(), "reconcile_close");
    assert_eq!(entry.followup(), "followup");
    assert!(
        entry
            .host_requirements()
            .contains(&"live-required-storage-and-fresh-local-initialization")
    );
    assert_eq!(r.revision(), e.document()["revision"].as_str().unwrap());
    assert!(r.checked_nodes().contains("followup_done"));
    assert!(!r.checked_nodes().contains("node_done"));
    assert!(
        r.links()
            .exceptional()
            .uncovered_nodes()
            .contains("followup_done")
    );
    assert!(r.uncovered_nodes().is_empty());
    let r = check_reconciliation_availability(&envelope(base()), &resources()).unwrap();
    assert!(r.entries().is_empty());
    assert!(r.checked_nodes().is_empty());
}

#[test]
fn parent_inputs_initializers_and_ordinary_results_are_not_retained_entry_values() {
    for kind in ["input", "initializer", "write"] {
        let mut b = body();
        cell(&mut b, "parent_value", ROOT, kind == "initializer");
        if kind == "input" {
            b["scopes"][ROOT]["inputs"] = json!({"argument":"parent_value"});
        }
        if kind == "write" {
            compute(
                &mut b,
                "parent_write",
                ROOT,
                json!({"parent_value":"zero"}),
                json!({}),
            );
            entry(&mut b, "parent_write");
            flow(
                &mut b,
                "parent_written",
                "parent_write",
                "next",
                "node_done",
            );
        }
        copy(&mut b, "parent_value");
        refusal(b, "/body/nodes/copy");
    }
}

#[test]
fn fresh_local_constants_are_available_but_peer_initializers_cannot_seed_each_other() {
    let mut b = body();
    cell(&mut b, "local", "followup", true);
    copy(&mut b, "local");
    let r = check_reconciliation_availability(&envelope(protected(b)), &resources()).unwrap();
    assert!(r.checked_nodes().contains("copy"));
    let mut b = body();
    cell(&mut b, "a", "followup", true);
    cell(&mut b, "b", "followup", false);
    expr(
        &mut b,
        "peer",
        json!({"kind":"integer"}),
        json!({"op":"read","cell":"a"}),
        json!({"a":true}),
    );
    b["expressions"]["peer"]["scope"] = "followup".into();
    b["data"]["b"]["initial"] = "peer".into();
    refusal(b, "/body/data/b");
}

#[test]
fn ordinary_proof_of_the_same_template_cannot_replace_reconciliation_entry() {
    let mut b = body();
    cell(&mut b, "argument", ROOT, false);
    b["scopes"][ROOT]["inputs"] = json!({"argument":"argument"});
    copy(&mut b, "argument");
    invoke(&mut b, "ordinary_call", "followup", json!({}));
    let e = envelope(protected(b.clone()));
    let r = check_reconciliation_links(&e, &resources()).unwrap();
    assert!(!r.exceptional().uncovered_nodes().contains("copy"));
    refusal(b, "/body/nodes/copy");
}

#[test]
fn handler_role_or_unused_closure_metadata_cannot_substitute_for_followup_entry() {
    let mut b = body();
    cell(&mut b, "argument", ROOT, false);
    b["scopes"][ROOT]["inputs"] = json!({"argument":"argument"});
    copy(&mut b, "argument");
    b["policies"]["root_faults"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"faults","reads":{},"handlers":{"failure":"followup"}}});
    b["scopes"][ROOT]["faults"] = "root_faults".into();
    let r = check_reconciliation_links(&envelope(protected(b.clone())), &resources()).unwrap();
    assert!(r.exceptional().checked_nodes().contains("copy"));
    refusal(b, "/body/nodes/copy");
    let mut b = body();
    b["scopes"][ROOT]["closure"] = b["scopes"]["followup"]["closure"].clone();
    let r = check_reconciliation_availability(&envelope(b), &resources()).unwrap();
    assert!(r.entries().is_empty());
    assert!(r.uncovered_nodes().contains("followup_done"));
}

#[test]
fn ordinary_children_and_nested_declared_followups_get_fresh_contexts() {
    let mut b = body();
    child(&mut b, "inner");
    b["scopes"]["inner"]["parent"] = "followup".into();
    b["scopes"]["inner"]["closure"] = b["scopes"]["followup"]["closure"].clone();
    cell(&mut b, "inner_local", "inner", true);
    for i in 0..2 {
        let id = format!("call{i}");
        b["nodes"][&id] = json!({"kind":"invoke","scope":"followup","body":"inner","inputs":{},"outputs":{},"reads":{},"writes":{},"outcomes":{"complete":true}});
        flow(
            &mut b,
            &format!("flow{i}"),
            &id,
            "complete",
            if i == 0 { "call1" } else { "followup_done" },
        );
    }
    b["scopes"]["followup"]["entry"] = "call0".into();
    let r = check_reconciliation_availability(&envelope(b.clone()), &resources()).unwrap();
    assert!(r.checked_scopes().contains("inner"));
    assert_eq!(r.entries().len(), 1);
    b["policies"]["nested_close"] = json!({"scope":"followup","dialect":"core","body":{"kind":"closure","reads":{},"mode":"reconcile","followup":"inner"}});
    b["scopes"]["followup"]["closure"] = "nested_close".into();
    let r = check_reconciliation_availability(&envelope(b), &resources()).unwrap();
    assert_eq!(r.entries().len(), 2);
    assert!(r.checked_scopes().contains("inner"));
}

#[test]
fn many_declared_entries_share_one_graph_and_initializer_index() {
    let mut b = body();
    let terminal = b["scopes"]["followup"]["closure"].clone();
    for i in 0..400 {
        let owner = format!("owner{i}");
        let followup = format!("followup{i}");
        child(&mut b, &owner);
        child(&mut b, &followup);
        b["scopes"][&followup]["parent"] = owner.clone().into();
        b["scopes"][&followup]["closure"] = terminal.clone();
        let closure = format!("close{i}");
        b["policies"][&closure] = json!({"scope":owner,"dialect":"core","body":{"kind":"closure","reads":{},"mode":"reconcile","followup":followup}});
        b["scopes"][&owner]["closure"] = closure.into();
    }
    let r = check_reconciliation_availability(&envelope(b), &resources()).unwrap();
    assert_eq!(r.entries().len(), 401);
    assert_eq!(r.checked_scopes().len(), 401);
    assert_eq!(r.uncovered_nodes().len(), 400);
}
