// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Original-request declarations and conditional proof, no actual retained values.
use super::*;
use choreoform_semantic_check::{CORE_V7, CORE_V8, check_concurrency};
fn fixture() -> Fixture {
    let mut f = super::general_fault::fixture();
    for s in f.b["scopes"].as_object_mut().unwrap().values_mut() {
        s["faultInput"] = Value::Null;
        s["retainedRequests"] = json!({});
    }
    f.b["dialects"]["core"] = json!({"id":CORE_V8.0,"revision":CORE_V8.1});
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for p in [CORE_V7, CORE_V8] {
        f.extra.push((
            Pin {
                id: p.0.into(),
                revision: p.1.into(),
            },
            std::fs::read(root.join(format!("docs/ir/contracts/{}.txt", p.1.replace(':', "-"))))
                .unwrap(),
        ));
    }
    f.b["scopes"]["payment_recovery"]["retainedRequests"] =
        json!({"original_request":"payment_request_step"});
    let ty = f.b["capabilities"]["payment_capability"]["input"].clone();
    f.b["data"]["original_request"] = json!({"scope":"payment_recovery","type":ty,"protection":f.b["data"]["payment_result"]["protection"],"invalidates":{"request_branch":true}});
    f.b["data"]["request_copy"] = f.b["data"]["original_request"].clone();
    f.b["data"]["request_copy"]["invalidates"] = json!({});
    f.b["data"]["request_copy"]["initial"] = "copy_original".into();
    f.b["expressions"]["copy_original"] = json!({"scope":"payment_recovery","dialect":"core","parameters":{},"reads":{"original_request":true},"resultType":ty,"body":{"op":"read","cell":"original_request"}});
    let revision = json!({"op":"field","name":"revision","value":{"op":"unwrapNamed","value":{"op":"field","name":"claim","value":{"op":"read","cell":"original_request"}}}});
    f.b["expressions"]["request_guard"] = json!({"scope":"payment_recovery","dialect":"core","parameters":{},"reads":{"original_request":true},"resultType":wrap(json!({"kind":"boolean"})),"body":{"op":"eq","left":revision,"right":revision}});
    let key = f.b["flows"]
        .as_object()
        .unwrap()
        .iter()
        .find(|(_, v)| v["source"] == "payment_observe")
        .unwrap()
        .0
        .clone();
    let done = f.b["flows"][&key]["target"].clone();
    f.b["flows"][&key]["target"] = "request_branch".into();
    f.b["nodes"]["request_branch"] = json!({"kind":"decision","scope":"payment_recovery","reads":{"original_request":true},"writes":{},"outcomes":{"yes":true,"no":true},"guards":{"yes":"request_guard"},"default":"no"});
    for outcome in ["yes", "no"] {
        f.b["flows"][format!("request_{outcome}")] =
            json!({"source":"request_branch","outcome":outcome,"target":done});
    }
    let invalidates = f.b["nodes"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(_, n)| n["scope"] == "payment_recovery")
        .map(|(id, _)| (id.clone(), Value::Bool(true)))
        .collect::<serde_json::Map<String, Value>>();
    f.b["data"]["original_request"]["invalidates"] = Value::Object(invalidates.clone());
    f.b["data"]["late_payment_fact"]["invalidates"] = Value::Object(invalidates);
    f
}
fn env(f: &Fixture) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.4.0", f.b.clone(), json!({})).unwrap()
}
fn refuse(f: &Fixture, code: &str) {
    assert_eq!(check(&env(f), &f.supplied()).unwrap_err().code, code);
}
fn proof_refuse(f: &Fixture, code: &str) {
    assert_eq!(
        check_concurrency(&env(f), &f.supplied()).unwrap_err().code,
        code
    );
}
#[test]
fn exact_original_input_and_copy_branch_have_only_conditional_owned_proof() {
    let f = fixture();
    let e = env(&f);
    let supplied = f.supplied();
    let r = check(&e, &supplied).unwrap();
    let [row] = r.retained_requests() else {
        panic!("one declaration")
    };
    assert_eq!(
        (row.scope(), row.origin(), row.result()),
        (
            "payment_recovery",
            "payment_request_step",
            "original_request"
        )
    );
    assert_eq!(row.revision(), e.document()["revision"]);
    assert!(!row.host_requirements().is_empty());
    let ordinary = check_control(&e, &supplied).unwrap();
    assert!(!ordinary.covered_nodes().contains("request_branch"));
    let c = check_concurrency(&e, &supplied).unwrap();
    let owned = c.owned_local().unwrap();
    assert!(owned.checked_nodes().contains("request_branch"));
    assert!(
        c.invalidation()
            .uncovered_nodes()
            .contains("request_branch")
    );
    let binding = owned
        .followups()
        .iter()
        .find(|p| p.followup() == "payment_recovery")
        .unwrap()
        .request_snapshot()
        .unwrap();
    assert_eq!(
        (binding.origin(), binding.result()),
        ("payment_request_step", "original_request")
    );
    assert!(
        owned
            .required_provenance()
            .iter()
            .any(|p| p.site() == "payment_request_step" && p.provider_role() == Some("activity"))
    );
    assert!(
        owned
            .accesses()
            .iter()
            .any(|a| a.cell() == "original_request" && a.operation() == "write")
    );
    let final_report = check_reconciliation_availability(&e, &supplied).unwrap();
    assert!(
        final_report
            .retained_request_scopes()
            .contains("payment_recovery")
    );
    assert!(!final_report.checked_nodes().contains("request_branch"));
    assert!(final_report.uncovered_nodes().contains("request_branch"));
}
#[test]
fn new_fields_cannot_be_ignored_by_old_cores_and_exact_bases_are_required() {
    let mut f = fixture();
    f.b["dialects"]["core"] = json!({"id":CORE_V7.0,"revision":CORE_V7.1});
    refuse(&f, "retained-request-wire-version");
    let f = fixture();
    let e = env(&f);
    for (p, _) in f
        .extra
        .iter()
        .filter(|(p, _)| p.id.contains("core-profile"))
    {
        let supplied: Vec<_> = f.supplied().into_iter().filter(|r| r.id != p.id).collect();
        assert!(check(&e, &supplied).is_err(), "{}", p.id);
    }
}
#[test]
fn initializer_and_unvisited_writer_cannot_impersonate_original_request() {
    let mut f = fixture();
    f.b["data"]["original_request"]["initial"] = "copy_original".into();
    refuse(&f, "retained-request-readonly");
    let mut f = fixture();
    f.b["nodes"]["forged_write"] = json!({"kind":"compute","scope":"payment_recovery","reads":{"original_request":true},"writes":{"original_request":true},"assignments":{"original_request":"copy_original"},"outcomes":{"next":true}});
    let finish = f.b["flows"]["request_yes"]["target"].clone();
    f.b["flows"]["forged_next"] = json!({"source":"forged_write","outcome":"next","target":finish});
    refuse(&f, "retained-request-readonly");
}
#[test]
fn wrong_source_scope_missing_capture_and_nonlocal_storage_refuse() {
    let mut f = fixture();
    f.b["scopes"]["payment_recovery"]["retainedRequests"]["original_request"] =
        "notice_request_step".into();
    refuse(&f, "retained-request-origin");
    let mut f = fixture();
    f.b["scopes"]["payment_recovery"]["entry"] = "request_branch".into();
    refuse(&f, "retained-request-capture");
    let mut f = fixture();
    f.b["scopes"]["expense"]["retainedRequests"] = json!({"current_claim":"payment_request_step"});
    refuse(&f, "retained-request-owner");
}
#[test]
fn constant_and_initializer_destinations_retain_source_control_protection() {
    let mut f = fixture();
    f.b["data"]["request_copy"]["protection"]["sensitivity"] = "public".into();
    proof_refuse(&f, "control-information-flow");
    let mut f = fixture();
    f.b["data"]["original_request"]["protection"]["sensitivity"] = "public".into();
    proof_refuse(&f, "control-information-flow");
    let mut f = fixture();
    let mut d = f.b["data"]["request_copy"].clone();
    d["type"] = wrap(json!({"kind":"text"}));
    d["protection"]["sensitivity"] = "public".into();
    d["initial"] = "constant_flag".into();
    f.b["data"]["public_flag"] = d;
    f.b["expressions"]["constant_flag"] = json!({"scope":"payment_recovery","dialect":"core","parameters":{},"reads":{},"resultType":wrap(json!({"kind":"text"})),"body":{"op":"literal","type":{"kind":"text"},"value":"flag"}});
    proof_refuse(&f, "control-information-flow");
}
#[test]
fn preentry_self_access_and_missing_strict_invalidation_refuse() {
    for read in ["original_request", "request_copy", "current_claim"] {
        let mut f = fixture();
        let reads = serde_json::Map::from_iter([(read.into(), Value::Bool(true))]);
        f.b["policies"]["request_access"] = json!({"scope":"payment_recovery","dialect":"core","body":{"kind":"protection","reads":reads,"permit":{"op":"eq","left":{"op":"read","cell":read},"right":{"op":"read","cell":read}}}});
        f.b["data"]["original_request"]["protection"]["policy"] = "request_access".into();
        f.b["nodes"]["request_branch"]["reads"][read] = true.into();
        proof_refuse(
            &f,
            if read == "current_claim" {
                "scope-lifetime-unproved"
            } else {
                "retained-request-access-precheck"
            },
        );
    }
    let mut f = fixture();
    f.b["data"]["original_request"]["invalidates"] = json!({});
    proof_refuse(&f, "owned-local-invalidation-coverage");
}
#[test]
fn changed_types_and_ordinary_creator_or_ports_do_not_become_snapshots() {
    let mut f = fixture();
    for id in ["original_request", "request_copy"] {
        f.b["data"][id]["type"] = wrap(json!({"kind":"text"}));
    }
    f.b["expressions"]["copy_original"]["resultType"] = wrap(json!({"kind":"text"}));
    f.b["expressions"]["request_guard"]["body"] = boolean(true);
    f.b["expressions"]["request_guard"]["reads"] = json!({});
    f.b["nodes"]["request_branch"]["reads"] = json!({});
    refuse(&f, "retained-request-type");
    let mut f = fixture();
    f.b["nodes"]["ordinary_recovery_call"] = json!({"kind":"invoke","scope":"payment_service","body":"payment_recovery","inputs":{},"outputs":{},"reads":{},"writes":{},"outcomes":{"complete":true}});
    f.b["flows"]["ordinary_recovery_next"] =
        json!({"source":"ordinary_recovery_call","outcome":"complete","target":"payment_done"});
    // Other established interface/read checks may refuse this creator first.
    assert!(check(&env(&f), &f.supplied()).is_err());
    let mut f = fixture();
    f.b["scopes"]["payment_recovery"]["inputs"] = json!({"input":"original_request"});
    refuse(&f, "closure-ports");
}
#[test]
fn an_unvisited_declared_origin_is_not_covered_by_a_different_live_request() {
    let mut f = fixture();
    let mut n = f.b["nodes"]["payment_request_step"].clone();
    let mut cell = f.b["data"]["payment_result"].clone();
    cell["invalidates"] = json!({});
    f.b["data"]["extra_result"] = cell;
    n["result"] = "extra_result".into();
    n["writes"] = json!({"extra_result":true});
    for outcome in n["outcomes"].as_object().unwrap().keys() {
        f.b["flows"][format!("extra_{outcome}")] =
            json!({"source":"extra_request","outcome":outcome,"target":"payment_done"});
    }
    f.b["nodes"]["extra_request"] = n;
    f.b["scopes"]["payment_recovery"]["retainedRequests"]["original_request"] =
        "extra_request".into();
    f.b["nodes"]["payment_observe"]["capture"]["origin"] = "extra_request".into();
    check(&env(&f), &f.supplied()).unwrap();
    proof_refuse(&f, "retained-request-source-control-unproved");
}
#[test]
fn current_ancestor_values_do_not_appear_when_an_original_request_is_retained() {
    let mut f = fixture();
    let current = f.b["expressions"][f.b["nodes"]["payment_request_step"]["input"]
        .as_str()
        .unwrap()]
    .clone();
    f.b["expressions"]["copy_original"]["body"] = current["body"].clone();
    f.b["expressions"]["copy_original"]["reads"] = current["reads"].clone();
    check(&env(&f), &f.supplied()).unwrap();
    proof_refuse(&f, "owned-local-availability-unproved");
}
#[test]
fn cardinality_and_fault_handler_reuse_are_explicitly_outside_the_subset() {
    let mut f = fixture();
    f.b["scopes"]["payment_recovery"]["retainedRequests"]["request_copy"] =
        "payment_request_step".into();
    f.b["data"]["request_copy"]
        .as_object_mut()
        .unwrap()
        .remove("initial");
    refuse(&f, "retained-request-cardinality");
    let mut f = fixture();
    f.b["policies"]["reuse_handler"] = json!({"scope":"payment_service","dialect":"core","body":{"kind":"faults","reads":{},"handlers":{"provider unknown":"payment_recovery"}}});
    f.b["scopes"]["payment_service"]["faults"] = "reuse_handler".into();
    refuse(&f, "retained-request-handler");
}
#[test]
fn shared_finite_cases_are_exercised_by_the_actual_sdk() {
    let corpus: Value =
        serde_json::from_slice(include_bytes!("../conformance/retained-request.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let mut f = fixture();
        match case["id"].as_str().unwrap() {
            "covered" => {}
            "old-core" => f.b["dialects"]["core"] = json!({"id":CORE_V7.0,"revision":CORE_V7.1}),
            "two-snapshots" => {
                f.b["scopes"]["payment_recovery"]["retainedRequests"]["request_copy"] =
                    "payment_request_step".into();
                f.b["data"]["request_copy"]
                    .as_object_mut()
                    .unwrap()
                    .remove("initial");
            }
            "wrong-origin" => {
                f.b["scopes"]["payment_recovery"]["retainedRequests"]["original_request"] =
                    "notice_request_step".into()
            }
            "missing-entry-capture" => {
                f.b["scopes"]["payment_recovery"]["entry"] = "request_branch".into()
            }
            "initialized" => f.b["data"]["original_request"]["initial"] = "copy_original".into(),
            "public-snapshot" => {
                f.b["data"]["original_request"]["protection"]["sensitivity"] = "public".into()
            }
            "public-copy" => {
                f.b["data"]["request_copy"]["protection"]["sensitivity"] = "public".into()
            }
            "missing-invalidation" => f.b["data"]["original_request"]["invalidates"] = json!({}),
            "handler-reuse" => {
                f.b["policies"]["reuse_handler"] = json!({"scope":"payment_service","dialect":"core","body":{"kind":"faults","reads":{},"handlers":{"provider unknown":"payment_recovery"}}});
                f.b["scopes"]["payment_service"]["faults"] = "reuse_handler".into();
            }
            "self-access" => {
                f.b["policies"]["request_access"] = json!({"scope":"payment_recovery","dialect":"core","body":{"kind":"protection","reads":{"original_request":true},"permit":{"op":"eq","left":{"op":"read","cell":"original_request"},"right":{"op":"read","cell":"original_request"}}}});
                f.b["data"]["original_request"]["protection"]["policy"] = "request_access".into();
            }
            "unvisited-origin" => {
                let mut n = f.b["nodes"]["payment_request_step"].clone();
                let mut cell = f.b["data"]["payment_result"].clone();
                cell["invalidates"] = json!({});
                f.b["data"]["extra_result"] = cell;
                n["result"] = "extra_result".into();
                n["writes"] = json!({"extra_result":true});
                for outcome in n["outcomes"].as_object().unwrap().keys() {
                    f.b["flows"][format!("extra_{outcome}")] =
                        json!({"source":"extra_request","outcome":outcome,"target":"payment_done"});
                }
                f.b["nodes"]["extra_request"] = n;
                f.b["scopes"]["payment_recovery"]["retainedRequests"]["original_request"] =
                    "extra_request".into();
                f.b["nodes"]["payment_observe"]["capture"]["origin"] = "extra_request".into();
            }
            id => panic!("unknown case {id}"),
        }
        let outcome = match check_concurrency(&env(&f), &f.supplied()) {
            Ok(_) => "covered",
            Err(e) => e.code,
        };
        assert_eq!(outcome, case["expect"], "{}", case["id"]);
    }
}
