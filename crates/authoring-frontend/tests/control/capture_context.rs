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
fn baseline_exact_all_and_partial_any_joins_both_leave_private_branch_context_live() {
    check_invalidation(&envelope(super::historical::closed_call()), &resources()).unwrap();
    for all in [false, true] {
        let e = envelope(joined_local(all));
        check(&e, &resources()).unwrap();
        check_control(&e, &resources()).unwrap();
        check_protection(&e, &resources()).unwrap();
        let error = check_invalidation(&e, &resources()).unwrap_err();
        assert_eq!(error.code, "invalidation-context-unproved");
        assert_eq!(error.path, "/body/nodes/after");
        let error = choreoform_semantic_check::derive_invalidation_requirements(&e, &resources())
            .unwrap_err();
        assert_eq!(error.code, "invalidation-context-unproved");
        assert_eq!(error.path, "/body/nodes/after");
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
