// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Real isolated raw/source judgments; synthetic resources confer no host authority.
use choreoform_authoring_frontend::Package;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};
use choreoform_semantic_check::{
    CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, CORE_V6, CORE_V7, CORE_V8, CORE_V9, CORE_V10,
    CORE_V11, CORE_V12, SEMANTICS, check, check_invalidation, check_reconciliation_availability,
};
use serde_json::{Value, json};

struct Fixture {
    source: String,
    ledger: Value,
    artifacts: Vec<(String, String, Vec<u8>)>,
}
impl Fixture {
    fn new() -> Self {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let resources: Value =
            serde_json::from_slice(include_bytes!("capture/effectful-instances.resources.json"))
                .unwrap();
        let mut artifacts: Vec<_> = resources
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                (
                    r["id"].as_str().unwrap().into(),
                    r["revision"].as_str().unwrap().into(),
                    r["bytes"].as_str().unwrap().as_bytes().to_vec(),
                )
            })
            .collect();
        for (id, revision) in [
            SEMANTICS, CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, CORE_V6, CORE_V7, CORE_V8,
            CORE_V9, CORE_V10, CORE_V11, CORE_V12,
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
        Self {
            source: include_str!("capture/effectful-instances.choreo").into(),
            ledger: serde_json::from_slice(include_bytes!(
                "capture/effectful-instances.bindings.json"
            ))
            .unwrap(),
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
    fn body(&self) -> Value {
        let bytes = serde_json::to_vec(&self.ledger).unwrap();
        let p = Package::parse(self.source.as_bytes(), &bytes).unwrap();
        let candidate = p.lower(&self.supplied()).unwrap();
        candidate
            .check_reconciliation_availability(&self.supplied())
            .unwrap();
        candidate.document()["body"].clone()
    }
}
fn raw(body: Value) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts_versioned("0.4.0", body, json!({})).unwrap()
}

#[test]
fn complete_two_caller_source_and_isolated_raw_body_keep_same_judgment_and_local_identity() {
    let f = Fixture::new();
    let b = f.body();
    let e = raw(b.clone());
    check_reconciliation_availability(&e, &f.supplied()).unwrap();
    for key in ["a", "b"] {
        assert_eq!(
            b["scopes"][format!("{key}_service")]["parent"],
            format!("{key}_caller")
        );
        assert_eq!(
            b["nodes"][format!("{key}_observe")]["capture"]["origin"],
            format!("{key}_activity")
        );
        assert_eq!(
            b["policies"][format!("{key}_work")]["body"]["completion"]["op"],
            "eq"
        );
    }
    let bytes = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
    let (source, ledger) = p.formatted().unwrap();
    let q = Package::parse(source.as_bytes(), ledger.as_bytes()).unwrap();
    assert_eq!(q.lower(&f.supplied()).unwrap().document()["body"], b);
}

#[test]
fn raw_parent_port_type_and_private_value_mutations_refuse_independently() {
    let f = Fixture::new();
    let baseline = f.body();
    for (case, code) in [
        ("parent", "visibility"),
        ("port", "invoke-ports"),
        ("type", "invoke-type"),
        ("private", "cell-reference"),
    ] {
        let mut b = baseline.clone();
        match case {
            "parent" => b["scopes"]["a_service"]["parent"] = "b_caller".into(),
            "port" => b["nodes"]["a_run"]["inputs"] = json!({}),
            "type" => {
                b["expressions"]["a_caller_input"]["body"] =
                    json!({"op":"literal","type":{"kind":"text"},"value":"synthetic"});
                b["expressions"]["a_caller_input"]["resultType"] =
                    json!({"dialect":"core","body":{"kind":"text"}});
                b["expressions"]["a_caller_input"]["reads"] = json!({});
                b["nodes"]["a_run"]["reads"] = json!({})
            }
            "private" => {
                b["expressions"]["b_input"]["body"]["cell"] = "a_request".into();
                b["expressions"]["b_input"]["reads"] = json!({"a_request":true});
            }
            _ => unreachable!(),
        }
        assert_eq!(
            check(&raw(b), &f.supplied()).unwrap_err().code,
            code,
            "{case}"
        );
    }
}

#[test]
fn raw_transitive_invalidation_and_capture_storage_remain_required() {
    let f = Fixture::new();
    let mut b = f.body();
    b["data"]["launch"]["invalidates"] = json!({});
    assert_eq!(
        check_invalidation(&raw(b), &f.supplied()).unwrap_err().code,
        "invalidation-coverage"
    );
    let mut b = f.body();
    b["scopes"]["a_recovery"]["retainedRequests"] = json!({"a_captured":"a_activity"});
    assert_eq!(
        check(&raw(b), &f.supplied()).unwrap_err().code,
        "retained-request-readonly"
    );
}
