// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Whole corrected source via real SDK; no candidate, admission or live effects.
use super::*;

fn fixture() -> Fixture {
    let mut f = super::compensation_identity::current();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let base = root.join("docs/benchmarks/rp03/history/source-0.16-pre-instances");
    f.source = std::fs::read_to_string(base.join("source.choreo")).unwrap();
    f.ledger = serde_json::from_slice(&std::fs::read(base.join("bindings.json")).unwrap()).unwrap();
    let m: Value =
        serde_json::from_slice(&std::fs::read(base.join("manifest.json")).unwrap()).unwrap();
    for r in m["resources"].as_array().unwrap() {
        let id = r["id"].as_str().unwrap();
        let revision = r["revision"].as_str().unwrap();
        if !f.artifacts.iter().any(|(i, v, _)| i == id && v == revision) {
            f.artifacts.push((
                id.into(),
                revision.into(),
                std::fs::read({
                    let path = r["path"].as_str().unwrap();
                    path.strip_prefix("docs/benchmarks/rp03/")
                        .map_or_else(|| root.join(path), |suffix| base.join(suffix))
                })
                .unwrap(),
            ));
        }
    }
    f
}

#[test]
fn corrected_whole_current_source_recognizes_then_refuses_child_template() {
    let f = fixture();
    let syntax = parse(f.source.as_bytes()).unwrap();
    assert_eq!(syntax.profile(), "0.16.0");
    assert_eq!(syntax.token_count(), 77135);
    assert_eq!(syntax.parser_work(), 832138);
    let bytes = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
    let e = p.lower(&f.supplied()).unwrap_err();
    assert_eq!(e.code, "child-template");
    assert_eq!(e.span, 0..0);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let m: Value = serde_json::from_slice(
        &std::fs::read(
            root.join("docs/benchmarks/rp03/history/source-0.16-pre-instances/manifest.json"),
        )
        .unwrap(),
    )
    .unwrap();
    let integrity = p.integrity();
    assert_eq!(integrity.source, m["sourceDigest"]);
    assert_eq!(integrity.bindings, m["bindingsDigest"]);
    assert_eq!(integrity.package, m["packageDigest"]);
    assert!(m["canonicalIR"].is_null());
    assert_eq!(m["semanticStageResults"], serde_json::json!([]));
}

#[test]
fn only_current_pair_core_and_the_one_declared_finish_caption_differ() {
    let f = fixture();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let history = root.join("docs/benchmarks/rp03/history/source-0.14");
    let original = std::fs::read_to_string(history.join("source.choreo")).unwrap();
    let expected = original
        .replacen("authoring \"0.14.0\"", "authoring \"0.16.0\"", 1)
        .replacen(
            "Step \"Finish authorization\". Finish as \"authorization recorded\".",
            "Step \"Finish authorization\". Finish as \"authorization-recorded\".",
            1,
        );
    assert_eq!(expected, f.source);
    let mut ledger: Value =
        serde_json::from_slice(&std::fs::read(history.join("bindings.json")).unwrap()).unwrap();
    ledger["profile"] = "0.16.0".into();
    ledger["version"] = "0.15.0".into();
    ledger["contracts"]["core"] = f.ledger["contracts"]["core"].clone();
    assert_eq!(ledger, f.ledger);
}
