// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_authoring_frontend::{Package, parse_fragment};
use choreoform_ir_probe_core::DefinitionEnvelope;
use choreoform_semantic_check::{CORE, CORE_V2, CORE_V3, CORE_V4, FAULT_SOURCE_PROFILE, SEMANTICS};

struct Fixture {
    source: String,
    ledger: Value,
    artifacts: Vec<(String, String, Vec<u8>)>,
}
impl Fixture {
    fn new() -> Self {
        let manifest: Value = serde_json::from_slice(MANIFEST).unwrap();
        let mut f = Self {
            source: std::str::from_utf8(SOURCE)
                .unwrap()
                .replace("authoring \"0.6.0\"", "authoring \"0.7.0\"")
                .replace(
                    "basis \"Payable request\".",
                    "basis \"Payable request\", fault source \"Payment faults\".",
                ),
            ledger: serde_json::from_slice(BINDINGS).unwrap(),
            artifacts: super::profile::source_resources(&manifest)
                .iter()
                .map(|r| (r.id.into(), r.revision.into(), r.bytes.to_vec()))
                .collect(),
        };
        for (pin,raw) in [
            (CORE_V4, include_bytes!("../../../../docs/ir/contracts/sha256-0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3.txt").as_slice()),
            (FAULT_SOURCE_PROFILE, include_bytes!("../../../../docs/ir/contracts/sha256-f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0.txt").as_slice()),
        ] { if !f.artifacts.iter().any(|(id,rev,_)|id==pin.0 && rev==pin.1) { f.artifacts.push((pin.0.into(),pin.1.into(),raw.to_vec())); } }
        f.ledger["profile"] = "0.7.0".into();
        f.ledger["version"] = "0.6.0".into();
        f.ledger["contracts"]["core"] =
            json!({"contract":CORE_V4.0,"digest":CORE_V4.1,"dialect":"core"});
        let provider = manifest["resources"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["id"] == "urn:choreoform:benchmarks:rp01:payment:0.1.0")
            .unwrap();
        let artifact = json!({"kind":"provider-fault-source","version":"0.1.0","id":"urn:test:source-faults","profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":{"id":provider["id"],"revision":provider["revision"]},"emissions":{"confirmed no effect":{"observations":{"no-effect":true},"evidence":"definitive-no-effect"},"provider unknown":{"observations":{"unknown":true},"evidence":"unresolved"}}});
        let raw = serde_json::to_vec(&artifact).unwrap();
        let rev = digest(&raw);
        f.artifacts
            .push(("urn:test:source-faults".into(), rev.clone(), raw));
        f.ledger["declarations"]
            .as_array_mut()
            .unwrap()
            .retain(|d| d["id"] != "payment_faults_binding");
        f.ledger["resources"]
            .as_array_mut()
            .unwrap()
            .retain(|r| r["binding"] != "payment_faults_binding");
        f.ledger["declarations"].as_array_mut().unwrap().push(json!({"id":"payment_faults_binding","kind":"resource","scope":"expense","name":"Payment faults"}));
        f.ledger["resources"].as_array_mut().unwrap().push(json!({"binding":"payment_faults_binding","kind":"provider-fault-source","contract":"urn:test:source-faults","digest":rev}));
        f
    }
    fn resources(&self) -> Vec<Resource<'_>> {
        self.artifacts
            .iter()
            .map(|(id, revision, raw)| Resource {
                id,
                revision,
                bytes: raw,
            })
            .collect()
    }
    fn package(&self) -> Package<'_> {
        Package::parse(
            self.source.as_bytes(),
            &serde_json::to_vec(&self.ledger).unwrap(),
        )
        .unwrap()
    }
}

#[test]
fn full_explicit_source_lowers_checks_and_formats_without_changing_old_package() {
    let f = Fixture::new();
    let r = f.resources();
    let p = f.package();
    let c = p.lower(&r).unwrap();
    let report = c.check_reconciliation_availability(&r).unwrap();
    let binding = report
        .links()
        .exceptional()
        .faults()
        .timing()
        .retry_uses()
        .first()
        .unwrap()
        .fault_source()
        .unwrap();
    assert_eq!(binding.site(), "payment_request_step");
    assert_eq!(binding.profile().revision, FAULT_SOURCE_PROFILE.1);
    assert_eq!(c.document()["body"]["dialects"]["core"]["id"], CORE_V4.0);
    let old: Value = serde_json::from_slice(include_bytes!(
        "../../../../docs/benchmarks/rp01/candidate.ir.json"
    ))
    .unwrap();
    assert_ne!(c.document()["revision"], old["revision"]);
    for (id, policy) in c.document()["body"]["policies"].as_object().unwrap() {
        if policy["body"]["kind"] == "work" {
            assert_eq!(
                policy["body"]["retry"]["faultSource"].is_null(),
                id != "payment_work"
            );
        }
    }
    let (s, l) = p.formatted().unwrap();
    let again = Package::parse(s.as_bytes(), l.as_bytes()).unwrap();
    assert_eq!(again.lower(&r).unwrap().document(), c.document());
    let mut alias = Fixture::new();
    alias.source = alias
        .source
        .replace("\"Payment faults\"", "\"Payment observations\"");
    alias.ledger["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|d| d["id"] == "payment_faults_binding")
        .unwrap()["name"] = "Payment observations".into();
    assert_eq!(
        alias
            .package()
            .lower(&alias.resources())
            .unwrap()
            .document(),
        c.document()
    );
    assert_ne!(alias.package().integrity(), p.integrity());
}

#[test]
fn all_inherited_and_source_resource_bytes_are_required_and_verified() {
    let f = Fixture::new();
    let p = f.package();
    for id in [
        SEMANTICS.0,
        CORE.0,
        CORE_V2.0,
        CORE_V3.0,
        CORE_V4.0,
        FAULT_SOURCE_PROFILE.0,
        "urn:test:source-faults",
        "urn:choreoform:benchmarks:rp01:payment:0.1.0",
        "urn:choreoform:benchmarks:rp01:types:0.1.0",
    ] {
        let r: Vec<_> = f.resources().into_iter().filter(|r| r.id != id).collect();
        assert!(
            p.lower(&r)
                .unwrap_err()
                .code
                .starts_with("resource-missing"),
            "{id}"
        );
    }
    for id in [
        CORE_V4.0,
        CORE_V3.0,
        FAULT_SOURCE_PROFILE.0,
        "urn:test:source-faults",
    ] {
        let r: Vec<_> = f
            .resources()
            .into_iter()
            .map(|r| Resource {
                bytes: if r.id == id { b"corrupt" } else { r.bytes },
                ..r
            })
            .collect();
        assert_eq!(p.lower(&r).unwrap_err().code, "resource-digest");
    }
}

#[test]
fn strict_pairs_resource_kinds_and_versioned_retry_sentences_refuse() {
    let mut f = Fixture::new();
    f.ledger["version"] = "0.5.0".into();
    assert_eq!(
        Package::parse(f.source.as_bytes(), &serde_json::to_vec(&f.ledger).unwrap())
            .unwrap_err()
            .code,
        "ledger-profile"
    );
    let mut f = Fixture::new();
    f.ledger["contracts"]["core"]["digest"] = CORE_V3.1.into();
    assert_eq!(
        Package::parse(f.source.as_bytes(), &serde_json::to_vec(&f.ledger).unwrap())
            .unwrap_err()
            .code,
        "contract-pin"
    );
    let f = Fixture::new();
    let missing = f.source.replace(", fault source \"Payment faults\"", "");
    assert_eq!(parse(missing.as_bytes()).unwrap_err().code, "syntax");
    for version in ["0.3.0", "0.4.0", "0.5.0", "0.6.0"] {
        assert_eq!(
            parse(
                f.source
                    .replace("authoring \"0.7.0\"", &format!("authoring \"{version}\""))
                    .as_bytes()
            )
            .unwrap_err()
            .code,
            if matches!(version, "0.3.0" | "0.4.0") {
                "tokens"
            } else {
                "syntax"
            }
        );
    }
    let mut legacy: Value = serde_json::from_slice(BINDINGS).unwrap();
    legacy["profile"] = "0.6.0".into();
    legacy["version"] = "0.5.0".into();
    legacy["contracts"]["core"] = json!({"contract":CORE_V3.0,"digest":CORE_V3.1,"dialect":"core"});
    legacy["declarations"]
        .as_array_mut()
        .unwrap()
        .retain(|d| d["id"] != "payment_faults_binding");
    legacy["resources"]
        .as_array_mut()
        .unwrap()
        .retain(|r| r["binding"] != "payment_faults_binding");
    legacy["declarations"].as_array_mut().unwrap().push(
        f.ledger["declarations"]
            .as_array()
            .unwrap()
            .last()
            .unwrap()
            .clone(),
    );
    legacy["resources"].as_array_mut().unwrap().push(
        f.ledger["resources"]
            .as_array()
            .unwrap()
            .last()
            .unwrap()
            .clone(),
    );
    assert_eq!(
        Ledger::decode(&serde_json::to_vec(&legacy).unwrap())
            .unwrap_err()
            .code,
        "resource-kind"
    );
}

#[test]
fn known_header_quota_keeps_fragments_and_unknown_headers_bounded() {
    let s = format!("Choreoform authoring \"0.7.0\". {}", "bad ".repeat(16380));
    assert_eq!(parse(s.as_bytes()).unwrap_err().code, "syntax");
    assert_eq!(
        parse_fragment(s.as_bytes(), "declaration")
            .unwrap_err()
            .code,
        "tokens"
    );
    assert_eq!(parse((s + "bad ").as_bytes()).unwrap_err().code, "tokens");
    let s = format!("Choreoform authoring \"0.8.0\". {}", "bad ".repeat(5000));
    assert_eq!(parse(s.as_bytes()).unwrap_err().code, "tokens");
}

#[test]
fn generated_source_metadata_reads_remain_semantically_required() {
    let f = Fixture::new();
    let r = f.resources();
    let c = f.package().lower(&r).unwrap();
    let mut b = c.document()["body"].clone();
    // Actual retry basis, clock/provider protection, actor and completion references
    // have all been derived; removing the computed set never confers trust.
    assert!(
        !b["policies"]["payment_work"]["body"]["reads"]
            .as_object()
            .unwrap()
            .is_empty()
    );
    b["policies"]["payment_work"]["body"]["reads"] = json!({});
    let e = DefinitionEnvelope::from_parts(b, json!({})).unwrap();
    let error = choreoform_semantic_check::check(&e, &r).unwrap_err();
    assert_eq!(error.code, "policy-reads");
}
