// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Actual source/resolver/semantic checks, not an authoring editor or admission.
use choreoform_authoring_frontend::{Candidate, Ledger, Package, parse};
use choreoform_ir_probe_core::Resource;
use choreoform_semantic_check::{
    CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, CORE_V6, CORE_V7, CORE_V8, CORE_V9, CORE_V10,
    CORE_V11, CORE_V12, SEMANTICS,
};
use serde_json::{Value, json};
#[path = "support/named_protection.rs"]
mod support;

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn contracts() -> Vec<(String, String, Vec<u8>)> {
    [
        SEMANTICS, CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, CORE_V6, CORE_V7, CORE_V8, CORE_V9,
        CORE_V10, CORE_V11, CORE_V12,
    ]
    .into_iter()
    .map(|(id, rev)| {
        (
            id.into(),
            rev.into(),
            std::fs::read(root().join(format!("docs/ir/contracts/{}.txt", rev.replace(':', "-"))))
                .unwrap(),
        )
    })
    .collect()
}
fn supplied(items: &[(String, String, Vec<u8>)]) -> Vec<Resource<'_>> {
    items
        .iter()
        .map(|(id, revision, bytes)| Resource {
            id,
            revision,
            bytes,
        })
        .collect()
}
fn promote(l: &mut Value) {
    l["profile"] = "0.17.0".into();
    l["version"] = "0.16.0".into();
    l["contracts"]["core"]["contract"] = CORE_V12.0.into();
    l["contracts"]["core"]["digest"] = CORE_V12.1.into();
}
fn lower(s: &str, l: &Value) -> Result<Candidate, choreoform_authoring_frontend::Diagnostic> {
    let bytes = serde_json::to_vec(l).unwrap();
    let p = Package::parse(s.as_bytes(), &bytes)?;
    p.lower(&supplied(&contracts()))
}
fn small() -> (String, Value) {
    let mut l: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/authoring/examples/terminal.bindings.json"
    ))
    .unwrap();
    promote(&mut l);
    let s=include_str!("../../../docs/authoring/examples/terminal.choreo").replace("authoring \"0.2.0\"","authoring \"0.17.0\"").replace("End process.","Rule for protection \"Access\". Permit access when (true). End policy.\nKeep \"Value\" as whole number under \"Access\". Sensitivity \"test\". Purposes [\"Use\"]. Participants [\"Reviewer\"]. Capabilities empty. Invalidate empty. Initially unavailable. End data.\nEnd process.");
    for (id, kind, name) in [("access", "policy", "Access"), ("value", "data", "Value")] {
        l["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"kind":kind,"scope":"scope_terminal","name":name}));
    }
    l["symbols"].as_array_mut().unwrap().push(
        json!({"id":"value_purpose","owner":"value","role":"purpose","name":"Use","wire":"use"}),
    );
    (s, l)
}
fn named() -> (String, Value) {
    let (s, mut l) = small();
    let s = support::factor(&s, &mut l);
    (s, l)
}
fn refusal(s: &str, l: &Value, code: &str) {
    let e = lower(s, l).unwrap_err();
    assert_eq!(e.code, code, "{e:?}");
    assert!(e.span.end <= s.len());
}

#[test]
fn named_and_inline_have_exact_ir_and_format_integrity_preserves_unknown_annotations() {
    let (s, l) = small();
    let expected = lower(&s, &l).unwrap();
    let mut n = l.clone();
    let named = support::factor(&s, &mut n);
    n["annotations"]["future:unrecognized"] = json!({"value":[1,"opaque"]});
    let c = lower(&named, &n).unwrap();
    assert_eq!(c.document()["body"], expected.document()["body"]);
    assert_eq!(c.document()["revision"], expected.document()["revision"]);
    assert_eq!(c.document()["annotations"], n["annotations"]);
    let b = serde_json::to_vec(&n).unwrap();
    let p = Package::parse(named.as_bytes(), &b).unwrap();
    let (f, b) = p.formatted().unwrap();
    let q = Package::parse(f.as_bytes(), b.as_bytes()).unwrap();
    assert_eq!(
        q.lower(&supplied(&contracts())).unwrap().document(),
        c.document()
    );
    assert_ne!(q.integrity().source, p.integrity().source);
    assert!(
        !c.document()["body"]["policies"]
            .as_object()
            .unwrap()
            .contains_key("shared_protection")
    );
    c.check_static(&supplied(&contracts())).unwrap();
    c.check_protection(&supplied(&contracts())).unwrap();
}
#[test]
fn rename_reorder_and_copy_keep_explicit_source_identity_without_wire_records() {
    let (s, l) = named();
    let original = lower(&s, &l).unwrap();
    let mut n = l.clone();
    let s = s.replace("Shared data protection", "保護");
    n["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "shared_protection")
        .unwrap()["name"] = "保護".into();
    n["declarations"].as_array_mut().unwrap().reverse();
    n["symbols"].as_array_mut().unwrap().reverse();
    assert_eq!(lower(&s, &n).unwrap().document(), original.document());
    refusal(&s, &l, "binding-mismatch");
    let mut duplicate = l.clone();
    let row = duplicate["declarations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == "shared_protection")
        .unwrap()
        .clone();
    duplicate["declarations"].as_array_mut().unwrap().push(row);
    assert_eq!(
        Ledger::decode(&serde_json::to_vec(&duplicate).unwrap())
            .unwrap_err()
            .code,
        "binding-duplicate"
    );
    let mut copied = l.clone();
    copied["declarations"].as_array_mut().unwrap().push(json!({"id":"fresh_protection","kind":"data-protection","scope":"scope_terminal","name":"Other protection"}));
    copied["symbols"].as_array_mut().unwrap().push(json!({"id":"fresh_purpose","owner":"fresh_protection","role":"purpose","name":"Use","wire":"use"}));
    let start = s.find("Protect data as").unwrap();
    let end = s[start..].find("End protection.").unwrap() + start + "End protection.".len();
    let group = s[start..end].replace("保護", "Other protection");
    let copied_source = s.replace("End process.", &format!("{group}\nEnd process.")); // use the renamed package consistently
    copied["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "shared_protection")
        .unwrap()["name"] = "保護".into();
    assert_eq!(
        lower(&copied_source, &copied).unwrap().document(),
        original.document()
    );
}
#[test]
fn definitions_validate_even_unused_and_wrong_kinds_cannot_fall_through() {
    let (s, l) = named();
    for (from, to, code) in [
        ("Under \"Access\"", "Under \"Missing\"", "reference-missing"),
        ("Under \"Access\"", "Under \"Reviewer\"", "reference-kind"),
        (
            "Under \"Access\"",
            "Under \"Stop\"",
            "protection-policy-kind",
        ),
        (
            "Participants [\"Reviewer\"]",
            "Participants [\"Access\"]",
            "reference-kind",
        ),
        (
            "Capabilities empty",
            "Capabilities [\"Reviewer\"]",
            "reference-kind",
        ),
        ("Purposes [\"Use\"]", "Purposes empty", "data-purposes"),
        (
            "Participants [\"Reviewer\"]",
            "Participants [\"Reviewer\", \"Reviewer\" in \"Terminal example\"]",
            "duplicate-set-member",
        ),
    ] {
        refusal(&s.replace(from, to), &l, code);
    }
    let original = small().0;
    let start = s.find("Protect data as").unwrap();
    let end = s[start..].find("End protection.").unwrap() + start + "End protection.".len();
    let unused = original.replace("End process.", &format!("{}\nEnd process.", &s[start..end]));
    assert!(lower(&unused, &l).is_ok());
    refusal(
        &unused.replace("Under \"Access\"", "Under \"Stop\""),
        &l,
        "protection-policy-kind",
    );
}
#[test]
fn missing_surplus_purpose_sets_duplicate_symbols_and_retired_ids_refuse() {
    let (s, l) = named();
    for owner in ["value", "shared_protection"] {
        let mut n = l.clone();
        n["symbols"]
            .as_array_mut()
            .unwrap()
            .retain(|r| r["owner"] != owner);
        refusal(
            &s,
            &n,
            if owner == "value" {
                "protection-purpose-bindings"
            } else {
                "symbol-mismatch"
            },
        );
    }
    let mut n = l.clone();
    n["symbols"].as_array_mut().unwrap().push(json!({"id":"extra_purpose","owner":"value","role":"purpose","name":"Extra","wire":"extra"}));
    refusal(&s, &n, "protection-purpose-bindings");
    let mut n = l.clone();
    n["retired"] = json!(["shared_protection"]);
    assert_eq!(
        Ledger::decode(&serde_json::to_vec(&n).unwrap())
            .unwrap_err()
            .code,
        "retired-identity"
    );
    let mut n = l.clone();
    n["declarations"].as_array_mut().unwrap().push(json!({"id":"unused_protection","kind":"data-protection","scope":"scope_terminal","name":"Extra"}));
    refusal(&s, &n, "source-ledger-inventory");
}
#[test]
fn mixed_inline_named_and_local_purpose_literal_captions_preserve_meaning() {
    let (s, l) = named();
    let mut n = l.clone();
    n["symbols"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "value_purpose")
        .unwrap()["name"] = "Local purpose".into();
    let source=s.replace("End process.","Let \"Purpose\" mean (purpose \"Local purpose\" of \"Value\") as text with parameters empty.\nKeep \"Other\" as text under \"Access\". Sensitivity \"test\". Purposes [\"Use\"]. Participants [\"Reviewer\"]. Capabilities empty. Invalidate empty. Initially unavailable. End data.\nEnd process.");
    for (id, kind, name) in [
        ("purpose_expr", "expression", "Purpose"),
        ("other", "data", "Other"),
    ] {
        n["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"kind":kind,"scope":"scope_terminal","name":name}));
    }
    n["symbols"].as_array_mut().unwrap().push(
        json!({"id":"other_purpose","owner":"other","role":"purpose","name":"Use","wire":"use"}),
    );
    let c = lower(&source, &n).unwrap();
    assert_eq!(
        c.document()["body"]["data"]["other"]["protection"],
        c.document()["body"]["data"]["value"]["protection"]
    );
    assert_eq!(
        c.document()["body"]["expressions"]["purpose_expr"]["body"]["value"],
        "use"
    );
}
#[test]
fn old_syntax_pair_and_kind_stay_closed_and_future_remains_unsupported() {
    let (s, l) = named();
    for (profile, version, core) in [
        ("0.2.0", "0.1.0", CORE),
        ("0.3.0", "0.2.0", CORE_V2),
        ("0.4.0", "0.3.0", CORE_V2),
        ("0.5.0", "0.4.0", CORE_V2),
        ("0.6.0", "0.5.0", CORE_V3),
        ("0.7.0", "0.6.0", CORE_V4),
        ("0.8.0", "0.7.0", CORE_V5),
        ("0.9.0", "0.8.0", CORE_V6),
        ("0.10.0", "0.9.0", CORE_V7),
        ("0.11.0", "0.10.0", CORE_V8),
        ("0.12.0", "0.11.0", CORE_V9),
        ("0.13.0", "0.12.0", CORE_V10),
        ("0.14.0", "0.13.0", CORE_V11),
        ("0.15.0", "0.14.0", CORE_V11),
        ("0.16.0", "0.15.0", CORE_V12),
    ] {
        let old = s.replace("authoring \"0.17.0\"", &format!("authoring {profile:?}"));
        assert!(parse(old.as_bytes()).is_err(), "{profile}");
        let mut n = l.clone();
        n["profile"] = profile.into();
        n["version"] = version.into();
        n["contracts"]["core"]["contract"] = core.0.into();
        n["contracts"]["core"]["digest"] = core.1.into();
        assert_eq!(
            Ledger::decode(&serde_json::to_vec(&n).unwrap())
                .unwrap_err()
                .code,
            "binding-kind",
            "{profile}"
        );
    }
    assert_eq!(
        parse(s.replace("0.17.0", "0.19.0").as_bytes())
            .unwrap_err()
            .code,
        "profile"
    );
}
#[test]
fn actual_whole_process_variants_reparse_with_exact_expanded_body_and_refusal() {
    for name in ["rp01", "rp03"] {
        let base = root().join(if name == "rp03" {
            "docs/benchmarks/rp03/history/source-0.16-before-factoring".to_owned()
        } else {
            "docs/benchmarks/rp01/history/source-0.14-before-factoring".to_owned()
        });
        let original = std::fs::read_to_string(base.join("source.choreo")).unwrap();
        let mut l: Value =
            serde_json::from_slice(&std::fs::read(base.join("bindings.json")).unwrap()).unwrap();
        let old_profile = l["profile"].as_str().unwrap().to_owned();
        let source = original.replacen(
            &format!("authoring \"{old_profile}\""),
            "authoring \"0.17.0\"",
            1,
        );
        promote(&mut l);
        let mut resources = contracts();
        let m: Value =
            serde_json::from_slice(&std::fs::read(base.join("manifest.json")).unwrap()).unwrap();
        for r in m["resources"].as_array().unwrap() {
            let id = r["id"].as_str().unwrap();
            let rev = r["revision"].as_str().unwrap();
            if !resources.iter().any(|(i, v, _)| i == id && v == rev) {
                resources.push((
                    id.into(),
                    rev.into(),
                    std::fs::read(
                        r["path"]
                            .as_str()
                            .unwrap()
                            .strip_prefix(&format!("docs/benchmarks/{name}/"))
                            .map_or_else(
                                || root().join(r["path"].as_str().unwrap()),
                                |relative| base.join(relative),
                            ),
                    )
                    .unwrap(),
                ));
            }
        }
        let bytes = serde_json::to_vec(&l).unwrap();
        let expanded = Package::parse(source.as_bytes(), &bytes)
            .unwrap()
            .lower(&supplied(&resources))
            .unwrap();
        let named = support::factor(&source, &mut l);
        assert_eq!(named.len(), if name == "rp03" { 448421 } else { 56059 });
        let bytes = serde_json::to_vec(&l).unwrap();
        let p = Package::parse(named.as_bytes(), &bytes).unwrap();
        let c = p.lower(&supplied(&resources)).unwrap();
        assert_eq!(c.document(), expanded.document());
        let (f, b) = p.formatted().unwrap();
        assert_eq!(
            Package::parse(f.as_bytes(), b.as_bytes())
                .unwrap()
                .lower(&supplied(&resources))
                .unwrap()
                .document(),
            c.document()
        );
        let current: Value =
            serde_json::from_slice(&std::fs::read(base.join("candidate.ir.json")).unwrap())
                .unwrap();
        let mut body = current["body"].clone();
        body["dialects"] = c.document()["body"]["dialects"].clone();
        assert_eq!(c.document()["body"], body);
        c.check_static(&supplied(&resources)).unwrap();
        c.check_control(&supplied(&resources)).unwrap();
        c.check_protection(&supplied(&resources)).unwrap();
        c.check_invalidation(&supplied(&resources)).unwrap();
        if name == "rp03" {
            assert!(named.len() * 100 <= original.len() * 85);
            assert_eq!(c.document(), &current);
            let e = c.check_concurrency(&supplied(&resources)).unwrap_err();
            assert_eq!(e.code, "parallel-effects-unproved");
            assert_eq!(e.path, "/body/nodes/start_acceptance");
        }
        println!(
            "{name}: {} -> {} bytes; revision {}",
            original.len(),
            named.len(),
            c.document()["revision"]
        );
    }
}

#[test]
fn explicit_ancestor_use_works_but_sibling_group_and_member_exposure_refuse() {
    let (s, mut l) = named();
    let child = "Scope \"Child\" within \"Terminal example\". Start at \"Child done\". Inputs empty. Outputs empty. Outcomes [\"complete\"]. Use cancellation \"Stop\", faults \"Fault handling\", closure \"Close\" and races \"Commit order\". Step \"Child done\". Finish as \"complete\". End step. End scope.";
    let source = s.replace("End process.", &format!("{child}\nEnd process."));
    for (id, kind, scope, name) in [
        ("child", "scope", "scope_terminal", "Child"),
        ("child_done", "node", "child", "Child done"),
    ] {
        l["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"kind":kind,"scope":scope,"name":name}));
    }
    l["symbols"].as_array_mut().unwrap().push(json!({"id":"child_complete","owner":"child","role":"outcome","name":"complete","wire":"complete"}));
    let start = source.find("Keep \"Value\"").unwrap();
    let end = source[start..].find("End data.").unwrap() + start + "End data.".len();
    let data = source[start..end].to_owned();
    let source = source.replacen(&data, "", 1).replace(
        "Step \"Child done\"",
        &(data.replace(
            "protected as \"Shared data protection\"",
            "protected as \"Shared data protection\" in \"Terminal example\"",
        ) + "\nStep \"Child done\""),
    );
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "value")
        .unwrap()["scope"] = "child".into();
    assert!(lower(&source, &l).is_ok());
    let mut private = l.clone();
    private["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "shared_protection")
        .unwrap()["scope"] = "child".into();
    let start = source.find("Protect data as").unwrap();
    let end = source[start..].find("End protection.").unwrap() + start + "End protection.".len();
    let group = source[start..end].to_owned();
    let relocated = source
        .replacen(&group, "", 1)
        .replace("Step \"Child done\"", &(group + "\nStep \"Child done\""));
    let root_use = relocated.replacen(
        "protected as \"Shared data protection\" in \"Terminal example\"",
        "protected as \"Shared data protection\"",
        1,
    );
    assert!(lower(&root_use, &private).is_ok());
    // A root definition cannot name a child's private actor, even unused.
    let mut member = l.clone();
    member["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"private_actor","kind":"actor","scope":"child","name":"Private actor"}));
    let source = source
        .replace(
            "Step \"Child done\"",
            "Require actor \"Private actor\" under \"Reviewer requirement\". Step \"Child done\"",
        )
        .replace(
            "Participants [\"Reviewer\"]",
            "Participants [\"Private actor\" in \"Child\" in \"Terminal example\"]",
        );
    refusal(&source, &member, "reference-missing");
    // Move the root data to the root again: a private child group stays hidden.
    let mut root_data = private.clone();
    root_data["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "value")
        .unwrap()["scope"] = "scope_terminal".into();
    let moved = root_use
        .replacen(&data, "", 1)
        .replace("End process.", &format!("{data}\nEnd process."));
    refusal(&moved, &root_data, "reference-missing");
}

#[test]
fn short_sources_cannot_expand_past_cumulative_bytes_or_dependency_work() {
    for wide in [false, true] {
        let (s, mut l) = named();
        let mut additions = String::new();
        let count = if wide { 350 } else { 300 };
        for i in 0..count {
            additions.push_str(&format!("Keep \"Value {i}\" as text, protected as \"Shared data protection\". Invalidate empty. Initially unavailable. End data.\n"));
            l["declarations"].as_array_mut().unwrap().push(json!({"id":format!("v{i}"),"kind":"data","scope":"scope_terminal","name":format!("Value {i}")}));
            l["symbols"].as_array_mut().unwrap().push(json!({"id":format!("vp{i}"),"owner":format!("v{i}"),"role":"purpose","name":"Use","wire":"use"}));
        }
        let mut source = s.replace("End process.", &format!("{additions}End process."));
        if wide {
            let mut actors = Vec::new();
            let mut definitions = String::new();
            for i in 0..200 {
                actors.push(format!("\"Actor {i}\""));
                definitions.push_str(&format!(
                    "Require actor \"Actor {i}\" under \"Reviewer requirement\".\n"
                ));
                l["declarations"].as_array_mut().unwrap().push(json!({"id":format!("a{i}"),"kind":"actor","scope":"scope_terminal","name":format!("Actor {i}")}));
            }
            source = source
                .replace(
                    "Participants [\"Reviewer\"]",
                    &format!("Participants [{}]", actors.join(", ")),
                )
                .replace("End process.", &format!("{definitions}End process."));
        } else {
            source = source.replace(
                "Sensitivity \"test\"",
                &format!("Sensitivity \"{}\"", "x".repeat(4000)),
            );
        }
        assert!(source.len() < 100_000);
        assert!(serde_json::to_vec(&l).unwrap().len() < 1024 * 1024);
        refusal(&source, &l, if wide { "work-limit" } else { "size" });
    }
}

#[test]
fn inline_compound_defects_preserve_legacy_diagnostic_order_and_spans() {
    for (profile, version, core) in [
        ("0.2.0", "0.1.0", CORE),
        ("0.16.0", "0.15.0", CORE_V12),
        ("0.17.0", "0.16.0", CORE_V12),
    ] {
        let (s, mut l) = small();
        let s = s
            .replace("authoring \"0.17.0\"", &format!("authoring {profile:?}"))
            .replace("under \"Access\"", "under \"Missing policy\"")
            .replace("Invalidate empty", "Invalidate [\"Missing boundary\"]");
        l["profile"] = profile.into();
        l["version"] = version.into();
        l["contracts"]["core"]["contract"] = core.0.into();
        l["contracts"]["core"]["digest"] = core.1.into();
        let e = lower(&s, &l).unwrap_err();
        assert_eq!(e.code, "reference-missing");
        assert_eq!(&s[e.span], "\"Missing boundary\"");
        let s = s.replace("Purposes [\"Use\"]", "Purposes empty");
        let e = lower(&s, &l).unwrap_err();
        assert_eq!(e.code, "data-purposes");
        assert!(s[e.span].starts_with("Keep \"Value\""));
    }
}
