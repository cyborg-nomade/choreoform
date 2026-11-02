// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use choreoform_authoring_frontend::Package;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource, digest};
use choreoform_semantic_check::{CORE, SEMANTICS, check};
use serde_json::{Value, json};

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
        ("timer", "clock-contract-unsupported"),
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
