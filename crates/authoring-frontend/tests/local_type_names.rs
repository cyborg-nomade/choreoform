// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Transparent source types, never new nominal membership or execution admission.
use choreoform_authoring_frontend::{Candidate, Package};
use choreoform_ir_probe_core::Resource;
use choreoform_semantic_check::{
    CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, CORE_V6, CORE_V7, CORE_V8, CORE_V9, CORE_V10,
    CORE_V11, CORE_V12, SEMANTICS,
};
use serde_json::{Value, json};
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
fn supplied(v: &[(String, String, Vec<u8>)]) -> Vec<Resource<'_>> {
    v.iter()
        .map(|(id, revision, bytes)| Resource {
            id,
            revision,
            bytes,
        })
        .collect()
}
fn lower(s: &str, l: &Value) -> Result<Candidate, choreoform_authoring_frontend::Diagnostic> {
    let b = serde_json::to_vec(l).unwrap();
    Package::parse(s.as_bytes(), &b)?.lower(&supplied(&contracts()))
}
fn pair(ty: &str) -> (String, Value) {
    let s=include_str!("../../../docs/authoring/examples/terminal.choreo").replace("authoring \"0.2.0\"","authoring \"0.18.0\"").replace("End process.",&format!("Rule for protection \"Access\". Permit access when (true). End policy.\nKeep \"Value\" as {ty} under \"Access\". Sensitivity \"test\". Purposes [\"Use\"]. Participants [\"Reviewer\"]. Capabilities empty. Invalidate empty. Initially unavailable. End data.\nEnd process."));
    let mut l: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/authoring/examples/terminal.bindings.json"
    ))
    .unwrap();
    l["profile"] = "0.18.0".into();
    l["version"] = "0.17.0".into();
    l["contracts"]["core"]["contract"] = CORE_V12.0.into();
    l["contracts"]["core"]["digest"] = CORE_V12.1.into();
    for (id, kind, name) in [("access", "policy", "Access"), ("value", "data", "Value")] {
        l["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"kind":kind,"scope":"scope_terminal","name":name}));
    }
    l["symbols"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"purpose","owner":"value","role":"purpose","name":"Use","wire":"use"}));
    let s=s.replace("End process.","Let \"Condition\" mean (true) as truth value with parameters empty. Step \"Tag owner\". Choose by [\"complete\" when \"Condition\"]; otherwise none. On \"complete\", continue with \"Done\". End step. End process.");
    for (id, kind, name) in [
        ("condition", "expression", "Condition"),
        ("tag_owner", "node", "Tag owner"),
    ] {
        l["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"kind":kind,"scope":"scope_terminal","name":name}));
    }
    l["symbols"].as_array_mut().unwrap().push(json!({"id":"tag_complete","owner":"tag_owner","role":"outcome","name":"complete","wire":"complete"}));
    l["generated"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"tag_flow","owner":"tag_owner","role":"flow","key":"tag_complete"}));
    (s, l)
}
fn add(s: &mut String, l: &mut Value, id: &str, name: &str, ty: &str) {
    *s = s.replace(
        "End process.",
        &format!("Describe type {name:?} as {ty}.\nEnd process."),
    );
    let owner = l["root"].clone();
    l["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":id,"kind":"type-alias","scope":owner,"name":name}));
}
#[test]
fn scalar_nested_structural_and_outcome_types_expand_exactly_and_reparse() {
    for ty in [
        "truth value",
        "text",
        "whole number",
        "decimal with 2 places",
        "record with empty",
        "record with [\"x\" as list of optional whole number]",
        "choice with [outcome \"complete\" of \"Tag owner\" as text, \"other\" as whole number]",
    ] {
        let (s, l) = pair(ty);
        let original = lower(&s, &l).unwrap();
        let (mut s, mut l) = pair("type \"Description\"");
        add(&mut s, &mut l, "description", "Description", ty);
        let c = lower(&s, &l).unwrap();
        assert_eq!(c.document(), original.document(), "{ty}");
        let b = serde_json::to_vec(&l).unwrap();
        let p = Package::parse(s.as_bytes(), &b).unwrap();
        let (f, b) = p.formatted().unwrap();
        assert_eq!(
            Package::parse(f.as_bytes(), b.as_bytes())
                .unwrap()
                .lower(&supplied(&contracts()))
                .unwrap()
                .document(),
            c.document()
        );
        c.check_static(&supplied(&contracts())).unwrap();
    }
}
#[test]
fn forward_descriptions_resolve_before_use_without_semantic_records() {
    let (mut s, mut l) = pair("type \"Pair\"");
    add(
        &mut s,
        &mut l,
        "pair_type",
        "Pair",
        "record with [\"left\" as type \"Element\", \"right\" as type \"Element\"]",
    );
    add(&mut s, &mut l, "element_type", "Element", "optional text");
    let c = lower(&s, &l).unwrap();
    let (a, b) = pair("record with [\"left\" as optional text, \"right\" as optional text]");
    assert_eq!(c.document(), lower(&a, &b).unwrap().document());
    assert!(
        !c.document()["body"]
            .as_object()
            .unwrap()
            .contains_key("types")
    );
}
#[test]
fn unused_cycles_bad_fields_and_decimal_scales_refuse_before_candidates() {
    for (first, second, code) in [
        ("type \"Second\"", "type \"First\"", "type-alias-cycle"),
        (
            "record with [\"x\" as text, \"x\" as text]",
            "text",
            "duplicate-map-key",
        ),
        ("decimal with 19 places", "text", "decimal-scale"),
        ("type \"Missing\"", "text", "reference-missing"),
    ] {
        let (mut s, mut l) = pair("text");
        add(&mut s, &mut l, "first", "First", first);
        add(&mut s, &mut l, "second", "Second", second);
        let e = lower(&s, &l).unwrap_err();
        assert_eq!(e.code, code);
        assert!(e.span.start > 0);
    }
    for (ty, code) in [
        (
            format!("record with [\"{}\" as text]", "x".repeat(4097)),
            "type-name",
        ),
        (
            format!("choice with [\"{}\" as text]", "x".repeat(4097)),
            "type-name",
        ),
        (
            format!(
                "record with [{}]",
                (0..10001)
                    .map(|i| format!("\"f{i}\" as text"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            "type-members",
        ),
    ] {
        let (mut s, mut l) = pair("text");
        add(&mut s, &mut l, "invalid_unused", "Invalid unused", &ty);
        assert_eq!(lower(&s, &l).unwrap_err().code, code);
    }
}
#[test]
fn tiny_acyclic_dags_cannot_expand_exponentially_or_evade_expanded_depth() {
    for wide in [false, true] {
        let (mut s, mut l) = pair("text");
        add(&mut s, &mut l, "t0", "Type 0", "text");
        for i in 1..25 {
            let ty = if wide {
                format!(
                    "record with [\"a\" as type \"Type {}\", \"b\" as type \"Type {}\"]",
                    i - 1,
                    i - 1
                )
            } else {
                format!(
                    "record with [\"a\" as record with [\"b\" as type \"Type {}\"]]",
                    i - 1
                )
            };
            add(&mut s, &mut l, &format!("t{i}"), &format!("Type {i}"), &ty);
        }
        let e = lower(&s, &l).unwrap_err();
        assert!(
            matches!(e.code, "type-expansion" | "size" | "work-limit" | "depth"),
            "{e:?}"
        );
        assert!(s.len() < 10000);
    }
}

#[path = "support/named_protection.rs"]
mod protections;
#[path = "support/type_names.rs"]
mod types;
#[test]
fn actual_whole_variants_preserve_exact_ir_and_reduce_source_beyond_protection_names() {
    for name in ["rp01", "rp03"] {
        let base = root().join(format!("docs/benchmarks/{name}"));
        let raw = std::fs::read_to_string(base.join("source.choreo")).unwrap();
        let mut l: Value =
            serde_json::from_slice(&std::fs::read(base.join("bindings.json")).unwrap()).unwrap();
        let version = l["profile"].as_str().unwrap();
        let s = raw.replacen(&format!("authoring {version:?}"), "authoring \"0.17.0\"", 1);
        l["profile"] = "0.17.0".into();
        l["version"] = "0.16.0".into();
        l["contracts"]["core"]["contract"] = CORE_V12.0.into();
        l["contracts"]["core"]["digest"] = CORE_V12.1.into();
        let s = protections::factor(&s, &mut l);
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
                    std::fs::read(root().join(r["path"].as_str().unwrap())).unwrap(),
                ));
            }
        }
        let b = serde_json::to_vec(&l).unwrap();
        let expected = Package::parse(s.as_bytes(), &b)
            .unwrap()
            .lower(&supplied(&resources))
            .unwrap();
        let old = s.len();
        let s = s.replace("authoring \"0.17.0\"", "authoring \"0.18.0\"");
        l["profile"] = "0.18.0".into();
        l["version"] = "0.17.0".into();
        let s = types::factor(&s, &mut l);
        assert_eq!(s.len(), if name == "rp03" { 412288 } else { 55702 });
        let b = serde_json::to_vec(&l).unwrap();
        let p = Package::parse(s.as_bytes(), &b).unwrap();
        let c = p.lower(&supplied(&resources)).unwrap();
        assert_eq!(c.document(), expected.document());
        assert!(s.len() < old);
        let (f, b) = p.formatted().unwrap();
        assert_eq!(
            Package::parse(f.as_bytes(), b.as_bytes())
                .unwrap()
                .lower(&supplied(&resources))
                .unwrap()
                .document(),
            c.document()
        );
        c.check_static(&supplied(&resources)).unwrap();
        c.check_control(&supplied(&resources)).unwrap();
        c.check_protection(&supplied(&resources)).unwrap();
        c.check_invalidation(&supplied(&resources)).unwrap();
        if name == "rp03" {
            assert_eq!(
                c.check_concurrency(&supplied(&resources)).unwrap_err().code,
                "parallel-effects-unproved"
            );
        }
        fn observation_type(
            tree: &choreoform_authoring_frontend::Tree,
        ) -> Option<&choreoform_authoring_frontend::Tree> {
            if tree.rule == "wait_policy" {
                return tree.children.iter().find(|t| t.rule == "type");
            }
            tree.children.iter().find_map(observation_type)
        }
        let observation =
            observation_type(p.syntax().tree()).expect("whole case has a wait policy");
        let range = p.syntax().span(observation);
        let original_type = s[range.clone()].to_owned();
        let mut observed = s.clone();
        observed.replace_range(range, "type \"Observation description\"");
        add(
            &mut observed,
            &mut l,
            "observation_description",
            "Observation description",
            &original_type,
        );
        let observed_bindings = serde_json::to_vec(&l).unwrap();
        let observed_candidate = Package::parse(observed.as_bytes(), &observed_bindings)
            .unwrap()
            .lower(&supplied(&resources))
            .unwrap();
        assert_eq!(observed_candidate.document(), c.document());
        observed_candidate
            .check_static(&supplied(&resources))
            .unwrap();
        println!(
            "{name}: protection-only {old} -> local types {} bytes, exact revision {}",
            s.len(),
            c.document()["revision"]
        );
    }
}

#[test]
fn parameter_result_literal_and_initializer_types_share_exact_transparent_meaning() {
    let (mut s, mut l) = pair("whole number");
    s=s.replace("End process.","Let \"Identity\" mean (the parameter \"item\") as whole number with parameters [\"item\" as whole number]. Let \"Literal\" mean (literal of whole number with value 3) as whole number with parameters empty. End process.");
    for (id, name) in [("identity", "Identity"), ("literal", "Literal")] {
        l["declarations"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":id,"kind":"expression","scope":"scope_terminal","name":name}));
    }
    s = s.replace(
        "Initially unavailable.",
        "Initially (literal of whole number with value 3).",
    );
    l["generated"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"initial","owner":"value","role":"initial","key":null}));
    let original = lower(&s, &l).unwrap();
    let mut named = s.replace("whole number", "type \"Count\"");
    add(&mut named, &mut l, "count_type", "Count", "whole number");
    let c = lower(&named, &l).unwrap();
    assert_eq!(c.document(), original.document());
    assert_eq!(
        c.check_static(&supplied(&contracts())).unwrap_err().code,
        original
            .check_static(&supplied(&contracts()))
            .unwrap_err()
            .code
    );
    // A standalone parameterized expression still needs its existing call context.
    assert_eq!(
        c.check_static(&supplied(&contracts())).unwrap_err().code,
        "parameter-binding"
    );
}
#[test]
fn local_captions_cannot_create_nominal_membership() {
    let (mut s, mut l) = pair("text");
    s=s.replace("End process.","Let \"Wrapped\" mean ((\"value\") as named text) as text with parameters empty. End process.");
    l["declarations"].as_array_mut().unwrap().push(
        json!({"id":"wrapped","kind":"expression","scope":"scope_terminal","name":"Wrapped"}),
    );
    let original = lower(&s, &l).unwrap();
    let mut alias = s.replace("as named text", "as named type \"Not nominal\"");
    add(&mut alias, &mut l, "not_nominal", "Not nominal", "text");
    let c = lower(&alias, &l).unwrap();
    assert_eq!(c.document(), original.document());
    assert_eq!(
        c.check_static(&supplied(&contracts())).unwrap_err().code,
        original
            .check_static(&supplied(&contracts()))
            .unwrap_err()
            .code
    );
}
#[test]
fn managed_rename_reorder_copy_tombstones_annotations_and_missing_inventory_remain_explicit() {
    let (mut s, mut l) = pair("type \"Description\"");
    add(
        &mut s,
        &mut l,
        "description",
        "Description",
        "optional text",
    );
    l["annotations"]["future:opaque"] = json!({"x":[1,"kept"]});
    let c = lower(&s, &l).unwrap();
    let mut n = l.clone();
    n["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "description")
        .unwrap()["name"] = "説明".into();
    n["declarations"].as_array_mut().unwrap().reverse();
    let renamed = s.replace("Description", "説明");
    assert_eq!(lower(&renamed, &n).unwrap().document(), c.document());
    assert_eq!(lower(&renamed, &l).unwrap_err().code, "binding-mismatch");
    let mut copy = s.clone();
    let mut copied = l.clone();
    add(
        &mut copy,
        &mut copied,
        "fresh",
        "Fresh description",
        "optional text",
    );
    assert_eq!(lower(&copy, &copied).unwrap().document(), c.document());
    let mut retired = l.clone();
    retired["retired"] = json!(["description"]);
    assert_eq!(lower(&s, &retired).unwrap_err().code, "retired-identity");
    let mut surplus = l.clone();
    surplus["declarations"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"extra","kind":"type-alias","scope":"scope_terminal","name":"Extra"}));
    assert_eq!(
        lower(&s, &surplus).unwrap_err().code,
        "source-ledger-inventory"
    );
}
#[test]
fn cumulative_use_bytes_and_work_refuse_short_acyclic_alias_sources() {
    for wide in [false, true] {
        let (mut s, mut l) = pair("type \"Description\"");
        let ty = if wide {
            format!(
                "record with [{}]",
                (0..80)
                    .map(|i| format!("\"f{i}\" as text"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            format!("record with [\"{}\" as text]", "x".repeat(4000))
        };
        add(&mut s, &mut l, "description", "Description", &ty);
        let mut data = String::new();
        for i in 0..300 {
            data.push_str(&format!("Keep \"Data {i}\" as type \"Description\" under \"Access\". Sensitivity \"test\". Purposes [\"Use\"]. Participants [\"Reviewer\"]. Capabilities empty. Invalidate empty. Initially unavailable. End data.\n"));
            l["declarations"].as_array_mut().unwrap().push(json!({"id":format!("d{i}"),"kind":"data","scope":"scope_terminal","name":format!("Data {i}")}));
            l["symbols"].as_array_mut().unwrap().push(json!({"id":format!("dp{i}"),"owner":format!("d{i}"),"role":"purpose","name":"Use","wire":"use"}));
        }
        s = s.replace("End process.", &format!("{data}End process."));
        assert!(s.len() < 100000);
        let e = lower(&s, &l).unwrap_err();
        assert_eq!(e.code, if wide { "work-limit" } else { "size" }, "{e:?}");
    }
}

#[test]
fn lexical_ancestor_aliases_work_but_private_sibling_aliases_and_members_stay_hidden() {
    let (mut s, mut l) = pair("type \"Description\" in \"Terminal example\"");
    add(&mut s, &mut l, "description", "Description", "text");
    let child = "Scope \"Child\" within \"Terminal example\". Start at \"Child done\". Inputs empty. Outputs empty. Outcomes [\"complete\"]. Use cancellation \"Stop\", faults \"Fault handling\", closure \"Close\" and races \"Commit order\". Step \"Child done\". Finish as \"complete\". End step. End scope.";
    s = s.replace("End process.", &format!("{child} End process."));
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
    let start = s.find("Keep \"Value\"").unwrap();
    let end = s[start..].find("End data.").unwrap() + start + "End data.".len();
    let data = s[start..end].to_owned();
    let nested = s.replacen(&data, "", 1).replace(
        "Step \"Child done\"",
        &(data.clone() + " Step \"Child done\""),
    );
    let mut n = l.clone();
    n["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "value")
        .unwrap()["scope"] = "child".into();
    assert!(lower(&nested, &n).is_ok());
    let mut private = l.clone();
    private["declarations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "description")
        .unwrap()["scope"] = "child".into();
    let group = "Describe type \"Description\" as text.";
    let hidden = s.replacen(group, "", 1).replace(
        "Step \"Child done\"",
        &(group.to_owned() + " Step \"Child done\""),
    );
    assert_eq!(
        lower(&hidden, &private).unwrap_err().code,
        "reference-missing"
    );
    let member=s.replace("Describe type \"Description\" as text.","Describe type \"Description\" as choice with [outcome \"complete\" of \"Child done\" in \"Child\" in \"Terminal example\" as text].");
    assert_eq!(lower(&member, &l).unwrap_err().code, "reference-missing");
    let wrong = s.replace(
        "type \"Description\" in \"Terminal example\"",
        "type \"Reviewer\"",
    );
    assert_eq!(lower(&wrong, &l).unwrap_err().code, "reference-kind");
}
#[test]
fn old_source_pairs_do_not_accept_local_type_syntax_or_kind_and_future_stays_unknown() {
    use choreoform_authoring_frontend::{Ledger, parse};
    let (mut s, l) = pair("type \"Description\"");
    let mut l = l;
    add(&mut s, &mut l, "description", "Description", "text");
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
        ("0.17.0", "0.16.0", CORE_V12),
    ] {
        let source = s.replace("authoring \"0.18.0\"", &format!("authoring {profile:?}"));
        assert!(parse(source.as_bytes()).is_err(), "{profile}");
        let mut old = l.clone();
        old["profile"] = profile.into();
        old["version"] = version.into();
        old["contracts"]["core"]["contract"] = core.0.into();
        old["contracts"]["core"]["digest"] = core.1.into();
        assert_eq!(
            Ledger::decode(&serde_json::to_vec(&old).unwrap())
                .unwrap_err()
                .code,
            "binding-kind"
        );
    }
    assert_eq!(
        parse(s.replace("0.18.0", "0.19.0").as_bytes())
            .unwrap_err()
            .code,
        "profile"
    );
}

#[test]
fn nested_structure_and_forward_aliases_share_one_preflight_stack_depth() {
    let (mut s, mut l) = pair("text");
    for i in 0..60 {
        let tail = if i == 59 {
            "text".to_owned()
        } else {
            format!("type \"Deep {}\"", i + 1)
        };
        let ty = format!("{}{tail}", "optional ".repeat(10));
        add(
            &mut s,
            &mut l,
            &format!("a{i:02}"),
            &format!("Deep {i}"),
            &ty,
        );
    }
    assert!(s.len() < 20000);
    let e = lower(&s, &l).unwrap_err();
    assert_eq!(e.code, "depth");
    assert!(e.span.start > 0);
}
