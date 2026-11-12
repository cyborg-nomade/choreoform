// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Standalone declared metadata, never accepted requests or independence evidence.
use choreoform_ir_probe_core::{Resource, digest};
use choreoform_semantic_check::{
    CORE_V12, ExternalFootprint, INTERFERENCE_PROFILE, Pin, RegionKeyType, RequestSelectorStep,
    Resources, Type,
};
use serde_json::{Value, json};
use std::sync::Arc;

const PROFILE: &[u8] = include_bytes!(
    "../../../docs/ir/contracts/sha256-0b10eb4dbb19caace45b93f35dec44aa5f26458d631393932caddb5589210250.txt"
);
const CORE: &[u8] = include_bytes!(
    "../../../docs/ir/contracts/sha256-73158731c33084fde009abcc86530926447902609fda9535f04daa7c768c2007.txt"
);
struct Fixture {
    artifacts: Vec<(Pin, Vec<u8>)>,
    provider: Value,
    descriptor: Value,
    namespace: Value,
}
fn p(id: &str, revision: &str) -> Value {
    json!({"id":id,"revision":revision})
}
fn pref(pin: &Pin) -> Value {
    p(&pin.id, &pin.revision)
}
impl Fixture {
    fn raw(&mut self, id: &str, bytes: &[u8]) -> Pin {
        let pin = Pin {
            id: id.into(),
            revision: digest(bytes),
        };
        if !self.artifacts.iter().any(|(old, _)| old == &pin) {
            self.artifacts.push((pin.clone(), bytes.to_vec()));
        }
        pin
    }
    fn artifact(&mut self, doc: &Value) -> Pin {
        self.raw(
            doc["id"].as_str().unwrap(),
            &serde_json::to_vec(doc).unwrap(),
        )
    }
    fn new() -> Self {
        let mut f = Self {
            artifacts: vec![],
            provider: serde_json::from_slice(include_bytes!(
                "../../../docs/dialects/drafts/external-interference-examples/provider-a.json"
            ))
            .unwrap(),
            descriptor: Value::Null,
            namespace: Value::Null,
        };
        f.raw(INTERFERENCE_PROFILE.0, PROFILE);
        f.raw(CORE_V12.0, CORE);
        let authority = f.raw(
            "urn:test:authority",
            b"Synthetic canonical-authority behavior contract; no host registration.\n",
        );
        f.namespace = json!({"kind":"external-region-namespace","version":"0.1.0","id":"urn:test:namespace","profile":p(INTERFERENCE_PROFILE.0,INTERFERENCE_PROFILE.1),"authority":pref(&authority),"keyTypes":["integer"]});
        let ns = f.artifact(&f.namespace.clone());
        let provider = f.artifact(&f.provider.clone());
        let region = json!({"namespace":pref(&ns),"key":[{"path":[],"type":"integer"}]});
        f.descriptor = json!({"kind":"external-interference","version":"0.1.0","id":"urn:test:descriptor","profile":p(INTERFERENCE_PROFILE.0,INTERFERENCE_PROFILE.1),"providerProfile":p(CORE_V12.0,CORE_V12.1),"provider":pref(&provider),"coverage":"exhaustive","activity":{"reads":[region.clone()],"writes":[region.clone()]},"terminal":{"reads":[region],"writes":[]}});
        f
    }
    fn supplied(&self) -> Vec<Resource<'_>> {
        self.artifacts
            .iter()
            .map(|(pin, bytes)| Resource {
                id: &pin.id,
                revision: &pin.revision,
                bytes,
            })
            .collect()
    }
    fn bind(&mut self) -> Pin {
        self.artifact(&self.descriptor.clone())
    }
    fn provider(&mut self) {
        let pin = self.artifact(&self.provider.clone());
        self.descriptor["provider"] = pref(&pin);
    }
    fn namespace(&mut self) {
        let pin = self.artifact(&self.namespace.clone());
        for channel in ["activity", "terminal"] {
            for access in ["reads", "writes"] {
                for region in self.descriptor[channel][access].as_array_mut().unwrap() {
                    region["namespace"] = pref(&pin);
                }
            }
        }
    }
    fn decode(&mut self) -> Arc<choreoform_semantic_check::ProviderInterference> {
        let pin = self.bind();
        let supplied = self.supplied();
        Resources::new(&supplied)
            .unwrap()
            .provider_interference(&pin)
            .unwrap()
    }
    fn refuse(&mut self, code: &str) {
        let pin = self.bind();
        let supplied = self.supplied();
        let error = Resources::new(&supplied)
            .unwrap()
            .provider_interference(&pin)
            .unwrap_err();
        assert_eq!(error.code, code, "{error:?}");
        assert!(!format!("{error:?}").contains("SECRET-MARKER"));
    }
}

#[test]
fn exact_complete_provider_metadata_and_both_channels_are_shared_without_admission() {
    let mut f = Fixture::new();
    let pin = f.bind();
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    let d = r.provider_interference(&pin).unwrap();
    assert_eq!(d.pin(), &pin);
    assert_eq!(d.profile().revision, INTERFERENCE_PROFILE.1);
    assert_eq!(d.provider_profile().revision, CORE_V12.1);
    assert_eq!(d.provider_contract().input(), &Type::Integer);
    assert!(Arc::ptr_eq(
        d.provider_contract(),
        &r.provider(d.provider()).unwrap()
    ));
    assert!(Arc::ptr_eq(&d, &r.provider_interference(&pin).unwrap()));
    let ExternalFootprint::Finite { reads, writes } = d.activity() else {
        panic!()
    };
    assert_eq!((reads.len(), writes.len()), (1, 1));
    assert!(Arc::ptr_eq(reads[0].namespace(), writes[0].namespace()));
    assert_eq!(reads[0].key()[0].key_type(), RegionKeyType::Integer);
    assert_eq!(reads[0].key()[0].path(), &[]);
    let ns = reads[0].namespace();
    assert_eq!(ns.pin().id, "urn:test:namespace");
    assert_eq!(ns.profile(), d.profile());
    assert_eq!(ns.key_types(), &[RegionKeyType::Integer]);
    assert_eq!(ns.key_types()[0].as_str(), "integer");
    assert_eq!(ns.authority().id, "urn:test:authority");
    assert!(!ns.remaining().is_empty());
    assert!(
        d.remaining()
            .contains(&"explicit-unique-semantic-definition-selection-and-dependency-closure")
    );
}

#[test]
fn opacity_and_empty_readonly_footprints_are_declarations_only() {
    let mut f = Fixture::new();
    f.descriptor["activity"] = json!("opaque");
    f.descriptor["terminal"] = json!("opaque");
    let d = f.decode();
    assert!(matches!(d.activity(), ExternalFootprint::Opaque));
    assert!(matches!(d.terminal(), ExternalFootprint::Opaque));
    let mut f = Fixture::new();
    f.provider["effectClass"] = json!("read-only");
    f.provider();
    f.descriptor["activity"] = json!({"reads":[],"writes":[]});
    f.decode();
    let mut f = Fixture::new();
    f.provider["effectClass"] = json!("irreversible");
    f.provider();
    f.decode();
}

#[test]
fn class_coverage_and_terminal_effect_claims_refuse_without_relabeling_provider() {
    let mut f = Fixture::new();
    f.provider["effectClass"] = json!("read-only");
    f.provider();
    f.refuse("interference-class");
    let mut f = Fixture::new();
    f.descriptor["activity"]["writes"] = json!([]);
    f.refuse("interference-class");
    let mut f = Fixture::new();
    f.descriptor["terminal"]["writes"] = f.descriptor["activity"]["writes"].clone();
    f.refuse("interference-class");
    let mut f = Fixture::new();
    f.descriptor["coverage"] = json!("partial");
    f.refuse("interference-coverage");
}

#[test]
fn exact_profiles_versions_shapes_and_unknown_fields_refuse() {
    for field in ["unknown", "independent", "currentRequest"] {
        let mut f = Fixture::new();
        f.descriptor[field] = json!("SECRET-MARKER");
        f.refuse("fields");
    }
    for version in ["0.1.0-draft.1", "0.2.0"] {
        let mut f = Fixture::new();
        f.descriptor["version"] = json!(version);
        f.refuse("resource-format");
    }
    let mut f = Fixture::new();
    f.descriptor["profile"] = p(CORE_V12.0, CORE_V12.1);
    f.refuse("interference-profile-unsupported");
    let mut f = Fixture::new();
    f.descriptor["providerProfile"] = p(INTERFERENCE_PROFILE.0, INTERFERENCE_PROFILE.1);
    f.refuse("interference-provider-profile");
    let mut f = Fixture::new();
    f.namespace["version"] = json!("0.2.0");
    f.namespace();
    f.refuse("resource-format");
    let mut f = Fixture::new();
    f.namespace["extra"] = json!(true);
    f.namespace();
    f.refuse("fields");
}

#[test]
fn all_supplied_pins_and_authority_utf8_integrity_are_required() {
    for id in [
        INTERFERENCE_PROFILE.0,
        CORE_V12.0,
        "urn:test:namespace",
        "urn:test:authority",
    ] {
        let mut f = Fixture::new();
        f.artifacts.retain(|(pin, _)| pin.id != id);
        f.refuse("resource-missing");
    }
    for id in [
        INTERFERENCE_PROFILE.0,
        CORE_V12.0,
        "urn:test:namespace",
        "urn:test:authority",
    ] {
        let mut f = Fixture::new();
        f.artifacts
            .iter_mut()
            .find(|(pin, _)| pin.id == id)
            .unwrap()
            .1
            .push(b' ');
        f.refuse("resource-digest");
    }
    for raw in [b"".as_slice(), b"\xff".as_slice()] {
        let mut f = Fixture::new();
        let authority = f.raw("urn:test:other-authority", raw);
        f.namespace["authority"] = pref(&authority);
        f.namespace();
        f.refuse("interference-authority-contract");
    }
    let mut f = Fixture::new();
    f.provider["extra"] = json!(true);
    f.provider();
    f.refuse("fields");
    let mut f = Fixture::new();
    f.provider["observations"]["done"]["result"] = json!({"kind":"text"});
    f.provider();
    f.refuse("provider-result");
}

#[test]
fn composite_record_option_paths_are_typed_without_assuming_present_values() {
    let mut f = Fixture::new();
    f.provider["input"] = json!({"kind":"record","fields":{"tenant":{"kind":"text"},"slot":{"kind":"option","element":{"kind":"integer"}}}});
    f.provider();
    f.namespace["keyTypes"] = json!(["text", "integer"]);
    f.namespace();
    for channel in ["activity", "terminal"] {
        for access in ["reads", "writes"] {
            for region in f.descriptor[channel][access].as_array_mut().unwrap() {
                region["key"] = json!([{"path":[{"op":"field","name":"tenant"}],"type":"text"},{"path":[{"op":"field","name":"slot"},{"op":"present"}],"type":"integer"}]);
            }
        }
    }
    let d = f.decode();
    let ExternalFootprint::Finite { reads, .. } = d.activity() else {
        panic!()
    };
    assert_eq!(
        reads[0].key()[1].path(),
        &[
            RequestSelectorStep::Field("slot".into()),
            RequestSelectorStep::Present
        ]
    );
    // No request value, event or assumed Option-present Boolean was supplied.
}

#[test]
fn actual_unchanged_rp03_nominal_graph_has_explicit_representation_paths() {
    let mut f = Fixture::new();
    let types = f.raw(
        "urn:choreoform:benchmarks:rp03:types:0.1.0",
        include_bytes!("../../../docs/benchmarks/rp03/types.json"),
    );
    f.provider = serde_json::from_slice(include_bytes!(
        "../../../docs/benchmarks/rp03/reserve-a.provider.json"
    ))
    .unwrap();
    f.provider();
    f.namespace["keyTypes"] = json!(["text"]);
    f.namespace();
    for channel in ["activity", "terminal"] {
        for access in ["reads", "writes"] {
            for region in f.descriptor[channel][access].as_array_mut().unwrap() {
                region["key"] = json!([{"path":[{"op":"field","name":"order"},{"op":"representation"},{"op":"field","name":"first line"},{"op":"field","name":"warehouse"}],"type":"text"}]);
            }
        }
    }
    let d = f.decode();
    assert!(matches!(d.provider_contract().input(), Type::Record(_)));
    assert_eq!(
        types.revision,
        digest(include_bytes!("../../../docs/benchmarks/rp03/types.json"))
    );
    f.descriptor["terminal"]["reads"][0]["key"][0]["path"][1] =
        json!({"op":"field","name":"first line"});
    f.refuse("interference-selector-type");
}

#[test]
fn missing_nominal_bad_unused_invariant_and_deep_graph_refuse() {
    let mut f = Fixture::new();
    let resource = json!({"kind":"named-types","version":"0.1.0","id":"urn:test:types","definitions":{"N":{"representation":{"kind":"integer"},"invariant":{"op":"literal","type":{"kind":"boolean"},"value":true}},"Unused":{"representation":{"kind":"integer"},"invariant":{"op":"literal","type":{"kind":"integer"},"value":"0"}}}});
    let named = f.artifact(&resource);
    f.provider["input"] = json!({"kind":"named","contract":pref(&named),"name":"N"});
    f.provider();
    f.refuse("nominal-invariant");
    f.artifacts.retain(|(pin, _)| pin != &named);
    f.refuse("resource-missing");
    let mut f = Fixture::new();
    let mut ty = json!({"kind":"integer"});
    for _ in 0..65 {
        ty = json!({"kind":"option","element":ty});
    }
    f.provider["input"] = ty;
    f.provider();
    let pin = f.bind();
    let supplied = f.supplied();
    assert!(
        Resources::new(&supplied)
            .unwrap()
            .provider_interference(&pin)
            .is_err()
    );
}

#[test]
fn unsupported_paths_leaf_types_and_namespace_arity_refuse_in_either_channel() {
    for input in [
        json!({"kind":"boolean"}),
        json!({"kind":"decimal","scale":2}),
        json!({"kind":"list","element":{"kind":"integer"}}),
        json!({"kind":"option","element":{"kind":"integer"}}),
        json!({"kind":"variant","cases":{"one":{"kind":"integer"}}}),
    ] {
        let mut f = Fixture::new();
        f.provider["input"] = input;
        f.provider();
        f.refuse("interference-selector-type");
    }
    for channel in ["activity", "terminal"] {
        for path in [
            json!([{"op":"field","name":"missing"}]),
            json!([{"op":"present"}]),
            json!([{"op":"representation"}]),
        ] {
            let mut f = Fixture::new();
            f.descriptor[channel]["reads"][0]["key"][0]["path"] = path;
            f.refuse("interference-selector-type");
        }
        let mut f = Fixture::new();
        f.descriptor[channel]["reads"][0]["key"][0]["path"] = json!([{"op":"current"}]);
        f.refuse("interference-selector-step");
    }
    for kind in ["boolean", "decimal", "list", "unknown"] {
        let mut f = Fixture::new();
        f.descriptor["activity"]["reads"][0]["key"][0]["type"] = json!(kind);
        f.refuse("interference-key-type");
    }
    let mut f = Fixture::new();
    f.namespace["keyTypes"] = json!(["text"]);
    f.namespace();
    f.refuse("interference-key-type");
    let mut f = Fixture::new();
    f.descriptor["activity"]["reads"][0]["key"] = json!([]);
    f.refuse("interference-key-arity");
}

#[test]
fn finite_path_region_key_and_utf8_name_bounds_precede_traversal() {
    let mut f = Fixture::new();
    f.descriptor["activity"]["reads"][0]["key"][0]["path"] =
        json!(vec![json!({"op":"present"}); 9]);
    f.refuse("interference-selector-depth");
    let mut f = Fixture::new();
    f.descriptor["activity"]["reads"] =
        json!(vec![f.descriptor["activity"]["reads"][0].clone(); 17]);
    f.refuse("interference-regions");
    let mut f = Fixture::new();
    f.namespace["keyTypes"] = json!(["integer", "integer", "integer", "integer", "integer"]);
    f.namespace();
    f.refuse("interference-key-arity");
    let mut f = Fixture::new();
    f.descriptor["activity"]["reads"][0]["key"][0]["path"] =
        json!([{"op":"field","name":"é".repeat(129)}]);
    f.refuse("interference-name");
    let mut f = Fixture::new();
    f.namespace["id"] = json!("x".repeat(257));
    f.namespace();
    f.refuse("interference-name");
}

#[test]
fn failed_parent_is_not_cached_and_storing_two_versions_does_not_select_either() {
    let mut f = Fixture::new();
    let good = f.bind();
    f.descriptor["terminal"]["reads"][0]["key"][0]["path"] = json!([{"op":"present"}]);
    let bad = f.bind();
    f.descriptor["terminal"] = json!("opaque");
    let other = f.bind();
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    for _ in 0..2 {
        assert_eq!(
            r.provider_interference(&bad).unwrap_err().code,
            "interference-selector-type"
        );
    }
    let a = r.provider_interference(&good).unwrap();
    let b = r.provider_interference(&other).unwrap();
    assert_eq!(a.provider(), b.provider());
    assert_ne!(a.pin(), b.pin());
    assert!(!Arc::ptr_eq(&a, &b));
    assert!(Arc::ptr_eq(&a, &r.provider_interference(&good).unwrap()));
}

#[test]
fn supplied_duplicate_and_corrupt_transport_refuse() {
    let mut f = Fixture::new();
    let pin = f.bind();
    let supplied = f.supplied();
    let mut duplicate = f.supplied();
    duplicate.push(Resource {
        id: supplied[0].id,
        revision: supplied[0].revision,
        bytes: supplied[0].bytes,
    });
    assert_eq!(
        Resources::new(&duplicate).err().unwrap().code,
        "resource-ambiguous"
    );
    let raw = br#"{"kind":"external-interference","kind":"external-interference"}"#;
    let bad = f.raw("urn:test:duplicate-json", raw);
    let supplied = f.supplied();
    assert_eq!(
        Resources::new(&supplied)
            .unwrap()
            .provider_interference(&bad)
            .unwrap_err()
            .code,
        "resource-transport"
    );
    let mut f = Fixture::new();
    let id = f.descriptor["provider"]["id"].as_str().unwrap().to_owned();
    f.artifacts
        .iter_mut()
        .find(|(p, _)| p.id == id)
        .unwrap()
        .1
        .push(b' ');
    f.refuse("resource-digest");
    assert_eq!(pin.id, "urn:test:descriptor");
}

#[test]
fn repeated_cached_reads_share_one_exhaustible_budget_without_reset() {
    let mut f = Fixture::new();
    let pin = f.bind();
    let supplied = f.supplied();
    let mut resources = Resources::new(&supplied).unwrap();
    resources.provider_interference(&pin).unwrap();
    let mut exhausted = false;
    for _ in 0..1_000_001 {
        if let Err(error) = resources.provider_interference(&pin) {
            assert_eq!(error.code, "budget");
            exhausted = true;
            break;
        }
    }
    assert!(exhausted);
    assert_eq!(
        resources.provider_interference(&pin).unwrap_err().code,
        "budget"
    );
}

#[test]
fn wide_input_and_maximum_finite_footprints_use_borrowed_type_paths() {
    let mut f = Fixture::new();
    let fields: serde_json::Map<String, Value> = (0..1024)
        .map(|n| (format!("field{n}"), json!({"kind":"integer"})))
        .collect();
    f.provider["input"] = json!({"kind":"record","fields":fields});
    f.provider();
    let mut region = f.descriptor["activity"]["reads"][0].clone();
    region["key"][0]["path"] = json!([{"op":"field","name":"field1023"}]);
    f.descriptor["activity"] =
        json!({"reads":vec![region.clone();16],"writes":vec![region.clone();16]});
    f.descriptor["terminal"] = json!({"reads":vec![region;16],"writes":[]});
    let d = f.decode();
    let ExternalFootprint::Finite { reads, writes } = d.activity() else {
        panic!()
    };
    assert_eq!((reads.len(), writes.len()), (16, 16));
}
