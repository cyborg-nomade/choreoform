// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_authoring_frontend::{Candidate, Package, parse_fragment};
use choreoform_ir_probe_core::DefinitionEnvelope;
use choreoform_semantic_check::{CORE_V5, CORE_V6, FAULT_SOURCE_PROFILE};

pub(super) struct Fixture {
    pub(super) source: String,
    pub(super) ledger: Value,
    pub(super) artifacts: Vec<(String, String, Vec<u8>)>,
}
fn recognise(source: &mut String, work: &str, name: &str) {
    let start = source.find(&format!("Rule for work \"{work}\".")).unwrap();
    let end = start + source[start..].find("End policy.").unwrap();
    let block = source[start..end].replacen(
        "Authority \"Payment operator\".",
        &format!("Authority \"Payment operator\". Recognise faults from \"{name}\"."),
        1,
    );
    source.replace_range(start..end, &block);
}
impl Fixture {
    pub(super) fn new() -> Self {
        let m: Value = serde_json::from_slice(MANIFEST).unwrap();
        let mut f = Self {
            source: std::str::from_utf8(SOURCE)
                .unwrap()
                .replace("authoring \"0.7.0\"", "authoring \"0.9.0\"")
                .replace(", fault source \"Payment faults\"", ""),
            ledger: serde_json::from_slice(BINDINGS).unwrap(),
            artifacts: super::profile::source_resources(&m)
                .iter()
                .map(|r| (r.id.into(), r.revision.into(), r.bytes.to_vec()))
                .collect(),
        };
        f.ledger["profile"] = "0.9.0".into();
        f.ledger["version"] = "0.8.0".into();
        f.ledger["contracts"]["core"] =
            json!({"contract":CORE_V6.0,"digest":CORE_V6.1,"dialect":"core"});
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        for p in [CORE_V5, CORE_V6] {
            f.artifacts.push((
                p.0.into(),
                p.1.into(),
                std::fs::read(
                    root.join(format!("docs/ir/contracts/{}.txt", p.1.replace(':', "-"))),
                )
                .unwrap(),
            ));
        }
        let provider = f.ledger["resources"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["contract"] == "urn:choreoform:benchmarks:rp01:notice:0.1.0")
            .unwrap();
        let source = json!({"kind":"provider-fault-source","version":"0.1.0","id":"urn:test:authoring-notice-faults","profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":{"id":provider["contract"],"revision":provider["digest"]},"emissions":{"notice uncertainty":{"observations":{"unknown":true},"evidence":"unresolved"}}});
        let raw = serde_json::to_vec(&source).unwrap();
        let revision = digest(&raw);
        f.artifacts
            .push((source["id"].as_str().unwrap().into(), revision.clone(), raw));
        f.ledger["declarations"].as_array_mut().unwrap().push(json!({"id":"notice_faults_binding","kind":"resource","scope":"expense","name":"Notice faults"}));
        f.ledger["resources"].as_array_mut().unwrap().push(json!({"binding":"notice_faults_binding","kind":"provider-fault-source","contract":source["id"],"digest":revision}));
        recognise(&mut f.source, "Payment work", "Payment faults");
        recognise(&mut f.source, "Notice work", "Notice faults");
        f
    }
    pub(super) fn resources(&self) -> Vec<Resource<'_>> {
        self.artifacts
            .iter()
            .map(|(id, revision, bytes)| Resource {
                id,
                revision,
                bytes,
            })
            .collect()
    }
    pub(super) fn lower(&self) -> Candidate {
        Package::parse(
            self.source.as_bytes(),
            &serde_json::to_vec(&self.ledger).unwrap(),
        )
        .unwrap()
        .lower(&self.resources())
        .unwrap()
    }
}

#[test]
fn readable_general_sentence_drives_single_and_multiple_attempts_through_all_partial_checks() {
    let f = Fixture::new();
    let c = f.lower();
    let r = f.resources();
    assert_eq!(c.document()["version"], "0.2.0");
    let b = &c.document()["body"];
    assert_eq!(b["dialects"]["core"]["revision"], CORE_V6.1);
    assert!(!b["policies"]["notice_work"]["body"]["faultSource"].is_null());
    assert!(b["policies"]["notice_work"]["body"]["retry"]["faultSource"].is_null());
    assert_eq!(
        b["policies"]["payment_work"]["body"]["faultSource"],
        b["policies"]["payment_work"]["body"]["retry"]["faultSource"]
    );
    let report = c.check_static(&r).unwrap();
    assert_eq!(report.fault_source_uses().len(), 2);
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
            .fault_source_uses()
            .len(),
        2
    );
}

#[test]
fn formatting_aliases_and_revision_domain_preserve_semantic_identity_within_profile() {
    let mut f = Fixture::new();
    let c = f.lower();
    let ledger = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &ledger).unwrap();
    let (source, bindings) = p.formatted().unwrap();
    assert_eq!(
        Package::parse(source.as_bytes(), bindings.as_bytes())
            .unwrap()
            .lower(&f.resources())
            .unwrap()
            .document(),
        c.document()
    );
    f.source = f.source.replace("\"Notice faults\"", "\"Notice failures\"");
    for d in f.ledger["declarations"].as_array_mut().unwrap() {
        if d["id"] == "notice_faults_binding" {
            d["name"] = "Notice failures".into();
        }
    }
    let renamed = f.lower();
    assert_eq!(renamed.document(), c.document());
    assert_ne!(renamed.package_digest(), c.package_digest());
    let old: Value = serde_json::from_slice(include_bytes!(
        "../../../../docs/benchmarks/rp01/history/source-0.7/candidate.ir.json"
    ))
    .unwrap();
    assert_ne!(c.document()["revision"], old["revision"]);
}

#[test]
fn old_grammar_and_missing_general_source_do_not_silently_select_new_meaning() {
    let f = Fixture::new();
    for old in [
        "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0", "0.7.0", "0.8.0",
    ] {
        assert_eq!(
            parse(
                f.source
                    .replace("authoring \"0.9.0\"", &format!("authoring \"{old}\""))
                    .as_bytes()
            )
            .unwrap_err()
            .code,
            if matches!(old, "0.2.0" | "0.3.0" | "0.4.0") {
                "tokens"
            } else {
                "syntax"
            }
        );
    }
    let old_clause = f.source.replace(
        "basis \"Payable request\".",
        "basis \"Payable request\", fault source \"Payment faults\".",
    );
    assert_eq!(parse(old_clause.as_bytes()).unwrap_err().code, "syntax");
    let mut f = Fixture::new();
    f.source = f
        .source
        .replace(" Recognise faults from \"Payment faults\".", "");
    let l = serde_json::to_vec(&f.ledger).unwrap();
    assert_eq!(
        Package::parse(f.source.as_bytes(), &l)
            .unwrap()
            .lower(&f.resources())
            .unwrap_err()
            .code,
        "fault-source-required"
    );
    let mut f = Fixture::new();
    f.source = f
        .source
        .replace(" Recognise faults from \"Notice faults\".", "");
    assert!(
        f.lower().document()["body"]["policies"]["notice_work"]["body"]["faultSource"].is_null()
    );
}

#[test]
fn known_new_header_retains_closed_program_and_legacy_fragment_quotas() {
    let s = format!("Choreoform authoring \"0.9.0\". {}", "bad ".repeat(16380));
    assert_eq!(parse(s.as_bytes()).unwrap_err().code, "syntax");
    assert_eq!(
        parse_fragment(s.as_bytes(), "declaration")
            .unwrap_err()
            .code,
        "tokens"
    );
    assert_eq!(parse((s + "bad ").as_bytes()).unwrap_err().code, "tokens");
    let s = format!("Choreoform authoring \"1.0.0\". {}", "bad ".repeat(5000));
    assert_eq!(parse(s.as_bytes()).unwrap_err().code, "tokens");
}

#[test]
fn all_core_bases_and_explicit_resources_are_required_even_without_a_retry_use() {
    let f = Fixture::new();
    let ledger = serde_json::to_vec(&f.ledger).unwrap();
    let p = Package::parse(f.source.as_bytes(), &ledger).unwrap();
    for (id, revision, _) in &f.artifacts {
        let resources: Vec<_> = f
            .resources()
            .into_iter()
            .filter(|r| !(r.id == id && r.revision == revision))
            .collect();
        assert!(p.lower(&resources).is_err(), "missing {id}");
        let mut g = Fixture::new();
        g.artifacts
            .iter_mut()
            .find(|(i, h, _)| i == id && h == revision)
            .unwrap()
            .2
            .push(b' ');
        assert!(p.lower(&g.resources()).is_err(), "corrupt {id}");
    }
}

#[test]
fn human_and_unvisited_wrong_provider_uses_still_refuse_with_source_spans() {
    let mut f = Fixture::new();
    f.source = f.source.replace(
        "Request \"Notify the employee\" using",
        "Ask \"Payment operator\" to decide using",
    );
    let l = serde_json::to_vec(&f.ledger).unwrap();
    let e = Package::parse(f.source.as_bytes(), &l)
        .unwrap()
        .lower(&f.resources())
        .unwrap_err();
    assert_eq!(e.code, "wait-endpoint-use-unproved");
    assert_ne!(e.span, 0..0);
    let mut f = Fixture::new();
    f.source = f.source.replace(
        "Recognise faults from \"Notice faults\".",
        "Recognise faults from \"Payment faults\".",
    );
    let c = f.lower();
    assert_eq!(
        c.check_static(&f.resources()).unwrap_err().code,
        "fault-source-provider"
    );
}

#[test]
fn source_only_unused_policy_reads_are_derived_and_stale_raw_reads_are_rejected() {
    let mut f = Fixture::new();
    let original = f
        .artifacts
        .iter()
        .find(|(id, _, _)| id == "urn:choreoform:benchmarks:rp01:notice:0.1.0")
        .unwrap();
    let mut provider: Value = serde_json::from_slice(&original.2).unwrap();
    provider["id"] = "urn:test:source-only-provider".into();
    provider["resultProtection"]["policy"] = "source_access".into();
    let raw = serde_json::to_vec(&provider).unwrap();
    let revision = digest(&raw);
    f.artifacts.push((
        provider["id"].as_str().unwrap().into(),
        revision.clone(),
        raw,
    ));
    let source = json!({"kind":"provider-fault-source","version":"0.1.0","id":"urn:test:source-only-faults","profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":{"id":provider["id"],"revision":revision},"emissions":{"uncertain":{"observations":{"unknown":true},"evidence":"unresolved"}}});
    let raw = serde_json::to_vec(&source).unwrap();
    let revision = digest(&raw);
    f.artifacts
        .push((source["id"].as_str().unwrap().into(), revision.clone(), raw));
    for row in [
        json!({"id":"source_access","kind":"policy","scope":"expense","name":"Source access"}),
        json!({"id":"source_only_work","kind":"policy","scope":"expense","name":"Source only work"}),
        json!({"id":"source_only_binding","kind":"resource","scope":"expense","name":"Source only faults"}),
    ] {
        f.ledger["declarations"].as_array_mut().unwrap().push(row);
    }
    f.ledger["resources"].as_array_mut().unwrap().push(json!({"binding":"source_only_binding","kind":"provider-fault-source","contract":source["id"],"digest":revision}));
    let extra = "Rule for protection \"Source access\". Permit access when ((the value of \"Review needed\") equals (the value of \"Review needed\")). End policy. Rule for work \"Source only work\". Instructions \"Synthetic\". Authority \"Payment operator\". Recognise faults from \"Source only faults\". Attempts 1. Delays empty. Retry faults empty. Timeout none. Complete when (true). Compensates none. End policy. ";
    f.source = f
        .source
        .replace("End process.", &format!("{extra}End process."));
    let c = f.lower();
    assert_eq!(
        c.document()["body"]["policies"]["source_only_work"]["body"]["reads"],
        json!({"review_needed":true})
    );
    assert_eq!(
        c.check_static(&f.resources())
            .unwrap()
            .fault_source_uses()
            .len(),
        2
    );
    let mut body = c.document()["body"].clone();
    body["policies"]["source_only_work"]["body"]["reads"] = json!({});
    let e = DefinitionEnvelope::from_parts_versioned("0.2.0", body, json!({})).unwrap();
    assert_eq!(
        choreoform_semantic_check::check(&e, &f.resources())
            .unwrap_err()
            .code,
        "policy-reads"
    );
}

#[test]
fn unused_policy_has_no_row_and_unvisited_nodes_have_exact_checked_source_uses() {
    let mut f = Fixture::new();
    let rule = "Rule for work \"Unused work\". Instructions \"Synthetic\". Authority \"Payment operator\". Recognise faults from \"Notice faults\". Attempts 1. Delays empty. Retry faults empty. Timeout none. Complete when (true). Compensates none. End policy. ";
    f.source = f
        .source
        .replace("End process.", &format!("{rule}End process."));
    f.ledger["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"unused_work","kind":"policy","scope":"expense","name":"Unused work"}));
    assert_eq!(
        f.lower()
            .check_static(&f.resources())
            .unwrap()
            .fault_source_uses()
            .len(),
        2
    );
    let step = "Step \"Extra Notice\". Request \"Notify the employee\" using \"Notice input\" under \"Unused work\" and record in nothing. On \"notified\", continue with \"Finish Notice\". End step. ";
    f.source = f.source.replace(
        "Step \"Finish Notice\".",
        &format!("{step}Step \"Finish Notice\"."),
    );
    f.ledger["declarations"].as_array_mut().unwrap().push(
        json!({"id":"extra_notice","kind":"node","scope":"notice_service","name":"Extra Notice"}),
    );
    f.ledger["symbols"].as_array_mut().unwrap().push(json!({"id":"extra_notice_outcome","owner":"extra_notice","role":"outcome","name":"notified","wire":"notified"}));
    f.ledger["generated"].as_array_mut().unwrap().push(json!({"id":"extra_notice_flow","owner":"extra_notice","role":"flow","key":"extra_notice_outcome"}));
    let c = f.lower();
    let report = c.check_static(&f.resources()).unwrap();
    let uses: Vec<_> = report
        .fault_source_uses()
        .iter()
        .filter(|u| u.provider().id.contains(":notice:"))
        .collect();
    assert_eq!(uses.len(), 2);
    assert!(std::ptr::eq(
        uses[0].source_metadata(),
        uses[1].source_metadata()
    ));
    assert!(
        c.check_control(&f.resources())
            .unwrap()
            .uncovered_nodes()
            .contains("extra_notice")
    );
    let human = f.source.replace(
        r#"Step "Extra Notice". Request "Notify the employee" using"#,
        r#"Step "Extra Notice". Ask "Payment operator" to decide using"#,
    );
    let l = serde_json::to_vec(&f.ledger).unwrap();
    let human_candidate = Package::parse(human.as_bytes(), &l)
        .unwrap()
        .lower(&f.resources())
        .unwrap();
    let e = human_candidate.check_static(&f.resources()).unwrap_err();
    assert_eq!(e.code, "fault-source-context-unproved");
    assert!(human_candidate.source_span(&e).is_some());
    recognise(&mut f.source, "Unused work", "Payment faults");
    // Replace the first declaration's source rather than accepting two sentences.
    f.source = f.source.replace(
        "Recognise faults from \"Payment faults\". Recognise faults from \"Notice faults\".",
        "Recognise faults from \"Payment faults\".",
    );
    let c = f.lower();
    let e = c.check_static(&f.resources()).unwrap_err();
    assert_eq!(e.code, "fault-source-provider");
    assert_eq!(e.path, "/body/nodes/extra_notice");
}
