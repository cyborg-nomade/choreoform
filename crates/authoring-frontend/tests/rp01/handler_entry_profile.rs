// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_authoring_frontend::Package;
use choreoform_semantic_check::{CORE_V7, FAULT_HISTORY_PROFILE, FAULT_HISTORY_TYPES};
use general_fault_profile::Fixture;
const CLAUSE: &str =
    "Receive fault history from \"Payment history\" and record in \"Fault history\".";
pub(super) fn fixture() -> Fixture {
    let mut f = Fixture::new();
    f.source = f
        .source
        .replace("authoring \"0.9.0\"", "authoring \"0.10.0\"");
    f.ledger["profile"] = "0.10.0".into();
    f.ledger["version"] = "0.9.0".into();
    f.ledger["contracts"]["core"] =
        json!({"contract":CORE_V7.0,"digest":CORE_V7.1,"dialect":"core"});
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for p in [CORE_V7, FAULT_HISTORY_PROFILE, FAULT_HISTORY_TYPES] {
        f.artifacts.push((
            p.0.into(),
            p.1.into(),
            std::fs::read(root.join(format!("docs/ir/contracts/{}.txt", p.1.replace(':', "-"))))
                .unwrap(),
        ));
    }
    let raw = include_bytes!("../capture/payment.history.json");
    f.artifacts.push((
        "urn:test:readable-payment-history".into(),
        digest(raw),
        raw.to_vec(),
    ));
    let delta: Value = serde_json::from_slice(include_bytes!(
        "../capture/handler_entry.bindings-fragment.json"
    ))
    .unwrap();
    for k in ["declarations", "generated", "symbols", "resources"] {
        f.ledger[k]
            .as_array_mut()
            .unwrap()
            .extend(delta[k].as_array().unwrap().iter().cloned());
    }
    let start = f
        .source
        .find("Scope \"Payment\" within \"Expense reimbursement\".")
        .unwrap();
    let end = start
        + f.source[start..]
            .find("Scope \"Notice\" within \"Expense reimbursement\".")
            .unwrap();
    let mut block = f.source[start..end].replacen(
        "faults \"Unhandled faults\"",
        "faults \"Payment fault handling\"",
        1,
    );
    block.insert_str(
        block.rfind("End scope.").unwrap(),
        include_str!("../capture/handler_entry.choreo"),
    );
    f.source.replace_range(start..end, &block);
    f
}
#[test]
fn readable_history_stays_separate_through_every_pipeline_stage() {
    let f = fixture();
    let c = f.lower();
    let r = f.resources();
    assert_eq!(c.document()["version"], "0.3.0");
    assert_eq!(
        c.document()["body"]["scopes"]["provider_handler"]["faultInput"]["result"],
        "fault_history"
    );
    assert!(c.document()["body"]["scopes"]["expense"]["faultInput"].is_null());
    assert_eq!(c.check_static(&r).unwrap().fault_entries().len(), 1);
    c.check_control(&r).unwrap();
    c.check_protection(&r).unwrap();
    c.check_invalidation(&r).unwrap();
    c.invalidation_requirements(&r).unwrap();
    c.check_concurrency(&r).unwrap();
    c.check_timing(&r).unwrap();
    c.check_fault_routes(&r).unwrap();
    let exceptional = c.check_exceptional_availability(&r).unwrap();
    assert!(
        exceptional
            .history_entries()
            .unwrap()
            .checked_nodes()
            .contains("history_branch")
    );
    assert!(!exceptional.checked_nodes().contains("history_branch"));
    assert!(exceptional.uncovered_nodes().contains("history_branch"));
    c.check_reconciliation_links(&r).unwrap();
    assert!(
        c.check_reconciliation_availability(&r)
            .unwrap()
            .links()
            .exceptional()
            .history_entries()
            .is_some()
    );
}
#[test]
fn formatting_aliases_and_utf8_clause_diagnostics_preserve_identity() {
    let mut f = fixture();
    let c = f.lower();
    let bindings = serde_json::to_vec(&f.ledger).unwrap();
    let (source, bindings) = Package::parse(f.source.as_bytes(), &bindings)
        .unwrap()
        .formatted()
        .unwrap();
    assert_eq!(
        Package::parse(source.as_bytes(), bindings.as_bytes())
            .unwrap()
            .lower(&f.resources())
            .unwrap()
            .document(),
        c.document()
    );
    f.source = f
        .source
        .replace("\"Payment history\"", "\"Historique été\"");
    f.ledger["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|v| v["id"] == "payment_history_binding")
        .unwrap()["name"] = "Historique été".into();
    let renamed = f.lower();
    assert_eq!(renamed.document(), c.document());
    assert_ne!(renamed.package_digest(), c.package_digest());
    // A visible ancestor result is still rejected as an entry-local target.
    let renamed_source = f.source.clone();
    f.source = f.source.replace(
        "and record in \"Fault history\".",
        "and record in \"Current claim\".",
    );
    let result = Package::parse(f.source.as_bytes(), &serde_json::to_vec(&f.ledger).unwrap())
        .unwrap()
        .lower(&f.resources())
        .unwrap_err();
    assert_eq!(result.code, "fault-entry-owner");
    // Semantic diagnostics map to the clause itself, using UTF-8 byte offsets.
    let diagnostic = choreoform_semantic_check::Diagnostic {
        code: "test",
        path: "/body/scopes/provider_handler/faultInput/history".into(),
    };
    let span = renamed.source_span(&diagnostic).unwrap();
    assert_eq!(
        &renamed_source[span],
        CLAUSE.replace("Payment history", "Historique été")
    );
}
#[test]
fn libraries_require_every_exact_base_and_unused_history_resource() {
    let f = fixture();
    for index in 0..f.artifacts.len() {
        let artifact = &f.artifacts[index];
        if artifact.0.contains("core-profile")
            || artifact.0 == FAULT_HISTORY_PROFILE.0
            || artifact.0 == FAULT_HISTORY_TYPES.0
            || artifact.0 == "urn:test:readable-payment-history"
        {
            let supplied: Vec<_> = f
                .resources()
                .into_iter()
                .filter(|r| r.id != artifact.0)
                .collect();
            assert!(
                Package::parse(f.source.as_bytes(), &serde_json::to_vec(&f.ledger).unwrap())
                    .unwrap()
                    .lower(&supplied)
                    .is_err(),
                "{}",
                artifact.0
            );
        }
    }
    let mut f = fixture();
    f.source = f.source.replace(CLAUSE, "");
    f.artifacts
        .iter_mut()
        .find(|r| r.0 == "urn:test:readable-payment-history")
        .unwrap()
        .2
        .push(b' ');
    let e = Package::parse(f.source.as_bytes(), &serde_json::to_vec(&f.ledger).unwrap())
        .unwrap()
        .lower(&f.resources())
        .unwrap_err();
    assert_eq!(e.code, "resource-digest");
}
