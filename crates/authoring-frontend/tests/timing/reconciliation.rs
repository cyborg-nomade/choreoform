// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_semantic_check::{check_exceptional_availability, check_reconciliation_links};

impl Fixture {
    fn owned_reconciliation(&mut self) {
        let root = "scope_service";
        let closure = self.b["scopes"][root]["closure"]
            .as_str()
            .unwrap()
            .to_owned();
        self.b["policies"]["recovery_terminal"] = self.b["policies"][&closure].clone();
        self.b["policies"][&closure]["body"]["mode"] = "reconcile".into();
        self.b["policies"][&closure]["body"]["followup"] = "recovery".into();
        let mut recovery = self.b["scopes"][root].clone();
        recovery["parent"] = root.into();
        recovery["inputs"] = json!({});
        recovery["outputs"] = json!({});
        recovery["closure"] = "recovery_terminal".into();
        recovery["entry"] = "service_observe".into();
        self.b["scopes"]["recovery"] = recovery;
        self.b["nodes"]["recovery_done"] = self.b["nodes"]["service_done"].clone();
        self.b["nodes"]["recovery_done"]["scope"] = "recovery".into();
        self.b["nodes"]["service_observe"]["scope"] = "recovery".into();
        for f in self.b["flows"].as_object_mut().unwrap().values_mut() {
            if f["source"] == "service_observe" {
                f["target"] = "recovery_done".into();
            }
        }
    }
}

#[test]
fn a_unique_ordinary_wait_is_not_a_surviving_reconciliation_boundary() {
    let f = Fixture::new(true);
    let e = f.envelope();
    let r = f.resources();
    check_exceptional_availability(&e, &r).unwrap();
    let error = check_reconciliation_links(&e, &r).unwrap_err();
    assert_eq!(error.code, "reconciliation-followup-unproved");
    assert_eq!(error.path, "/body/scopes/scope_service/closure");
}

#[test]
fn reachable_followup_wait_reports_exact_links_and_only_host_requirements() {
    for v2 in [false, true] {
        let mut f = Fixture::new(true);
        f.owned_reconciliation();
        if v2 {
            let p = choreoform_semantic_check::CORE_V2;
            f.artifact(p.0,include_bytes!("../../../../docs/ir/contracts/sha256-5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623.txt").to_vec());
            f.b["dialects"]["core"] = json!({"id":p.0,"revision":p.1});
            f.b["policies"]["service_work"]["body"]["retry"]["clock"] = Value::Null;
            f.b["policies"]["service_work"]["body"]["retry"]["basis"] = Value::Null;
        }
        let report = check_reconciliation_links(&f.envelope(), &f.resources()).unwrap();
        let u = report.uses().iter().next().unwrap();
        assert_eq!(u.site(), "service_step");
        assert_eq!(u.scope(), "scope_service");
        assert_eq!(u.followup(), "recovery");
        assert_eq!(u.wait_node(), "service_observe");
        assert_eq!(
            u.wait_policy(),
            f.b["nodes"]["service_observe"]["policy"].as_str().unwrap()
        );
        assert!(
            u.host_requirements()
                .contains(&"accepted-authorized-atomic-obligation-and-subscription-handoff")
        );
        assert_eq!(
            report.revision(),
            f.envelope().document()["revision"].as_str().unwrap()
        );
        assert!(
            report
                .exceptional()
                .uncovered_nodes()
                .contains("service_observe")
        );
        assert_eq!(f.document(PROVIDER_ID)["effectClass"], "read-only");
    }
}

#[test]
fn ordinary_nested_or_unreachable_waits_cannot_supply_the_direct_followup_route() {
    for mutation in ["ordinary", "nested", "unreachable"] {
        let mut f = Fixture::new(true);
        f.owned_reconciliation();
        f.b["scopes"]["recovery"]["entry"] = "recovery_done".into();
        if mutation == "ordinary" {
            f.b["nodes"]["service_observe"]["scope"] = "scope_service".into();
            for flow in f.b["flows"].as_object_mut().unwrap().values_mut() {
                if flow["source"] == "service_observe" {
                    flow["target"] = "service_done".into();
                }
            }
        } else if mutation == "nested" {
            f.b["scopes"]["inner"] = f.b["scopes"]["recovery"].clone();
            f.b["scopes"]["inner"]["parent"] = "recovery".into();
            f.b["scopes"]["inner"]["entry"] = "service_observe".into();
            f.b["nodes"]["service_observe"]["scope"] = "inner".into();
            f.b["nodes"]["inner_done"] = f.b["nodes"]["recovery_done"].clone();
            f.b["nodes"]["inner_done"]["scope"] = "inner".into();
            for flow in f.b["flows"].as_object_mut().unwrap().values_mut() {
                if flow["source"] == "service_observe" {
                    flow["target"] = "inner_done".into();
                }
            }
        }
        let e = f.envelope();
        let r = f.resources();
        check_exceptional_availability(&e, &r).unwrap();
        let error = check_reconciliation_links(&e, &r).unwrap_err();
        assert_eq!(
            error.code,
            if mutation == "unreachable" {
                "reconciliation-route-unproved"
            } else {
                "reconciliation-context-unproved"
            },
            "{mutation}: {error:?}"
        );
        assert!(e.document().pointer(&error.path).is_some());
    }
}

#[test]
fn a_shared_effect_policy_cannot_guess_an_owner_for_an_incompatible_activity_scope() {
    let mut f = Fixture::new(true);
    f.owned_reconciliation();
    f.b["scopes"]["other"] = f.b["scopes"]["recovery"].clone();
    f.b["scopes"]["other"]["parent"] = "scope_service".into();
    f.b["scopes"]["other"]["entry"] = "other_activity".into();
    f.b["nodes"]["other_done"] = f.b["nodes"]["recovery_done"].clone();
    f.b["nodes"]["other_done"]["scope"] = "other".into();
    f.b["nodes"]["other_activity"] = f.b["nodes"]["service_step"].clone();
    f.b["nodes"]["other_activity"]["scope"] = "other".into();
    for outcome in ["approved", "correction_needed"] {
        f.b["flows"][format!("other_{outcome}")] =
            json!({"source":"other_activity","outcome":outcome,"target":"other_done"});
    }
    let e = f.envelope();
    let r = f.resources();
    check_exceptional_availability(&e, &r).unwrap();
    let error = check_reconciliation_links(&e, &r).unwrap_err();
    assert_eq!(error.code, "reconciliation-followup-unproved");
    assert_eq!(error.path, "/body/scopes/other/closure");
}

#[test]
fn unused_effects_get_no_invented_activity_owner() {
    let mut f = Fixture::new(true);
    f.owned_reconciliation();
    f.b["nodes"].as_object_mut().unwrap().remove("service_step");
    for data in f.b["data"].as_object_mut().unwrap().values_mut() {
        data["invalidates"]
            .as_object_mut()
            .unwrap()
            .remove("service_step");
    }
    f.b["flows"]
        .as_object_mut()
        .unwrap()
        .retain(|_, v| v["source"] != "service_step");
    f.b["scopes"]["scope_service"]["entry"] = "service_done".into();
    f.b["scopes"]["scope_service"]["outputs"] = json!({});
    let r = check_reconciliation_links(&f.envelope(), &f.resources()).unwrap();
    assert!(r.uses().is_empty());
    assert!(r.unused_effects().contains("service_effect"));
}

#[test]
fn repeated_declared_activity_sites_share_indexes_without_ordinary_coverage_credit() {
    let mut f = Fixture::new(true);
    f.owned_reconciliation();
    let flows: Vec<_> = f.b["flows"]
        .as_object()
        .unwrap()
        .values()
        .filter(|v| v["source"] == "service_step")
        .cloned()
        .collect();
    for i in 0..500 {
        let id = format!("site{i}");
        f.b["nodes"][&id] = f.b["nodes"]["service_step"].clone();
        for (j, flow) in flows.iter().enumerate() {
            let mut flow = flow.clone();
            flow["source"] = id.clone().into();
            f.b["flows"][format!("site{i}_flow{j}")] = flow;
        }
    }
    let r = check_reconciliation_links(&f.envelope(), &f.resources()).unwrap();
    assert_eq!(r.uses().len(), 501);
    assert_eq!(r.ordinary_unvisited_sites().len(), 500);
}

#[test]
fn human_only_definitions_have_no_capability_reconciliation_use() {
    let f = Fixture::new(false);
    let r = check_reconciliation_links(&f.envelope(), &f.resources()).unwrap();
    assert!(r.uses().is_empty());
    assert!(r.unused_effects().is_empty());
}
