// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_authoring_frontend::{Package, parse_fragment};
use choreoform_semantic_check::{CORE, CORE_V2, CORE_V3, SEMANTICS};

pub(super) fn source_resources(m: &Value) -> Vec<Resource<'_>> {
    let mut r = resources(m);
    r.extend([
        Resource { id:SEMANTICS.0,revision:SEMANTICS.1,bytes:include_bytes!("../../../../docs/ir/contracts/sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt") },
        Resource { id:CORE.0,revision:CORE.1,bytes:include_bytes!("../../../../docs/ir/contracts/sha256-26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58.txt") },
        Resource { id:CORE_V2.0,revision:CORE_V2.1,bytes:include_bytes!("../../../../docs/ir/contracts/sha256-5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623.txt") },
        Resource { id:CORE_V3.0,revision:CORE_V3.1,bytes:include_bytes!("../../../../docs/ir/contracts/sha256-30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3.txt") },
    ]);
    r
}
fn promote(source: &[u8], bindings: &[u8]) -> (String, Vec<u8>) {
    let source = std::str::from_utf8(source)
        .unwrap()
        .replacen("\"0.4.0\"", "\"0.5.0\"", 1);
    let mut ledger: Value = serde_json::from_slice(bindings).unwrap();
    ledger["profile"] = "0.5.0".into();
    ledger["version"] = "0.4.0".into();
    (source, serde_json::to_vec(&ledger).unwrap())
}

#[test]
fn whole_current_source_lowers_but_cannot_silently_rebind_legacy_metadata_cycles() {
    let m: Value = serde_json::from_slice(MANIFEST).unwrap();
    let r = source_resources(&m);
    let package = Package::parse(SOURCE, BINDINGS).unwrap();
    assert_eq!(package.syntax().profile(), "0.6.0");
    assert_eq!(
        package.syntax().token_count(),
        m["measuredTokens"].as_u64().unwrap() as usize
    );
    assert!(package.syntax().parser_work() > choreoform_authoring_frontend::MAX_WORK);
    let c = package.lower(&r).unwrap();
    c.check_protection(&r).unwrap();
    assert_eq!(c.document()["body"]["scopes"].as_object().unwrap().len(), 9);
    let source = std::str::from_utf8(SOURCE).unwrap().replacen(
        "authoring \"0.6.0\"",
        "authoring \"0.5.0\"",
        1,
    );
    let mut l: Value = serde_json::from_slice(BINDINGS).unwrap();
    l["profile"] = "0.5.0".into();
    l["version"] = "0.4.0".into();
    l["contracts"]["core"]["contract"] = CORE_V2.0.into();
    l["contracts"]["core"]["digest"] = CORE_V2.1.into();
    let bytes = serde_json::to_vec(&l).unwrap();
    let old = Package::parse(source.as_bytes(), &bytes).unwrap();
    assert_eq!(old.syntax().token_count(), package.syntax().token_count());
    assert_eq!(old.lower(&r).unwrap_err().code, "policy-cycle");
}

#[test]
fn strict_pair_pin_resource_and_integrity_domains_are_preserved() {
    let m: Value = serde_json::from_slice(MANIFEST).unwrap();
    let supplied = source_resources(&m);
    let original_source =
        include_bytes!("../../../../docs/authoring/examples/reconciliation.choreo");
    let original_bindings =
        include_bytes!("../../../../docs/authoring/examples/reconciliation.bindings.json");
    // Existing example uses the separate historical review-provider resources.
    // A human-only fixture exercises equivalent promotion without those extras.
    let mut old_l: Value = serde_json::from_slice(include_bytes!(
        "../../../../docs/authoring/examples/terminal.bindings.json"
    ))
    .unwrap();
    old_l["profile"] = "0.4.0".into();
    old_l["version"] = "0.3.0".into();
    old_l["contracts"]["core"]["contract"] = CORE_V2.0.into();
    old_l["contracts"]["core"]["digest"] = CORE_V2.1.into();
    let old_source = include_str!("../../../../docs/authoring/examples/terminal.choreo")
        .replace("\"0.2.0\"", "\"0.4.0\"");
    let old_bytes = serde_json::to_vec(&old_l).unwrap();
    let old = Package::parse(old_source.as_bytes(), &old_bytes).unwrap();
    let (new_source, new_bindings) = promote(old_source.as_bytes(), &old_bytes);
    let new = Package::parse(new_source.as_bytes(), &new_bindings).unwrap();
    assert_eq!(
        old.lower(&supplied).unwrap().document(),
        new.lower(&supplied).unwrap().document()
    );
    assert_ne!(old.integrity(), new.integrity());
    assert_eq!(
        Package::parse(new_source.as_bytes(), &old_bytes)
            .unwrap_err()
            .code,
        "package-profile"
    );
    let (s, mut l) = promote(original_source, original_bindings);
    let mut document: Value = serde_json::from_slice(&l).unwrap();
    document["version"] = "0.3.0".into();
    l = serde_json::to_vec(&document).unwrap();
    assert_eq!(
        Package::parse(s.as_bytes(), &l).unwrap_err().code,
        "ledger-profile"
    );
    let mut missing = source_resources(&m);
    missing.retain(|r| r.id != CORE.0);
    assert_eq!(
        new.lower(&missing).unwrap_err().code,
        "resource-missing-or-ambiguous"
    );
}

#[test]
fn new_quotas_require_the_explicit_program_header_and_stay_bounded() {
    for profile in ["0.2.0", "0.3.0", "0.4.0", "0.8.0"] {
        let source = format!(
            "Choreoform authoring \"{profile}\". {}",
            "bad ".repeat(5000)
        );
        assert_eq!(parse(source.as_bytes()).unwrap_err().code, "tokens");
    }
    let source = format!("Choreoform authoring \"0.5.0\". {}", "bad ".repeat(5000));
    assert_eq!(parse(source.as_bytes()).unwrap_err().code, "syntax");
    assert_eq!(
        parse_fragment(source.as_bytes(), "declaration")
            .unwrap_err()
            .code,
        "tokens"
    );
    let source = format!("Wrong header \"0.5.0\". {}", "bad ".repeat(5000));
    assert_eq!(parse(source.as_bytes()).unwrap_err().code, "tokens");
    let source = format!("Choreoform authoring \"0.5.0\". {}", "bad ".repeat(16380));
    assert_eq!(parse(source.as_bytes()).unwrap_err().code, "syntax");
    let source = format!("Choreoform authoring \"0.5.0\". {}", "bad ".repeat(16381));
    assert_eq!(parse(source.as_bytes()).unwrap_err().code, "tokens");
    let source = format!(
        "Choreoform authoring \"0.5.0\". Process \"P\". Start at \"Done\". Inputs empty. Outputs empty. Outcomes [\"complete\"]. Use cancellation \"C\", faults \"F\", closure \"L\" and races \"R\". Let \"Malformed\" mean ({}true{}) as truth value with parameters empty. End process.",
        "(".repeat(40),
        ")".repeat(40)
    );
    assert_eq!(parse(source.as_bytes()).unwrap_err().code, "parse-budget");
}
