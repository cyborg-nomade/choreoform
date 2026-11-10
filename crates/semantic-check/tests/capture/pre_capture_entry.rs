// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Conditional pre-capture declarations, never actual entry or source observations.
use super::*;
use choreoform_semantic_check::{CORE_V9, CORE_V10, check_concurrency};
fn fixture() -> Fixture {
    promote(super::readonly_continuation::fixture())
}
fn promote(mut f: Fixture) -> Fixture {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    f.extra.push((
        Pin {
            id: CORE_V10.0.into(),
            revision: CORE_V10.1.into(),
        },
        std::fs::read(root.join(format!(
            "docs/ir/contracts/{}.txt",
            CORE_V10.1.replace(':', "-")
        )))
        .unwrap(),
    ));
    f.b["dialects"]["core"] = json!({"id":CORE_V10.0,"revision":CORE_V10.1});
    f.b["scopes"]["recovery"]["entry"] = "lookup".into();
    f.b["flows"]["lookup_next"]["target"] = "capture_wait".into();
    f.b["flows"]["read_yes"]["target"] = "done".into();
    f
}
fn env(f: &Fixture) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.4.0", f.b.clone(), json!({})).unwrap()
}
fn refuse(f: &Fixture, code: &str) {
    match check_concurrency(&env(f), &f.supplied()) {
        Err(e) => assert_eq!(e.code, code, "{e:?}"),
        Ok(_) => panic!("expected {code}"),
    }
}
#[test]
fn query_before_original_capture_has_separate_optional_entry_relation_and_no_ordinary_promotion() {
    let f = fixture();
    let e = env(&f);
    let supplied = f.supplied();
    check(&e, &supplied).unwrap();
    let report = check_reconciliation_availability(&e, &supplied).unwrap();
    let concurrency = report.links().exceptional().faults().timing().concurrency();
    let tree = concurrency.owned_continuations().unwrap();
    let root = tree.continuations().iter().next().unwrap();
    let mut levels = root.scopes();
    let first = levels.next().unwrap();
    let entry = first.pre_capture_entry().unwrap();
    assert_eq!(
        (entry.entry(), entry.capture_wait()),
        ("lookup", "capture_wait")
    );
    assert!(!entry.host_requirements().is_empty());
    assert!(levels.next().unwrap().pre_capture_entry().is_none());
    assert!(tree.checked_nodes().contains("lookup"));
    assert!(tree.checked_nodes().contains("capture_wait"));
    assert_eq!(tree.gates().len(), 7);
    assert!(
        !concurrency
            .invalidation()
            .protection()
            .control()
            .covered_nodes()
            .contains("lookup")
    );
    assert!(report.uncovered_nodes().contains("lookup"));
    assert!(!report.checked_nodes().contains("lookup"));
    assert_eq!(report.continuation_scopes(), tree.checked_scopes());
    assert!(report.retained_request_scopes().is_empty());
}
#[test]
fn older_core_stays_fixed_and_new_selection_requires_every_base_and_uniform_alias() {
    let mut f = fixture();
    f.b["dialects"]["core"] = json!({"id":CORE_V9.0,"revision":CORE_V9.1});
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "retained-request-capture"
    );
    let mut f = fixture();
    f.b["dialects"]["older"] = json!({"id":CORE_V9.0,"revision":CORE_V9.1});
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "retained-request-wire-version"
    );
    let f = fixture();
    for (pin, _) in f
        .extra
        .iter()
        .filter(|(p, _)| p.id.contains("core-profile"))
    {
        let supplied: Vec<_> = f
            .supplied()
            .into_iter()
            .filter(|r| r.id != pin.id)
            .collect();
        assert!(check(&env(&f), &supplied).is_err(), "{}", pin.id);
    }
}
#[test]
fn fresh_local_computation_and_request_branch_can_precede_both_query_and_capture() {
    let mut f = fixture();
    let mut copy = f.b["data"]["original_request"].clone();
    copy["invalidates"] = json!({});
    f.b["data"]["request_copy"] = copy;
    f.b["nodes"]["early_copy"] = json!({"kind":"compute","scope":"recovery","reads":{"original_request":true},"writes":{"request_copy":true},"assignments":{"request_copy":"lookup_input"},"outcomes":{"next":true}});
    let read = json!({"op":"read","cell":"original_request"});
    f.b["expressions"]["early_guard"] = json!({"scope":"recovery","dialect":"core","parameters":{},"reads":{"original_request":true},"resultType":wrap(json!({"kind":"boolean"})),"body":{"op":"eq","left":read,"right":read}});
    f.b["nodes"]["early_branch"] = json!({"kind":"decision","scope":"recovery","reads":{"original_request":true},"writes":{},"outcomes":{"yes":true,"no":true},"guards":{"yes":"early_guard"},"default":"no"});
    f.b["flows"]["early_copy_next"] =
        json!({"source":"early_copy","outcome":"next","target":"early_branch"});
    for outcome in ["yes", "no"] {
        f.b["flows"][format!("early_{outcome}")] =
            json!({"source":"early_branch","outcome":outcome,"target":"lookup"});
    }
    f.b["scopes"]["recovery"]["entry"] = "early_copy".into();
    super::continuation_audit::refresh(&mut f);
    let r = check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
    let t = r
        .links()
        .exceptional()
        .faults()
        .timing()
        .concurrency()
        .owned_continuations()
        .unwrap();
    assert!(t.checked_nodes().contains("early_branch"));
    assert_eq!(
        t.continuations()
            .iter()
            .next()
            .unwrap()
            .scopes()
            .next()
            .unwrap()
            .pre_capture_entry()
            .unwrap()
            .entry(),
        "early_copy"
    );
}
#[test]
fn captured_result_is_unavailable_to_an_entry_branch_before_its_wait() {
    let mut f = fixture();
    let mut branch = f.b["nodes"]["read_capture"].clone();
    branch["scope"] = "recovery".into();
    f.b["nodes"]["early_branch"] = branch;
    f.b["scopes"]["recovery"]["entry"] = "early_branch".into();
    for outcome in ["yes", "no"] {
        f.b["flows"][format!("early_{outcome}")] =
            json!({"source":"early_branch","outcome":outcome,"target":"lookup"});
    }
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-availability-unproved");
}
#[test]
fn source_wait_cannot_be_substituted_duplicated_omitted_or_left_unreachable() {
    let mut f = fixture();
    f.b["flows"]["lookup_next"]["target"] = "done".into();
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-capture-unreachable");
    let mut f = fixture();
    f.b["policies"]["effects"]["body"]["reconciliation"] = "recovery_wait".into();
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "retained-request-source-wait"
    );
    let mut f = fixture();
    f.b["nodes"]["capture_wait"]["capture"] = Value::Null;
    f.b["nodes"]["capture_wait"]["writes"] = json!({});
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "retained-request-source-wait"
    );
    let mut f = fixture();
    f.b["nodes"]["extra_wait"] = f.b["nodes"]["capture_wait"].clone();
    f.b["flows"]["extra_wait_next"] =
        json!({"source":"extra_wait","outcome":"observed","target":"done"});
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "reconciliation-owner-unproved"
    );
    let mut f = fixture();
    f.b["nodes"].as_object_mut().unwrap().remove("capture_wait");
    f.b["flows"].as_object_mut().unwrap().remove("capture_next");
    f.b["flows"]["lookup_next"]["target"] = "read_capture".into();
    for cell in f.b["data"].as_object_mut().unwrap().values_mut() {
        cell["invalidates"]
            .as_object_mut()
            .unwrap()
            .remove("capture_wait");
    }
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "reconciliation-owner-missing"
    );
}
#[test]
fn actual_use_wait_in_another_owner_does_not_bind_the_surviving_request() {
    let mut f = fixture();
    f.b["policies"]["outside_wait"] = f.b["policies"]["ordinary_wait"].clone();
    f.b["policies"]["effects"]["body"]["reconciliation"] = "outside_wait".into();
    let mut wait = f.b["nodes"]["capture_wait"].clone();
    wait["scope"] = "root".into();
    wait["policy"] = "outside_wait".into();
    wait["capture"]["result"] = "outside_result".into();
    wait["writes"] = json!({"outside_result":true});
    f.b["nodes"]["outside_wait_step"] = wait;
    let mut cell = f.b["data"]["captured"].clone();
    cell["scope"] = "root".into();
    cell["invalidates"] = json!({});
    f.b["data"]["outside_result"] = cell;
    f.b["flows"]["outside_next"] =
        json!({"source":"outside_wait_step","outcome":"observed","target":"owner_done"});
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "retained-request-source-wait"
    );
}
#[test]
fn seed_access_remains_empty_and_each_level_requires_strict_invalidation() {
    for read in ["lookup_request", "lookup_captured", "original_request"] {
        let mut f = fixture();
        f.b["policies"]["seed_access"] = json!({"scope":"lookup_recovery","dialect":"core","body":{"kind":"protection","reads":{read:true},"permit":{"op":"eq","left":{"op":"read","cell":read},"right":{"op":"read","cell":read}}}});
        f.b["data"]["lookup_request"]["protection"]["policy"] = "seed_access".into();
        refuse(&f, "owned-continuation-access-precheck");
    }
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
fn inherited_control_protects_pre_capture_initializers_and_query_recipients() {
    let mut f = fixture();
    let mut flag = f.b["data"]["original_request"].clone();
    flag["type"] = wrap(json!({"kind":"boolean"}));
    flag["initial"] = "flag_initial".into();
    flag["invalidates"] = json!({});
    flag["protection"]["sensitivity"] = "public".into();
    f.b["data"]["flag"] = flag;
    f.b["expressions"]["flag_initial"] = json!({"scope":"recovery","dialect":"core","parameters":{},"reads":{},"resultType":wrap(json!({"kind":"boolean"})),"body":boolean(true)});
    refuse(&f, "control-information-flow");
    f.b["data"]["flag"]["protection"]["sensitivity"] = "internal".into();
    f.b["data"]["flag"]["protection"]["capabilities"] = json!({"adapter":true});
    let read = json!({"op":"read","cell":"flag"});
    f.b["expressions"]["flag_guard"] = json!({"scope":"recovery","dialect":"core","parameters":{},"reads":{"flag":true},"resultType":wrap(json!({"kind":"boolean"})),"body":{"op":"eq","left":read,"right":read}});
    f.b["nodes"]["early_branch"] = json!({"kind":"decision","scope":"recovery","reads":{"flag":true},"writes":{},"outcomes":{"yes":true,"no":true},"guards":{"yes":"flag_guard"},"default":"no"});
    f.b["scopes"]["recovery"]["entry"] = "early_branch".into();
    for outcome in ["yes", "no"] {
        f.b["flows"][format!("early_{outcome}")] =
            json!({"source":"early_branch","outcome":outcome,"target":"lookup"});
    }
    super::continuation_audit::refresh(&mut f);
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-recipient");
}
#[test]
fn original_command_can_remain_accountable_but_new_commands_and_input_transforms_stay_refused() {
    let mut f = fixture();
    super::owned_entry_audit::provider(&mut f, "origin", "irreversible", "stable-key");
    check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
    for class in ["irreversible", "compensatable"] {
        let mut f = fixture();
        super::owned_entry_audit::provider(&mut f, "lookup", class, "stable-key");
        check(&env(&f), &f.supplied()).unwrap();
        refuse(&f, "owned-continuation-query-effect");
    }
    let mut f = fixture();
    let read = json!({"op":"read","cell":"original_request"});
    f.b["expressions"]["lookup_input"]["body"] =
        json!({"op":"if","condition":boolean(true),"then":read,"else":read});
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-query-input");
    let mut f = fixture();
    f.b["policies"]["lookup_work"]["scope"] = "recovery".into();
    f.b["policies"]["lookup_work"]["body"]["compensates"] = "origin".into();
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-query-policy");
    let mut f = fixture();
    f.b["policies"]["lookup_work"]["body"]["faultSource"] = Value::Null;
    refuse(&f, "owned-continuation-query-policy");
}
#[test]
fn capturing_entry_paths_remain_supported_and_terminal_leaf_cannot_acquire_early_entry() {
    let mut f = fixture();
    f.b["scopes"]["recovery"]["entry"] = "capture_wait".into();
    f.b["flows"]["read_yes"]["target"] = "lookup".into();
    f.b["flows"]["lookup_next"]["target"] = "done".into();
    let r = check_concurrency(&env(&f), &f.supplied()).unwrap();
    assert!(
        r.owned_continuations()
            .unwrap()
            .continuations()
            .iter()
            .all(|c| c.scopes().all(|s| s.pre_capture_entry().is_none()))
    );
    let mut f = fixture();
    f.b["scopes"]["lookup_recovery"]["entry"] = "lookup_branch".into();
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "retained-request-capture"
    );
}
#[test]
fn a_real_clock_descriptor_does_not_waive_the_source_capture_timer_barrier() {
    use choreoform_semantic_check::CLOCK_PROFILE;
    let mut f = fixture();
    let mut p = f.provider.clone();
    p["id"] = "urn:test:pre-capture-clock-provider".into();
    p["outcomes"] = json!({"tick":true});
    p["observations"] =
        json!({"tick":{"state":"succeeded","result":{"kind":"integer"},"outcome":"tick"}});
    let p = f.artifact(p);
    let clock=f.artifact(json!({"kind":"clock","version":"0.1.0","id":"urn:test:pre-capture-clock","profile":{"id":CLOCK_PROFILE.0,"revision":CLOCK_PROFILE.1},"source":{"id":p.id,"revision":p.revision}}));
    f.extra.push((Pin{id:CLOCK_PROFILE.0.into(),revision:CLOCK_PROFILE.1.into()},include_bytes!("../../../../docs/ir/contracts/sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt").to_vec()));
    let mut basis = f.b["data"]["origin_value"].clone();
    basis["scope"] = "main".into();
    basis["initial"] = "basis_initial".into();
    basis["invalidates"] = json!({});
    f.b["data"]["time_basis"] = basis;
    f.b["expressions"]["basis_initial"] = json!({"scope":"main","dialect":"core","parameters":{},"reads":{},"resultType":wrap(json!({"kind":"integer"})),"body":{"op":"literal","type":{"kind":"integer"},"value":"0"}});
    f.b["policies"]["ordinary_wait"]["body"]["timer"] = json!({"clock":{"id":clock.id,"revision":clock.revision},"basis":"time_basis","due":{"op":"literal","type":{"kind":"integer"},"value":"0"},"calendar":null,"pauseAuthority":"operator"});
    f.b["policies"]["ordinary_wait"]["body"]["onTimer"] = "expired".into();
    f.b["policies"]["ordinary_wait"]["body"]["reads"] = json!({"time_basis":true});
    f.b["nodes"]["capture_wait"]["reads"] = json!({"time_basis":true});
    f.b["nodes"]["capture_wait"]["outcomes"]["expired"] = true.into();
    f.b["flows"]["capture_timeout"] =
        json!({"source":"capture_wait","outcome":"expired","target":"done"});
    for policy in ["effects", "provider_policy", "lookup_work"] {
        f.b["policies"][policy]["body"]["reads"] = json!({"time_basis":true});
    }
    let error = check(&env(&f), &f.supplied()).unwrap_err();
    assert_eq!(error.code, "wait-capture-timer-unproved", "{error:?}");
}
#[test]
fn shared_isolated_vectors_are_checked_by_the_actual_complete_sdk() {
    let corpus: Value =
        serde_json::from_slice(include_bytes!("../conformance/pre-capture-entry.json")).unwrap();
    assert_eq!(
        corpus["core"],
        json!({"id":CORE_V10.0,"revision":CORE_V10.1})
    );
    for case in corpus["cases"].as_array().unwrap() {
        let mut f = fixture();
        for change in case["put"].as_array().unwrap() {
            let path = change["path"].as_str().unwrap();
            let (parent, key) = path.rsplit_once('/').unwrap();
            f.b.pointer_mut(parent)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .insert(key.into(), change["value"].clone());
        }
        let result = check_reconciliation_availability(&env(&f), &f.supplied());
        let actual = result
            .as_ref()
            .map(|_| "covered")
            .unwrap_or_else(|e| e.code);
        assert_eq!(actual, case["expect"], "{}: {:?}", case["id"], result.err());
    }
}
#[test]
fn nested_pre_capture_entries_use_prior_conditional_query_contexts() {
    let mut f = promote(super::readonly_continuation::three_level_fixture());
    f.b["scopes"]["lookup_recovery"]["entry"] = "third_query".into();
    f.b["flows"]["third_query_next"]["target"] = "unused_lookup_wait".into();
    f.b["flows"]["lookup_branch_yes"]["target"] = "lookup_done".into();
    let r = check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
    let tree = r
        .links()
        .exceptional()
        .faults()
        .timing()
        .concurrency()
        .owned_continuations()
        .unwrap();
    let levels: Vec<_> = tree
        .continuations()
        .iter()
        .next()
        .unwrap()
        .scopes()
        .collect();
    assert_eq!(levels.len(), 3);
    assert_eq!(tree.gates().len(), 10);
    assert!(levels[0].pre_capture_entry().is_some());
    assert!(levels[1].pre_capture_entry().is_some());
    assert!(levels[2].pre_capture_entry().is_none());
    assert!(levels[1].conditional_source());
    assert!(
        tree.required_provenance()
            .iter()
            .any(|p| p.owner() == "lookup_recovery"
                && p.site() == "third_query"
                && p.provider_role() == Some("activity"))
    );
    f.b["scopes"]["lookup_recovery"]["entry"] = "unused_lookup_wait".into();
    refuse(&f, "owned-continuation-source-unproved");
}
#[test]
fn no_result_query_before_capture_cannot_disclose_a_narrow_local_control_context() {
    let mut f = promote(super::readonly_continuation::no_result_fixture());
    let mut flag = f.b["data"]["original_request"].clone();
    flag["type"] = wrap(json!({"kind":"boolean"}));
    flag["initial"] = "flag_initial".into();
    flag["protection"]["capabilities"] = json!({"adapter":true});
    f.b["data"]["flag"] = flag;
    f.b["expressions"]["flag_initial"] = json!({"scope":"recovery","dialect":"core","parameters":{},"reads":{},"resultType":wrap(json!({"kind":"boolean"})),"body":boolean(true)});
    let read = json!({"op":"read","cell":"flag"});
    f.b["expressions"]["flag_guard"] = json!({"scope":"recovery","dialect":"core","parameters":{},"reads":{"flag":true},"resultType":wrap(json!({"kind":"boolean"})),"body":{"op":"eq","left":read,"right":read}});
    f.b["nodes"]["early_branch"] = json!({"kind":"decision","scope":"recovery","reads":{"flag":true},"writes":{},"outcomes":{"yes":true,"no":true},"guards":{"yes":"flag_guard"},"default":"no"});
    f.b["scopes"]["recovery"]["entry"] = "early_branch".into();
    for outcome in ["yes", "no"] {
        f.b["flows"][format!("early_{outcome}")] =
            json!({"source":"early_branch","outcome":outcome,"target":"lookup"});
    }
    for cell in ["lookup_request", "lookup_captured"] {
        f.b["data"][cell]["protection"]["capabilities"] = json!({"adapter":true});
    }
    super::continuation_audit::refresh(&mut f);
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-recipient");
}
#[test]
fn reused_creators_share_only_declarations_and_keep_new_entry_relations() {
    let mut f = fixture();
    f.b["nodes"]["again"] = f.b["nodes"]["run_owner"].clone();
    f.b["flows"]["main_next"]["target"] = "again".into();
    f.b["flows"]["again_next"] =
        json!({"source":"again","outcome":"complete","target":"main_done"});
    let r = check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
    let tree = r
        .links()
        .exceptional()
        .faults()
        .timing()
        .concurrency()
        .owned_continuations()
        .unwrap();
    assert_eq!(tree.continuations().len(), 2);
    assert_eq!(tree.checked_scopes().len(), 2);
    assert_eq!(tree.gates().len(), 7);
    assert!(tree.continuations().iter().all(|c| {
        c.scopes()
            .next()
            .unwrap()
            .pre_capture_entry()
            .unwrap()
            .entry()
            == "lookup"
    }));
}
#[test]
fn an_initializer_or_competing_compute_cannot_fabricate_a_pre_capture_observation() {
    for mode in ["initializer", "compute"] {
        let mut f = fixture();
        let ty = f.b["data"]["captured"]["type"].clone();
        f.b["expressions"]["invented_capture"] = json!({"scope":"recovery","dialect":"core","parameters":{},"reads":{},"resultType":ty,"body":{"op":"variant","type":ty["body"],"tag":"confirmed","value":{"op":"literal","type":{"kind":"integer"},"value":"0"}}});
        f.b["nodes"]["early_branch"] = f.b["nodes"]["read_capture"].clone();
        for outcome in ["yes", "no"] {
            f.b["flows"][format!("early_{outcome}")] =
                json!({"source":"early_branch","outcome":outcome,"target":"lookup"});
        }
        if mode == "initializer" {
            f.b["data"]["captured"]["initial"] = "invented_capture".into();
            f.b["scopes"]["recovery"]["entry"] = "early_branch".into();
        } else {
            f.b["nodes"]["invent_capture"] = json!({"kind":"compute","scope":"recovery","reads":{},"writes":{"captured":true},"assignments":{"captured":"invented_capture"},"outcomes":{"next":true}});
            f.b["flows"]["invent_next"] =
                json!({"source":"invent_capture","outcome":"next","target":"early_branch"});
            f.b["scopes"]["recovery"]["entry"] = "invent_capture".into();
        }
        super::continuation_audit::refresh(&mut f);
        let result = check_reconciliation_availability(&env(&f), &f.supplied());
        assert!(
            matches!(result,Err(ref e) if e.code=="retained-request-source-result"),
            "{mode}: a fabricated capture must refuse"
        );
    }
}
