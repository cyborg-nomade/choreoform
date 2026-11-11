// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Test-only exact refusal capture; no runtime diagnostics or benchmark mutation.
use crate::Package;
use choreoform_ir_probe_core::Resource;
use choreoform_semantic_check::{
    CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, CORE_V6, CORE_V7, CORE_V8, CORE_V9, CORE_V10,
    CORE_V11, CORE_V12, SEMANTICS,
};
use serde_json::{Value, json};
use std::cell::RefCell;

type Request = (String, String, String);
thread_local! { static LAST:RefCell<(usize,Option<Request>)>=const {RefCell::new((0,None))}; }
pub(super) fn record(owner: &str, role: &str, name: &str) {
    LAST.with_borrow_mut(|last| {
        last.0 += 1;
        last.1 = Some((owner.into(), role.into(), name.into()));
    });
}
struct Fixture {
    source: String,
    ledger: Value,
    artifacts: Vec<(String, String, Vec<u8>)>,
}
impl Fixture {
    fn new() -> Self {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let base = root.join("docs/benchmarks/rp03/history/source-0.14");
        let source = std::fs::read_to_string(base.join("source.choreo"))
            .unwrap()
            .replacen("authoring \"0.14.0\"", "authoring \"0.16.0\"", 1);
        let mut ledger: Value =
            serde_json::from_slice(&std::fs::read(base.join("bindings.json")).unwrap()).unwrap();
        ledger["profile"] = "0.16.0".into();
        ledger["version"] = "0.15.0".into();
        ledger["contracts"]["core"] =
            json!({"contract":CORE_V12.0,"digest":CORE_V12.1,"dialect":"core"});
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
    fn bounded(interface: &str, finish: &str, symbol: &str) -> Self {
        let mut f = Self::new();
        f.artifacts
            .retain(|(id, _, _)| id == SEMANTICS.0 || id.contains("core-profile:"));
        f.source = format!(
            "Choreoform authoring \"0.16.0\".\nProcess \"Audit\".\nStart at \"Done\".\nInputs empty. Outputs empty. Outcomes [{interface:?}].\nUse cancellation \"Cancel\", faults \"Faults\", closure \"Close\" and races \"Race\".\nRule for actor \"Authority\". Permit when (true). End policy.\nRequire actor \"Operator\" under \"Authority\".\nRule for cancel \"Cancel\". Authority \"Operator\". For children use \"Settlement\". End policy.\nRule for settlement \"Settlement\". Unfinished work cancel. End policy.\nRule for closure \"Close\". Fully terminal. End policy.\nRule for faults \"Faults\". Handle empty. End policy.\nRule for race \"Race\". Accept in acceptance order. End policy.\nStep \"Done\". Finish as {finish:?}. End step.\nEnd process.\n"
        );
        f.ledger = json!({"format":"choreoform-authoring-bindings","profile":"0.16.0","version":"0.15.0","definition":"bounded_symbol_audit","root":"root","contracts":{"semantics":{"contract":SEMANTICS.0,"digest":SEMANTICS.1},"core":{"contract":CORE_V12.0,"digest":CORE_V12.1,"dialect":"core"}},"declarations":[{"id":"root","kind":"scope","scope":null,"name":"Audit"},{"id":"done","kind":"node","scope":"root","name":"Done"},{"id":"operator","kind":"actor","scope":"root","name":"Operator"}],"symbols":[{"id":"root_outcome","owner":"root","role":"outcome","name":symbol,"wire":"authorization-recorded"}],"generated":[],"resources":[],"retired":[],"annotations":{}});
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
                .push(json!({"id":id,"kind":"policy","scope":"root","name":name}));
        }
        f
    }
}

#[test]
fn actual_whole_order_records_the_requested_symbol_without_runtime_exposure() {
    let f = Fixture::new();
    let bytes = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
    LAST.with_borrow_mut(|last| *last = (0, None));
    let e = p.lower(&f.supplied()).unwrap_err();
    assert_eq!(e.code, "symbol-mismatch");
    assert_eq!(e.span, 0..0);
    let request = LAST.with_borrow(|last| last.clone());
    assert_eq!(
        request,
        (
            1,
            Some((
                "accept_payment".into(),
                "outcome".into(),
                "authorization recorded".into()
            ))
        )
    );
    let declared: Vec<_> = f.ledger["symbols"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["owner"] == "accept_payment" && r["role"] == "outcome")
        .collect();
    assert_eq!(declared.len(), 1);
    assert_eq!(declared[0]["name"], "authorization-recorded");
    assert_eq!(declared[0]["wire"], "authorization-recorded");
    assert!(
        f.source
            .contains("Step \"Finish authorization\". Finish as \"authorization recorded\".")
    );
}

#[test]
fn whole_order_metadata_only_rename_moves_refusal_to_the_declared_interface() {
    let mut f = Fixture::new();
    f.ledger["symbols"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["owner"] == "accept_payment" && r["role"] == "outcome")
        .unwrap()["name"] = "authorization recorded".into();
    let bytes = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
    LAST.with_borrow_mut(|last| *last = (0, None));
    let e = p.lower(&f.supplied()).unwrap_err();
    assert_eq!(e.code, "symbol-mismatch");
    assert_eq!(
        LAST.with_borrow(|last| last.clone()),
        (
            1,
            Some((
                "accept_payment".into(),
                "outcome".into(),
                "authorization-recorded".into()
            ))
        )
    );
}

#[test]
fn exact_caption_mismatch_and_metadata_only_rename_fail_at_distinct_requests() {
    for (interface, finish, symbol, missing) in [
        (
            "authorization-recorded",
            "authorization recorded",
            "authorization-recorded",
            "authorization recorded",
        ),
        (
            "authorization-recorded",
            "authorization recorded",
            "authorization recorded",
            "authorization-recorded",
        ),
    ] {
        let f = Fixture::bounded(interface, finish, symbol);
        let bytes = serde_json::to_vec(&f.ledger).unwrap();
        let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
        LAST.with_borrow_mut(|last| *last = (0, None));
        let e = p.lower(&f.supplied()).unwrap_err();
        assert_eq!(e.code, "symbol-mismatch");
        assert_eq!(
            LAST.with_borrow(|last| last.clone()),
            (1, Some(("root".into(), "outcome".into(), missing.into())))
        );
    }
}

#[test]
fn exact_matching_captions_can_keep_the_same_stable_wire_identity() {
    for caption in ["authorization-recorded", "authorization recorded"] {
        let f = Fixture::bounded(caption, caption, caption);
        let bytes = serde_json::to_vec(&f.ledger).unwrap();
        let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
        let c = p.lower(&f.supplied()).unwrap();
        assert_eq!(
            c.document()["body"]["nodes"]["done"]["outcome"],
            "authorization-recorded"
        );
        c.check_reconciliation_availability(&f.supplied()).unwrap();
    }
}

#[test]
fn alias_wire_or_wrong_role_is_not_a_companion_only_repair() {
    let mut f = Fixture::bounded(
        "authorization-recorded",
        "authorization recorded",
        "authorization-recorded",
    );
    let mut alias = f.ledger["symbols"][0].clone();
    alias["id"] = "alias".into();
    alias["name"] = "authorization recorded".into();
    f.ledger["symbols"].as_array_mut().unwrap().push(alias);
    assert_eq!(
        crate::Ledger::decode(&serde_json::to_vec(&f.ledger).unwrap())
            .unwrap_err()
            .code,
        "symbol-duplicate"
    );
    f.ledger["symbols"][1]["wire"] = "alternate_wire".into();
    crate::Ledger::decode(&serde_json::to_vec(&f.ledger).unwrap()).unwrap();
    let bytes = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &bytes).unwrap();
    assert_eq!(
        p.lower(&f.supplied()).unwrap_err().code,
        "source-ledger-inventory"
    );
    f.ledger["symbols"].as_array_mut().unwrap().pop();
    f.ledger["symbols"][0]["role"] = "branch".into();
    assert_eq!(
        crate::Ledger::decode(&serde_json::to_vec(&f.ledger).unwrap())
            .unwrap_err()
            .code,
        "symbol-owner"
    );
}
