// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Readable exact identity, never full-case semantic admission or live effects.
use super::*;
use choreoform_semantic_check::{CORE_V11, CORE_V12};
use serde_json::json;

pub(super) fn current() -> Fixture {
    let mut f = Fixture::small();
    f.promote();
    f.source = f
        .source
        .replacen("authoring \"0.15.0\"", "authoring \"0.16.0\"", 1);
    f.ledger["profile"] = "0.16.0".into();
    f.ledger["version"] = "0.15.0".into();
    f.ledger["contracts"]["core"]["contract"] = CORE_V12.0.into();
    f.ledger["contracts"]["core"]["digest"] = CORE_V12.1.into();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    f.artifacts.push((
        CORE_V12.0.into(),
        CORE_V12.1.into(),
        std::fs::read(root.join(format!(
            "docs/ir/contracts/{}.txt",
            CORE_V12.1.replace(':', "-")
        )))
        .unwrap(),
    ));
    f
}
fn sibling(target: &str) -> Fixture {
    let mut f = current();
    f.source=f.source.replace("End process.",&format!("Scope \"Audit\" within \"Main\".\nStart at \"Audit done\".\nInputs empty. Outputs empty. Outcomes [\"complete\"].\nUse cancellation \"Cancel\", faults \"Faults\", closure \"Close\" and races \"Race\".\nRule for work \"Audit work\". Instructions \"Synthetic declaration\". Authority \"Operator\". Attempts 1. Delays empty. Retry faults empty. Timeout none. Complete when (true). Compensates {target}. End policy.\nStep \"Audit done\". Finish as \"complete\". End step.\nEnd scope.\nEnd process."));
    for (id, kind, scope, name) in [
        ("audit", "scope", "main", "Audit"),
        ("audit_done", "node", "audit", "Audit done"),
        ("audit_work", "policy", "audit", "Audit work"),
    ] {
        f.ledger["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"kind":kind,"scope":scope,"name":name}));
    }
    f.ledger["symbols"].as_array_mut().unwrap().push(json!({"id":"audit_outcome_complete","owner":"audit","role":"outcome","name":"complete","wire":"complete"}));
    f
}
const PRIOR: &str = "\"Create payable\" in \"Root\" in \"Main\"";

#[test]
fn exact_sibling_identity_passes_static_and_conditional_reports_and_reparses() {
    let f = sibling(PRIOR);
    let bytes = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
    let c = p.lower(&f.supplied()).unwrap();
    assert_eq!(
        c.document()["body"]["policies"]["audit_work"]["body"]["compensates"],
        "origin"
    );
    c.check_static(&f.supplied()).unwrap();
    c.check_reconciliation_availability(&f.supplied()).unwrap();
    let (source, ledger) = p.formatted().unwrap();
    let q = Package::parse(source.as_bytes(), ledger.as_bytes()).unwrap();
    assert_eq!(q.lower(&f.supplied()).unwrap().document(), c.document());
    assert_ne!(p.integrity().source, q.integrity().source);
}

#[test]
fn failed_paths_and_nonactivity_nodes_keep_exact_reference_spans() {
    for (reference, code) in [
        ("\"Create payable\"", "reference-missing"),
        ("\"Create payable\" in \"Root\"", "reference-qualification"),
        ("\"Origin value\" in \"Root\" in \"Main\"", "reference-kind"),
        ("\"Main done\" in \"Main\"", "compensation-target"),
    ] {
        let f = sibling(reference);
        let bytes = serde_json::to_vec(&f.ledger).unwrap();
        let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
        let e = p.lower(&f.supplied()).unwrap_err();
        assert_eq!(e.code, code);
        assert_eq!(&f.source.as_bytes()[e.span], reference.as_bytes());
    }
}

#[test]
fn explicit_pair_core_every_base_and_future_header_are_required() {
    let f = current();
    for change in ["pair", "core"] {
        let mut l = f.ledger.clone();
        if change == "pair" {
            l["version"] = "0.14.0".into();
        } else {
            l["contracts"]["core"]["contract"] = CORE_V11.0.into();
            l["contracts"]["core"]["digest"] = CORE_V11.1.into();
        }
        assert_eq!(
            Ledger::decode(&serde_json::to_vec(&l).unwrap())
                .unwrap_err()
                .code,
            if change == "pair" {
                "ledger-profile"
            } else {
                "contract-pin"
            }
        );
    }
    let bytes = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
    for (id, _, _) in f
        .artifacts
        .iter()
        .filter(|(id, _, _)| id.contains("core-profile:"))
    {
        let resources: Vec<_> = f.supplied().into_iter().filter(|r| r.id != id).collect();
        assert_eq!(
            p.lower(&resources).unwrap_err().code,
            "resource-missing-or-ambiguous",
            "{id}"
        );
    }
    assert_eq!(
        parse(f.source.replacen("0.16.0", "0.20.0", 1).as_bytes())
            .unwrap_err()
            .code,
        "profile"
    );
}

#[test]
fn complete_order_uses_the_real_new_frontend_then_refuses_a_later_binding() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let base = root.join("docs/benchmarks/rp03/history/source-0.14");
    let mut f = current();
    f.source = std::fs::read_to_string(base.join("source.choreo"))
        .unwrap()
        .replacen("authoring \"0.14.0\"", "authoring \"0.16.0\"", 1);
    let mut ledger: Value =
        serde_json::from_slice(&std::fs::read(base.join("bindings.json")).unwrap()).unwrap();
    ledger["profile"] = "0.16.0".into();
    ledger["version"] = "0.15.0".into();
    ledger["contracts"]["core"] = f.ledger["contracts"]["core"].clone();
    f.ledger = ledger;
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(base.join("manifest.json")).unwrap()).unwrap();
    for r in manifest["resources"].as_array().unwrap() {
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
    let syntax = parse(f.source.as_bytes()).unwrap();
    assert_eq!(syntax.token_count(), 77135);
    println!(
        "full source0.16 parser pattern visits: {}",
        syntax.parser_work()
    );
    assert!(syntax.parser_work() <= 1_000_000);
    let bytes = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
    let e = p.lower(&f.supplied()).unwrap_err();
    assert_eq!(e.code, "symbol-mismatch");
    assert_eq!(e.span, 0..0);
}
