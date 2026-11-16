// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Raw core-0.4 declaration checks; no source migration or actual fault acceptance.
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource, digest};
use choreoform_semantic_check::{
    CLOCK_PROFILE, CORE, CORE_V2, CORE_V3, CORE_V4, FAULT_SOURCE_PROFILE, FaultEvidence, Pin,
    SEMANTICS, check, check_fault_routes, check_timing,
};
use serde_json::{Value, json};

struct Fixture {
    b: Value,
    source: Value,
    extra: Vec<(Pin, Vec<u8>)>,
}
impl Fixture {
    fn new() -> Self {
        let candidate: Value = serde_json::from_slice(include_bytes!(
            "../../../docs/benchmarks/rp01/history/source-0.7/candidate.ir.json"
        ))
        .unwrap();
        let mut f = Self {
            b: candidate["body"].clone(),
            source: Value::Null,
            extra: Vec::new(),
        };
        for raw in [
            include_bytes!("../../../docs/benchmarks/rp01/types.json").as_slice(),
            include_bytes!("../../../docs/benchmarks/rp01/checks.provider.json").as_slice(),
            include_bytes!("../../../docs/benchmarks/rp01/payment.provider.json").as_slice(),
            include_bytes!("../../../docs/benchmarks/rp01/notice.provider.json").as_slice(),
            include_bytes!("../../../docs/benchmarks/rp01/clock.provider.json").as_slice(),
            include_bytes!("../../../docs/benchmarks/rp01/clock.json").as_slice(),
        ] {
            let v: Value = serde_json::from_slice(raw).unwrap();
            f.extra.push((
                Pin {
                    id: v["id"].as_str().unwrap().into(),
                    revision: digest(raw),
                },
                raw.to_vec(),
            ));
        }
        for (pin,raw) in [
            (SEMANTICS,include_bytes!("../../../docs/ir/contracts/sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt").as_slice()),
            (CORE,include_bytes!("../../../docs/ir/contracts/sha256-26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58.txt").as_slice()),
            (CORE_V2,include_bytes!("../../../docs/ir/contracts/sha256-5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623.txt").as_slice()),
            (CORE_V3,include_bytes!("../../../docs/ir/contracts/sha256-30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3.txt").as_slice()),
            (CORE_V4,include_bytes!("../../../docs/ir/contracts/sha256-0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3.txt").as_slice()),
            (CLOCK_PROFILE,include_bytes!("../../../docs/ir/contracts/sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt").as_slice()),
            (FAULT_SOURCE_PROFILE,include_bytes!("../../../docs/ir/contracts/sha256-f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0.txt").as_slice()),
        ] {f.extra.push((Pin{id:pin.0.into(),revision:pin.1.into()},raw.to_vec()));}
        f.b["dialects"]["core"] = json!({"id":CORE_V4.0,"revision":CORE_V4.1});
        for p in f.b["policies"].as_object_mut().unwrap().values_mut() {
            if p["body"]["kind"] == "work" {
                p["body"]["retry"]["faultSource"] = Value::Null;
            }
        }
        f.source = json!({"kind":"provider-fault-source","version":"0.1.0","id":"urn:test:retry-fault-source","profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":f.b["policies"]["payment_capability_provider"]["body"]["provider"],"emissions":{"confirmed no effect":{"observations":{"no-effect":true},"evidence":"definitive-no-effect"},"provider unknown":{"observations":{"unknown":true},"evidence":"unresolved"}}});
        f.bind_source();
        f
    }
    fn artifact(&mut self, v: Value) -> Pin {
        let raw = serde_json::to_vec(&v).unwrap();
        let p = Pin {
            id: v["id"].as_str().unwrap().into(),
            revision: digest(&raw),
        };
        if !self.extra.iter().any(|(pin, _)| pin == &p) {
            self.extra.push((p.clone(), raw));
        }
        p
    }
    fn bind_source(&mut self) -> Pin {
        let p = self.artifact(self.source.clone());
        self.b["policies"]["payment_work"]["body"]["retry"]["faultSource"] =
            json!({"id":p.id,"revision":p.revision});
        p
    }
    fn resources(&self) -> Vec<Resource<'_>> {
        self.extra
            .iter()
            .map(|(p, raw)| Resource {
                id: &p.id,
                revision: &p.revision,
                bytes: raw,
            })
            .collect()
    }
    fn envelope(&self) -> DefinitionEnvelope {
        DefinitionEnvelope::from_parts(self.b.clone(), json!({})).unwrap()
    }
    fn refuse(&self, code: &str) {
        let error = check(&self.envelope(), &self.resources()).unwrap_err();
        assert_eq!(error.code, code, "{error:?}");
        assert!(!format!("{error:?}").contains("SECRET-FAULT-MARKER"));
    }
}

#[test]
fn raw_v4_package_binds_actual_source_provider_and_revision_without_permission() {
    let f = Fixture::new();
    let e = f.envelope();
    let supplied = f.resources();
    let report = check_timing(&e, &supplied).unwrap();
    let retry = report.retry_uses().first().unwrap();
    let source = retry.fault_source().unwrap();
    assert_eq!(source.revision(), report.revision());
    assert_eq!(source.site(), "payment_request_step");
    assert_eq!(source.source().id, "urn:test:retry-fault-source");
    assert_eq!(source.profile().revision, FAULT_SOURCE_PROFILE.1);
    assert_eq!(
        source.provider().id,
        "urn:choreoform:benchmarks:rp01:payment:0.1.0"
    );
    assert_eq!(source.faults().len(), 1);
    assert_eq!(
        source.faults()["confirmed no effect"].evidence(),
        FaultEvidence::DefinitiveNoEffect
    );
    assert!(
        source.faults()["confirmed no effect"]
            .observations()
            .contains("no-effect")
    );
    assert!(!source.host_requirements().is_empty());
    let old: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/benchmarks/rp01/history/source-0.7/candidate.ir.json"
    ))
    .unwrap();
    assert_ne!(old["revision"], e.document()["revision"]);
}

#[test]
fn exact_inherited_source_profile_provider_and_type_artifacts_are_required() {
    for id in [
        CORE.0,
        CORE_V2.0,
        CORE_V3.0,
        CORE_V4.0,
        FAULT_SOURCE_PROFILE.0,
        "urn:test:retry-fault-source",
        "urn:choreoform:benchmarks:rp01:payment:0.1.0",
        "urn:choreoform:benchmarks:rp01:types:0.1.0",
    ] {
        let mut f = Fixture::new();
        f.extra.retain(|(p, _)| p.id != id);
        f.refuse("resource-missing");
    }
    let mut f = Fixture::new();
    f.extra
        .iter_mut()
        .find(|(p, _)| p.id == CORE_V4.0)
        .unwrap()
        .1 = b"wrong contract".to_vec();
    f.refuse("resource-digest");
}

#[test]
fn retry_shape_null_unused_fields_and_unmapped_fault_names_fail_closed() {
    for mutation in [
        "missing",
        "null",
        "faults_empty",
        "fault_unknown",
        "unused_source",
        "unused_faults",
    ] {
        let mut f = Fixture::new();
        let policy = if mutation.starts_with("unused") {
            "manager_review_work"
        } else {
            "payment_work"
        };
        let r = &mut f.b["policies"][policy]["body"]["retry"];
        match mutation {
            "missing" => {
                r.as_object_mut().unwrap().remove("faultSource");
            }
            "null" => r["faultSource"] = Value::Null,
            "faults_empty" => r["faults"] = json!({}),
            "fault_unknown" => r["faults"] = json!({"SECRET-FAULT-MARKER":true}),
            "unused_source" => {
                r["faultSource"] =
                    json!({"id":"urn:test:unused","revision":format!("sha256:{}","0".repeat(64))})
            }
            _ => r["faults"] = json!({"unbound":true}),
        }
        f.refuse(match mutation {
            "missing" => "fields",
            "null" | "faults_empty" => "retry-fault-source-required",
            "fault_unknown" => "retry-fault-source-name",
            _ => "retry-fault-source-unused",
        });
    }
}

#[test]
fn exact_provider_identity_not_just_equal_types_and_labels_is_required_even_unvisited() {
    for unvisited in [false, true] {
        let mut f = Fixture::new();
        let provider: Value = serde_json::from_slice(include_bytes!(
            "../../../docs/benchmarks/rp01/payment.provider.json"
        ))
        .unwrap();
        // Reserialization alone creates different exact bytes despite identical
        // provider identity/type/label declarations. No implicit rebind is allowed.
        let p = f.artifact(provider);
        f.source["provider"] = json!({"id":p.id,"revision":p.revision});
        f.bind_source();
        if unvisited {
            f.b["scopes"]["expense"]["entry"] = "finish_paid".into();
        }
        f.refuse("retry-fault-source-provider");
    }
}

#[test]
fn unresolved_requires_stable_key_but_declared_no_effect_does_not_authorize_a_retry() {
    for unknown in [false, true] {
        let mut f = Fixture::new();
        let mut provider: Value = serde_json::from_slice(include_bytes!(
            "../../../docs/benchmarks/rp01/payment.provider.json"
        ))
        .unwrap();
        provider["idempotency"] = "none".into();
        let p = f.artifact(provider);
        f.source["provider"] = json!({"id":p.id,"revision":p.revision});
        f.bind_source();
        f.b["policies"]["payment_capability_provider"]["body"]["provider"] =
            json!({"id":p.id,"revision":p.revision});
        f.b["policies"]["payment_effects"]["body"]["idempotency"] = "none".into();
        if unknown {
            f.b["policies"]["payment_work"]["body"]["retry"]["faults"] =
                json!({"provider unknown":true});
            f.refuse("retry-fault-evidence-unproved");
        } else {
            let report = check_timing(&f.envelope(), &f.resources()).unwrap();
            assert!(
                !report
                    .retry_uses()
                    .first()
                    .unwrap()
                    .host_requirements()
                    .is_empty()
            );
        }
    }
    let mut f = Fixture::new();
    f.b["policies"]["payment_work"]["body"]["retry"]["faults"] = json!({"provider unknown":true});
    assert_eq!(
        check_timing(&f.envelope(), &f.resources())
            .unwrap()
            .retry_uses()
            .first()
            .unwrap()
            .fault_source()
            .unwrap()
            .faults()["provider unknown"]
            .evidence(),
        FaultEvidence::Unresolved
    );
}

#[test]
fn every_source_emission_including_noneligible_faults_is_counted_without_actual_emission() {
    let f = Fixture::new();
    let report = check_fault_routes(&f.envelope(), &f.resources()).unwrap();
    assert_eq!(report.unhandled_cases(), 2 * report.checked_scopes().len());
    assert!(report.routes().next().is_none());
    assert!(!report.remaining().is_empty());
}

#[test]
fn noneligible_source_faults_keep_exact_nearest_handler_selection() {
    let mut f = Fixture::new();
    let mut handler = f.b["scopes"]["remaining"].clone();
    handler["entry"] = "handle_done".into();
    handler["closure"] = "ordinary_close".into();
    handler["faults"] = "no_handlers".into();
    f.b["scopes"]["handle_provider_fault"] = handler;
    f.b["nodes"]["handle_done"] = json!({"kind":"finish","scope":"handle_provider_fault","reads":{},"writes":{},"outcomes":{},"outcome":"complete"});
    f.b["policies"]["handle_faults"] = json!({"scope":"expense","dialect":"core","body":{"kind":"faults","reads":{},"handlers":{"provider unknown":"handle_provider_fault"}}});
    f.b["scopes"]["expense"]["faults"] = "handle_faults".into();
    let report = check_fault_routes(&f.envelope(), &f.resources()).unwrap();
    assert!(
        report
            .covered_handlers()
            .any(|h| h.fault() == "provider unknown" && h.owner() == "expense")
    );
    assert!(
        !report
            .timing()
            .retry_uses()
            .first()
            .unwrap()
            .fault_source()
            .unwrap()
            .faults()
            .contains_key("provider unknown")
    );
    assert!(
        report
            .routes()
            .any(|r| r.selected().fault() == "provider unknown")
    );
}

#[test]
fn human_contexts_refuse_and_unused_policies_do_not_create_retry_uses() {
    let mut f = Fixture::new();
    let mut retry = f.b["policies"]["payment_work"]["body"]["retry"].clone();
    retry["basis"] = "current_claim".into();
    f.b["policies"]["manager_review_work"]["body"]["retry"] = retry;
    f.refuse("retry-fault-source-context-unproved");
    let mut f = Fixture::new();
    let mut retry = f.b["policies"]["payment_work"]["body"]["retry"].clone();
    retry["basis"] = "payment_due".into();
    f.b["policies"]["unused_work"] = json!({"scope":"expense","dialect":"core","body":{"kind":"work","reads":{"payment_due":true},"instructions":"Unused declaration","authority":"operator_actor","retry":retry,"timeout":null,"completion":{"op":"literal","type":{"kind":"boolean"},"value":true},"compensates":null}});
    let r = check_timing(&f.envelope(), &f.resources()).unwrap();
    assert_eq!(r.retry_uses().len(), 1);
}

#[test]
fn source_provider_metadata_contributes_exact_reads_even_in_unused_policies() {
    let mut f = Fixture::new();
    f.b["policies"]["special_protect"] = json!({"scope":"expense","dialect":"core","body":{"kind":"protection","reads":{"review_needed":true},"permit":{"op":"read","cell":"review_needed"}}});
    let mut provider: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/benchmarks/rp01/payment.provider.json"
    ))
    .unwrap();
    provider["id"] = "urn:test:source-metadata-provider".into();
    provider["resultProtection"]["policy"] = "special_protect".into();
    let p = f.artifact(provider);
    let mut source = f.source.clone();
    source["id"] = "urn:test:unused-source".into();
    source["provider"] = json!({"id":p.id,"revision":p.revision});
    let s = f.artifact(source);
    let mut retry = f.b["policies"]["payment_work"]["body"]["retry"].clone();
    retry["basis"] = "payment_due".into();
    retry["faultSource"] = json!({"id":s.id,"revision":s.revision});
    f.b["policies"]["unused_work"] = json!({"scope":"expense","dialect":"core","body":{"kind":"work","reads":{"payment_due":true},"instructions":"Unused source metadata","authority":"operator_actor","retry":retry,"timeout":null,"completion":{"op":"literal","type":{"kind":"boolean"},"value":true},"compensates":null}});
    let e = check(&f.envelope(), &f.resources()).unwrap_err();
    assert_eq!(e.code, "policy-reads");
    assert_eq!(e.path, "/body/policies/unused_work/body/reads");
    f.b["policies"]["unused_work"]["body"]["reads"]["review_needed"] = true.into();
    check(&f.envelope(), &f.resources()).unwrap();
}

#[test]
fn mixed_finite_profiles_keep_legacy_cycles_refused_and_old_retry_sources_unbound() {
    let mut f = Fixture::new();
    f.b["dialects"]["v3"] = json!({"id":CORE_V3.0,"revision":CORE_V3.1});
    for (id, p) in f.b["policies"].as_object_mut().unwrap() {
        if id != "payment_work" {
            p["dialect"] = "v3".into();
            if p["body"]["kind"] == "work" {
                p["body"]["retry"]
                    .as_object_mut()
                    .unwrap()
                    .remove("faultSource");
            }
        }
    }
    check(&f.envelope(), &f.resources()).unwrap();
    f.b["dialects"]["v2"] = json!({"id":CORE_V2.0,"revision":CORE_V2.1});
    f.b["policies"]["checks_capability_provider"]["dialect"] = "v2".into();
    f.refuse("policy-cycle");
    let mut f = Fixture::new();
    f.b["dialects"]["core"] = json!({"id":CORE_V3.0,"revision":CORE_V3.1});
    for p in f.b["policies"].as_object_mut().unwrap().values_mut() {
        if p["body"]["kind"] == "work" {
            p["body"]["retry"]
                .as_object_mut()
                .unwrap()
                .remove("faultSource");
        }
    }
    assert!(
        check_timing(&f.envelope(), &f.resources())
            .unwrap()
            .retry_uses()
            .first()
            .unwrap()
            .fault_source()
            .is_none()
    );
}
