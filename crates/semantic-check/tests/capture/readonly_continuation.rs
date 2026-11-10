// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Finite conditional scopes, never actual source values or accepted host gates.
use super::*;
use choreoform_semantic_check::{CORE_V9, FAULT_SOURCE_PROFILE, check_concurrency};
pub(super) fn fixture() -> Fixture {
    let mut f = super::continuation_audit::flat_query();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for p in [CORE_V9, FAULT_SOURCE_PROFILE] {
        f.extra.push((
            Pin {
                id: p.0.into(),
                revision: p.1.into(),
            },
            std::fs::read(root.join(format!("docs/ir/contracts/{}.txt", p.1.replace(':', "-"))))
                .unwrap(),
        ));
    }
    f.b["dialects"]["core"] = json!({"id":CORE_V9.0,"revision":CORE_V9.1});
    let mut child = f.b["scopes"]["recovery"].clone();
    child["parent"] = "recovery".into();
    child["entry"] = "unused_lookup_wait".into();
    child["retainedRequests"] = json!({"lookup_request":"lookup"});
    f.b["scopes"]["lookup_recovery"] = child;
    f.b["nodes"]["unused_lookup_wait"]["scope"] = "lookup_recovery".into();
    f.b["nodes"]["unused_lookup_wait"]["capture"] = json!({"source":f.b["nodes"]["capture_wait"]["capture"]["source"],"origin":"lookup","result":"lookup_captured"});
    f.b["nodes"]["unused_lookup_wait"]["writes"] = json!({"lookup_captured":true});
    let mut finish = f.b["nodes"]["done"].clone();
    finish["scope"] = "lookup_recovery".into();
    f.b["nodes"]["lookup_done"] = finish;
    f.b["flows"]["unused_lookup_next"]["target"] = "lookup_branch".into();
    let mut branch = f.b["nodes"]["read_capture"].clone();
    branch["scope"] = "lookup_recovery".into();
    branch["reads"] = json!({"lookup_captured":true});
    branch["guards"]["yes"] = "lookup_confirmed".into();
    f.b["nodes"]["lookup_branch"] = branch;
    let mut guard = f.b["expressions"]["capture_is_confirmed"].clone();
    guard["scope"] = "lookup_recovery".into();
    guard["reads"] = json!({"lookup_captured":true});
    guard["body"] = serde_json::from_str(
        &guard["body"]
            .to_string()
            .replace("\"captured\"", "\"lookup_captured\""),
    )
    .unwrap();
    f.b["expressions"]["lookup_confirmed"] = guard;
    for outcome in ["yes", "no"] {
        f.b["flows"][format!("lookup_branch_{outcome}")] =
            json!({"source":"lookup_branch","outcome":outcome,"target":"lookup_done"});
    }
    f.b["policies"]["lookup_keep"] = json!({"scope":"recovery","dialect":"core","body":{"kind":"closure","reads":{},"mode":"reconcile","followup":"lookup_recovery"}});
    f.b["scopes"]["recovery"]["closure"] = "lookup_keep".into();
    let mut request = f.b["data"]["original_request"].clone();
    request["scope"] = "lookup_recovery".into();
    f.b["data"]["lookup_request"] = request;
    let mut captured = f.b["data"]["captured"].clone();
    captured["scope"] = "lookup_recovery".into();
    f.b["data"]["lookup_captured"] = captured;
    let source = json!({"kind":"provider-fault-source","version":"0.1.0","id":"urn:test:owned-query-faults","profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":f.b["policies"]["provider_policy"]["body"]["provider"],"emissions":{"lookup uncertainty":{"observations":{"unknown":true},"evidence":"unresolved"}}});
    let p = f.artifact(source);
    let mut work = f.b["policies"]["work"].clone();
    work["body"]["faultSource"] = json!({"id":p.id,"revision":p.revision});
    f.b["policies"]["lookup_work"] = work;
    f.b["nodes"]["lookup"]["policy"] = "lookup_work".into();
    super::continuation_audit::refresh(&mut f);
    f
}
fn env(f: &Fixture) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.4.0", f.b.clone(), json!({})).unwrap()
}
#[test]
fn nested_query_source_and_original_requests_have_distinct_conditional_proof() {
    let f = fixture();
    let e = env(&f);
    let supplied = f.supplied();
    let declared = check(&e, &supplied).unwrap();
    assert_eq!(declared.retained_requests().len(), 2);
    let c = check_concurrency(&e, &supplied).unwrap();
    assert!(c.owned_local().is_none());
    let tree = c.owned_continuations().unwrap();
    assert_eq!(tree.revision(), e.document()["revision"]);
    assert_eq!(
        tree.checked_scopes(),
        &std::collections::BTreeSet::from(["recovery".into(), "lookup_recovery".into()])
    );
    assert!(tree.checked_nodes().contains("lookup"));
    assert!(
        !c.invalidation()
            .protection()
            .control()
            .covered_nodes()
            .contains("lookup")
    );
    assert!(!tree.gates().is_empty());
    assert!(!tree.remaining().is_empty());
    let root = tree.continuations().iter().next().unwrap();
    assert_eq!(root.creator(), "run_owner");
    let scopes: Vec<_> = root.scopes().collect();
    assert_eq!(scopes.len(), 2);
    assert!(!scopes[0].conditional_source());
    assert!(scopes[1].conditional_source());
    assert_eq!(scopes[1].source(), "lookup");
    let r = check_reconciliation_availability(&e, &supplied).unwrap();
    assert_eq!(r.continuation_scopes(), tree.checked_scopes());
    assert!(r.retained_request_scopes().is_empty());
    assert!(!r.checked_nodes().contains("lookup"));
    assert!(r.uncovered_nodes().contains("lookup"));
}
fn refuse(f: &Fixture, code: &str) {
    match check_concurrency(&env(f), &f.supplied()) {
        Err(error) => assert_eq!(error.code, code, "{error:?}"),
        Ok(_) => panic!("expected refusal {code}"),
    }
}
#[test]
fn exact_new_core_all_bases_and_uniform_aliases_are_mandatory() {
    use choreoform_semantic_check::CORE_V8;
    let f = fixture();
    for (p, _) in f
        .extra
        .iter()
        .filter(|(p, _)| p.id.contains("core-profile"))
    {
        let supplied: Vec<_> = f.supplied().into_iter().filter(|r| r.id != p.id).collect();
        assert!(check(&env(&f), &supplied).is_err(), "{}", p.id);
    }
    let mut f = fixture();
    f.b["dialects"]["old"] = json!({"id":CORE_V8.0,"revision":CORE_V8.1});
    refuse(&f, "retained-request-wire-version");
    let mut f = fixture();
    f.b["dialects"]["core"] = json!({"id":CORE_V8.0,"revision":CORE_V8.1});
    refuse(&f, "retained-request-creator");
    let mut f = fixture();
    f.b["dialects"]["alias"] = f.b["dialects"]["core"].clone();
    check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
}
#[test]
fn nested_sources_need_conditional_coverage_and_root_sources_need_ordinary_coverage() {
    let mut f = fixture();
    f.b["flows"]["read_yes"]["target"] = "done".into();
    refuse(&f, "owned-continuation-source-unproved");
    let mut f = fixture();
    f.b["scopes"]["root"]["entry"] = "owner_done".into();
    refuse(&f, "owned-continuation-source-unproved");
}
#[test]
fn snapshot_seed_permissions_cannot_read_self_peer_or_ancestor_values() {
    for (scope, cell, read) in [
        ("recovery", "original_request", "original_request"),
        ("recovery", "original_request", "captured"),
        ("lookup_recovery", "lookup_request", "lookup_request"),
        ("lookup_recovery", "lookup_request", "lookup_captured"),
        ("lookup_recovery", "lookup_request", "original_request"),
    ] {
        let mut f = fixture();
        f.b["policies"]["seed_access"] = json!({"scope":scope,"dialect":"core","body":{"kind":"protection","reads":{read:true},"permit":{"op":"eq","left":{"op":"read","cell":read},"right":{"op":"read","cell":read}}}});
        f.b["data"][cell]["protection"]["policy"] = "seed_access".into();
        for n in f.b["nodes"].as_object_mut().unwrap().values_mut() {
            if n["reads"].get(cell).is_some() {
                n["reads"][read] = true.into();
            }
        }
        refuse(
            &f,
            // The root query's direct input/provider check rejects changed
            // protection identity before conditional entry analysis begins.
            if scope == "recovery" {
                "information-flow"
            } else {
                "owned-continuation-access-precheck"
            },
        );
    }
    let mut f = fixture();
    f.b["expressions"]["lookup_input"]["scope"] = "recovery".into();
    f.b["expressions"]["lookup_input"]["body"] = json!({"op":"record","fields":{}});
    f.b["expressions"]["lookup_input"]["reads"] = json!({});
    f.b["nodes"]["lookup"]["reads"] = json!({});
    refuse(&f, "owned-continuation-query-input");
}
#[test]
fn source_control_labels_protect_snapshot_branch_and_constant_initializers() {
    for cell in ["original_request", "lookup_request", "lookup_captured"] {
        let mut f = fixture();
        f.b["data"][cell]["protection"]["sensitivity"] = "public".into();
        refuse(
            &f,
            if cell == "lookup_request" {
                "control-information-flow"
            } else {
                "information-flow"
            },
        );
    }
    let mut f = fixture();
    let mut cell = f.b["data"]["lookup_captured"].clone();
    cell["type"] = wrap(json!({"kind":"boolean"}));
    cell["protection"]["sensitivity"] = "public".into();
    cell["initial"] = "public_flag".into();
    f.b["data"]["flag"] = cell;
    f.b["expressions"]["public_flag"] = json!({"scope":"lookup_recovery","dialect":"core","parameters":{},"reads":{},"resultType":wrap(json!({"kind":"boolean"})),"body":boolean(true)});
    refuse(&f, "control-information-flow");
}
#[test]
fn query_provider_egress_needs_the_entire_source_envelope() {
    let mut f = fixture();
    f.b["data"]["original_request"]["protection"]["capabilities"] = json!({"adapter":true});
    refuse(&f, "information-flow");
    // Input publication remains valid here. Only the captured branch context
    // is narrower, so the conditional provider egress must preserve its label.
    let mut f = fixture();
    f.b["data"]["captured"]["protection"]["capabilities"] = json!({"adapter":true});
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-recipient");
}
pub(super) fn no_result_fixture() -> Fixture {
    let mut f = fixture();
    let empty = json!({"kind":"record","fields":{}});
    let mut provider = f.provider.clone();
    provider["id"] = "urn:test:empty-query-provider".into();
    provider["output"] = empty.clone();
    provider["observations"]["done"]["result"] = empty.clone();
    let pin = f.artifact(provider);
    let provider = json!({"id":pin.id,"revision":pin.revision});
    let mut policy = f.b["policies"]["provider_policy"].clone();
    policy["body"]["provider"] = provider.clone();
    f.b["policies"]["empty_query_provider"] = policy;
    f.b["capabilities"]["adapter_lookup"]["contract"] = "empty_query_provider".into();
    f.b["capabilities"]["adapter_lookup"]["output"] = wrap(empty.clone());
    f.b["nodes"]["lookup"]["result"] = Value::Null;
    f.b["nodes"]["lookup"]["writes"] = json!({});
    let mut terminal = f.source.clone();
    terminal["id"] = "urn:test:empty-query-terminal".into();
    terminal["provider"] = provider.clone();
    let source = f.artifact(terminal);
    f.b["nodes"]["unused_lookup_wait"]["capture"]["source"] =
        json!({"id":source.id,"revision":source.revision});
    f.b["data"]["lookup_captured"]["type"] =
        wrap(json!({"kind":"variant","cases":{"confirmed":empty}}));
    f.b["policies"]["recovery_wait"]["body"]["observationType"] =
        f.b["data"]["lookup_captured"]["type"]["body"].clone();
    let faults = f.artifact(json!({"kind":"provider-fault-source","version":"0.1.0","id":"urn:test:empty-query-faults","profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":provider,"emissions":{"lookup uncertainty":{"observations":{"unknown":true},"evidence":"unresolved"}}}));
    f.b["policies"]["lookup_work"]["body"]["faultSource"] =
        json!({"id":faults.id,"revision":faults.revision});
    f
}
#[test]
fn a_query_without_a_result_still_cannot_disclose_a_narrower_branch_context() {
    let mut f = no_result_fixture();
    check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
    for cell in ["captured", "lookup_request", "lookup_captured"] {
        f.b["data"][cell]["protection"]["capabilities"] = json!({"adapter":true});
    }
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-recipient");
}
#[test]
fn strict_invalidation_applies_to_each_level_not_only_the_ordinary_owner() {
    for cell in [
        "original_request",
        "captured",
        "lookup_request",
        "lookup_captured",
    ] {
        let mut f = fixture();
        f.b["data"][cell]["invalidates"] = json!({});
        refuse(&f, "owned-continuation-invalidation-coverage");
    }
}
#[test]
fn unvisited_extra_query_and_nonlocal_storage_do_not_disappear_from_the_proof() {
    let mut f = fixture();
    let mut n = f.b["nodes"]["lookup"].clone();
    let mut d = f.b["data"]["lookup_result"].clone();
    d["invalidates"] = json!({});
    f.b["data"]["extra_result"] = d;
    n["result"] = "extra_result".into();
    n["writes"] = json!({"extra_result":true});
    f.b["nodes"]["extra_query"] = n;
    f.b["flows"]["extra_next"] = json!({"source":"extra_query","outcome":"done","target":"done"});
    refuse(&f, "owned-continuation-body-unproved");
    let mut f = fixture();
    f.b["nodes"]["lookup"]["result"] = "origin_value".into();
    f.b["nodes"]["lookup"]["writes"] = json!({"origin_value":true});
    refuse(&f, "owned-continuation-storage-unproved");
    let mut f = fixture();
    f.b["nodes"]["lookup"]["reads"]["captured"] = true.into();
    refuse(&f, "owned-continuation-query-reads");
}
#[test]
fn foreign_root_storage_and_arbitrary_lexical_children_remain_outside_the_family() {
    let mut f = fixture();
    let mut parent = f.b["data"]["origin_value"].clone();
    parent["scope"] = "main".into();
    parent["initial"] = "parent_initial".into();
    parent["invalidates"] = json!({"origin":true});
    f.b["data"]["parent_value"] = parent;
    f.b["expressions"]["parent_initial"] = json!({"scope":"main","dialect":"core","parameters":{},"reads":{},"resultType":wrap(json!({"kind":"integer"})),"body":{"op":"literal","type":{"kind":"integer"},"value":"1"}});
    f.b["nodes"]["origin"]["reads"]["parent_value"] = true.into();
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-root-storage-unproved");
    let mut f = fixture();
    let mut scope = f.b["scopes"]["lookup_recovery"].clone();
    scope["retainedRequests"] = json!({});
    scope["entry"] = "extra_done".into();
    f.b["scopes"]["extra_child"] = scope;
    let mut node = f.b["nodes"]["lookup_done"].clone();
    node["scope"] = "extra_child".into();
    f.b["nodes"]["extra_done"] = node;
    refuse(&f, "retained-request-creator");
    f.b["scopes"]["extra_child"]["parent"] = "root".into();
    refuse(&f, "owned-continuation-root-child-unproved");
}
#[test]
fn the_bound_child_and_each_exact_readonly_request_remain_required() {
    let mut f = fixture();
    f.b["scopes"]["lookup_recovery"]["retainedRequests"] = json!({});
    refuse(&f, "owned-continuation-request-unproved");
    let mut f = fixture();
    f.b["data"]["lookup_request"]["initial"] = "lookup_input".into();
    assert!(check(&env(&f), &f.supplied()).is_err());
    let mut f = fixture();
    f.b["nodes"]["unused_lookup_wait"]["capture"]["origin"] = "origin".into();
    assert!(check(&env(&f), &f.supplied()).is_err());
}
#[test]
fn an_unsupported_timer_or_general_source_is_not_a_query_permission() {
    let mut f = fixture();
    f.b["policies"]["lookup_work"]["body"]["faultSource"] = Value::Null;
    refuse(&f, "owned-continuation-query-policy");
    let mut f = fixture();
    f.b["policies"]["lookup_work"]["body"]["retry"]["maxAttempts"] = 2.into();
    assert!(check(&env(&f), &f.supplied()).is_err());
}
#[test]
fn matching_irreversible_metadata_does_not_turn_a_command_into_a_query() {
    let mut f = fixture();
    let mut provider = f.provider.clone();
    provider["id"] = "urn:test:owned-command".into();
    provider["effectClass"] = "irreversible".into();
    provider["idempotency"] = "stable-key".into();
    let pin = f.artifact(provider);
    let mut policy = f.b["policies"]["provider_policy"].clone();
    policy["body"]["provider"] = json!({"id":pin.id,"revision":pin.revision});
    f.b["policies"]["lookup_provider"] = policy;
    f.b["capabilities"]["adapter_lookup"]["contract"] = "lookup_provider".into();
    f.b["policies"]["lookup_effects"]["body"]["class"] = "irreversible".into();
    f.b["policies"]["lookup_effects"]["body"]["idempotency"] = "stable-key".into();
    let original = f.b["policies"]["lookup_work"]["body"]["faultSource"].clone();
    let raw = f
        .extra
        .iter()
        .find(|(p, _)| p.id == original["id"] && p.revision == original["revision"])
        .unwrap()
        .1
        .clone();
    let mut faults: Value = serde_json::from_slice(&raw).unwrap();
    faults["id"] = "urn:test:owned-command-faults".into();
    faults["provider"] = json!({"id":pin.id,"revision":pin.revision});
    let faults = f.artifact(faults);
    f.b["policies"]["lookup_work"]["body"]["faultSource"] =
        json!({"id":faults.id,"revision":faults.revision});
    let mut source = f.source.clone();
    source["id"] = "urn:test:owned-command-terminal".into();
    source["provider"] = json!({"id":pin.id,"revision":pin.revision});
    let source = f.artifact(source);
    f.b["nodes"]["unused_lookup_wait"]["capture"]["source"] =
        json!({"id":source.id,"revision":source.revision});
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-query-effect");
}
#[test]
fn repeated_creators_share_declarations_without_sharing_occurrence_values() {
    let mut f = fixture();
    f.b["nodes"]["run_again"] = f.b["nodes"]["run_owner"].clone();
    f.b["flows"]["main_next"]["target"] = "run_again".into();
    f.b["flows"]["again_next"] =
        json!({"source":"run_again","outcome":"complete","target":"main_done"});
    let r = check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
    let t = r
        .links()
        .exceptional()
        .faults()
        .timing()
        .concurrency()
        .owned_continuations()
        .unwrap();
    assert_eq!(t.continuations().len(), 2);
    assert_eq!(t.checked_scopes().len(), 2);
    assert_eq!(t.gates().len(), 7);
    assert!(t.gates().iter().all(|g| g.host_requirements().contains(
        &"fresh-occurrence-owned-immutable-storage-and-authenticated-complete-provenance"
    )));
}
#[test]
fn committed_static_definition_and_shared_vectors_run_through_the_real_sdk() {
    let v: Value = serde_json::from_slice(include_bytes!(
        "../conformance/owned-readonly-definition.json"
    ))
    .unwrap();
    let f = fixture();
    assert_eq!(v["definition"], *env(&f).document());
    let corpus: Value =
        serde_json::from_slice(include_bytes!("../conformance/owned-readonly.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let mut f = fixture();
        for change in case["replace"].as_array().unwrap() {
            *f.b.pointer_mut(change["path"].as_str().unwrap()).unwrap() = change["value"].clone();
        }
        let result = check_reconciliation_availability(&env(&f), &f.supplied());
        let actual = result
            .as_ref()
            .map(|_| "covered")
            .unwrap_or_else(|e| e.code);
        assert_eq!(actual, case["expect"], "{}: {:?}", case["id"], result.err());
    }
}
pub(super) fn three_level_fixture() -> Fixture {
    let mut f = fixture();
    let mut third = f.b["scopes"]["lookup_recovery"].clone();
    third["parent"] = "lookup_recovery".into();
    third["entry"] = "third_wait".into();
    third["retainedRequests"] = json!({"third_request":"third_query"});
    f.b["scopes"]["third"] = third;
    f.b["policies"]["third_keep"] = json!({"scope":"lookup_recovery","dialect":"core","body":{"kind":"closure","reads":{},"mode":"reconcile","followup":"third"}});
    f.b["scopes"]["lookup_recovery"]["closure"] = "third_keep".into();
    let mut query = f.b["nodes"]["lookup"].clone();
    query["scope"] = "lookup_recovery".into();
    query["target"] = "third_capability".into();
    query["input"] = "third_input".into();
    query["result"] = "third_result".into();
    query["reads"] = json!({"lookup_request":true});
    query["writes"] = json!({"third_result":true});
    f.b["nodes"]["third_query"] = query;
    let mut capability = f.b["capabilities"]["adapter_lookup"].clone();
    capability["effects"] = "third_effects".into();
    f.b["capabilities"]["third_capability"] = capability;
    let mut effect = f.b["policies"]["lookup_effects"].clone();
    effect["body"]["reconciliation"] = "third_wait_policy".into();
    f.b["policies"]["third_effects"] = effect;
    f.b["policies"]["third_wait_policy"] = f.b["policies"]["recovery_wait"].clone();
    let mut input = f.b["expressions"]["lookup_input"].clone();
    input["scope"] = "lookup_recovery".into();
    input["reads"] = json!({"lookup_request":true});
    input["body"]["cell"] = "lookup_request".into();
    f.b["expressions"]["third_input"] = input;
    let mut result = f.b["data"]["lookup_result"].clone();
    result["scope"] = "lookup_recovery".into();
    result["invalidates"] = json!({});
    f.b["data"]["third_result"] = result;
    f.b["flows"]["lookup_branch_yes"]["target"] = "third_query".into();
    f.b["flows"]["third_query_next"] =
        json!({"source":"third_query","outcome":"done","target":"lookup_done"});
    let mut wait = f.b["nodes"]["unused_lookup_wait"].clone();
    wait["scope"] = "third".into();
    wait["policy"] = "third_wait_policy".into();
    wait["capture"]["origin"] = "third_query".into();
    wait["capture"]["result"] = "third_captured".into();
    wait["writes"] = json!({"third_captured":true});
    f.b["nodes"]["third_wait"] = wait;
    let mut finish = f.b["nodes"]["lookup_done"].clone();
    finish["scope"] = "third".into();
    f.b["nodes"]["third_done"] = finish;
    f.b["flows"]["third_wait_next"] =
        json!({"source":"third_wait","outcome":"observed","target":"third_done"});
    for (new, old) in [
        ("third_request", "lookup_request"),
        ("third_captured", "lookup_captured"),
    ] {
        let mut cell = f.b["data"][old].clone();
        cell["scope"] = "third".into();
        cell["invalidates"] = json!({"third_wait":true,"third_done":true});
        f.b["data"][new] = cell;
    }
    super::continuation_audit::refresh(&mut f);
    // This fresh synthetic provider explicitly permits the third recipient.
    // Published fixtures/contracts are not broadened to obtain the positive case.
    f.provider["id"] = "urn:test:three-level-provider".into();
    f.provider["resultProtection"]["capabilities"]["third_capability"] = true.into();
    let provider = f.artifact(f.provider.clone());
    let provider = json!({"id":provider.id,"revision":provider.revision});
    f.b["policies"]["provider_policy"]["body"]["provider"] = provider.clone();
    f.source["id"] = "urn:test:three-level-source".into();
    f.source["provider"] = provider.clone();
    f.bind_source();
    let source = f.b["nodes"]["capture_wait"]["capture"]["source"].clone();
    for n in f.b["nodes"].as_object_mut().unwrap().values_mut() {
        if n["kind"] == "wait" && !n["capture"].is_null() {
            n["capture"]["source"] = source.clone();
        }
    }
    for cell in f.b["data"].as_object_mut().unwrap().values_mut() {
        cell["protection"]["capabilities"]["third_capability"] = true.into();
    }
    let faults = f.artifact(json!({"kind":"provider-fault-source","version":"0.1.0","id":"urn:test:three-level-faults","profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":provider,"emissions":{"lookup uncertainty":{"observations":{"unknown":true},"evidence":"unresolved"}}}));
    f.b["policies"]["lookup_work"]["body"]["faultSource"] =
        json!({"id":faults.id,"revision":faults.revision});
    f
}
#[test]
fn three_levels_keep_each_query_context_and_its_own_original_request() {
    let mut f = three_level_fixture();
    let r = check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
    let t = r
        .links()
        .exceptional()
        .faults()
        .timing()
        .concurrency()
        .owned_continuations()
        .unwrap();
    assert_eq!(t.checked_scopes().len(), 3);
    assert_eq!(t.gates().len(), 10);
    assert!(
        t.required_provenance()
            .iter()
            .any(|p| p.owner() == "lookup_recovery"
                && p.site() == "third_query"
                && p.provider_role() == Some("activity"))
    );
    f.b["flows"]["lookup_branch_yes"]["target"] = "lookup_done".into();
    refuse(&f, "owned-continuation-source-unproved");
}
#[test]
fn conditional_query_faults_cannot_borrow_ordinary_handler_entry_coverage() {
    use choreoform_semantic_check::{FAULT_HISTORY_PROFILE, FAULT_HISTORY_TYPES};
    let mut f = fixture();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for p in [FAULT_HISTORY_PROFILE, FAULT_HISTORY_TYPES] {
        f.extra.push((
            Pin {
                id: p.0.into(),
                revision: p.1.into(),
            },
            std::fs::read(root.join(format!("docs/ir/contracts/{}.txt", p.1.replace(':', "-"))))
                .unwrap(),
        ));
    }
    let history = f.artifact(json!({"kind":"provider-fault-history","version":"0.1.0","id":"urn:test:conditional-query-history","profile":{"id":FAULT_HISTORY_PROFILE.0,"revision":FAULT_HISTORY_PROFILE.1},"source":f.b["policies"]["lookup_work"]["body"]["faultSource"]}));
    let mut handler = f.b["scopes"]["main"].clone();
    handler["parent"] = "main".into();
    handler["entry"] = "handler_done".into();
    handler["faultInput"] =
        json!({"history":{"id":history.id,"revision":history.revision},"result":"handler_history"});
    f.b["scopes"]["handler"] = handler;
    let mut done = f.b["nodes"]["main_done"].clone();
    done["scope"] = "handler".into();
    f.b["nodes"]["handler_done"] = done;
    f.b["policies"]["query_faults"] = json!({"scope":"main","dialect":"core","body":{"kind":"faults","reads":{},"handlers":{"lookup uncertainty":"handler"}}});
    f.b["scopes"]["main"]["faults"] = "query_faults".into();
    let mut data = f.b["data"]["lookup_request"].clone();
    data["scope"] = "handler".into();
    data["type"] = wrap(
        json!({"kind":"named","contract":{"id":FAULT_HISTORY_TYPES.0,"revision":FAULT_HISTORY_TYPES.1},"name":"Provider fault history"}),
    );
    data["invalidates"] = json!({});
    f.b["data"]["handler_history"] = data;
    check(&env(&f), &f.supplied()).unwrap();
    check_concurrency(&env(&f), &f.supplied()).unwrap();
    let origins = choreoform_semantic_check::check_fault_origins(&env(&f), &f.supplied()).unwrap();
    assert!(
        origins
            .sites()
            .any(|r| r.site() == "lookup" && !r.scope_explored() && r.source_use().is_some())
    );
    assert_eq!(
        choreoform_semantic_check::check_fault_entries(&env(&f), &f.supplied())
            .unwrap_err()
            .code,
        "fault-entry-source-control-unproved"
    );
}
