// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

use choreoform_authoring_frontend::{Ledger, Package, parse, parse_fragment};
use choreoform_ir_probe_core::{Resource, SUPPORTED_CONTRACTS};
use serde_json::{Value, json};

const SOURCE: &str = include_str!("../../../docs/authoring/examples/terminal.choreo");
const LEDGER: &[u8] = include_bytes!("../../../docs/authoring/examples/terminal.bindings.json");
const REVIEW_SOURCE: &str = include_str!("../../../docs/authoring/examples/review.choreo");
const REVIEW_LEDGER: &[u8] =
    include_bytes!("../../../docs/authoring/examples/review.bindings.json");
fn resources() -> [Resource<'static>; 2] {
    [
        Resource {
            id: SUPPORTED_CONTRACTS[0].0,
            revision: SUPPORTED_CONTRACTS[0].1,
            bytes: include_bytes!(
                "../../../docs/ir/contracts/sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt"
            ),
        },
        Resource {
            id: "urn:choreoform:contracts:core-profile:0.1.0",
            revision: "sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58",
            bytes: include_bytes!(
                "../../../docs/ir/contracts/sha256-26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58.txt"
            ),
        },
    ]
}
fn ledger() -> Value {
    serde_json::from_slice(LEDGER).unwrap()
}
fn bytes(v: &Value) -> Vec<u8> {
    serde_json::to_vec(v).unwrap()
}

#[test]
fn terminal_package_lowers_without_execution_authority() {
    let package = Package::parse(SOURCE.as_bytes(), LEDGER).unwrap();
    let candidate = package.lower(&resources()).unwrap();
    assert_eq!(
        candidate.document()["body"]["nodes"]["node_done"]["outcome"],
        "complete"
    );
    assert_eq!(candidate.document()["annotations"], ledger()["annotations"]);
    assert_eq!(
        package.integrity().package,
        "sha256:9d0328a4cd9266221ac31393567bc23ef6c513bfe8bb24be573047e0601b44d5"
    );
    assert_eq!(candidate.spans.len(), 9);
}

#[test]
fn formatting_and_alias_rename_preserve_ids_and_candidate_revision() {
    let package = Package::parse(SOURCE.as_bytes(), LEDGER).unwrap();
    let original = package.lower(&resources()).unwrap();
    let (source, companion) = package.formatted().unwrap();
    let formatted = Package::parse(source.as_bytes(), companion.as_bytes()).unwrap();
    assert_eq!(
        formatted.lower(&resources()).unwrap().document(),
        original.document()
    );
    assert_eq!(
        formatted.formatted().unwrap(),
        (source.clone(), companion.clone())
    );
    assert_ne!(formatted.integrity().source, package.integrity().source);
    let renamed = SOURCE.replace("\"Done\"", "\"終わり\"");
    let mut bindings = ledger();
    bindings["declarations"][8]["name"] = "終わり".into();
    let renamed = Package::parse(renamed.as_bytes(), &bytes(&bindings))
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert_eq!(renamed.document(), original.document());
}

#[test]
fn missing_extra_stale_and_retired_bindings_refuse() {
    let source = SOURCE.replace("\"Done\"", "\"New name\"");
    assert_eq!(
        Package::parse(source.as_bytes(), LEDGER)
            .unwrap()
            .lower(&resources())
            .unwrap_err()
            .code,
        "reference-missing"
    );
    let mut b = ledger();
    b["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"extra", "kind":"node", "scope":"scope_terminal", "name":"Extra"}));
    assert_eq!(
        Package::parse(SOURCE.as_bytes(), &bytes(&b))
            .unwrap()
            .lower(&resources())
            .unwrap_err()
            .code,
        "source-ledger-inventory"
    );
    b["retired"] = json!(["node_done"]);
    assert_eq!(
        Ledger::decode(&bytes(&b)).unwrap_err().code,
        "retired-identity"
    );
}

#[test]
fn registry_ambiguity_digest_and_missing_resources_refuse() {
    let package = Package::parse(SOURCE.as_bytes(), LEDGER).unwrap();
    assert_eq!(
        package.lower(&[]).unwrap_err().code,
        "resource-missing-or-ambiguous"
    );
    let mut r = resources();
    r[0].bytes = b"changed";
    assert_eq!(package.lower(&r).unwrap_err().code, "resource-digest");
    let r = resources();
    let duplicate = [
        Resource {
            id: r[0].id,
            revision: r[0].revision,
            bytes: r[0].bytes,
        },
        Resource {
            id: r[1].id,
            revision: r[1].revision,
            bytes: r[1].bytes,
        },
        Resource {
            id: r[0].id,
            revision: r[0].revision,
            bytes: r[0].bytes,
        },
    ];
    assert_eq!(
        package.lower(&duplicate).unwrap_err().code,
        "resource-missing-or-ambiguous"
    );
}

#[test]
fn unicode_spans_and_lexical_bounds_are_byte_based() {
    let text = b"\"\xc3\xa9\"";
    let syntax = parse_fragment(text, "literal").unwrap();
    assert_eq!(syntax.span(syntax.tree()), 0..4);
    assert!(parse(b"\xff").is_err());
    assert!(parse_fragment(b"\"bad\\n\"", "literal").is_err());
    assert!(parse_fragment(b"\"bad\n\"", "literal").is_err());
    assert!(parse_fragment(b"-0", "literal").is_err());
    assert!(parse_fragment(b"01", "literal").is_err());
    assert!(parse_fragment(b"(true) and (false)", "expression").is_ok());
    assert!(parse_fragment(b"true and false", "expression").is_err());
    assert!(parse_fragment(b"true trailing", "expression").is_err());
    assert!(parse_fragment(&vec![b' '; 1024 * 1024 + 1], "expression").is_err());
}

#[test]
fn strict_companion_transport_preserves_unknown_annotations() {
    for input in [
        b"{\"a\":-0}".as_slice(),
        b"{\"a\":1.0}",
        b"{\"a\":1,\"a\":2}",
        b"{\"a\":\"\\ud800\"}",
        b"{\"a\":9007199254740992}",
    ] {
        assert!(Ledger::decode(input).is_err());
    }
    let mut b = ledger();
    b["annotations"]["x:text"] = "-0 \" \\".into();
    assert_eq!(
        Ledger::decode(&bytes(&b)).unwrap().document()["annotations"],
        b["annotations"]
    );
}

#[test]
fn human_outcome_and_purpose_aliases_and_conservative_reads_survive() {
    let candidate = Package::parse(REVIEW_SOURCE.as_bytes(), REVIEW_LEDGER)
        .unwrap()
        .lower(&resources())
        .unwrap();
    let body = &candidate.document()["body"];
    assert_eq!(
        body["nodes"]["review_step"]["reads"],
        json!({"review_audit":true,"review_request":true})
    );
    assert_eq!(
        body["data"]["review_decision"]["type"]["body"]["cases"]["correction_needed"],
        json!({"kind":"record","fields":{}})
    );
    assert_eq!(
        body["flows"]["review_flow_correction"]["outcome"],
        "correction_needed"
    );
    assert_eq!(
        body["policies"]["review_protection"]["body"]["permit"]["right"]["value"],
        "expense_review"
    );
    let changed = REVIEW_SOURCE.replace("correction needed", "needs revision");
    let mut bindings: Value = serde_json::from_slice(REVIEW_LEDGER).unwrap();
    bindings["symbols"][3]["name"] = "needs revision".into();
    let renamed = Package::parse(changed.as_bytes(), &bytes(&bindings))
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert_eq!(candidate.document(), renamed.document());
}

#[test]
fn generated_assignment_slots_are_not_reconstructed_from_positions() {
    let source = REVIEW_SOURCE.replace("Start at \"Review\".", "Start at \"Update audit\".").replace("End process.", "Step \"Update audit\". Set together [\"Audit basis\" to ((the value of \"Audit basis\") plus (1))]. On \"updated\", continue with \"Review\". End step. End process.");
    let mut bindings: Value = serde_json::from_slice(REVIEW_LEDGER).unwrap();
    bindings["declarations"].as_array_mut().unwrap().push(
        json!({"id":"audit_compute","kind":"node","scope":"scope_review","name":"Update audit"}),
    );
    bindings["symbols"].as_array_mut().unwrap().push(json!({"id":"audit_updated","owner":"audit_compute","role":"outcome","name":"updated","wire":"updated"}));
    bindings["generated"].as_array_mut().unwrap().extend([
        json!({"id":"audit_assignment","owner":"audit_compute","role":"assignment","key":"review_audit"}),
        json!({"id":"audit_flow","owner":"audit_compute","role":"flow","key":"audit_updated"})]);
    let candidate = Package::parse(source.as_bytes(), &bytes(&bindings))
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert_eq!(
        candidate.document()["body"]["nodes"]["audit_compute"]["assignments"]["review_audit"],
        "audit_assignment"
    );
    assert_eq!(
        candidate.document()["body"]["expressions"]["audit_assignment"]["reads"],
        json!({"review_audit":true})
    );
    bindings["generated"]
        .as_array_mut()
        .unwrap()
        .retain(|r| r["id"] != "audit_assignment");
    assert_eq!(
        Package::parse(source.as_bytes(), &bytes(&bindings))
            .unwrap()
            .lower(&resources())
            .unwrap_err()
            .code,
        "slot-mismatch"
    );
}

fn expression_candidate(expression: &str, ty: &str) -> Value {
    let source = REVIEW_SOURCE.replace(
        "Let \"Current request\" mean (the value of \"Request\") as text with parameters empty.",
        &format!(
            "Let \"Current request\" mean ({expression}) as {ty} with parameters [\"x\" as text]."
        ),
    );
    Package::parse(source.as_bytes(), REVIEW_LEDGER)
        .unwrap()
        .lower(&resources())
        .unwrap()
        .into_document()
}

#[test]
fn expression_lowering_keeps_closed_operations_and_literal_constructor_distinctions() {
    for (expression, ty, op) in [
        ("true", "truth value", "literal"),
        ("the parameter \"x\"", "text", "param"),
        (
            "field \"x\" of (record with [\"x\" is (true)])",
            "truth value",
            "field",
        ),
        ("record with empty", "record with empty", "record"),
        (
            "list of whole number containing [(1), (2)]",
            "list of whole number",
            "list",
        ),
        (
            "case \"ok\" of choice with [\"ok\" as whole number] containing (1)",
            "choice with [\"ok\" as whole number]",
            "variant",
        ),
        ("no value of text", "optional text", "none"),
        ("some (true)", "optional truth value", "some"),
        ("a value exists in (some (true))", "truth value", "isSome"),
        (
            "the present value of (some (true))",
            "truth value",
            "unwrap",
        ),
        (
            "the case of (case \"ok\" of choice with [\"ok\" as whole number] containing (1))",
            "text",
            "tag",
        ),
        (
            "the payload of (case \"ok\" of choice with [\"ok\" as whole number] containing (1)) for \"ok\"",
            "whole number",
            "payload",
        ),
        ("if (true) then (1) otherwise (2)", "whole number", "if"),
        ("not (true)", "truth value", "not"),
        ("(true) and (false)", "truth value", "and"),
        ("(true) or (false)", "truth value", "or"),
        ("(1) equals (2)", "truth value", "eq"),
        ("(1) is less than (2)", "truth value", "lt"),
        ("(1) plus (2)", "whole number", "add"),
        ("(1) minus (2)", "whole number", "sub"),
        ("(1) times (2)", "whole number", "mul"),
        (
            "(list of whole number containing [(1)]) contains (1)",
            "truth value",
            "contains",
        ),
        (
            "the length of (list of whole number containing [(1)])",
            "whole number",
            "length",
        ),
        (
            "item (list of whole number containing [(1)]) at index (0)",
            "whole number",
            "index",
        ),
        (
            "(decimal 1.20 with 2 places) expressed with 3 decimal places",
            "decimal with 3 places",
            "rescale",
        ),
    ] {
        let candidate = expression_candidate(expression, ty);
        assert_eq!(
            candidate["body"]["expressions"]["review_input"]["body"]["op"], op,
            "{expression}"
        );
    }
    let literal = expression_candidate(
        "literal of record with [\"op\" as text, \"cell\" as text] with value record with [\"op\" is (\"read\"), \"cell\" is (\"not_a_cell\")]",
        "record with [\"op\" as text, \"cell\" as text]",
    );
    assert_eq!(
        literal["body"]["expressions"]["review_input"]["body"]["op"],
        "literal"
    );
    assert_eq!(
        literal["body"]["expressions"]["review_input"]["reads"],
        json!({})
    );
    let conditional = expression_candidate(
        "if ((the value of \"Audit basis\") equals (0)) then (the value of \"Request\") otherwise (the value of \"Request\")",
        "text",
    );
    assert_eq!(
        conditional["body"]["expressions"]["review_input"]["reads"],
        json!({"review_request":true,"review_audit":true})
    );
}

#[test]
fn decimals_use_exact_signed_coefficients_and_refuse_negative_zero_and_overflow() {
    for (spelling, places, coefficient) in [
        ("0.05", 2, "5"),
        ("-0.05", 2, "-5"),
        ("0.00", 2, "0"),
        ("-92233720368547758.08", 2, "-9223372036854775808"),
    ] {
        let candidate = expression_candidate(
            &format!("decimal {spelling} with {places} places"),
            &format!("decimal with {places} places"),
        );
        assert_eq!(
            candidate["body"]["expressions"]["review_input"]["body"]["value"],
            coefficient
        );
    }
    for expression in [
        "decimal -0.00 with 2 places",
        "decimal 1.2 with 2 places",
        "decimal 92233720368547758.08 with 2 places",
    ] {
        let source = REVIEW_SOURCE.replace(
            "mean (the value of \"Request\")",
            &format!("mean ({expression})"),
        );
        assert!(
            Package::parse(source.as_bytes(), REVIEW_LEDGER)
                .unwrap()
                .lower(&resources())
                .is_err()
        );
    }
}

fn child_fixture(
    source: &str,
    bindings: &mut Value,
    name: &str,
    id: &str,
    parent_name: &str,
    parent_id: &str,
) -> String {
    bindings["declarations"].as_array_mut().unwrap().extend([
        json!({"id":id,"kind":"scope","scope":parent_id,"name":name}),
        json!({"id":format!("{id}_done"),"kind":"node","scope":id,"name":"Child done"}),
    ]);
    bindings["symbols"].as_array_mut().unwrap().push(json!({"id":format!("{id}_complete"),"owner":id,"role":"outcome","name":"complete","wire":"complete"}));
    source.replace("End process.", &format!("Scope \"{name}\" within \"{parent_name}\". Start at \"Child done\". Inputs empty. Outputs empty. Outcomes [\"complete\"]. Use cancellation \"Stop\", faults \"Fault handling\", closure \"Close\" and races \"Commit order\". Step \"Child done\". Finish as \"complete\". End step. End scope. End process."))
}

fn flow(bindings: &mut Value, owner: &str, name: &str, wire: &str) {
    bindings["symbols"].as_array_mut().unwrap().push(json!({"id":format!("{owner}_{wire}"),"owner":owner,"role":"outcome","name":name,"wire":wire}));
    bindings["generated"].as_array_mut().unwrap().push(json!({"id":format!("{owner}_{wire}_flow"),"owner":owner,"role":"flow","key":format!("{owner}_{wire}")}));
}

#[test]
fn invocation_repeat_and_qualified_descendant_invalidation_keep_explicit_scope_ids() {
    for action in [
        "Run \"Child\" with inputs empty and outputs empty.",
        "While \"Continue\" is true, repeat \"Child\".",
    ] {
        let mut b = ledger();
        let source = child_fixture(
            SOURCE,
            &mut b,
            "Child",
            "child",
            "Terminal example",
            "scope_terminal",
        );
        b["declarations"].as_array_mut().unwrap().extend([
            json!({"id":"end_node","kind":"node","scope":"scope_terminal","name":"End"}),
            json!({"id":"condition","kind":"expression","scope":"scope_terminal","name":"Continue"})]);
        flow(&mut b, "node_done", "complete", "complete");
        let source = source.replacen("Finish as \"complete\".", &format!("{action} On \"complete\", continue with \"End\"."), 1).replace("End process.", "Let \"Continue\" mean (false) as truth value with parameters empty. Step \"End\". Finish as \"complete\". End step. End process.");
        let c = Package::parse(source.as_bytes(), &bytes(&b))
            .unwrap()
            .lower(&resources())
            .unwrap();
        assert_eq!(c.document()["body"]["nodes"]["node_done"]["body"], "child");
    }
    let mut b: Value = serde_json::from_slice(REVIEW_LEDGER).unwrap();
    let source = child_fixture(
        REVIEW_SOURCE,
        &mut b,
        "Child",
        "child",
        "Review example",
        "scope_review",
    );
    let source = source.replacen(
        "Invalidate [\"Review\"].",
        "Invalidate [\"Child done\" in \"Child\" in \"Review example\"].",
        1,
    );
    let candidate = Package::parse(source.as_bytes(), &bytes(&b))
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert_eq!(
        candidate.document()["body"]["data"]["review_request"]["invalidates"],
        json!({"child_done":true})
    );
    let missing_chain = source.replace(
        "\"Child done\" in \"Child\" in \"Review example\"",
        "\"Child done\" in \"Child\"",
    );
    assert!(
        Package::parse(missing_chain.as_bytes(), &bytes(&b))
            .unwrap()
            .lower(&resources())
            .is_err()
    );
}

#[test]
fn split_join_refuses_population_wide_wire_collisions_unless_explicitly_selected() {
    let mut b = ledger();
    let source = child_fixture(
        SOURCE,
        &mut b,
        "Left",
        "left",
        "Terminal example",
        "scope_terminal",
    );
    let source = child_fixture(
        &source,
        &mut b,
        "Right",
        "right",
        "Terminal example",
        "scope_terminal",
    );
    for (id, name) in [("gather", "Gather"), ("end_node", "End")] {
        b["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"kind":"node","scope":"scope_terminal","name":name}));
    }
    for (id, name) in [("left_branch", "left"), ("right_branch", "right")] {
        b["symbols"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"owner":"node_done","role":"branch","name":name,"wire":name}));
    }
    flow(&mut b, "gather", "gathered", "gathered");
    let source = source.replacen("Finish as \"complete\".", "Start together [\"left\" runs \"Left\", \"right\" runs \"Right\"] and gather at \"Gather\".", 1).replace("End process.", "Step \"Gather\". Gather from \"Done\" when all report [outcome \"complete\" of \"Left\", outcome \"complete\" of \"Right\"]; for unfinished work use \"Settle\". On \"gathered\", continue with \"End\". End step. Step \"End\". Finish as \"complete\". End step. End process.");
    let candidate = Package::parse(source.as_bytes(), &bytes(&b))
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert_eq!(
        candidate.document()["body"]["nodes"]["gather"]["predicate"]["outcomes"],
        json!({"complete":true})
    );
    let ambiguous = source.replace(", outcome \"complete\" of \"Right\"", "");
    assert_eq!(
        Package::parse(ambiguous.as_bytes(), &bytes(&b))
            .unwrap()
            .lower(&resources())
            .unwrap_err()
            .code,
        "join-wire-collision"
    );
}

#[test]
fn wrong_policy_variants_cycles_and_reference_only_symbols_refuse() {
    let wrong = SOURCE.replace("Use cancellation \"Stop\"", "Use cancellation \"Settle\"");
    assert_eq!(
        Package::parse(wrong.as_bytes(), LEDGER)
            .unwrap()
            .lower(&resources())
            .unwrap_err()
            .code,
        "policy-kind"
    );
    let cycle = SOURCE.replace("For children use \"Settle\"", "For children use \"Stop\"");
    assert_eq!(
        Package::parse(cycle.as_bytes(), LEDGER)
            .unwrap()
            .lower(&resources())
            .unwrap_err()
            .code,
        "policy-cycle"
    );
    let mut b: Value = serde_json::from_slice(REVIEW_LEDGER).unwrap();
    b["symbols"].as_array_mut().unwrap().push(json!({"id":"not_declared","owner":"review_step","role":"outcome","name":"not declared","wire":"not_declared"}));
    let source = REVIEW_SOURCE.replace(
        "mean (the value of \"Request\")",
        "mean (outcome \"not declared\" of \"Review\")",
    );
    assert_eq!(
        Package::parse(source.as_bytes(), &bytes(&b))
            .unwrap()
            .lower(&resources())
            .unwrap_err()
            .code,
        "source-ledger-inventory"
    );
    let policy = REVIEW_SOURCE.replace(
        "Permit when (true).",
        "Permit when ((the value of \"Audit basis\") equals (0)).",
    );
    let candidate = Package::parse(policy.as_bytes(), REVIEW_LEDGER)
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert_eq!(
        candidate.document()["body"]["policies"]["review_work"]["body"]["reads"],
        json!({"review_audit":true})
    );
}

#[test]
fn arbitrary_bytes_and_source_mutations_return_without_panicking() {
    for n in 0..=255 {
        let bytes = [n, b'"', b'\\', n];
        assert!(std::panic::catch_unwind(|| parse(&bytes)).is_ok());
    }
    for i in 0..SOURCE.len() {
        let mut source = SOURCE.as_bytes().to_vec();
        source[i] = b'!';
        assert!(
            std::panic::catch_unwind(|| {
                if let Ok(package) = Package::parse(&source, LEDGER) {
                    let _ = package.lower(&resources());
                }
            })
            .is_ok(),
            "mutation at {i}"
        );
    }
}

#[test]
fn decision_and_untimed_wait_use_explicit_outcomes_and_wait_owners() {
    for action in [
        "Choose by [\"observed\" when \"Condition\"]; otherwise none.",
        "Wait under \"Observation\".",
    ] {
        let mut b = ledger();
        b["declarations"].as_array_mut().unwrap().extend([
            json!({"id":"end_node","kind":"node","scope":"scope_terminal","name":"End"}),
            json!({"id":"condition","kind":"expression","scope":"scope_terminal","name":"Condition"}),
            json!({"id":"wait_policy","kind":"policy","scope":"scope_terminal","name":"Observation"})]);
        flow(&mut b, "node_done", "observed", "observed");
        b["declarations"].as_array_mut().unwrap().push(
            json!({"id":"effect_policy","kind":"policy","scope":"scope_terminal","name":"Effect"}),
        );
        let source = SOURCE.replacen("Finish as \"complete\".", &format!("{action} On \"observed\", continue with \"End\"."), 1).replace("End process.", "Let \"Condition\" mean (true) as truth value with parameters empty. Rule for wait \"Observation\". Observe text. Match when (true). Authority \"Reviewer\". Timer none. On observation outcome \"observed\" of \"Done\". On timer none. End policy. Step \"End\". Finish as \"complete\". End step. End process.");
        let source = source.replace("End process.", "Rule for effect \"Effect\". Effect is read-only. Idempotency is stable-key. Reconcile under \"Observation\". End policy. End process.");
        let candidate = Package::parse(source.as_bytes(), &bytes(&b))
            .unwrap()
            .lower(&resources())
            .unwrap();
        assert_eq!(
            candidate.document()["body"]["policies"]["wait_policy"]["body"]["onObservation"],
            "observed"
        );
        assert_eq!(
            candidate.document()["body"]["policies"]["effect_policy"]["body"]["class"],
            "read-only"
        );
        assert_eq!(
            candidate.document()["body"]["policies"]["effect_policy"]["body"]["idempotency"],
            "stable-key"
        );
        let wrong_reconciliation = source.replace(
            "Reconcile under \"Observation\"",
            "Reconcile under \"Settle\"",
        );
        assert_eq!(
            Package::parse(wrong_reconciliation.as_bytes(), &bytes(&b))
                .unwrap()
                .lower(&resources())
                .unwrap_err()
                .code,
            "policy-kind"
        );
        if action.starts_with("Wait") {
            let wrong_owner = source.replace("of \"Done\"", "of \"Terminal example\"");
            assert!(
                Package::parse(wrong_owner.as_bytes(), &bytes(&b))
                    .unwrap()
                    .lower(&resources())
                    .is_err()
            );
        }
    }
}

#[test]
fn transfer_can_name_the_root_ancestor_but_ports_cannot_borrow_ancestor_cells() {
    let mut b: Value = serde_json::from_slice(REVIEW_LEDGER).unwrap();
    let source = child_fixture(
        REVIEW_SOURCE,
        &mut b,
        "Child",
        "child",
        "Review example",
        "scope_review",
    );
    b["declarations"].as_array_mut().unwrap().push(
        json!({"id":"child_settlement","kind":"policy","scope":"child","name":"Transfer work"}),
    );
    let source = source.replace("End scope.", "Rule for settlement \"Transfer work\". Unfinished work transfer to \"Review example\". End policy. End scope.");
    let candidate = Package::parse(source.as_bytes(), &bytes(&b))
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert_eq!(
        candidate.document()["body"]["policies"]["child_settlement"]["body"]["owner"],
        "scope_review"
    );
    let self_transfer = source.replace("transfer to \"Review example\"", "transfer to \"Child\"");
    assert_eq!(
        Package::parse(self_transfer.as_bytes(), &bytes(&b))
            .unwrap()
            .lower(&resources())
            .unwrap_err()
            .code,
        "settlement-ancestor"
    );
    b["symbols"].as_array_mut().unwrap().push(
        json!({"id":"child_input","owner":"child","role":"input","name":"input","wire":"input"}),
    );
    let borrowed = source.replace(
        "Start at \"Child done\". Inputs empty.",
        "Start at \"Child done\". Inputs [\"input\" uses \"Request\"].",
    );
    assert_eq!(
        Package::parse(borrowed.as_bytes(), &bytes(&b))
            .unwrap()
            .lower(&resources())
            .unwrap_err()
            .code,
        "port-ownership"
    );
}

#[test]
fn matching_external_hashes_do_not_grant_unsupported_lowering_authority() {
    let mut b = ledger();
    let payload = b"opaque clock contract";
    let revision = choreoform_ir_probe_core::digest(payload);
    b["declarations"].as_array_mut().unwrap().push(
        json!({"id":"clock_resource","kind":"resource","scope":"scope_terminal","name":"Clock"}),
    );
    b["resources"] = json!([{"binding":"clock_resource","kind":"clock","contract":"urn:test:clock","digest":revision}]);
    let r = resources();
    let supplied = [
        Resource {
            id: r[0].id,
            revision: r[0].revision,
            bytes: r[0].bytes,
        },
        Resource {
            id: r[1].id,
            revision: r[1].revision,
            bytes: r[1].bytes,
        },
        Resource {
            id: "urn:test:clock",
            revision: &revision,
            bytes: payload,
        },
    ];
    let package = Package::parse(SOURCE.as_bytes(), &bytes(&b)).unwrap();
    package.ledger().verify_resources(&supplied).unwrap();
    assert_eq!(
        package.lower(&supplied).unwrap_err().code,
        "external-contract-lowering-unsupported"
    );
}

#[test]
fn fanout_item_alias_resolves_to_the_child_owned_cell() {
    let mut b = ledger();
    let source = child_fixture(
        SOURCE,
        &mut b,
        "Item scope",
        "child",
        "Terminal example",
        "scope_terminal",
    );
    for (id, kind, scope, name) in [
        ("gather", "node", "scope_terminal", "Gather"),
        ("end_node", "node", "scope_terminal", "End"),
        ("collection", "expression", "scope_terminal", "Items"),
        ("key", "expression", "scope_terminal", "Key"),
        ("seal", "expression", "scope_terminal", "Sealed"),
        ("membership", "policy", "scope_terminal", "Changes"),
        ("protection", "policy", "scope_terminal", "Protection"),
        ("item_cell", "data", "child", "Item value"),
    ] {
        b["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"kind":kind,"scope":scope,"name":name}));
    }
    b["symbols"].as_array_mut().unwrap().extend([
        json!({"id":"item_input","owner":"child","role":"input","name":"item","wire":"item"}),
        json!({"id":"item_purpose","owner":"item_cell","role":"purpose","name":"Process item","wire":"process_item"})]);
    flow(&mut b, "gather", "gathered", "gathered");
    let source = source.replace("Start at \"Child done\". Inputs empty.", "Start at \"Child done\". Inputs [\"item\" uses \"Item value\"].").replacen("Finish as \"complete\".", "For each item of \"Items\", using key \"Key\", run \"Item scope\" with item \"item\"; seal when \"Sealed\" is true; for changes use \"Changes\"; gather at \"Gather\".", 1).replace("End scope.", "Keep \"Item value\" as text under \"Protection\". Sensitivity \"internal\". Purposes [\"Process item\"]. Participants [\"Reviewer\"]. Capabilities empty. Invalidate empty. Initially unavailable. End data. End scope.").replace("End process.", "Let \"Items\" mean (list of text containing [(\"one\")]) as list of text with parameters empty. Let \"Key\" mean (the parameter \"item\") as text with parameters [\"item\" as text]. Let \"Sealed\" mean (true) as truth value with parameters empty. Rule for membership \"Changes\". For removal use \"Settle\". Changed items invalidate. End policy. Rule for protection \"Protection\". Permit access when (true). End policy. Step \"Gather\". Gather from \"Done\" when all report [outcome \"complete\" of \"Item scope\"]; for unfinished work use \"Settle\". On \"gathered\", continue with \"End\". End step. Step \"End\". Finish as \"complete\". End step. End process.");
    let candidate = Package::parse(source.as_bytes(), &bytes(&b))
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert_eq!(
        candidate.document()["body"]["nodes"]["node_done"]["item"],
        "item_cell"
    );
    assert_eq!(
        candidate.document()["body"]["nodes"]["node_done"]["changes"],
        "membership"
    );
    let renamed = source.replace("with item \"item\"", "with item \"missing\"");
    assert_eq!(
        Package::parse(renamed.as_bytes(), &bytes(&b))
            .unwrap()
            .lower(&resources())
            .unwrap_err()
            .code,
        "symbol-mismatch"
    );
}

#[test]
fn nested_literal_decimal_markers_cannot_change_coefficient_meaning() {
    for (value, ty) in [
        ("decimal 0.05 with 2 places", "decimal with 2 places"),
        (
            "record with [\"amount\" is (decimal 0.05 with 2 places)]",
            "record with [\"amount\" as decimal with 2 places]",
        ),
        (
            "list containing [(decimal 0.05 with 2 places)]",
            "list of decimal with 2 places",
        ),
        (
            "present (decimal 0.05 with 2 places)",
            "optional decimal with 2 places",
        ),
        (
            "case \"amount\" containing (decimal 0.05 with 2 places)",
            "choice with [\"amount\" as decimal with 2 places]",
        ),
    ] {
        let expr = format!("literal of {ty} with value {value}");
        let _ = expression_candidate(&expr, ty);
        let expr = format!(
            "literal of {ty} with value {}",
            value
                .replace("2 places", "3 places")
                .replace("0.05", "0.005")
        );
        let source =
            REVIEW_SOURCE.replace("mean (the value of \"Request\")", &format!("mean ({expr})"));
        assert_eq!(
            Package::parse(source.as_bytes(), REVIEW_LEDGER)
                .unwrap()
                .lower(&resources())
                .unwrap_err()
                .code,
            "decimal-scale"
        );
    }
}

#[test]
fn source_identity_order_is_not_semantics_but_work_instructions_are() {
    let original = Package::parse(REVIEW_SOURCE.as_bytes(), REVIEW_LEDGER)
        .unwrap()
        .lower(&resources())
        .unwrap();
    let mut b: Value = serde_json::from_slice(REVIEW_LEDGER).unwrap();
    for field in ["declarations", "symbols", "generated"] {
        b[field].as_array_mut().unwrap().reverse();
    }
    let reordered = Package::parse(REVIEW_SOURCE.as_bytes(), &bytes(&b))
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert_eq!(reordered.document(), original.document());
    assert_ne!(reordered.bindings_digest, original.bindings_digest);
    let changed = REVIEW_SOURCE.replace(
        "Instructions \"Review the current request and record the decision.\"",
        "Instructions \"Compare the synthetic request.\"",
    );
    assert_ne!(changed, REVIEW_SOURCE);
    let changed = Package::parse(changed.as_bytes(), REVIEW_LEDGER)
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert_ne!(
        changed.document()["revision"],
        original.document()["revision"]
    );
}

#[test]
fn depth_token_cycles_shadowing_and_ledger_fields_fail_closed() {
    let deep = format!("{}true{}", "not (".repeat(65), ")".repeat(65));
    assert_eq!(
        parse_fragment(deep.as_bytes(), "expression")
            .unwrap_err()
            .code,
        "depth"
    );
    assert_eq!(
        parse_fragment("true ".repeat(4097).as_bytes(), "expression")
            .unwrap_err()
            .code,
        "tokens"
    );
    let cycle = REVIEW_SOURCE.replace("continue with \"Done\"", "continue with \"Review\"");
    assert_eq!(
        Package::parse(cycle.as_bytes(), REVIEW_LEDGER)
            .unwrap()
            .lower(&resources())
            .unwrap_err()
            .code,
        "flow-cycle"
    );
    let mut b = ledger();
    let _ = child_fixture(
        SOURCE,
        &mut b,
        "Child",
        "child",
        "Terminal example",
        "scope_terminal",
    );
    b["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"shadow","kind":"data","scope":"child","name":"Reviewer"}));
    assert_eq!(Ledger::decode(&bytes(&b)).unwrap_err().code, "shadowing");
    b = ledger();
    b["unknown"] = true.into();
    assert_eq!(
        Ledger::decode(&bytes(&b)).unwrap_err().code,
        "ledger-fields"
    );
}
