// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use choreoform_ir_probe_core::{Resource, digest};
use choreoform_semantic_check::{
    FAULT_SOURCE_PROFILE, FaultEvidence, Pin, ProviderObservationState, Resources,
};
use serde_json::{Value, json};
use std::sync::Arc;

const PROFILE: &[u8] = include_bytes!(
    "../../../docs/ir/contracts/sha256-f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0.txt"
);
const PROVIDER: &[u8] = include_bytes!("../../../docs/benchmarks/rp01/payment.provider.json");
const TYPES: &[u8] = include_bytes!("../../../docs/benchmarks/rp01/types.json");
struct Fixture {
    source: Value,
    artifacts: Vec<(Pin, Vec<u8>)>,
}
impl Fixture {
    fn new() -> Self {
        let mut f = Self {
            source: Value::Null,
            artifacts: Vec::new(),
        };
        let provider = f.bytes(PROVIDER.to_vec());
        f.bytes(TYPES.to_vec());
        f.artifacts.push((
            Pin {
                id: FAULT_SOURCE_PROFILE.0.into(),
                revision: FAULT_SOURCE_PROFILE.1.into(),
            },
            PROFILE.to_vec(),
        ));
        f.source = json!({"kind":"provider-fault-source","version":"0.1.0","id":"urn:test:provider-fault-source","profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":{"id":provider.id,"revision":provider.revision},"emissions":{"confirmed no effect":{"observations":{"no-effect":true},"evidence":"definitive-no-effect"},"uncertain":{"observations":{"unknown":true},"evidence":"unresolved"}}});
        f
    }
    fn artifact(&mut self, v: Value) -> Pin {
        let raw = serde_json::to_vec(&v).unwrap();
        self.bytes(raw)
    }
    fn bytes(&mut self, raw: Vec<u8>) -> Pin {
        let v: Value = serde_json::from_slice(&raw).unwrap();
        let pin = Pin {
            id: v["id"].as_str().unwrap().into(),
            revision: digest(&raw),
        };
        self.artifacts.push((pin.clone(), raw));
        pin
    }
    fn check(
        &mut self,
    ) -> Result<
        Arc<choreoform_semantic_check::ProviderFaultSource>,
        choreoform_semantic_check::Diagnostic,
    > {
        let source = self.artifact(self.source.clone());
        let resources: Vec<_> = self
            .artifacts
            .iter()
            .map(|(p, bytes)| Resource {
                id: &p.id,
                revision: &p.revision,
                bytes,
            })
            .collect();
        Resources::new(&resources)?.provider_fault_source(&source)
    }
}

#[test]
fn exact_metadata_states_mappings_and_cached_views_are_not_retry_or_observation_authority() {
    let mut f = Fixture::new();
    let source = f.artifact(f.source.clone());
    let supplied: Vec<_> = f
        .artifacts
        .iter()
        .map(|(p, bytes)| Resource {
            id: &p.id,
            revision: &p.revision,
            bytes,
        })
        .collect();
    let mut r = Resources::new(&supplied).unwrap();
    let a = r.provider_fault_source(&source).unwrap();
    let b = r.provider_fault_source(&source).unwrap();
    assert!(Arc::ptr_eq(&a, &b));
    assert_eq!(a.pin(), &source);
    assert_eq!(
        (a.profile().id.as_str(), a.profile().revision.as_str()),
        FAULT_SOURCE_PROFILE
    );
    assert_eq!(
        a.emissions()["confirmed no effect"].evidence(),
        FaultEvidence::DefinitiveNoEffect
    );
    assert_eq!(a.emissions()["uncertain"].evidence().as_str(), "unresolved");
    assert!(
        a.emissions()["uncertain"]
            .observations()
            .contains("unknown")
    );
    assert_eq!(
        a.provider_contract().observations()["unknown"],
        ProviderObservationState::Unknown
    );
    assert_eq!(
        a.provider_contract().observations()["result-paid"].as_str(),
        "succeeded"
    );
    assert!(
        a.remaining()
            .contains(&"explicit-supported-definition-retry-and-actual-provider-use-binding")
    );
    assert!(
        a.remaining()
            .contains(&"stable-key-or-definitive-correlated-no-effect-check-before-retry")
    );
}

#[test]
fn exact_state_mapping_vectors_refuse_aliases_ambiguity_and_claimed_safety() {
    let v: Value =
        serde_json::from_slice(include_bytes!("conformance/provider-fault-sources.json")).unwrap();
    assert_eq!(v["profile"]["revision"], FAULT_SOURCE_PROFILE.1);
    for case in v["cases"].as_array().unwrap() {
        let mut f = Fixture::new();
        assert_eq!(v["provider"], f.source["provider"]);
        f.source["emissions"] = case["emissions"].clone();
        for (k, v) in case["extras"].as_object().unwrap() {
            f.source[k] = v.clone();
        }
        let result = match f.check() {
            Ok(_) => json!({"accepted":true}),
            Err(e) => json!({"code":e.code}),
        };
        assert_eq!(result, case["expect"], "{}", case["id"]);
    }
}

#[test]
fn profile_provider_and_named_type_bytes_are_all_explicit_and_exact() {
    for target in ["profile", "provider", "types"] {
        for corrupt in [false, true] {
            let mut f = Fixture::new();
            let index = match target {
                "profile" => 2,
                "provider" => 0,
                _ => 1,
            };
            if corrupt {
                f.artifacts[index].1 = b"{}".to_vec();
            } else {
                f.artifacts.remove(index);
            }
            let e = f.check().unwrap_err();
            assert_eq!(
                e.code,
                if corrupt {
                    "resource-digest"
                } else {
                    "resource-missing"
                }
            );
        }
    }
    let mut f = Fixture::new();
    f.source["profile"]["revision"] = format!("sha256:{}", "0".repeat(64)).into();
    assert_eq!(
        f.check().unwrap_err().code,
        "fault-source-profile-unsupported"
    );
}

#[test]
fn closed_resource_transport_versions_references_and_provider_rules_stay_strict() {
    for mutation in [
        "version",
        "kind",
        "extra",
        "profile_flag",
        "provider_kind",
        "provider_state",
        "provider_result",
    ] {
        let mut f = Fixture::new();
        match mutation {
            "version" => f.source["version"] = "0.2.0".into(),
            "kind" => f.source["kind"] = "safe-fault-source".into(),
            "extra" => f.source["safe"] = true.into(),
            "profile_flag" => f.source["profile"]["trusted"] = true.into(),
            _ => {
                let mut provider: Value = serde_json::from_slice(PROVIDER).unwrap();
                match mutation {
                    "provider_kind" => provider["kind"] = "named-types".into(),
                    "provider_state" => {
                        provider["observations"]["unknown"]["state"] = "timeout means safe".into()
                    }
                    _ => provider["observations"]["unknown"]["outcome"] = "paid".into(),
                }
                let p = f.artifact(provider);
                f.source["provider"] = json!({"id":p.id,"revision":p.revision});
            }
        }
        let e = f.check().unwrap_err();
        assert_eq!(
            e.code,
            match mutation {
                "extra" | "profile_flag" => "fields",
                "provider_state" => "provider-observation",
                "provider_result" => "provider-result",
                _ => "resource-format",
            }
        );
        assert!(!format!("{e:?}").contains("timeout means safe"));
    }
}

#[test]
fn declared_unknown_without_stable_key_is_classification_only_and_cannot_authorize_retry() {
    let mut f = Fixture::new();
    let mut provider: Value = serde_json::from_slice(PROVIDER).unwrap();
    provider["idempotency"] = "none".into();
    let p = f.artifact(provider);
    f.source["provider"] = json!({"id":p.id,"revision":p.revision});
    let report = f.check().unwrap();
    assert_eq!(
        report.emissions()["uncertain"].evidence(),
        FaultEvidence::Unresolved
    );
    assert!(!report.remaining().is_empty());
    // No dispatch/receipt object or definition binding is returned by this API.
}

#[test]
fn exact_unicode_observation_names_and_collection_bounds_are_not_guessed() {
    for observation in ["é", "e\u{301}"] {
        let mut f = Fixture::new();
        let mut provider: Value = serde_json::from_slice(PROVIDER).unwrap();
        let no_effect = provider["observations"]
            .as_object_mut()
            .unwrap()
            .remove("no-effect")
            .unwrap();
        provider["observations"]["é"] = no_effect;
        let p = f.artifact(provider);
        f.source["provider"] = json!({"id":p.id,"revision":p.revision});
        f.source["emissions"] = json!({"named fault":{"observations":{observation:true},"evidence":"definitive-no-effect"}});
        let result = f.check();
        if observation == "é" {
            result.unwrap();
        } else {
            assert_eq!(result.unwrap_err().code, "fault-source-observation");
        }
    }
    let mut f = Fixture::new();
    f.source["emissions"] = (0..10001)
        .map(|i| {
            (
                format!("fault_{i}"),
                json!({"observations":{"no-effect":true},"evidence":"definitive-no-effect"}),
            )
        })
        .collect::<serde_json::Map<_, _>>()
        .into();
    assert!(matches!(
        f.check().unwrap_err().code,
        "fault-source-emissions" | "resource-transport"
    ));
}
