// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Untrusted claims are not fault receipts or handler initialization evidence.
use choreoform_ir_probe_core::{Resource, digest};
use choreoform_semantic_check::{
    FAULT_HISTORY_PROFILE, FAULT_HISTORY_TYPES, FAULT_SOURCE_PROFILE, Nominal, Pin, Resources, Type,
};
use serde_json::{Value, json};
use std::sync::Arc;
const PROFILE: &[u8] = include_bytes!(
    "../../../docs/ir/contracts/sha256-2f580f46eb6ef9b7fff42906f6b0b5cafc6c5fa71ae14365828935bfe2da070c.txt"
);
const TYPES: &[u8] = include_bytes!(
    "../../../docs/ir/contracts/sha256-a2e177ce1dd4bad5b9cd68b74e6da7749ec2b04b68a39dae1bd72729e2aae7a7.txt"
);
struct Fixture {
    artifacts: Vec<(Pin, Vec<u8>)>,
    descriptor: Value,
}
impl Fixture {
    fn real() -> Self {
        let mut f = Self {
            artifacts: Vec::new(),
            descriptor: Value::Null,
        };
        for raw in [
            include_bytes!("../../../docs/benchmarks/rp01/types.json").as_slice(),
            include_bytes!("../../../docs/benchmarks/rp01/payment.provider.json"),
            include_bytes!("../../../docs/benchmarks/rp01/payment.fault-source.json"),
            TYPES,
        ] {
            f.raw(raw);
        }
        for (p, raw) in [
            (FAULT_HISTORY_PROFILE, PROFILE),
            (
                FAULT_SOURCE_PROFILE,
                include_bytes!(
                    "../../../docs/ir/contracts/sha256-f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0.txt"
                ),
            ),
        ] {
            f.artifacts.push((
                Pin {
                    id: p.0.into(),
                    revision: p.1.into(),
                },
                raw.to_vec(),
            ));
        }
        let source = &f.artifacts[2].0;
        f.descriptor = json!({"kind":"provider-fault-history","version":"0.1.0","id":"urn:test:fault-history","profile":{"id":FAULT_HISTORY_PROFILE.0,"revision":FAULT_HISTORY_PROFILE.1},"source":{"id":source.id,"revision":source.revision}});
        f
    }
    fn raw(&mut self, raw: &[u8]) -> Pin {
        let v: Value = serde_json::from_slice(raw).unwrap();
        let pin = Pin {
            id: v["id"].as_str().unwrap().into(),
            revision: digest(raw),
        };
        self.artifacts.push((pin.clone(), raw.to_vec()));
        pin
    }
    fn bind(&mut self) -> Pin {
        self.raw(&serde_json::to_vec(&self.descriptor).unwrap())
    }
    fn supplied(&self) -> Vec<Resource<'_>> {
        self.artifacts
            .iter()
            .map(|(p, bytes)| Resource {
                id: &p.id,
                revision: &p.revision,
                bytes,
            })
            .collect()
    }
    fn refuse(&mut self, code: &str) {
        let pin = self.bind();
        let supplied = self.supplied();
        let e = Resources::new(&supplied)
            .unwrap()
            .provider_fault_history(&pin)
            .unwrap_err();
        assert_eq!(e.code, code, "{e:?}");
        assert!(!format!("{e:?}").contains("SECRET-MARKER"));
    }
}
fn corpus() -> Value {
    serde_json::from_slice(include_bytes!("conformance/provider-fault-history.json")).unwrap()
}
fn claim() -> Value {
    corpus()["base"].clone()
}

#[test]
fn real_source_fixed_nominal_and_lower_bound_protection_are_retained() {
    let mut f = Fixture::real();
    let pin = f.bind();
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    let h = r.provider_fault_history(&pin).unwrap();
    assert_eq!(h.pin(), &pin);
    assert_eq!(h.profile().revision, FAULT_HISTORY_PROFILE.1);
    assert_eq!(
        h.history_type(),
        &Type::Named(Nominal {
            contract: Pin {
                id: FAULT_HISTORY_TYPES.0.into(),
                revision: FAULT_HISTORY_TYPES.1.into()
            },
            name: "Provider fault history".into()
        })
    );
    assert_eq!(h.source_metadata().pin(), &f.artifacts[2].0);
    assert_eq!(
        h.source_protection(),
        h.source_metadata().provider_contract().protection()
    );
    assert!(
        h.remaining().contains(
            &"complete-source-input-fault-existence-control-and-creator-owner-provenance"
        )
    );
    assert_eq!(
        h.source_metadata().emissions()["provider unknown"]
            .evidence()
            .as_str(),
        "unresolved"
    );
    assert!(Arc::ptr_eq(&h, &r.provider_fault_history(&pin).unwrap()));
}
#[test]
fn every_exact_transitive_resource_is_required_and_digest_checked() {
    let mut f = Fixture::real();
    let pin = f.bind();
    for i in 0..f.artifacts.len() {
        let mut missing = Fixture::real();
        let p = missing.bind();
        missing.artifacts.remove(i);
        assert_eq!(
            Resources::new(&missing.supplied())
                .unwrap()
                .provider_fault_history(&p)
                .unwrap_err()
                .code,
            "resource-missing"
        );
        let mut corrupt = f.artifacts.clone();
        corrupt[i].1.push(b' ');
        let supplied: Vec<_> = corrupt
            .iter()
            .map(|(p, bytes)| Resource {
                id: &p.id,
                revision: &p.revision,
                bytes,
            })
            .collect();
        assert_eq!(
            Resources::new(&supplied)
                .unwrap()
                .provider_fault_history(&pin)
                .unwrap_err()
                .code,
            "resource-digest"
        );
    }
}
#[test]
fn closed_descriptor_profile_source_kind_and_identity_refuse() {
    for (field, value, code) in [
        ("approved", json!(true), "fields"),
        ("version", json!("9.0.0"), "resource-format"),
        ("kind", json!("named-types"), "resource-format"),
    ] {
        let mut f = Fixture::real();
        f.descriptor[field] = value;
        f.refuse(code);
    }
    let mut f = Fixture::real();
    f.descriptor["profile"]["id"] = json!("SECRET-MARKER");
    f.refuse("fault-history-profile-unsupported");
    let mut f = Fixture::real();
    f.descriptor["profile"]["revision"] = json!(format!("sha256:{}", "0".repeat(64)));
    f.refuse("fault-history-profile-unsupported");
    let mut f = Fixture::real();
    let p = &f.artifacts[1].0;
    f.descriptor["source"] = json!({"id":p.id,"revision":p.revision});
    f.refuse("resource-format");
    let mut f = Fixture::real();
    f.descriptor["source"]["id"] = json!("SECRET-MARKER");
    f.refuse("resource-missing");
}
#[test]
fn aliases_share_source_and_nominal_without_sharing_any_live_event() {
    let mut f = Fixture::real();
    let a = f.bind();
    f.descriptor["id"] = json!("urn:test:history-alias");
    let b = f.bind();
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    let x = r.provider_fault_history(&a).unwrap();
    let y = r.provider_fault_history(&b).unwrap();
    assert_ne!(x.pin(), y.pin());
    assert!(std::ptr::eq(x.history_type(), y.history_type()));
    assert!(std::ptr::eq(x.source_metadata(), y.source_metadata()));
    r.check_fault_history_claim(&a, &claim()).unwrap();
    r.check_fault_history_claim(&b, &claim()).unwrap();
}
#[test]
fn all_shared_finite_claim_vectors_match_actual_sdk_judgment() {
    let mut f = Fixture::real();
    let pin = f.bind();
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    let v = corpus();
    assert_eq!(v["cases"].as_array().unwrap().len(), 20);
    for case in v["cases"].as_array().unwrap() {
        let mut c = claim();
        for (path, value) in case["patch"].as_object().unwrap() {
            let (parent, key) = path.rsplit_once('/').unwrap();
            c.pointer_mut(parent)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .insert(key.into(), value.clone());
        }
        for path in case["remove"].as_array().unwrap() {
            let (parent, key) = path.as_str().unwrap().rsplit_once('/').unwrap();
            c.pointer_mut(parent)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove(key);
        }
        let actual = r
            .check_fault_history_claim(&pin, &c)
            .map(|()| "passed")
            .unwrap_or_else(|e| e.code);
        assert_eq!(actual, case["expect"].as_str().unwrap(), "{}", case["id"]);
    }
}
#[test]
fn ordinary_nominal_values_are_not_authenticated_source_claims() {
    let mut f = Fixture::real();
    let pin = f.bind();
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    let h = r.provider_fault_history(&pin).unwrap();
    let mut c = claim();
    c["type"] = json!("invented fault");
    r.literal(h.history_type(), &c).unwrap();
    assert_eq!(
        r.check_fault_history_claim(&pin, &c).unwrap_err().code,
        "fault-history-source"
    );
    // Even an invented activity identity can pass the limited shape/mapping
    // judgment. It produces unit, no receipt or initialized handler cell.
    c = claim();
    c["sourceOccurrence"]["site"] = json!("invented site");
    r.check_fault_history_claim(&pin, &c).unwrap();
}
#[test]
fn exact_closed_type_rejects_ordinary_missing_extra_or_wrong_typed_fields() {
    let mut f = Fixture::real();
    let pin = f.bind();
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    for (path, value) in [
        ("/cause", json!("text")),
        ("/inputRevisions", json!({})),
        ("/sourceOccurrence/attempt", json!(true)),
    ] {
        let mut c = claim();
        *c.pointer_mut(path).unwrap() = value;
        assert!(r.check_fault_history_claim(&pin, &c).is_err());
    }
    let mut c = claim();
    c["cause"]["approved"] = json!(true);
    assert_eq!(
        r.check_fault_history_claim(&pin, &c).unwrap_err().code,
        "record-value"
    );
}
