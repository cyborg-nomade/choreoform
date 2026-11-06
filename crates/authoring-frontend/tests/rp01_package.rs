// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Whole-source candidate/resource evidence, never execution authority.
use choreoform_authoring_frontend::{Ledger, parse};
use choreoform_ir_probe_core::{Resource, digest};
use choreoform_semantic_check::{Context, Pin, Resources};
use serde_json::{Value, json};

#[path = "rp01/metadata_profile.rs"]
mod metadata_profile;
#[path = "rp01/profile.rs"]
mod profile;

const MANIFEST: &[u8] = include_bytes!("../../../docs/benchmarks/rp01/manifest.json");
const SOURCE: &[u8] = include_bytes!("../../../docs/benchmarks/rp01/source.choreo");
const BINDINGS: &[u8] = include_bytes!("../../../docs/benchmarks/rp01/bindings.json");
const ARTIFACTS: &[(&str, &[u8])] = &[
    (
        "docs/benchmarks/rp01/types.json",
        include_bytes!("../../../docs/benchmarks/rp01/types.json"),
    ),
    (
        "docs/benchmarks/rp01/checks.provider.json",
        include_bytes!("../../../docs/benchmarks/rp01/checks.provider.json"),
    ),
    (
        "docs/benchmarks/rp01/payment.provider.json",
        include_bytes!("../../../docs/benchmarks/rp01/payment.provider.json"),
    ),
    (
        "docs/benchmarks/rp01/notice.provider.json",
        include_bytes!("../../../docs/benchmarks/rp01/notice.provider.json"),
    ),
    (
        "docs/benchmarks/rp01/clock.provider.json",
        include_bytes!("../../../docs/benchmarks/rp01/clock.provider.json"),
    ),
    (
        "docs/benchmarks/rp01/clock.json",
        include_bytes!("../../../docs/benchmarks/rp01/clock.json"),
    ),
    (
        "docs/ir/contracts/sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt",
        include_bytes!(
            "../../../docs/ir/contracts/sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt"
        ),
    ),
];
fn resources(manifest: &Value) -> Vec<Resource<'_>> {
    manifest["resources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            let path = r["path"].as_str().unwrap();
            let bytes = ARTIFACTS.iter().find(|(p, _)| *p == path).unwrap().1;
            Resource {
                id: r["id"].as_str().unwrap(),
                revision: r["revision"].as_str().unwrap(),
                bytes,
            }
        })
        .collect()
}
fn pin(manifest: &Value, suffix: &str) -> Pin {
    let r = manifest["resources"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["path"].as_str().unwrap().ends_with(suffix))
        .unwrap();
    Pin {
        id: r["id"].as_str().unwrap().into(),
        revision: r["revision"].as_str().unwrap().into(),
    }
}

#[test]
fn draft_manifest_and_resource_pins_do_not_claim_program_admission() {
    let m: Value = serde_json::from_slice(MANIFEST).unwrap();
    assert_eq!(m["executable"], false);
    let ir: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/benchmarks/rp01/candidate.ir.json"
    ))
    .unwrap();
    assert_eq!(m["canonicalIR"]["revision"], ir["revision"]);
    assert_eq!(
        m["canonicalIR"]["digest"],
        digest(include_bytes!(
            "../../../docs/benchmarks/rp01/candidate.ir.json"
        ))
    );
    let stages = m["semanticStageResults"].as_array().unwrap();
    assert_eq!(stages.len(), 10);
    assert!(stages.iter().all(|s| s["result"] == "refused"
        && s["code"] == "information-flow"
        && s["path"] == "/body/nodes/checks_request_step"));
    assert_eq!(m["sourceDigest"], digest(SOURCE));
    assert_eq!(m["sourceBytes"], SOURCE.len());
    assert_eq!(m["scenarios"].as_array().unwrap().len(), 4);
    assert!(
        m["scenarios"]
            .as_array()
            .unwrap()
            .iter()
            .all(|s| s["status"] == "Partial")
    );
    Ledger::decode(BINDINGS).unwrap();
    let supplied = resources(&m);
    for r in &supplied {
        assert_eq!(r.revision, digest(r.bytes));
    }
    assert!(
        m["measuredTokens"].as_u64().unwrap() > choreoform_authoring_frontend::MAX_TOKENS as u64
    );
    assert_eq!(parse(SOURCE).unwrap().profile(), "0.5.0");
    let old = std::str::from_utf8(SOURCE)
        .unwrap()
        .replacen("\"0.5.0\"", "\"0.4.0\"", 1);
    assert_eq!(parse(old.as_bytes()).unwrap_err().code, "tokens");
}

#[test]
fn exact_synthetic_nominal_claims_check_real_invariants_without_host_facts() {
    let m: Value = serde_json::from_slice(MANIFEST).unwrap();
    let supplied = resources(&m);
    let mut r = Resources::new(&supplied).unwrap();
    let p = pin(&m, "types.json");
    r.named_types(&p).unwrap();
    let ty = json!({"kind":"named","contract":{"id":p.id,"revision":p.revision},"name":"Claim"});
    for raw in [
        include_bytes!("../../../docs/benchmarks/rp01/claim-v1.json").as_slice(),
        include_bytes!("../../../docs/benchmarks/rp01/claim-v2.json").as_slice(),
    ] {
        let claim: Value = serde_json::from_slice(raw).unwrap();
        r.expression(
            &json!({"op":"literal","type":ty,"value":claim}),
            &Context::default(),
        )
        .unwrap();
        for (field, value) in [
            ("amount", json!("-1")),
            ("currency", json!("JPY")),
            ("revision", json!("")),
        ] {
            let mut invalid = claim.clone();
            invalid[field] = value;
            let d = r
                .expression(
                    &json!({"op":"literal","type":ty,"value":invalid}),
                    &Context::default(),
                )
                .unwrap_err();
            assert_eq!(d.code, "invalid-nominal-value");
            assert!(!format!("{d:?}").contains("fictional-employee"));
        }
    }
}

#[test]
fn provider_and_clock_metadata_decode_without_asserting_operational_support() {
    let m: Value = serde_json::from_slice(MANIFEST).unwrap();
    let supplied = resources(&m);
    let mut r = Resources::new(&supplied).unwrap();
    for suffix in [
        "checks.provider.json",
        "payment.provider.json",
        "notice.provider.json",
    ] {
        let provider = r.provider(&pin(&m, suffix)).unwrap();
        assert!(!provider.outcomes().is_empty());
        assert_eq!(provider.protection()["policy"], "expense_access");
    }
    let clock = r.clock(&pin(&m, "clock.json")).unwrap();
    assert_eq!(clock.source(), &pin(&m, "clock.provider.json"));
    assert_eq!(
        clock.profile().id,
        choreoform_semantic_check::CLOCK_PROFILE.0
    );
}

#[test]
fn sample_artifact_links_are_integrity_checks_not_receipt_or_policy_authentication() {
    let claim: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/benchmarks/rp01/claim-v1.json"
    ))
    .unwrap();
    assert_eq!(
        claim["receipt"]["value"]["digest"],
        digest(include_bytes!("../../../docs/benchmarks/rp01/receipt.txt"))
    );
    assert_eq!(
        claim["policy revision"],
        digest(include_bytes!("../../../docs/benchmarks/rp01/policy.json"))
    );
    let m: Value = serde_json::from_slice(MANIFEST).unwrap();
    let supplied = resources(&m);
    let mut r = Resources::new(&supplied).unwrap();
    let p = pin(&m, "types.json");
    let mut value = claim;
    value["date"] = "not a calendar date".into();
    // The named representation promises nonempty text, not civil-date validation
    // or truthful evidence. Those are explicit check-provider/host obligations.
    r.expression(&json!({"op":"literal","type":{"kind":"named","contract":{"id":p.id,"revision":p.revision},"name":"Claim"},"value":value}), &Context::default()).unwrap();
}
