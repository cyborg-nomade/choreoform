// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Exact declared normalization metadata, never actual provider observations.
use choreoform_ir_probe_core::{Resource, digest};
use choreoform_semantic_check::{
    Pin, ProviderObservationState, Resources, TERMINAL_OBSERVATION_PROFILE, TerminalFactTag, Type,
};
use serde_json::{Value, json};
use std::sync::Arc;

const PROFILE: &[u8] = include_bytes!(
    "../../../docs/ir/contracts/sha256-a224fc2b8318af2ff8dfc9955b55af03f4d5b7bd0de719de0384421698d20ead.txt"
);
struct Fixture {
    artifacts: Vec<(Pin, Vec<u8>)>,
    source: Value,
    provider: Value,
}
impl Fixture {
    fn real() -> Self {
        let provider: Value = serde_json::from_slice(include_bytes!(
            "../../../docs/benchmarks/rp01/payment.provider.json"
        ))
        .unwrap();
        let mut f = Self {
            artifacts: vec![],
            source: Value::Null,
            provider,
        };
        f.raw(include_bytes!("../../../docs/benchmarks/rp01/types.json"));
        f.raw(include_bytes!(
            "../../../docs/benchmarks/rp01/payment.provider.json"
        ));
        f.artifacts.push((
            Pin {
                id: TERMINAL_OBSERVATION_PROFILE.0.into(),
                revision: TERMINAL_OBSERVATION_PROFILE.1.into(),
            },
            PROFILE.to_vec(),
        ));
        let p = &f.artifacts[1].0;
        f.source = json!({"kind":"provider-terminal-observation-source","version":"0.1.0","id":"urn:test:terminal-source","profile":{"id":TERMINAL_OBSERVATION_PROFILE.0,"revision":TERMINAL_OBSERVATION_PROFILE.1},"provider":{"id":p.id,"revision":p.revision}});
        f
    }
    fn raw(&mut self, raw: &[u8]) -> Pin {
        let v: Value = serde_json::from_slice(raw).unwrap();
        let p = Pin {
            id: v["id"].as_str().unwrap().into(),
            revision: digest(raw),
        };
        if !self.artifacts.iter().any(|(old, _)| old == &p) {
            self.artifacts.push((p.clone(), raw.to_vec()));
        }
        p
    }
    fn artifact(&mut self, v: &Value) -> Pin {
        self.raw(&serde_json::to_vec(v).unwrap())
    }
    fn bind(&mut self) -> Pin {
        self.artifact(&self.source.clone())
    }
    fn replace_provider(&mut self) {
        let p = self.artifact(&self.provider.clone());
        self.source["provider"] = json!({"id":p.id,"revision":p.revision});
    }
    fn supplied(&self) -> Vec<Resource<'_>> {
        self.artifacts
            .iter()
            .map(|(p, raw)| Resource {
                id: &p.id,
                revision: &p.revision,
                bytes: raw,
            })
            .collect()
    }
    fn refuse(&mut self, code: &str) {
        let pin = self.bind();
        let supplied = self.supplied();
        let e = Resources::new(&supplied)
            .unwrap()
            .provider_terminal_source(&pin)
            .unwrap_err();
        assert_eq!(e.code, code, "{e:?}");
        assert!(!format!("{e:?}").contains("SECRET-MARKER"));
    }
}
fn minimal() -> Value {
    json!({"kind":"provider-contract","version":"0.1.0","id":"urn:test:minimal-terminal-provider","input":{"kind":"record","fields":{}},"output":{"kind":"integer"},"outcomes":{"done":true},"resultProtection":{"policy":"protect","sensitivity":"internal","purposes":{"test":true},"participants":{},"capabilities":{}},"observations":{"done":{"state":"succeeded","result":{"kind":"integer"},"outcome":"done"}},"idempotency":"none","effectClass":"read-only"})
}
fn correlation() -> Type {
    Type::Record(
        ["logical effect", "request digest", "observation"]
            .map(|k| (k.into(), Type::Text))
            .into(),
    )
}

#[test]
fn real_provider_nominal_type_and_protection_are_derived_and_cached_exactly() {
    let mut f = Fixture::real();
    let pin = f.bind();
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    let s = r.provider_terminal_source(&pin).unwrap();
    assert_eq!(s.pin(), &pin);
    assert_eq!(s.profile().revision, TERMINAL_OBSERVATION_PROFILE.1);
    assert_eq!(
        s.provider().revision,
        digest(include_bytes!(
            "../../../docs/benchmarks/rp01/payment.provider.json"
        ))
    );
    let Type::Variant(cases) = s.observation_type() else {
        panic!("variant required")
    };
    assert_eq!(cases.len(), 4);
    assert_eq!(&cases["confirmed"], s.provider_contract().output());
    for name in ["noeffect", "rejected", "cancelled"] {
        assert_eq!(cases[name], correlation());
    }
    let Type::Variant(results) = &cases["confirmed"] else {
        panic!("provider output retained")
    };
    let Type::Named(paid) = &results["paid"] else {
        panic!("nominal retained")
    };
    assert_eq!(paid.name, "Paid evidence");
    assert_eq!(
        paid.contract.revision,
        digest(include_bytes!("../../../docs/benchmarks/rp01/types.json"))
    );
    assert_eq!(s.source_protection(), &f.provider["resultProtection"]);
    assert_eq!(
        s.terminal_observations()["result-paid"].tag(),
        TerminalFactTag::Confirmed
    );
    assert!(!s.terminal_observations().contains_key("unknown"));
    assert!(!s.remaining().is_empty());
    assert!(Arc::ptr_eq(&s, &r.provider_terminal_source(&pin).unwrap()));
}

#[test]
fn every_exact_source_profile_provider_and_nominal_artifact_is_required() {
    let mut f = Fixture::real();
    let pin = f.bind();
    for id in [
        &pin.id,
        TERMINAL_OBSERVATION_PROFILE.0,
        "urn:choreoform:benchmarks:rp01:payment:0.1.0",
        "urn:choreoform:benchmarks:rp01:types:0.1.0",
    ] {
        let supplied: Vec<_> = f.supplied().into_iter().filter(|r| r.id != id).collect();
        assert_eq!(
            Resources::new(&supplied)
                .unwrap()
                .provider_terminal_source(&pin)
                .unwrap_err()
                .code,
            "resource-missing"
        );
    }
    for id in [
        &pin.id,
        TERMINAL_OBSERVATION_PROFILE.0,
        "urn:choreoform:benchmarks:rp01:payment:0.1.0",
        "urn:choreoform:benchmarks:rp01:types:0.1.0",
    ] {
        let supplied: Vec<_> = f
            .supplied()
            .into_iter()
            .map(|r| Resource {
                bytes: if r.id == id { b"corrupt" } else { r.bytes },
                ..r
            })
            .collect();
        assert_eq!(
            Resources::new(&supplied)
                .unwrap()
                .provider_terminal_source(&pin)
                .unwrap_err()
                .code,
            "resource-digest"
        );
    }
}

#[test]
fn closed_resource_does_not_accept_type_protection_trust_or_state_overrides() {
    for key in [
        "observationType",
        "resultProtection",
        "states",
        "trusted",
        "normalizer",
        "safe",
    ] {
        let mut f = Fixture::real();
        f.source[key] = "SECRET-MARKER".into();
        f.refuse("fields");
    }
    for key in ["profile", "provider"] {
        let mut f = Fixture::real();
        f.source.as_object_mut().unwrap().remove(key);
        f.refuse("fields");
    }
    for (key, value, code) in [
        ("version", "0.2.0", "resource-format"),
        ("kind", "provider-fault-source", "resource-format"),
    ] {
        let mut f = Fixture::real();
        f.source[key] = value.into();
        f.refuse(code);
    }
    let mut f = Fixture::real();
    f.source["profile"]["revision"] = format!("sha256:{}", "0".repeat(64)).into();
    f.refuse("terminal-source-profile-unsupported");
    let mut f = Fixture::real();
    f.provider["observations"]["unknown"]["result"] = json!({"kind":"text"});
    f.replace_provider();
    f.refuse("provider-result");
}

#[test]
fn only_present_declared_terminal_states_define_tags_regardless_of_names() {
    let mut f = Fixture::real();
    f.provider = minimal();
    f.provider["observations"]["paid"] = json!({"state":"unknown","result":null,"outcome":null});
    f.provider["observations"]["settled"] =
        json!({"state":"acknowledged","result":null,"outcome":null});
    f.provider["observations"]["unknown"] =
        json!({"state":"succeeded","result":{"kind":"integer"},"outcome":"done"});
    f.provider["observations"]["é"] = json!({"state":"no-effect","result":null,"outcome":null});
    f.provider["observations"]["e\u{301}"] =
        json!({"state":"cancelled","result":null,"outcome":null});
    f.replace_provider();
    let pin = f.bind();
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    let s = r.provider_terminal_source(&pin).unwrap();
    assert_eq!(s.terminal_observations().len(), 4);
    assert!(!s.terminal_observations().contains_key("paid"));
    assert!(!s.terminal_observations().contains_key("settled"));
    assert_eq!(
        s.terminal_observations()["unknown"].state(),
        ProviderObservationState::Succeeded
    );
    assert_eq!(
        s.terminal_observations()["é"].tag(),
        TerminalFactTag::NoEffect
    );
    assert_eq!(
        s.terminal_observations()["e\u{301}"].tag(),
        TerminalFactTag::Cancelled
    );
    let Type::Variant(cases) = s.observation_type() else {
        panic!("variant required")
    };
    assert_eq!(cases.len(), 3);
    assert_eq!(cases["confirmed"], Type::Integer);
    assert!(!cases.contains_key("rejected"));
    // Minimal success-only provider gains no imaginary terminal cases.
    let mut f = Fixture::real();
    f.provider = minimal();
    f.replace_provider();
    let pin = f.bind();
    let supplied = f.supplied();
    let s = Resources::new(&supplied)
        .unwrap()
        .provider_terminal_source(&pin)
        .unwrap();
    assert_eq!(
        s.observation_type(),
        &Type::Variant([("confirmed".into(), Type::Integer)].into())
    );
}

#[test]
fn nominal_revisions_remain_distinct_even_with_equal_representations() {
    let mut f = Fixture::real();
    let old_pin = f.bind();
    let old_supply = f.supplied();
    let old = Resources::new(&old_supply)
        .unwrap()
        .provider_terminal_source(&old_pin)
        .unwrap();
    let types: Value =
        serde_json::from_slice(include_bytes!("../../../docs/benchmarks/rp01/types.json")).unwrap();
    let new_types = f.artifact(&types);
    fn rebind(v: &mut Value, pin: &Pin) {
        match v {
            Value::Object(m) => {
                if m.get("kind") == Some(&json!("named")) {
                    m.insert(
                        "contract".into(),
                        json!({"id":pin.id,"revision":pin.revision}),
                    );
                }
                for x in m.values_mut() {
                    rebind(x, pin);
                }
            }
            Value::Array(a) => {
                for x in a {
                    rebind(x, pin)
                }
            }
            _ => {}
        }
    }
    rebind(&mut f.provider, &new_types);
    f.replace_provider();
    let pin = f.bind();
    let supplied = f.supplied();
    let new = Resources::new(&supplied)
        .unwrap()
        .provider_terminal_source(&pin)
        .unwrap();
    assert_ne!(
        new_types.revision,
        digest(include_bytes!("../../../docs/benchmarks/rp01/types.json"))
    );
    assert_ne!(old.observation_type(), new.observation_type());
    assert_ne!(old.provider(), new.provider());
    assert_eq!(old.source_protection(), new.source_protection());
}

#[test]
fn many_sources_share_exact_provider_schema_without_type_copy_amplification() {
    let mut f = Fixture::real();
    f.provider = minimal();
    let fields: serde_json::Map<String, Value> = (0..2500)
        .map(|i| (format!("f{i}"), json!({"kind":"text"})))
        .collect();
    let wide = json!({"kind":"record","fields":fields});
    f.provider["output"] = wide.clone();
    f.provider["observations"]["done"]["result"] = wide;
    f.replace_provider();
    let mut pins = Vec::new();
    for i in 0..225 {
        f.source["id"] = format!("urn:test:terminal-copy:{i}").into();
        pins.push(f.bind());
    }
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    let first = r.provider_terminal_source(&pins[0]).unwrap();
    for pin in pins {
        let source = r.provider_terminal_source(&pin).unwrap();
        assert_eq!(source.pin(), &pin);
        assert!(Arc::ptr_eq(
            &source,
            &r.provider_terminal_source(&pin).unwrap()
        ));
        assert!(std::ptr::eq(
            first.observation_type(),
            source.observation_type()
        ));
        assert!(std::ptr::eq(
            first.terminal_observations(),
            source.terminal_observations()
        ));
    }
}

#[test]
fn finite_state_vectors_exercise_the_actual_verified_provider_reader() {
    let corpus: Value = serde_json::from_slice(include_bytes!(
        "conformance/terminal-observation-sources.json"
    ))
    .unwrap();
    assert_eq!(
        corpus["profile"]["revision"],
        TERMINAL_OBSERVATION_PROFILE.1
    );
    for case in corpus["cases"].as_array().unwrap() {
        let mut f = Fixture::real();
        f.provider = minimal();
        let mut observations = serde_json::Map::new();
        for (name, state) in case["states"].as_object().unwrap() {
            observations.insert(name.clone(),json!({"state":state,"result":if state=="succeeded"{json!({"kind":"integer"})}else{Value::Null},"outcome":if state=="succeeded"{json!("done")}else{Value::Null}}));
        }
        f.provider["observations"] = observations.into();
        f.replace_provider();
        let pin = f.bind();
        let supplied = f.supplied();
        let actual = Resources::new(&supplied)
            .unwrap()
            .provider_terminal_source(&pin);
        if let Some(code) = case["expect"]["code"].as_str() {
            assert_eq!(actual.unwrap_err().code, code, "{}", case["id"]);
        } else {
            let source = actual.unwrap();
            let terminal: serde_json::Map<String, Value> = source
                .terminal_observations()
                .iter()
                .map(|(n, d)| (n.clone(), d.tag().as_str().into()))
                .collect();
            assert_eq!(
                Value::Object(terminal),
                case["expect"]["terminal"],
                "{}",
                case["id"]
            );
            let Type::Variant(cases) = source.observation_type() else {
                panic!("variant required")
            };
            assert_eq!(
                json!(cases.keys().collect::<Vec<_>>()),
                case["expect"]["cases"],
                "{}",
                case["id"]
            );
            for (tag, ty) in cases {
                assert_eq!(
                    ty,
                    &if tag == "confirmed" {
                        Type::Integer
                    } else {
                        correlation()
                    }
                );
            }
            assert_eq!(source.source_protection(), &f.provider["resultProtection"]);
        }
    }
}
