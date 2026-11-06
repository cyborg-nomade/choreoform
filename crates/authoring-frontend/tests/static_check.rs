// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use choreoform_authoring_frontend::Package;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource, digest};
use choreoform_semantic_check::{CORE, SEMANTICS, check, check_control, check_protection};
use serde_json::{Value, json};

#[path = "static/diagnostics.rs"]
mod diagnostics;
#[path = "static/settlement.rs"]
mod settlement;

const SOURCE: &[u8] = include_bytes!("../../../docs/authoring/examples/review.choreo");
const LEDGER: &[u8] = include_bytes!("../../../docs/authoring/examples/review.bindings.json");
const SERVICE: &[u8] = include_bytes!("../../../docs/authoring/examples/service.choreo");
const SERVICE_LEDGER: &[u8] =
    include_bytes!("../../../docs/authoring/examples/service.bindings.json");
const PROVIDER: &[u8] = include_bytes!("../../../docs/authoring/examples/service.provider.json");
const NAMED: &[u8] = include_bytes!("../../../docs/authoring/examples/service.named-types.json");
fn resources() -> Vec<Resource<'static>> {
    vec![
        Resource {
            id: SEMANTICS.0,
            revision: SEMANTICS.1,
            bytes: include_bytes!(
                "../../../docs/ir/contracts/sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt"
            ),
        },
        Resource {
            id: CORE.0,
            revision: CORE.1,
            bytes: include_bytes!(
                "../../../docs/ir/contracts/sha256-26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58.txt"
            ),
        },
    ]
}
fn service_resources() -> Vec<Resource<'static>> {
    let mut r = resources();
    r.extend([
        Resource {
            id: "urn:choreoform:examples:review-provider:0.1.0",
            revision: "sha256:4b48fb8169aa6959702579a1fcad831db3aaf0668be2de93d02ac6404e0c5835",
            bytes: PROVIDER,
        },
        Resource {
            id: "urn:choreoform:examples:number-types:0.1.0",
            revision: "sha256:f6bd253b88ec8732955e50cc0d9ac3ac0cff3a8975f61878075cf6eff1f0e644",
            bytes: NAMED,
        },
    ]);
    r
}
fn review() -> Value {
    Package::parse(SOURCE, LEDGER)
        .unwrap()
        .lower(&resources())
        .unwrap()
        .into_document()
}
fn envelope(body: Value) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts(body, json!({"unknown:retain":[true,null,7]})).unwrap()
}
fn result(body: Value) -> std::result::Result<(), String> {
    check(&envelope(body), &resources())
        .map(|_| ())
        .map_err(|e| e.code.into())
}

#[test]
fn static_report_is_explicitly_not_a_planning_or_execution_capability() {
    let package = Package::parse(SOURCE, LEDGER).unwrap();
    let c = package.lower(&resources()).unwrap();
    let before = c.document().clone();
    let report = c.check_static(&resources()).unwrap();
    assert_eq!(report.revision(), before["revision"].as_str().unwrap());
    assert_eq!(report.expressions(), 3);
    assert_eq!(report.policies(), 8);
    assert!(!report.remaining().is_empty());
    assert_eq!(c.document(), &before);
    let mut forged = before;
    forged["revision"] = format!("sha256:{}", "0".repeat(64)).into();
    let raw = serde_json::to_vec(&forged).unwrap();
    assert_eq!(
        check(&DefinitionEnvelope::decode(&raw).unwrap(), &resources())
            .unwrap_err()
            .code,
        "revision"
    );
}

#[test]
fn closed_payload_mutations_are_rejected_independently() {
    for (mutation, expected) in [
        ("result", "expression-result"),
        ("missing-read", "expression-reads"),
        ("extra-read", "expression-reads"),
        ("parameter", "parameter-binding"),
        ("dead-branch", "expression-type"),
        ("ordinary-fact", "policy-fact"),
        ("actor-fact", "policy-fact"),
        ("protection-fact", "expression-type"),
        ("predicate", "predicate-type"),
        ("policy-field", "fields"),
        ("retry-count", "retry-delays"),
        ("work-result-fact", "expression-type"),
        ("timer", "fields"),
        ("role", "dialect-contract"),
        ("semantic-role", "semantic-contract"),
    ] {
        let mut b = review()["body"].clone();
        let int = json!({"op":"literal","type":{"kind":"integer"},"value":"0"});
        let boolean = json!({"op":"literal","type":{"kind":"boolean"},"value":true});
        match mutation {
            "result" => {
                b["expressions"]["review_input"]["resultType"]["body"] = json!({"kind":"integer"})
            }
            "missing-read" => b["expressions"]["review_input"]["reads"] = json!({}),
            "extra-read" => b["expressions"]["review_input"]["reads"]["review_audit"] = true.into(),
            "parameter" => {
                b["expressions"]["review_input"]["parameters"]["x"] =
                    json!({"dialect":"core","body":{"kind":"text"}})
            }
            "dead-branch" => {
                b["expressions"]["review_input"]["body"] = json!({"op":"if","condition":boolean,"then":{"op":"read","cell":"review_request"},"else":{"op":"add","left":int,"right":boolean}})
            }
            "ordinary-fact" => {
                b["expressions"]["review_input"]["body"] = json!({"op":"fact","name":"principal"})
            }
            "actor-fact" => {
                b["policies"]["review_actor_policy"]["body"]["permit"] =
                    json!({"op":"fact","name":"purpose"})
            }
            "protection-fact" => {
                b["policies"]["review_protection"]["body"]["permit"] =
                    json!({"op":"and","left":{"op":"fact","name":"purpose"},"right":boolean})
            }
            "predicate" => b["policies"]["review_actor_policy"]["body"]["permit"] = int,
            "policy-field" => {
                b["policies"]["review_actor_policy"]["body"]["callback"] =
                    "secret-value-never-echo".into()
            }
            "retry-count" => {
                b["policies"]["review_work"]["body"]["retry"]["maxAttempts"] = 2.into()
            }
            "work-result-fact" => {
                b["policies"]["review_work"]["body"]["completion"] =
                    json!({"op":"eq","left":{"op":"fact","name":"result"},"right":boolean})
            }
            "timer" => {
                b["policies"]["review_work"]["body"]["timeout"] =
                    json!({"clock":"opaque-unsupported"})
            }
            "role" => b["dialects"]["core"] = json!({"id":SEMANTICS.0,"revision":SEMANTICS.1}),
            "semantic-role" => b["semantics"] = json!({"id":CORE.0,"revision":CORE.1}),
            _ => unreachable!(),
        }
        assert_eq!(result(b).unwrap_err(), expected, "{mutation}");
    }
}

#[test]
fn reference_types_dependencies_and_direct_information_flow_are_checked() {
    for (mutation, expected) in [
        ("assignment", "assignment-type"),
        ("node-read", "node-reads"),
        ("human-outcomes", "human-outcomes"),
        ("policy-kind", "policy-kind"),
        ("information-flow", "information-flow"),
        ("visibility", "cell-reference"),
        ("policy-reads", "policy-reads"),
    ] {
        let mut b = review()["body"].clone();
        match mutation {
            "assignment" => {
                b["nodes"]["review_step"] = json!({"kind":"compute","scope":"scope_review","outcomes":{"approved":true},"reads":{"review_request":true},"writes":{"review_audit":true},"assignments":{"review_audit":"review_input"}});
                b["flows"]
                    .as_object_mut()
                    .unwrap()
                    .remove("review_flow_correction");
            }
            "node-read" => b["nodes"]["review_step"]["reads"]
                .as_object_mut()
                .unwrap()
                .clear(),
            "human-outcomes" => {
                b["nodes"]["review_step"]["outcomes"]
                    .as_object_mut()
                    .unwrap()
                    .remove("correction_needed");
                b["flows"]
                    .as_object_mut()
                    .unwrap()
                    .remove("review_flow_correction");
            }
            "policy-kind" => b["scopes"]["scope_review"]["cancellation"] = "review_settle".into(),
            "information-flow" => {
                b["data"]["review_decision"]["protection"]["sensitivity"] = "public".into()
            }
            "policy-reads" => {
                b["policies"]["review_actor_policy"]["body"]["reads"]["review_audit"] = true.into()
            }
            "visibility" => {
                let mut child = b["scopes"]["scope_review"].clone();
                child["parent"] = "scope_review".into();
                child["entry"] = "child_done".into();
                child["outputs"] = json!({});
                b["scopes"]["child"] = child;
                b["nodes"]["child_done"] = json!({"kind":"finish","scope":"child","outcomes":{},"reads":{},"writes":{},"outcome":"complete"});
                b["data"]["review_request"]["scope"] = "child".into();
            }
            _ => unreachable!(),
        }
        assert_eq!(result(b).unwrap_err(), expected, "{mutation}");
    }
}

#[test]
fn provider_and_nominal_source_package_checks_without_implicit_ids_or_host_support() {
    let supplied = service_resources();
    let package = Package::parse(SERVICE, SERVICE_LEDGER).unwrap();
    let candidate = package.lower(&supplied).unwrap();
    let report = candidate.check_static(&supplied).unwrap();
    assert_eq!(report.policies(), 11);
    let before = candidate.document().clone();
    assert_eq!(
        before["body"]["data"]["service_audit"]["type"]["body"]["kind"],
        "named"
    );
    assert_eq!(
        before["body"]["policies"]["service_generated_provider"]["body"]["provider"]["revision"],
        supplied[2].revision
    );
    let (source, companion) = package.formatted().unwrap();
    let again = Package::parse(source.as_bytes(), companion.as_bytes())
        .unwrap()
        .lower(&supplied)
        .unwrap();
    assert_eq!(again.document(), &before);
    again.check_static(&supplied).unwrap();
    let mut body = before["body"].clone();
    body["capabilities"]["service_capability"]["input"]["body"] = json!({"kind":"integer"});
    assert_eq!(
        check(&envelope(body), &supplied).unwrap_err().code,
        "capability-type"
    );
    let mut body = before["body"].clone();
    body["policies"]["service_effect"]["body"]["idempotency"] = "stable-key".into();
    assert_eq!(
        check(&envelope(body), &supplied).unwrap_err().code,
        "capability-effect"
    );
    let mut body = before["body"].clone();
    body["nodes"]["service_step"]["result"] = Value::Null;
    body["nodes"]["service_step"]["writes"] = json!({});
    // Null discards exported data; it does not redefine the provider's output.
    check(&envelope(body), &supplied).unwrap();
    let mut body = before["body"].clone();
    body["policies"]["service_reconciliation"]["body"]["match"] =
        json!({"op":"fact","name":"payload"});
    assert_eq!(
        check(&envelope(body), &supplied).unwrap_err().code,
        "predicate-type"
    );
    let bad = String::from_utf8(SERVICE.to_vec())
        .unwrap()
        .replace("with value 0", "with value -1");
    let bad = Package::parse(bad.as_bytes(), SERVICE_LEDGER)
        .unwrap()
        .lower(&supplied)
        .unwrap();
    assert_eq!(
        bad.check_static(&supplied).unwrap_err().code,
        "invalid-nominal-value"
    );
}

#[test]
fn external_protection_dependencies_are_derived_not_assumed_empty() {
    // Provider-only policy identity, deliberately not attached to the source cells.
    let mut ledger: Value = serde_json::from_slice(SERVICE_LEDGER).unwrap();
    ledger["declarations"].as_array_mut().unwrap().push(json!({"id":"service_external_protection","kind":"policy","scope":"scope_service","name":"Provider protection"}));
    let source=String::from_utf8(SERVICE.to_vec()).unwrap().replace("End process.","Rule for protection \"Provider protection\". Permit access when ((the value of \"Audit basis\") equals (the value of \"Audit basis\")). End policy. End process.").replace("Also read [\"Audit basis\"].","");
    let mut provider: Value = serde_json::from_slice(PROVIDER).unwrap();
    provider["resultProtection"]["policy"] = "service_external_protection".into();
    let bytes = serde_json::to_vec(&provider).unwrap();
    let revision = digest(&bytes);
    ledger["resources"][0]["digest"] = revision.clone().into();
    let mut supplied = service_resources();
    supplied[2] = Resource {
        id: "urn:choreoform:examples:review-provider:0.1.0",
        revision: &revision,
        bytes: &bytes,
    };
    let candidate = Package::parse(source.as_bytes(), &serde_json::to_vec(&ledger).unwrap())
        .unwrap()
        .lower(&supplied)
        .unwrap();
    let b = &candidate.document()["body"];
    assert_eq!(
        b["policies"]["service_generated_provider"]["body"]["reads"],
        json!({"service_audit":true})
    );
    assert!(b["nodes"]["service_step"]["reads"]["service_audit"] == true);
    // The deliberately changed provider/result policy identities prevent transfer.
    assert_eq!(
        candidate.check_static(&supplied).unwrap_err().code,
        "information-flow"
    );
    let mut body = b.clone();
    body["policies"]["service_generated_provider"]["body"]["reads"] = json!({});
    assert_eq!(
        check(&envelope(body), &supplied).unwrap_err().code,
        "policy-reads"
    );
    provider["resultProtection"]["capabilities"] = json!({"service_capability":true});
    let bytes = serde_json::to_vec(&provider).unwrap();
    let revision = digest(&bytes);
    ledger["resources"][0]["digest"] = revision.clone().into();
    supplied[2] = Resource {
        id: "urn:choreoform:examples:review-provider:0.1.0",
        revision: &revision,
        bytes: &bytes,
    };
    assert_eq!(
        Package::parse(source.as_bytes(), &serde_json::to_vec(&ledger).unwrap())
            .unwrap()
            .lower(&supplied)
            .unwrap_err()
            .code,
        "policy-cycle"
    );
    let mut body = b.clone();
    body["policies"]["service_generated_provider"]["body"]["provider"]["revision"] =
        revision.clone().into();
    assert_eq!(
        check(&envelope(body), &supplied).unwrap_err().code,
        "policy-cycle"
    );
}

#[test]
fn policy_and_data_access_dependency_closure_is_preserved() {
    let source=String::from_utf8(SOURCE.to_vec()).unwrap().replace("Also read [\"Audit basis\"].","").replace("Permit access when ((the fact \"purpose\") equals (purpose \"Expense review\" of \"Request\")) .","Permit access when ((the value of \"Audit basis\") equals (the value of \"Audit basis\")).");
    let candidate = Package::parse(source.as_bytes(), LEDGER)
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert!(candidate.document()["body"]["nodes"]["review_step"]["reads"]["review_audit"] == true);
    candidate.check_static(&resources()).unwrap();
    let source = String::from_utf8(SOURCE.to_vec()).unwrap().replace(
        "Permit when (true).",
        "Permit when ((the value of \"Audit basis\") equals (0)).",
    );
    let candidate = Package::parse(source.as_bytes(), LEDGER)
        .unwrap()
        .lower(&resources())
        .unwrap();
    candidate.check_static(&resources()).unwrap();
    assert_eq!(
        candidate.document()["body"]["policies"]["review_work"]["body"]["reads"],
        json!({"review_audit":true})
    );
}

/// A concrete output transfer with no parent reads: the generic read/write
/// cross-product cannot substitute for checking this child-owned source cell.
fn invoke_output_body() -> Value {
    let mut body = Package::parse(SERVICE, SERVICE_LEDGER)
        .unwrap()
        .lower(&service_resources())
        .unwrap()
        .into_document()["body"]
        .clone();
    let mut child = body["scopes"]["scope_service"].clone();
    child["parent"] = "scope_service".into();
    child["entry"] = "child_done".into();
    child["outputs"] = json!({"decision":"child_decision"});
    body["scopes"]["child"] = child;
    let mut cell = body["data"]["service_decision"].clone();
    cell["scope"] = "child".into();
    cell["initial"] = "child_initial".into();
    body["expressions"]["child_initial"] = json!({"scope":"child","dialect":"core","resultType":cell["type"],"parameters":{},"reads":{},"body":{"op":"literal","type":cell["type"]["body"],"value":{"tag":"approved","value":{}}}});
    body["data"]["child_decision"] = cell;
    body["nodes"]["child_done"] = json!({"kind":"finish","scope":"child","outcomes":{},"reads":{},"writes":{},"outcome":"complete"});
    body["nodes"]["service_step"] = json!({"kind":"invoke","scope":"scope_service","outcomes":{"complete":true},"reads":{},"writes":{"service_decision":true},"body":"child","inputs":{},"outputs":{"decision":"service_decision"}});
    body["flows"]["service_flow_approved"]["outcome"] = "complete".into();
    body["flows"]
        .as_object_mut()
        .unwrap()
        .remove("service_flow_correction");
    body["actors"]["extra_actor"] =
        json!({"scope":"scope_service","requirement":"service_actor_policy"});
    body["policies"]["extra_protection"] = body["policies"]["service_protection"].clone();
    body
}

#[test]
fn invoke_outputs_reject_broader_destination_protection() {
    let body = invoke_output_body();
    check(&envelope(body.clone()), &service_resources()).unwrap();
    for (field, value) in [
        (
            "purposes",
            json!({"expense_review":true,"extra_purpose":true}),
        ),
        (
            "participants",
            json!({"service_actor":true,"extra_actor":true}),
        ),
        ("capabilities", json!({"service_capability":true})),
        ("sensitivity", json!("public")),
        ("policy", json!("extra_protection")),
    ] {
        let mut changed = body.clone();
        changed["data"]["service_decision"]["protection"][field] = value;
        let diagnostic = check(&envelope(changed), &service_resources()).unwrap_err();
        assert_eq!(diagnostic.code, "information-flow", "{field}");
        assert_eq!(diagnostic.path, "/body/nodes/service_step");
    }
}

#[test]
fn invoke_outputs_allow_equal_or_narrower_destination_protection() {
    let mut body = invoke_output_body();
    check(&envelope(body.clone()), &service_resources()).unwrap();
    let protection = &mut body["data"]["child_decision"]["protection"];
    protection["purposes"]["extra_purpose"] = true.into();
    protection["participants"]["extra_actor"] = true.into();
    protection["capabilities"]["service_capability"] = true.into();
    check(&envelope(body), &service_resources()).unwrap();
}

#[test]
fn invoke_outputs_check_every_port_and_preserve_binding_guards() {
    let mut body = invoke_output_body();
    let mut cell = body["data"]["service_request"].clone();
    cell["scope"] = "child".into();
    cell["invalidates"] = json!({});
    body["data"]["child_request"] = cell;
    body["scopes"]["child"]["outputs"]["request"] = "child_request".into();
    body["nodes"]["service_step"]["outputs"]["request"] = "service_request".into();
    body["nodes"]["service_step"]["writes"]["service_request"] = true.into();
    check(&envelope(body.clone()), &service_resources()).unwrap();
    let mut broad = body.clone();
    broad["data"]["service_request"]["protection"]["purposes"]["extra_purpose"] = true.into();
    assert_eq!(
        check(&envelope(broad), &service_resources())
            .unwrap_err()
            .code,
        "information-flow"
    );
    for (mutation, expected) in [
        ("write", "invoke-writes"),
        ("type", "invoke-type"),
        ("visibility", "visibility"),
    ] {
        let mut changed = body.clone();
        match mutation {
            "write" => {
                changed["nodes"]["service_step"]["writes"]
                    .as_object_mut()
                    .unwrap()
                    .remove("service_decision");
            }
            "type" => {
                changed["nodes"]["service_step"]["outputs"]["decision"] = "service_request".into();
            }
            "visibility" => {
                changed["nodes"]["service_step"]["outputs"]["decision"] = "child_decision".into();
                changed["nodes"]["service_step"]["writes"]["child_decision"] = true.into();
            }
            _ => unreachable!(),
        }
        assert_eq!(
            check(&envelope(changed), &service_resources())
                .unwrap_err()
                .code,
            expected,
            "{mutation}"
        );
    }
}

#[test]
fn source_invocation_output_bindings_preserve_protection_after_reparse() {
    let output_type = "choice with [outcome \"approved\" of \"Review\" as record with empty, outcome \"correction needed\" of \"Review\" as record with empty]";
    let child = format!(
        "Scope \"Child\" within \"Review example\". Start at \"Child done\". Inputs empty. Outputs [\"Decision\" uses \"Child decision\"]. Outcomes [\"approved\", \"correction needed\"]. Use cancellation \"Stop\", faults \"Fault handling\", closure \"Close\" and races \"Commit order\". Keep \"Child decision\" as {output_type} under \"Protection\". Sensitivity \"internal\". Purposes [\"Expense review\"]. Participants [\"Reviewer\"]. Capabilities empty. Invalidate empty. Initially (literal of {output_type} with value case outcome \"approved\" of \"Review\" containing (record with empty)). End data. Step \"Child done\". Finish as \"approved\". End step. End scope. End process."
    );
    let source = String::from_utf8(SOURCE.to_vec()).unwrap()
        .replace("Ask \"Reviewer\" to decide using \"Current request\" under \"Review work\" and record in \"Review decision\".", "Run \"Child\" with inputs empty and outputs [\"Decision\" uses \"Review decision\"].")
        .replace("End process.", &child);
    let mut ledger: Value = serde_json::from_slice(LEDGER).unwrap();
    ledger["declarations"].as_array_mut().unwrap().extend([
        json!({"id":"scope_child","kind":"scope","scope":"scope_review","name":"Child"}),
        json!({"id":"child_decision","kind":"data","scope":"scope_child","name":"Child decision"}),
        json!({"id":"child_done","kind":"node","scope":"scope_child","name":"Child done"}),
    ]);
    ledger["symbols"].as_array_mut().unwrap().extend([
        json!({"id":"child_output","owner":"scope_child","role":"output","name":"Decision","wire":"decision"}),
        json!({"id":"child_approved","owner":"scope_child","role":"outcome","name":"approved","wire":"approved"}),
        json!({"id":"child_correction","owner":"scope_child","role":"outcome","name":"correction needed","wire":"correction_needed"}),
        json!({"id":"child_purpose","owner":"child_decision","role":"purpose","name":"Expense review","wire":"expense_review"}),
    ]);
    ledger["generated"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"child_initial","owner":"child_decision","role":"initial","key":null}));
    let package = Package::parse(source.as_bytes(), &serde_json::to_vec(&ledger).unwrap()).unwrap();
    let candidate = package.lower(&resources()).unwrap();
    candidate.check_static(&resources()).unwrap();
    let (formatted, companion) = package.formatted().unwrap();
    let roundtrip = Package::parse(formatted.as_bytes(), companion.as_bytes())
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert_eq!(candidate.document(), roundtrip.document());
    roundtrip.check_static(&resources()).unwrap();
    // Change only the caller destination's purposes, not the child output's.
    let start = source.find("Keep \"Review decision\"").unwrap();
    let mut broader = source.clone();
    let purposes = start
        + broader[start..]
            .find("Purposes [\"Expense review\"].")
            .unwrap();
    broader.replace_range(
        purposes..purposes + "Purposes [\"Expense review\"].".len(),
        "Purposes [\"Expense review\", \"Wider access\"].",
    );
    ledger["symbols"].as_array_mut().unwrap().push(json!({"id":"caller_wider_purpose","owner":"review_decision","role":"purpose","name":"Wider access","wire":"wider_access"}));
    let candidate = Package::parse(broader.as_bytes(), &serde_json::to_vec(&ledger).unwrap())
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert_eq!(
        candidate.check_static(&resources()).unwrap_err().code,
        "information-flow"
    );
}

#[test]
fn protection_keeps_provider_result_labels_without_a_published_result_cell() {
    let mut b = Package::parse(SERVICE, SERVICE_LEDGER)
        .unwrap()
        .lower(&service_resources())
        .unwrap()
        .into_document()["body"]
        .clone();
    let mut provider: Value = serde_json::from_slice(PROVIDER).unwrap();
    provider["output"] = json!({"kind":"record","fields":{}});
    for obs in provider["observations"]
        .as_object_mut()
        .unwrap()
        .values_mut()
    {
        if obs["result"] != Value::Null {
            obs["result"] = json!({"kind":"record","fields":{}});
        }
    }
    provider["resultProtection"]["purposes"] = json!({"private":true});
    let bytes = serde_json::to_vec(&provider).unwrap();
    let revision = digest(&bytes);
    b["capabilities"]["service_capability"]["output"]["body"] =
        json!({"kind":"record","fields":{}});
    let policy = b["capabilities"]["service_capability"]["contract"]
        .as_str()
        .unwrap()
        .to_owned();
    b["policies"][policy]["body"]["provider"]["revision"] = revision.clone().into();
    b["nodes"]["service_step"]["result"] = Value::Null;
    b["nodes"]["service_step"]["writes"] = json!({});
    b["scopes"]["scope_service"]["outputs"] = json!({});
    b["nodes"]["publish_public"] = json!({"kind":"compute","scope":"scope_service","assignments":{"service_request":"service_initial_request"},"reads":{},"writes":{"service_request":true},"outcomes":{"next":true}});
    for flow in b["flows"].as_object_mut().unwrap().values_mut() {
        if flow["source"] == "service_step" {
            flow["target"] = "publish_public".into();
        }
    }
    b["flows"]["public_done"] =
        json!({"source":"publish_public","outcome":"next","target":"service_done"});
    let mut registry: Vec<Resource<'_>> = service_resources();
    registry.retain(|r| r.id != "urn:choreoform:examples:review-provider:0.1.0");
    registry.push(Resource {
        id: "urn:choreoform:examples:review-provider:0.1.0",
        revision: &revision,
        bytes: &bytes,
    });
    let e = envelope(b);
    check_control(&e, &registry).unwrap();
    assert_eq!(
        check_protection(&e, &registry).unwrap_err().code,
        "control-information-flow"
    );
}

fn child_input_source() -> (String, Vec<u8>) {
    let output_type = "choice with [outcome \"approved\" of \"Review\" as record with empty, outcome \"correction needed\" of \"Review\" as record with empty]";
    let addition = format!(
        "Rule for protection \"Input access\". Permit access when ((the value of \"Audit basis\") equals (0)). End policy. Scope \"Child\" within \"Review example\". Start at \"Child done\". Inputs [\"Argument\" uses \"Child input\"]. Outputs [\"Decision\" uses \"Child decision\"]. Outcomes [\"approved\", \"correction needed\"]. Use cancellation \"Stop\", faults \"Fault handling\", closure \"Close\" and races \"Commit order\". Keep \"Child input\" as text under \"Input access\". Sensitivity \"internal\". Purposes [\"Expense review\"]. Participants [\"Reviewer\"]. Capabilities empty. Invalidate empty. Initially unavailable. End data. Keep \"Child decision\" as {output_type} under \"Protection\". Sensitivity \"internal\". Purposes [\"Expense review\"]. Participants [\"Reviewer\"]. Capabilities empty. Invalidate empty. Initially (literal of {output_type} with value case outcome \"approved\" of \"Review\" containing (record with empty)). End data. Step \"Child done\". Finish as \"approved\". End step. End scope. End process."
    );
    let source=String::from_utf8(SOURCE.to_vec()).unwrap()
        .replace("Ask \"Reviewer\" to decide using \"Current request\" under \"Review work\" and record in \"Review decision\".","Run \"Child\" with inputs [\"Argument\" uses \"Current request\"] and outputs [\"Decision\" uses \"Review decision\"].")
        .replace("Also read [\"Audit basis\"].","").replace("End process.",&addition);
    let mut ledger: Value = serde_json::from_slice(LEDGER).unwrap();
    ledger["declarations"].as_array_mut().unwrap().extend([
        json!({"id":"input_access_policy","kind":"policy","scope":"scope_review","name":"Input access"}),
        json!({"id":"scope_child","kind":"scope","scope":"scope_review","name":"Child"}),
        json!({"id":"child_input","kind":"data","scope":"scope_child","name":"Child input"}),
        json!({"id":"child_decision","kind":"data","scope":"scope_child","name":"Child decision"}),
        json!({"id":"child_done","kind":"node","scope":"scope_child","name":"Child done"}),
    ]);
    ledger["symbols"].as_array_mut().unwrap().extend([
        json!({"id":"child_input_port","owner":"scope_child","role":"input","name":"Argument","wire":"request"}),
        json!({"id":"child_output_port","owner":"scope_child","role":"output","name":"Decision","wire":"decision"}),
        json!({"id":"child_approved","owner":"scope_child","role":"outcome","name":"approved","wire":"approved"}),
        json!({"id":"child_correction","owner":"scope_child","role":"outcome","name":"correction needed","wire":"correction_needed"}),
        json!({"id":"child_input_purpose","owner":"child_input","role":"purpose","name":"Expense review","wire":"expense_review"}),
        json!({"id":"child_output_purpose","owner":"child_decision","role":"purpose","name":"Expense review","wire":"expense_review"}),
    ]);
    ledger["generated"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"child_initial","owner":"child_decision","role":"initial","key":null}));
    let companion = serde_json::to_vec(&ledger).unwrap();
    (source, companion)
}

#[test]
fn source_derives_child_input_access_dependencies_without_reading_fresh_values() {
    let (source, companion) = child_input_source();
    let package = Package::parse(source.as_bytes(), &companion).unwrap();
    let candidate = package.lower(&resources()).unwrap();
    assert_eq!(
        candidate.document()["body"]["nodes"]["review_step"]["reads"],
        json!({"review_request":true,"review_audit":true})
    );
    candidate.check_static(&resources()).unwrap();
    candidate.check_control(&resources()).unwrap();
    let (formatted, bindings) = package.formatted().unwrap();
    let roundtrip = Package::parse(formatted.as_bytes(), bindings.as_bytes())
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert_eq!(candidate.document(), roundtrip.document());
    roundtrip.check_control(&resources()).unwrap();
}

#[test]
fn source_derives_initializer_value_and_destination_access_ancestor_reads() {
    for metadata in [false, true] {
        let (source, companion) = child_input_source();
        let mut source = source.replace(
            "Keep \"Child input\" as text under \"Input access\".",
            "Keep \"Child input\" as text under \"Protection\".",
        );
        let (protection, initial) = if metadata {
            ("Input access", "0")
        } else {
            source = source.replace(
                "Permit access when ((the value of \"Audit basis\") equals (0)).",
                "Permit access when (true).",
            );
            ("Protection", "the value of \"Audit basis\"")
        };
        source = source.replace("End scope. End process.", &format!(
            "Keep \"Child seed\" as whole number under \"{protection}\". Sensitivity \"internal\". Purposes [\"Expense review\"]. Participants [\"Reviewer\"]. Capabilities empty. Invalidate empty. Initially ({initial}). End data. End scope. End process."));
        let mut ledger: Value = serde_json::from_slice(&companion).unwrap();
        ledger["declarations"].as_array_mut().unwrap().push(
            json!({"id":"child_seed","kind":"data","scope":"scope_child","name":"Child seed"}),
        );
        ledger["symbols"].as_array_mut().unwrap().push(json!({"id":"child_seed_purpose","owner":"child_seed","role":"purpose","name":"Expense review","wire":"expense_review"}));
        ledger["generated"].as_array_mut().unwrap().push(
            json!({"id":"child_seed_initial","owner":"child_seed","role":"initial","key":null}),
        );
        let companion = serde_json::to_vec(&ledger).unwrap();
        let package = Package::parse(source.as_bytes(), &companion).unwrap();
        let candidate = package.lower(&resources()).unwrap();
        assert_eq!(
            candidate.document()["body"]["nodes"]["review_step"]["reads"],
            json!({"review_request":true,"review_audit":true})
        );
        candidate.check_control(&resources()).unwrap();
        let mut b = candidate.document()["body"].clone();
        b["nodes"]["review_step"]["reads"] = json!({"review_request":true});
        assert_eq!(
            check(&envelope(b), &resources()).unwrap_err().code,
            "node-reads"
        );
        let (formatted, bindings) = package.formatted().unwrap();
        let roundtrip = Package::parse(formatted.as_bytes(), bindings.as_bytes())
            .unwrap()
            .lower(&resources())
            .unwrap();
        assert_eq!(roundtrip.document(), candidate.document());
        roundtrip.check_control(&resources()).unwrap();
    }
}

#[test]
fn effect_reconciliation_requires_one_declared_wait_definition_site() {
    let b = Package::parse(SERVICE, SERVICE_LEDGER)
        .unwrap()
        .lower(&service_resources())
        .unwrap()
        .into_document()["body"]
        .clone();
    check(&envelope(b.clone()), &service_resources()).unwrap();
    let effect = b["capabilities"]["service_capability"]["effects"]
        .as_str()
        .unwrap();
    let wait = b["policies"][effect]["body"]["reconciliation"]
        .as_str()
        .unwrap();
    let owner = b["nodes"]
        .as_object()
        .unwrap()
        .iter()
        .find(|(_, n)| n["kind"] == "wait" && n["policy"] == wait)
        .unwrap()
        .0
        .clone();
    let mut absent = b.clone();
    absent["nodes"].as_object_mut().unwrap().remove(&owner);
    absent["flows"]
        .as_object_mut()
        .unwrap()
        .retain(|_, f| f["source"] != owner);
    let e = envelope(absent);
    let error = check(&e, &service_resources()).unwrap_err();
    assert_eq!(error.code, "reconciliation-owner-missing");
    assert!(e.document().pointer(&error.path).is_some());
    let mut ambiguous = b.clone();
    ambiguous["nodes"]["another_observer"] = ambiguous["nodes"][&owner].clone();
    ambiguous["flows"]["another_observation"] =
        json!({"source":"another_observer","outcome":"observed","target":"service_done"});
    let e = envelope(ambiguous);
    let error = check(&e, &service_resources()).unwrap_err();
    assert_eq!(error.code, "reconciliation-owner-unproved");
    assert_eq!(
        error.path,
        format!("/body/policies/{effect}/body/reconciliation")
    );
}

#[test]
fn concurrency_distinguishes_read_only_contracts_from_unproved_effect_overlap() {
    let mut b = Package::parse(SERVICE, SERVICE_LEDGER)
        .unwrap()
        .lower(&service_resources())
        .unwrap()
        .into_document()["body"]
        .clone();
    let root = "scope_service";
    b["scopes"][root]["outputs"] = json!({});
    b["scopes"][root]["entry"] = "fork".into();
    b["nodes"]["fork"] = json!({"kind":"split","scope":root,"children":{"a":"child_a","b":"child_b"},"join":"barrier","reads":{},"writes":{},"outcomes":{}});
    b["nodes"]["barrier"] = json!({"kind":"join","scope":root,"source":"fork","predicate":{"op":"all","outcomes":{"complete":true}},"remaining":"service_settle","reads":{},"writes":{},"outcomes":{"next":true}});
    b["flows"]["barrier_done"] =
        json!({"source":"barrier","outcome":"next","target":"service_done"});
    for child in ["child_a", "child_b"] {
        let work = format!("{child}_work");
        let done = format!("{child}_done");
        let result = format!("{child}_result");
        let mut scope = b["scopes"][root].clone();
        scope["parent"] = root.into();
        scope["entry"] = work.clone().into();
        b["scopes"][child] = scope;
        b["data"][&result] = b["data"]["service_decision"].clone();
        b["data"][&result]["scope"] = child.into();
        b["nodes"][&work] = b["nodes"]["service_step"].clone();
        b["nodes"][&work]["scope"] = child.into();
        b["nodes"][&work]["result"] = result.clone().into();
        b["nodes"][&work]["writes"] = json!({result:true});
        b["nodes"][&done] = json!({"kind":"finish","scope":child,"outcome":"complete","reads":{},"writes":{},"outcomes":{}});
        for outcome in ["approved", "correction_needed"] {
            b["flows"][format!("{child}_{outcome}")] =
                json!({"source":work,"outcome":outcome,"target":done});
        }
        for cell in ["service_request", "service_audit"] {
            b["data"][cell]["invalidates"][&work] = true.into();
        }
    }
    let report =
        choreoform_semantic_check::check_concurrency(&envelope(b.clone()), &service_resources())
            .unwrap();
    assert!(report.checked_forks().contains("fork"));
    // The host still owes truthful provider enforcement; the digest is not proof.
    for class in ["compensatable", "irreversible"] {
        let mut provider: Value = serde_json::from_slice(PROVIDER).unwrap();
        provider["effectClass"] = class.into();
        let bytes = serde_json::to_vec(&provider).unwrap();
        let revision = digest(&bytes);
        let mut variant = b.clone();
        let effects = variant["capabilities"]["service_capability"]["effects"]
            .as_str()
            .unwrap()
            .to_owned();
        let contract = variant["capabilities"]["service_capability"]["contract"]
            .as_str()
            .unwrap()
            .to_owned();
        variant["policies"][effects]["body"]["class"] = class.into();
        variant["policies"][contract]["body"]["provider"]["revision"] = revision.clone().into();
        let mut registry = service_resources();
        registry.retain(|r| r.id != "urn:choreoform:examples:review-provider:0.1.0");
        registry.push(Resource {
            id: "urn:choreoform:examples:review-provider:0.1.0",
            revision: &revision,
            bytes: &bytes,
        });
        let e = envelope(variant);
        choreoform_semantic_check::check_invalidation(&e, &registry).unwrap();
        let error = choreoform_semantic_check::check_concurrency(&e, &registry).unwrap_err();
        assert_eq!(error.code, "parallel-effects-unproved");
        assert_eq!(error.path, "/body/nodes/fork");
    }
}
