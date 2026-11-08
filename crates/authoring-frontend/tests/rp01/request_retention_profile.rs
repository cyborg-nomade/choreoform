// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_authoring_frontend::Package;
use choreoform_semantic_check::{CORE_V8, Diagnostic, TERMINAL_OBSERVATION_PROFILE};
use general_fault_profile::Fixture;
const CLAUSE: &str =
    "Retain the original request from \"Request Payment\" and record in \"Original request\".";
fn fixture() -> Fixture {
    let mut f = super::handler_entry_profile::fixture();
    f.source = f
        .source
        .replace("authoring \"0.10.0\"", "authoring \"0.11.0\"");
    f.ledger["profile"] = "0.11.0".into();
    f.ledger["version"] = "0.10.0".into();
    f.ledger["contracts"]["core"] =
        json!({"contract":CORE_V8.0,"digest":CORE_V8.1,"dialect":"core"});
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for p in [CORE_V8, TERMINAL_OBSERVATION_PROFILE] {
        f.artifacts.push((
            p.0.into(),
            p.1.into(),
            std::fs::read(root.join(format!("docs/ir/contracts/{}.txt", p.1.replace(':', "-"))))
                .unwrap(),
        ));
    }
    let raw = include_bytes!("../capture/request-retention.terminal-source.json");
    f.artifacts
        .push(("urn:test:terminal-source".into(), digest(raw), raw.to_vec()));
    let delta: Value = serde_json::from_slice(include_bytes!(
        "../capture/request-retention.bindings-fragment.json"
    ))
    .unwrap();
    for k in ["declarations", "generated", "symbols", "resources"] {
        let table = f.ledger[k].as_array_mut().unwrap();
        for row in delta[k].as_array().unwrap() {
            let key = if k == "resources" { "binding" } else { "id" };
            if let Some(old) = table.iter_mut().find(|v| v[key] == row[key]) {
                *old = row.clone();
            } else {
                table.push(row.clone());
            }
        }
    }
    let a = f
        .source
        .find("Scope \"Payment recovery\" within \"Payment\".")
        .unwrap();
    let b = a + f.source[a..].find("End scope.").unwrap() + "End scope.".len();
    f.source
        .replace_range(a..b, include_str!("../capture/request-retention.choreo"));
    f
}
#[test]
fn retained_read_copy_branch_composes_with_capture_retry_and_history_without_legacy_promotion() {
    let f = fixture();
    let c = f.lower();
    let r = f.resources();
    assert_eq!(c.document()["version"], "0.4.0");
    assert_eq!(c.check_static(&r).unwrap().retained_requests().len(), 1);
    c.check_control(&r).unwrap();
    c.check_protection(&r).unwrap();
    c.check_invalidation(&r).unwrap();
    c.invalidation_requirements(&r).unwrap();
    c.check_concurrency(&r).unwrap();
    c.check_timing(&r).unwrap();
    c.check_fault_routes(&r).unwrap();
    c.check_exceptional_availability(&r).unwrap();
    c.check_reconciliation_links(&r).unwrap();
    let final_report = c.check_reconciliation_availability(&r).unwrap();
    assert!(
        final_report
            .retained_request_scopes()
            .contains("payment_recovery")
    );
    assert!(!final_report.checked_nodes().contains("request_branch"));
    assert!(final_report.uncovered_nodes().contains("request_branch"));
    assert!(
        final_report
            .links()
            .exceptional()
            .history_entries()
            .is_some()
    );
}
#[test]
fn formatting_qualified_utf8_aliases_and_clause_spans_preserve_semantics() {
    let mut f = fixture();
    let c = f.lower();
    let bindings = serde_json::to_vec(&f.ledger).unwrap();
    let (s, l) = Package::parse(f.source.as_bytes(), &bindings)
        .unwrap()
        .formatted()
        .unwrap();
    assert_eq!(
        Package::parse(s.as_bytes(), l.as_bytes())
            .unwrap()
            .lower(&f.resources())
            .unwrap()
            .document(),
        c.document()
    );
    f.source = f
        .source
        .replace("\"Original request\"", "\"Requête d’origine\"");
    f.ledger["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|v| v["id"] == "original_request")
        .unwrap()["name"] = "Requête d’origine".into();
    let renamed = f.lower();
    assert_eq!(renamed.document(), c.document());
    assert_ne!(renamed.package_digest(), c.package_digest());
    let d = Diagnostic {
        code: "example",
        path: "/body/scopes/payment_recovery/retainedRequests/original_request".into(),
    };
    let range = renamed.source_span(&d).unwrap();
    assert_eq!(
        &f.source[range],
        CLAUSE.replace("Original request", "Requête d’origine")
    );
    f.source = f.source.replace(
        "from \"Request Payment\" and",
        "from \"Request Payment\" in \"Payment\" in \"Expense reimbursement\" and",
    );
    assert_eq!(f.lower().document(), c.document());
}
#[test]
fn exact_bases_local_ownership_and_readonly_invariants_remain_mandatory() {
    let f = fixture();
    let supplied: Vec<_> = f
        .resources()
        .into_iter()
        .filter(|r| r.id != CORE_V8.0)
        .collect();
    assert!(
        Package::parse(f.source.as_bytes(), &serde_json::to_vec(&f.ledger).unwrap())
            .unwrap()
            .lower(&supplied)
            .is_err()
    );
    let mut f = fixture();
    f.source = f.source.replace(
        "and record in \"Original request\".",
        "and record in \"Current claim\".",
    );
    let e = Package::parse(f.source.as_bytes(), &serde_json::to_vec(&f.ledger).unwrap())
        .unwrap()
        .lower(&f.resources())
        .unwrap_err();
    assert_eq!(e.code, "retained-request-owner");
    let mut f = fixture();
    f.source=f.source.replacen("Invalidate [\"Observe Payment\", \"Inspect original request\", \"Finish Payment recovery\"]. Initially unavailable.","Invalidate [\"Observe Payment\", \"Inspect original request\", \"Finish Payment recovery\"]. Initially (the value of \"Request copy\").",1);
    f.ledger["generated"].as_array_mut().unwrap().push(
        json!({"id":"forged_original","owner":"original_request","role":"initial","key":null}),
    );
    let c = f.lower();
    assert_eq!(
        c.check_static(&f.resources()).unwrap_err().code,
        "retained-request-readonly"
    );
}
