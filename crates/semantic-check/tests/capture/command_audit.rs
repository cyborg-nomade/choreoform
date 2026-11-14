// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Current SDK judgments only, never permission for new work or actual effects.
use super::owned_entry_audit::provider;
use super::*;
use choreoform_semantic_check::check_concurrency;

pub(super) fn fixture(capturing: bool) -> Fixture {
    let mut f = super::pre_capture_entry::fixture();
    if capturing {
        f.b["scopes"]["recovery"]["entry"] = "capture_wait".into();
        f.b["flows"]["read_yes"]["target"] = "lookup".into();
        f.b["flows"]["lookup_next"]["target"] = "done".into();
    }
    f
}
fn env(f: &Fixture) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.4.0", f.b.clone(), json!({})).unwrap()
}
fn refusal(f: &Fixture, code: &str) {
    match check_concurrency(&env(f), &f.supplied()) {
        Err(e) => assert_eq!(e.code, code, "{e:?}"),
        Ok(_) => panic!("expected {code}"),
    }
}
pub(super) fn compensates(f: &mut Fixture) {
    f.b["policies"]["lookup_work"]["scope"] = "recovery".into();
    f.b["policies"]["lookup_work"]["body"]["compensates"] = "origin".into();
}
pub(super) fn captured_input(capturing: bool) -> Fixture {
    let mut f = fixture(capturing);
    // Only the new capability takes an integer. Original source/request stays
    // the original empty record; the new child's seed has the new exact type.
    f.provider["input"] = json!({"kind":"integer"});
    provider(&mut f, "lookup", "read-only", "none");
    let ty = wrap(json!({"kind":"integer"}));
    f.b["capabilities"]["adapter_lookup"]["input"] = ty.clone();
    f.b["data"]["lookup_request"]["type"] = ty.clone();
    f.b["expressions"]["lookup_input"]["resultType"] = ty;
    f.b["expressions"]["lookup_input"]["body"] =
        json!({"op":"payload","value":{"op":"read","cell":"captured"},"tag":"confirmed"});
    f.b["expressions"]["lookup_input"]["reads"] = json!({"captured":true});
    f.b["nodes"]["lookup"]["reads"] = json!({"captured":true});
    f
}

#[test]
fn own_effect_class_and_compensation_link_are_independent_matched_dimensions() {
    for original in ["read-only", "compensatable", "irreversible"] {
        for class in ["compensatable", "irreversible"] {
            for linked in [false, true] {
                for capturing in [false, true] {
                    let mut f = fixture(capturing);
                    provider(&mut f, "origin", original, "stable-key");
                    provider(&mut f, "lookup", class, "stable-key");
                    if linked {
                        compensates(&mut f);
                    }
                    check(&env(&f), &f.supplied()).unwrap();
                    refusal(&f, "owned-continuation-query-effect");
                }
            }
        }
    }
}
#[test]
fn readonly_compensation_link_has_a_policy_barrier_separate_from_effect_class() {
    for original in ["read-only", "compensatable", "irreversible"] {
        for capturing in [false, true] {
            let mut f = fixture(capturing);
            provider(&mut f, "origin", original, "stable-key");
            compensates(&mut f);
            check(&env(&f), &f.supplied()).unwrap();
            refusal(&f, "owned-continuation-query-policy");
        }
    }
}
#[test]
fn matching_one_attempt_non_idempotent_metadata_does_not_grant_owned_commands() {
    for class in ["irreversible", "compensatable"] {
        for key in ["none", "stable-key"] {
            let mut f = fixture(false);
            provider(&mut f, "lookup", class, key);
            check(&env(&f), &f.supplied()).unwrap();
            refusal(&f, "owned-continuation-query-effect");
        }
    }
}
#[test]
fn mismatched_provider_key_and_nonactivity_target_keep_their_earlier_refusals() {
    let mut f = fixture(false);
    provider(&mut f, "lookup", "irreversible", "stable-key");
    f.b["policies"]["lookup_effects"]["body"]["idempotency"] = "none".into();
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "capability-effect"
    );
    let mut f = fixture(false);
    let wrong = f.b["policies"]["lookup_work"]["body"]["faultSource"].clone();
    provider(&mut f, "lookup", "irreversible", "stable-key");
    f.b["policies"]["lookup_work"]["body"]["faultSource"] = wrong;
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "fault-source-provider"
    );
    let mut f = fixture(false);
    compensates(&mut f);
    f.b["policies"]["lookup_work"]["body"]["compensates"] = "read_capture".into();
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "compensation-target"
    );
}
#[test]
fn a_correct_pure_request_type_or_identity_wrapper_is_not_the_exact_request_rule() {
    for capturing in [false, true] {
        for constant in [false, true] {
            let mut f = fixture(capturing);
            if constant {
                f.b["expressions"]["lookup_input"]["body"] = json!({"op":"record","fields":{}});
                f.b["expressions"]["lookup_input"]["reads"] = json!({});
                f.b["nodes"]["lookup"]["reads"] = json!({});
            } else {
                let read = json!({"op":"read","cell":"original_request"});
                f.b["expressions"]["lookup_input"]["body"] =
                    json!({"op":"if","condition":boolean(true),"then":read,"else":read});
            }
            check(&env(&f), &f.supplied()).unwrap();
            refusal(&f, "owned-continuation-query-input");
        }
    }
}
#[test]
fn typed_capture_constructor_and_new_typed_child_seed_fit_the_existing_wire() {
    for capturing in [false, true] {
        let f = captured_input(capturing);
        check(&env(&f), &f.supplied()).unwrap();
        assert_ne!(
            f.b["data"]["original_request"]["type"],
            f.b["data"]["lookup_request"]["type"]
        );
        // In the early-entry layout the shape/input check precedes conditional
        // availability. This is not a claimed successful early capture read.
        refusal(&f, "owned-continuation-query-input");
    }
}
#[test]
fn constructed_input_still_has_real_static_type_and_complete_read_obligations() {
    let mut f = captured_input(true);
    f.b["nodes"]["lookup"]["reads"] = json!({});
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "node-reads"
    );
    let mut f = captured_input(true);
    f.b["expressions"]["lookup_input"]["body"] =
        json!({"op":"literal","type":{"kind":"text"},"value":"not an integer"});
    f.b["expressions"]["lookup_input"]["resultType"] = wrap(json!({"kind":"text"}));
    f.b["expressions"]["lookup_input"]["reads"] = json!({});
    f.b["nodes"]["lookup"]["reads"] = json!({});
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "activity-input"
    );
}
#[test]
fn an_unvisited_second_capability_is_not_covered_by_the_one_query_report() {
    let mut f = fixture(false);
    f.b["nodes"]["extra_query"] = f.b["nodes"]["lookup"].clone();
    f.b["flows"]["extra_next"] = json!({"source":"extra_query","outcome":"done","target":"done"});
    check(&env(&f), &f.supplied()).unwrap();
    refusal(&f, "owned-continuation-body-unproved");
}
