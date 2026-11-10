// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Experimental full-case definitions, never accepted observations or adoption.
use super::*;
use choreoform_semantic_check::{
    CORE_V6, CORE_V7, CORE_V8, CORE_V9, CORE_V10, CORE_V11, check_concurrency,
};

fn fixture() -> Fixture {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let original: Value = serde_json::from_slice(
        &std::fs::read(root.join("docs/benchmarks/rp01/candidate.ir.json")).unwrap(),
    )
    .unwrap();
    let manifest: Value = serde_json::from_slice(
        &std::fs::read(root.join("docs/benchmarks/rp01/manifest.json")).unwrap(),
    )
    .unwrap();
    let mut b = original["body"].clone();
    b["dialects"]["core"] = json!({"id":CORE_V11.0,"revision":CORE_V11.1});
    for scope in b["scopes"].as_object_mut().unwrap().values_mut() {
        scope["faultInput"] = Value::Null;
        scope["retainedRequests"] = json!({});
    }
    for n in b["nodes"].as_object_mut().unwrap().values_mut() {
        if n["kind"] == "wait" {
            n["capture"] = Value::Null;
        }
    }
    for p in b["policies"].as_object_mut().unwrap().values_mut() {
        if p["body"]["kind"] == "work" {
            p["body"]["faultSource"] = p["body"]["retry"]["faultSource"].clone();
        }
    }
    let mut extra: Vec<_> = manifest["resources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                Pin {
                    id: r["id"].as_str().unwrap().into(),
                    revision: r["revision"].as_str().unwrap().into(),
                },
                std::fs::read(root.join(r["path"].as_str().unwrap())).unwrap(),
            )
        })
        .collect();
    for pin in [
        SEMANTICS,
        CORE,
        CORE_V2,
        CORE_V3,
        CORE_V4,
        CORE_V5,
        CORE_V6,
        CORE_V7,
        CORE_V8,
        CORE_V9,
        CORE_V10,
        CORE_V11,
        TERMINAL_OBSERVATION_PROFILE,
    ] {
        if !extra
            .iter()
            .any(|(p, _)| p.id == pin.0 && p.revision == pin.1)
        {
            extra.push((
                Pin {
                    id: pin.0.into(),
                    revision: pin.1.into(),
                },
                std::fs::read(
                    root.join(format!("docs/ir/contracts/{}.txt", pin.1.replace(':', "-"))),
                )
                .unwrap(),
            ));
        }
    }
    let payment = extra
        .iter()
        .find(|(p, _)| p.id == "urn:choreoform:benchmarks:rp01:payment:0.1.0")
        .unwrap();
    let provider = serde_json::from_slice(&payment.1).unwrap();
    let source = json!({"kind":"provider-terminal-observation-source","version":"0.1.0","id":"urn:test:rp01-current-payment-terminal","profile":{"id":TERMINAL_OBSERVATION_PROFILE.0,"revision":TERMINAL_OBSERVATION_PROFILE.1},"provider":{"id":payment.0.id,"revision":payment.0.revision}});
    Fixture {
        b,
        provider,
        source,
        extra,
    }
}
fn capture(mut f: Fixture, retain: bool) -> Fixture {
    let pin = f.artifact(f.source.clone());
    let mut cell = f.b["data"]["payment_result"].clone();
    cell["scope"] = "payment_recovery".into();
    cell["type"] = wrap(f.b["policies"]["payment_wait"]["body"]["observationType"].clone());
    cell["invalidates"] = json!({"payment_observe":true,"payment_recovered":true});
    f.b["data"]["audit_payment_capture"] = cell;
    f.b["nodes"]["payment_observe"]["capture"] = json!({"origin":"payment_request_step","result":"audit_payment_capture","source":{"id":pin.id,"revision":pin.revision}});
    f.b["nodes"]["payment_observe"]["writes"] = json!({"audit_payment_capture":true});
    if retain {
        let mut cell = f.b["data"]["payment_request"].clone();
        cell["scope"] = "payment_recovery".into();
        cell["invalidates"] = json!({"payment_observe":true,"payment_recovered":true});
        f.b["data"]["audit_payment_request"] = cell;
        f.b["scopes"]["payment_recovery"]["retainedRequests"] =
            json!({"audit_payment_request":"payment_request_step"});
    }
    f
}
fn env(f: &Fixture) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.4.0", f.b.clone(), json!({})).unwrap()
}
#[test]
fn whole_rp01_neutral_profile_promotion_keeps_ports_ancestor_reads_and_partial_boundaries() {
    let f = fixture();
    let r = check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
    assert!(
        f.b["nodes"]["payment_request_step"]["reads"]
            .get("current_claim")
            .is_some()
    );
    assert_eq!(f.b["data"]["current_claim"]["scope"], "expense");
    assert!(
        !f.b["scopes"]["payment_service"]["inputs"]
            .as_object()
            .unwrap()
            .is_empty()
    );
    let c = r.links().exceptional().faults().timing().concurrency();
    assert_eq!(c.isolated_followups().len(), 3);
    assert!(c.owned_local().is_none());
}
#[test]
fn whole_payment_capture_and_exact_nominal_request_pass_without_metadata_broadening() {
    let f = capture(fixture(), true);
    let r = check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
    let c = r.links().exceptional().faults().timing().concurrency();
    let owned = c.owned_local().unwrap();
    assert_eq!(owned.followups().len(), 1);
    let followup = owned.followups().iter().next().unwrap();
    assert_eq!(followup.body(), "payment_service");
    assert_eq!(followup.followup(), "payment_recovery");
    assert_eq!(followup.local_cells().len(), 2);
    assert_eq!(
        followup.request_snapshot().unwrap().origin(),
        "payment_request_step"
    );
    assert!(
        owned
            .required_provenance()
            .iter()
            .any(|p| p.site() == "current_claim" && p.owner() == "payment_service")
    );
    assert!(r.retained_request_scopes().contains("payment_recovery"));
    assert!(r.uncovered_nodes().contains("payment_observe"));
    assert!(r.continuation_scopes().is_empty());
    assert!(c.owned_continuations().is_none());
}
#[test]
fn whole_late_classification_matches_original_receipt_and_never_writes_parent() {
    let mut f = capture(fixture(), true);
    let corpus: Value = serde_json::from_slice(include_bytes!(
        "../conformance/rp01-late-classification.json"
    ))
    .unwrap();
    let expr = corpus["expression"].clone();
    f.b["expressions"]["audit_classify_assignment"] = json!({"scope":"payment_recovery","dialect":"core","parameters":{},"reads":{"audit_payment_capture":true,"audit_payment_request":true},"resultType":wrap(json!({"kind":"text"})),"body":expr});
    let mut cell = f.b["data"]["audit_payment_capture"].clone();
    cell["type"] = wrap(json!({"kind":"text"}));
    f.b["data"]["audit_late_category"] = cell;
    f.b["nodes"]["audit_classify_payment"] = json!({"kind":"compute","scope":"payment_recovery","reads":{"audit_payment_capture":true,"audit_payment_request":true},"writes":{"audit_late_category":true},"assignments":{"audit_late_category":"audit_classify_assignment"},"outcomes":{"next":true}});
    for flow in f.b["flows"].as_object_mut().unwrap().values_mut() {
        if flow["source"] == "payment_observe" {
            flow["target"] = "audit_classify_payment".into();
        }
    }
    f.b["flows"]["audit_classify_flow"] =
        json!({"source":"audit_classify_payment","outcome":"next","target":"payment_recovered"});
    for cell in [
        "audit_payment_capture",
        "audit_payment_request",
        "audit_late_category",
    ] {
        f.b["data"][cell]["invalidates"]["audit_classify_payment"] = true.into();
    }
    let r = check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
    assert!(
        r.links()
            .exceptional()
            .faults()
            .timing()
            .concurrency()
            .owned_local()
            .unwrap()
            .checked_nodes()
            .contains("audit_classify_payment")
    );
    assert!(r.uncovered_nodes().contains("audit_classify_payment"));
    assert_eq!(
        f.b["nodes"]["audit_classify_payment"]["writes"],
        json!({"audit_late_category":true})
    );
}
#[test]
fn finite_active_work_still_rejects_ordinary_root_ancestor_reads_and_writes() {
    for write in [false, true] {
        let mut f = super::finite_work::fixture("read-only", false, true);
        let mut cell = f.b["data"]["origin_value"].clone();
        cell["scope"] = "main".into();
        cell["initial"] = "parent_initial".into();
        cell["invalidates"] = json!({"origin":true});
        f.b["data"]["parent_value"] = cell;
        f.b["expressions"]["parent_initial"] = json!({"scope":"main","dialect":"core","parameters":{},"reads":{},"resultType":wrap(json!({"kind":"integer"})),"body":{"op":"literal","type":{"kind":"integer"},"value":"1"}});
        if write {
            f.b["nodes"]["origin"]["result"] = "parent_value".into();
            f.b["nodes"]["origin"]["writes"] = json!({"parent_value":true});
        } else {
            f.b["nodes"]["origin"]["reads"]["parent_value"] = true.into();
        }
        check(&env(&f), &f.supplied()).unwrap();
        assert!(
            matches!(check_concurrency(&env(&f),&f.supplied()),Err(e) if e.code=="owned-continuation-root-storage-unproved")
        );
    }
}
