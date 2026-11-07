// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Source-profile integration, not observation acceptance or execution evidence.
use choreoform_authoring_frontend::{Candidate, Ledger, Package, parse};
use choreoform_ir_probe_core::{Resource, digest};
use choreoform_semantic_check::{
    CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, SEMANTICS, TERMINAL_OBSERVATION_PROFILE,
};
use serde_json::{Value, json};

struct Fixture {
    source: String,
    ledger: Value,
    resources: Vec<(String, String, Vec<u8>)>,
}
impl Fixture {
    fn new() -> Self {
        let mut f = Self {
            source: include_str!("capture/ordinary.choreo").into(),
            ledger: serde_json::from_slice(include_bytes!("capture/bindings.json")).unwrap(),
            resources: Vec::new(),
        };
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        for p in [
            SEMANTICS,
            CORE,
            CORE_V2,
            CORE_V3,
            CORE_V4,
            CORE_V5,
            TERMINAL_OBSERVATION_PROFILE,
        ] {
            f.resources.push((
                p.0.into(),
                p.1.into(),
                std::fs::read(
                    root.join(format!("docs/ir/contracts/{}.txt", p.1.replace(':', "-"))),
                )
                .unwrap(),
            ));
        }
        for raw in [
            include_bytes!("capture/provider.json").as_slice(),
            include_bytes!("capture/source.json").as_slice(),
        ] {
            let v: Value = serde_json::from_slice(raw).unwrap();
            f.resources
                .push((v["id"].as_str().unwrap().into(), digest(raw), raw.into()));
        }
        f
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
    fn lower(&self) -> Candidate {
        Package::parse(
            self.source.as_bytes(),
            &serde_json::to_vec(&self.ledger).unwrap(),
        )
        .unwrap()
        .lower(&self.supplied())
        .unwrap()
    }
}

#[test]
fn readable_capture_then_branch_passes_all_partial_stages_and_retains_typed_declarations() {
    let f = Fixture::new();
    let c = f.lower();
    let r = f.supplied();
    assert_eq!(c.document()["version"], "0.2.0");
    assert_eq!(
        c.document()["body"]["dialects"]["core"]["revision"],
        CORE_V5.1
    );
    assert_eq!(
        c.document()["body"]["nodes"]["capture0"]["writes"],
        json!({"captured":true})
    );
    assert!(c.document()["body"]["nodes"]["observe"]["capture"].is_null());
    let report = c.check_static(&r).unwrap();
    let [capture] = report.capture_bindings() else {
        panic!("one declaration")
    };
    assert_eq!(capture.revision(), c.document()["revision"]);
    assert_eq!(capture.site(), "capture0");
    assert_eq!(capture.origin(), "origin");
    assert_eq!(capture.result(), "captured");
    assert!(!capture.remaining().is_empty());
    c.check_control(&r).unwrap();
    c.check_protection(&r).unwrap();
    c.check_invalidation(&r).unwrap();
    c.invalidation_requirements(&r).unwrap();
    c.check_concurrency(&r).unwrap();
    c.check_timing(&r).unwrap();
    c.check_fault_routes(&r).unwrap();
    c.check_exceptional_availability(&r).unwrap();
    c.check_reconciliation_links(&r).unwrap();
    let final_report = c.check_reconciliation_availability(&r).unwrap();
    assert_eq!(
        final_report
            .links()
            .exceptional()
            .faults()
            .timing()
            .concurrency()
            .invalidation()
            .protection()
            .control()
            .static_report()
            .capture_bindings()[0]
            .site(),
        "capture0"
    );
}

#[test]
fn formatting_and_coordinated_readable_alias_changes_preserve_identity() {
    let f = Fixture::new();
    let raw = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &raw).unwrap();
    let c = p.lower(&f.supplied()).unwrap();
    let (s, l) = p.formatted().unwrap();
    assert_eq!(
        Package::parse(s.as_bytes(), l.as_bytes())
            .unwrap()
            .lower(&f.supplied())
            .unwrap()
            .document(),
        c.document()
    );
    let mut alias = Fixture::new();
    alias.source = alias
        .source
        .replace("\"Terminal facts\"", "\"Provider facts\"")
        .replace("\"Captured result\"", "\"Stored terminal result\"");
    for row in alias.ledger["declarations"].as_array_mut().unwrap() {
        if row["id"] == "terminal_binding" {
            row["name"] = "Provider facts".into();
        }
        if row["id"] == "captured" {
            row["name"] = "Stored terminal result".into();
        }
    }
    let renamed = alias.lower();
    assert_eq!(renamed.document(), c.document());
    assert_ne!(renamed.package_digest(), c.package_digest());
}

#[test]
fn old_profiles_and_wrong_companion_pairs_cannot_select_capture_or_new_resource_kind() {
    let f = Fixture::new();
    for old in ["0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0", "0.7.0"] {
        assert_eq!(
            parse(
                f.source
                    .replace("authoring \"0.8.0\"", &format!("authoring \"{old}\""))
                    .as_bytes()
            )
            .unwrap_err()
            .code,
            "syntax"
        );
    }
    let mut l = f.ledger.clone();
    l["version"] = "0.6.0".into();
    assert_eq!(
        Ledger::decode(&serde_json::to_vec(&l).unwrap())
            .unwrap_err()
            .code,
        "ledger-profile"
    );
    l = f.ledger.clone();
    l["contracts"]["core"]["digest"] = CORE_V4.1.into();
    assert_eq!(
        Ledger::decode(&serde_json::to_vec(&l).unwrap())
            .unwrap_err()
            .code,
        "contract-pin"
    );
    l = f.ledger.clone();
    l["version"] = "0.6.0".into();
    l["profile"] = "0.7.0".into();
    l["contracts"]["core"] = json!({"contract":CORE_V4.0,"digest":CORE_V4.1,"dialect":"core"});
    assert_eq!(
        Ledger::decode(&serde_json::to_vec(&l).unwrap())
            .unwrap_err()
            .code,
        "resource-kind"
    );
    let unknown = format!("Choreoform authoring \"0.9.0\". {}", "bad ".repeat(5000));
    assert_eq!(parse(unknown.as_bytes()).unwrap_err().code, "tokens");
}

#[test]
fn all_inherited_source_profile_and_provider_bytes_are_explicit_and_verified() {
    let f = Fixture::new();
    for (id, _, _) in &f.resources {
        let supplied: Vec<_> = f.supplied().into_iter().filter(|r| r.id != id).collect();
        let ledger = serde_json::to_vec(&f.ledger).unwrap();
        let p = Package::parse(f.source.as_bytes(), &ledger).unwrap();
        assert!(p.lower(&supplied).is_err(), "missing {id}");
        let mut corrupt = Fixture::new();
        corrupt
            .resources
            .iter_mut()
            .find(|r| &r.0 == id)
            .unwrap()
            .2
            .push(b' ');
        assert!(p.lower(&corrupt.supplied()).is_err(), "corrupt {id}");
    }
}

#[test]
fn capture_ref_roles_normal_visibility_and_semantic_type_refusals_survive_lowering() {
    let mut f = Fixture::new();
    f.source = f.source.replace("about \"Request\"", "about \"Done\"");
    let c = f.lower();
    let error = c.check_static(&f.supplied()).unwrap_err();
    assert_eq!(error.code, "wait-capture-origin-unproved");
    assert!(c.source_span(&error).is_some());
    f = Fixture::new();
    f.source = f
        .source
        .replace("about \"Request\"", "about \"Missing request\"");
    let l = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &l).unwrap();
    assert_eq!(
        p.lower(&f.supplied()).unwrap_err().code,
        "reference-missing"
    );
    f = Fixture::new();
    f.source = f.source.replace(
        "Observe choice with [\"confirmed\" as whole number]",
        "Observe text",
    );
    let c = f.lower();
    assert_eq!(
        c.check_static(&f.supplied()).unwrap_err().code,
        "wait-capture-type"
    );
    f = Fixture::new();
    f.source = f.source.replace(
        "about \"Request\"",
        "about \"Observe\" in \"Remaining work\"",
    );
    let l = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &l).unwrap();
    assert_eq!(
        p.lower(&f.supplied()).unwrap_err().code,
        "reference-missing"
    );
}

#[test]
fn unvisited_capture_is_disclosed_without_claiming_activation_and_bare_waits_stay_null() {
    let mut f = Fixture::new();
    f.source = f
        .source
        .replacen("Start at \"Request\".", "Start at \"Done\".", 1);
    let c = f.lower();
    let r = f.supplied();
    let control = c.check_control(&r).unwrap();
    assert!(control.uncovered_nodes().contains("capture0"));
    assert_eq!(control.static_report().capture_bindings().len(), 1);
    f.source=f.source.replace(" for observations from \"Terminal facts\" about \"Request\" and record in \"Captured result\"","");
    let c = f.lower();
    assert!(c.document()["body"]["nodes"]["capture0"]["capture"].is_null());
    assert!(
        c.check_static(&f.supplied())
            .unwrap()
            .capture_bindings()
            .is_empty()
    );
}

#[test]
fn source_only_metadata_reads_are_derived_and_stale_raw_declarations_refuse() {
    let mut f = Fixture::new();
    let provider_index = f
        .resources
        .iter()
        .position(|r| r.0 == "urn:test:terminal-provider")
        .unwrap();
    let mut provider: Value = serde_json::from_slice(&f.resources[provider_index].2).unwrap();
    provider["resultProtection"]["participants"]["source_actor"] = true.into();
    let raw = serde_json::to_vec(&provider).unwrap();
    let revision = digest(&raw);
    f.resources[provider_index].1 = revision.clone();
    f.resources[provider_index].2 = raw;
    let source_index = f
        .resources
        .iter()
        .position(|r| r.0 == "urn:test:terminal-source")
        .unwrap();
    let mut source: Value = serde_json::from_slice(&f.resources[source_index].2).unwrap();
    source["provider"]["revision"] = revision.clone().into();
    for r in f.ledger["resources"].as_array_mut().unwrap() {
        if r["binding"] == "provider_binding" {
            r["digest"] = revision.clone().into();
        }
    }
    let raw = serde_json::to_vec(&source).unwrap();
    let revision = digest(&raw);
    f.resources[source_index].1 = revision.clone();
    f.resources[source_index].2 = raw;
    for r in f.ledger["resources"].as_array_mut().unwrap() {
        if r["binding"] == "terminal_binding" {
            r["digest"] = revision.clone().into();
        }
    }
    for d in [
        json!({"id":"source_actor","kind":"actor","scope":"expense","name":"Source witness"}),
        json!({"id":"source_authority","kind":"policy","scope":"expense","name":"Source authority"}),
        json!({"id":"policy_value","kind":"data","scope":"expense","name":"Policy value"}),
    ] {
        f.ledger["declarations"].as_array_mut().unwrap().push(d);
    }
    f.ledger["generated"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"policy_initial","owner":"policy_value","role":"initial","key":null}));
    f.ledger["symbols"].as_array_mut().unwrap().push(json!({"id":"policy_purpose","owner":"policy_value","role":"purpose","name":"Reimbursement","wire":"reimbursement"}));
    let extra = r#"Require actor "Source witness" under "Source authority". Rule for actor "Source authority". Permit when ((false) and ((the value of "Policy value") equals (0))). End policy. Keep "Policy value" as whole number under "Expense access". Sensitivity "expense". Purposes ["Reimbursement"]. Participants ["Employee", "Manager", "Finance reviewer", "Payment operator", "Clock observer"]. Capabilities empty. Invalidate empty. Initially (0). End data. "#;
    f.source = f
        .source
        .replace("Capabilities [\"Capture service\"].", "Capabilities empty.")
        .replace("End process.", &format!("{extra}End process."));
    let c = f.lower();
    c.check_static(&f.supplied()).unwrap();
    assert_eq!(
        c.document()["body"]["nodes"]["capture0"]["reads"],
        json!({"policy_value":true})
    );
    assert_eq!(
        c.document()["body"]["policies"]["wait0"]["body"]["reads"],
        json!({})
    );
    let mut body = c.document()["body"].clone();
    body["nodes"]["capture0"]["reads"] = json!({});
    let e = choreoform_ir_probe_core::DefinitionEnvelope::from_parts_versioned(
        "0.2.0",
        body,
        json!({}),
    )
    .unwrap();
    assert_eq!(
        choreoform_semantic_check::check(&e, &f.supplied())
            .unwrap_err()
            .code,
        "node-reads"
    );
}

#[test]
fn type_equivalent_other_provider_does_not_bind_the_actual_origin() {
    let mut f = Fixture::new();
    let mut provider: Value =
        serde_json::from_slice(include_bytes!("capture/provider.json")).unwrap();
    provider["id"] = "urn:test:other-provider".into();
    let raw = serde_json::to_vec(&provider).unwrap();
    let pin = json!({"id":provider["id"],"revision":digest(&raw)});
    f.resources
        .push((provider["id"].as_str().unwrap().into(), digest(&raw), raw));
    let source_index = f
        .resources
        .iter()
        .position(|r| r.0 == "urn:test:terminal-source")
        .unwrap();
    let mut source: Value = serde_json::from_slice(&f.resources[source_index].2).unwrap();
    source["provider"] = pin;
    let raw = serde_json::to_vec(&source).unwrap();
    let revision = digest(&raw);
    f.resources[source_index].1 = revision.clone();
    f.resources[source_index].2 = raw;
    for r in f.ledger["resources"].as_array_mut().unwrap() {
        if r["binding"] == "terminal_binding" {
            r["digest"] = revision.clone().into();
        }
    }
    let c = f.lower();
    assert_eq!(
        c.check_static(&f.supplied()).unwrap_err().code,
        "wait-capture-provider"
    );
}

#[test]
fn explicit_new_header_keeps_legacy_fragment_and_unknown_header_quotas() {
    let source = format!("Choreoform authoring \"0.8.0\". {}", "bad ".repeat(16380));
    assert_eq!(parse(source.as_bytes()).unwrap_err().code, "syntax");
    assert_eq!(
        choreoform_authoring_frontend::parse_fragment(source.as_bytes(), "declaration")
            .unwrap_err()
            .code,
        "tokens"
    );
    assert_eq!(
        parse((source + "bad ").as_bytes()).unwrap_err().code,
        "tokens"
    );
}
