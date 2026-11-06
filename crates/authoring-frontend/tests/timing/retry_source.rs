// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_authoring_frontend::{Ledger, parse, parse_fragment};
use choreoform_semantic_check::CORE_V2;

const CLAUSE: &str = "For retries use clock \"Elapsed clock\", basis \"Audit basis\". ";
fn source_package(f: &mut Fixture, multiple: bool) -> (String, Value) {
    f.artifact(CORE_V2.0, include_bytes!("../../../../docs/ir/contracts/sha256-5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623.txt").to_vec());
    let mut source = include_str!("../../../../docs/authoring/examples/review.choreo")
        .replace("\"0.2.0\"", "\"0.3.0\"")
        .replace("    Also read [\"Audit basis\"].\n", "");
    if multiple {
        source = source
            .replace("Attempts 1.", "Attempts 2.")
            .replace("Delays empty.", "Delays [0 microseconds].")
            .replace("Timeout none.", &format!("{CLAUSE}Timeout none."));
    }
    let mut ledger: Value = serde_json::from_slice(include_bytes!(
        "../../../../docs/authoring/examples/review.bindings.json"
    ))
    .unwrap();
    ledger["version"] = "0.2.0".into();
    ledger["profile"] = "0.3.0".into();
    ledger["contracts"]["core"]["contract"] = CORE_V2.0.into();
    ledger["contracts"]["core"]["digest"] = CORE_V2.1.into();
    ledger["declarations"].as_array_mut().unwrap().push(json!({"id":"clock_binding","kind":"resource","scope":"scope_review","name":"Elapsed clock"}));
    ledger["resources"].as_array_mut().unwrap().push(json!({"binding":"clock_binding","kind":"clock","contract":f.clock.id,"digest":f.clock.revision}));
    (source, ledger)
}

#[test]
fn new_source_one_and_multiple_attempts_roundtrip_exact_ir_and_retry_reports() {
    for multiple in [false, true] {
        let mut f = Fixture::new(false);
        let (source, ledger) = source_package(&mut f, multiple);
        let raw = serde_json::to_vec(&ledger).unwrap();
        let resources = f.resources();
        let package = Package::parse(source.as_bytes(), &raw).unwrap();
        assert_eq!(package.syntax().profile(), "0.3.0");
        let candidate = package.lower(&resources).unwrap();
        let r = &candidate.document()["body"]["policies"]["review_work"]["body"]["retry"];
        assert_eq!(r["clock"].is_null(), !multiple);
        assert_eq!(r["basis"].is_null(), !multiple);
        assert_eq!(
            candidate.document()["body"]["dialects"]["core"]["id"],
            CORE_V2.0
        );
        let report = candidate.check_timing(&resources).unwrap();
        assert_eq!(report.retry_uses().len(), usize::from(multiple));
        let (formatted, companion) = package.formatted().unwrap();
        let new = Package::parse(formatted.as_bytes(), companion.as_bytes())
            .unwrap()
            .lower(&resources)
            .unwrap();
        assert_eq!(new.document(), candidate.document());
        assert_eq!(
            new.check_timing(&resources).unwrap().retry_uses(),
            report.retry_uses()
        );
        let mut retired = ledger.clone();
        retired["retired"] = json!(["retired_work"]);
        let raw = serde_json::to_vec(&retired).unwrap();
        let p = Package::parse(source.as_bytes(), &raw).unwrap();
        assert_eq!(
            p.lower(&resources).unwrap().document(),
            candidate.document()
        );
        assert_eq!(
            p.formatted().unwrap().1,
            Ledger::decode(&raw).unwrap().canonical()
        );
    }
}

#[test]
fn explicit_source_companion_core_and_grammar_versions_cannot_be_mixed() {
    let mut f = Fixture::new(false);
    let (source, ledger) = source_package(&mut f, true);
    let old = include_bytes!("../../../../docs/authoring/examples/review.bindings.json");
    assert_eq!(
        Package::parse(source.as_bytes(), old).unwrap_err().code,
        "package-profile"
    );
    let new = serde_json::to_vec(&ledger).unwrap();
    let old_source = include_bytes!("../../../../docs/authoring/examples/review.choreo");
    assert_eq!(
        Package::parse(old_source, &new).unwrap_err().code,
        "package-profile"
    );
    assert_eq!(
        parse(source.replace("\"0.3.0\"", "\"0.2.0\"").as_bytes())
            .unwrap_err()
            .code,
        "syntax"
    );
    assert_eq!(
        parse_fragment(CLAUSE.as_bytes(), "retry_binding")
            .unwrap_err()
            .code,
        "grammar-rule"
    );
    for wrong in ["companion", "pin"] {
        let mut l = ledger.clone();
        if wrong == "companion" {
            l["version"] = "0.1.0".into();
        } else {
            l["contracts"]["core"]["contract"] = CORE.0.into();
            l["contracts"]["core"]["digest"] = CORE.1.into();
        }
        let error = Ledger::decode(&serde_json::to_vec(&l).unwrap()).unwrap_err();
        assert_eq!(
            error.code,
            if wrong == "companion" {
                "ledger-profile"
            } else {
                "contract-pin"
            }
        );
    }
}

#[test]
fn source_retry_binding_presence_and_reference_roles_are_explicit() {
    for (mutation, code) in [
        ("missing", "retry-binding"),
        ("one", "retry-binding"),
        ("clock-role", "reference-kind"),
        ("basis-role", "reference-kind"),
        ("duplicate", "syntax"),
    ] {
        let mut f = Fixture::new(false);
        let (mut source, ledger) = source_package(&mut f, true);
        source = match mutation {
            "missing" => source.replace(CLAUSE, ""),
            "one" => source
                .replace("Attempts 2.", "Attempts 1.")
                .replace("Delays [0 microseconds].", "Delays empty."),
            "clock-role" => source.replace("use clock \"Elapsed clock\"", "use clock \"Request\""),
            "basis-role" => source.replace("basis \"Audit basis\"", "basis \"Reviewer\""),
            _ => source.replace(CLAUSE, &format!("{CLAUSE}{CLAUSE}")),
        };
        let raw = serde_json::to_vec(&ledger).unwrap();
        let error = match Package::parse(source.as_bytes(), &raw) {
            Ok(p) => p.lower(&f.resources()).unwrap_err(),
            Err(e) => e,
        };
        assert_eq!(error.code, code, "{mutation}: {error:?}");
    }
}

#[test]
fn source_retry_derives_basis_and_transitive_clock_authority_reads() {
    let mut f = Fixture::new(false);
    let mut clock_source = f.document(SOURCE_ID);
    clock_source["resultProtection"]["participants"]["clock_actor"] = true.into();
    f.clock_source(clock_source);
    let (mut source, mut ledger) = source_package(&mut f, true);
    source=source.replace("End process.","Require actor \"Clock observer\" under \"Clock observer requirement\". Rule for actor \"Clock observer requirement\". Permit when ((the value of \"Request\") equals (\"Synthetic request\")). End policy. End process.");
    ledger["declarations"].as_array_mut().unwrap().extend([
        json!({"id":"clock_actor","kind":"actor","scope":"scope_review","name":"Clock observer"}),
        json!({"id":"clock_requirement","kind":"policy","scope":"scope_review","name":"Clock observer requirement"}),
    ]);
    let raw = serde_json::to_vec(&ledger).unwrap();
    let resources = f.resources();
    let candidate = Package::parse(source.as_bytes(), &raw)
        .unwrap()
        .lower(&resources)
        .unwrap();
    for map in ["policies", "nodes"] {
        let record = &candidate.document()["body"][map][if map == "policies" {
            "review_work"
        } else {
            "review_step"
        }];
        let reads = if map == "policies" {
            &record["body"]["reads"]
        } else {
            &record["reads"]
        };
        assert_eq!(*reads, json!({"review_request":true,"review_audit":true}));
    }
    candidate.check_timing(&resources).unwrap();
    let mut missing = f.resources();
    missing.retain(|r| r.id != CORE.0);
    assert_eq!(
        Package::parse(source.as_bytes(), &raw)
            .unwrap()
            .lower(&missing)
            .unwrap_err()
            .code,
        "resource-missing-or-ambiguous"
    );
    let mut missing = f.resources();
    missing.retain(|r| r.id != CORE_V2.0);
    assert_eq!(
        Package::parse(source.as_bytes(), &raw)
            .unwrap()
            .lower(&missing)
            .unwrap_err()
            .code,
        "resource-missing-or-ambiguous"
    );
}

#[test]
fn both_versioned_grammars_preserve_bounded_parsing() {
    for profile in ["0.2.0", "0.3.0", "0.4.0"] {
        let source = format!(
            "Choreoform authoring \"{profile}\". {}",
            "bad ".repeat(5000)
        );
        assert_eq!(parse(source.as_bytes()).unwrap_err().code, "tokens");
    }
}
