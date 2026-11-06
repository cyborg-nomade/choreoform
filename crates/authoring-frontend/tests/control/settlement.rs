// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

fn transfer_body() -> Value {
    let mut b = base();
    cell(&mut b, "receiver_evidence", ROOT, true);
    child(&mut b, "child");
    b["policies"]["policy_actor"]["body"]["permit"] = json!({"op":"eq","left":{"op":"read","cell":"receiver_evidence"},"right":{"op":"literal","type":{"kind":"integer"},"value":INTEGER_MARKER}});
    b["policies"]["policy_actor"]["body"]["reads"] = json!({"receiver_evidence":true});
    let cancel = b["scopes"][ROOT]["cancellation"]
        .as_str()
        .unwrap()
        .to_owned();
    b["policies"][cancel]["body"]["reads"] = json!({"receiver_evidence":true});
    b["nodes"]["node_done"]["reads"] = json!({"receiver_evidence":true});
    b["actors"]["child_actor"] = json!({"scope":"child","requirement":"child_requirement"});
    b["policies"]["child_requirement"] = json!({"scope":"child","dialect":"core","body":{"kind":"actor","reads":{},"permit":{"op":"literal","type":{"kind":"boolean"},"value":true}}});
    b["policies"]["child_transfer"] = json!({"scope":"child","dialect":"core","body":{"kind":"settlement","reads":{},"unfinished":"transfer","owner":ROOT}});
    b["policies"]["child_cancel"] = json!({"scope":"child","dialect":"core","body":{"kind":"cancel","reads":{},"authority":"child_actor","children":"child_transfer"}});
    b["scopes"]["child"]["cancellation"] = "child_cancel".into();
    b
}

#[test]
fn transfer_requires_receiver_management_reads_and_propagates_them() {
    let mut b = transfer_body();
    let error = check(&envelope(b.clone()), &resources()).unwrap_err();
    assert_eq!(error.code, "policy-reads");
    assert_eq!(error.path, "/body/policies/child_transfer/body/reads");
    b["policies"]["child_transfer"]["body"]["reads"] = json!({"receiver_evidence":true});
    assert_eq!(
        check(&envelope(b.clone()), &resources()).unwrap_err().code,
        "policy-reads"
    );
    b["policies"]["child_cancel"]["body"]["reads"] = json!({"receiver_evidence":true});
    let error = check(&envelope(b.clone()), &resources()).unwrap_err();
    assert_eq!(error.code, "node-reads");
    assert_eq!(error.path, "/body/nodes/child_done");
    b["nodes"]["child_done"]["reads"] = json!({"receiver_evidence":true});
    check(&envelope(b.clone()), &resources()).unwrap();
    invoke(&mut b, "call", "child", json!({}));
    b["nodes"]["call"]["reads"] = json!({"receiver_evidence":true});
    check_control(&envelope(b), &resources()).unwrap();
}

fn complete_transfer() -> Value {
    let mut b = transfer_body();
    for policy in ["child_transfer", "child_cancel"] {
        b["policies"][policy]["body"]["reads"] = json!({"receiver_evidence":true});
    }
    b["nodes"]["child_done"]["reads"] = json!({"receiver_evidence":true});
    b
}

#[test]
fn transfer_reads_receiver_authority_not_its_own_children_settlement() {
    for inherited in [false, true] {
        let mut b = complete_transfer();
        child(&mut b, "grandchild");
        b["scopes"]["grandchild"]["parent"] = "child".into();
        b["nodes"]["grandchild_done"]["reads"] = json!({"receiver_evidence":true});
        b["policies"]["grand_transfer"] = json!({"scope":"grandchild","dialect":"core","body":{"kind":"settlement","reads":{},"unfinished":"transfer","owner":"child"}});
        if inherited {
            b["scopes"]["child"]["cancellation"] = b["scopes"][ROOT]["cancellation"].clone();
            let error = check(&envelope(b.clone()), &resources()).unwrap_err();
            assert_eq!(error.code, "policy-reads");
            assert_eq!(error.path, "/body/policies/grand_transfer/body/reads");
            b["policies"]["grand_transfer"]["body"]["reads"] = json!({"receiver_evidence":true});
        }
        check(&envelope(b), &resources()).unwrap();
    }
}

#[test]
fn cancel_and_retain_keep_null_owner_without_receiver_dependencies() {
    for mode in ["cancel", "retain"] {
        let mut b = transfer_body();
        b["policies"]["child_transfer"]["body"]["unfinished"] = mode.into();
        b["policies"]["child_transfer"]["body"]["owner"] = Value::Null;
        check(&envelope(b), &resources()).unwrap();
    }
}

#[test]
fn receiver_links_are_validated_in_its_actual_context() {
    for kind in [false, true] {
        let mut b = complete_transfer();
        if kind {
            b["scopes"][ROOT]["cancellation"] = "policy_actor".into();
        } else {
            let cancel = b["scopes"][ROOT]["cancellation"]
                .as_str()
                .unwrap()
                .to_owned();
            b["policies"][cancel]["body"]["authority"] = "child_actor".into();
        }
        let error = check(&envelope(b), &resources()).unwrap_err();
        assert_eq!(error.code, if kind { "policy-kind" } else { "visibility" });
        assert_eq!(error.path, "/body/policies/child_transfer");
    }
}

#[test]
fn repeated_receiver_dependency_edges_remain_within_work_limits() {
    let mut b = complete_transfer();
    for i in 0..500 {
        b["policies"][format!("transfer{i}")] = b["policies"]["child_transfer"].clone();
    }
    check(&envelope(b), &resources()).unwrap();
}
