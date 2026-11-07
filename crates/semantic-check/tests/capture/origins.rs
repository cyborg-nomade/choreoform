// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_semantic_check::{FaultReport, check_fault_origins, check_fault_routes};
fn fixture() -> Fixture {
    super::general_fault::fixture()
}
fn handler(f: &mut Fixture, parent: &str, id: &str, keys: &[&str]) {
    let mut scope = f.b["scopes"]["remaining"].clone();
    scope["parent"] = parent.into();
    scope["entry"] = format!("{id}_done").into();
    f.b["scopes"][id] = scope;
    f.b["nodes"][format!("{id}_done")] = json!({"kind":"finish","scope":id,"reads":{},"writes":{},"outcomes":{},"outcome":"complete"});
    let policy = format!("{id}_selection");
    let mappings: serde_json::Map<_, _> = keys
        .iter()
        .map(|key| ((*key).to_owned(), json!(id)))
        .collect();
    f.b["policies"][&policy] = json!({"scope":parent,"dialect":"core","body":{"kind":"faults","reads":{},"handlers":mappings}});
    f.b["scopes"][parent]["faults"] = policy.into();
}
fn activity(f: &mut Fixture, scope: &str, id: &str) {
    let result = format!("{id}_result");
    let policy = format!("{id}_work");
    let mut data = f.b["data"]["notice_result"].clone();
    data["scope"] = scope.into();
    data["invalidates"] = json!({});
    f.b["data"][&result] = data;
    let mut work = f.b["policies"]["notice_work"].clone();
    work["scope"] = scope.into();
    work["body"]["completion"] = boolean(true);
    work["body"]["reads"] = json!({});
    f.b["policies"][&policy] = work;
    let mut n = f.b["nodes"]["notice_request_step"].clone();
    n["scope"] = scope.into();
    n["result"] = result.clone().into();
    n["writes"] = json!({result:true});
    n["policy"] = policy.into();
    f.b["nodes"][id] = n;
    f.b["scopes"][scope]["entry"] = id.into();
    f.b["flows"][format!("{id}_flow")] =
        json!({"source":id,"outcome":"notified","target":format!("{scope}_done")});
}
fn abstract_rows(r: &FaultReport) -> Vec<String> {
    r.routes()
        .map(|v| {
            format!(
                "{}:{:?}:{}:{}:{}",
                v.origin(),
                v.from().map(|s| s.handler()),
                v.selected().owner(),
                v.selected().fault(),
                v.selected().handler()
            )
        })
        .collect()
}
#[test]
fn provider_single_attempt_and_noneligible_routes_keep_exact_source_use_identity() {
    let mut f = fixture();
    handler(&mut f, "expense", "handle", &["provider unknown"]);
    let e = f.envelope();
    let supplied = f.supplied();
    let r = check_fault_origins(&e, &supplied).unwrap();
    let supported: Vec<_> = r.routes().filter(|v| v.source_use().is_some()).collect();
    assert_eq!(supported.len(), 1);
    for row in supported {
        let u = row.source_use().unwrap();
        assert_eq!(u.site(), row.site());
        assert_eq!(u.revision(), r.revision());
        assert!(
            u.source_metadata()
                .emissions()
                .contains_key(row.selected().fault())
        );
        assert_eq!(row.unsupported_reason(), None);
        assert_eq!(row.active_before().count(), 0);
        assert_eq!(row.active_after().last().unwrap().handler(), "handle");
    }
    assert!(
        r.routes()
            .any(|v| v.unsupported_reason() == Some("human-origin-unproved"))
    );
    assert!(r.unsupported_unhandled_cases() <= r.unhandled_cases());
    let original = check_fault_routes(&e, &supplied).unwrap();
    assert_eq!(abstract_rows(&original), abstract_rows(r.faults()));
    assert_eq!(original.contexts(), r.faults().contexts());
    assert_eq!(original.unhandled_cases(), r.faults().unhandled_cases());
}
#[test]
fn same_fault_name_never_reassigns_another_providers_exact_source() {
    let mut f = fixture();
    let pin = Pin::decode(&f.b["policies"]["notice_work"]["body"]["faultSource"]).unwrap();
    let mut source: Value =
        serde_json::from_slice(&f.extra.iter().find(|(p, _)| p == &pin).unwrap().1).unwrap();
    let emission = source["emissions"]["notice uncertainty"].clone();
    source["emissions"] = json!({"provider unknown":emission});
    let p = f.artifact(source);
    f.b["policies"]["notice_work"]["body"]["faultSource"] =
        json!({"id":p.id,"revision":p.revision});
    handler(&mut f, "expense", "handle", &["provider unknown"]);
    let e = f.envelope();
    let supplied = f.supplied();
    let r = check_fault_origins(&e, &supplied).unwrap();
    let rows: Vec<_> = r.routes().filter(|v| v.source_use().is_some()).collect();
    assert_eq!(rows.len(), 2);
    let payment = rows
        .iter()
        .find(|v| v.site() == "payment_request_step")
        .unwrap()
        .source_use()
        .unwrap();
    let notice = rows
        .iter()
        .find(|v| v.site() == "notice_request_step")
        .unwrap()
        .source_use()
        .unwrap();
    assert_ne!(payment.source(), notice.source());
    assert_ne!(payment.provider(), notice.provider());
    assert_eq!(notice.source(), &p);
}
#[test]
fn unvisited_sites_stay_conservative_and_unexplored_scopes_stay_uncovered() {
    let mut f = fixture();
    handler(&mut f, "expense", "handle", &["notice uncertainty"]);
    let original_entry = f.b["scopes"]["notice_service"]["entry"].clone();
    activity(&mut f, "notice_service", "unvisited_notice");
    f.b["scopes"]["notice_service"]["entry"] = original_entry;
    f.b["flows"]["unvisited_notice_flow"]["target"] = "notice_done".into();
    let mut scope = f.b["scopes"]["remaining"].clone();
    scope["parent"] = "notice_service".into();
    f.b["scopes"]["unexplored"] = scope;
    f.b["nodes"]["unexplored_done"] = json!({"kind":"finish","scope":"unexplored","reads":{},"writes":{},"outcomes":{},"outcome":"complete"});
    activity(&mut f, "unexplored", "unexplored_notice");
    let e = f.envelope();
    let supplied = f.supplied();
    let r = check_fault_origins(&e, &supplied).unwrap();
    let row = r.routes().find(|v| v.site() == "unvisited_notice").unwrap();
    assert!(!row.ordinary_reachable());
    assert!(row.source_use().is_some());
    assert!(
        r.ordinary_unvisited_sites()
            .any(|s| s == "unvisited_notice")
    );
    assert!(r.uncovered_sites().any(|s| s == "unexplored_notice"));
    let site = r.sites().find(|s| s.site() == "unexplored_notice").unwrap();
    assert!(!site.scope_explored());
    assert!(site.source_use().is_some());
    assert_eq!(r.sites().count(), r.declared_sites());
    assert!(!r.routes().any(|v| v.site() == "unexplored_notice"));
}
#[test]
fn nested_handler_activity_preserves_full_before_after_and_failed_selection_exclusion() {
    let mut f = fixture();
    handler(&mut f, "expense", "outer", &["notice uncertainty"]);
    handler(&mut f, "notice_service", "first", &["notice uncertainty"]);
    handler(&mut f, "first", "second", &["notice uncertainty"]);
    activity(&mut f, "first", "first_activity");
    activity(&mut f, "second", "second_activity");
    let e = f.envelope();
    let supplied = f.supplied();
    let r = check_fault_origins(&e, &supplied).unwrap();
    let row = r.routes().find(|v| v.site() == "second_activity").unwrap();
    assert_eq!(
        row.active_before().map(|s| s.handler()).collect::<Vec<_>>(),
        vec!["first", "second"]
    );
    assert_eq!(row.selected().handler(), "outer");
    assert_eq!(
        row.active_after().map(|s| s.handler()).collect::<Vec<_>>(),
        vec!["outer"]
    );
    let row = r.routes().find(|v| v.site() == "first_activity").unwrap();
    assert_eq!(
        row.active_after().map(|s| s.handler()).collect::<Vec<_>>(),
        vec!["first", "second"]
    );
}
#[test]
fn legacy_retry_source_is_not_a_general_origin_binding_and_cycles_still_refuse() {
    let mut f = actual_rp01_capture();
    handler(&mut f, "expense", "handle", &["provider unknown"]);
    let e = f.envelope();
    let supplied = f.supplied();
    let r = check_fault_origins(&e, &supplied).unwrap();
    assert!(r.routes().all(|v| v.source_use().is_none()));
    assert!(r.routes().any(|v| v.site() == "payment_request_step"
        && v.unsupported_reason() == Some("general-source-binding-unproved")));
    let mut f = fixture();
    handler(&mut f, "expense", "first", &["a"]);
    let mut scope = f.b["scopes"]["first"].clone();
    scope["entry"] = "second_done".into();
    f.b["scopes"]["second"] = scope;
    f.b["nodes"]["second_done"] = json!({"kind":"finish","scope":"second","reads":{},"writes":{},"outcomes":{},"outcome":"complete"});
    f.b["policies"]["first_selection"]["body"]["handlers"]["b"] = "second".into();
    let e = f.envelope();
    let supplied = f.supplied();
    assert_eq!(
        check_fault_origins(&e, &supplied).unwrap_err().code,
        "fault-routing-cycle-unproved"
    );
}
