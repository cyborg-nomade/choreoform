// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_authoring_frontend::{Package, parse_fragment};
use choreoform_ir_probe_core::DefinitionEnvelope;
use choreoform_semantic_check::{CORE, CORE_V2, CORE_V3};

fn supplied(m: &Value) -> Vec<Resource<'_>> {
    super::profile::source_resources(m)
}

fn promote(source: &[u8], bindings: &[u8]) -> (String, Vec<u8>) {
    let source = std::str::from_utf8(source).unwrap();
    let mut l: Value = serde_json::from_slice(bindings).unwrap();
    let old = l["profile"].as_str().unwrap().to_owned();
    let source = source.replacen(&format!("authoring \"{old}\""), "authoring \"0.6.0\"", 1);
    l["profile"] = "0.6.0".into();
    l["version"] = "0.5.0".into();
    l["contracts"]["core"]["contract"] = CORE_V3.0.into();
    l["contracts"]["core"]["digest"] = CORE_V3.1.into();
    (source, serde_json::to_vec(&l).unwrap())
}

#[test]
fn explicit_core_promotion_changes_revision_and_integrity_with_compatible_body_and_format() {
    let m: Value = serde_json::from_slice(MANIFEST).unwrap();
    let r = supplied(&m);
    // This compatibility comparison needs an acyclic core-0.2 package; the
    // current whole RP-01 labels intentionally require the new core.
    let legacy_source = include_str!("../../../../docs/authoring/examples/terminal.choreo")
        .replace("authoring \"0.2.0\"", "authoring \"0.5.0\"");
    let mut legacy: Value = serde_json::from_slice(include_bytes!(
        "../../../../docs/authoring/examples/terminal.bindings.json"
    ))
    .unwrap();
    legacy["profile"] = "0.5.0".into();
    legacy["version"] = "0.4.0".into();
    legacy["contracts"]["core"]["contract"] = CORE_V2.0.into();
    legacy["contracts"]["core"]["digest"] = CORE_V2.1.into();
    let legacy_bytes = serde_json::to_vec(&legacy).unwrap();
    let original = Package::parse(legacy_source.as_bytes(), &legacy_bytes).unwrap();
    let old = original.lower(&r).unwrap();
    let (s, l) = promote(legacy_source.as_bytes(), &legacy_bytes);
    let package = Package::parse(s.as_bytes(), &l).unwrap();
    assert_eq!(
        package.syntax().token_count(),
        original.syntax().token_count()
    );
    let c = package.lower(&r).unwrap();
    assert_ne!(old.document()["revision"], c.document()["revision"]);
    assert_ne!(original.integrity(), package.integrity());
    assert_eq!(c.document()["body"]["dialects"]["core"]["id"], CORE_V3.0);
    let mut before = old.document()["body"].clone();
    let mut after = c.document()["body"].clone();
    before["dialects"]["core"] = Value::Null;
    after["dialects"]["core"] = Value::Null;
    assert_eq!(before, after);
    let (text, ledger) = package.formatted().unwrap();
    assert_eq!(
        Package::parse(text.as_bytes(), ledger.as_bytes())
            .unwrap()
            .lower(&r)
            .unwrap()
            .document(),
        c.document()
    );
    let corrupt = b"corrupt core bytes";
    let supplied_corrupt: Vec<_> = r
        .iter()
        .map(|r| Resource {
            id: r.id,
            revision: r.revision,
            bytes: if r.id == CORE_V3.0 { corrupt } else { r.bytes },
        })
        .collect();
    assert_eq!(
        package.lower(&supplied_corrupt).unwrap_err().code,
        "resource-digest"
    );
    for pin in [CORE, CORE_V2, CORE_V3] {
        let supplied: Vec<_> = r
            .iter()
            .filter(|r| r.id != pin.0)
            .map(|r| Resource {
                id: r.id,
                revision: r.revision,
                bytes: r.bytes,
            })
            .collect();
        assert_eq!(
            package.lower(&supplied).unwrap_err().code,
            "resource-missing-or-ambiguous"
        );
    }
    let mut wrong: Value = serde_json::from_slice(&l).unwrap();
    wrong["version"] = "0.4.0".into();
    assert_eq!(
        Package::parse(s.as_bytes(), &serde_json::to_vec(&wrong).unwrap())
            .unwrap_err()
            .code,
        "ledger-profile"
    );
    wrong["version"] = "0.5.0".into();
    wrong["contracts"]["core"]["contract"] = CORE_V2.0.into();
    wrong["contracts"]["core"]["digest"] = CORE_V2.1.into();
    assert_eq!(
        Package::parse(s.as_bytes(), &serde_json::to_vec(&wrong).unwrap())
            .unwrap_err()
            .code,
        "contract-pin"
    );
}

#[test]
fn actual_source_dependency_derivation_is_rechecked_and_missing_reads_refuse() {
    let m: Value = serde_json::from_slice(MANIFEST).unwrap();
    let r = supplied(&m);
    let (s, l) = promote(
        include_bytes!("../../../../docs/authoring/examples/review.choreo"),
        include_bytes!("../../../../docs/authoring/examples/review.bindings.json"),
    );
    let start = s.find("Permit access when (").unwrap();
    let end = s[start..].find("End policy.").unwrap() + start;
    let s = format!(
        "{}Permit access when ((the value of \"Request\") equals (the value of \"Request\")).\n{}",
        &s[..start],
        &s[end..]
    );
    let package = Package::parse(s.as_bytes(), &l).unwrap();
    let c = package.lower(&r).unwrap();
    c.check_static(&r).unwrap();
    let ledger: Value = serde_json::from_slice(&l).unwrap();
    let id = |name: &str| {
        ledger["declarations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|d| d["name"] == name)
            .unwrap()["id"]
            .as_str()
            .unwrap()
    };
    let protection = id("Protection");
    let data = id("Request");
    let reads = c.document()["body"]["policies"][protection]["body"]["reads"]
        .as_object()
        .unwrap();
    assert_eq!(reads.len(), 1);
    assert!(reads.contains_key(data));
    let mut body = c.document()["body"].clone();
    body["policies"][protection]["body"]["reads"] = json!({});
    let e = DefinitionEnvelope::from_parts(body, json!({})).unwrap();
    assert_eq!(
        choreoform_semantic_check::check(&e, &r).unwrap_err().code,
        "policy-reads"
    );
}

#[test]
fn known_program_quotas_are_shared_without_migrating_legacy_or_fragment_limits() {
    for version in ["0.5.0", "0.6.0"] {
        let s = format!(
            "Choreoform authoring \"{version}\". {}",
            "bad ".repeat(16380)
        );
        assert_eq!(parse(s.as_bytes()).unwrap_err().code, "syntax");
        assert_eq!(
            parse_fragment(s.as_bytes(), "declaration")
                .unwrap_err()
                .code,
            "tokens"
        );
        let s = s + "bad ";
        assert_eq!(parse(s.as_bytes()).unwrap_err().code, "tokens");
    }
    let s = format!("Choreoform authoring \"0.8.0\". {}", "bad ".repeat(5000));
    assert_eq!(parse(s.as_bytes()).unwrap_err().code, "tokens");
}
