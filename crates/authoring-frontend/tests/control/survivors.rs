// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_semantic_check::check_timing;

fn add_survivor(b: &mut Value, body: &str) {
    child(b, "followup");
    b["scopes"]["followup"]["parent"] = body.into();
    b["scopes"]["followup"]["entry"] = "observe".into();
    b["scopes"][body]["closure"] = "keep".into();
    b["policies"]["keep"] = json!({"scope":body,"dialect":"core","body":{"kind":"closure","reads":{},"mode":"reconcile","followup":"followup"}});
    b["policies"]["wait"] = json!({"scope":body,"dialect":"core","body":{"kind":"wait","reads":{},"observationType":{"kind":"text"},"match":{"op":"literal","type":{"kind":"boolean"},"value":true},"authority":"actor_reviewer","timer":null,"onObservation":"next","onTimer":null}});
    b["nodes"]["observe"] = json!({"kind":"wait","scope":"followup","policy":"wait","reads":{},"writes":{},"outcomes":{"next":true}});
    flow(b, "observed", "observe", "next", "followup_done");
}
fn isolated() -> Value {
    let mut b = base();
    child(&mut b, "body");
    add_survivor(&mut b, "body");
    invoke(&mut b, "call", "body", json!({}));
    protected(b)
}
fn refuses(b: Value) {
    let e = envelope(protected(b));
    check_invalidation(&e, &resources()).unwrap();
    let error = check_concurrency(&e, &resources()).unwrap_err();
    assert_eq!(error.code, "scope-lifetime-unproved");
    assert_eq!(error.path, "/body/nodes/call");
}

#[test]
fn synchronous_calls_and_repeats_disclose_conditional_isolation_without_actual_receipts() {
    for repeat in [false, true] {
        let mut b = isolated();
        if repeat {
            b["nodes"]["call"] = json!({"kind":"repeat","scope":ROOT,"body":"body","condition":"yes","reads":{},"writes":{},"outcomes":{"next":true}});
            b["flows"]["flow_call"]["outcome"] = "next".into();
        }
        let e = envelope(b);
        let before = e.document().clone();
        let r = check_concurrency(&e, &resources()).unwrap();
        let p = r.isolated_followups().first().unwrap();
        assert_eq!(r.revision(), e.document()["revision"]);
        assert_eq!(
            (p.creator(), p.body(), p.closure(), p.followup()),
            ("call", "body", "keep", "followup")
        );
        assert_eq!(
            p.waits(),
            &std::collections::BTreeSet::from(["observe".to_owned()])
        );
        assert!(
            p.host_requirements()
                .contains(&"all-owned-obligations-settled-before-terminal-disposal")
        );
        assert_eq!(before, *e.document());
        assert_eq!(
            check_timing(&e, &resources())
                .unwrap()
                .concurrency()
                .isolated_followups(),
            r.isolated_followups()
        );
    }
}

#[test]
fn shared_reads_and_unvisited_writes_remain_unproved() {
    let mut b = isolated();
    cell(&mut b, "basis", ROOT, true);
    b["policies"]["wait"]["body"]["match"] = json!({"op":"eq","left":{"op":"read","cell":"basis"},"right":{"op":"literal","type":{"kind":"integer"},"value":INTEGER_MARKER}});
    b["policies"]["wait"]["body"]["reads"] = json!({"basis":true});
    b["nodes"]["observe"]["reads"] = json!({"basis":true});
    refuses(b);
    let mut b = isolated();
    cell(&mut b, "basis", ROOT, true);
    compute(
        &mut b,
        "unvisited_writer",
        "followup",
        json!({"basis":"zero"}),
        json!({}),
    );
    flow(
        &mut b,
        "writer_done",
        "unvisited_writer",
        "next",
        "followup_done",
    );
    refuses(b);
}

#[test]
fn storage_initializers_nested_work_and_unvisited_control_are_not_isolated_waits() {
    for initial in [false, true] {
        let mut b = isolated();
        cell(&mut b, "unused", "followup", initial);
        refuses(b);
    }
    let mut b = isolated();
    child(&mut b, "nested");
    b["scopes"]["nested"]["parent"] = "followup".into();
    refuses(b);
    let mut b = isolated();
    compute(&mut b, "unvisited_choice", "followup", json!({}), json!({}));
    flow(
        &mut b,
        "choice_done",
        "unvisited_choice",
        "next",
        "followup_done",
    );
    refuses(b);
    let mut b = isolated();
    b["policies"]["work"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"work","reads":{},"instructions":"Opaque external judgment","authority":"actor_reviewer","retry":{"maxAttempts":1,"delays":[],"faults":{}},"timeout":null,"completion":{"op":"literal","type":{"kind":"boolean"},"value":true},"compensates":null}});
    b["nodes"]["unvisited_human"] = json!({"kind":"activity","scope":"followup","mode":"human","target":"actor_reviewer","policy":"work","input":"zero","result":null,"reads":{},"writes":{},"outcomes":{"next":true}});
    flow(
        &mut b,
        "human_done",
        "unvisited_human",
        "next",
        "followup_done",
    );
    refuses(b);
}

#[test]
fn scope_management_policy_reads_cannot_hide_behind_data_free_wait_nodes() {
    let mut b = isolated();
    cell(&mut b, "basis", ROOT, true);
    expr(
        &mut b,
        "management_guard",
        json!({"kind":"boolean"}),
        json!({"op":"eq","left":{"op":"read","cell":"basis"},"right":{"op":"literal","type":{"kind":"integer"},"value":INTEGER_MARKER}}),
        json!({"basis":true}),
    );
    b["policies"]["management_race"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"race","reads":{"basis":true},"mode":"guarded","invariant":b["expressions"]["management_guard"]["body"]}});
    b["scopes"]["followup"]["race"] = "management_race".into();
    b["nodes"]["followup_done"]["reads"] = json!({"basis":true});
    b["nodes"]["observe"]["reads"] = json!({"basis":true});
    refuses(b);
}

#[test]
fn finish_only_entry_and_nonterminal_followup_are_not_progress_or_lifetime_proofs() {
    let mut b = isolated();
    b["scopes"]["followup"]["entry"] = "followup_done".into();
    refuses(b);
    let mut b = isolated();
    child(&mut b, "nested");
    b["scopes"]["nested"]["parent"] = "followup".into();
    b["policies"]["keep_followup"] = json!({"scope":"followup","dialect":"core","body":{"kind":"closure","reads":{},"mode":"reconcile","followup":"nested"}});
    b["scopes"]["followup"]["closure"] = "keep_followup".into();
    refuses(b);
}

#[test]
fn split_and_fanout_still_require_terminal_children_despite_an_isolated_followup() {
    for fan in [false, true] {
        let mut b = if fan { fanout() } else { parallel() };
        add_survivor(&mut b, "child_a");
        concurrency_refuse(b, "scope-lifetime-unproved");
    }
}

#[test]
fn reused_templates_keep_each_creator_and_require_fresh_revision_checks() {
    let mut b = isolated();
    invoke(&mut b, "second_call", "body", json!({}));
    entry(&mut b, "call");
    b["flows"]["flow_call"]["target"] = "second_call".into();
    let old = envelope(b.clone());
    let r = check_concurrency(&old, &resources()).unwrap();
    assert_eq!(r.isolated_followups().len(), 2);
    assert_eq!(
        r.isolated_followups()
            .iter()
            .map(|p| p.creator())
            .collect::<Vec<_>>(),
        vec!["call", "second_call"]
    );
    assert!(r.isolated_followups().iter().all(|p| p.body() == "body"));
    cell(&mut b, "new_storage", "followup", false);
    let fresh = envelope(protected(b));
    assert_ne!(r.revision(), fresh.document()["revision"]);
    assert_eq!(
        check_concurrency(&fresh, &resources()).unwrap_err().code,
        "scope-lifetime-unproved"
    );
}
