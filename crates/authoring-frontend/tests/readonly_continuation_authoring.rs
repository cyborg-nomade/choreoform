// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Readable conditional declarations, never accepted events or ownership receipts.
use choreoform_authoring_frontend::{Candidate, Package};
use choreoform_ir_probe_core::Resource;
use choreoform_semantic_check::{CORE_V8, CORE_V9};
use serde_json::{Value, json};

struct Fixture {
    source: String,
    ledger: Value,
    artifacts: Vec<(String, String, Vec<u8>)>,
}
impl Fixture {
    fn new() -> Self {
        let source = include_str!("capture/readonly-continuation.choreo").into();
        let ledger = serde_json::from_slice(include_bytes!(
            "capture/readonly-continuation.bindings.json"
        ))
        .unwrap();
        let raw: Value = serde_json::from_slice(include_bytes!(
            "../../semantic-check/tests/conformance/owned-readonly-definition.json"
        ))
        .unwrap();
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let artifacts = raw["normative"]
            .as_array()
            .unwrap()
            .iter()
            .chain(raw["resources"].as_array().unwrap())
            .map(|r| {
                let revision = r["revision"].as_str().unwrap().to_owned();
                let bytes = if let Some(raw) = r["bytes"].as_str() {
                    raw.as_bytes().to_vec()
                } else {
                    std::fs::read(root.join(format!(
                        "docs/ir/contracts/{}.txt",
                        revision.replace(':', "-")
                    )))
                    .unwrap()
                };
                (r["id"].as_str().unwrap().into(), revision, bytes)
            })
            .collect();
        Self {
            source,
            ledger,
            artifacts,
        }
    }
    fn supplied(&self) -> Vec<Resource<'_>> {
        self.artifacts
            .iter()
            .map(|(id, revision, bytes)| Resource {
                id,
                revision,
                bytes,
            })
            .collect()
    }
    fn package(&self) -> Package<'_> {
        Package::parse(
            self.source.as_bytes(),
            &serde_json::to_vec(&self.ledger).unwrap(),
        )
        .unwrap()
    }
    fn lower(&self) -> Candidate {
        self.package().lower(&self.supplied()).unwrap()
    }
}
#[test]
fn readable_chain_matches_raw_scope_node_flow_expression_relations_and_every_sdk_view() {
    let f = Fixture::new();
    let c = f.lower();
    let r = f.supplied();
    let raw: Value = serde_json::from_slice(include_bytes!(
        "../../semantic-check/tests/conformance/owned-readonly-definition.json"
    ))
    .unwrap();
    for table in ["scopes", "nodes", "flows", "expressions"] {
        assert_eq!(
            c.document()["body"][table],
            raw["definition"]["body"][table],
            "{table}"
        );
    }
    assert_eq!(c.document()["version"], "0.4.0");
    assert_eq!(
        c.document()["body"]["dialects"]["core"],
        json!({"id":CORE_V9.0,"revision":CORE_V9.1})
    );
    let static_report = c.check_static(&r).unwrap();
    assert_eq!(static_report.retained_requests().len(), 2);
    assert_eq!(static_report.capture_bindings().len(), 2);
    c.check_control(&r).unwrap();
    c.check_protection(&r).unwrap();
    c.check_invalidation(&r).unwrap();
    c.invalidation_requirements(&r).unwrap();
    let concurrency = c.check_concurrency(&r).unwrap();
    let tree = concurrency.owned_continuations().unwrap();
    assert_eq!(tree.checked_scopes().len(), 2);
    assert_eq!(tree.gates().len(), 7);
    assert!(concurrency.owned_local().is_none());
    c.check_timing(&r).unwrap();
    c.check_fault_routes(&r).unwrap();
    c.check_exceptional_availability(&r).unwrap();
    c.check_reconciliation_links(&r).unwrap();
    let last = c.check_reconciliation_availability(&r).unwrap();
    assert_eq!(last.continuation_scopes(), tree.checked_scopes());
    assert!(last.retained_request_scopes().is_empty());
    assert!(!last.checked_nodes().contains("lookup"));
    assert!(last.uncovered_nodes().contains("lookup"));
}
#[test]
fn format_unicode_scope_alias_qualified_origin_and_private_clause_spans_survive() {
    let mut f = Fixture::new();
    let baseline = f.lower().document().clone();
    let (source, bindings) = f.package().formatted().unwrap();
    let c = Package::parse(source.as_bytes(), bindings.as_bytes())
        .unwrap()
        .lower(&f.supplied())
        .unwrap();
    assert_eq!(c.document(), &baseline);
    f.source = f
        .source
        .replace("\"Lookup recovery\"", "\"Fólio de recuperação\"")
        .replace(
            "from \"Lookup\" and record",
            "from \"Lookup\" in \"Recovery\" in \"Root\" in \"Main\" and record",
        );
    for row in f.ledger["declarations"].as_array_mut().unwrap() {
        if row["id"] == "lookup_recovery" {
            row["name"] = "Fólio de recuperação".into();
        }
    }
    assert_eq!(f.lower().document(), &baseline);
    let clause = "Retain the original request from \"Lookup\" in \"Recovery\" in \"Root\" in \"Main\" and record in \"Lookup captured\".";
    f.source = f.source.replace(
        "and record in \"Lookup request\".",
        "and record in \"Lookup captured\".",
    );
    let c = f.lower();
    let error = c.check_static(&f.supplied()).unwrap_err();
    assert_eq!(error.code, "retained-request-readonly");
    let span = c.source_span(&error).unwrap();
    assert_eq!(&f.source[span], clause);
}
#[test]
fn exact_new_pair_and_all_required_artifacts_do_not_upgrade_an_older_profile() {
    let f = Fixture::new();
    for (id, _, _) in f
        .artifacts
        .iter()
        .filter(|(id, _, _)| id.contains("core-profile") || id.contains("semantics:"))
    {
        let supplied: Vec<_> = f.supplied().into_iter().filter(|r| r.id != id).collect();
        assert!(f.package().lower(&supplied).is_err(), "{id}");
    }
    let mut f = Fixture::new();
    f.ledger["version"] = "0.10.0".into();
    assert!(Package::parse(f.source.as_bytes(), &serde_json::to_vec(&f.ledger).unwrap()).is_err());
    let mut f = Fixture::new();
    f.source = f
        .source
        .replace("authoring \"0.12.0\"", "authoring \"0.11.0\"");
    f.ledger["profile"] = "0.11.0".into();
    f.ledger["version"] = "0.10.0".into();
    f.ledger["contracts"]["core"] =
        json!({"contract":CORE_V8.0,"digest":CORE_V8.1,"dialect":"core"});
    let c = f.lower();
    assert_eq!(
        c.check_static(&f.supplied()).unwrap_err().code,
        "retained-request-creator"
    );
}
