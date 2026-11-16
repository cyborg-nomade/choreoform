// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Readable conditional declarations, never accepted events or ownership receipts.
use choreoform_authoring_frontend::{Candidate, Package};
use choreoform_ir_probe_core::Resource;
use choreoform_semantic_check::{CORE_V10, CORE_V11};
use serde_json::{Value, json};

struct Fixture {
    source: String,
    ledger: Value,
    artifacts: Vec<(String, String, Vec<u8>)>,
}
impl Fixture {
    fn new() -> Self {
        let source = include_str!("capture/finite-work.choreo").into();
        let ledger =
            serde_json::from_slice(include_bytes!("capture/finite-work.bindings.json")).unwrap();
        let raw: Value =
            serde_json::from_slice(include_bytes!("capture/finite-work.resources.json")).unwrap();
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
                (r["id"].as_str().unwrap().to_owned(), revision, bytes)
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
    let mut raw: Value = serde_json::from_slice(include_bytes!(
        "../../semantic-check/tests/conformance/owned-readonly-definition.json"
    ))
    .unwrap();
    raw["definition"]["body"]["nodes"]["lookup"]["reads"] = json!({"captured":true});
    raw["definition"]["body"]["expressions"]["lookup_input"]["reads"] = json!({"captured":true});
    raw["definition"]["body"]["expressions"]["lookup_input"]["body"] =
        json!({"op":"payload","value":{"op":"read","cell":"captured"},"tag":"confirmed"});
    raw["definition"]["body"]["expressions"]["lookup_input"]["resultType"] =
        json!({"dialect":"core","body":{"kind":"integer"}});
    for (site, binding) in [
        ("capture_wait", "terminal_resource"),
        ("unused_lookup_wait", "command_terminal_resource"),
    ] {
        let resource = f.ledger["resources"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["binding"] == binding)
            .unwrap();
        raw["definition"]["body"]["nodes"][site]["capture"]["source"] =
            json!({"id":resource["contract"],"revision":resource["digest"]});
    }
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
        json!({"id":CORE_V11.0,"revision":CORE_V11.1})
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
    let root = tree.continuations().iter().next().unwrap();
    let levels: Vec<_> = root.scopes().collect();
    assert!(levels[0].pre_capture_entry().is_none());
    assert!(levels[0].query().is_none());
    let work = levels[0].work().unwrap();
    assert_eq!(
        (work.site(), work.capability(), work.input()),
        ("lookup", "adapter_lookup", "lookup_input")
    );
    assert_eq!(work.effect_class().as_str(), "irreversible");
    assert_eq!(work.compensates(), Some("origin"));
    assert!(!work.host_requirements().is_empty());
    assert!(levels[1].work().is_none());
    assert_ne!(
        c.document()["body"]["data"]["original_request"]["type"],
        c.document()["body"]["data"]["lookup_request"]["type"]
    );
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
        .replace("\"Adjustment recovery\"", "\"Fólio de recuperação\"")
        .replace(
            "from \"Adjust account\" and record",
            "from \"Adjust account\" in \"Recovery\" in \"Root\" in \"Main\" and record",
        );
    for row in f.ledger["declarations"].as_array_mut().unwrap() {
        if row["id"] == "lookup_recovery" {
            row["name"] = "Fólio de recuperação".into();
        }
    }
    assert_eq!(f.lower().document(), &baseline);
    let clause = "Retain the original request from \"Adjust account\" in \"Recovery\" in \"Root\" in \"Main\" and record in \"Adjustment captured\".";
    f.source = f.source.replace(
        "and record in \"Adjustment request\".",
        "and record in \"Adjustment captured\".",
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
    for (id, _, _) in &f.artifacts {
        let supplied: Vec<_> = f.supplied().into_iter().filter(|r| r.id != id).collect();
        let refused = match f.package().lower(&supplied) {
            Err(_) => true,
            Ok(c) => c.check_static(&supplied).is_err(),
        };
        assert!(refused, "{id}");
    }
    let mut f = Fixture::new();
    f.ledger["version"] = "0.12.0".into();
    assert!(Package::parse(f.source.as_bytes(), &serde_json::to_vec(&f.ledger).unwrap()).is_err());
    let mut f = Fixture::new();
    f.source = f
        .source
        .replace("authoring \"0.14.0\"", "authoring \"0.13.0\"");
    f.ledger["profile"] = "0.13.0".into();
    f.ledger["version"] = "0.12.0".into();
    f.ledger["contracts"]["core"] =
        json!({"contract":CORE_V10.0,"digest":CORE_V10.1,"dialect":"core"});
    let c = f.lower();
    c.check_static(&f.supplied()).unwrap();
    assert_eq!(
        c.check_concurrency(&f.supplied()).unwrap_err().code,
        "owned-continuation-query-effect"
    );
}

#[test]
fn normal_source_retains_command_entry_capture_integrity_and_compensation_barriers() {
    let mut f = Fixture::new();
    f.source = f
        .source
        .replace("Start at \"Capture wait\".", "Start at \"Adjust account\".");
    let c = f.lower();
    c.check_static(&f.supplied()).unwrap();
    assert_eq!(
        c.check_concurrency(&f.supplied()).unwrap_err().code,
        "owned-continuation-work-entry"
    );
    let mut f = Fixture::new();
    f.source = f.source.replace(
        "Compensates \"Create payable\".",
        "Compensates \"Adjust account\".",
    );
    let c = f.lower();
    c.check_static(&f.supplied()).unwrap();
    assert_eq!(
        c.check_concurrency(&f.supplied()).unwrap_err().code,
        "owned-continuation-compensation-source"
    );
    let mut f = Fixture::new();
    let start = f.source.find("Keep \"Captured\"").unwrap();
    let end = start + f.source[start..].find("End data.").unwrap();
    let block=f.source[start..end].replace("Initially unavailable.","Initially (case \"confirmed\" of choice with [\"confirmed\" as whole number] containing (0)).");
    f.source.replace_range(start..end, &block);
    f.ledger["generated"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"invented_capture","owner":"captured","role":"initial","key":null}));
    let c = f.lower();
    c.check_static(&f.supplied()).unwrap();
    assert_eq!(
        c.check_concurrency(&f.supplied()).unwrap_err().code,
        "retained-request-source-result"
    );
}
