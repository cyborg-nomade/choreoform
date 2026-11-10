// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Conditional work declarations, not accepted commands, grants or compensation.
use super::*;
use super::{command_audit, owned_entry_audit::provider};
use choreoform_semantic_check::{CORE_V10, CORE_V11, check_concurrency};

fn promote(mut f: Fixture) -> Fixture {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for core in [CORE_V10, CORE_V11] {
        if !f
            .extra
            .iter()
            .any(|(p, _)| p.id == core.0 && p.revision == core.1)
        {
            f.extra.push((
                Pin {
                    id: core.0.into(),
                    revision: core.1.into(),
                },
                std::fs::read(root.join(format!(
                    "docs/ir/contracts/{}.txt",
                    core.1.replace(':', "-")
                )))
                .unwrap(),
            ));
        }
    }
    f.b["dialects"]["core"] = json!({"id":CORE_V11.0,"revision":CORE_V11.1});
    f
}
fn fixture(class: &str, linked: bool, capturing: bool) -> Fixture {
    let mut f = promote(command_audit::fixture(capturing));
    provider(
        &mut f,
        "lookup",
        class,
        if class == "read-only" {
            "none"
        } else {
            "stable-key"
        },
    );
    if linked {
        command_audit::compensates(&mut f);
    }
    f
}
fn env(f: &Fixture) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.4.0", f.b.clone(), json!({})).unwrap()
}
fn refuse(f: &Fixture, code: &str) {
    match check_reconciliation_availability(&env(f), &f.supplied()) {
        Err(e) => assert_eq!(e.code, code, "{e:?}"),
        Ok(_) => panic!("expected {code}"),
    }
}
#[test]
fn all_classes_and_independent_compensation_links_have_distinct_work_not_false_queries() {
    for original in ["read-only", "irreversible"] {
        for class in ["read-only", "compensatable", "irreversible"] {
            for linked in [false, true] {
                let mut f = fixture(class, linked, true);
                provider(&mut f, "origin", original, "stable-key");
                let r = check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
                let concurrency = r.links().exceptional().faults().timing().concurrency();
                let tree = concurrency.owned_continuations().unwrap();
                let levels: Vec<_> = tree
                    .continuations()
                    .iter()
                    .next()
                    .unwrap()
                    .scopes()
                    .collect();
                let work = levels[0].work().unwrap();
                assert_eq!(work.site(), "lookup");
                assert_eq!(work.capability(), "adapter_lookup");
                assert_eq!(work.input(), "lookup_input");
                assert_eq!(work.effect_class().as_str(), class);
                assert_eq!(work.compensates(), linked.then_some("origin"));
                assert_eq!(work.is_query(), class == "read-only" && !linked);
                assert_eq!(levels[0].query(), work.is_query().then_some("lookup"));
                assert!(!work.host_requirements().is_empty());
                assert!(levels[1].work().is_none());
                assert_eq!(tree.gates().len(), 7);
                assert_eq!(r.continuation_scopes(), tree.checked_scopes());
                assert!(r.retained_request_scopes().is_empty());
                assert!(r.uncovered_nodes().contains("lookup"));
                assert!(!r.checked_nodes().contains("lookup"));
            }
        }
    }
}
#[test]
fn only_uncompensated_readonly_inquiry_can_precede_original_capture() {
    let f = fixture("read-only", false, false);
    let r = check_concurrency(&env(&f), &f.supplied()).unwrap();
    assert!(
        r.owned_continuations()
            .unwrap()
            .continuations()
            .iter()
            .next()
            .unwrap()
            .scopes()
            .next()
            .unwrap()
            .pre_capture_entry()
            .is_some()
    );
    for class in ["read-only", "compensatable", "irreversible"] {
        for linked in [false, true] {
            if class == "read-only" && !linked {
                continue;
            }
            refuse(
                &fixture(class, linked, false),
                "owned-continuation-work-entry",
            );
        }
    }
}
#[test]
fn pure_identity_constant_and_captured_inputs_can_use_different_exact_child_seed_types() {
    for capturing in [false, true] {
        for constant in [false, true] {
            let mut f = fixture("read-only", false, capturing);
            if constant {
                f.b["expressions"]["lookup_input"]["body"] = json!({"op":"record","fields":{}});
                f.b["expressions"]["lookup_input"]["reads"] = json!({});
                f.b["nodes"]["lookup"]["reads"] = json!({});
            } else {
                let read = json!({"op":"read","cell":"original_request"});
                f.b["expressions"]["lookup_input"]["body"] =
                    json!({"op":"if","condition":boolean(true),"then":read,"else":read});
            }
            check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
        }
    }
    let f = promote(command_audit::captured_input(true));
    let r = check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
    assert_eq!(r.continuation_scopes().len(), 2);
    assert_ne!(
        f.b["data"]["original_request"]["type"],
        f.b["data"]["lookup_request"]["type"]
    );
    let f = promote(command_audit::captured_input(false));
    refuse(&f, "owned-continuation-availability-unproved");
}
#[test]
fn all_old_pins_aliases_exact_artifacts_and_query_refusals_stay_fixed() {
    let mut f = fixture("irreversible", false, true);
    f.b["dialects"]["core"] = json!({"id":CORE_V10.0,"revision":CORE_V10.1});
    refuse(&f, "owned-continuation-query-effect");
    let mut f = fixture("read-only", false, true);
    f.b["dialects"]["old"] = json!({"id":CORE_V10.0,"revision":CORE_V10.1});
    refuse(&f, "retained-request-wire-version");
    let f = fixture("irreversible", false, true);
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

// Fresh synthetic per-activity metadata; old resources/recipient sets stay fixed.
fn bind(f: &mut Fixture, node: &str, class: &str, key: &str) {
    use choreoform_semantic_check::FAULT_SOURCE_PROFILE;
    let mut p = f.provider.clone();
    p["id"] = format!("urn:test:finite:{node}:{class}:{key}").into();
    p["effectClass"] = class.into();
    p["idempotency"] = key.into();
    let pin = f.artifact(p);
    let pin = json!({"id":pin.id,"revision":pin.revision});
    let cap = f.b["nodes"][node]["target"].as_str().unwrap().to_owned();
    let effect = f.b["capabilities"][&cap]["effects"]
        .as_str()
        .unwrap()
        .to_owned();
    let contract = format!("{node}_finite_provider");
    let mut policy = f.b["policies"]["provider_policy"].clone();
    policy["body"]["provider"] = pin.clone();
    f.b["policies"][&contract] = policy;
    f.b["capabilities"][&cap]["contract"] = contract.into();
    f.b["policies"][&effect]["body"]["class"] = class.into();
    f.b["policies"][&effect]["body"]["idempotency"] = key.into();
    let mut terminal = f.source.clone();
    terminal["id"] = format!("urn:test:finite-terminal:{node}:{class}:{key}").into();
    terminal["provider"] = pin.clone();
    let terminal = f.artifact(terminal);
    for n in f.b["nodes"].as_object_mut().unwrap().values_mut() {
        if n["kind"] == "wait" && n["capture"]["origin"] == node {
            n["capture"]["source"] = json!({"id":terminal.id,"revision":terminal.revision});
        }
    }
    let work = f.b["nodes"][node]["policy"].as_str().unwrap().to_owned();
    let faults=f.artifact(json!({"kind":"provider-fault-source","version":"0.1.0","id":format!("urn:test:finite-faults:{node}:{class}:{key}"),"profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":pin,"emissions":{"lookup uncertainty":{"observations":{"unknown":true},"evidence":"unresolved"}}}));
    f.b["policies"][&work]["body"]["faultSource"] =
        json!({"id":faults.id,"revision":faults.revision});
}
#[test]
fn commands_and_compensators_accept_a_typed_capture_derived_new_input() {
    for class in ["read-only", "compensatable", "irreversible"] {
        for linked in [false, true] {
            let mut f = promote(command_audit::captured_input(true));
            bind(
                &mut f,
                "lookup",
                class,
                if class == "read-only" {
                    "none"
                } else {
                    "stable-key"
                },
            );
            if linked {
                command_audit::compensates(&mut f);
            }
            check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
        }
    }
}
#[test]
fn key_general_source_and_compensation_source_are_independent_obligations() {
    let mut f = fixture("irreversible", false, true);
    bind(&mut f, "lookup", "irreversible", "none");
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-work-idempotency");
    let mut f = fixture("irreversible", false, true);
    f.b["policies"]["lookup_work"]["body"]["faultSource"] = Value::Null;
    refuse(&f, "owned-continuation-work-policy");
    let mut f = fixture("irreversible", true, true);
    f.b["policies"]["lookup_work"]["body"]["compensates"] = "lookup".into();
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-compensation-source");
    let mut f = fixture("irreversible", true, true);
    f.b["policies"]["lookup_work"]["body"]["compensates"] = "read_capture".into();
    refuse(&f, "compensation-target");
}
#[test]
fn command_capture_cannot_bind_an_arbitrary_wait_or_initialized_or_competing_target() {
    let mut f = fixture("irreversible", false, true);
    f.b["policies"]["effects"]["body"]["reconciliation"] = "recovery_wait".into();
    refuse(&f, "retained-request-source-wait");
    // This otherwise valid capturing-first path was not constrained by core 0.10's
    // early-entry integrity check. The new command relation must check it itself.
    let mut f = fixture("irreversible", false, true);
    let ty = f.b["data"]["captured"]["type"].clone();
    f.b["expressions"]["invented_capture"] = json!({"scope":"recovery","dialect":"core","parameters":{},"reads":{},"resultType":ty,"body":{"op":"variant","type":ty["body"],"tag":"confirmed","value":{"op":"literal","type":{"kind":"integer"},"value":"0"}}});
    f.b["data"]["captured"]["initial"] = "invented_capture".into();
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "retained-request-source-result");
    f.b["data"]["captured"]
        .as_object_mut()
        .unwrap()
        .remove("initial");
    f.b["nodes"]["unvisited_writer"] = json!({"kind":"compute","scope":"recovery","reads":{},"writes":{"captured":true},"assignments":{"captured":"invented_capture"},"outcomes":{"next":true}});
    f.b["flows"]["unvisited_next"] =
        json!({"source":"unvisited_writer","outcome":"next","target":"done"});
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "retained-request-source-result");
}
#[test]
fn constructed_constant_no_result_command_still_enforces_local_control_recipient() {
    let mut f = promote(super::readonly_continuation::no_result_fixture());
    f.provider["output"] = json!({"kind":"record","fields":{}});
    f.provider["observations"]["done"]["result"] = f.provider["output"].clone();
    bind(&mut f, "lookup", "irreversible", "stable-key");
    f.b["expressions"]["lookup_input"]["body"] = json!({"op":"record","fields":{}});
    f.b["expressions"]["lookup_input"]["reads"] = json!({});
    f.b["nodes"]["lookup"]["reads"] = json!({});
    check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
    let mut flag = f.b["data"]["original_request"].clone();
    flag["type"] = wrap(json!({"kind":"boolean"}));
    flag["initial"] = "flag_initial".into();
    flag["protection"]["capabilities"] = json!({"adapter":true});
    f.b["data"]["flag"] = flag;
    f.b["expressions"]["flag_initial"] = json!({"scope":"recovery","dialect":"core","parameters":{},"reads":{},"resultType":wrap(json!({"kind":"boolean"})),"body":boolean(true)});
    let read = json!({"op":"read","cell":"flag"});
    f.b["expressions"]["flag_guard"] = json!({"scope":"recovery","dialect":"core","parameters":{},"reads":{"flag":true},"resultType":wrap(json!({"kind":"boolean"})),"body":{"op":"eq","left":read,"right":read}});
    f.b["nodes"]["branch"] = json!({"kind":"decision","scope":"recovery","reads":{"flag":true},"writes":{},"guards":{"yes":"flag_guard"},"default":"no","outcomes":{"yes":true,"no":true}});
    for outcome in ["yes", "no"] {
        f.b["flows"][format!("branch_{outcome}")] =
            json!({"source":"branch","outcome":outcome,"target":"lookup"});
    }
    f.b["flows"]["read_yes"]["target"] = "branch".into();
    super::continuation_audit::refresh(&mut f);
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-recipient");
}
#[test]
fn mixed_chains_use_actual_work_as_next_source_and_keep_each_prior_context() {
    for command_first in [false, true] {
        let mut f = promote(super::readonly_continuation::three_level_fixture());
        // Distinct policies preserve exact provider sources for the two activities.
        f.b["policies"]["third_work"] = f.b["policies"]["lookup_work"].clone();
        f.b["nodes"]["third_query"]["policy"] = "third_work".into();
        bind(
            &mut f,
            if command_first {
                "lookup"
            } else {
                "third_query"
            },
            "irreversible",
            "stable-key",
        );
        if command_first {
            f.b["scopes"]["lookup_recovery"]["entry"] = "third_query".into();
            f.b["flows"]["third_query_next"]["target"] = "unused_lookup_wait".into();
            f.b["flows"]["lookup_branch_yes"]["target"] = "lookup_done".into();
        }
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
        assert_eq!(levels[1].source(), "lookup");
        assert!(levels[1].conditional_source());
        assert_eq!(levels[2].source(), "third_query");
        assert!(levels[2].conditional_source());
        assert_eq!(levels[0].query().is_none(), command_first);
        assert_eq!(levels[1].query().is_none(), !command_first);
        assert!(levels[2].work().is_none());
    }
}

#[test]
fn a_supported_clock_does_not_turn_timeouts_or_retries_into_finite_work() {
    use choreoform_semantic_check::CLOCK_PROFILE;
    for retry in [false, true] {
        let mut f = fixture("irreversible", false, true);
        let mut p = f.provider.clone();
        p["id"] = "urn:test:finite-clock-provider".into();
        p["outcomes"] = json!({"tick":true});
        p["observations"] =
            json!({"tick":{"state":"succeeded","result":{"kind":"integer"},"outcome":"tick"}});
        let p = f.artifact(p);
        let clock=f.artifact(json!({"kind":"clock","version":"0.1.0","id":"urn:test:finite-clock","profile":{"id":CLOCK_PROFILE.0,"revision":CLOCK_PROFILE.1},"source":{"id":p.id,"revision":p.revision}}));
        f.extra.push((Pin{id:CLOCK_PROFILE.0.into(),revision:CLOCK_PROFILE.1.into()},include_bytes!("../../../../docs/ir/contracts/sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt").to_vec()));
        let mut basis = f.b["data"]["lookup_result"].clone();
        basis["initial"] = "basis_initial".into();
        basis["invalidates"] = json!({});
        f.b["data"]["time_basis"] = basis;
        f.b["expressions"]["basis_initial"] = json!({"scope":"recovery","dialect":"core","parameters":{},"reads":{},"resultType":wrap(json!({"kind":"integer"})),"body":{"op":"literal","type":{"kind":"integer"},"value":"0"}});
        f.b["policies"]["lookup_work"]["scope"] = "recovery".into();
        f.b["policies"]["lookup_work"]["body"]["reads"] = json!({"time_basis":true});
        f.b["nodes"]["lookup"]["reads"]["time_basis"] = true.into();
        if retry {
            let fault = f.b["policies"]["lookup_work"]["body"]["faultSource"].clone();
            f.b["policies"]["lookup_work"]["body"]["retry"] = json!({"maxAttempts":2,"delays":["0"],"faults":{"lookup uncertainty":true},"clock":{"id":clock.id,"revision":clock.revision},"basis":"time_basis","faultSource":fault});
        } else {
            f.b["policies"]["lookup_work"]["body"]["timeout"] = json!({"clock":{"id":clock.id,"revision":clock.revision},"basis":"time_basis","due":{"op":"literal","type":{"kind":"integer"},"value":"0"},"calendar":null,"pauseAuthority":"operator"});
        }
        check(&env(&f), &f.supplied()).unwrap();
        refuse(&f, "owned-continuation-work-policy");
    }
}

#[test]
fn empty_seed_access_strict_invalidation_and_retained_recipients_survive_new_inputs() {
    for read in ["lookup_request", "lookup_captured", "original_request"] {
        let mut f = fixture("irreversible", false, true);
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
        let mut f = fixture("irreversible", false, true);
        f.b["data"][cell]["invalidates"] = json!({});
        refuse(&f, "owned-continuation-invalidation-coverage");
    }
    let mut f = fixture("irreversible", false, true);
    f.b["expressions"]["lookup_input"]["body"] = json!({"op":"record","fields":{}});
    f.b["expressions"]["lookup_input"]["reads"] = json!({});
    f.b["nodes"]["lookup"]["reads"] = json!({});
    f.b["data"]["original_request"]["protection"]["capabilities"] = json!({"adapter":true});
    // Preserve the same narrow label at the preceding capture publication so
    // the probe reaches actual capability egress rather than an earlier leak.
    f.b["data"]["captured"]["protection"]["capabilities"] = json!({"adapter":true});
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-recipient");
}
#[test]
fn new_pure_input_permission_keeps_normal_read_and_type_checks() {
    let mut f = promote(command_audit::captured_input(true));
    f.b["nodes"]["lookup"]["reads"] = json!({});
    refuse(&f, "node-reads");
    let mut f = promote(command_audit::captured_input(true));
    f.b["expressions"]["lookup_input"]["body"] =
        json!({"op":"literal","type":{"kind":"text"},"value":"wrong type"});
    f.b["expressions"]["lookup_input"]["resultType"] = wrap(json!({"kind":"text"}));
    f.b["expressions"]["lookup_input"]["reads"] = json!({});
    f.b["nodes"]["lookup"]["reads"] = json!({});
    refuse(&f, "activity-input");
}
#[test]
fn a_compensator_cannot_substitute_another_visible_ancestor_for_its_actual_source() {
    let mut f = promote(super::readonly_continuation::three_level_fixture());
    f.b["policies"]["third_work"] = f.b["policies"]["lookup_work"].clone();
    f.b["nodes"]["third_query"]["policy"] = "third_work".into();
    bind(&mut f, "third_query", "irreversible", "stable-key");
    f.b["policies"]["third_work"]["scope"] = "lookup_recovery".into();
    f.b["policies"]["third_work"]["body"]["compensates"] = "origin".into();
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-compensation-source");
    f.b["policies"]["third_work"]["body"]["compensates"] = "lookup".into();
    check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
}
#[test]
fn repeated_creators_share_work_templates_without_supplying_occurrence_values() {
    let mut f = fixture("irreversible", true, true);
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
    let scopes: Vec<_> = tree
        .continuations()
        .iter()
        .map(|root| root.scopes().next().unwrap())
        .collect();
    let work: Vec<_> = scopes.iter().map(|scope| scope.work().unwrap()).collect();
    assert!(std::ptr::eq(work[0], work[1]));
}

#[test]
fn shared_isolated_cases_use_actual_sdk_with_explicit_matching_metadata_premises() {
    let corpus: Value =
        serde_json::from_slice(include_bytes!("../conformance/finite-owned-work.json")).unwrap();
    assert_eq!(
        corpus["core"],
        json!({"id":CORE_V11.0,"revision":CORE_V11.1})
    );
    for case in corpus["cases"].as_array().unwrap() {
        let setup = &case["setup"];
        let class = setup["class"].as_str().unwrap();
        let capturing = setup["capturing"].as_bool().unwrap();
        let linked = setup["compensates"].as_bool().unwrap();
        let key = setup["idempotency"].as_str().unwrap();
        let mut f = if setup["input"] == "captured" {
            let mut f = promote(command_audit::captured_input(capturing));
            bind(&mut f, "lookup", class, key);
            if linked {
                command_audit::compensates(&mut f);
            }
            f
        } else {
            let mut f = fixture(class, linked, capturing);
            bind(&mut f, "lookup", class, key);
            if setup["input"] == "constant" {
                f.b["expressions"]["lookup_input"]["body"] = json!({"op":"record","fields":{}});
                f.b["expressions"]["lookup_input"]["reads"] = json!({});
                f.b["nodes"]["lookup"]["reads"] = json!({});
            } else {
                assert_eq!(setup["input"], "original");
            }
            f
        };
        for change in case["put"].as_array().unwrap() {
            let path = change["path"].as_str().unwrap();
            let (parent, key) = path.rsplit_once('/').unwrap();
            f.b.pointer_mut(parent)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .insert(key.into(), change["value"].clone());
        }
        let r = check_reconciliation_availability(&env(&f), &f.supplied());
        let actual = r.as_ref().map(|_| "covered").unwrap_or_else(|e| e.code);
        assert_eq!(actual, case["expect"], "{}: {:?}", case["id"], r.err());
    }
}
