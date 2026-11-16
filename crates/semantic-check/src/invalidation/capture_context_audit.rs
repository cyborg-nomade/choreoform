// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Private bounded declaration-only observer; no source/value/authority diagnostic.
use super::*;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Relation {
    site: String,
    cell: String,
    cell_scope: String,
    use_scope: String,
    ancestors: Vec<String>,
    complete: bool,
}
thread_local! {static LAST: RefCell<Option<Relation>> = const {RefCell::new(None)};}
pub(super) fn record(b: &Value, site: &str, cell: &str, ancestors: &BTreeSet<&str>) {
    LAST.with_borrow_mut(|last| {
        *last = Some(Relation {
            site: site.into(),
            cell: cell.into(),
            cell_scope: b["data"][cell]["scope"].as_str().unwrap().into(),
            use_scope: b["nodes"][site]["scope"].as_str().unwrap().into(),
            ancestors: ancestors.iter().take(64).map(|id| (*id).into()).collect(),
            complete: ancestors.len() <= 64,
        });
    });
}

#[test]
fn whole_current_history_and_partial_join_capture_context_tuple() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let base = root.join("docs/benchmarks/rp03");
    let envelope =
        DefinitionEnvelope::decode(&std::fs::read(base.join("candidate.ir.json")).unwrap())
            .unwrap();
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(base.join("manifest.json")).unwrap()).unwrap();
    let mut artifacts: Vec<(String, String, Vec<u8>)> = vec![];
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
    let supplied: Vec<_> = artifacts
        .iter()
        .map(|(id, revision, bytes)| Resource {
            id,
            revision,
            bytes,
        })
        .collect();
    let (_, dependencies) = crate::protection::analyze(&envelope, &supplied, true).unwrap();
    assert!(!dependencies.nodes["capture_activity"].contains("authorize_call_value"));
    assert!(
        dependencies
            .historical
            .iter()
            .any(|h| h.site() == "capture_activity"
                && h.cell() == "authorize_call_value"
                && h.scope() == "accept_payment"
                && h.boundary() == "start_acceptance_gather")
    );
    // The signed baseline audit captured the original full all-join refusal.
    // This isolated one-field raw variant retains the genuine partial-join negative.
    let mut body = envelope.document()["body"].clone();
    body["nodes"]["start_acceptance_gather"]["predicate"]["op"] = "any".into();
    let envelope =
        DefinitionEnvelope::from_parts_versioned("0.4.0", body, serde_json::json!({})).unwrap();
    for coverage in [Coverage::Required, Coverage::Advisory] {
        LAST.with_borrow_mut(|last| *last = None);
        let e = analyze(&envelope, &supplied, coverage).unwrap_err();
        assert_eq!(e.code, "invalidation-context-unproved");
        assert_eq!(e.path, "/body/nodes/capture_activity");
        let tuple = LAST.with_borrow(|last| last.clone().unwrap());
        println!("actual capture tuple {tuple:?}");
        assert_eq!(
            tuple,
            Relation {
                site: "capture_activity".into(),
                cell: "authorize_call_value".into(),
                cell_scope: "accept_payment".into(),
                use_scope: "capture_service".into(),
                ancestors: vec!["capture_service".into(), "order".into()],
                complete: true,
            }
        );
    }
}
