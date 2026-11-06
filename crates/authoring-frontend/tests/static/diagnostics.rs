// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

fn refused(body: Value, supplied: &[Resource<'_>], code: &str, path: &str) {
    let e = envelope(body);
    e.check_structure()
        .unwrap_or_else(|error| panic!("{path}: {error:?}"));
    let d = check(&e, supplied).unwrap_err();
    assert_eq!(d.code, code, "{d:?}");
    assert_eq!(d.path, path);
    assert!(e.document().pointer(path).is_some(), "{path}");
}
fn child(b: &mut Value, id: &str, parent: &str) {
    b["scopes"][id] = b["scopes"]["scope_review"].clone();
    b["scopes"][id]["parent"] = parent.into();
    b["scopes"][id]["inputs"] = json!({});
    b["scopes"][id]["outputs"] = json!({});
    b["scopes"][id]["entry"] = format!("{id}_done").into();
    b["nodes"][format!("{id}_done")] = b["nodes"]["review_done"].clone();
    b["nodes"][format!("{id}_done")]["scope"] = id.into();
}

#[test]
fn scope_policy_outcome_and_initialized_input_failures_have_exact_paths() {
    let base = review()["body"].clone();
    let mut b = base.clone();
    b["scopes"]["scope_review"]["cancellation"] = "review_settle".into();
    refused(
        b,
        &resources(),
        "policy-kind",
        "/body/scopes/scope_review/cancellation",
    );
    let mut b = base.clone();
    b["scopes"]["scope_review"]["inputs"] = json!({"argument":"review_request"});
    refused(
        b,
        &resources(),
        "input-initializer",
        "/body/scopes/scope_review/inputs/argument",
    );
    let mut b = base;
    b["scopes"]["scope_review"]["outcomes"] = json!({});
    refused(
        b,
        &resources(),
        "scope-outcomes",
        "/body/scopes/scope_review/outcomes",
    );
}

#[test]
fn unused_actor_and_data_failures_identify_their_own_declarations() {
    let base = review()["body"].clone();
    let mut b = base.clone();
    b["actors"]["unused_actor"] = json!({"scope":"scope_review","requirement":"review_protection"});
    refused(
        b,
        &resources(),
        "policy-kind",
        "/body/actors/unused_actor/requirement",
    );
    let mut b = base.clone();
    child(&mut b, "private_scope", "scope_review");
    b["actors"]["private_actor"] = b["actors"]["review_actor"].clone();
    b["actors"]["private_actor"]["scope"] = "private_scope".into();
    b["data"]["review_audit"]["protection"]["participants"] = json!({"private_actor":true});
    refused(
        b,
        &resources(),
        "visibility",
        "/body/data/review_audit/protection",
    );
    let mut b = base.clone();
    b["data"]["review_audit"]["initial"] = b["data"]["review_request"]["initial"].clone();
    refused(
        b,
        &resources(),
        "initializer-type",
        "/body/data/review_audit/initial",
    );
    let mut b = base;
    child(&mut b, "child", "scope_review");
    b["data"]["child_only"] = b["data"]["review_audit"].clone();
    b["data"]["child_only"]["scope"] = "child".into();
    b["data"]["child_only"]["invalidates"] = json!({"review_step":true});
    refused(
        b,
        &resources(),
        "visibility",
        "/body/data/child_only/invalidates/review_step",
    );
}

#[test]
fn capability_contract_input_output_authority_and_effect_failures_are_distinct() {
    let supplied = service_resources();
    let base = Package::parse(SERVICE, SERVICE_LEDGER)
        .unwrap()
        .lower(&supplied)
        .unwrap()
        .document()["body"]
        .clone();
    for (field, value, code) in [
        ("contract", json!("service_work"), "policy-kind"),
        (
            "input",
            json!({"dialect":"core","body":{"kind":"integer"}}),
            "capability-type",
        ),
        (
            "output",
            json!({"dialect":"core","body":{"kind":"integer"}}),
            "capability-type",
        ),
        ("effects", json!("service_work"), "policy-kind"),
    ] {
        let mut b = base.clone();
        b["capabilities"]["service_capability"][field] = value;
        refused(
            b,
            &supplied,
            code,
            &format!("/body/capabilities/service_capability/{field}"),
        );
    }
    let mut b = base.clone();
    b["policies"]["service_effect"]["body"]["idempotency"] = "stable-key".into();
    refused(
        b,
        &supplied,
        "capability-effect",
        "/body/capabilities/service_capability/effects",
    );
    let mut b = base;
    b["scopes"]["child"] = b["scopes"]["scope_service"].clone();
    b["scopes"]["child"]["parent"] = "scope_service".into();
    b["scopes"]["child"]["entry"] = "child_done".into();
    b["scopes"]["child"]["inputs"] = json!({});
    b["scopes"]["child"]["outputs"] = json!({});
    b["nodes"]["child_done"] = b["nodes"]["service_done"].clone();
    b["nodes"]["child_done"]["scope"] = "child".into();
    b["actors"]["child_actor"] = b["actors"]["service_actor"].clone();
    b["actors"]["child_actor"]["scope"] = "child".into();
    b["capabilities"]["service_capability"]["authority"] = "child_actor".into();
    refused(
        b,
        &supplied,
        "visibility",
        "/body/capabilities/service_capability/authority",
    );
}

#[test]
fn capability_input_comparison_keeps_short_circuit_order_under_both_core_profiles() {
    for modern in [false, true] {
        let mut supplied = service_resources();
        let mut b = Package::parse(SERVICE, SERVICE_LEDGER)
            .unwrap()
            .lower(&supplied)
            .unwrap()
            .document()["body"]
            .clone();
        if modern {
            let p = choreoform_semantic_check::CORE_V2;
            supplied.push(Resource { id:p.0, revision:p.1, bytes:include_bytes!("../../../../docs/ir/contracts/sha256-5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623.txt") });
            b["dialects"]["core"] = json!({"id":p.0,"revision":p.1});
            b["policies"]["service_work"]["body"]["retry"]["clock"] = Value::Null;
            b["policies"]["service_work"]["body"]["retry"]["basis"] = Value::Null;
        }
        b["capabilities"]["service_capability"]["input"]["body"] = json!({"kind":"integer"});
        // The output descriptor is opaque to structure checks. As before, an
        // input mismatch must refuse before attempting this unsupported shape.
        b["capabilities"]["service_capability"]["output"]["body"] = json!({"kind":"future"});
        // Remove ordinary activity use so work-completion inference does not
        // independently inspect that output before the capability judgment.
        b["nodes"]["service_step"]["mode"] = "human".into();
        b["nodes"]["service_step"]["target"] = "service_actor".into();
        refused(
            b,
            &supplied,
            "capability-type",
            "/body/capabilities/service_capability/input",
        );
    }
}

#[test]
fn ancestry_limit_refusal_has_a_real_parent_path_and_global_revision_stays_global() {
    let mut b = review()["body"].clone();
    for i in 0..64 {
        child(&mut b, &format!("depth{i:02}"), "scope_review");
        if i > 0 {
            b["scopes"][format!("depth{i:02}")]["parent"] = format!("depth{:02}", i - 1).into();
        }
    }
    refused(
        b,
        &resources(),
        "scope-cycle",
        "/body/scopes/depth63/parent",
    );
    let mut document = review();
    document["revision"] = format!("sha256:{}", "0".repeat(64)).into();
    let e = DefinitionEnvelope::decode(&serde_json::to_vec(&document).unwrap()).unwrap();
    let d = check(&e, &resources()).unwrap_err();
    assert_eq!(d.code, "revision");
    assert!(d.path.is_empty());
}
