// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Raw declaration checks do not initialize the reserved history cell.
use super::*;
use choreoform_semantic_check::{CORE_V7, FAULT_HISTORY_PROFILE, FAULT_HISTORY_TYPES};
fn fixture() -> Fixture {
    let mut f = super::general_fault::fixture();
    for s in f.b["scopes"].as_object_mut().unwrap().values_mut() {
        s["faultInput"] = Value::Null;
    }
    f.b["dialects"]["core"] = json!({"id":CORE_V7.0,"revision":CORE_V7.1});
    for (p,raw) in [
        (CORE_V7,include_bytes!("../../../../docs/ir/contracts/sha256-e286e54441b3d785ae823eef359c12d64fb8405829c70709227282e5af62f54b.txt").as_slice()),
        (FAULT_HISTORY_PROFILE,include_bytes!("../../../../docs/ir/contracts/sha256-2f580f46eb6ef9b7fff42906f6b0b5cafc6c5fa71ae14365828935bfe2da070c.txt")),
        (FAULT_HISTORY_TYPES,include_bytes!("../../../../docs/ir/contracts/sha256-a2e177ce1dd4bad5b9cd68b74e6da7749ec2b04b68a39dae1bd72729e2aae7a7.txt")),
    ]{f.extra.push((Pin{id:p.0.into(),revision:p.1.into()},raw.to_vec()));}
    let history = json!({"kind":"provider-fault-history","version":"0.1.0","id":"urn:test:payment-history","profile":{"id":FAULT_HISTORY_PROFILE.0,"revision":FAULT_HISTORY_PROFILE.1},"source":f.b["policies"]["payment_work"]["body"]["faultSource"]});
    let h = f.artifact(history);
    let mut handler = f.b["scopes"]["remaining"].clone();
    handler["parent"] = "payment_service".into();
    handler["entry"] = "handler_done".into();
    handler["faultInput"] =
        json!({"history":{"id":h.id,"revision":h.revision},"result":"fault_history"});
    f.b["scopes"]["provider_handler"] = handler;
    f.b["policies"]["payment_faults"] = json!({"scope":"payment_service","dialect":"core","body":{"kind":"faults","reads":{},"handlers":{"provider unknown":"provider_handler"}}});
    f.b["scopes"]["payment_service"]["faults"] = "payment_faults".into();
    f.b["nodes"]["handler_done"] = json!({"kind":"finish","scope":"provider_handler","reads":{},"writes":{},"outcomes":{},"outcome":"complete"});
    f.b["data"]["fault_history"] = json!({"scope":"provider_handler","type":wrap(json!({"kind":"named","contract":{"id":FAULT_HISTORY_TYPES.0,"revision":FAULT_HISTORY_TYPES.1},"name":"Provider fault history"})),"protection":f.b["data"]["payment_result"]["protection"],"invalidates":{}});
    f
}
fn envelope(f: &Fixture) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.3.0", f.b.clone(), json!({})).unwrap()
}
fn refuse(f: &Fixture, code: &str) {
    let e = check(&envelope(f), &f.supplied()).unwrap_err();
    assert_eq!(e.code, code, "{e:?}");
}
fn forged(f: &mut Fixture) {
    let corpus: Value =
        serde_json::from_slice(include_bytes!("../conformance/provider-fault-history.json"))
            .unwrap();
    f.b["expressions"]["forged_history"] = json!({"scope":"provider_handler","dialect":"core","parameters":{},"reads":{},"resultType":f.b["data"]["fault_history"]["type"],"body":{"op":"literal","type":f.b["data"]["fault_history"]["type"]["body"],"value":corpus["base"]}});
}
#[test]
fn raw_binding_retains_exact_local_nominal_source_without_initializing_it() {
    let f = fixture();
    let e = envelope(&f);
    let r = check(&e, &f.supplied()).unwrap();
    assert_eq!(r.fault_entries().len(), 1);
    let row = &r.fault_entries()[0];
    assert_eq!(row.handler(), "provider_handler");
    assert_eq!(row.result(), "fault_history");
    assert_eq!(row.revision(), e.document()["revision"]);
    assert_eq!(
        row.history_metadata().history_type(),
        &choreoform_semantic_check::Type::Named(choreoform_semantic_check::Nominal {
            contract: Pin {
                id: FAULT_HISTORY_TYPES.0.into(),
                revision: FAULT_HISTORY_TYPES.1.into()
            },
            name: "Provider fault history".into()
        })
    );
    assert_eq!(
        row.history_metadata().source_metadata().pin(),
        &Pin::decode(&f.b["policies"]["payment_work"]["body"]["faultSource"]).unwrap()
    );
    assert!(!row.host_requirements().is_empty());
    assert_eq!(r.fault_source_uses().len(), 2);
    assert_eq!(r.capture_bindings().len(), 1);
}
#[test]
fn older_dialects_cannot_ignore_new_scope_field_and_core7_requires_new_wire() {
    let mut f = fixture();
    f.b["dialects"]["core"] = json!({"id":choreoform_semantic_check::CORE_V6.0,"revision":choreoform_semantic_check::CORE_V6.1});
    refuse(&f, "fault-entry-wire-version");
    let mut f = fixture();
    for s in f.b["scopes"].as_object_mut().unwrap().values_mut() {
        s.as_object_mut().unwrap().remove("faultInput");
    }
    let e = DefinitionEnvelope::from_parts_versioned("0.2.0", f.b.clone(), json!({})).unwrap();
    assert_eq!(
        check(&e, &f.supplied()).unwrap_err().code,
        "fault-entry-wire-version"
    );
}
#[test]
fn ordinary_nominal_initializer_and_even_unvisited_writer_cannot_impersonate_entry() {
    let mut f = fixture();
    forged(&mut f);
    f.b["data"]["fault_history"]["initial"] = "forged_history".into();
    refuse(&f, "fault-entry-readonly");
    let mut f = fixture();
    forged(&mut f);
    f.b["nodes"]["forged_write"] = json!({"kind":"compute","scope":"provider_handler","reads":{},"writes":{"fault_history":true},"outcomes":{"next":true},"assignments":{"fault_history":"forged_history"}});
    f.b["flows"]["forged_flow"] =
        json!({"source":"forged_write","outcome":"next","target":"handler_done"});
    refuse(&f, "fault-entry-readonly");
}
#[test]
fn root_wrong_nominal_and_ordinary_creator_are_not_handler_entries() {
    let mut f = fixture();
    f.b["scopes"]["expense"]["faultInput"] = json!({"history":f.b["scopes"]["provider_handler"]["faultInput"]["history"],"result":"current_claim"});
    refuse(&f, "fault-entry-handler");
    let mut f = fixture();
    f.b["data"]["fault_history"]["type"] = wrap(json!({"kind":"text"}));
    refuse(&f, "fault-entry-type");
    let mut f = fixture();
    f.b["nodes"]["ordinary_handler_call"] = json!({"kind":"invoke","scope":"payment_service","body":"provider_handler","inputs":{},"outputs":{},"reads":{},"writes":{},"outcomes":{"complete":true}});
    let mut reads = serde_json::Map::new();
    for field in ["cancellation", "faults", "closure", "race"] {
        let policy = f.b["scopes"]["payment_service"][field].as_str().unwrap();
        reads.extend(
            f.b["policies"][policy]["body"]["reads"]
                .as_object()
                .unwrap()
                .clone(),
        );
    }
    f.b["nodes"]["ordinary_handler_call"]["reads"] = reads.into();
    f.b["flows"]["ordinary_handler_flow"] =
        json!({"source":"ordinary_handler_call","outcome":"complete","target":"payment_done"});
    refuse(&f, "fault-entry-creator");
}

fn readable_body(f: &mut Fixture) {
    f.b["expressions"]["history_read"] = json!({"scope":"provider_handler","dialect":"core","parameters":{},"reads":{"fault_history":true},"resultType":f.b["data"]["fault_history"]["type"],"body":{"op":"read","cell":"fault_history"}});
    f.b["expressions"]["history_guard"] = json!({"scope":"provider_handler","dialect":"core","parameters":{},"reads":{"fault_history":true},"resultType":wrap(json!({"kind":"boolean"})),"body":{"op":"eq","left":{"op":"field","name":"type","value":{"op":"unwrapNamed","value":{"op":"read","cell":"fault_history"}}},"right":{"op":"literal","type":{"kind":"text"},"value":"provider unknown"}}});
    let mut copy = f.b["data"]["fault_history"].clone();
    copy["initial"] = "history_read".into();
    f.b["data"]["history_copy"] = copy;
    f.b["nodes"]["history_branch"] = json!({"kind":"decision","scope":"provider_handler","reads":{"fault_history":true},"writes":{},"outcomes":{"yes":true,"no":true},"guards":{"yes":"history_guard"},"default":"no"});
    for outcome in ["yes", "no"] {
        f.b["flows"][format!("history_{outcome}")] =
            json!({"source":"history_branch","outcome":outcome,"target":"handler_done"});
    }
    f.b["scopes"]["provider_handler"]["entry"] = "history_branch".into();
    f.b["data"]["fault_history"]["invalidates"] = json!({"history_branch":true});
}
#[test]
fn conditional_history_initialization_flow_and_strict_local_invalidation_are_separate() {
    let mut f = fixture();
    readable_body(&mut f);
    let e = envelope(&f);
    let supplied = f.supplied();
    let r = choreoform_semantic_check::check_fault_entries(&e, &supplied).unwrap();
    assert!(r.covered_entries().contains("provider_handler"));
    assert!(r.uncovered_entries().is_empty());
    assert!(r.checked_nodes().contains("history_branch"));
    assert!(r.checked_scopes().contains("provider_handler"));
    assert!(r.transfers() > 0);
    assert!(
        r.accesses()
            .iter()
            .any(|a| a.cell() == "fault_history" && a.operation() == "write")
    );
    assert!(
        r.obligations()
            .iter()
            .any(|o| o.site() == "history_branch" && o.cell() == "fault_history")
    );
    assert!(
        r.required_provenance()
            .iter()
            .any(|p| p.owner() == "payment_service"
                && p.kind() == "provider"
                && p.site() == "payment_request_step")
    );
    assert!(
        !r.origins().faults().static_report().fault_entries()[0]
            .host_requirements()
            .is_empty()
    );
    assert!(
        !r.origins()
            .faults()
            .timing()
            .concurrency()
            .invalidation()
            .protection()
            .control()
            .covered_nodes()
            .contains("history_branch")
    );
    assert_eq!(
        choreoform_semantic_check::check_exceptional_availability(&e, &supplied)
            .unwrap_err()
            .code,
        "exceptional-availability-unproved"
    );
}

fn proof_refuse(f: &Fixture, code: &str) {
    let e =
        choreoform_semantic_check::check_fault_entries(&envelope(f), &f.supplied()).unwrap_err();
    assert_eq!(e.code, code, "{e:?}");
}
#[test]
fn every_incoming_source_must_be_supported_exact_and_ordinary_reachable() {
    let mut f = fixture();
    let original =
        Pin::decode(&f.b["scopes"]["provider_handler"]["faultInput"]["history"]).unwrap();
    let mut descriptor: Value =
        serde_json::from_slice(&f.extra.iter().find(|(p, _)| p == &original).unwrap().1).unwrap();
    descriptor["source"] = f.b["policies"]["notice_work"]["body"]["faultSource"].clone();
    let p = f.artifact(descriptor);
    f.b["scopes"]["provider_handler"]["faultInput"]["history"] =
        json!({"id":p.id,"revision":p.revision});
    proof_refuse(&f, "fault-entry-source");
    for human in [false, true] {
        let mut f = fixture();
        let owner = if human {
            f.b["nodes"]
                .as_object()
                .unwrap()
                .values()
                .find(|n| n["kind"] == "activity" && n["mode"] == "human")
                .unwrap()["scope"]
                .as_str()
                .unwrap()
                .to_owned()
        } else {
            "checks_service".into()
        };
        f.b["scopes"]["payment_service"]["faults"] = "no_handlers".into();
        f.b["scopes"]["provider_handler"]["parent"] = owner.clone().into();
        f.b["scopes"][&owner]["faults"] = "payment_faults".into();
        f.b["policies"]["payment_faults"]["scope"] = owner.into();
        proof_refuse(&f, "fault-entry-origin-unproved");
    }
    let mut f = fixture();
    let mut result = f.b["data"]["payment_result"].clone();
    result["invalidates"] = json!({});
    f.b["data"]["unvisited_payment_result"] = result;
    let mut n = f.b["nodes"]["payment_request_step"].clone();
    n["result"] = "unvisited_payment_result".into();
    n["writes"] = json!({"unvisited_payment_result":true});
    f.b["nodes"]["unvisited_payment"] = n;
    f.b["flows"]["unvisited_payment_flow"] =
        json!({"source":"unvisited_payment","outcome":"received","target":"payment_done"});
    // Preserve the exact declared outcome spelling from the actual source node.
    let outcomes: Vec<_> = f.b["nodes"]["payment_request_step"]["outcomes"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    f.b["flows"]
        .as_object_mut()
        .unwrap()
        .remove("unvisited_payment_flow");
    for (i, outcome) in outcomes.iter().enumerate() {
        f.b["flows"][format!("unvisited_payment_{i}")] =
            json!({"source":"unvisited_payment","outcome":outcome,"target":"payment_done"});
    }
    proof_refuse(&f, "fault-entry-source-control-unproved");
}
#[test]
fn no_incoming_context_is_uncovered_and_exceptional_source_context_is_not_borrowed() {
    let mut f = fixture();
    let mut scope = f.b["scopes"]["remaining"].clone();
    scope["parent"] = "payment_service".into();
    scope["entry"] = "unused_done".into();
    scope["faults"] = "payment_faults".into();
    f.b["scopes"]["unused_owner"] = scope;
    f.b["nodes"]["unused_done"] = json!({"kind":"finish","scope":"unused_owner","reads":{},"writes":{},"outcomes":{},"outcome":"complete"});
    f.b["scopes"]["provider_handler"]["parent"] = "unused_owner".into();
    f.b["policies"]["payment_faults"]["scope"] = "unused_owner".into();
    f.b["scopes"]["payment_service"]["faults"] = "no_handlers".into();
    let r = choreoform_semantic_check::check_fault_entries(&envelope(&f), &f.supplied()).unwrap();
    assert!(r.covered_entries().is_empty());
    assert!(r.checked_nodes().is_empty());
    assert!(r.uncovered_entries().contains("provider_handler"));
    let mut f = fixture();
    let mut first = f.b["scopes"]["remaining"].clone();
    first["parent"] = "payment_service".into();
    first["entry"] = "exceptional_request".into();
    first["faults"] = "first_faults".into();
    first["race"] = f.b["scopes"]["payment_service"]["race"].clone();
    f.b["scopes"]["first_handler"] = first;
    f.b["policies"]["first_faults"] = json!({"scope":"first_handler","dialect":"core","body":{"kind":"faults","reads":{},"handlers":{"provider unknown":"provider_handler"}}});
    f.b["policies"]["payment_faults"]["body"]["handlers"]["provider unknown"] =
        "first_handler".into();
    f.b["scopes"]["provider_handler"]["parent"] = "first_handler".into();
    let mut result = f.b["data"]["payment_result"].clone();
    result["scope"] = "first_handler".into();
    result["invalidates"] = json!({});
    f.b["data"]["exceptional_result"] = result;
    let mut n = f.b["nodes"]["payment_request_step"].clone();
    n["scope"] = "first_handler".into();
    n["result"] = "exceptional_result".into();
    n["writes"] = json!({"exceptional_result":true});
    f.b["nodes"]["exceptional_request"] = n;
    f.b["nodes"]["first_done"] = json!({"kind":"finish","scope":"first_handler","reads":{},"writes":{},"outcomes":{},"outcome":"complete"});
    let outcomes: Vec<_> = f.b["nodes"]["payment_request_step"]["outcomes"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    for (i, outcome) in outcomes.iter().enumerate() {
        f.b["flows"][format!("exceptional_flow_{i}")] =
            json!({"source":"exceptional_request","outcome":outcome,"target":"first_done"});
    }
    let mut reads = serde_json::Map::new();
    for field in ["cancellation", "faults", "closure", "race"] {
        let p = f.b["scopes"]["first_handler"][field].as_str().unwrap();
        reads.extend(
            f.b["policies"][p]["body"]["reads"]
                .as_object()
                .unwrap()
                .clone(),
        );
    }
    f.b["nodes"]["first_done"]["reads"] = reads.into();
    proof_refuse(&f, "fault-entry-source-control-unproved");
}
#[test]
fn history_and_initializers_cannot_drop_source_or_branch_protection() {
    let mut f = fixture();
    readable_body(&mut f);
    f.b["data"]["fault_history"]["protection"]["sensitivity"] = "public".into();
    proof_refuse(&f, "control-information-flow");
    let mut f = fixture();
    readable_body(&mut f);
    f.b["data"]["history_copy"]["protection"]["sensitivity"] = "public".into();
    proof_refuse(&f, "control-information-flow");
    let mut f = fixture();
    readable_body(&mut f);
    let mut flag = f.b["data"]["fault_history"].clone();
    flag["type"] = wrap(json!({"kind":"boolean"}));
    flag["protection"]["sensitivity"] = "public".into();
    flag["invalidates"] = json!({});
    f.b["data"]["public_flag"] = flag;
    f.b["expressions"]["flag_true"] = json!({"scope":"provider_handler","dialect":"core","parameters":{},"reads":{},"resultType":wrap(json!({"kind":"boolean"})),"body":boolean(true)});
    f.b["nodes"]["public_compute"] = json!({"kind":"compute","scope":"provider_handler","reads":{},"writes":{"public_flag":true},"outcomes":{"next":true},"assignments":{"public_flag":"flag_true"}});
    f.b["flows"]["history_yes"]["target"] = "public_compute".into();
    f.b["flows"]["public_flag_done"] =
        json!({"source":"public_compute","outcome":"next","target":"handler_done"});
    proof_refuse(&f, "control-information-flow");
}
#[test]
fn pre_entry_access_is_checked_before_history_seed_and_strict_invalidation_is_required() {
    let mut f = fixture();
    f.b["policies"]["history_access"] = json!({"scope":"provider_handler","dialect":"core","body":{"kind":"protection","reads":{"fault_history":true},"permit":{"op":"eq","left":{"op":"read","cell":"fault_history"},"right":{"op":"read","cell":"fault_history"}}}});
    f.b["data"]["fault_history"]["protection"]["policy"] = "history_access".into();
    proof_refuse(&f, "fault-entry-access-precheck");
    let mut f = fixture();
    readable_body(&mut f);
    f.b["data"]["fault_history"]["invalidates"] = json!({});
    proof_refuse(&f, "fault-entry-invalidation-coverage");
}

#[test]
fn unexplored_possible_source_cannot_be_dropped_from_incoming_entry_obligations() {
    let mut f = fixture();
    let mut scope = f.b["scopes"]["remaining"].clone();
    scope["parent"] = "payment_service".into();
    scope["entry"] = "unexplored_request".into();
    scope["race"] = f.b["scopes"]["payment_service"]["race"].clone();
    f.b["scopes"]["unexplored_source"] = scope;
    let mut data = f.b["data"]["payment_result"].clone();
    data["scope"] = "unexplored_source".into();
    data["invalidates"] = json!({});
    f.b["data"]["unexplored_result"] = data;
    let mut n = f.b["nodes"]["payment_request_step"].clone();
    n["scope"] = "unexplored_source".into();
    n["result"] = "unexplored_result".into();
    n["writes"] = json!({"unexplored_result":true});
    f.b["nodes"]["unexplored_request"] = n;
    let mut reads = serde_json::Map::new();
    for field in ["cancellation", "faults", "closure", "race"] {
        let p = f.b["scopes"]["unexplored_source"][field].as_str().unwrap();
        reads.extend(
            f.b["policies"][p]["body"]["reads"]
                .as_object()
                .unwrap()
                .clone(),
        );
    }
    f.b["nodes"]["unexplored_done"] = json!({"kind":"finish","scope":"unexplored_source","reads":reads,"writes":{},"outcomes":{},"outcome":"complete"});
    let outcomes: Vec<_> = f.b["nodes"]["payment_request_step"]["outcomes"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    for (i, outcome) in outcomes.iter().enumerate() {
        f.b["flows"][format!("unexplored_flow_{i}")] =
            json!({"source":"unexplored_request","outcome":outcome,"target":"unexplored_done"});
    }
    proof_refuse(&f, "fault-entry-source-control-unproved");
}

#[test]
fn all_shared_conditional_entry_vectors_match_actual_sdk_without_event_values() {
    let corpus: Value =
        serde_json::from_slice(include_bytes!("../conformance/fault-entry.json")).unwrap();
    assert_eq!(corpus["cases"].as_array().unwrap().len(), 13);
    for c in corpus["cases"].as_array().unwrap() {
        let mut f = fixture();
        readable_body(&mut f);
        match c["id"].as_str().unwrap() {
            "baseline" => {}
            "old-wire" => {
                for s in f.b["scopes"].as_object_mut().unwrap().values_mut() {
                    s.as_object_mut().unwrap().remove("faultInput");
                }
            }
            "other-source" => {
                let p = Pin::decode(&f.b["scopes"]["provider_handler"]["faultInput"]["history"])
                    .unwrap();
                let mut v: Value =
                    serde_json::from_slice(&f.extra.iter().find(|(old, _)| old == &p).unwrap().1)
                        .unwrap();
                v["source"] = f.b["policies"]["notice_work"]["body"]["faultSource"].clone();
                let p = f.artifact(v);
                f.b["scopes"]["provider_handler"]["faultInput"]["history"] =
                    json!({"id":p.id,"revision":p.revision});
            }
            "unsupported-source" => {
                f.b["scopes"]["payment_service"]["faults"] = "no_handlers".into();
                f.b["scopes"]["checks_service"]["faults"] = "payment_faults".into();
                f.b["scopes"]["provider_handler"]["parent"] = "checks_service".into();
                f.b["policies"]["payment_faults"]["scope"] = "checks_service".into();
            }
            "no-incoming" => {
                let mut s = f.b["scopes"]["remaining"].clone();
                s["parent"] = "payment_service".into();
                s["entry"] = "unused_done".into();
                s["faults"] = "payment_faults".into();
                f.b["scopes"]["unused_owner"] = s;
                f.b["nodes"]["unused_done"] = json!({"kind":"finish","scope":"unused_owner","reads":{},"writes":{},"outcomes":{},"outcome":"complete"});
                f.b["scopes"]["payment_service"]["faults"] = "no_handlers".into();
                f.b["scopes"]["provider_handler"]["parent"] = "unused_owner".into();
                f.b["policies"]["payment_faults"]["scope"] = "unused_owner".into();
            }
            "self-precheck" => {
                f.b["policies"]["history_access"] = json!({"scope":"provider_handler","dialect":"core","body":{"kind":"protection","reads":{"fault_history":true},"permit":{"op":"eq","left":{"op":"read","cell":"fault_history"},"right":{"op":"read","cell":"fault_history"}}}});
                f.b["data"]["fault_history"]["protection"]["policy"] = "history_access".into();
            }
            "public-history" => {
                f.b["data"]["fault_history"]["protection"]["sensitivity"] = "public".into()
            }
            "public-initializer" => {
                f.b["data"]["history_copy"]["protection"]["sensitivity"] = "public".into()
            }
            "missing-invalidates" => f.b["data"]["fault_history"]["invalidates"] = json!({}),
            "ancestor-read" => {
                f.b["expressions"]["history_guard"]["body"] = json!({"op":"eq","left":{"op":"read","cell":"current_claim"},"right":{"op":"read","cell":"current_claim"}});
                f.b["expressions"]["history_guard"]["reads"] = json!({"current_claim":true});
                f.b["nodes"]["history_branch"]["reads"] = json!({"current_claim":true});
            }
            "uninitialized-local" => {
                let mut d = f.b["data"]["fault_history"].clone();
                d["type"] = wrap(json!({"kind":"boolean"}));
                f.b["data"]["uninitialized_local"] = d;
                f.b["expressions"]["history_guard"]["body"] =
                    json!({"op":"read","cell":"uninitialized_local"});
                f.b["expressions"]["history_guard"]["reads"] = json!({"uninitialized_local":true});
                f.b["nodes"]["history_branch"]["reads"] = json!({"uninitialized_local":true});
            }
            "child-body" => {
                let mut s = f.b["scopes"]["remaining"].clone();
                s["parent"] = "provider_handler".into();
                s["entry"] = "unused_child_done".into();
                f.b["scopes"]["unused_child"] = s;
                f.b["nodes"]["unused_child_done"] = json!({"kind":"finish","scope":"unused_child","reads":{},"writes":{},"outcomes":{},"outcome":"complete"});
            }
            "wait-body" => {
                f.b["policies"]["handler_wait_policy"] = json!({"scope":"provider_handler","dialect":"core","body":{"kind":"wait","reads":{},"observationType":{"kind":"text"},"match":boolean(true),"authority":"operator_actor","timer":null,"onObservation":"observed","onTimer":null}});
                f.b["nodes"]["handler_wait"] = json!({"kind":"wait","scope":"provider_handler","policy":"handler_wait_policy","capture":null,"reads":{},"writes":{},"outcomes":{"observed":true}});
                f.b["flows"]["handler_wait_done"] =
                    json!({"source":"handler_wait","outcome":"observed","target":"handler_done"});
            }
            other => panic!("unknown finite vector {other}"),
        }
        let version = c["premises"]["wire"].as_str().unwrap();
        let e = DefinitionEnvelope::from_parts_versioned(version, f.b.clone(), json!({})).unwrap();
        let actual = choreoform_semantic_check::check_fault_entries(&e, &f.supplied())
            .map(|r| {
                if r.covered_entries().is_empty() {
                    "uncovered"
                } else {
                    "passed"
                }
            })
            .unwrap_or_else(|e| e.code);
        assert_eq!(actual, c["expect"].as_str().unwrap(), "{}", c["id"]);
    }
}

#[test]
fn every_new_exact_artifact_is_required_and_corrupt_bytes_refuse() {
    for pin in [
        CORE_V7,
        choreoform_semantic_check::CORE_V6,
        CORE_V5,
        CORE_V4,
        CORE_V3,
        CORE_V2,
        CORE,
        SEMANTICS,
        FAULT_HISTORY_PROFILE,
        FAULT_HISTORY_TYPES,
    ] {
        let mut f = fixture();
        f.extra
            .retain(|(p, _)| (p.id.as_str(), p.revision.as_str()) != pin);
        refuse(&f, "resource-missing");
        let mut f = fixture();
        f.extra
            .iter_mut()
            .find(|(p, _)| (p.id.as_str(), p.revision.as_str()) == pin)
            .unwrap()
            .1
            .push(b' ');
        refuse(&f, "resource-digest");
    }
    let mut f = fixture();
    let history = Pin::decode(&f.b["scopes"]["provider_handler"]["faultInput"]["history"]).unwrap();
    f.extra.retain(|(p, _)| p != &history);
    refuse(&f, "resource-missing");
}
#[test]
fn p6_creator_ports_and_private_source_protection_are_not_entry_exceptions() {
    let mut f = fixture();
    f.b["policies"]["payment_close"]["body"]["followup"] = "provider_handler".into();
    refuse(&f, "fault-entry-creator");
    let mut f = fixture();
    f.b["scopes"]["provider_handler"]["inputs"] = json!({"history":"fault_history"});
    refuse(&f, "handler-ports");
    let mut f = fixture();
    let mut access = f.b["policies"]["expense_access"].clone();
    access["scope"] = "notice_service".into();
    f.b["policies"]["private_access"] = access;
    let provider_pin =
        Pin::decode(&f.b["policies"]["payment_capability_provider"]["body"]["provider"]).unwrap();
    let mut provider: Value =
        serde_json::from_slice(&f.extra.iter().find(|(p, _)| p == &provider_pin).unwrap().1)
            .unwrap();
    provider["resultProtection"]["policy"] = "private_access".into();
    let provider = f.artifact(provider);
    let source = Pin::decode(&f.b["policies"]["payment_work"]["body"]["faultSource"]).unwrap();
    let mut value: Value =
        serde_json::from_slice(&f.extra.iter().find(|(p, _)| p == &source).unwrap().1).unwrap();
    value["id"] = "urn:test:private-history-source".into();
    value["provider"] = json!({"id":provider.id,"revision":provider.revision});
    let source = f.artifact(value);
    let history = Pin::decode(&f.b["scopes"]["provider_handler"]["faultInput"]["history"]).unwrap();
    let mut value: Value =
        serde_json::from_slice(&f.extra.iter().find(|(p, _)| p == &history).unwrap().1).unwrap();
    value["source"] = json!({"id":source.id,"revision":source.revision});
    let history = f.artifact(value);
    f.b["scopes"]["provider_handler"]["faultInput"]["history"] =
        json!({"id":history.id,"revision":history.revision});
    refuse(&f, "visibility");
}

#[test]
fn fault_existence_taints_constant_initializers_and_ancestor_access_has_no_inherited_value() {
    let mut f = fixture();
    let mut flag = f.b["data"]["fault_history"].clone();
    flag["type"] = wrap(json!({"kind":"boolean"}));
    flag["protection"]["sensitivity"] = "public".into();
    flag["initial"] = "seen_true".into();
    f.b["data"]["fault_seen"] = flag;
    f.b["expressions"]["seen_true"] = json!({"scope":"provider_handler","dialect":"core","parameters":{},"reads":{},"resultType":wrap(json!({"kind":"boolean"})),"body":boolean(true)});
    proof_refuse(&f, "control-information-flow");
    let mut f = fixture();
    f.b["policies"]["history_access"] = json!({"scope":"provider_handler","dialect":"core","body":{"kind":"protection","reads":{"current_claim":true},"permit":{"op":"eq","left":{"op":"read","cell":"current_claim"},"right":{"op":"read","cell":"current_claim"}}}});
    f.b["data"]["fault_history"]["protection"]["policy"] = "history_access".into();
    proof_refuse(&f, "fault-entry-access-precheck");
}
