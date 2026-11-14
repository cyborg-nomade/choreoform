// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Real raw SDK selection checks over fictional declarations, never host events.
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource, digest};
use choreoform_semantic_check::{
    CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, CORE_V6, CORE_V7, CORE_V8, CORE_V9, CORE_V10,
    CORE_V11, CORE_V12, CORE_V13, INTERFERENCE_PROFILE, Pin, SEMANTICS, check, check_concurrency,
    check_control, check_invalidation, check_protection,
};
use serde_json::{Value, json};
use std::path::Path;
use std::sync::Arc;

struct Fixture {
    b: Value,
    artifacts: Vec<(Pin, Vec<u8>)>,
    descriptors: [Pin; 2],
}
fn pref(pin: &Pin) -> Value {
    json!({"id":pin.id,"revision":pin.revision})
}
fn core(pin: (&str, &str)) -> Value {
    json!({"id":pin.0,"revision":pin.1})
}
impl Fixture {
    fn new() -> Self {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let document: Value =
            serde_json::from_slice(include_bytes!("interference/parallel-effects.ir.json"))
                .unwrap();
        let mut artifacts = vec![];
        for (id, revision) in [
            SEMANTICS,
            CORE,
            CORE_V2,
            CORE_V3,
            CORE_V4,
            CORE_V5,
            CORE_V6,
            CORE_V7,
            CORE_V8,
            CORE_V9,
            CORE_V10,
            CORE_V11,
            CORE_V12,
            CORE_V13,
            INTERFERENCE_PROFILE,
        ] {
            artifacts.push((
                Pin {
                    id: id.into(),
                    revision: revision.into(),
                },
                std::fs::read(root.join(format!(
                    "docs/ir/contracts/{}.txt",
                    revision.replace(':', "-")
                )))
                .unwrap(),
            ));
        }
        let dummy = Pin {
            id: "unused".into(),
            revision: "unused".into(),
        };
        let mut f = Self {
            b: document["body"].clone(),
            artifacts,
            descriptors: [dummy.clone(), dummy],
        };
        for raw in [
            include_bytes!(
                "../../../docs/dialects/drafts/external-interference-examples/provider-a.json"
            )
            .as_slice(),
            include_bytes!(
                "../../../docs/dialects/drafts/external-interference-examples/provider-b.json"
            )
            .as_slice(),
            include_bytes!("../../../docs/dialects/interference-examples/namespace.json")
                .as_slice(),
        ] {
            let doc: Value = serde_json::from_slice(raw).unwrap();
            f.raw(doc["id"].as_str().unwrap(), raw);
        }
        f.raw(
            "urn:test:standalone:authority",
            include_bytes!("../../../docs/dialects/interference-examples/synthetic-authority.txt"),
        );
        let mut descriptor: Value = serde_json::from_slice(include_bytes!(
            "../../../docs/dialects/interference-examples/descriptor.json"
        ))
        .unwrap();
        f.descriptors[0] = f.artifact(&descriptor);
        descriptor["id"] = json!("urn:test:standalone:descriptor-b");
        descriptor["provider"] = f.b["policies"]["b_cap_provider"]["body"]["provider"].clone();
        f.descriptors[1] = f.artifact(&descriptor);
        f
    }
    fn raw(&mut self, id: &str, raw: &[u8]) -> Pin {
        let pin = Pin {
            id: id.into(),
            revision: digest(raw),
        };
        if !self.artifacts.iter().any(|(old, _)| old == &pin) {
            self.artifacts.push((pin.clone(), raw.to_vec()));
        }
        pin
    }
    fn artifact(&mut self, doc: &Value) -> Pin {
        self.raw(
            doc["id"].as_str().unwrap(),
            &serde_json::to_vec(doc).unwrap(),
        )
    }
    fn upgrade(&mut self) {
        self.b["dialects"]["core"] = core(CORE_V13);
        for policy in self.b["policies"].as_object_mut().unwrap().values_mut() {
            if policy["body"]["kind"] == "capability" {
                policy["body"]["interference"] = Value::Null;
            }
        }
    }
    fn select(&mut self) {
        for (n, key) in ["a_cap_provider", "b_cap_provider"].into_iter().enumerate() {
            self.b["policies"][key]["body"]["interference"] = pref(&self.descriptors[n]);
        }
    }
    fn env(&self) -> DefinitionEnvelope {
        DefinitionEnvelope::from_parts_versioned("0.4.0", self.b.clone(), json!({"synthetic":true}))
            .unwrap()
    }
    fn supplied(&self) -> Vec<Resource<'_>> {
        self.artifacts
            .iter()
            .map(|(pin, raw)| Resource {
                id: &pin.id,
                revision: &pin.revision,
                bytes: raw,
            })
            .collect()
    }
    fn refuse(&self, code: &str) {
        let e = check(&self.env(), &self.supplied()).unwrap_err();
        assert_eq!(e.code, code, "{e:?}");
        assert!(!format!("{e:?}").contains("SECRET-MARKER"));
    }
}

#[test]
fn old_core_and_new_explicit_null_have_no_declared_selection_and_keep_concurrency_refusal() {
    let mut f = Fixture::new();
    assert!(
        check(&f.env(), &f.supplied())
            .unwrap()
            .interference_bindings()
            .is_empty()
    );
    f.upgrade();
    assert!(
        check(&f.env(), &f.supplied())
            .unwrap()
            .interference_bindings()
            .is_empty()
    );
    let e = check_concurrency(&f.env(), &f.supplied()).unwrap_err();
    assert_eq!(e.code, "parallel-effects-unproved");
    assert_eq!(e.path, "/body/nodes/fork");
}

#[test]
fn exact_selected_metadata_is_revision_bound_without_any_concurrency_premise() {
    let mut f = Fixture::new();
    f.upgrade();
    let unselected = f.env();
    f.select();
    let selected = f.env();
    assert_ne!(
        unselected.document()["revision"],
        selected.document()["revision"]
    );
    let report = check(&selected, &f.supplied()).unwrap();
    let uses = report.interference_bindings();
    assert_eq!(uses.len(), 2);
    for (n, key) in ["a_cap_provider", "b_cap_provider"].into_iter().enumerate() {
        assert_eq!(uses[n].policy(), key);
        assert_eq!(uses[n].scope(), if n == 0 { "a" } else { "b" });
        assert_eq!(uses[n].definition_revision(), report.revision());
        assert_eq!(uses[n].descriptor().pin(), &f.descriptors[n]);
        assert!(uses[n].remaining().contains(
            &"generic-original-request-disjointness-for-all-overlapping-surviving-occurrences"
        ));
    }
    check_control(&selected, &f.supplied()).unwrap();
    check_protection(&selected, &f.supplied()).unwrap();
    check_invalidation(&selected, &f.supplied()).unwrap();
    assert_eq!(
        check_concurrency(&selected, &f.supplied())
            .unwrap_err()
            .code,
        "parallel-effects-unproved"
    );
}

#[test]
fn identical_unused_wrapper_selection_is_checked_and_shares_metadata() {
    let mut f = Fixture::new();
    f.upgrade();
    f.select();
    f.b["policies"]["unused_contract"] = f.b["policies"]["a_cap_provider"].clone();
    let r = check(&f.env(), &f.supplied()).unwrap();
    let uses = r.interference_bindings();
    assert_eq!(uses.len(), 3);
    assert!(Arc::ptr_eq(uses[0].descriptor(), uses[2].descriptor()));
    assert_eq!(uses[2].policy(), "unused_contract");
}

#[test]
fn same_exact_provider_cannot_mix_null_ref_or_conflicting_descriptor_versions() {
    for original_null in [false, true] {
        let mut f = Fixture::new();
        f.upgrade();
        if !original_null {
            f.select();
        }
        let mut other = f.b["policies"]["a_cap_provider"].clone();
        other["body"]["interference"] = if original_null {
            pref(&f.descriptors[0])
        } else {
            Value::Null
        };
        f.b["policies"]["unused_contract"] = other;
        f.refuse("interference-selection-conflict");
    }
    let mut f = Fixture::new();
    f.upgrade();
    f.select();
    let raw = f
        .artifacts
        .iter()
        .find(|(p, _)| p == &f.descriptors[0])
        .unwrap()
        .1
        .clone();
    let mut d: Value = serde_json::from_slice(&raw).unwrap();
    d["terminal"] = json!("opaque");
    let changed = f.artifact(&d);
    f.b["policies"]["unused_contract"] = f.b["policies"]["a_cap_provider"].clone();
    f.b["policies"]["unused_contract"]["body"]["interference"] = pref(&changed);
    f.refuse("interference-selection-conflict");
}

#[test]
fn foreign_provider_same_id_different_revision_and_malformed_refs_refuse() {
    let mut f = Fixture::new();
    f.upgrade();
    f.select();
    f.b["policies"]["a_cap_provider"]["body"]["interference"] = pref(&f.descriptors[1]);
    f.refuse("interference-provider-binding");
    let mut f = Fixture::new();
    f.upgrade();
    f.select();
    let provider = f.b["policies"]["a_cap_provider"]["body"]["provider"].clone();
    let pin = Pin::decode(&provider).unwrap();
    let raw = f
        .artifacts
        .iter()
        .find(|(p, _)| p == &pin)
        .unwrap()
        .1
        .clone();
    let mut changed = raw;
    changed.push(b' ');
    let new = f.raw(&pin.id, &changed);
    f.b["policies"]["a_cap_provider"]["body"]["provider"] = pref(&new);
    f.refuse("interference-provider-binding");
    for value in [
        json!("SECRET-MARKER"),
        json!({"id":"SECRET-MARKER","revision":"bad"}),
        json!({"id":"x","revision":format!("sha256:{}","0".repeat(64)),"extra":true}),
    ] {
        let mut f = Fixture::new();
        f.upgrade();
        f.b["policies"]["a_cap_provider"]["body"]["interference"] = value;
        let e = check(&f.env(), &f.supplied()).unwrap_err();
        assert!(!format!("{e:?}").contains("SECRET-MARKER"));
    }
}

#[test]
fn all_twelve_bases_new_core_and_standalone_profile_are_exact_even_when_unselected() {
    for id in [
        CORE.0,
        CORE_V2.0,
        CORE_V3.0,
        CORE_V4.0,
        CORE_V5.0,
        CORE_V6.0,
        CORE_V7.0,
        CORE_V8.0,
        CORE_V9.0,
        CORE_V10.0,
        CORE_V11.0,
        CORE_V12.0,
        CORE_V13.0,
        INTERFERENCE_PROFILE.0,
    ] {
        let mut f = Fixture::new();
        f.upgrade();
        f.artifacts.retain(|(pin, _)| pin.id != id);
        f.refuse("resource-missing");
        let mut f = Fixture::new();
        f.upgrade();
        f.artifacts
            .iter_mut()
            .find(|(pin, _)| pin.id == id)
            .unwrap()
            .1
            .push(b' ');
        f.refuse("resource-digest");
    }
}

#[test]
fn old_new_fields_unknown_core_mixed_alias_and_wire_guards_stay_closed() {
    let mut f = Fixture::new();
    f.select();
    f.refuse("fields");
    let mut f = Fixture::new();
    f.upgrade();
    f.b["policies"]["a_cap_provider"]["body"]
        .as_object_mut()
        .unwrap()
        .remove("interference");
    f.refuse("fields");
    let mut f = Fixture::new();
    f.upgrade();
    f.b["dialects"]["old"] = core(CORE_V12);
    f.refuse("retained-request-wire-version");
    let mut f = Fixture::new();
    f.upgrade();
    f.b["dialects"]["core"]["revision"] = json!(format!("sha256:{}", "0".repeat(64)));
    f.refuse("dialect-contract");
    let mut f = Fixture::new();
    f.upgrade();
    f.b["dialects"]["core"]["id"] = json!("urn:choreoform:contracts:core-profile:0.14.0");
    f.refuse("retained-request-wire-version");
    let mut f = Fixture::new();
    f.upgrade();
    f.b["dialects"]["also"] = core(CORE_V13);
    check(&f.env(), &f.supplied()).unwrap();
}

#[test]
fn transitive_byte_substitution_refuses_and_valid_rebinding_changes_definition_revision() {
    for id in [
        "urn:test:standalone:namespace",
        "urn:test:standalone:authority",
    ] {
        let mut f = Fixture::new();
        f.upgrade();
        f.select();
        f.artifacts
            .iter_mut()
            .find(|(p, _)| p.id == id)
            .unwrap()
            .1
            .push(b' ');
        f.refuse("resource-digest");
    }
    let mut f = Fixture::new();
    f.upgrade();
    f.select();
    let old = f.env();
    let authority = f.raw(
        "urn:test:new-authority",
        b"Synthetic replacement authority contract; no registered support.\n",
    );
    let raw = f
        .artifacts
        .iter()
        .find(|(p, _)| p.id == "urn:test:standalone:namespace")
        .unwrap()
        .1
        .clone();
    let mut ns: Value = serde_json::from_slice(&raw).unwrap();
    ns["authority"] = pref(&authority);
    let nspin = f.artifact(&ns);
    for n in 0..2 {
        let raw = f
            .artifacts
            .iter()
            .find(|(p, _)| p == &f.descriptors[n])
            .unwrap()
            .1
            .clone();
        let mut d: Value = serde_json::from_slice(&raw).unwrap();
        for channel in ["activity", "terminal"] {
            for access in ["reads", "writes"] {
                for region in d[channel][access].as_array_mut().unwrap() {
                    region["namespace"] = pref(&nspin);
                }
            }
        }
        f.descriptors[n] = f.artifact(&d);
    }
    f.select();
    let new = f.env();
    assert_ne!(old.document()["revision"], new.document()["revision"]);
    let r = check(&new, &f.supplied()).unwrap();
    assert_eq!(r.interference_bindings().len(), 2);
    assert_eq!(
        check_concurrency(&new, &f.supplied()).unwrap_err().code,
        "parallel-effects-unproved"
    );
}

#[test]
fn unused_wrapper_keeps_lexical_provider_protection_and_does_not_grant_sibling_access() {
    let mut f = Fixture::new();
    f.upgrade();
    f.select();
    f.b["policies"]["unused_contract"] = f.b["policies"]["a_cap_provider"].clone();
    f.b["policies"]["unused_contract"]["scope"] = json!("b");
    let e = check(&f.env(), &f.supplied()).unwrap_err();
    assert_eq!(e.code, "visibility");
    assert_eq!(e.path, "/body/policies/unused_contract");
}

#[test]
fn bounded_many_unused_selections_share_one_metadata_graph() {
    let mut f = Fixture::new();
    f.upgrade();
    f.select();
    let wrapper = f.b["policies"]["a_cap_provider"].clone();
    for n in 0..1024 {
        f.b["policies"][format!("unused_{n}")] = wrapper.clone();
    }
    let r = check(&f.env(), &f.supplied()).unwrap();
    assert_eq!(r.interference_bindings().len(), 1026);
    let first = r.interference_bindings()[0].descriptor();
    for use_ in r
        .interference_bindings()
        .iter()
        .filter(|u| u.scope() == "a")
    {
        assert!(Arc::ptr_eq(first, use_.descriptor()));
    }
}

#[test]
fn derived_complete_rp03_raw_variant_keeps_inherited_proofs_and_actual_concurrency_refusal() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut f = Fixture::new();
    let document: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/benchmarks/rp03/candidate.ir.json"
    ))
    .unwrap();
    f.b = document["body"].clone();
    let manifest: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/benchmarks/rp03/manifest.json"
    ))
    .unwrap();
    for resource in manifest["resources"].as_array().unwrap() {
        let raw = std::fs::read(root.join(resource["path"].as_str().unwrap())).unwrap();
        let pin = f.raw(resource["id"].as_str().unwrap(), &raw);
        assert_eq!(pin.revision, resource["revision"].as_str().unwrap());
    }
    f.upgrade();
    let authority = f.raw(
        "urn:test:whole:authority",
        b"Synthetic metadata authority contract only, not registered physical mappings.\n",
    );
    let namespace = f.artifact(&json!({"kind":"external-region-namespace","version":"0.1.0","id":"urn:test:whole:namespace","profile":core(INTERFERENCE_PROFILE),"authority":pref(&authority),"keyTypes":["text"]}));
    for key in ["a", "b"] {
        let provider_id = format!("urn:choreoform:benchmarks:rp03:reserve-{key}:0.1.0");
        let wrappers: Vec<_> = f.b["policies"]
            .as_object()
            .unwrap()
            .iter()
            .filter(|(_, p)| {
                p["body"]["kind"] == "capability" && p["body"]["provider"]["id"] == provider_id
            })
            .map(|(id, p)| (id.clone(), p["body"]["provider"].clone()))
            .collect();
        assert!(!wrappers.is_empty());
        let region = json!({"namespace":pref(&namespace),"key":[{"path":[{"op":"field","name":"target warehouse"},{"op":"present"}],"type":"text"}]});
        let descriptor = f.artifact(&json!({"kind":"external-interference","version":"0.1.0","id":format!("urn:test:whole:reserve-{key}:regions"),"profile":core(INTERFERENCE_PROFILE),"providerProfile":core(CORE_V12),"provider":wrappers[0].1,"coverage":"exhaustive","activity":{"reads":[region.clone()],"writes":[region.clone()]},"terminal":{"reads":[region],"writes":[]}}));
        for (id, _) in wrappers {
            f.b["policies"][id]["body"]["interference"] = pref(&descriptor);
        }
    }
    let env = f.env();
    let supplied = f.supplied();
    assert_eq!(
        check(&env, &supplied)
            .unwrap()
            .interference_bindings()
            .len(),
        2
    );
    check_control(&env, &supplied).unwrap();
    check_protection(&env, &supplied).unwrap();
    check_invalidation(&env, &supplied).unwrap();
    let error = check_concurrency(&env, &supplied).unwrap_err();
    assert_eq!(error.code, "parallel-effects-unproved");
    assert_eq!(error.path, "/body/nodes/start_acceptance");
    // Only a derived raw SDK variant; no benchmark/current source/resource bytes
    // were rewritten, and no claimed exhaustive behavior was established as truth.
}

#[test]
fn same_provider_id_at_two_exact_revisions_has_distinct_consistent_selections() {
    let mut f = Fixture::new();
    f.upgrade();
    f.select();
    let original = Pin::decode(&f.b["policies"]["a_cap_provider"]["body"]["provider"]).unwrap();
    let mut provider = f
        .artifacts
        .iter()
        .find(|(p, _)| p == &original)
        .unwrap()
        .1
        .clone();
    provider.push(b' ');
    let revised = f.raw(&original.id, &provider);
    let raw = f
        .artifacts
        .iter()
        .find(|(p, _)| p == &f.descriptors[0])
        .unwrap()
        .1
        .clone();
    let mut descriptor: Value = serde_json::from_slice(&raw).unwrap();
    descriptor["provider"] = pref(&revised);
    let selected = f.artifact(&descriptor);
    f.b["policies"]["unused_contract"] = f.b["policies"]["a_cap_provider"].clone();
    f.b["policies"]["unused_contract"]["body"]["provider"] = pref(&revised);
    f.b["policies"]["unused_contract"]["body"]["interference"] = pref(&selected);
    let r = check(&f.env(), &f.supplied()).unwrap();
    assert_eq!(r.interference_bindings().len(), 3);
    assert_eq!(
        r.interference_bindings()[0].descriptor().provider().id,
        r.interference_bindings()[2].descriptor().provider().id
    );
    assert_ne!(
        r.interference_bindings()[0].descriptor().provider(),
        r.interference_bindings()[2].descriptor().provider()
    );
}
