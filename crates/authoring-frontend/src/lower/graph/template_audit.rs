// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Test-only declared template relations, never actual occurrence ownership.
use crate::Package;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};
use choreoform_semantic_check::{
    CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, CORE_V6, CORE_V7, CORE_V8, CORE_V9, CORE_V10,
    CORE_V11, CORE_V12, SEMANTICS,
};
use serde_json::{Value, json};
use std::cell::RefCell;
type Relation = (String, String, String, String, Option<String>);
thread_local! {static LAST:RefCell<(usize,Option<Relation>)>=const {RefCell::new((0,None))};}
pub(super) fn record(id: &str, node: &Value, body: &str, parent: &Value) {
    LAST.with_borrow_mut(|last| {
        last.0 += 1;
        last.1 = Some((
            id.into(),
            node["kind"].as_str().unwrap().into(),
            node["scope"].as_str().unwrap().into(),
            body.into(),
            parent.as_str().map(str::to_owned),
        ));
    });
}
struct Fixture {
    source: String,
    ledger: Value,
    artifacts: Vec<(String, String, Vec<u8>)>,
}
impl Fixture {
    fn full() -> Self {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let base = root.join("docs/benchmarks/rp03");
        let source = std::fs::read_to_string(base.join("source.choreo")).unwrap();
        let ledger =
            serde_json::from_slice(&std::fs::read(base.join("bindings.json")).unwrap()).unwrap();
        let manifest: Value =
            serde_json::from_slice(&std::fs::read(base.join("manifest.json")).unwrap()).unwrap();
        let mut artifacts: Vec<_> = [
            SEMANTICS, CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, CORE_V6, CORE_V7, CORE_V8,
            CORE_V9, CORE_V10, CORE_V11, CORE_V12,
        ]
        .iter()
        .map(|(id, revision)| {
            (
                (*id).into(),
                (*revision).into(),
                std::fs::read(root.join(format!(
                    "docs/ir/contracts/{}.txt",
                    revision.replace(':', "-")
                )))
                .unwrap(),
            )
        })
        .collect();
        for r in manifest["resources"].as_array().unwrap() {
            let id = r["id"].as_str().unwrap();
            let revision = r["revision"].as_str().unwrap();
            if !artifacts.iter().any(|(i, v, _)| i == id && v == revision) {
                artifacts.push((
                    id.into(),
                    revision.into(),
                    std::fs::read(root.join(r["path"].as_str().unwrap())).unwrap(),
                ));
            }
        }
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
    fn bounded() -> Self {
        let mut f = Self::full();
        f.artifacts
            .retain(|(id, _, _)| id == SEMANTICS.0 || id.contains("core-profile:"));
        let interface = |entry: &str| {
            format!(
                "Start at {entry:?}.\nInputs empty. Outputs empty. Outcomes [\"complete\"].\nUse cancellation \"Cancel\", faults \"Faults\", closure \"Close\" and races \"Race\".\n"
            )
        };
        f.source = "Choreoform authoring \"0.16.0\".\nProcess \"Main\".\n".to_owned()
            + &interface("Use A")
            + "Rule for actor \"Authority\". Permit when (true). End policy.\nRequire actor \"Operator\" under \"Authority\".\nRule for cancel \"Cancel\". Authority \"Operator\". For children use \"Settlement\". End policy.\nRule for settlement \"Settlement\". Unfinished work cancel. End policy.\nRule for closure \"Close\". Fully terminal. End policy.\nRule for faults \"Faults\". Handle empty. End policy.\nRule for race \"Race\". Accept in acceptance order. End policy.\nStep \"Use A\". Run \"A\" with inputs empty and outputs empty. On \"complete\", continue with \"Use B\". End step.\nStep \"Use B\". Run \"B\" with inputs empty and outputs empty. On \"complete\", continue with \"Main done\". End step.\nStep \"Main done\". Finish as \"complete\". End step.\nScope \"A\" within \"Main\".\n"
            + &interface("A done")
            + "Step \"A done\". Finish as \"complete\". End step.\nEnd scope.\nScope \"B\" within \"Main\".\n"
            + &interface("Use C")
            + "Step \"Use C\". Run \"C\" with inputs empty and outputs empty. On \"complete\", continue with \"B done\". End step.\nStep \"B done\". Finish as \"complete\". End step.\nScope \"C\" within \"B\".\n"
            + &interface("C done")
            + "Step \"C done\". Finish as \"complete\". End step.\nEnd scope.\nEnd scope.\nEnd process.\n";
        f.ledger = json!({"format":"choreoform-authoring-bindings","profile":"0.16.0","version":"0.15.0","definition":"bounded_template_audit","root":"main","contracts":{"semantics":{"contract":SEMANTICS.0,"digest":SEMANTICS.1},"core":{"contract":CORE_V12.0,"digest":CORE_V12.1,"dialect":"core"}},"declarations":[],"symbols":[],"generated":[],"resources":[],"retired":[],"annotations":{}});
        for (id, parent, name) in [
            ("main", None, "Main"),
            ("a", Some("main"), "A"),
            ("b", Some("main"), "B"),
            ("c", Some("b"), "C"),
        ] {
            f.ledger["declarations"]
                .as_array_mut()
                .unwrap()
                .push(json!({"id":id,"scope":parent,"kind":"scope","name":name}));
            f.ledger["symbols"].as_array_mut().unwrap().push(json!({"id":format!("{id}_outcome_complete"),"owner":id,"role":"outcome","name":"complete","wire":"complete"}));
        }
        for (id, scope, name) in [
            ("use_a", "main", "Use A"),
            ("use_b", "main", "Use B"),
            ("main_done", "main", "Main done"),
            ("a_done", "a", "A done"),
            ("use_c", "b", "Use C"),
            ("b_done", "b", "B done"),
            ("c_done", "c", "C done"),
        ] {
            f.ledger["declarations"]
                .as_array_mut()
                .unwrap()
                .push(json!({"id":id,"scope":scope,"kind":"node","name":name}));
            if id.starts_with("use_") {
                f.ledger["symbols"].as_array_mut().unwrap().push(json!({"id":format!("{id}_outcome_complete"),"owner":id,"role":"outcome","name":"complete","wire":"complete"}));
                f.ledger["generated"].as_array_mut().unwrap().push(json!({"id":format!("{id}_next"),"owner":id,"role":"flow","key":format!("{id}_outcome_complete")}));
            }
        }
        for (id, name) in [
            ("authority", "Authority"),
            ("cancel", "Cancel"),
            ("settlement", "Settlement"),
            ("close", "Close"),
            ("faults", "Faults"),
            ("race", "Race"),
        ] {
            f.ledger["declarations"]
                .as_array_mut()
                .unwrap()
                .push(json!({"id":id,"scope":"main","kind":"policy","name":name}));
        }
        f.ledger["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":"operator","scope":"main","kind":"actor","name":"Operator"}));
        f
    }
    fn candidate(&self) -> crate::Candidate {
        let bytes = serde_json::to_vec(&self.ledger).unwrap();
        Package::parse(self.source.as_bytes(), &bytes)
            .unwrap()
            .lower(&self.supplied())
            .unwrap()
    }
}

#[test]
fn real_current_source_records_the_actual_failed_template_relation() {
    let f = Fixture::full();
    let bytes = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
    LAST.with_borrow_mut(|last| *last = (0, None));
    let e = p.lower(&f.supplied()).unwrap_err();
    assert_eq!(e.code, "child-template");
    assert_eq!(e.span, 0..0);
    assert_eq!(
        LAST.with_borrow(|last| last.clone()),
        (
            1,
            Some((
                "authorize_call".into(),
                "invoke".into(),
                "accept_payment".into(),
                "authorize_service".into(),
                Some("order".into())
            ))
        )
    );
}

#[test]
fn direct_children_and_same_parent_multiple_creators_are_not_cross_parent_calls() {
    let f = Fixture::bounded();
    let c = f.candidate();
    c.check_reconciliation_availability(&f.supplied()).unwrap();
    let mut body = c.document()["body"].clone();
    body["nodes"]["again"] = body["nodes"]["use_a"].clone();
    body["flows"]["use_a_next"]["target"] = "again".into();
    body["flows"]["again_next"] = json!({"source":"again","outcome":"complete","target":"use_b"});
    let e = DefinitionEnvelope::from_parts_versioned("0.4.0", body, json!({})).unwrap();
    choreoform_semantic_check::check_reconciliation_availability(&e, &f.supplied()).unwrap();
}

#[test]
fn sibling_body_is_visible_by_name_but_not_a_child_of_its_caller() {
    let mut f = Fixture::bounded();
    f.candidate().check_static(&f.supplied()).unwrap();
    f.source = f
        .source
        .replace("Step \"Use C\". Run \"C\"", "Step \"Use C\". Run \"A\"");
    let bytes = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
    LAST.with_borrow_mut(|last| *last = (0, None));
    let e = p.lower(&f.supplied()).unwrap_err();
    assert_eq!(e.code, "child-template");
    assert_eq!(
        LAST.with_borrow(|last| last.clone()),
        (
            1,
            Some((
                "use_c".into(),
                "invoke".into(),
                "b".into(),
                "a".into(),
                Some("main".into())
            ))
        )
    );
}

#[test]
fn raw_sdk_also_requires_the_exact_declared_parent_under_both_current_cores() {
    let f = Fixture::bounded();
    let c = f.candidate();
    for core in [CORE_V11, CORE_V12] {
        let mut body = c.document()["body"].clone();
        body["dialects"]["core"] = json!({"id":core.0,"revision":core.1});
        let e = DefinitionEnvelope::from_parts_versioned("0.4.0", body.clone(), json!({})).unwrap();
        choreoform_semantic_check::check(&e, &f.supplied()).unwrap();
        body["nodes"]["use_c"]["body"] = "a".into();
        let e = DefinitionEnvelope::from_parts_versioned("0.4.0", body, json!({})).unwrap();
        assert_eq!(
            choreoform_semantic_check::check(&e, &f.supplied())
                .unwrap_err()
                .code,
            "child-scope"
        );
    }
}

#[test]
fn malformed_body_refs_have_their_earlier_structural_refusal() {
    let f = Fixture::bounded();
    let c = f.candidate();
    for target in ["missing", "use_a"] {
        let mut body = c.document()["body"].clone();
        body["nodes"]["use_c"]["body"] = target.into();
        let e = DefinitionEnvelope::from_parts_versioned("0.4.0", body, json!({})).unwrap();
        assert_eq!(
            choreoform_semantic_check::check(&e, &f.supplied())
                .unwrap_err()
                .code,
            "structure"
        );
    }
}
