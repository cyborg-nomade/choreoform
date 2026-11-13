// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Exact source-only contracts, actual caller proofs and bounded whole variants.
use choreoform_authoring_frontend::Package;
use choreoform_ir_probe_core::Resource;
use choreoform_semantic_check::{self as sdk, CORE_V12};
use serde_json::{Value, json};
fn base() -> (String, Value) {
    let source = include_str!("../../../docs/authoring/examples/terminal.choreo")
        .replace("authoring \"0.2.0\"", "authoring \"0.19.0\"");
    let mut ledger: Value = serde_json::from_str(include_str!(
        "../../../docs/authoring/examples/terminal.bindings.json"
    ))
    .unwrap();
    ledger["profile"] = "0.19.0".into();
    ledger["version"] = "0.18.0".into();
    ledger["contracts"]["core"]["contract"] = CORE_V12.0.into();
    ledger["contracts"]["core"]["digest"] = CORE_V12.1.into();
    (source, ledger)
}
#[test]
fn no_use_source19_keeps_inherited_ir_and_candidate_staging() {
    let (s, mut l) = base();
    let s = s.replace(
        "End process.",
        "Let \"Bad result\" mean (0) as text with parameters empty.\nEnd process.",
    );
    let owner = l["root"].clone();
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"bad_result","kind":"expression","scope":owner,"name":"Bad result"}));
    let b = serde_json::to_vec(&l).unwrap();
    let p = Package::parse(s.as_bytes(), &b).unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let raw: Vec<_> = [
        sdk::SEMANTICS,
        sdk::CORE,
        sdk::CORE_V2,
        sdk::CORE_V3,
        sdk::CORE_V4,
        sdk::CORE_V5,
        sdk::CORE_V6,
        sdk::CORE_V7,
        sdk::CORE_V8,
        sdk::CORE_V9,
        sdk::CORE_V10,
        sdk::CORE_V11,
        CORE_V12,
    ]
    .into_iter()
    .map(|pin| {
        (
            pin,
            std::fs::read(root.join(format!("docs/ir/contracts/{}.txt", pin.1.replace(':', "-"))))
                .unwrap(),
        )
    })
    .collect();
    let resources: Vec<_> = raw
        .iter()
        .map(|((id, revision), bytes)| Resource {
            id,
            revision,
            bytes,
        })
        .collect();
    let c = p.lower(&resources).unwrap();
    assert_eq!(
        c.check_static(&resources).unwrap_err().code,
        "expression-result"
    );
    let mut old = l.clone();
    old["profile"] = "0.18.0".into();
    old["version"] = "0.17.0".into();
    let bytes = serde_json::to_vec(&old).unwrap();
    let previous = s.replace("0.19.0", "0.18.0");
    assert_eq!(
        Package::parse(previous.as_bytes(), &bytes)
            .unwrap()
            .lower(&resources)
            .unwrap()
            .document(),
        c.document()
    );
    assert_eq!(c.document()["version"], "0.4.0");
    assert_eq!(c.document()["body"]["dialects"]["core"]["id"], CORE_V12.0);
}
#[test]
fn source_only_definition_and_input_have_a_distinct_registered_kind_and_owner_role() {
    let (s, mut l) = base();
    let s=s.replace("End process.","Define test \"Equal input\" with inputs [\"Observed\" as text]. It holds when ((the parameter \"Observed\") equals (the parameter \"Observed\")). End test.\nEnd process.");
    let root = l["root"].clone();
    l["declarations"].as_array_mut().unwrap().push(
        json!({"id":"source_only_test","kind":"predicate","scope":root,"name":"Equal input"}),
    );
    l["symbols"].as_array_mut().unwrap().push(json!({"id":"source_only_input","owner":"source_only_test","role":"predicate-input","name":"Observed","wire":"observed"}));
    let b = serde_json::to_vec(&l).unwrap();
    Package::parse(s.as_bytes(), &b).unwrap();
}

fn contracts() -> Vec<(String, String, Vec<u8>)> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    [
        sdk::SEMANTICS,
        sdk::CORE,
        sdk::CORE_V2,
        sdk::CORE_V3,
        sdk::CORE_V4,
        sdk::CORE_V5,
        sdk::CORE_V6,
        sdk::CORE_V7,
        sdk::CORE_V8,
        sdk::CORE_V9,
        sdk::CORE_V10,
        sdk::CORE_V11,
        CORE_V12,
    ]
    .into_iter()
    .map(|(id, rev)| {
        (
            id.into(),
            rev.into(),
            std::fs::read(root.join(format!("docs/ir/contracts/{}.txt", rev.replace(':', "-"))))
                .unwrap(),
        )
    })
    .collect()
}
fn supplied(rows: &[(String, String, Vec<u8>)]) -> Vec<Resource<'_>> {
    rows.iter()
        .map(|(id, revision, bytes)| Resource {
            id,
            revision,
            bytes,
        })
        .collect()
}
fn sample(ty: &str, body: &str, arg: Option<&str>) -> (String, Value) {
    let (s, mut l) = base();
    let root = l["root"].clone();
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"pure_test","kind":"predicate","scope":root,"name":"Check value"}));
    l["symbols"].as_array_mut().unwrap().push(json!({"id":"pure_input","owner":"pure_test","role":"predicate-input","name":"Input","wire":"value"}));
    let mut declarations = format!(
        "Define test \"Check value\" with inputs [\"Input\" as {ty}]. It holds when ({body}). End test.\n"
    );
    if let Some(arg) = arg {
        l["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":"test_result","kind":"expression","scope":root,"name":"Result"}));
        declarations.push_str(&format!("Let \"Result\" mean (test \"Check value\" using [\"Input\" is ({arg})]) as truth value with parameters empty.\n"));
    }
    (
        s.replace("End process.", &(declarations + "End process.")),
        l,
    )
}
fn candidate(
    s: &str,
    l: &Value,
    resources: &[Resource<'_>],
) -> Result<choreoform_authoring_frontend::Candidate, choreoform_authoring_frontend::Diagnostic> {
    let b = serde_json::to_vec(l).unwrap();
    Package::parse(s.as_bytes(), &b)?.lower(resources)
}
#[test]
fn actual_use_expands_to_exact_old_ast_and_checks_declared_argument_type_before_emission() {
    let owned = contracts();
    let resources = supplied(&owned);
    let body = "(the parameter \"Input\") equals (the parameter \"Input\")";
    let (s, l) = sample("text", body, Some("\"same\""));
    let c = candidate(&s, &l, &resources).unwrap();
    assert_eq!(
        c.document()["body"]["expressions"]["test_result"]["body"],
        json!({"op":"eq","left":{"op":"literal","type":{"kind":"text"},"value":"same"},"right":{"op":"literal","type":{"kind":"text"},"value":"same"}})
    );
    c.check_static(&resources).unwrap();
    let (s, l) = sample("text", body, Some("1"));
    let e = candidate(&s, &l, &resources).unwrap_err();
    assert_eq!(e.code, "source-type-mismatch");
    assert!(e.span.start > 0);
}
#[test]
fn unused_definitions_check_types_inputs_purity_and_cycles() {
    let owned = contracts();
    let resources = supplied(&owned);
    for (body, code) in [
        ("the parameter \"Input\"", "predicate-result"),
        ("true", "predicate-unused-input"),
        ("the value of \"SECRET\"", "predicate-purity"),
        ("the fact \"result\"", "predicate-purity"),
    ] {
        let (s, l) = sample("text", body, None);
        assert_eq!(candidate(&s, &l, &resources).unwrap_err().code, code);
    }
    let (s, mut l) = base();
    let root = l["root"].clone();
    for (id, name) in [("cycle_a", "A"), ("cycle_b", "B")] {
        l["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"kind":"predicate","scope":root,"name":name}));
    }
    let s=s.replace("End process.","Define test \"A\" with inputs empty. It holds when (test \"B\" using empty). End test. Define test \"B\" with inputs empty. It holds when (test \"A\" using empty). End test. End process.");
    assert_eq!(
        candidate(&s, &l, &resources).unwrap_err().code,
        "predicate-cycle"
    );
}

#[test]
fn nested_forward_uses_keep_parameter_wires_hygienic_and_format_exactly() {
    let owned = contracts();
    let resources = supplied(&owned);
    let (s, mut l) = sample(
        "text",
        "test \"Inner\" using [\"Value\" is (the parameter \"Input\")]",
        Some("\"same\""),
    );
    let root = l["root"].clone();
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"zz_inner","kind":"predicate","scope":root,"name":"Inner"}));
    // Same wire as the outer input is intentional; inserted ASTs must not rebind.
    l["symbols"].as_array_mut().unwrap().push(json!({"id":"inner_input","owner":"zz_inner","role":"predicate-input","name":"Value","wire":"value"}));
    let s=s.replace("End process.","Define test \"Inner\" with inputs [\"Value\" as text]. It holds when ((the parameter \"Value\") equals (the parameter \"Value\")). End test. End process.");
    let c = candidate(&s, &l, &resources).unwrap();
    let b = serde_json::to_vec(&l).unwrap();
    let (f, b) = Package::parse(s.as_bytes(), &b)
        .unwrap()
        .formatted()
        .unwrap();
    assert_eq!(
        Package::parse(f.as_bytes(), b.as_bytes())
            .unwrap()
            .lower(&resources)
            .unwrap()
            .document(),
        c.document()
    );
    assert_eq!(
        c.document()["body"]["expressions"]["test_result"]["body"]["left"]["value"],
        "same"
    );
}
#[test]
fn complete_argument_record_type_is_enforced_despite_a_valid_expanded_field_comparison() {
    let owned = contracts();
    let resources = supplied(&owned);
    let body = "(field \"first\" of (the parameter \"Input\")) equals (field \"first\" of (the parameter \"Input\"))";
    let (s, l) = sample(
        "record with [\"first\" as text]",
        body,
        Some("record with [\"first\" is (\"a\"), \"second\" is (\"b\")]"),
    );
    assert_eq!(
        candidate(&s, &l, &resources).unwrap_err().code,
        "source-type-mismatch"
    );
}
#[test]
fn literal_operation_like_values_and_semantic_ids_named_type_are_not_traces() {
    let owned = contracts();
    let resources = supplied(&owned);
    let ty = "record with [\"op\" as text, \"name\" as text]";
    let arg = "literal of record with [\"op\" as text, \"name\" as text] with value record with [\"op\" is (\"param\"), \"name\" is (\"value\")]";
    let (s, mut l) = sample(
        ty,
        "(the parameter \"Input\") equals (the parameter \"Input\")",
        Some(arg),
    );
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|d| d["id"] == "test_result")
        .unwrap()["id"] = "type".into();
    let c = candidate(&s, &l, &resources).unwrap();
    assert_eq!(
        c.document()["body"]["expressions"]["type"]["body"]["left"]["value"],
        json!({"op":"param","name":"value"})
    );
    c.check_static(&resources).unwrap();
}
#[test]
fn stale_source_definitions_and_bad_arity_refuse_instead_of_panicking() {
    let owned = contracts();
    let resources = supplied(&owned);
    let (s, l) = sample(
        "text",
        "(the parameter \"Input\") equals (the parameter \"Input\")",
        Some("\"a\""),
    );
    let bad=s.replace("Define test \"Check value\" with inputs [\"Input\" as text]. It holds when ((the parameter \"Input\") equals (the parameter \"Input\")). End test.\n","");
    assert_ne!(bad, s);
    assert_eq!(
        candidate(&bad, &l, &resources).unwrap_err().code,
        "predicate-definition"
    );
    let bad = s.replace("using [\"Input\" is (\"a\")]", "using empty");
    assert_ne!(bad, s);
    assert_eq!(
        candidate(&bad, &l, &resources).unwrap_err().code,
        "predicate-arguments"
    );
}

#[test]
fn managed_names_order_fresh_copies_and_retirement_do_not_recover_ids_from_captions() {
    let owned = contracts();
    let resources = supplied(&owned);
    let (s, l) = sample(
        "text",
        "(the parameter \"Input\") equals (the parameter \"Input\")",
        Some("\"same\""),
    );
    let expected = candidate(&s, &l, &resources).unwrap();
    let renamed = s
        .replace("Check value", "同じ値")
        .replace("\"Input\"", "\"観測値\"");
    let mut n = l.clone();
    for row in n["declarations"].as_array_mut().unwrap() {
        if row["id"] == "pure_test" {
            row["name"] = "同じ値".into();
        }
    }
    for row in n["symbols"].as_array_mut().unwrap() {
        if row["id"] == "pure_input" {
            row["name"] = "観測値".into();
        }
    }
    n["declarations"].as_array_mut().unwrap().reverse();
    n["symbols"].as_array_mut().unwrap().reverse();
    assert_eq!(
        candidate(&renamed, &n, &resources).unwrap().document(),
        expected.document()
    );
    assert_eq!(
        candidate(&renamed, &l, &resources).unwrap_err().code,
        "binding-mismatch"
    );
    let copy = s.replace("End process.", "Define test \"Copied\" with inputs [\"Input\" as text]. It holds when ((the parameter \"Input\") equals (the parameter \"Input\")). End test. End process.");
    let mut copied = l.clone();
    let root = copied["root"].clone();
    copied["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"copy_test","kind":"predicate","scope":root,"name":"Copied"}));
    copied["symbols"].as_array_mut().unwrap().push(json!({"id":"copy_input","owner":"copy_test","role":"predicate-input","name":"Input","wire":"value"}));
    assert_eq!(
        candidate(&copy, &copied, &resources).unwrap().document(),
        expected.document()
    );
    for id in ["pure_test", "pure_input"] {
        let mut retired = l.clone();
        retired["retired"] = json!([id]);
        assert_eq!(
            candidate(&s, &retired, &resources).unwrap_err().code,
            "retired-identity"
        );
    }
    let mut missing = l.clone();
    missing["symbols"]
        .as_array_mut()
        .unwrap()
        .retain(|r| r["id"] != "pure_input");
    assert!(candidate(&s, &missing, &resources).is_err());
    let mut extra = l.clone();
    extra["declarations"].as_array_mut().unwrap().push(
        json!({"id":"extra_test","kind":"predicate","scope":"scope_terminal","name":"Extra"}),
    );
    assert_eq!(
        candidate(&s, &extra, &resources).unwrap_err().code,
        "source-ledger-inventory"
    );
}

#[test]
fn nested_call_type_contracts_apply_inside_unused_definitions() {
    let owned = contracts();
    let resources = supplied(&owned);
    let (s, mut l) = sample(
        "whole number",
        "test \"Inner\" using [\"Value\" is (the parameter \"Input\")]",
        None,
    );
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"zz_inner","kind":"predicate","scope":"scope_terminal","name":"Inner"}));
    l["symbols"].as_array_mut().unwrap().push(json!({"id":"inner_input","owner":"zz_inner","role":"predicate-input","name":"Value","wire":"x"}));
    let s = s.replace("End process.", "Define test \"Inner\" with inputs [\"Value\" as text]. It holds when ((the parameter \"Value\") equals (the parameter \"Value\")). End test. End process.");
    assert_eq!(
        candidate(&s, &l, &resources).unwrap_err().code,
        "source-type-mismatch"
    );
}

#[test]
fn zero_input_calls_need_an_actual_context_and_keep_boolean_result_contract() {
    let owned = contracts();
    let resources = supplied(&owned);
    let (s, mut l) = base();
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"always","kind":"predicate","scope":"scope_terminal","name":"Always"}));
    let s = s.replace("Permit when (true).", "Permit when (test \"Always\" using empty).").replace("End process.", "Define test \"Always\" with inputs empty. It holds when (true). End test. End process.");
    candidate(&s, &l, &resources)
        .unwrap()
        .check_static(&resources)
        .unwrap();
    let bad = s.replace("It holds when (true)", "It holds when (1)");
    assert_eq!(
        candidate(&bad, &l, &resources).unwrap_err().code,
        "predicate-result"
    );
    let s = s.replace("End process.", "Rule for work \"Unused\". Instructions \"Test\". Authority \"Reviewer\". Attempts 1. Delays empty. Retry faults empty. Timeout none. Complete when (test \"Always\" using empty). Compensates none. End policy. End process.");
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"unused","kind":"policy","scope":"scope_terminal","name":"Unused"}));
    assert_eq!(
        candidate(&s, &l, &resources).unwrap_err().code,
        "source-type-context"
    );
}

#[test]
fn ancestor_tests_are_visible_but_qualified_private_descendants_are_not() {
    let owned = contracts();
    let resources = supplied(&owned);
    let (s, mut l) = sample(
        "text",
        "(the parameter \"Input\") equals (the parameter \"Input\")",
        Some("\"a\""),
    );
    let child = "Scope \"Child\" within \"Terminal example\". Start at \"Child done\". Inputs empty. Outputs empty. Outcomes [\"complete\"]. Use cancellation \"Stop\", faults \"Fault handling\", closure \"Close\" and races \"Commit order\". Step \"Child done\". Finish as \"complete\". End step. End scope.";
    for (id, kind, scope, name) in [
        ("child", "scope", "scope_terminal", "Child"),
        ("child_done", "node", "child", "Child done"),
    ] {
        l["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"kind":kind,"scope":scope,"name":name}));
    }
    l["symbols"].as_array_mut().unwrap().push(json!({"id":"child_complete","owner":"child","role":"outcome","name":"complete","wire":"complete"}));
    let s = s.replace("End process.", &format!("{child} End process."));
    let expr = "Let \"Result\" mean (test \"Check value\" using [\"Input\" is (\"a\")]) as truth value with parameters empty.";
    assert!(s.contains(expr));
    let nested = s.replacen(expr, "", 1).replace(
        "Step \"Child done\"",
        &format!("{expr} Step \"Child done\""),
    );
    let mut n = l.clone();
    n["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "test_result")
        .unwrap()["scope"] = "child".into();
    candidate(&nested, &n, &resources).unwrap();
    let def = "Define test \"Check value\" with inputs [\"Input\" as text]. It holds when ((the parameter \"Input\") equals (the parameter \"Input\")). End test.";
    assert!(s.contains(def));
    let hidden = s
        .replacen(def, "", 1)
        .replace("Step \"Child done\"", &format!("{def} Step \"Child done\""));
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "pure_test")
        .unwrap()["scope"] = "child".into();
    let hidden = hidden.replace(
        "test \"Check value\" using",
        "test \"Check value\" in \"Child\" in \"Terminal example\" using",
    );
    assert_eq!(
        candidate(&hidden, &l, &resources).unwrap_err().code,
        "reference-missing"
    );
}

#[test]
fn duplicated_nested_arguments_and_many_unused_definitions_share_bounds() {
    let owned = contracts();
    let resources = supplied(&owned);
    let (mut s, mut l) = base();
    for i in 0..20 {
        let body = if i == 0 {
            "(the parameter \"Value\") equals (the parameter \"Value\")".into()
        } else {
            format!(
                "(test \"Test {}\" using [\"Value\" is (the parameter \"Value\")]) and (test \"Test {}\" using [\"Value\" is (the parameter \"Value\")])",
                i - 1,
                i - 1
            )
        };
        s=s.replace("End process.",&format!("Define test \"Test {i}\" with inputs [\"Value\" as text]. It holds when ({body}). End test. End process."));
        l["declarations"].as_array_mut().unwrap().push(json!({"id":format!("t{i:02}"),"kind":"predicate","scope":"scope_terminal","name":format!("Test {i}")}));
        l["symbols"].as_array_mut().unwrap().push(json!({"id":format!("p{i}"),"owner":format!("t{i:02}"),"role":"predicate-input","name":"Value","wire":"v"}));
    }
    assert!(s.len() < 10000);
    assert!(matches!(
        candidate(&s, &l, &resources).unwrap_err().code,
        "depth" | "size" | "work-limit" | "predicate-expansion"
    ));
    let (mut s, mut l) = base();
    for i in 0..5000 {
        s=s.replace("End process.",&format!("Define test \"Test {i}\" with inputs empty. It holds when (true). End test. End process."));
        l["declarations"].as_array_mut().unwrap().push(json!({"id":format!("t{i:04}"),"kind":"predicate","scope":"scope_terminal","name":format!("Test {i}")}));
    }
    assert!(s.len() < 500000);
    assert!(matches!(
        candidate(&s, &l, &resources).unwrap_err().code,
        "work-limit" | "size" | "budget"
    ));
}

#[path = "support/predicate_recipes.rs"]
mod variants;
#[test]
fn distinct_nominals_with_equal_representations_and_hidden_static_pins_are_not_interchangeable() {
    use choreoform_ir_probe_core::digest;
    let id = "urn:test:pure-predicate:types";
    let raw=serde_json::to_vec(&json!({"kind":"named-types","version":"0.1.0","id":id,"definitions":{
        "A":{"representation":{"kind":"text"},"invariant":{"op":"literal","type":{"kind":"boolean"},"value":true}},
        "B":{"representation":{"kind":"text"},"invariant":{"op":"literal","type":{"kind":"boolean"},"value":true}}
    }})).unwrap();
    let rev = digest(&raw);
    let mut owned = contracts();
    owned.push((id.into(), rev.clone(), raw));
    let resources = supplied(&owned);
    let (s, mut l) = sample(
        "type \"A\" from \"Types\"",
        "(the parameter \"Input\") equals (the parameter \"Input\")",
        Some("(\"a\") as named type \"A\" from \"Types\""),
    );
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"types","kind":"resource","scope":"scope_terminal","name":"Types"}));
    l["resources"]
        .as_array_mut()
        .unwrap()
        .push(json!({"binding":"types","kind":"named-type","contract":id,"digest":rev}));
    candidate(&s, &l, &resources).unwrap();
    let bad = s.replace("(\"a\") as named type \"A\"", "(\"a\") as named type \"B\"");
    assert_ne!(s, bad);
    assert_eq!(
        candidate(&bad, &l, &resources).unwrap_err().code,
        "source-type-mismatch"
    );
    // A visible helper cannot leak a private static pin from a descendant.
    let child = "Scope \"Child\" within \"Terminal example\". Start at \"Child done\". Inputs empty. Outputs empty. Outcomes [\"complete\"]. Use cancellation \"Stop\", faults \"Fault handling\", closure \"Close\" and races \"Commit order\". Step \"Child done\". Finish as \"complete\". End step. End scope.";
    for (id, kind, scope, name) in [
        ("child", "scope", "scope_terminal", "Child"),
        ("child_done", "node", "child", "Child done"),
    ] {
        l["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"kind":kind,"scope":scope,"name":name}));
    }
    l["symbols"].as_array_mut().unwrap().push(json!({"id":"child_complete","owner":"child","role":"outcome","name":"complete","wire":"complete"}));
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "types")
        .unwrap()["scope"] = "child".into();
    let hidden = s
        .replace("End process.", &format!("{child} End process."))
        .replace(
            "from \"Types\"",
            "from \"Types\" in \"Child\" in \"Terminal example\"",
        );
    assert_eq!(
        candidate(&hidden, &l, &resources).unwrap_err().code,
        "reference-missing"
    );
}

#[test]
fn actual_complete_expense_and_order_variants_keep_exact_current_ir_and_measure_real_source() {
    use choreoform_authoring_frontend::parse;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for name in ["rp01", "rp03"] {
        let base = root.join(format!("docs/benchmarks/{name}"));
        let source = std::fs::read_to_string(base.join("source.choreo")).unwrap();
        let mut ledger: Value =
            serde_json::from_slice(&std::fs::read(base.join("bindings.json")).unwrap()).unwrap();
        let manifest: Value =
            serde_json::from_slice(&std::fs::read(base.join("manifest.json")).unwrap()).unwrap();
        let doc: Value =
            serde_json::from_slice(&std::fs::read(base.join("candidate.ir.json")).unwrap())
                .unwrap();
        let types: Value =
            serde_json::from_slice(&std::fs::read(base.join("types.json")).unwrap()).unwrap();
        let mut owned = contracts();
        for r in manifest["resources"].as_array().unwrap() {
            let id = r["id"].as_str().unwrap();
            let rev = r["revision"].as_str().unwrap();
            if !owned.iter().any(|(i, v, _)| i == id && v == rev) {
                owned.push((
                    id.into(),
                    rev.into(),
                    std::fs::read(root.join(r["path"].as_str().unwrap())).unwrap(),
                ));
            }
        }
        let result = variants::factor(name, &source, &mut ledger, &doc, &types);
        let published = root.join(format!("docs/evaluation/0090-predicate-variants/{name}"));
        assert_eq!(
            std::fs::read_to_string(published.join("source.choreo")).unwrap(),
            result
        );
        assert_eq!(
            serde_json::from_slice::<Value>(
                &std::fs::read(published.join("bindings.json")).unwrap()
            )
            .unwrap(),
            ledger
        );
        let syntax = parse(result.as_bytes()).unwrap();
        println!(
            "{name} pure recipe variant {} bytes, {} tokens, {} parser work",
            result.len(),
            syntax.token_count(),
            syntax.parser_work()
        );
        if let Ok(directory) = std::env::var("CHOREOFORM_RECIPE_VARIANTS") {
            let path = std::path::Path::new(&directory).join(name);
            std::fs::create_dir_all(&path).unwrap();
            std::fs::write(path.join("source.choreo"), &result).unwrap();
            std::fs::write(
                path.join("bindings.json"),
                serde_json::to_vec_pretty(&ledger).unwrap(),
            )
            .unwrap();
        }
        let resources = supplied(&owned);
        let c = candidate(&result, &ledger, &resources).unwrap();
        assert_eq!(c.document(), &doc);
        c.check_static(&resources).unwrap();
        c.check_control(&resources).unwrap();
        c.check_protection(&resources).unwrap();
        c.check_invalidation(&resources).unwrap();
        let bytes = serde_json::to_vec(&ledger).unwrap();
        let p = Package::parse(result.as_bytes(), &bytes).unwrap();
        let (formatted, bindings) = p.formatted().unwrap();
        let roundtrip = Package::parse(formatted.as_bytes(), bindings.as_bytes()).unwrap();
        assert_eq!(
            roundtrip.lower(&resources).unwrap().document(),
            c.document()
        );
        assert_ne!(p.integrity().source, roundtrip.integrity().source);
        assert_ne!(p.integrity().package, roundtrip.integrity().package);
        if name == "rp03" {
            assert_eq!(
                c.check_concurrency(&resources).unwrap_err().code,
                "parallel-effects-unproved"
            );
        } else {
            c.check_concurrency(&resources).unwrap();
            c.check_timing(&resources).unwrap();
            c.check_fault_routes(&resources).unwrap();
            c.check_exceptional_availability(&resources).unwrap();
            c.check_reconciliation_links(&resources).unwrap();
            c.check_reconciliation_availability(&resources).unwrap();
        }
    }
}
