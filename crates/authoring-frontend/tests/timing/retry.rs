// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_semantic_check::CORE_V2;

const RETRY_PROFILE: &[u8] = include_bytes!(
    "../../../../docs/ir/contracts/sha256-5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623.txt"
);
impl Fixture {
    fn retry_binding(&mut self) {
        let profile = self.artifact(CORE_V2.0, RETRY_PROFILE.to_vec());
        assert_eq!(profile.revision, CORE_V2.1);
        self.b["dialects"]["retry_core"] = json!({"id":profile.id,"revision":profile.revision});
        let p = &mut self.b["policies"][format!("{}_work", self.prefix)];
        p["dialect"] = "retry_core".into();
        p["body"]["retry"] = json!({"maxAttempts":3,"delays":["0","9223372036854775807"],"faults":{"transient":true},"clock":{"id":self.clock.id,"revision":self.clock.revision},"basis":format!("{}_audit",self.prefix)});
        p["body"]["reads"][format!("{}_audit", self.prefix)] = true.into();
        self.b["nodes"][format!("{}_step", self.prefix)]["reads"]
            [format!("{}_audit", self.prefix)] = true.into();
    }
}

#[test]
fn publication_and_exact_retry_uses_preserve_live_obligations() {
    assert_eq!(digest(RETRY_PROFILE), CORE_V2.1);
    assert_eq!(
        RETRY_PROFILE,
        include_bytes!("../../../../docs/dialects/core-retry-v0.2.md")
    );
    for timeout in [false, true] {
        let mut f = Fixture::new(false);
        f.retry_binding();
        if timeout {
            f.work_timer();
        }
        let report = check_timing(&f.envelope(), &f.resources()).unwrap();
        assert_eq!(report.timer_uses().len(), usize::from(timeout));
        let r = report.retry_uses().iter().next().unwrap();
        assert_eq!(r.site(), "review_step");
        assert_eq!(r.policy(), "review_work");
        assert_eq!(r.clock(), &f.clock);
        assert_eq!(r.source().id, SOURCE_ID);
        assert_eq!(r.basis(), "review_audit");
        assert_eq!(r.authority(), "review_actor");
        assert_eq!(r.max_attempts(), 3);
        assert_eq!(r.delays(), &[0, i64::MAX]);
        assert!(
            r.host_requirements()
                .contains(&"accepted-eligible-fault-attempt-correlation-and-domain-stamp")
        );
        assert!(
            r.host_requirements()
                .contains(&"stable-key-or-definitive-correlated-no-effect-evidence-as-required")
        );
        assert_eq!(
            report.checked_nodes(),
            &BTreeSet::from(["review_step".into()])
        );
    }
}

use std::collections::BTreeSet;

#[test]
fn retry_shape_and_binding_are_selected_by_each_records_exact_dialect() {
    for (mutation, code) in [
        ("old", "fields"),
        ("new-missing-fields", "fields"),
        ("missing-clock", "retry-binding-required"),
        ("missing-basis", "retry-binding-required"),
        ("no-cell", "reference"),
        ("extra", "fields"),
        ("negative", "retry-delay"),
        ("one-bound", "retry-binding-unused"),
        ("policy-read", "policy-reads"),
        ("node-read", "node-reads"),
        ("profile", "dialect-contract"),
        ("clock", "resource-missing"),
    ] {
        let mut f = Fixture::new(false);
        f.retry_binding();
        let r = &mut f.b["policies"]["review_work"]["body"]["retry"];
        match mutation {
            "old" => f.b["policies"]["review_work"]["dialect"] = "core".into(),
            "new-missing-fields" => {
                r.as_object_mut().unwrap().remove("clock");
            }
            "missing-clock" => r["clock"] = Value::Null,
            "missing-basis" => r["basis"] = Value::Null,
            "no-cell" => r["basis"] = "absent_cell".into(),
            "extra" => r["approved"] = true.into(),
            "negative" => r["delays"][0] = "-1".into(),
            "one-bound" => {
                r["maxAttempts"] = 1.into();
                r["delays"] = json!([]);
            }
            "policy-read" => f.b["policies"]["review_work"]["body"]["reads"] = json!({}),
            "node-read" => {
                f.b["nodes"]["review_step"]["reads"]
                    .as_object_mut()
                    .unwrap()
                    .remove("review_audit");
            }
            "profile" => f.b["dialects"]["retry_core"]["revision"] = CORE.1.into(),
            _ => {
                r["clock"]["id"] = "urn:missing:clock".into();
            }
        }
        let error = check(&f.envelope(), &f.resources()).unwrap_err();
        assert_eq!(error.code, code, "{mutation}: {error:?}");
    }
    let mut f = Fixture::new(false);
    f.retry_binding();
    f.b["policies"]["review_work"]["body"]["retry"] =
        json!({"maxAttempts":1,"delays":[],"faults":{},"clock":null,"basis":null});
    f.b["policies"]["review_work"]["body"]["reads"] = json!({});
    let report = check_timing(&f.envelope(), &f.resources()).unwrap();
    assert!(report.retry_uses().is_empty());
}

#[test]
fn inherited_contract_bytes_and_exact_retry_profile_are_required() {
    let mut f = Fixture::new(false);
    f.retry_binding();
    fn use_v2(v: &mut Value) {
        match v {
            Value::Object(m) => {
                if m.get("dialect") == Some(&Value::from("core")) {
                    m.insert("dialect".into(), "retry_core".into());
                }
                for child in m.values_mut() {
                    use_v2(child);
                }
            }
            Value::Array(a) => {
                for child in a {
                    use_v2(child);
                }
            }
            _ => (),
        }
    }
    use_v2(&mut f.b);
    f.b["dialects"].as_object_mut().unwrap().remove("core");
    check(&f.envelope(), &f.resources()).unwrap();
    let e = f.envelope();
    let mut resources = f.resources();
    resources.retain(|r| r.id != CORE.0);
    assert_eq!(check(&e, &resources).unwrap_err().code, "resource-missing");
    let mut resources = f.resources();
    resources.retain(|r| r.id != CORE_V2.0);
    assert_eq!(check(&e, &resources).unwrap_err().code, "resource-missing");
    let mut resources = f.resources();
    resources
        .iter_mut()
        .find(|r| r.id == CORE_V2.0)
        .unwrap()
        .bytes = b"wrong bytes";
    assert_eq!(check(&e, &resources).unwrap_err().code, "resource-digest");
}

#[test]
fn retry_closes_transitive_source_actor_reads_and_checks_basis_visibility() {
    let mut f = Fixture::new(false);
    f.retry_binding();
    f.b["actors"]["clock_actor"] =
        json!({"scope":"scope_review","requirement":"clock_requirement"});
    f.b["policies"]["clock_requirement"] = json!({"scope":"scope_review","dialect":"core","body":{"kind":"actor","reads":{"review_request":true},"permit":{"op":"eq","left":{"op":"read","cell":"review_request"},"right":{"op":"literal","type":{"kind":"text"},"value":"Synthetic request"}}}});
    let mut source = f.document(SOURCE_ID);
    source["resultProtection"]["participants"]["clock_actor"] = true.into();
    f.clock_source(source);
    f.b["policies"]["review_work"]["body"]["retry"]["clock"] =
        json!({"id":f.clock.id,"revision":f.clock.revision});
    f.b["policies"]["review_work"]["body"]["reads"]["review_request"] = true.into();
    check_timing(&f.envelope(), &f.resources()).unwrap();
    f.b["policies"]["review_work"]["body"]["reads"]
        .as_object_mut()
        .unwrap()
        .remove("review_request");
    assert_eq!(
        check(&f.envelope(), &f.resources()).unwrap_err().code,
        "policy-reads"
    );
    f.b["policies"]["review_work"]["body"]["reads"]["review_request"] = true.into();
    f.b["scopes"]["child"] = f.b["scopes"]["scope_review"].clone();
    f.b["scopes"]["child"]["parent"] = "scope_review".into();
    f.b["scopes"]["child"]["entry"] = "child_done".into();
    f.b["scopes"]["child"]["inputs"] = json!({});
    f.b["scopes"]["child"]["outputs"] = json!({});
    f.b["nodes"]["child_done"] = json!({"kind":"finish","scope":"child","outcome":"complete","reads":{},"writes":{},"outcomes":{}});
    f.b["data"]["hidden_basis"] = f.b["data"]["review_audit"].clone();
    f.b["data"]["hidden_basis"]["scope"] = "child".into();
    f.b["policies"]["review_work"]["body"]["retry"]["basis"] = "hidden_basis".into();
    assert_eq!(
        check(&f.envelope(), &f.resources()).unwrap_err().code,
        "visibility"
    );
}

#[test]
fn retry_and_timeout_origins_coexist_even_when_no_result_is_stored() {
    for private_role in ["retry", "timeout", "activity", "none"] {
        let mut f = Fixture::new(true);
        f.retry_binding();
        f.work_timer();
        let mut other_source = f.document(SOURCE_ID);
        other_source["id"] = "urn:test:retry-source".into();
        if private_role == "retry" {
            other_source["resultProtection"]["purposes"] = json!({"private":true});
        }
        let source = f.artifact(
            "urn:test:retry-source",
            serde_json::to_vec(&other_source).unwrap(),
        );
        let mut other_clock = f.document(CLOCK_ID);
        other_clock["id"] = "urn:test:retry-clock".into();
        other_clock["source"] = json!({"id":source.id,"revision":source.revision});
        let clock = f.artifact(
            "urn:test:retry-clock",
            serde_json::to_vec(&other_clock).unwrap(),
        );
        f.b["policies"]["service_work"]["body"]["retry"]["clock"] =
            json!({"id":clock.id,"revision":clock.revision});
        if private_role == "timeout" {
            f.clock_private();
        }
        f.unit_service(private_role == "activity");
        if private_role == "none" {
            let report = check_timing(&f.envelope(), &f.resources()).unwrap();
            let retry = report.retry_uses().iter().next().unwrap();
            let timeout = report.timer_uses().iter().next().unwrap();
            assert_eq!(retry.clock(), &clock);
            assert_eq!(timeout.clock(), &f.clock);
            assert_ne!(retry.source(), timeout.source());
            continue;
        }
        f.public_after("service_step");
        check_control(&f.envelope(), &f.resources()).unwrap();
        let error = check_protection(&f.envelope(), &f.resources()).unwrap_err();
        assert_eq!(
            error.code, "control-information-flow",
            "{private_role}: {error:?}"
        );
        assert_eq!(error.path, "/body/nodes/publish_public");
    }
    let mut f = Fixture::new(false);
    f.retry_binding();
    f.clock_private();
    f.b["policies"]["review_work"]["body"]["retry"]["clock"] =
        json!({"id":f.clock.id,"revision":f.clock.revision});
    assert_eq!(
        check_protection(&f.envelope(), &f.resources())
            .unwrap_err()
            .path,
        "/body/nodes/review_step"
    );
}

#[test]
fn unvisited_explicit_retries_remain_uncovered_and_single_attempts_need_no_clock() {
    let mut f = Fixture::new(false);
    f.retry_binding();
    f.b["scopes"]["scope_review"]["entry"] = "review_done".into();
    f.b["scopes"]["scope_review"]["outputs"] = json!({});
    let report = check_timing(&f.envelope(), &f.resources()).unwrap();
    assert!(report.retry_uses().is_empty());
    assert!(report.timer_uses().is_empty());
    assert!(report.uncovered_nodes().contains("review_step"));
}

#[test]
fn repeated_retry_policies_share_large_source_metadata_with_bounded_work() {
    let mut f = Fixture::new(false);
    f.retry_binding();
    let mut source = f.document(SOURCE_ID);
    for i in 0..4000 {
        source["resultProtection"]["purposes"][format!("p{i}")] = true.into();
    }
    f.clock_source(source);
    f.b["policies"]["review_work"]["body"]["retry"]["clock"] =
        json!({"id":f.clock.id,"revision":f.clock.revision});
    for i in 0..700 {
        f.b["policies"][format!("retry_{i}")] = f.b["policies"]["review_work"].clone();
    }
    check_timing(&f.envelope(), &f.resources()).unwrap();
}
