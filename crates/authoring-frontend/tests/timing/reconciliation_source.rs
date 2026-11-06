// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_authoring_frontend::Ledger;
use choreoform_semantic_check::CORE_V2;

const SOURCE: &str = include_str!("../../../../docs/authoring/examples/reconciliation.choreo");
fn ledger() -> Value {
    serde_json::from_slice(include_bytes!(
        "../../../../docs/authoring/examples/reconciliation.bindings.json"
    ))
    .unwrap()
}
fn fixture() -> Fixture {
    let mut f = Fixture::new(true);
    f.artifact(CORE_V2.0, include_bytes!("../../../../docs/ir/contracts/sha256-5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623.txt").to_vec());
    f
}
fn candidate(
    f: &Fixture,
    source: &str,
    l: &Value,
) -> choreoform_authoring_frontend::Result<choreoform_authoring_frontend::Candidate> {
    Package::parse(source.as_bytes(), &serde_json::to_vec(l).unwrap())?.lower(&f.resources())
}

#[test]
fn owned_source_roundtrips_exact_links_ids_and_conditional_host_requirements() {
    let f = fixture();
    let l = ledger();
    let raw = serde_json::to_vec(&l).unwrap();
    let package = Package::parse(SOURCE.as_bytes(), &raw).unwrap();
    assert_eq!(package.syntax().profile(), "0.4.0");
    let c = package.lower(&f.resources()).unwrap();
    let report = c.check_reconciliation_links(&f.resources()).unwrap();
    let use_site = report.uses().first().unwrap();
    assert_eq!(use_site.site(), "service_step");
    assert_eq!(use_site.followup(), "recovery");
    assert_eq!(use_site.wait_node(), "service_observe");
    assert!(
        use_site
            .host_requirements()
            .contains(&"exact-effect-request-input-and-observation-correlation")
    );
    let (source, companion) = package.formatted().unwrap();
    let next = Package::parse(source.as_bytes(), companion.as_bytes())
        .unwrap()
        .lower(&f.resources())
        .unwrap();
    assert_eq!(c.document(), next.document());
    assert_eq!(
        report.uses(),
        next.check_reconciliation_links(&f.resources())
            .unwrap()
            .uses()
    );
    let renamed = SOURCE
        .replace("Observe service", "Observe result")
        .replace("\"observed\"", "\"received\"");
    let mut renamed_l = l;
    for row in renamed_l["declarations"].as_array_mut().unwrap() {
        if row["id"] == "service_observe" {
            row["name"] = "Observe result".into();
        }
    }
    for row in renamed_l["symbols"].as_array_mut().unwrap() {
        if row["owner"] == "service_observe" {
            row["name"] = "received".into();
        }
    }
    assert_eq!(
        candidate(&f, &renamed, &renamed_l).unwrap().document(),
        c.document()
    );
}

#[test]
fn endpoint_binding_is_independent_of_policy_and_child_declaration_order() {
    let f = fixture();
    let block = "  Rule for closure \"Close\".\n    Reconcile in \"Recovery\".\n  End policy.\n";
    assert!(SOURCE.contains(block));
    let source = SOURCE
        .replace(block, "")
        .replace("End process.", &format!("{block}End process."));
    assert_eq!(
        candidate(&f, &source, &ledger()).unwrap().document(),
        candidate(&f, SOURCE, &ledger()).unwrap().document()
    );
}

#[test]
fn designated_boundary_node_kind_policy_and_ordinary_qualifiers_are_checked() {
    let f = fixture();
    for (from, to, code) in [
        (
            "Reconcile in \"Recovery\".",
            "Fully terminal.",
            "wait-endpoint-boundary",
        ),
        (
            "On observation outcome \"observed\" of \"Observe service\" in \"Recovery\".",
            "On observation outcome \"complete\" of \"Done\" in \"Recovery\".",
            "reference-qualification",
        ),
        (
            "On observation outcome \"observed\" of \"Observe service\" in \"Recovery\".",
            "On observation outcome \"complete\" of \"Recovery done\" in \"Recovery\".",
            "symbol-mismatch",
        ),
        (
            "On observation outcome \"observed\" of \"Observe service\" in \"Recovery\".",
            "On observation outcome \"observed\" of \"Observe service\".",
            "reference-missing",
        ),
        (
            "On observation outcome \"observed\" of \"Observe service\" in \"Recovery\".",
            "On observation outcome \"observed\" of \"Request\" in \"Recovery\".",
            "reference-kind",
        ),
    ] {
        let source = SOURCE.replace(from, to);
        let e = candidate(&f, &source, &ledger()).unwrap_err();
        assert_eq!(e.code, code, "{to}: {e:?}");
        assert!(!e.span.is_empty());
    }
    let source = SOURCE.replace("Step \"Observe service\". Wait under \"Service reconciliation\". On \"observed\", continue with \"Recovery done\". End step.", "Step \"Observe service\". Finish as \"complete\". End step.");
    assert_eq!(
        candidate(&f, &source, &ledger()).unwrap_err().code,
        "wait-endpoint-owner"
    );
    // An unused second policy cannot borrow another policy's actual wait site.
    let mut l = ledger();
    l["declarations"].as_array_mut().unwrap().push(
        json!({"id":"other_wait","kind":"policy","scope":"scope_service","name":"Other wait"}),
    );
    let policy = SOURCE
        .split("  Rule for wait \"Service reconciliation\".")
        .nth(1)
        .unwrap()
        .split("  End policy.")
        .next()
        .unwrap();
    let source = SOURCE.replace(
        "End process.",
        &format!("Rule for wait \"Other wait\".{policy} End policy. End process."),
    );
    let e = candidate(&f, &source, &l).unwrap_err();
    assert_eq!(e.code, "wait-endpoint-owner");
}

#[test]
fn ordinary_data_and_general_outcome_references_keep_child_privacy() {
    let f = fixture();
    let source = SOURCE.replace(
        "Let \"Current request\" mean (the value of \"Request\")",
        "Let \"Current request\" mean (the value of \"Request\" in \"Recovery\")",
    );
    assert_eq!(
        candidate(&f, &source, &ledger()).unwrap_err().code,
        "reference-qualification"
    );
    let source = SOURCE.replace(
        "outcome \"approved\" of \"Review\"",
        "outcome \"observed\" of \"Observe service\" in \"Recovery\"",
    );
    assert_eq!(
        candidate(&f, &source, &ledger()).unwrap_err().code,
        "reference-missing"
    );
    let source = SOURCE.replace("Step \"Recovery done\".", "Keep \"Secret\" as text under \"Protection\". Sensitivity \"internal\". Purposes [\"Private\"]. Participants empty. Capabilities empty. Invalidate empty. Initially unavailable. End data. Step \"Recovery done\".")
        .replace("Let \"Current request\" mean (the value of \"Request\")", "Let \"Current request\" mean (the value of \"Secret\" in \"Recovery\")");
    let mut l = ledger();
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"secret","kind":"data","scope":"recovery","name":"Secret"}));
    l["symbols"].as_array_mut().unwrap().push(json!({"id":"secret_purpose","owner":"secret","role":"purpose","name":"Private","wire":"private"}));
    assert_eq!(
        candidate(&f, &source, &l).unwrap_err().code,
        "reference-missing"
    );
}

#[test]
fn observation_and_timer_share_the_exact_exported_wait_owner() {
    let f = fixture();
    let mut l = ledger();
    l["declarations"].as_array_mut().unwrap().push(json!({"id":"clock_binding","kind":"resource","scope":"scope_service","name":"Elapsed clock"}));
    l["resources"].as_array_mut().unwrap().push(json!({"binding":"clock_binding","kind":"clock","contract":f.clock.id,"digest":f.clock.revision}));
    l["symbols"].as_array_mut().unwrap().push(json!({"id":"expired","owner":"service_observe","role":"outcome","name":"expired","wire":"expired"}));
    l["generated"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"expired_flow","owner":"service_observe","role":"flow","key":"expired"}));
    let source = SOURCE.replace("Timer none.", "Timer clock \"Elapsed clock\", basis \"Audit basis\", due (10), calendar none, pause authority \"Reviewer\".")
        .replace("On timer none.", "On timer outcome \"expired\" of \"Observe service\" in \"Recovery\".")
        .replace("On \"observed\", continue with \"Recovery done\".", "On \"observed\", continue with \"Recovery done\". On \"expired\", continue with \"Recovery done\".");
    let c = candidate(&f, &source, &l).unwrap();
    let p = &c.document()["body"]["policies"]["service_reconciliation"]["body"];
    assert_eq!(p["onObservation"], "observed");
    assert_eq!(p["onTimer"], "expired");
    // The follow-up is not invented as an ordinary timing/data execution context.
    let report = c.check_reconciliation_links(&f.resources()).unwrap();
    assert_eq!(report.uses().len(), 1);
    let source = source.replace(
        "On timer outcome \"expired\" of \"Observe service\" in \"Recovery\".",
        "On timer outcome \"approved\" of \"Review\".",
    );
    assert_eq!(
        candidate(&f, &source, &l).unwrap_err().code,
        "wait-outcome-owner"
    );
}

#[test]
fn new_version_is_strict_and_legacy_child_endpoint_refusals_remain() {
    let f = fixture();
    for (profile, version) in [("0.2.0", "0.1.0"), ("0.3.0", "0.2.0")] {
        let source = SOURCE.replace("\"0.4.0\"", &format!("\"{profile}\""));
        let mut l = ledger();
        l["profile"] = profile.into();
        l["version"] = version.into();
        if profile == "0.2.0" {
            l["contracts"]["core"]["contract"] = CORE.0.into();
            l["contracts"]["core"]["digest"] = CORE.1.into();
        }
        assert_eq!(
            candidate(&f, &source, &l).unwrap_err().code,
            "reference-missing"
        );
    }
    for (mutation, code) in [
        ("version", "ledger-profile"),
        ("pin", "contract-pin"),
        ("source", "package-profile"),
    ] {
        let mut l = ledger();
        match mutation {
            "version" => l["version"] = "0.2.0".into(),
            "pin" => l["contracts"]["core"]["digest"] = CORE.1.into(),
            _ => {
                l["profile"] = "0.3.0".into();
                l["version"] = "0.2.0".into();
            }
        }
        assert_eq!(candidate(&f, SOURCE, &l).unwrap_err().code, code);
    }
    let l = ledger();
    let p = Package::parse(SOURCE.as_bytes(), &serde_json::to_vec(&l).unwrap()).unwrap();
    let mut missing = f.resources();
    missing.retain(|r| r.id != CORE.0);
    assert_eq!(
        p.lower(&missing).unwrap_err().code,
        "resource-missing-or-ambiguous"
    );
    let mut extra = l;
    extra["unexpected"] = true.into();
    assert!(Ledger::decode(&serde_json::to_vec(&extra).unwrap()).is_err());
}
