// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Purpose-only declaration identity, never a dynamic eligible effect or RP-03 IR.
use super::*;
use choreoform_semantic_check::{CORE_V11, CORE_V12, check_concurrency};

fn env(f: &Fixture) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.4.0", f.b.clone(), json!({})).unwrap()
}
fn promote(mut f: Fixture) -> Fixture {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    f.extra.push((
        Pin {
            id: CORE_V12.0.into(),
            revision: CORE_V12.1.into(),
        },
        std::fs::read(root.join(format!(
            "docs/ir/contracts/{}.txt",
            CORE_V12.1.replace(':', "-")
        )))
        .unwrap(),
    ));
    f.b["dialects"]["core"] = json!({"id":CORE_V12.0,"revision":CORE_V12.1});
    f
}
fn fixture(scope: &str) -> Fixture {
    let f = promote(super::compensation_reference_audit::fixture(scope));
    check(&env(&f), &f.supplied()).unwrap();
    f
}
fn refuse(f: &Fixture, code: &str) {
    assert_eq!(check(&env(f), &f.supplied()).unwrap_err().code, code);
}

#[test]
fn only_new_core_allows_same_definition_prior_identity_outside_lexical_scope() {
    for scope in ["main", "root", "recovery", "lookup_recovery", "audit"] {
        let mut f = fixture(scope);
        f.b["policies"]["audit_work"]["body"]["compensates"] = "origin".into();
        check(&env(&f), &f.supplied()).unwrap();
        f.b["dialects"]["core"] = json!({"id":CORE_V11.0,"revision":CORE_V11.1});
        if matches!(scope, "main" | "audit") {
            refuse(&f, "visibility");
        } else {
            check(&env(&f), &f.supplied()).unwrap();
        }
    }
}

#[test]
fn missing_foreign_and_nonactivity_targets_do_not_become_resolvable() {
    for (target, code) in [
        ("missing", "reference"),
        ("urn:other-definition:origin", "reference"),
        ("origin_value", "reference"),
        ("owner_done", "compensation-target"),
        ("run_owner", "compensation-target"),
    ] {
        let mut f = fixture("audit");
        f.b["policies"]["audit_work"]["body"]["compensates"] = target.into();
        refuse(&f, code);
    }
}

#[test]
fn a_valid_human_activity_is_not_a_capability_compensation_identity() {
    let mut f = fixture("audit");
    let mut n = f.b["nodes"]["origin"].clone();
    n["mode"] = "human".into();
    n["target"] = "operator".into();
    n["result"] = Value::Null;
    n["writes"] = json!({});
    f.b["nodes"]["audit_human"] = n;
    f.b["flows"]["audit_human_next"] =
        json!({"source":"audit_human","outcome":"done","target":"owner_done"});
    check(&env(&f), &f.supplied()).unwrap();
    f.b["policies"]["audit_work"]["body"]["compensates"] = "audit_human".into();
    refuse(&f, "compensation-target");
}

#[test]
fn actual_policy_users_get_only_identity_not_eligible_occurrence_or_owned_permission() {
    let mut f = fixture("audit");
    // The original activity uses this main-scope rule; the prior is private in
    // Root. This is a declaration test, not a temporal self-dispatch witness.
    f.b["policies"]["work"]["body"]["compensates"] = "origin".into();
    check(&env(&f), &f.supplied()).unwrap();
    let mut f = promote(super::finite_work::fixture("irreversible", true, true));
    check_concurrency(&env(&f), &f.supplied()).unwrap();
    f.b["policies"]["lookup_work"]["body"]["compensates"] = "lookup".into();
    check(&env(&f), &f.supplied()).unwrap();
    assert_eq!(
        check_concurrency(&env(&f), &f.supplied()).unwrap_err().code,
        "owned-continuation-compensation-source"
    );
}

#[test]
fn own_class_and_purpose_keep_all_finite_work_and_live_host_boundaries() {
    for original in ["read-only", "compensatable", "irreversible"] {
        for class in ["read-only", "compensatable", "irreversible"] {
            for linked in [false, true] {
                let mut f = promote(super::finite_work::fixture(class, linked, true));
                super::owned_entry_audit::provider(&mut f, "origin", original, "stable-key");
                let r = check_reconciliation_availability(&env(&f), &f.supplied()).unwrap();
                let tree = r
                    .links()
                    .exceptional()
                    .faults()
                    .timing()
                    .concurrency()
                    .owned_continuations()
                    .unwrap();
                let root = tree.continuations().iter().next().unwrap();
                let level = root.scopes().next().unwrap();
                let work = level.work().unwrap();
                assert_eq!(work.effect_class().as_str(), class);
                assert_eq!(work.compensates(), linked.then_some("origin"));
                assert_eq!(work.is_query(), class == "read-only" && !linked);
                assert!(!work.host_requirements().is_empty());
                assert_eq!(tree.gates().len(), 7);
                assert!(r.uncovered_nodes().contains("lookup"));
            }
        }
    }
}

#[test]
fn private_values_control_and_other_policy_references_stay_lexical() {
    let mut f = fixture("audit");
    f.b["policies"]["audit_work"]["body"]["compensates"] = "origin".into();
    let mut expression = f.b["expressions"]["capture_is_confirmed"].clone();
    expression["scope"] = "audit".into();
    f.b["expressions"]["audit_read"] = expression;
    refuse(&f, "cell-reference");
    let mut f = fixture("audit");
    f.b["policies"]["audit_work"]["body"]["compensates"] = "origin".into();
    f.b["policies"]["audit_work"]["body"]["authority"] = "private_actor".into();
    let mut actor = f.b["actors"]["operator"].clone();
    actor["scope"] = "root".into();
    f.b["actors"]["private_actor"] = actor;
    refuse(&f, "visibility");
    let mut f = fixture("audit");
    f.b["flows"]["origin_next"]["target"] = "audit_done".into();
    refuse(&f, "structure");
}

#[test]
fn exact_new_artifact_all_eleven_bases_uniform_aliases_and_wire_are_required() {
    let f = fixture("audit");
    assert_eq!(
        f.extra
            .iter()
            .filter(|(p, _)| p.id.contains("core-profile:"))
            .count(),
        12
    );
    for (pin, _) in f
        .extra
        .iter()
        .filter(|(p, _)| p.id.contains("core-profile:"))
    {
        let supplied: Vec<_> = f
            .supplied()
            .into_iter()
            .filter(|r| r.id != pin.id)
            .collect();
        assert_eq!(
            check(&env(&f), &supplied).unwrap_err().code,
            "resource-missing",
            "{}",
            pin.id
        );
    }
    let mut f = fixture("audit");
    f.b["dialects"]["core"]["revision"] = ("sha256:".to_owned() + &"0".repeat(64)).into();
    refuse(&f, "dialect-contract");
    let mut f = fixture("audit");
    f.b["dialects"]["old"] = json!({"id":CORE_V11.0,"revision":CORE_V11.1});
    refuse(&f, "retained-request-wire-version");
    let f = fixture("audit");
    let wrong = DefinitionEnvelope::from_parts_versioned("0.3.0", f.b.clone(), json!({})).unwrap();
    assert_eq!(check(&wrong, &f.supplied()).unwrap_err().code, "structure");
    let mut f = fixture("audit");
    f.b["dialects"]["core"]["id"] = "urn:choreoform:contracts:core-profile:0.13.0".into();
    refuse(&f, "retained-request-wire-version");
    let mut f = fixture("audit");
    f.b["dialects"]["also"] = json!({"id":CORE_V12.0,"revision":CORE_V12.1});
    check(&env(&f), &f.supplied()).unwrap();
    for selected in [CORE_V12.0, CORE_V11.0] {
        let mut f = fixture("audit");
        f.extra
            .iter_mut()
            .find(|(p, _)| p.id == selected)
            .unwrap()
            .1
            .push(b' ');
        refuse(&f, "resource-digest");
    }
}

#[test]
fn inherited_capture_entry_protection_and_invalidation_refusals_still_apply() {
    let f = promote(super::finite_work::fixture("irreversible", true, false));
    assert_eq!(
        check_concurrency(&env(&f), &f.supplied()).unwrap_err().code,
        "owned-continuation-work-entry"
    );
    let mut f = promote(super::finite_work::fixture("irreversible", true, true));
    f.b["data"]["original_request"]["invalidates"] = json!({});
    assert_eq!(
        check_concurrency(&env(&f), &f.supplied()).unwrap_err().code,
        "owned-continuation-invalidation-coverage"
    );
    let mut f = promote(super::finite_work::fixture("irreversible", true, true));
    f.b["data"]["original_request"]["protection"]["capabilities"]
        .as_object_mut()
        .unwrap()
        .remove("adapter_lookup");
    assert_eq!(
        check_concurrency(&env(&f), &f.supplied()).unwrap_err().code,
        "information-flow"
    );
}
