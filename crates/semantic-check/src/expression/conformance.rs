// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Exercise the existing invariant evaluator, not a public process-read interface.
use super::*;
use choreoform_ir_probe_core::{Resource, transport};

const CORPUS: &[u8] = include_bytes!("../../tests/conformance/value-expressions.json");
const NAMED: (&str, &str) = (
    "urn:choreoform:examples:number-types:0.1.0",
    "sha256:f6bd253b88ec8732955e50cc0d9ac3ac0cff3a8975f61878075cf6eff1f0e644",
);
fn registry() -> Resources<'static> {
    Resources::new(&[
        Resource { id: CORE.0, revision: CORE.1, bytes: include_bytes!("../../../../docs/ir/contracts/sha256-26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58.txt") },
        Resource { id: CORE_V2.0, revision: CORE_V2.1, bytes: include_bytes!("../../../../docs/ir/contracts/sha256-5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623.txt") },
        Resource { id: NAMED.0, revision: NAMED.1, bytes: include_bytes!("../../../../docs/authoring/examples/service.named-types.json") },
    ]).unwrap()
}
fn corpus() -> Value {
    let (v, _) = transport::decode_canonical(CORPUS).unwrap();
    fields(
        &v,
        &[
            "format",
            "version",
            "contract",
            "base",
            "namedResource",
            "cases",
        ],
    )
    .unwrap();
    assert_eq!(v["format"], "choreoform-value-conformance");
    assert_eq!(v["version"], "0.1.0");
    let mut r = registry();
    for (key, expected) in [
        ("contract", CORE_V2),
        ("base", CORE),
        ("namedResource", NAMED),
    ] {
        let pin = Pin::decode(&v[key]).unwrap();
        assert_eq!((pin.id.as_str(), pin.revision.as_str()), expected);
        r.verify(&pin).unwrap();
    }
    let mut ids = BTreeSet::new();
    for c in items(&v["cases"]).unwrap() {
        fields(c, &["id", "rule", "binding", "expression", "expect"]).unwrap();
        assert!(ids.insert(name(&c["id"]).unwrap()));
        name(&c["rule"]).unwrap();
        if !c["binding"].is_null() {
            fields(&c["binding"], &["type", "value"]).unwrap();
        }
        fields(
            &c["expect"],
            match text(&c["expect"]["phase"]).unwrap() {
                "value" => &["phase", "type", "value"],
                "static" => &["phase", "code"],
                "evaluation" => &["phase", "code", "type"],
                _ => panic!("unknown corpus phase"),
            },
        )
        .unwrap();
    }
    v
}
fn context(r: &mut Resources<'_>, c: &Value) -> (Context, BTreeMap<String, Value>) {
    let mut context = Context::default();
    let mut params = BTreeMap::new();
    if !c["binding"].is_null() {
        let ty = r.type_descriptor(&c["binding"]["type"]).unwrap();
        r.literal(&ty, &c["binding"]["value"]).unwrap();
        context.parameters.insert("value".into(), ty);
        params.insert("value".into(), c["binding"]["value"].clone());
    }
    (context, params)
}

#[test]
fn exact_value_vectors_use_the_production_static_and_invariant_judgments() {
    let v = corpus();
    let mut count = 0;
    for c in items(&v["cases"])
        .unwrap()
        .iter()
        .filter(|c| c["expect"]["phase"] == "value")
    {
        count += 1;
        let mut r = registry();
        let (context, params) = context(&mut r, c);
        let j = r
            .expression(&c["expression"], &context)
            .unwrap_or_else(|e| panic!("{}: {e:?}", c["id"]));
        assert!(j.reads.is_empty(), "{}", c["id"]);
        assert_eq!(
            j.ty,
            r.type_descriptor(&c["expect"]["type"]).unwrap(),
            "{}",
            c["id"]
        );
        let value = r
            .evaluate(&c["expression"], &params, &context, 0)
            .unwrap_or_else(|e| panic!("{}: {e:?}", c["id"]));
        assert_eq!(value, c["expect"]["value"], "{}", c["id"]);
        r.literal(&j.ty, &value).unwrap();
    }
    assert_eq!(count, 42);
}

#[test]
fn full_static_checking_refuses_invalid_and_unselected_forms() {
    let v = corpus();
    let mut count = 0;
    for c in items(&v["cases"])
        .unwrap()
        .iter()
        .filter(|c| c["expect"]["phase"] == "static")
    {
        count += 1;
        let mut r = registry();
        let (context, _) = context(&mut r, c);
        let error = r.expression(&c["expression"], &context).unwrap_err();
        assert_eq!(error.code, c["expect"]["code"], "{}", c["id"]);
        assert!(!format!("{error:?}").contains("SECRET-MUST-NOT-APPEAR"));
    }
    assert_eq!(count, 16);
}

#[test]
fn selected_evaluation_faults_are_distinct_from_static_refusals() {
    let v = corpus();
    let mut count = 0;
    for c in items(&v["cases"])
        .unwrap()
        .iter()
        .filter(|c| c["expect"]["phase"] == "evaluation")
    {
        count += 1;
        let mut r = registry();
        let (context, params) = context(&mut r, c);
        let j = r.expression(&c["expression"], &context).unwrap();
        assert_eq!(
            j.ty,
            r.type_descriptor(&c["expect"]["type"]).unwrap(),
            "{}",
            c["id"]
        );
        let error = r
            .evaluate(&c["expression"], &params, &context, 0)
            .unwrap_err();
        assert_eq!(error.code, c["expect"]["code"], "{}", c["id"]);
    }
    assert_eq!(count, 11);
}
