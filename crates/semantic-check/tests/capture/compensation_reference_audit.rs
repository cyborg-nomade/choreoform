// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! C11 declaration boundaries, never live eligible effects or full RP-03 IR.
use super::*;
use choreoform_semantic_check::check_concurrency;

fn env(f: &Fixture) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.4.0", f.b.clone(), json!({})).unwrap()
}
pub(super) fn fixture(scope: &str) -> Fixture {
    let mut f = super::finite_work::fixture("irreversible", true, true);
    let mut sibling = f.b["scopes"]["main"].clone();
    sibling["parent"] = "main".into();
    sibling["entry"] = "audit_done".into();
    f.b["scopes"]["audit"] = sibling;
    let mut done = f.b["nodes"]["main_done"].clone();
    done["scope"] = "audit".into();
    f.b["nodes"]["audit_done"] = done;
    let mut policy = f.b["policies"]["work"].clone();
    policy["scope"] = scope.into();
    // Preserve the original fixture's completion predicate, authority and retry
    // metadata. An unused rule isolates static identity from actual use sites.
    f.b["policies"]["audit_work"] = policy;
    check(&env(&f), &f.supplied()).unwrap();
    f
}
fn refuse(f: &Fixture, code: &str) {
    assert_eq!(check(&env(f), &f.supplied()).unwrap_err().code, code);
}

#[test]
fn prior_identity_uses_policy_lexical_scope_not_global_id_presence() {
    for (scope, expected) in [
        ("root", None),
        ("recovery", None),
        ("lookup_recovery", None),
        ("main", Some("visibility")),
        ("audit", Some("visibility")),
    ] {
        let mut f = fixture(scope);
        f.b["policies"]["audit_work"]["body"]["compensates"] = "origin".into();
        if let Some(code) = expected {
            refuse(&f, code);
        } else {
            check(&env(&f), &f.supplied()).unwrap();
        }
    }
}

#[test]
fn missing_foreign_and_wrong_kind_targets_have_distinct_first_refusals() {
    for (target, code) in [
        ("absent", "reference"),
        ("urn:another-definition:origin", "reference"),
        ("owner_done", "compensation-target"),
        ("run_owner", "compensation-target"),
        ("origin_value", "reference"),
    ] {
        let mut f = fixture("root");
        f.b["policies"]["audit_work"]["body"]["compensates"] = target.into();
        refuse(&f, code);
    }
    let mut f = fixture("audit");
    f.b["policies"]["audit_work"]["body"]["compensates"] = "owner_done".into();
    refuse(&f, "visibility"); // Privacy precedes the activity-kind check.
}

#[test]
fn static_self_site_is_not_a_proof_of_a_prior_dynamic_occurrence() {
    let mut f = fixture("root");
    f.b["policies"]["work"]["scope"] = "root".into();
    f.b["policies"]["work"]["body"]["compensates"] = "origin".into();
    check(&env(&f), &f.supplied()).unwrap();
    // The SDK does not manufacture a current eligible occurrence, new command
    // key or actual provider event from this accepted declaration.
}

#[test]
fn visible_static_activity_still_must_match_owned_continuations_incoming_source() {
    let mut f = super::finite_work::fixture("irreversible", true, true);
    check_concurrency(&env(&f), &f.supplied()).unwrap();
    f.b["policies"]["lookup_work"]["body"]["compensates"] = "lookup".into();
    check(&env(&f), &f.supplied()).unwrap();
    assert_eq!(
        check_concurrency(&env(&f), &f.supplied()).unwrap_err().code,
        "owned-continuation-compensation-source"
    );
}

#[test]
fn ordinary_private_data_and_outcomes_do_not_gain_access_from_an_identity_link() {
    let mut f = fixture("audit");
    let mut expression = f.b["expressions"]["capture_is_confirmed"].clone();
    expression["scope"] = "audit".into();
    f.b["expressions"]["audit_read"] = expression;
    refuse(&f, "cell-reference");
    let mut f = fixture("audit");
    f.b["flows"]["audit_private_outcome"] =
        json!({"source":"origin","outcome":"done","target":"audit_done"});
    // A cross-scope flow is a control violation, not a prior-identity lookup.
    assert_eq!(
        check_control(&env(&f), &f.supplied()).unwrap_err().code,
        "structure"
    );
}
