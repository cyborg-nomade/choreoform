// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_semantic_check::{FaultReport, check_fault_routes, check_timing};

fn scope(b: &mut Value, id: &str, parent: &str) {
    child(b, id);
    b["scopes"][id]["parent"] = parent.into();
    b["scopes"][id]["faults"] = "empty_faults".into();
}
fn base_routes() -> Value {
    let mut b = base();
    b["policies"]["empty_faults"] =
        json!({"scope":ROOT,"dialect":"core","body":{"kind":"faults","reads":{},"handlers":{}}});
    b
}
fn handlers(b: &mut Value, owner: &str, entries: Value) {
    let id = format!("{owner}_faults");
    b["policies"][&id] = json!({"scope":owner,"dialect":"core","body":{"kind":"faults","reads":{},"handlers":entries}});
    b["scopes"][owner]["faults"] = id.into();
}
fn call_in(b: &mut Value, owner: &str, target: &str) {
    let id = format!("call_{target}");
    invoke(b, &id, target, json!({}));
    b["nodes"][&id]["scope"] = owner.into();
    b["scopes"][owner]["entry"] = id.clone().into();
    b["scopes"][ROOT]["entry"] = "node_done".into();
    let done = if owner == ROOT {
        "node_done".into()
    } else {
        format!("{owner}_done")
    };
    b["flows"][format!("flow_{id}")]["target"] = done.into();
    if owner == ROOT {
        b["scopes"][ROOT]["entry"] = id.into();
    }
}
fn report(b: Value) -> FaultReport {
    check_fault_routes(&envelope(b), &resources()).unwrap()
}
fn cycle(b: Value) {
    let e = envelope(b);
    check_timing(&e, &resources()).unwrap();
    let error = check_fault_routes(&e, &resources()).unwrap_err();
    assert_eq!(error.code, "fault-routing-cycle-unproved");
    assert!(e.document().pointer(&error.path).is_some());
    assert!(!format!("{error:?}").contains(INTEGER_MARKER));
}

#[test]
fn no_handlers_or_calls_supply_no_invented_exceptional_context() {
    let mut b = base_routes();
    scope(&mut b, "unused", ROOT);
    scope(&mut b, "unused_handler", "unused");
    handlers(&mut b, "unused", json!({"failure":"unused_handler"}));
    let r = report(b);
    assert_eq!(r.contexts(), 1);
    assert_eq!(
        r.checked_scopes(),
        &std::collections::BTreeSet::from([ROOT.into()])
    );
    assert_eq!(r.uncovered_handlers().count(), 1);
    assert_eq!(r.routes().count(), 0);
    assert!(r.uncovered_scopes().contains("unused_handler"));
    assert!(
        r.timing()
            .concurrency()
            .invalidation()
            .protection()
            .control()
            .uncovered_nodes()
            .contains("unused_handler_done")
    );
}

#[test]
fn one_handler_excludes_its_failed_selection_after_nested_ordinary_calls() {
    let mut b = base_routes();
    scope(&mut b, "handler", ROOT);
    scope(&mut b, "nested", "handler");
    call_in(&mut b, "handler", "nested");
    handlers(&mut b, ROOT, json!({"failure":"handler"}));
    let r = report(b);
    assert_eq!(r.contexts(), 3);
    assert_eq!(r.covered_handlers().count(), 1);
    assert_eq!(r.routes().count(), 1);
    let route = r.routes().next().unwrap();
    assert_eq!(route.origin(), ROOT);
    assert!(route.from().is_none());
    assert_eq!(route.selected().handler(), "handler");
    assert!(r.checked_scopes().contains("nested"));
    assert_eq!(r.unhandled_cases(), 2);
}

#[test]
fn exact_lexical_selection_precedes_ancestor_with_no_prefix_or_wildcard_match() {
    let mut b = base_routes();
    scope(&mut b, "ordinary", ROOT);
    scope(&mut b, "outer_handler", ROOT);
    scope(&mut b, "inner_handler", "ordinary");
    call_in(&mut b, ROOT, "ordinary");
    handlers(&mut b, ROOT, json!({"failure":"outer_handler"}));
    handlers(&mut b, "ordinary", json!({"failure":"inner_handler"}));
    let r = report(b);
    let route = r
        .routes()
        .find(|r| r.origin() == "ordinary" && r.from().is_none())
        .unwrap();
    assert_eq!(route.selected().handler(), "inner_handler");
    assert!(
        r.routes()
            .any(|r| r.from().is_some_and(|s| s.handler() == "inner_handler")
                && r.selected().handler() == "outer_handler")
    );

    let mut b = base_routes();
    scope(&mut b, "handler", ROOT);
    handlers(&mut b, ROOT, json!({"failure":"handler"}));
    // An unused policy contributes a distinct exact name, which has no match.
    b["policies"]["unused_faults"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"faults","reads":{},"handlers":{"failure.more":"handler"}}});
    let r = report(b);
    assert_eq!(r.routes().count(), 1);
    assert_eq!(r.unhandled_cases(), 3);
    assert!(r.routes().all(|r| r.selected().fault() == "failure"));
}

#[test]
fn nested_handlers_keep_outer_failed_selections_excluded_when_faults_escape() {
    let mut b = base_routes();
    scope(&mut b, "handler", ROOT);
    scope(&mut b, "inner", "handler");
    handlers(&mut b, ROOT, json!({"failure":"handler"}));
    handlers(&mut b, "handler", json!({"secondary":"inner"}));
    let r = report(b);
    assert_eq!(r.contexts(), 3);
    assert_eq!(r.covered_handlers().count(), 2);
    assert!(
        r.routes()
            .any(|r| r.from().is_some_and(|s| s.handler() == "handler")
                && r.selected().handler() == "inner")
    );
    assert!(
        !r.routes()
            .any(|r| r.from().is_some_and(|s| s.handler() == "inner"))
    );
}

#[test]
fn different_key_mutual_and_same_template_reentry_remain_unproved() {
    for same in [false, true] {
        let mut b = base_routes();
        scope(&mut b, "one", ROOT);
        if !same {
            scope(&mut b, "two", ROOT);
        }
        handlers(
            &mut b,
            ROOT,
            json!({"first":"one","second":if same {"one"} else {"two"}}),
        );
        cycle(b);
    }
}

#[test]
fn ordinary_and_active_handler_uses_of_a_template_remain_distinct() {
    let mut b = base_routes();
    scope(&mut b, "handler", ROOT);
    call_in(&mut b, ROOT, "handler");
    handlers(&mut b, ROOT, json!({"failure":"handler"}));
    let r = report(b);
    assert_eq!(r.contexts(), 3);
    assert!(r.routes().any(|r| r.origin() == "handler"
        && r.from().is_none()
        && r.selected().handler() == "handler"));
    assert!(!r.routes().any(|r| r.from().is_some()));
}

#[test]
fn many_uncalled_declarations_stay_uncovered_without_quadratic_context_creation() {
    let mut b = base_routes();
    for i in 0..1000 {
        scope(&mut b, &format!("unused{i}"), ROOT);
    }
    let r = report(b);
    assert_eq!(r.contexts(), 1);
    assert_eq!(r.uncovered_scopes().len(), 1000);
}

#[test]
fn deep_handler_stacks_preserve_exclusion_without_native_recursion() {
    let mut b = base_routes();
    let mut owner = ROOT.to_owned();
    for i in 0..63 {
        let target = format!("handler{i}");
        scope(&mut b, &target, &owner);
        handlers(&mut b, &owner, json!({format!("failure{i}"):target}));
        owner = target;
    }
    let r = report(b);
    assert_eq!(r.contexts(), 64);
    assert_eq!(r.covered_handlers().count(), 63);
    assert_eq!(r.checked_scopes().len(), 64);
}

#[test]
fn wide_fault_lookup_work_refuses_within_the_explicit_budget() {
    let mut b = base_routes();
    scope(&mut b, "unused_handler", ROOT);
    let mut faults = serde_json::Map::new();
    for i in 0..1000 {
        let target = format!("ordinary{i}");
        scope(&mut b, &target, ROOT);
        let call = format!("call{i}");
        invoke(&mut b, &call, &target, json!({}));
        if i > 0 {
            b["flows"][format!("flow_call{}", i - 1)]["target"] = call.into();
        }
        faults.insert(format!("failure{i}"), "unused_handler".into());
    }
    entry(&mut b, "call0");
    b["policies"]["unused_policy"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"faults","reads":{},"handlers":faults}});
    let e = envelope(b);
    check_timing(&e, &resources()).unwrap();
    assert_eq!(
        check_fault_routes(&e, &resources()).unwrap_err().code,
        "budget"
    );
}

#[test]
fn fault_name_spelling_never_authenticates_an_engine_business_class() {
    let mut b = base_routes();
    scope(&mut b, "handler", ROOT);
    handlers(&mut b, ROOT, json!({"engine":"handler"}));
    let r = report(b);
    assert_eq!(r.covered_handlers().next().unwrap().fault(), "engine");
    assert!(
        r.remaining()
            .contains(&"truthful-business-versus-engine-fault-classification")
    );
}

#[test]
fn retry_eligibility_never_narrows_potential_business_fault_emission() {
    let mut b = base_routes();
    scope(&mut b, "handler", ROOT);
    handlers(&mut b, ROOT, json!({"failure":"handler"}));
    b["policies"]["unrelated_work"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"work","reads":{},"instructions":"Synthetic","authority":"actor_reviewer","retry":{"maxAttempts":1,"delays":[],"faults":{"different":true}},"timeout":null,"completion":{"op":"literal","type":{"kind":"boolean"},"value":true},"compensates":null}});
    let r = report(b);
    assert_eq!(r.covered_handlers().next().unwrap().fault(), "failure");
}
