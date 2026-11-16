// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Bounded private declaration observation, never actual conflicts or work events.
use super::*;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Projection {
    effects: bool,
    observations: bool,
    reads: Vec<String>,
    writes: Vec<String>,
    read_count: usize,
    write_count: usize,
}
impl Projection {
    fn of(summary: &Summary) -> Self {
        Self {
            effects: summary.effects,
            observations: summary.observations,
            reads: summary.reads.iter().take(64).cloned().collect(),
            writes: summary.writes.iter().take(64).cloned().collect(),
            read_count: summary.reads.len(),
            write_count: summary.writes.len(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Relation {
    site: String,
    child: String,
    peers: Vec<String>,
    peer_count: usize,
    current: Projection,
    previous: Projection,
}
thread_local! {static LAST:RefCell<Option<Relation>>=const{RefCell::new(None)};}
pub(super) fn record(
    site: &str,
    child: &str,
    peers: &(Vec<&str>, usize),
    current: &Summary,
    previous: &Summary,
    error: &Diagnostic,
) {
    if error.code != "parallel-effects-unproved" {
        return;
    }
    LAST.with_borrow_mut(|last| {
        *last = Some(Relation {
            site: site.into(),
            child: child.into(),
            peers: peers.0.iter().map(|s| (*s).into()).collect(),
            peer_count: peers.1,
            current: Projection::of(current),
            previous: Projection::of(previous),
        })
    });
}
fn fixture() -> (DefinitionEnvelope, Vec<(String, String, Vec<u8>)>) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let base = root.join("docs/benchmarks/rp03");
    let envelope =
        DefinitionEnvelope::decode(&std::fs::read(base.join("candidate.ir.json")).unwrap())
            .unwrap();
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(base.join("manifest.json")).unwrap()).unwrap();
    let mut artifacts = vec![];
    for (id, revision) in [
        SEMANTICS, CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, CORE_V6, CORE_V7, CORE_V8, CORE_V9,
        CORE_V10, CORE_V11, CORE_V12,
    ] {
        artifacts.push((
            id.into(),
            revision.into(),
            std::fs::read(root.join(format!(
                "docs/ir/contracts/{}.txt",
                revision.replace(':', "-")
            )))
            .unwrap(),
        ));
    }
    for r in manifest["resources"].as_array().unwrap() {
        artifacts.push((
            r["id"].as_str().unwrap().into(),
            r["revision"].as_str().unwrap().into(),
            std::fs::read(root.join(r["path"].as_str().unwrap())).unwrap(),
        ));
    }
    (envelope, artifacts)
}

#[test]
fn real_full_first_external_comparison_and_guarded_true_variant() {
    let (envelope, artifacts) = fixture();
    let supplied: Vec<_> = artifacts
        .iter()
        .map(|(id, revision, bytes)| Resource {
            id,
            revision,
            bytes,
        })
        .collect();
    for guarded_true in [false, true] {
        let mut b = envelope.document()["body"].clone();
        if guarded_true {
            b["policies"]["order_commits"]["body"]["invariant"] =
                serde_json::json!({"op":"literal","type":{"kind":"boolean"},"value":true});
            b["policies"]["order_commits"]["body"]["reads"] = serde_json::json!({});
        }
        let e =
            DefinitionEnvelope::from_parts_versioned("0.4.0", b, serde_json::json!({})).unwrap();
        check_invalidation(&e, &supplied).unwrap();
        LAST.with_borrow_mut(|last| *last = None);
        let error = check_concurrency(&e, &supplied).unwrap_err();
        assert_eq!(error.code, "parallel-effects-unproved");
        assert_eq!(error.path, "/body/nodes/start_acceptance");
        let relation = LAST.with_borrow(|last| last.clone().unwrap());
        println!("actual external comparison {relation:?}");
        assert_eq!(
            relation,
            Relation {
                site: "start_acceptance".into(),
                child: "accept_b".into(),
                peers: vec!["accept_a".into()],
                peer_count: 1,
                current: Projection {
                    effects: true,
                    observations: true,
                    reads: vec!["original_order".into()],
                    writes: vec!["reserve_b_record".into()],
                    read_count: 1,
                    write_count: 1
                },
                previous: Projection {
                    effects: true,
                    observations: true,
                    reads: vec!["original_order".into()],
                    writes: vec!["reserve_a_record".into()],
                    read_count: 1,
                    write_count: 1
                },
            }
        );
    }
}

#[test]
fn full_finite_summary_inventory_keeps_storage_external_and_declared_work_separate() {
    let (envelope, artifacts) = fixture();
    let supplied: Vec<_> = artifacts
        .iter()
        .map(|(id, revision, bytes)| Resource {
            id,
            revision,
            bytes,
        })
        .collect();
    let invalidation = check_invalidation(&envelope, &supplied).unwrap();
    let b = &envelope.document()["body"];
    let summaries =
        Summary::build(b, invalidation.scope_contexts(), &mut Budget::default()).unwrap();
    for (scope, effect) in [
        ("accept_a", true),
        ("accept_b", true),
        ("accept_payment", true),
        ("prepare_a", true),
        ("prepare_b", true),
        ("delivery_a", false),
        ("delivery_b", false),
    ] {
        let shared = summaries[scope]
            .shared(b, scope, &mut Budget::default())
            .unwrap();
        println!("declared scope {scope}: {:?}", Projection::of(&shared));
        assert_eq!(shared.effects, effect);
        assert!(shared.observations);
    }
    let left = summaries["delivery_a"]
        .shared(b, "delivery_a", &mut Budget::default())
        .unwrap();
    let right = summaries["delivery_b"]
        .shared(b, "delivery_b", &mut Budget::default())
        .unwrap();
    independent(
        &left,
        &right,
        "isolated-declared-delivery-pair",
        &mut Budget::default(),
    )
    .unwrap();
    // This predicate control is not a claim that the complete process passed.
}
