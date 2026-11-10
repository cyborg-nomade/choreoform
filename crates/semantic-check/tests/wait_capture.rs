// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Declared timerless capture foundation; no host observations or execution.
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource, digest};
use choreoform_semantic_check::{
    CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, Pin, SEMANTICS, TERMINAL_OBSERVATION_PROFILE, check,
    check_control, check_invalidation, check_protection, check_reconciliation_availability,
};
use serde_json::{Value, json};

fn wrap(t: Value) -> Value {
    json!({"dialect":"core","body":t})
}
fn boolean(v: bool) -> Value {
    json!({"op":"literal","type":{"kind":"boolean"},"value":v})
}
fn policy(kind: &str, mut b: Value) -> Value {
    b["kind"] = kind.into();
    b["reads"] = json!({});
    json!({"scope":"root","dialect":"core","body":b})
}
struct Fixture {
    b: Value,
    provider: Value,
    source: Value,
    extra: Vec<(Pin, Vec<u8>)>,
}
impl Fixture {
    fn new() -> Self {
        let corpus: Value =
            serde_json::from_slice(include_bytes!("conformance/wait-capture.json")).unwrap();
        let mut f = Self {
            b: corpus["definition"]["body"].clone(),
            provider: corpus["resources"]["provider"].clone(),
            source: corpus["resources"]["source"].clone(),
            extra: vec![],
        };
        for pin in [
            SEMANTICS,
            CORE,
            CORE_V2,
            CORE_V3,
            CORE_V4,
            CORE_V5,
            TERMINAL_OBSERVATION_PROFILE,
        ] {
            let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
            f.extra.push((
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
        let provider = f.artifact(f.provider.clone());
        let source = f.artifact(f.source.clone());
        assert_eq!(
            f.b["policies"]["provider_policy"]["body"]["provider"],
            json!({"id":provider.id,"revision":provider.revision})
        );
        assert_eq!(
            f.b["nodes"]["capture_wait"]["capture"]["source"],
            json!({"id":source.id,"revision":source.revision})
        );
        assert_eq!(f.envelope().document(), &corpus["definition"]);
        f
    }
    fn artifact(&mut self, v: Value) -> Pin {
        let raw = serde_json::to_vec(&v).unwrap();
        let pin = Pin {
            id: v["id"].as_str().unwrap().into(),
            revision: digest(&raw),
        };
        if !self.extra.iter().any(|(p, _)| p == &pin) {
            self.extra.push((pin.clone(), raw));
        }
        pin
    }
    fn bind_source(&mut self) {
        let p = self.artifact(self.source.clone());
        self.b["nodes"]["capture_wait"]["capture"]["source"] =
            json!({"id":p.id,"revision":p.revision});
    }
    fn envelope(&self) -> DefinitionEnvelope {
        DefinitionEnvelope::from_parts_versioned("0.2.0", self.b.clone(), json!({})).unwrap()
    }
    fn supplied(&self) -> Vec<Resource<'_>> {
        self.extra
            .iter()
            .map(|(p, b)| Resource {
                id: &p.id,
                revision: &p.revision,
                bytes: b,
            })
            .collect()
    }
    fn refuse(&self, code: &str) {
        let e = check(&self.envelope(), &self.supplied()).unwrap_err();
        assert_eq!(e.code, code, "{e:?}");
        assert!(!format!("{e:?}").contains("SECRET-MARKER"));
    }
}

#[test]
fn ordinary_capture_then_branch_passes_all_partial_stages_and_exposes_only_declarations() {
    let f = Fixture::new();
    let e = f.envelope();
    let supplied = f.supplied();
    let report = check(&e, &supplied).unwrap();
    let [capture] = report.capture_bindings() else {
        panic!("one capture")
    };
    assert_eq!(capture.revision(), e.document()["revision"]);
    assert_eq!(capture.site(), "capture_wait");
    assert_eq!(capture.origin(), "origin");
    assert_eq!(capture.result(), "captured");
    assert_eq!(capture.source().id, "urn:test:capture-source");
    assert_eq!(capture.profile().revision, TERMINAL_OBSERVATION_PROFILE.1);
    assert!(!capture.remaining().is_empty());
    check_control(&e, &supplied).unwrap();
    let p = check_protection(&e, &supplied).unwrap();
    assert!(
        p.accesses().iter().any(|a| a.site() == "capture_wait"
            && a.cell() == "captured"
            && a.operation() == "write")
    );
    check_invalidation(&e, &supplied).unwrap();
    let complete = check_reconciliation_availability(&e, &supplied).unwrap();
    let retained = complete
        .links()
        .exceptional()
        .faults()
        .timing()
        .concurrency()
        .invalidation()
        .protection()
        .control()
        .static_report();
    assert_eq!(retained.capture_bindings().len(), 1);
    assert_eq!(
        retained.capture_bindings()[0].revision(),
        e.document()["revision"]
    );
}

#[test]
fn every_core_base_source_profile_and_provider_pin_is_explicit() {
    let f = Fixture::new();
    let e = f.envelope();
    for id in [
        CORE.0,
        CORE_V2.0,
        CORE_V3.0,
        CORE_V4.0,
        CORE_V5.0,
        TERMINAL_OBSERVATION_PROFILE.0,
        "urn:test:capture-source",
        "urn:test:capture-provider",
    ] {
        let supplied: Vec<_> = f.supplied().into_iter().filter(|r| r.id != id).collect();
        assert_eq!(
            check(&e, &supplied).unwrap_err().code,
            "resource-missing",
            "{id}"
        );
    }
    let supplied: Vec<_> = f
        .supplied()
        .into_iter()
        .map(|r| Resource {
            bytes: if r.id == CORE_V5.0 {
                b"corrupt"
            } else {
                r.bytes
            },
            ..r
        })
        .collect();
    assert_eq!(check(&e, &supplied).unwrap_err().code, "resource-digest");
}

#[test]
fn old_wire_old_core_null_and_unknown_fields_keep_distinct_meaning() {
    let f = Fixture::new();
    let old = DefinitionEnvelope::from_parts(f.b.clone(), json!({})).unwrap();
    assert_eq!(check(&old, &f.supplied()).unwrap_err().code, "structure");
    let mut f = Fixture::new();
    f.b["dialects"]["core"] = json!({"id":CORE_V4.0,"revision":CORE_V4.1});
    f.refuse("wait-capture-dialect");
    f.b["nodes"]["capture_wait"]["capture"] = Value::Null;
    f.b["nodes"]["capture_wait"]["writes"] = json!({});
    let r = check(&f.envelope(), &f.supplied()).unwrap();
    assert!(r.capture_bindings().is_empty());
    let mut f = Fixture::new();
    f.b["nodes"]["capture_wait"]["capture"]["trusted"] = true.into();
    f.refuse("structure");
    let mut f = Fixture::new();
    for n in f.b["nodes"].as_object_mut().unwrap().values_mut() {
        if n["kind"] == "wait" {
            n.as_object_mut().unwrap().remove("capture");
            n["writes"] = json!({});
        }
    }
    let old = DefinitionEnvelope::from_parts(f.b.clone(), json!({})).unwrap();
    assert_eq!(
        check(&old, &f.supplied()).unwrap_err().code,
        "capture-wire-version"
    );
}

#[test]
fn provider_identity_origin_kind_and_unvisited_capture_are_not_inferred() {
    let mut same = Fixture::new();
    let raw = serde_json::to_vec_pretty(&same.provider).unwrap();
    let equivalent = Pin {
        id: same.provider["id"].as_str().unwrap().into(),
        revision: digest(&raw),
    };
    same.extra.push((equivalent.clone(), raw));
    same.source["provider"] = json!({"id":equivalent.id,"revision":equivalent.revision});
    same.bind_source();
    same.refuse("wait-capture-provider");
    let mut f = Fixture::new();
    f.provider["id"] = "urn:test:other-provider".into();
    let p = f.artifact(f.provider.clone());
    f.source["provider"] = json!({"id":p.id,"revision":p.revision});
    f.bind_source();
    f.refuse("wait-capture-provider");
    f.b["scopes"]["root"]["entry"] = "done".into();
    f.refuse("wait-capture-provider");
    let mut f = Fixture::new();
    f.b["nodes"]["capture_wait"]["capture"]["origin"] = "done".into();
    f.refuse("wait-capture-origin-unproved");
    let mut f = Fixture::new();
    f.b["nodes"]["origin"]["mode"] = "human".into();
    f.b["nodes"]["origin"]["target"] = "operator".into();
    f.refuse("wait-capture-origin-unproved");
    let mut f = Fixture::new();
    f.b["scopes"]["root"]["entry"] = "done".into();
    let r = check(&f.envelope(), &f.supplied()).unwrap();
    assert_eq!(r.capture_bindings().len(), 1);
    assert!(
        !check_control(&f.envelope(), &f.supplied())
            .unwrap()
            .covered_nodes()
            .contains("capture_wait")
    );
}

#[test]
fn exact_type_write_protection_and_invalidation_checks_remain_required() {
    let mut f = Fixture::new();
    f.b["data"]["captured"]["type"] =
        wrap(json!({"kind":"variant","cases":{"confirmed":{"kind":"text"}}}));
    f.refuse("wait-capture-type");
    let mut f = Fixture::new();
    f.b["nodes"]["capture_wait"]["writes"] = json!({});
    f.refuse("wait-capture-writes");
    let mut f = Fixture::new();
    f.b["nodes"]["capture_wait"]["writes"]["origin_value"] = true.into();
    f.refuse("wait-capture-writes");
    let mut f = Fixture::new();
    f.b["data"]["captured"]["protection"]["sensitivity"] = "public".into();
    f.refuse("information-flow");
    f.b["scopes"]["root"]["entry"] = "done".into();
    f.refuse("information-flow");
    let mut f = Fixture::new();
    f.b["data"]["captured"]["invalidates"] = json!({});
    let e = check_invalidation(&f.envelope(), &f.supplied()).unwrap_err();
    assert_eq!(e.code, "invalidation-coverage");
    let mut f = Fixture::new();
    f.b["scopes"]["root"]["entry"] = "read_capture".into();
    assert_eq!(
        check_control(&f.envelope(), &f.supplied())
            .unwrap_err()
            .code,
        "data-availability-unproved"
    );
}

#[test]
fn inherited_control_provenance_cannot_launder_a_captured_value_through_a_literal() {
    let mut f = Fixture::new();
    f.provider["resultProtection"]["sensitivity"] = "secret".into();
    let pin = f.artifact(f.provider.clone());
    f.b["policies"]["provider_policy"]["body"]["provider"] =
        json!({"id":pin.id,"revision":pin.revision});
    f.source["provider"] = json!({"id":pin.id,"revision":pin.revision});
    f.bind_source();
    for id in ["origin_value", "captured"] {
        f.b["data"][id]["protection"]["sensitivity"] = "secret".into();
    }
    f.b["scopes"]["root"]["entry"] = "capture_wait".into();
    let mut public = f.b["data"]["origin_value"].clone();
    public["protection"]["sensitivity"] = "public".into();
    public["invalidates"] = json!({});
    f.b["data"]["public_value"] = public;
    f.b["expressions"]["literal_one"] = json!({"scope":"root","dialect":"core","resultType":wrap(json!({"kind":"integer"})),"parameters":{},"reads":{},"body":{"op":"literal","type":{"kind":"integer"},"value":"1"}});
    f.b["nodes"]["publish_public"] = json!({"kind":"compute","scope":"root","reads":{},"writes":{"public_value":true},"outcomes":{"next":true},"assignments":{"public_value":"literal_one"}});
    for id in ["read_yes", "read_no"] {
        f.b["flows"][id]["target"] = "publish_public".into();
    }
    f.b["flows"]["publish_next"] =
        json!({"source":"publish_public","outcome":"next","target":"done"});
    check(&f.envelope(), &f.supplied()).unwrap();
    check_control(&f.envelope(), &f.supplied()).unwrap();
    let e = check_protection(&f.envelope(), &f.supplied()).unwrap_err();
    assert_eq!(e.code, "control-information-flow");
    assert_eq!(e.path, "/body/nodes/publish_public");
    // Static origin linkage is not a claim that this unvisited origin occurrence exists.
    assert!(
        !check_control(&f.envelope(), &f.supplied())
            .unwrap()
            .covered_nodes()
            .contains("origin")
    );
}

#[test]
fn timer_branch_cannot_pretend_to_initialize_an_observation_capture() {
    let mut f = Fixture::new();
    let mut clock_provider = f.provider.clone();
    clock_provider["id"] = "urn:test:capture-clock-provider".into();
    clock_provider["outcomes"] = json!({"tick":true});
    clock_provider["observations"] =
        json!({"tick":{"state":"succeeded","result":{"kind":"integer"},"outcome":"tick"}});
    let p = f.artifact(clock_provider);
    let clock = json!({"kind":"clock","version":"0.1.0","id":"urn:test:capture-clock","profile":{"id":choreoform_semantic_check::CLOCK_PROFILE.0,"revision":choreoform_semantic_check::CLOCK_PROFILE.1},"source":{"id":p.id,"revision":p.revision}});
    let clock = f.artifact(clock);
    f.extra.push((Pin{id:choreoform_semantic_check::CLOCK_PROFILE.0.into(),revision:choreoform_semantic_check::CLOCK_PROFILE.1.into()},include_bytes!("../../../docs/ir/contracts/sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt").to_vec()));
    let wait = &mut f.b["policies"]["ordinary_wait"]["body"];
    wait["timer"] = json!({"clock":{"id":clock.id,"revision":clock.revision},"basis":"origin_value","due":{"op":"literal","type":{"kind":"integer"},"value":"0"},"calendar":null,"pauseAuthority":"operator"});
    wait["onTimer"] = "expired".into();
    wait["reads"] = json!({"origin_value":true});
    f.b["nodes"]["capture_wait"]["reads"] = json!({"origin_value":true});
    f.b["nodes"]["capture_wait"]["outcomes"]["expired"] = true.into();
    f.b["flows"]["capture_timeout"] =
        json!({"source":"capture_wait","outcome":"expired","target":"done"});
    f.refuse("wait-capture-timer-unproved");
}

#[test]
fn private_result_or_origin_is_not_made_visible_by_a_capture() {
    let mut f = Fixture::new();
    f.b["nodes"]["capture_wait"]["capture"]["origin"] = "recover_done".into();
    f.refuse("visibility");
    let mut f = Fixture::new();
    let mut d = f.b["data"]["captured"].clone();
    d["scope"] = "recovery".into();
    d["invalidates"] = json!({});
    f.b["data"]["private_fact"] = d;
    f.b["nodes"]["capture_wait"]["capture"]["result"] = "private_fact".into();
    f.b["nodes"]["capture_wait"]["writes"] = json!({"private_fact":true});
    f.refuse("visibility");
}

fn wide_captures(count: usize, width: usize) -> Fixture {
    let mut f = Fixture::new();
    let fields: serde_json::Map<String, Value> = (0..width)
        .map(|i| (format!("f{i}"), json!({"kind":"text"})))
        .collect();
    let output = json!({"kind":"record","fields":fields});
    let observed = json!({"kind":"variant","cases":{"confirmed":output}});
    f.provider["output"] = output.clone();
    f.provider["observations"]["done"]["result"] = output.clone();
    let p = f.artifact(f.provider.clone());
    f.b["policies"]["provider_policy"]["body"]["provider"] =
        json!({"id":p.id,"revision":p.revision});
    f.source["provider"] = json!({"id":p.id,"revision":p.revision});
    f.bind_source();
    f.b["capabilities"]["adapter"]["output"] = wrap(output.clone());
    f.b["data"]["origin_value"]["type"] = wrap(output);
    f.b["data"]["captured"]["type"] = wrap(observed.clone());
    for id in ["ordinary_wait", "recovery_wait"] {
        f.b["policies"][id]["body"]["observationType"] = observed.clone();
    }
    for i in 0..count {
        let id = format!("unused_capture_{i}");
        f.b["nodes"][&id] = f.b["nodes"]["capture_wait"].clone();
        f.b["flows"][format!("unused_capture_flow_{i}")] =
            json!({"source":id,"outcome":"observed","target":"done"});
    }
    f
}

#[test]
fn repeated_wide_capture_types_reuse_only_immutable_checked_type_relations() {
    let f = wide_captures(420, 2500);
    let e = check(&f.envelope(), &f.supplied()).unwrap();
    assert_eq!(e.capture_bindings().len(), 421);
    let first = e.capture_bindings().first().unwrap();
    assert!(
        e.capture_bindings()
            .iter()
            .all(|r| std::ptr::eq(first.source_metadata(), r.source_metadata()))
    );
}

#[test]
fn cached_type_relations_do_not_hide_a_different_policy_or_destination_type() {
    for wrong_policy in [false, true] {
        let mut f = Fixture::new();
        let mut alias = f.b["nodes"]["capture_wait"].clone();
        let wrong = json!({"kind":"variant","cases":{"confirmed":{"kind":"text"}}});
        if wrong_policy {
            let mut p = f.b["policies"]["ordinary_wait"].clone();
            p["body"]["observationType"] = wrong;
            f.b["policies"]["wrong_wait"] = p;
            alias["policy"] = "wrong_wait".into();
        } else {
            let mut d = f.b["data"]["captured"].clone();
            d["type"] = wrap(wrong);
            d["invalidates"] = json!({});
            f.b["data"]["wrong_captured"] = d;
            alias["capture"]["result"] = "wrong_captured".into();
            alias["writes"] = json!({"wrong_captured":true});
        }
        f.b["nodes"]["z_bad_capture"] = alias;
        f.b["flows"]["bad_capture_next"] =
            json!({"source":"z_bad_capture","outcome":"observed","target":"done"});
        f.refuse("wait-capture-type");
    }
}

fn actual_rp01_capture() -> Fixture {
    let mut f = Fixture::new();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let manifest: Value = serde_json::from_slice(
        &std::fs::read(root.join("docs/benchmarks/rp01/manifest.json")).unwrap(),
    )
    .unwrap();
    let current: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/benchmarks/rp01/candidate.ir.json"
    ))
    .unwrap();
    f.b = current["body"].clone();
    f.b["dialects"]["core"] = json!({"id":CORE_V5.0,"revision":CORE_V5.1});
    for n in f.b["nodes"].as_object_mut().unwrap().values_mut() {
        if n["kind"] == "wait" {
            n["capture"] = Value::Null;
        }
    }
    for r in manifest["resources"].as_array().unwrap() {
        let pin = Pin {
            id: r["id"].as_str().unwrap().into(),
            revision: r["revision"].as_str().unwrap().into(),
        };
        if !f.extra.iter().any(|(p, _)| p == &pin) {
            f.extra.push((
                pin,
                std::fs::read(root.join(r["path"].as_str().unwrap())).unwrap(),
            ));
        }
    }
    f.source["id"] = "urn:test:rp01-capture-source".into();
    f.source["provider"] =
        f.b["policies"]["payment_capability_provider"]["body"]["provider"].clone();
    let source = f.artifact(f.source.clone());
    let observed = f.b["policies"]["payment_wait"]["body"]["observationType"].clone();
    f.b["data"]["late_payment_fact"] = json!({"scope":"payment_recovery","type":wrap(observed),"protection":f.b["data"]["payment_result"]["protection"],"invalidates":{}});
    f.b["nodes"]["payment_observe"]["capture"] = json!({"source":{"id":source.id,"revision":source.revision},"origin":"payment_request_step","result":"late_payment_fact"});
    f.b["nodes"]["payment_observe"]["writes"] = json!({"late_payment_fact":true});
    f
}

#[test]
fn promoted_rp01_binding_gets_separate_conditional_local_proof_without_runtime_ownership() {
    let mut f = actual_rp01_capture();
    let e = f.envelope();
    let supplied = f.supplied();
    let report = check(&e, &supplied).unwrap();
    assert_eq!(report.capture_bindings().len(), 1);
    let row = &report.capture_bindings()[0];
    assert_eq!(row.site(), "payment_observe");
    assert_eq!(row.origin(), "payment_request_step");
    assert_eq!(row.result(), "late_payment_fact");
    assert_eq!(
        row.provider().id,
        "urn:choreoform:benchmarks:rp01:payment:0.1.0"
    );
    check_invalidation(&e, &supplied).unwrap();
    let concurrency = choreoform_semantic_check::check_concurrency(&e, &supplied).unwrap();
    let owned = concurrency.owned_local().unwrap();
    assert_eq!(owned.revision(), e.document()["revision"]);
    assert!(owned.checked_nodes().contains("payment_observe"));
    assert!(
        owned
            .followups()
            .iter()
            .any(|p| p.creator() == "run_payment" && p.local_cells().contains("late_payment_fact"))
    );
    check_reconciliation_availability(&e, &supplied).unwrap();
    // The adjacent retained-ancestor case remains outside this fresh-local proof.
    f.b["data"]["late_payment_fact"]["scope"] = "payment_service".into();
    check(&f.envelope(), &f.supplied()).unwrap();
    let error =
        choreoform_semantic_check::check_concurrency(&f.envelope(), &f.supplied()).unwrap_err();
    assert_eq!(error.code, "scope-lifetime-unproved");
    assert_eq!(error.path, "/body/nodes/run_payment");
}

#[test]
fn capture_does_not_equate_a_reserialized_nominal_artifact_with_its_original_revision() {
    let mut f = actual_rp01_capture();
    let types: Value =
        serde_json::from_slice(include_bytes!("../../../docs/benchmarks/rp01/types.json")).unwrap();
    let pin = f.artifact(types);
    fn rebind(v: &mut Value, p: &Pin) {
        match v {
            Value::Object(m) => {
                if m.get("kind") == Some(&json!("named")) {
                    m.insert("contract".into(), json!({"id":p.id,"revision":p.revision}));
                }
                for v in m.values_mut() {
                    rebind(v, p);
                }
            }
            Value::Array(a) => {
                for v in a {
                    rebind(v, p)
                }
            }
            _ => {}
        }
    }
    rebind(&mut f.b["data"]["late_payment_fact"]["type"], &pin);
    f.refuse("wait-capture-type");
}

#[test]
fn source_protection_reads_remain_complete_even_in_unselected_predicate_branches() {
    let mut f = Fixture::new();
    f.b["policies"]["source_permission"] = policy(
        "protection",
        json!({"permit":{"op":"if","condition":boolean(false),"then":{"op":"read","cell":"permission"},"else":boolean(true)}}),
    );
    f.b["policies"]["source_permission"]["body"]["reads"] = json!({"permission":true});
    let mut permission = f.b["data"]["origin_value"].clone();
    permission["type"] = wrap(json!({"kind":"boolean"}));
    permission["invalidates"] = json!({});
    f.b["data"]["permission"] = permission;
    f.provider["resultProtection"]["policy"] = "source_permission".into();
    let p = f.artifact(f.provider.clone());
    f.b["policies"]["provider_policy"]["body"]["provider"] =
        json!({"id":p.id,"revision":p.revision});
    f.source["provider"] = json!({"id":p.id,"revision":p.revision});
    f.bind_source();
    for d in f.b["data"].as_object_mut().unwrap().values_mut() {
        d["protection"]["policy"] = "source_permission".into();
    }
    f.b["policies"]["provider_policy"]["body"]["reads"] = json!({"permission":true});
    for id in ["origin", "capture_wait", "read_capture"] {
        f.b["nodes"][id]["reads"]["permission"] = true.into();
    }
    check(&f.envelope(), &f.supplied()).unwrap();
    f.b["nodes"]["capture_wait"]["reads"] = json!({});
    f.refuse("node-reads");
    // This is a static dependency assertion; no initial/current permission value is supplied.
}

#[test]
fn shared_finite_capture_vectors_match_the_actual_native_judgments() {
    let corpus: Value =
        serde_json::from_slice(include_bytes!("conformance/wait-capture.json")).unwrap();
    assert_eq!(corpus["cases"].as_array().unwrap().len(), 17);
    for case in corpus["cases"].as_array().unwrap() {
        let mut f = Fixture::new();
        for change in case["changes"].as_array().unwrap() {
            let path = change["path"].as_str().unwrap();
            if let Some(v) = f.b.pointer_mut(path) {
                *v = change["value"].clone();
            } else {
                let (parent, key) = path.rsplit_once('/').unwrap();
                f.b.pointer_mut(parent)
                    .unwrap()
                    .as_object_mut()
                    .unwrap()
                    .insert(key.into(), change["value"].clone());
            }
        }
        let e = DefinitionEnvelope::from_parts_versioned(
            case["wire"].as_str().unwrap(),
            f.b.clone(),
            json!({}),
        )
        .unwrap();
        let supplied = f.supplied();
        let result = match case["stage"].as_str().unwrap() {
            "static" => check(&e, &supplied).map(|_| ()),
            "control" => check_control(&e, &supplied).map(|_| ()),
            "invalidation" => check_invalidation(&e, &supplied).map(|_| ()),
            _ => panic!("closed corpus stage"),
        };
        if let Some(code) = case["expect"]["code"].as_str() {
            assert_eq!(result.unwrap_err().code, code, "{}", case["id"]);
        } else {
            result.unwrap_or_else(|e| panic!("{}: {e:?}", case["id"]));
        }
    }
}

#[test]
fn core5_inherits_retry_binding_and_finite_metadata_without_weakening_legacy_cycles() {
    let mut f = actual_rp01_capture();
    f.b["dialects"]["v4"] = json!({"id":CORE_V4.0,"revision":CORE_V4.1});
    f.b["policies"]["checks_capability_provider"]["dialect"] = "v4".into();
    check(&f.envelope(), &f.supplied()).unwrap();
    f.b["dialects"]["v2"] = json!({"id":CORE_V2.0,"revision":CORE_V2.1});
    f.b["policies"]["checks_capability_provider"]["dialect"] = "v2".into();
    f.refuse("policy-cycle");
    let mut f = actual_rp01_capture();
    f.b["data"]
        .as_object_mut()
        .unwrap()
        .remove("late_payment_fact");
    f.b["nodes"]["payment_observe"]["capture"] = Value::Null;
    f.b["nodes"]["payment_observe"]["writes"] = json!({});
    let e = f.envelope();
    let supplied = f.supplied();
    let timing = choreoform_semantic_check::check_timing(&e, &supplied).unwrap();
    assert_eq!(timing.retry_uses().len(), 1);
    assert!(
        timing
            .retry_uses()
            .first()
            .unwrap()
            .fault_source()
            .is_some()
    );
    let faults = choreoform_semantic_check::check_fault_routes(&e, &supplied).unwrap();
    assert_eq!(faults.unhandled_cases(), 10);
    check_reconciliation_availability(&e, &supplied).unwrap();
    assert!(check(&e, &supplied).unwrap().capture_bindings().is_empty());
}

#[path = "capture/owned.rs"]
mod owned;

#[path = "capture/general_fault.rs"]
mod general_fault;

#[path = "capture/origins.rs"]
mod origins;

#[path = "capture/fault_entry.rs"]
mod fault_entry;

#[path = "capture/retained_request.rs"]
mod retained_request;

#[path = "capture/continuation_audit.rs"]
mod continuation_audit;

#[path = "capture/readonly_continuation.rs"]
mod readonly_continuation;

#[path = "capture/owned_entry_audit.rs"]
mod owned_entry_audit;
