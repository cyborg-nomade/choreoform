// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_semantic_check::{CORE_V6, FAULT_SOURCE_PROFILE, check_fault_routes};

fn fixture() -> Fixture {
    let mut f = actual_rp01_capture();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    f.extra.push((
        Pin {
            id: CORE_V6.0.into(),
            revision: CORE_V6.1.into(),
        },
        std::fs::read(root.join(format!(
            "docs/ir/contracts/{}.txt",
            CORE_V6.1.replace(':', "-")
        )))
        .unwrap(),
    ));
    f.b["dialects"]["core"] = json!({"id":CORE_V6.0,"revision":CORE_V6.1});
    for p in f.b["policies"].as_object_mut().unwrap().values_mut() {
        if p["body"]["kind"] == "work" {
            p["body"]["faultSource"] = p["body"]["retry"]["faultSource"].clone();
        }
    }
    let source = json!({"kind":"provider-fault-source","version":"0.1.0","id":"urn:test:general-notice-faults","profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":f.b["policies"]["notice_capability_provider"]["body"]["provider"],"emissions":{"notice uncertainty":{"observations":{"unknown":true},"evidence":"unresolved"}}});
    let pin = f.artifact(source);
    f.b["policies"]["notice_work"]["body"]["faultSource"] =
        json!({"id":pin.id,"revision":pin.revision});
    f
}
fn source_for(f: &Fixture, policy: &str) -> Value {
    f.b["policies"][policy]["body"]["faultSource"].clone()
}
fn mismatch(f: &mut Fixture, same_id: bool) {
    let old =
        Pin::decode(&f.b["policies"]["notice_capability_provider"]["body"]["provider"]).unwrap();
    let mut provider: Value =
        serde_json::from_slice(&f.extra.iter().find(|(p, _)| p == &old).unwrap().1).unwrap();
    if !same_id {
        provider["id"] = "urn:test:other-notice-provider".into();
    }
    let pin = f.artifact(provider);
    assert_ne!(pin, old);
    f.b["policies"]["notice_capability_provider"]["body"]["provider"] =
        json!({"id":pin.id,"revision":pin.revision});
}

#[test]
fn general_single_attempt_and_retry_uses_remain_distinct_and_all_composed_stages_pass() {
    let f = fixture();
    let e = f.envelope();
    let r = f.supplied();
    let report = check(&e, &r).unwrap();
    assert_eq!(report.fault_source_uses().len(), 2);
    let row = report
        .fault_source_uses()
        .iter()
        .find(|u| u.site() == "notice_request_step")
        .unwrap();
    assert_eq!(row.revision(), e.document()["revision"]);
    assert_eq!(row.policy(), "notice_work");
    assert_eq!(row.profile().revision, FAULT_SOURCE_PROFILE.1);
    assert_eq!(row.source_metadata().emissions().len(), 1);
    assert!(!row.host_requirements().is_empty());
    assert!(f.b["policies"]["notice_work"]["body"]["retry"]["faultSource"].is_null());
    assert_eq!(
        f.b["policies"]["notice_work"]["body"]["retry"]["faults"],
        json!({})
    );
    let final_report = check_reconciliation_availability(&e, &r).unwrap();
    let timing = final_report.links().exceptional().faults().timing();
    assert_eq!(timing.retry_uses().len(), 1);
    assert_eq!(
        timing
            .retry_uses()
            .iter()
            .next()
            .unwrap()
            .fault_source()
            .unwrap()
            .faults()
            .len(),
        1
    );
    assert!(timing.concurrency().owned_local().is_some());
    let retained = timing
        .concurrency()
        .invalidation()
        .protection()
        .control()
        .static_report();
    assert_eq!(retained.fault_source_uses().len(), 2);
    assert_eq!(retained.capture_bindings().len(), 1);
}

#[test]
fn every_core_source_profile_provider_and_type_pin_is_verified() {
    let f = fixture();
    let e = f.envelope();
    for (pin, _) in &f.extra {
        // The shared raw-capture factory retains two inert small-fixture artifacts.
        if matches!(
            pin.id.as_str(),
            "urn:test:capture-provider" | "urn:test:capture-source"
        ) {
            continue;
        }
        let r: Vec<_> = f
            .supplied()
            .into_iter()
            .filter(|r| !(r.id == pin.id && r.revision == pin.revision))
            .collect();
        assert!(check(&e, &r).is_err(), "missing {}", pin.id);
        let mut g = fixture();
        g.extra
            .iter_mut()
            .find(|(p, _)| p == pin)
            .unwrap()
            .1
            .push(b' ');
        assert!(
            check(&g.envelope(), &g.supplied()).is_err(),
            "corrupt {}",
            pin.id
        );
    }
}

#[test]
fn mismatched_provider_identity_human_and_unvisited_uses_are_not_inferred() {
    for same_id in [false, true] {
        let mut f = fixture();
        mismatch(&mut f, same_id);
        f.refuse("fault-source-provider");
    }
    let mut f = fixture();
    let mut work = f.b["policies"]["notice_work"].clone();
    work["body"]["completion"] = boolean(true);
    work["body"]["reads"] = json!({});
    f.b["policies"]["plain_source_work"] = work;
    let mut n = f.b["nodes"]["notice_request_step"].clone();
    n["mode"] = "human".into();
    n["target"] = "operator_actor".into();
    n["policy"] = "plain_source_work".into();
    n["result"] = Value::Null;
    n["writes"] = json!({});
    f.b["nodes"]["unvisited_human"] = n;
    f.b["flows"]["unvisited_human_done"] =
        json!({"source":"unvisited_human","outcome":"notified","target":"notice_done"});
    f.refuse("fault-source-context-unproved");
    let mut f = fixture();
    let mut work = f.b["policies"]["notice_work"].clone();
    work["body"]["completion"] = boolean(true);
    work["body"]["reads"] = json!({});
    let original = source_for(&f, "notice_work");
    let mut source: Value = serde_json::from_slice(
        &f.extra
            .iter()
            .find(|(p, _)| p.id == original["id"])
            .unwrap()
            .1,
    )
    .unwrap();
    let provider_pin = Pin::decode(&source["provider"]).unwrap();
    let mut provider: Value =
        serde_json::from_slice(&f.extra.iter().find(|(p, _)| p == &provider_pin).unwrap().1)
            .unwrap();
    provider["id"] = "urn:test:unvisited-other-provider".into();
    let pin = f.artifact(provider);
    source["id"] = "urn:test:unvisited-other-source".into();
    source["provider"] = json!({"id":pin.id,"revision":pin.revision});
    let pin = f.artifact(source);
    work["body"]["faultSource"] = json!({"id":pin.id,"revision":pin.revision});
    f.b["policies"]["wrong_unvisited_work"] = work;
    let mut n = f.b["nodes"]["notice_request_step"].clone();
    n["policy"] = "wrong_unvisited_work".into();
    f.b["nodes"]["wrong_unvisited_node"] = n;
    f.b["flows"]["wrong_unvisited_done"] =
        json!({"source":"wrong_unvisited_node","outcome":"notified","target":"notice_done"});
    f.refuse("fault-source-provider");
}

#[test]
fn unused_policies_have_no_use_row_but_unvisited_actual_nodes_share_metadata() {
    let mut f = fixture();
    let mut work = f.b["policies"]["notice_work"].clone();
    work["body"]["completion"] = boolean(true);
    work["body"]["reads"] = json!({});
    f.b["policies"]["extra_source_work"] = work;
    assert_eq!(
        check(&f.envelope(), &f.supplied())
            .unwrap()
            .fault_source_uses()
            .len(),
        2
    );
    let mut n = f.b["nodes"]["notice_request_step"].clone();
    n["policy"] = "extra_source_work".into();
    f.b["nodes"]["unvisited_notice"] = n;
    f.b["flows"]["unvisited_notice_done"] =
        json!({"source":"unvisited_notice","outcome":"notified","target":"notice_done"});
    let report = check(&f.envelope(), &f.supplied()).unwrap();
    let uses: Vec<_> = report
        .fault_source_uses()
        .iter()
        .filter(|u| u.provider().id.contains(":notice:"))
        .collect();
    assert_eq!(uses.len(), 2);
    assert!(std::ptr::eq(
        uses[0].source_metadata(),
        uses[1].source_metadata()
    ));
    let control = check_control(&f.envelope(), &f.supplied()).unwrap();
    assert!(control.uncovered_nodes().contains("unvisited_notice"));
    assert_eq!(control.static_report().fault_source_uses().len(), 3);
    let mut source = source_for(&f, "extra_source_work");
    source["revision"] = format!("sha256:{}", "0".repeat(64)).into();
    f.b["policies"]["extra_source_work"]["body"]["faultSource"] = source;
    f.refuse("resource-missing");
}

#[test]
fn source_agreement_and_legacy_wire_core_boundaries_remain_closed() {
    let mut f = fixture();
    f.b["policies"]["payment_work"]["body"]["faultSource"] = Value::Null;
    f.refuse("fault-source-retry-consistency");
    let mut f = fixture();
    f.b["policies"]["payment_work"]["body"]["faultSource"] = source_for(&f, "notice_work");
    f.refuse("fault-source-retry-consistency");
    let mut f = fixture();
    let source = source_for(&f, "notice_work");
    f.b["policies"]["notice_work"]["body"]["retry"]["faultSource"] = source;
    f.refuse("retry-fault-source-unused");
    let mut f = fixture();
    for n in f.b["nodes"].as_object_mut().unwrap().values_mut() {
        if n["kind"] == "wait" {
            n.as_object_mut().unwrap().remove("capture");
        }
    }
    let e = DefinitionEnvelope::from_parts(f.b.clone(), json!({})).unwrap();
    assert_eq!(
        check(&e, &f.supplied()).unwrap_err().code,
        "capture-wire-version"
    );
    let mut f = fixture();
    f.b["dialects"]["core"] = json!({"id":CORE_V5.0,"revision":CORE_V5.1});
    f.refuse("fields");
    let mut f = fixture();
    f.b["dialects"]["legacy"] = json!({"id":CORE_V5.0,"revision":CORE_V5.1});
    f.b["policies"]["notice_capability_provider"]["dialect"] = "legacy".into();
    check(&f.envelope(), &f.supplied()).unwrap();
    f.b["dialects"]["legacy"] = json!({"id":CORE_V2.0,"revision":CORE_V2.1});
    f.refuse("policy-cycle");
}

#[test]
fn general_catalogue_adds_noneligible_names_without_actual_emission_or_handler_data() {
    let mut f = fixture();
    let with = check_fault_routes(&f.envelope(), &f.supplied()).unwrap();
    f.b["policies"]["notice_work"]["body"]["faultSource"] = Value::Null;
    let without = check_fault_routes(&f.envelope(), &f.supplied()).unwrap();
    assert_eq!(
        with.unhandled_cases(),
        without.unhandled_cases() + with.checked_scopes().len()
    );
    assert_eq!(with.routes().count(), without.routes().count());
    assert!(
        with.remaining()
            .contains(&"authenticated-fault-source-occurrence-cause-and-input-revisions")
    );
}

#[test]
fn unused_source_metadata_dependencies_are_still_complete_and_budgeted() {
    let mut f = fixture();
    let mut p = f.b["policies"]["notice_work"].clone();
    p["body"]["completion"] = boolean(true);
    p["body"]["reads"] = json!({});
    f.b["policies"]["source_only_work"] = p;
    let mut provider: Value = serde_json::from_slice(
        &f.extra
            .iter()
            .find(|(p, _)| p.id == "urn:choreoform:benchmarks:rp01:notice:0.1.0")
            .unwrap()
            .1,
    )
    .unwrap();
    provider["id"] = "urn:test:metadata-only-provider".into();
    provider["resultProtection"]["policy"] = "source_permission".into();
    f.b["policies"]["source_permission"] = policy(
        "protection",
        json!({"permit":{"op":"eq","left":{"op":"read","cell":"current_claim"},"right":{"op":"read","cell":"current_claim"}}}),
    );
    f.b["policies"]["source_permission"]["scope"] = "expense".into();
    f.b["policies"]["source_permission"]["body"]["reads"] = json!({"current_claim":true});
    let pin = f.artifact(provider);
    let source = json!({"kind":"provider-fault-source","version":"0.1.0","id":"urn:test:metadata-only-faults","profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":{"id":pin.id,"revision":pin.revision},"emissions":{"unknown":{"observations":{"unknown":true},"evidence":"unresolved"}}});
    let pin = f.artifact(source);
    f.b["policies"]["source_only_work"]["body"]["faultSource"] =
        json!({"id":pin.id,"revision":pin.revision});
    f.refuse("policy-reads");
    f.b["policies"]["source_only_work"]["body"]["reads"] = json!({"current_claim":true});
    assert_eq!(
        check(&f.envelope(), &f.supplied())
            .unwrap()
            .fault_source_uses()
            .len(),
        2
    );
}

#[test]
fn shared_finite_general_source_vectors_match_real_raw_sdk_judgments() {
    let corpus: Value =
        serde_json::from_slice(include_bytes!("../conformance/general-fault-use.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let mut f = fixture();
        let policy = if case["attempts"] == 1 {
            "notice_work"
        } else {
            "payment_work"
        };
        let original = source_for(&f, policy);
        f.b["policies"][policy]["body"]["faultSource"] = match case["outer"].as_str().unwrap() {
            "none" => Value::Null,
            "other" => source_for(&f, "notice_work"),
            _ => original.clone(),
        };
        f.b["policies"][policy]["body"]["retry"]["faultSource"] = if case["retrySource"] == true {
            original.clone()
        } else {
            Value::Null
        };
        if case["mode"] == "human" {
            let n = &mut f.b["nodes"]["notice_request_step"];
            n["mode"] = "human".into();
            n["target"] = "operator_actor".into();
            n["result"] = Value::Null;
            n["writes"] = json!({});
            f.b["policies"][policy]["body"]["completion"] = boolean(true);
            f.b["policies"][policy]["body"]["reads"] = json!({});
        }
        if case["provider"] != "same" {
            mismatch(&mut f, case["provider"] == "same-id-new-revision");
        }
        if case["resources"] == false {
            f.extra.retain(|(p, _)| p.id != original["id"]);
        }
        if case["legacy"] == true {
            f.b["dialects"]["core"] = json!({"id":CORE_V5.0,"revision":CORE_V5.1});
        }
        if let Some(name) = case["eligible"].as_str() {
            f.b["policies"][policy]["body"]["retry"]["faults"] = json!({name:true});
        }
        let got = match check(&f.envelope(), &f.supplied()) {
            Ok(_) => "passed".to_owned(),
            Err(e) => e.code.into(),
        };
        assert_eq!(got, case["expect"].as_str().unwrap(), "{}", case["id"]);
    }
}
