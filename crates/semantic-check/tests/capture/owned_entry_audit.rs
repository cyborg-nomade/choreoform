// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Genuine current SDK contract probes only: no actual events, grants or dispatch.
use super::*;
use choreoform_semantic_check::{
    FAULT_HISTORY_PROFILE, FAULT_HISTORY_TYPES, FAULT_SOURCE_PROFILE, ProviderObservationState,
    Resources, Type, check_concurrency,
};
fn fixture() -> Fixture {
    super::readonly_continuation::fixture()
}
fn env(f: &Fixture) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.4.0", f.b.clone(), json!({})).unwrap()
}
fn refuse(f: &Fixture, code: &str) {
    match check_concurrency(&env(f), &f.supplied()) {
        Err(e) => assert_eq!(e.code, code, "{e:?}"),
        Ok(_) => panic!("expected {code}"),
    }
}
pub(super) fn provider(f: &mut Fixture, node: &str, class: &str, idempotency: &str) {
    let mut p = f.provider.clone();
    p["id"] = format!("urn:test:entry-audit:{node}:{class}").into();
    p["effectClass"] = class.into();
    p["idempotency"] = idempotency.into();
    let pin = f.artifact(p);
    let pin = json!({"id":pin.id,"revision":pin.revision});
    let policy = format!("{node}_provider");
    let mut contract = f.b["policies"]["provider_policy"].clone();
    contract["body"]["provider"] = pin.clone();
    f.b["policies"][&policy] = contract;
    let cap = f.b["nodes"][node]["target"].as_str().unwrap().to_owned();
    let effect = f.b["capabilities"][&cap]["effects"]
        .as_str()
        .unwrap()
        .to_owned();
    f.b["capabilities"][&cap]["contract"] = policy.into();
    f.b["policies"][&effect]["body"]["class"] = class.into();
    f.b["policies"][&effect]["body"]["idempotency"] = idempotency.into();
    let mut terminal = f.source.clone();
    terminal["id"] = format!("urn:test:entry-terminal:{node}:{class}").into();
    terminal["provider"] = pin.clone();
    let terminal = f.artifact(terminal);
    let wait = if node == "origin" {
        "capture_wait"
    } else {
        "unused_lookup_wait"
    };
    f.b["nodes"][wait]["capture"]["source"] =
        json!({"id":terminal.id,"revision":terminal.revision});
    let work = f.b["nodes"][node]["policy"].as_str().unwrap().to_owned();
    if !f.b["policies"][&work]["body"]["faultSource"].is_null() {
        let source=f.artifact(json!({"kind":"provider-fault-source","version":"0.1.0","id":format!("urn:test:entry-faults:{node}:{class}"),"profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":pin,"emissions":{"lookup uncertainty":{"observations":{"unknown":true},"evidence":"unresolved"}}}));
        f.b["policies"][&work]["body"]["faultSource"] =
            json!({"id":source.id,"revision":source.revision});
    }
}
#[test]
fn original_command_and_readonly_chain_pass_without_normalizing_unknown_as_terminal() {
    let mut f = fixture();
    provider(&mut f, "origin", "irreversible", "stable-key");
    let e = env(&f);
    let supplied = f.supplied();
    let final_report = check_reconciliation_availability(&e, &supplied).unwrap();
    assert_eq!(final_report.continuation_scopes().len(), 2);
    let mut resources = Resources::new(&supplied).unwrap();
    let source = resources
        .provider_terminal_source(
            &Pin::decode(&f.b["nodes"]["capture_wait"]["capture"]["source"]).unwrap(),
        )
        .unwrap();
    assert_eq!(
        source.provider_contract().observations()["unknown"],
        ProviderObservationState::Unknown
    );
    assert!(!source.terminal_observations().contains_key("unknown"));
    assert_eq!(
        source.observation_type(),
        &Type::Variant(std::collections::BTreeMap::from([(
            "confirmed".into(),
            Type::Integer
        )]))
    );
    assert_eq!(f.b["scopes"]["recovery"]["entry"], "capture_wait");
    assert!(
        final_report
            .links()
            .exceptional()
            .faults()
            .timing()
            .concurrency()
            .owned_continuations()
            .unwrap()
            .remaining()
            .contains(&"no-runtime-liveness-termination-or-eventual-settlement-guarantee")
    );
}
#[test]
fn matching_new_commands_and_compensation_are_rejected_by_the_current_family() {
    for class in ["irreversible", "compensatable"] {
        let mut f = fixture();
        provider(&mut f, "origin", "irreversible", "stable-key");
        provider(&mut f, "lookup", class, "stable-key");
        if class == "compensatable" {
            f.b["policies"]["lookup_work"]["scope"] = "recovery".into();
            f.b["policies"]["lookup_work"]["body"]["compensates"] = "origin".into();
        }
        check(&env(&f), &f.supplied()).unwrap();
        refuse(&f, "owned-continuation-query-effect");
    }
    let mut f = fixture();
    provider(&mut f, "origin", "irreversible", "stable-key");
    f.b["policies"]["lookup_work"]["scope"] = "recovery".into();
    f.b["policies"]["lookup_work"]["body"]["compensates"] = "origin".into();
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-query-policy");
}
#[test]
fn provider_identity_idempotency_and_compensation_targets_have_earlier_real_refusals() {
    let mut f = fixture();
    provider(&mut f, "lookup", "irreversible", "stable-key");
    f.b["policies"]["lookup_effects"]["body"]["idempotency"] = "none".into();
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "capability-effect"
    );
    let mut f = fixture();
    let wrong = f.b["policies"]["lookup_work"]["body"]["faultSource"].clone();
    provider(&mut f, "lookup", "irreversible", "stable-key");
    f.b["policies"]["lookup_work"]["body"]["faultSource"] = wrong;
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "fault-source-provider"
    );
    let mut f = fixture();
    f.b["policies"]["lookup_work"]["scope"] = "recovery".into();
    f.b["policies"]["lookup_work"]["body"]["compensates"] = "read_capture".into();
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "compensation-target"
    );
}
#[test]
fn query_before_terminal_capture_is_representable_but_has_no_current_entry_contract() {
    let mut f = fixture();
    f.b["scopes"]["recovery"]["entry"] = "lookup".into();
    f.b["flows"]["lookup_next"]["target"] = "capture_wait".into();
    f.b["flows"]["read_yes"]["target"] = "done".into();
    env(&f); // Closed existing IR 0.4 transport, not semantic permission.
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "retained-request-capture"
    );
    assert_eq!(
        f.b["nodes"]["lookup"]["reads"],
        json!({"original_request":true})
    );
    assert_eq!(f.b["flows"]["lookup_next"]["target"], "capture_wait");
    let mut f = fixture();
    f.b["scopes"]["recovery"]["entry"] = "done".into();
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "retained-request-capture"
    );
    let mut f = fixture();
    let mut copy = f.b["data"]["original_request"].clone();
    copy["invalidates"] = json!({});
    f.b["data"]["entry_copy"] = copy;
    f.b["nodes"]["entry_copy_step"] = json!({"kind":"compute","scope":"recovery","reads":{"original_request":true},"writes":{"entry_copy":true},"assignments":{"entry_copy":"lookup_input"},"outcomes":{"next":true}});
    f.b["flows"]["entry_copy_next"] =
        json!({"source":"entry_copy_step","outcome":"next","target":"capture_wait"});
    f.b["scopes"]["recovery"]["entry"] = "entry_copy_step".into();
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "retained-request-capture"
    );
}
#[test]
fn a_new_pure_request_projection_is_not_the_exact_original_request_rule() {
    let mut f = fixture();
    f.b["expressions"]["lookup_input"]["body"] = json!({"op":"record","fields":{}});
    f.b["expressions"]["lookup_input"]["reads"] = json!({});
    f.b["nodes"]["lookup"]["reads"] = json!({});
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-query-input");
    let mut f = fixture();
    let original = json!({"op":"read","cell":"original_request"});
    f.b["expressions"]["lookup_input"]["body"] =
        json!({"op":"if","condition":boolean(true),"then":original,"else":original});
    // Even a pure identity wrapper with unchanged complete reads is not the
    // current family's exact original-request expression. No value is supplied.
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-query-input");
}
#[test]
fn p4_history_and_p6_ownership_cannot_be_combined_by_reusing_one_scope() {
    let mut f = fixture();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for p in [FAULT_HISTORY_PROFILE, FAULT_HISTORY_TYPES] {
        f.extra.push((
            Pin {
                id: p.0.into(),
                revision: p.1.into(),
            },
            std::fs::read(root.join(format!("docs/ir/contracts/{}.txt", p.1.replace(':', "-"))))
                .unwrap(),
        ));
    }
    let history=f.artifact(json!({"kind":"provider-fault-history","version":"0.1.0","id":"urn:test:entry-audit-history","profile":{"id":FAULT_HISTORY_PROFILE.0,"revision":FAULT_HISTORY_PROFILE.1},"source":f.b["policies"]["lookup_work"]["body"]["faultSource"]}));
    f.b["policies"]["work"]["body"]["faultSource"] =
        f.b["policies"]["lookup_work"]["body"]["faultSource"].clone();
    f.b["policies"]["entry_faults"] = json!({"scope":"root","dialect":"core","body":{"kind":"faults","reads":{},"handlers":{"lookup uncertainty":"recovery"}}});
    f.b["scopes"]["root"]["faults"] = "entry_faults".into();
    f.b["scopes"]["recovery"]["faultInput"] =
        json!({"history":{"id":history.id,"revision":history.revision},"result":"entry_history"});
    let mut cell = f.b["data"]["original_request"].clone();
    cell["type"] = wrap(
        json!({"kind":"named","contract":{"id":FAULT_HISTORY_TYPES.0,"revision":FAULT_HISTORY_TYPES.1},"name":"Provider fault history"}),
    );
    cell["invalidates"] = json!({});
    f.b["data"]["entry_history"] = cell;
    assert_eq!(
        check(&env(&f), &f.supplied()).unwrap_err().code,
        "fault-entry-creator"
    );
}
#[test]
fn an_initializer_or_empty_seed_cannot_counterfeit_an_original_request_entry() {
    let mut f = fixture();
    f.b["scopes"]["recovery"]["retainedRequests"] = json!({});
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-request-unproved");
    f.b["data"]["original_request"]["initial"] = "invented_request".into();
    f.b["expressions"]["invented_request"] = json!({"scope":"recovery","dialect":"core","parameters":{},"reads":{},"resultType":wrap(json!({"kind":"record","fields":{}})),"body":{"op":"record","fields":{}}});
    check(&env(&f), &f.supplied()).unwrap();
    refuse(&f, "owned-continuation-request-unproved");
}
