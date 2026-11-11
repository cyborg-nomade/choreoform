// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Explicit capacity only; no wider semantics or accepted host contexts.
use choreoform_authoring_frontend::{Ledger, Package, parse};
use choreoform_ir_probe_core::Resource;
use choreoform_semantic_check::CORE_V10;
use serde_json::Value;

struct Fixture {
    source: String,
    ledger: Value,
    artifacts: Vec<(String, String, Vec<u8>)>,
}
impl Fixture {
    fn small() -> Self {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let doc: Value =
            serde_json::from_slice(include_bytes!("capture/finite-work.resources.json")).unwrap();
        let artifacts = doc["normative"]
            .as_array()
            .unwrap()
            .iter()
            .chain(doc["resources"].as_array().unwrap())
            .map(|r| {
                let revision = r["revision"].as_str().unwrap();
                let bytes = r["bytes"]
                    .as_str()
                    .map(|s| s.as_bytes().to_vec())
                    .unwrap_or_else(|| {
                        std::fs::read(root.join(format!(
                            "docs/ir/contracts/{}.txt",
                            revision.replace(':', "-")
                        )))
                        .unwrap()
                    });
                (r["id"].as_str().unwrap().into(), revision.into(), bytes)
            })
            .collect();
        Self {
            source: include_str!("capture/finite-work.choreo").into(),
            ledger: serde_json::from_slice(include_bytes!("capture/finite-work.bindings.json"))
                .unwrap(),
            artifacts,
        }
    }
    fn promote(&mut self) {
        self.source = self
            .source
            .replacen("authoring \"0.14.0\"", "authoring \"0.15.0\"", 1);
        self.ledger["profile"] = "0.15.0".into();
        self.ledger["version"] = "0.14.0".into();
    }
    fn supplied(&self) -> Vec<Resource<'_>> {
        self.artifacts
            .iter()
            .map(|(id, revision, bytes)| Resource {
                id,
                revision,
                bytes,
            })
            .collect()
    }
}

#[test]
fn explicit_pair_preserves_exact_semantic_body_and_conditional_work_report() {
    let old = Fixture::small();
    let mut new = Fixture::small();
    new.promote();
    let old_ledger = serde_json::to_vec(&old.ledger).unwrap();
    let new_ledger = serde_json::to_vec(&new.ledger).unwrap();
    let a = Package::parse(old.source.as_bytes(), &old_ledger).unwrap();
    let b = Package::parse(new.source.as_bytes(), &new_ledger).unwrap();
    let x = a.lower(&old.supplied()).unwrap();
    let y = b.lower(&new.supplied()).unwrap();
    assert_eq!(x.document(), y.document());
    x.check_reconciliation_availability(&old.supplied())
        .unwrap();
    y.check_reconciliation_availability(&new.supplied())
        .unwrap();
    assert_eq!(parse(new.source.as_bytes()).unwrap().profile(), "0.15.0");
}

#[test]
fn new_pair_does_not_guess_an_old_pair_core_or_missing_inherited_contract() {
    let mut f = Fixture::small();
    f.promote();
    let mut l = f.ledger.clone();
    l["version"] = "0.13.0".into();
    assert_eq!(
        Ledger::decode(&serde_json::to_vec(&l).unwrap())
            .unwrap_err()
            .code,
        "ledger-profile"
    );
    let mut l = f.ledger.clone();
    l["contracts"]["core"]["contract"] = CORE_V10.0.into();
    l["contracts"]["core"]["digest"] = CORE_V10.1.into();
    assert_eq!(
        Ledger::decode(&serde_json::to_vec(&l).unwrap())
            .unwrap_err()
            .code,
        "contract-pin"
    );
    let bytes = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
    let supplied: Vec<_> = f
        .supplied()
        .into_iter()
        .filter(|r| r.id != CORE_V10.0)
        .collect();
    assert_eq!(
        p.lower(&supplied).unwrap_err().code,
        "resource-missing-or-ambiguous"
    );
    let future = f.source.replacen("0.15.0", "0.17.0", 1);
    assert_eq!(parse(future.as_bytes()).unwrap_err().code, "profile");
}

#[test]
fn actual_full_order_recognizes_with_old_work_limit_and_keeps_private_compensation_refusal() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let base = root.join("docs/benchmarks/rp03/history/source-0.14");
    let old = std::fs::read_to_string(base.join("source.choreo")).unwrap();
    let error = parse(old.as_bytes()).unwrap_err();
    assert_eq!(error.code, "tokens");
    assert_eq!(error.span, 108626..108627);
    let source = old.replacen("authoring \"0.14.0\"", "authoring \"0.15.0\"", 1);
    let syntax = parse(source.as_bytes()).unwrap();
    assert_eq!(syntax.token_count(), 77135);
    assert!(syntax.parser_work() <= 1_000_000);
    println!("full RP-03 parser pattern visits: {}", syntax.parser_work());
    let mut ledger: Value =
        serde_json::from_slice(&std::fs::read(base.join("bindings.json")).unwrap()).unwrap();
    ledger["profile"] = "0.15.0".into();
    ledger["version"] = "0.14.0".into();
    let mut f = Fixture::small();
    let m: Value =
        serde_json::from_slice(&std::fs::read(base.join("manifest.json")).unwrap()).unwrap();
    for r in m["resources"].as_array().unwrap() {
        let id = r["id"].as_str().unwrap();
        let revision = r["revision"].as_str().unwrap();
        if !f.artifacts.iter().any(|(i, v, _)| i == id && v == revision) {
            f.artifacts.push((
                id.into(),
                revision.into(),
                std::fs::read(root.join(r["path"].as_str().unwrap())).unwrap(),
            ));
        }
    }
    let bytes = serde_json::to_vec(&ledger).unwrap();
    let p = Package::parse(source.as_bytes(), &bytes).unwrap();
    let e = p.lower(&f.supplied()).unwrap_err();
    assert_eq!(e.code, "reference-missing");
    assert_eq!(e.span, 60937..61023);
    assert_eq!(&source.as_bytes()[e.span],b"\"Request Reserve first line\" in \"Reserve first line occurrence\" in \"Order fulfillment\"");
}

#[path = "capture/compensation-identity.rs"]
mod compensation_identity;

#[path = "capture/rp03-current-checkpoint.rs"]
mod rp03_current_checkpoint;
