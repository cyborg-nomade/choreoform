// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Synthetic contract probes only; no production permission or live new work.
use super::*;
use choreoform_semantic_check::{CORE_V6, CORE_V7, CORE_V8, check_concurrency};
fn env(f: &Fixture) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.4.0", f.b.clone(), json!({})).unwrap()
}
pub(super) fn refresh(f: &mut Fixture) {
    for scope in ["recovery", "lookup_recovery"] {
        let nodes: serde_json::Map<String, Value> = f.b["nodes"]
            .as_object()
            .unwrap()
            .iter()
            .filter(|(_, n)| n["scope"] == scope)
            .map(|(id, _)| (id.clone(), true.into()))
            .collect();
        for d in f.b["data"].as_object_mut().unwrap().values_mut() {
            if d["scope"] == scope {
                d["invalidates"] = nodes.clone().into();
            }
        }
    }
}
fn baseline() -> Fixture {
    let mut f = Fixture::new();
    for p in [CORE_V6, CORE_V7, CORE_V8] {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        f.extra.push((
            Pin {
                id: p.0.into(),
                revision: p.1.into(),
            },
            std::fs::read(root.join(format!("docs/ir/contracts/{}.txt", p.1.replace(':', "-"))))
                .unwrap(),
        ));
    }
    f.b["dialects"]["core"] = json!({"id":CORE_V8.0,"revision":CORE_V8.1});
    for s in f.b["scopes"].as_object_mut().unwrap().values_mut() {
        s["faultInput"] = Value::Null;
        s["retainedRequests"] = json!({});
    }
    for p in f.b["policies"].as_object_mut().unwrap().values_mut() {
        p["scope"] = "main".into();
        if p["body"]["kind"] == "work" {
            p["body"]["faultSource"] = Value::Null;
        }
    }
    f.b["policies"]["keep"]["scope"] = "root".into();
    for a in f.b["actors"].as_object_mut().unwrap().values_mut() {
        a["scope"] = "main".into();
    }
    for c in f.b["capabilities"].as_object_mut().unwrap().values_mut() {
        c["scope"] = "main".into();
    }
    let mut main = f.b["scopes"]["root"].clone();
    main["entry"] = "run_owner".into();
    main["closure"] = "close".into();
    f.b["scopes"]["main"] = main;
    f.b["root"] = "main".into();
    f.b["scopes"]["root"]["parent"] = "main".into();
    f.b["nodes"]["run_owner"] = json!({"kind":"invoke","scope":"main","body":"root","inputs":{},"outputs":{},"reads":{},"writes":{},"outcomes":{"complete":true}});
    f.b["nodes"]["main_done"] = json!({"kind":"finish","scope":"main","outcome":"complete","outcomes":{},"reads":{},"writes":{}});
    f.b["flows"]["main_next"] =
        json!({"source":"run_owner","outcome":"complete","target":"main_done"});
    f.b["nodes"]["owner_done"] = json!({"kind":"finish","scope":"root","outcome":"complete","outcomes":{},"reads":{},"writes":{}});
    f.b["flows"]["origin_next"]["target"] = "owner_done".into();
    for id in ["capture_wait", "read_capture", "done"] {
        f.b["nodes"][id]["scope"] = "recovery".into();
    }
    for id in ["recover_wait", "recover_done"] {
        f.b["nodes"].as_object_mut().unwrap().remove(id);
    }
    f.b["flows"].as_object_mut().unwrap().remove("recover_next");
    f.b["scopes"]["recovery"]["entry"] = "capture_wait".into();
    f.b["data"]["captured"]["scope"] = "recovery".into();
    f.b["expressions"]["capture_is_confirmed"]["scope"] = "recovery".into();
    f.b["policies"]["effects"]["body"]["reconciliation"] = "ordinary_wait".into();
    // Fresh synthetic provider/resource pins: accepted fixture/artifact files stay fixed.
    f.provider["id"] = "urn:test:continuation-audit-provider".into();
    f.provider["resultProtection"]["capabilities"] = json!({"adapter":true,"adapter_lookup":true});
    let p = f.artifact(f.provider.clone());
    f.b["policies"]["provider_policy"]["body"]["provider"] =
        json!({"id":p.id,"revision":p.revision});
    f.source["id"] = "urn:test:continuation-audit-terminal".into();
    f.source["provider"] = json!({"id":p.id,"revision":p.revision});
    f.bind_source();
    let mut lookup = f.b["capabilities"]["adapter"].clone();
    lookup["effects"] = "lookup_effects".into();
    f.b["capabilities"]["adapter_lookup"] = lookup;
    // Its unused effect policy still has an actual declared wait user.
    let mut effect = f.b["policies"]["effects"].clone();
    effect["body"]["reconciliation"] = "recovery_wait".into();
    f.b["policies"]["lookup_effects"] = effect;
    f.b["nodes"]["unused_lookup_wait"] = json!({"kind":"wait","scope":"recovery","policy":"recovery_wait","capture":null,"reads":{},"writes":{},"outcomes":{"observed":true}});
    f.b["flows"]["unused_lookup_next"] =
        json!({"source":"unused_lookup_wait","outcome":"observed","target":"done"});
    for d in f.b["data"].as_object_mut().unwrap().values_mut() {
        d["protection"]["capabilities"] = json!({"adapter":true,"adapter_lookup":true});
    }
    let mut snapshot = f.b["data"]["captured"].clone();
    snapshot["type"] = wrap(json!({"kind":"record","fields":{}}));
    f.b["data"]["original_request"] = snapshot;
    f.b["scopes"]["recovery"]["retainedRequests"] = json!({"original_request":"origin"});
    refresh(&mut f);
    f
}
pub(super) fn flat_query() -> Fixture {
    let mut f = baseline();
    f.b["expressions"]["lookup_input"] = json!({"scope":"recovery","dialect":"core","parameters":{},"reads":{"original_request":true},"resultType":wrap(json!({"kind":"record","fields":{}})),"body":{"op":"read","cell":"original_request"}});
    let mut result = f.b["data"]["original_request"].clone();
    result["type"] = wrap(json!({"kind":"integer"}));
    f.b["data"]["lookup_result"] = result;
    f.b["nodes"]["lookup"] = json!({"kind":"activity","mode":"capability","scope":"recovery","target":"adapter_lookup","policy":"work","input":"lookup_input","result":"lookup_result","reads":{"original_request":true},"writes":{"lookup_result":true},"outcomes":{"done":true}});
    f.b["flows"]["read_yes"]["target"] = "lookup".into();
    f.b["flows"]["lookup_next"] = json!({"source":"lookup","outcome":"done","target":"done"});
    refresh(&mut f);
    f
}
fn pure_copy(f: &mut Fixture) {
    f.b["data"]["lookup_result"]["type"] = wrap(json!({"kind":"record","fields":{}}));
    f.b["nodes"]["lookup"] = json!({"kind":"compute","scope":"recovery","reads":{"original_request":true},"writes":{"lookup_result":true},"assignments":{"lookup_result":"lookup_input"},"outcomes":{"next":true}});
    f.b["flows"]["lookup_next"]["outcome"] = "next".into();
    refresh(f);
}
#[test]
fn baseline_is_a_real_complete_composed_sdk_pass_without_new_work() {
    let f = baseline();
    let r = check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
    assert!(r.retained_request_scopes().contains("recovery"));
}
#[test]
fn flat_readonly_work_passes_earlier_stages_but_has_no_owned_lifetime_proof() {
    let f = flat_query();
    let e = env(&f);
    let supplied = f.supplied();
    check(&e, &supplied).unwrap();
    let control = check_control(&e, &supplied).unwrap();
    assert!(!control.covered_nodes().contains("lookup"));
    check_protection(&e, &supplied).unwrap();
    check_invalidation(&e, &supplied).unwrap();
    let error = check_concurrency(&e, &supplied).unwrap_err();
    assert_eq!(error.code, "scope-lifetime-unproved");
    assert_eq!(error.path, "/body/nodes/run_owner");
    assert_eq!(f.b["scopes"]["recovery"]["closure"], "close");
    assert_eq!(f.b["nodes"]["unused_lookup_wait"]["scope"], "recovery");
}
#[test]
fn nested_reconciliation_is_not_the_existing_flat_retention_contract() {
    let mut f = flat_query();
    let mut child = f.b["scopes"]["recovery"].clone();
    child["parent"] = "recovery".into();
    child["entry"] = "unused_lookup_wait".into();
    child["retainedRequests"] = json!({});
    f.b["scopes"]["lookup_recovery"] = child;
    f.b["nodes"]["unused_lookup_wait"]["scope"] = "lookup_recovery".into();
    let mut finish = f.b["nodes"]["done"].clone();
    finish["scope"] = "lookup_recovery".into();
    f.b["nodes"]["lookup_done"] = finish;
    f.b["flows"]["unused_lookup_next"]["target"] = "lookup_done".into();
    f.b["policies"]["lookup_keep"] = json!({"scope":"recovery","dialect":"core","body":{"kind":"closure","reads":{},"mode":"reconcile","followup":"lookup_recovery"}});
    f.b["scopes"]["recovery"]["closure"] = "lookup_keep".into();
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "retained-request-creator"
    );
}
#[test]
fn a_pure_copy_without_the_declared_snapshot_cannot_invent_its_value() {
    let mut f = flat_query();
    pure_copy(&mut f);
    check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
    f.b["scopes"]["recovery"]["retainedRequests"] = json!({});
    assert_eq!(
        check_concurrency(&env(&f), &f.supplied()).unwrap_err().code,
        "owned-local-availability-unproved"
    );
}
#[test]
fn an_ancestor_value_is_not_a_surviving_storage_shortcut() {
    let mut f = flat_query();
    pure_copy(&mut f);
    f.b["expressions"]["lookup_input"]["reads"] = json!({"origin_value":true});
    f.b["expressions"]["lookup_input"]["body"] = json!({"op":"read","cell":"origin_value"});
    f.b["expressions"]["lookup_input"]["resultType"] = wrap(json!({"kind":"integer"}));
    f.b["data"]["lookup_result"]["type"] = wrap(json!({"kind":"integer"}));
    f.b["nodes"]["lookup"]["reads"] = json!({"origin_value":true});
    f.b["data"]["origin_value"]["invalidates"]["lookup"] = true.into();
    check(&env(&f), &f.supplied()).unwrap();
    assert_eq!(
        check_concurrency(&env(&f), &f.supplied()).unwrap_err().code,
        "scope-lifetime-unproved"
    );
}
#[test]
fn ordinary_parent_work_has_no_late_followup_resumption_edge() {
    let mut f = baseline();
    let mut cell = f.b["data"]["origin_value"].clone();
    cell["invalidates"] = json!({});
    f.b["data"]["parent_copy_result"] = cell;
    f.b["expressions"]["parent_copy_expr"] = json!({"scope":"root","dialect":"core","parameters":{},"reads":{"origin_value":true},"resultType":wrap(json!({"kind":"integer"})),"body":{"op":"read","cell":"origin_value"}});
    f.b["nodes"]["parent_copy"] = json!({"kind":"compute","scope":"root","reads":{"origin_value":true},"writes":{"parent_copy_result":true},"assignments":{"parent_copy_result":"parent_copy_expr"},"outcomes":{"next":true}});
    f.b["flows"]["origin_next"]["target"] = "parent_copy".into();
    f.b["flows"]["parent_copy_next"] =
        json!({"source":"parent_copy","outcome":"next","target":"owner_done"});
    f.b["data"]["origin_value"]["invalidates"]["parent_copy"] = true.into();
    let e = env(&f);
    let r = check_reconciliation_availability(&e, &f.supplied()).unwrap();
    assert!(
        r.links()
            .exceptional()
            .faults()
            .timing()
            .concurrency()
            .invalidation()
            .protection()
            .control()
            .covered_nodes()
            .contains("parent_copy")
    );
    assert_eq!(f.b["nodes"]["parent_copy"]["scope"], "root");
    for flow in f.b["flows"].as_object().unwrap().values() {
        assert_eq!(
            f.b["nodes"][flow["source"].as_str().unwrap()]["scope"],
            f.b["nodes"][flow["target"].as_str().unwrap()]["scope"]
        );
    }
}
#[test]
fn an_irreversible_provider_is_not_covered_by_a_readonly_label_or_true_completion() {
    let mut f = flat_query();
    let mut provider = f.provider.clone();
    provider["id"] = "urn:test:continuation-audit-command".into();
    provider["effectClass"] = "irreversible".into();
    provider["idempotency"] = "stable-key".into();
    let p = f.artifact(provider);
    let mut policy = f.b["policies"]["provider_policy"].clone();
    policy["body"]["provider"] = json!({"id":p.id,"revision":p.revision});
    f.b["policies"]["lookup_provider"] = policy;
    f.b["capabilities"]["adapter_lookup"]["contract"] = "lookup_provider".into();
    f.b["policies"]["lookup_effects"]["body"]["class"] = "irreversible".into();
    f.b["policies"]["lookup_effects"]["body"]["idempotency"] = "stable-key".into();

    let e = env(&f);
    check(&e, &f.supplied()).unwrap();
    assert_eq!(
        check_concurrency(&e, &f.supplied()).unwrap_err().code,
        "scope-lifetime-unproved"
    );
    assert_eq!(
        f.b["policies"]["lookup_effects"]["body"]["class"],
        "irreversible"
    );
    assert_eq!(f.b["policies"]["work"]["body"]["completion"], boolean(true));
}
