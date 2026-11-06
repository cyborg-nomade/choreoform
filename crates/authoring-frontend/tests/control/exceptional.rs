// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_semantic_check::{
    ExceptionalReport, check_exceptional_availability, check_fault_routes,
};

fn handler_body() -> Value {
    let mut b = base();
    child(&mut b, "handler");
    b["policies"]["root_faults"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"faults","reads":{},"handlers":{"failure":"handler"}}});
    b["scopes"][ROOT]["faults"] = "root_faults".into();
    b
}
fn copy_in_handler(b: &mut Value, source: &str) {
    cell(b, "handler_result", "handler", false);
    expr(
        b,
        "handler_read",
        json!({"kind":"integer"}),
        json!({"op":"read","cell":source}),
        json!({source:true}),
    );
    b["expressions"]["handler_read"]["scope"] = "handler".into();
    compute(
        b,
        "handler_copy",
        "handler",
        json!({"handler_result":"handler_read"}),
        json!({source:true}),
    );
    b["scopes"]["handler"]["entry"] = "handler_copy".into();
    flow(b, "handler_copied", "handler_copy", "next", "handler_done");
}
fn pass_exceptional(b: Value) -> ExceptionalReport {
    check_exceptional_availability(&envelope(protected(b)), &resources()).unwrap()
}
fn refuse_exceptional(b: Value, path: &str) {
    let e = envelope(protected(b));
    check_fault_routes(&e, &resources()).unwrap();
    let error = check_exceptional_availability(&e, &resources()).unwrap_err();
    assert_eq!(error.code, "exceptional-availability-unproved");
    assert_eq!(error.path, path);
    assert!(!format!("{error:?}").contains(INTEGER_MARKER));
}

fn repeated_empty_scope_body() -> Value {
    let mut b = base();
    child(&mut b, "empty_child");
    child(&mut b, "unused_storage");
    for i in 0..1000 {
        cell(&mut b, &format!("storage{i}"), "unused_storage", false);
        let call = format!("call{i}");
        invoke(&mut b, &call, "empty_child", json!({}));
        if i > 0 {
            b["flows"][format!("flow_call{}", i - 1)]["target"] = call.into();
        }
    }
    entry(&mut b, "call0");
    b
}

#[test]
fn repeated_empty_scopes_ignore_unrelated_storage_after_one_index_pass() {
    let e = envelope(repeated_empty_scope_body());
    check(&e, &resources()).unwrap();
    let control = check_control(&e, &resources()).unwrap();
    assert!(!control.covered_scopes().contains("unused_storage"));
    let protection = check_protection(&e, &resources()).unwrap();
    assert!(
        protection
            .accesses()
            .iter()
            .all(|a| !a.cell().starts_with("storage"))
    );
}

#[test]
fn selected_handler_reads_only_explicit_ancestor_input_assumptions() {
    let mut b = handler_body();
    cell(&mut b, "input", ROOT, false);
    b["scopes"][ROOT]["inputs"] = json!({"argument":"input"});
    copy_in_handler(&mut b, "input");
    let r = pass_exceptional(b);
    assert_eq!(
        r.entry_assumptions()["handler"],
        std::collections::BTreeSet::from(["input".into()])
    );
    assert!(r.checked_nodes().contains("handler_copy"));
    assert!(r.checked_nodes().contains("handler_done"));
    assert!(!r.uncovered_nodes().contains("handler_copy"));
    assert!(
        r.faults()
            .timing()
            .concurrency()
            .invalidation()
            .protection()
            .control()
            .uncovered_nodes()
            .contains("handler_copy")
    );
    assert!(
        r.remaining()
            .contains(&"live-ancestor-input-revisions-and-successful-activation-premises")
    );
}

#[test]
fn owner_initializer_and_prior_ordinary_template_proof_supply_no_fault_snapshot() {
    for ordinary in [false, true] {
        let mut b = handler_body();
        cell(&mut b, "owner_initial", ROOT, true);
        copy_in_handler(&mut b, "owner_initial");
        if ordinary {
            invoke(&mut b, "ordinary_call", "handler", json!({}));
        }
        refuse_exceptional(b, "/body/nodes/handler_copy");
    }
}

#[test]
fn own_initializers_are_fresh_and_do_not_read_peer_initializers() {
    let mut b = handler_body();
    cell(&mut b, "local", "handler", true);
    copy_in_handler(&mut b, "local");
    let r = pass_exceptional(b.clone());
    assert!(r.entry_assumptions()["handler"].is_empty());
    cell(&mut b, "peer", "handler", false);
    read(&mut b, "read_peer", "peer");
    b["expressions"]["read_peer"]["scope"] = "handler".into();
    b["data"]["local"]["initial"] = "read_peer".into();
    cell(&mut b, "peer", "handler", true);
    refuse_exceptional(b, "/body/data/local");
}

#[test]
fn nested_handlers_get_separate_minimal_entry_assumptions() {
    let mut b = handler_body();
    child(&mut b, "nested");
    b["scopes"]["nested"]["parent"] = "handler".into();
    // The original fixture's actual empty faults ID is retained by handler.
    b["scopes"]["nested"]["faults"] = b["scopes"]["handler"]["faults"].clone();
    b["policies"]["handler_faults"] = json!({"scope":"handler","dialect":"core","body":{"kind":"faults","reads":{},"handlers":{"secondary":"nested"}}});
    b["scopes"]["handler"]["faults"] = "handler_faults".into();
    cell(&mut b, "input", ROOT, false);
    b["scopes"][ROOT]["inputs"] = json!({"argument":"input"});
    let r = pass_exceptional(b.clone());
    assert_eq!(
        r.entry_assumptions()["nested"],
        r.entry_assumptions()["handler"]
    );
    assert!(r.checked_nodes().contains("nested_done"));
    cell(&mut b, "local", "handler", true);
    cell(&mut b, "result", "nested", false);
    read(&mut b, "read_local", "local");
    b["expressions"]["read_local"]["scope"] = "nested".into();
    compute(
        &mut b,
        "nested_copy",
        "nested",
        json!({"result":"read_local"}),
        json!({"local":true}),
    );
    flow(
        &mut b,
        "nested_copied",
        "nested_copy",
        "next",
        "nested_done",
    );
    b["scopes"]["nested"]["entry"] = "nested_copy".into();
    refuse_exceptional(b, "/body/nodes/nested_copy");
}

#[test]
fn uncalled_handler_declarations_receive_no_invented_availability() {
    let mut b = base();
    child(&mut b, "unused");
    child(&mut b, "handler");
    b["scopes"]["handler"]["parent"] = "unused".into();
    b["policies"]["unused_faults"] = json!({"scope":"unused","dialect":"core","body":{"kind":"faults","reads":{},"handlers":{"failure":"handler"}}});
    b["scopes"]["unused"]["faults"] = "unused_faults".into();
    cell(&mut b, "unavailable", ROOT, false);
    copy_in_handler(&mut b, "unavailable");
    let r = pass_exceptional(b);
    assert!(r.entry_assumptions().is_empty());
    assert!(r.checked_nodes().is_empty());
    assert!(r.uncovered_nodes().contains("handler_copy"));
}

#[test]
fn handler_access_metadata_reads_need_their_own_available_inputs() {
    let mut b = handler_body();
    cell(&mut b, "input", ROOT, false);
    cell(&mut b, "guard", ROOT, true);
    b["scopes"][ROOT]["inputs"] = json!({"argument":"input"});
    copy_in_handler(&mut b, "input");
    b["policies"]["handler_access"] = json!({"scope":"handler","dialect":"core","body":{"kind":"actor","reads":{"guard":true},"permit":{"op":"eq","left":{"op":"read","cell":"guard"},"right":{"op":"literal","type":{"kind":"integer"},"value":"0"}}}});
    b["actors"]["handler_actor"] = json!({"scope":"handler","requirement":"handler_access"});
    let cancellation = b["scopes"]["handler"]["cancellation"]
        .as_str()
        .unwrap()
        .to_owned();
    let settlement = b["policies"][cancellation]["body"]["children"].clone();
    b["policies"]["handler_cancel"] = json!({"scope":"handler","dialect":"core","body":{"kind":"cancel","reads":{"guard":true},"authority":"handler_actor","children":settlement}});
    b["scopes"]["handler"]["cancellation"] = "handler_cancel".into();
    b["nodes"]["handler_done"]["reads"] = json!({"guard":true});
    b["nodes"]["handler_copy"]["reads"]["guard"] = true.into();
    refuse_exceptional(b.clone(), "/body/nodes/handler_copy");
    b["data"]["guard"]
        .as_object_mut()
        .unwrap()
        .remove("initial");
    b["scopes"][ROOT]["inputs"]["guard_argument"] = "guard".into();
    let r = pass_exceptional(b);
    assert!(r.entry_assumptions()["handler"].contains("guard"));
}
