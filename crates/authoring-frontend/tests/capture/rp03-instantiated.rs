// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Complete actual current IR; partial checks never imply a live instance or grant.
use super::*;
use choreoform_ir_probe_core::DefinitionEnvelope;

fn fixture() -> Fixture {
    let mut f = super::compensation_identity::current();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let base = root.join("docs/benchmarks/rp03");
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
                std::fs::read(root.join(r["path"].as_str().unwrap())).unwrap(),
            ));
        }
    }
    f
}

#[test]
fn full_current_source_emits_exact_ir_and_passes_real_source_and_raw_first_four_checks() {
    let f = fixture();
    let syntax = parse(f.source.as_bytes()).unwrap();
    assert_eq!(syntax.token_count(), 66783);
    assert_eq!(syntax.profile(), "0.18.0");
    assert_eq!(f.source.len(), 410752);
    assert!(syntax.parser_work() <= 1_000_000);
    println!("instantiated whole parser visits {}", syntax.parser_work());
    let bytes = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
    let c = p.lower(&f.supplied()).unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let golden: Value = serde_json::from_slice(
        &std::fs::read(root.join("docs/benchmarks/rp03/candidate.ir.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(c.document(), &golden);
    assert_eq!(
        std::fs::read(root.join("docs/benchmarks/rp03/candidate.ir.json")).unwrap(),
        std::fs::read(
            root.join(
                "docs/benchmarks/rp03/history/source-0.16-before-factoring/candidate.ir.json"
            )
        )
        .unwrap()
    );
    let manifest: Value = serde_json::from_slice(
        &std::fs::read(root.join("docs/benchmarks/rp03/manifest.json")).unwrap(),
    )
    .unwrap();
    let integrity = p.integrity();
    assert_eq!(integrity.source, manifest["sourceDigest"]);
    assert_eq!(integrity.bindings, manifest["bindingsDigest"]);
    assert_eq!(integrity.package, manifest["packageDigest"]);
    let (formatted, companion) = p.formatted().unwrap();
    assert_eq!(
        Package::parse(formatted.as_bytes(), companion.as_bytes())
            .unwrap()
            .lower(&f.supplied())
            .unwrap()
            .document(),
        c.document()
    );
    c.check_static(&f.supplied()).unwrap();
    c.check_control(&f.supplied()).unwrap();
    c.check_protection(&f.supplied()).unwrap();
    let raw = DefinitionEnvelope::decode(&serde_json::to_vec(c.document()).unwrap()).unwrap();
    choreoform_semantic_check::check(&raw, &f.supplied()).unwrap();
    choreoform_semantic_check::check_control(&raw, &f.supplied()).unwrap();
    choreoform_semantic_check::check_protection(&raw, &f.supplied()).unwrap();
    let report = c.check_invalidation(&f.supplied()).unwrap();
    assert_eq!(report.obligations().len(), 312);
    assert_eq!(report.historical_origins().len(), 849);
    choreoform_semantic_check::check_invalidation(&raw, &f.supplied()).unwrap();
    let e = c.check_concurrency(&f.supplied()).unwrap_err();
    assert_eq!(e.code, "parallel-effects-unproved");
    assert_eq!(e.path, "/body/nodes/start_acceptance");
    let e = choreoform_semantic_check::check_concurrency(&raw, &f.supplied()).unwrap_err();
    assert_eq!(e.code, "parallel-effects-unproved");
    assert_eq!(e.path, "/body/nodes/start_acceptance");
}

#[test]
fn every_declared_invoke_and_split_body_is_an_actual_direct_child() {
    let f = fixture();
    let bytes = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
    let c = p.lower(&f.supplied()).unwrap();
    let b = &c.document()["body"];
    let mut invokes = 0;
    let mut children = 0;
    for node in b["nodes"].as_object().unwrap().values() {
        if node["kind"] == "invoke" {
            invokes += 1;
            assert_eq!(
                b["scopes"][node["body"].as_str().unwrap()]["parent"],
                node["scope"]
            );
        }
        if node["kind"] == "split" {
            for body in node["children"].as_object().unwrap().values() {
                children += 1;
                assert_eq!(b["scopes"][body.as_str().unwrap()]["parent"], node["scope"]);
            }
        }
    }
    assert_eq!((invokes, children), (26, 7));
    assert_eq!(b["capabilities"].as_object().unwrap().len(), 25);
    assert_eq!(
        b["policies"]["cancel_release_a_work"]["body"]["compensates"],
        "reserve_a_activity"
    );
    assert_eq!(
        b["policies"]["release_a_work"]["body"]["compensates"],
        "reserve_a_activity"
    );
}
