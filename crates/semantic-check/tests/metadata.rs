// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource, digest};
use choreoform_semantic_check::{
    CORE, CORE_V2, CORE_V3, PolicyReadDependencies, SEMANTICS, check, policy_read_closure,
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const SNAPSHOT: &[u8] = include_bytes!(
    "../../../docs/ir/contracts/sha256-30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3.txt"
);

struct Fixture {
    body: Value,
    resources: Vec<(String, String, Vec<u8>)>,
}
impl Fixture {
    fn new() -> Self {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let m: Value = serde_json::from_slice(
            &std::fs::read(root.join("docs/benchmarks/rp01/manifest.json")).unwrap(),
        )
        .unwrap();
        let candidate: Value = serde_json::from_slice(include_bytes!(
            "../../../docs/benchmarks/rp01/candidate.ir.json"
        ))
        .unwrap();
        let mut f = Self {
            body: candidate["body"].clone(),
            resources: Vec::new(),
        };
        for r in m["resources"].as_array().unwrap() {
            f.resources.push((
                r["id"].as_str().unwrap().into(),
                r["revision"].as_str().unwrap().into(),
                std::fs::read(root.join(r["path"].as_str().unwrap())).unwrap(),
            ));
        }
        for pin in [SEMANTICS, CORE, CORE_V2, CORE_V3] {
            f.resources.push((
                pin.0.into(),
                pin.1.into(),
                std::fs::read(
                    root.join(format!("docs/ir/contracts/{}.txt", pin.1.replace(':', "-"))),
                )
                .unwrap(),
            ));
        }
        f.body["dialects"]["core"] = json!({"id":CORE_V3.0,"revision":CORE_V3.1});
        for key in ["checks", "payment", "notice"] {
            let wrapper = format!("{key}_capability_provider");
            let id = f.body["policies"][&wrapper]["body"]["provider"]["id"]
                .as_str()
                .unwrap()
                .to_owned();
            let bytes = &f.resources.iter().find(|r| r.0 == id).unwrap().2;
            let mut provider: Value = serde_json::from_slice(bytes).unwrap();
            provider["resultProtection"]["capabilities"] = json!({"checks_capability":true,"payment_capability":true,"notice_capability":true});
            let pin = f.replace(provider);
            f.body["policies"][wrapper]["body"]["provider"] = pin;
        }
        let mut source: Value = serde_json::from_slice(
            &f.resources
                .iter()
                .find(|r| r.0.ends_with(":clock-source:0.1.0"))
                .unwrap()
                .2,
        )
        .unwrap();
        source["resultProtection"]["capabilities"] = json!({});
        let source_pin = f.replace(source);
        let mut clock: Value = serde_json::from_slice(
            &f.resources
                .iter()
                .find(|r| r.0.ends_with(":clock:0.1.0"))
                .unwrap()
                .2,
        )
        .unwrap();
        clock["source"] = source_pin;
        let clock_pin = f.replace(clock);
        for role in ["timeout", "retry"] {
            f.body["policies"]["payment_work"]["body"][role]["clock"] = clock_pin.clone();
        }
        f
    }
    fn replace(&mut self, provider: Value) -> Value {
        let id = provider["id"].as_str().unwrap().to_owned();
        let raw = serde_json::to_vec(&provider).unwrap();
        let revision = digest(&raw);
        self.resources.retain(|r| r.0 != id);
        self.resources.push((id.clone(), revision.clone(), raw));
        json!({"id":id,"revision":revision})
    }
    fn supplied(&self) -> Vec<Resource<'_>> {
        self.resources
            .iter()
            .map(|(id, revision, bytes)| Resource {
                id,
                revision,
                bytes,
            })
            .collect()
    }
    fn check(&self) -> choreoform_semantic_check::Result<choreoform_semantic_check::Report> {
        let e =
            DefinitionEnvelope::from_parts(self.body.clone(), json!({"synthetic":true})).unwrap();
        check(&e, &self.supplied())
    }
    fn policy(&mut self, id: &str, body: Value) {
        self.body["policies"][id] = json!({"scope":"expense","dialect":"core","body":body});
    }
    fn with_real_metadata_reads() -> Self {
        let mut f = Self::new();
        let read = |id: &str| json!({"op":"read","cell":id});
        let eq = |v: Value| json!({"op":"eq","left":v,"right":{"op":"literal","type":{"kind":"integer"},"value":"0"}});
        f.policy("meta_protect", json!({"kind":"protection","reads":{"payment_due":true},"permit":eq(read("payment_due"))}));
        f.policy("meta_authority", json!({"kind":"actor","reads":{"disposition":true},"permit":{"op":"eq","left":read("disposition"),"right":{"op":"literal","type":{"kind":"text"},"value":"pending"}}}));
        f.body["actors"]["meta_actor"] = json!({"scope":"expense","requirement":"meta_authority"});
        let mut wait = f.body["policies"]["checks_wait"]["body"].clone();
        wait["authority"] = "meta_actor".into();
        wait["match"] = read("review_needed");
        wait["reads"] = json!({"disposition":true,"review_needed":true});
        f.policy("meta_wait", wait);
        let mut observe = f.body["nodes"]["checks_observe"].clone();
        observe["scope"] = "expense".into();
        observe["policy"] = "meta_wait".into();
        observe["reads"] = json!({"disposition":true,"review_needed":true});
        f.body["nodes"]["meta_observe"] = observe;
        let mut flow = f.body["flows"]
            .as_object()
            .unwrap()
            .values()
            .find(|v| v["source"] == "checks_observe")
            .unwrap()
            .clone();
        flow["source"] = "meta_observe".into();
        flow["target"] = "finish_paid".into();
        f.body["flows"]["meta_flow"] = flow;

        let mut effect = f.body["policies"]["checks_effects"]["body"].clone();
        effect["reconciliation"] = "meta_wait".into();
        effect["reads"] = json!({"disposition":true,"review_needed":true});
        f.policy("meta_effect", effect);
        let mut cap = f.body["capabilities"]["checks_capability"].clone();
        cap["contract"] = "meta_provider".into();
        cap["effects"] = "meta_effect".into();
        cap["authority"] = "meta_actor".into();
        f.body["capabilities"]["meta_capability"] = cap;
        let mut provider: Value = serde_json::from_slice(
            &f.resources
                .iter()
                .find(|r| r.0.ends_with(":checks:0.1.0"))
                .unwrap()
                .2,
        )
        .unwrap();
        provider["id"] = "urn:test:metadata-provider".into();
        provider["resultProtection"]["policy"] = "meta_protect".into();
        provider["resultProtection"]["participants"] = json!({"meta_actor":true});
        provider["resultProtection"]["capabilities"] = json!({"meta_capability":true});
        let pin = f.replace(provider);
        f.policy("meta_provider", json!({"kind":"capability","reads":{"disposition":true,"review_needed":true,"payment_due":true},"provider":pin}));
        f
    }
}

#[test]
fn real_shared_recipient_cycle_requires_explicit_core_and_inherited_bytes() {
    let mut f = Fixture::new();
    assert_eq!(digest(SNAPSHOT), CORE_V3.1);
    assert_eq!(
        SNAPSHOT,
        include_bytes!("../../../docs/dialects/core-metadata-v0.3.md")
    );
    let report = f.check().unwrap();
    assert!(!report.remaining().is_empty());
    let original = f.body.clone();
    for legacy in [CORE, CORE_V2] {
        f.body = original.clone();
        if legacy == CORE {
            for p in f.body["policies"].as_object_mut().unwrap().values_mut() {
                if p["body"]["kind"] == "work" {
                    let retry = p["body"]["retry"].as_object_mut().unwrap();
                    retry.remove("clock");
                    retry.remove("basis");
                }
            }
        }
        f.body["dialects"]["core"] = json!({"id":legacy.0,"revision":legacy.1});
        assert_eq!(f.check().unwrap_err().code, "policy-cycle");
    }
    f.body["dialects"]["core"] = json!({"id":CORE_V3.0,"revision":CORE_V3.1});
    for pin in [CORE, CORE_V2, CORE_V3] {
        let index = f.resources.iter().position(|r| r.0 == pin.0).unwrap();
        let row = f.resources.remove(index);
        assert_eq!(f.check().unwrap_err().code, "resource-missing");
        f.resources.push(row);
    }
    f.body["dialects"]["core"]["revision"] = format!("sha256:{}", "0".repeat(64)).into();
    assert_eq!(f.check().unwrap_err().code, "dialect-contract");
    f.body["dialects"]["core"]["revision"] = CORE_V3.1.into();
    let i = f.resources.iter().position(|r| r.0 == CORE_V3.0).unwrap();
    f.resources[i].2.push(b' ');
    assert_eq!(f.check().unwrap_err().code, "resource-digest");
}

#[test]
fn actual_protection_actor_effect_and_wait_reads_survive_self_metadata_closure() {
    let mut f = Fixture::with_real_metadata_reads();
    f.check().unwrap();
    f.body["policies"]["meta_provider"]["body"]["reads"] = json!({});
    let e = f.check().unwrap_err();
    assert_eq!(e.code, "policy-reads");
    assert_eq!(e.path, "/body/policies/meta_provider/body/reads");
}

#[test]
fn mixed_cyclic_records_and_bad_result_references_still_refuse() {
    let mut f = Fixture::new();
    f.body["dialects"]["legacy"] = json!({"id":CORE_V2.0,"revision":CORE_V2.1});
    f.body["policies"]["checks_capability_provider"]["dialect"] = "legacy".into();
    assert_eq!(f.check().unwrap_err().code, "policy-cycle");
    let mut f = Fixture::with_real_metadata_reads();
    let mut p: Value = serde_json::from_slice(
        &f.resources
            .iter()
            .find(|r| r.0 == "urn:test:metadata-provider")
            .unwrap()
            .2,
    )
    .unwrap();
    p["resultProtection"]["policy"] = "operator_actor".into();
    let pin = f.replace(p);
    f.body["policies"]["meta_provider"]["body"]["provider"] = pin;
    assert_eq!(f.check().unwrap_err().code, "reference");
    let mut f = Fixture::with_real_metadata_reads();
    f.body["capabilities"]["meta_capability"]["scope"] = "checks_service".into();
    assert_eq!(f.check().unwrap_err().code, "visibility");
}

fn node(
    local: &[&str],
    strong: &[&str],
    metadata: &[&str],
    eligible: bool,
) -> PolicyReadDependencies {
    let set = |x: &[&str]| x.iter().map(|s| (*s).to_owned()).collect();
    PolicyReadDependencies {
        local: set(local),
        strong: set(strong),
        metadata: set(metadata),
        metadata_cycles: eligible,
    }
}

#[test]
fn iterative_large_metadata_ring_and_fixed_caller_budgets() {
    let mut graph = BTreeMap::new();
    for i in 0..5000 {
        graph.insert(
            format!("p{i}"),
            PolicyReadDependencies {
                local: if i == 0 {
                    BTreeSet::from(["claim".into()])
                } else {
                    BTreeSet::new()
                },
                metadata: BTreeSet::from([format!("p{}", (i + 1) % 5000)]),
                metadata_cycles: true,
                ..Default::default()
            },
        );
    }
    let closure = policy_read_closure(&graph, 1_000_000).unwrap();
    assert_eq!(closure.reads.len(), 5000);
    assert!(
        closure
            .reads
            .values()
            .all(|r| r == &BTreeSet::from(["claim".into()]))
    );
    assert_eq!(
        policy_read_closure(&graph, closure.work - 1)
            .unwrap_err()
            .code,
        "budget"
    );
    assert_eq!(
        policy_read_closure(&graph, 1_000_001).unwrap_err().code,
        "budget"
    );
    for i in 0..5000 {
        graph
            .get_mut(&format!("p{i}"))
            .unwrap()
            .local
            .insert(format!("cell{i}"));
    }
    assert_eq!(
        policy_read_closure(&graph, 1_000_000).unwrap_err().code,
        "budget"
    );
}

#[test]
fn strong_and_condensed_depth_and_duplicate_edge_rules_remain_closed() {
    for weak in [false, true] {
        let mut graph = BTreeMap::new();
        for i in 0..64 {
            let next = format!("p{}", i + 1);
            let refs = [next.as_str()];
            graph.insert(
                format!("p{i}"),
                node(
                    &[],
                    if !weak && i < 63 { &refs } else { &[] },
                    if weak && i < 63 { &refs } else { &[] },
                    true,
                ),
            );
        }
        policy_read_closure(&graph, 1_000_000).unwrap();
        graph.insert(
            "tail".into(),
            node(
                &[],
                if weak { &[] } else { &["p0"] },
                if weak { &["p0"] } else { &[] },
                true,
            ),
        );
        assert_eq!(
            policy_read_closure(&graph, 1_000_000).unwrap_err().code,
            "policy-depth"
        );
    }
    let graph = BTreeMap::from([
        ("a".into(), node(&[], &["b"], &["b"], true)),
        ("b".into(), node(&[], &["a"], &[], true)),
    ]);
    assert_eq!(
        policy_read_closure(&graph, 1_000_000).unwrap_err().code,
        "policy-cycle"
    );
}

#[test]
fn independent_fixed_point_conformance_vectors() {
    let cases: Value =
        serde_json::from_slice(include_bytes!("conformance/policy-reads.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let graph: BTreeMap<_, _> = case["nodes"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(id, v)| {
                let set = |field: &str| {
                    v[field]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|s| s.as_str().unwrap().to_owned())
                        .collect()
                };
                (
                    id.clone(),
                    PolicyReadDependencies {
                        local: set("local"),
                        strong: set("strong"),
                        metadata: set("metadata"),
                        metadata_cycles: v["metadata_cycles"].as_bool().unwrap(),
                    },
                )
            })
            .collect();
        let result = match policy_read_closure(&graph, 1_000_000) {
            Ok(r) => json!({"reads":r.reads}),
            Err(e) => json!({"code":e.code}),
        };
        assert_eq!(result, case["expected"], "{}", case["name"]);
    }
}

#[test]
fn timeout_and_retry_clock_metadata_keep_their_transitive_reads() {
    for roles in [&["timeout"][..], &["retry"][..], &["timeout", "retry"][..]] {
        let mut f = Fixture::with_real_metadata_reads();
        let mut source: Value = serde_json::from_slice(
            &f.resources
                .iter()
                .find(|r| r.0.ends_with(":clock-source:0.1.0"))
                .unwrap()
                .2,
        )
        .unwrap();
        source["id"] = "urn:test:metadata-clock-source".into();
        source["resultProtection"] = json!({"policy":"meta_protect","sensitivity":"expense","purposes":{"reimbursement":true},"participants":{"meta_actor":true},"capabilities":{"meta_capability":true}});
        let source_pin = f.replace(source);
        let mut clock: Value = serde_json::from_slice(
            &f.resources
                .iter()
                .find(|r| r.0.ends_with(":clock:0.1.0"))
                .unwrap()
                .2,
        )
        .unwrap();
        clock["id"] = "urn:test:metadata-clock".into();
        clock["source"] = source_pin;
        let clock_pin = f.replace(clock);
        for role in roles {
            f.body["policies"]["payment_work"]["body"][*role]["clock"] = clock_pin.clone();
        }
        for cell in ["payment_due", "disposition", "review_needed"] {
            f.body["policies"]["payment_work"]["body"]["reads"][cell] = true.into();
            f.body["nodes"]["payment_request_step"]["reads"][cell] = true.into();
        }
        f.check().unwrap();
        f.body["policies"]["payment_work"]["body"]["reads"]
            .as_object_mut()
            .unwrap()
            .remove("disposition");
        let e = f.check().unwrap_err();
        assert_eq!(e.code, "policy-reads");
        assert_eq!(e.path, "/body/policies/payment_work/body/reads");
    }
}

#[test]
fn raw_package_control_pass_does_not_hide_its_separate_clock_label_refusal() {
    let f = Fixture::new();
    let e = DefinitionEnvelope::from_parts(f.body.clone(), json!({})).unwrap();
    let r = f.supplied();
    choreoform_semantic_check::check_control(&e, &r).unwrap();
    let refusal = choreoform_semantic_check::check_protection(&e, &r).unwrap_err();
    assert_eq!(refusal.code, "control-information-flow");
    assert_eq!(refusal.path, "/body/nodes/payment_request_step");
}

#[test]
fn coherent_raw_labels_keep_strict_coverage_and_separate_lifetime_requirements() {
    let mut f = Fixture::new();
    let mut source: Value = serde_json::from_slice(
        &f.resources
            .iter()
            .find(|r| r.0.ends_with(":clock-source:0.1.0"))
            .unwrap()
            .2,
    )
    .unwrap();
    source["id"] = "urn:test:shared-expense-clock-source".into();
    source["resultProtection"]["capabilities"] =
        json!({"checks_capability":true,"payment_capability":true,"notice_capability":true});
    let source_pin = f.replace(source);
    let mut clock: Value = serde_json::from_slice(
        &f.resources
            .iter()
            .find(|r| r.0.ends_with(":clock:0.1.0"))
            .unwrap()
            .2,
    )
    .unwrap();
    clock["id"] = "urn:test:shared-expense-clock".into();
    clock["source"] = source_pin;
    let clock_pin = f.replace(clock);
    for role in ["timeout", "retry"] {
        f.body["policies"]["payment_work"]["body"][role]["clock"] = clock_pin.clone();
    }
    let e = DefinitionEnvelope::from_parts(f.body.clone(), json!({})).unwrap();
    let r = f.supplied();
    choreoform_semantic_check::check_protection(&e, &r).unwrap();
    choreoform_semantic_check::check_invalidation(&e, &r).unwrap();
    assert_eq!(
        choreoform_semantic_check::check_concurrency(&e, &r)
            .unwrap_err()
            .code,
        "scope-lifetime-unproved"
    );
    let mut body = f.body.clone();
    body["data"]["disposition"]["invalidates"] = json!({});
    let e = DefinitionEnvelope::from_parts(body, json!({})).unwrap();
    let refusal = choreoform_semantic_check::check_invalidation(&e, &r).unwrap_err();
    assert_eq!(refusal.code, "invalidation-coverage");
    assert_eq!(refusal.path, "/body/data/disposition/invalidates");
}
