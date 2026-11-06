// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Test-only finite predicate inputs; no public evaluator or host permission.
use super::*;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource, digest, transport};

mod bindings;
mod boundaries;

const CANDIDATE: &[u8] = include_bytes!("../../../../docs/benchmarks/rp01/candidate.ir.json");
const CATALOGUE: &[u8] = include_bytes!("../../tests/conformance/rp01-guard-snapshots.json");
const ARTIFACTS: &[(&str, &[u8])] = &[
    (
        "types.json",
        include_bytes!("../../../../docs/benchmarks/rp01/types.json"),
    ),
    (
        "checks.provider.json",
        include_bytes!("../../../../docs/benchmarks/rp01/checks.provider.json"),
    ),
    (
        "payment.provider.json",
        include_bytes!("../../../../docs/benchmarks/rp01/payment.provider.json"),
    ),
    (
        "notice.provider.json",
        include_bytes!("../../../../docs/benchmarks/rp01/notice.provider.json"),
    ),
    (
        "clock.provider.json",
        include_bytes!("../../../../docs/benchmarks/rp01/clock.provider.json"),
    ),
    (
        "clock.json",
        include_bytes!("../../../../docs/benchmarks/rp01/clock.json"),
    ),
];

struct Fixture {
    candidate: Value,
    catalogue: Value,
    resources: Vec<(String, String, &'static [u8])>,
}
impl Fixture {
    fn new() -> Self {
        let (candidate, _) = transport::decode_canonical(CANDIDATE).unwrap();
        let (catalogue, _) = transport::decode_canonical(CATALOGUE).unwrap();
        let mut resources = Vec::new();
        for (_, raw) in ARTIFACTS {
            let (doc, _) = transport::decode_canonical(raw).unwrap();
            resources.push((doc["id"].as_str().unwrap().to_owned(), digest(raw), *raw));
        }
        for (pin,raw) in [
            (SEMANTICS,include_bytes!("../../../../docs/ir/contracts/sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt").as_slice()),
            (CORE,include_bytes!("../../../../docs/ir/contracts/sha256-26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58.txt").as_slice()),
            (CORE_V2,include_bytes!("../../../../docs/ir/contracts/sha256-5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623.txt").as_slice()),
            (CORE_V3,include_bytes!("../../../../docs/ir/contracts/sha256-30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3.txt").as_slice()),
            (crate::CLOCK_PROFILE,include_bytes!("../../../../docs/ir/contracts/sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt").as_slice()),
        ] { resources.push((pin.0.into(),pin.1.into(),raw)); }
        let f = Self {
            candidate,
            catalogue,
            resources,
        };
        f.validate_catalogue().unwrap();
        let e = DefinitionEnvelope::from_parts(f.candidate["body"].clone(), json!({})).unwrap();
        assert_eq!(e.document()["revision"], f.candidate["revision"]);
        crate::check_reconciliation_availability(&e, &f.supplied()).unwrap();
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
    fn validate_catalogue(&self) -> Result<()> {
        let v = &self.catalogue;
        fields(
            v,
            &[
                "format",
                "version",
                "candidateRevision",
                "candidateDigest",
                "values",
                "cases",
            ],
        )?;
        require(
            v["format"] == "choreoform-rp01-guard-snapshots" && v["version"] == "0.1.0",
            "snapshot-format",
        )?;
        require(
            v["candidateRevision"] == self.candidate["revision"]
                && v["candidateDigest"] == digest(CANDIDATE),
            "snapshot-revision",
        )?;
        object(&v["values"])?;
        let mut ids = BTreeSet::new();
        for c in items(&v["cases"])? {
            fields(
                c,
                &["id", "scenario", "subject", "cells", "facts", "expect"],
            )?;
            require(ids.insert(name(&c["id"])?), "snapshot-id")?;
            require(
                matches!(
                    text(&c["scenario"])?,
                    "RP-01-A" | "RP-01-B" | "RP-01-C" | "RP-01-D"
                ),
                "snapshot-scenario",
            )?;
            fields(&c["subject"], &["map", "id", "field", "site"])?;
            for set in ["cells", "facts"] {
                for value in object(&c[set])?.values() {
                    require(v["values"].get(name(value)?).is_some(), "snapshot-value")?;
                }
            }
            let expect = &c["expect"];
            match text(&expect["phase"])? {
                "value" => {
                    fields(expect, &["phase", "value"])?;
                    require(expect["value"].is_boolean(), "snapshot-expect")?;
                }
                "binding" | "evaluation" => {
                    fields(expect, &["phase", "code"])?;
                    name(&expect["code"])?;
                }
                _ => return Err(err("snapshot-expect")),
            }
        }
        require(!ids.is_empty(), "snapshot-cases")
    }
}

// Select only the original AST and its accepted lexical/fact type context.
fn select<'a>(
    b: &'a Value,
    subject: &Value,
    r: &mut Resources<'_>,
) -> Result<(&'a Value, Context)> {
    fields(subject, &["map", "id", "field", "site"])?;
    let map = text(&subject["map"])?;
    require(
        matches!(map, "expressions" | "policies"),
        "snapshot-subject",
    )?;
    let record = b[map]
        .get(name(&subject["id"])?)
        .ok_or_else(|| err("snapshot-subject"))?;
    let field = text(&subject["field"])?;
    let mut c = Context::default();
    if map == "expressions" {
        require(
            field == "body"
                && subject["site"].is_null()
                && object(&record["parameters"])?.is_empty(),
            "snapshot-subject",
        )?;
    } else {
        match text(&record["body"]["kind"])? {
            "actor" => {
                require(
                    field == "permit" && subject["site"].is_null(),
                    "snapshot-subject",
                )?;
                for key in ["principal", "inputRevision"] {
                    c.facts.insert(key.into(), Type::Text);
                }
                for key in [
                    "roles",
                    "authorities",
                    "relationships",
                    "priorDecisionPrincipals",
                ] {
                    c.facts.insert(key.into(), Type::List(Box::new(Type::Text)));
                }
            }
            "work" => {
                require(field == "completion", "snapshot-subject")?;
                let site = b["nodes"]
                    .get(name(&subject["site"])?)
                    .ok_or_else(|| err("snapshot-subject"))?;
                require(
                    site["kind"] == "activity" && site["policy"] == subject["id"],
                    "snapshot-subject",
                )?;
                for key in ["principal", "inputRevision", "occurrence", "observation"] {
                    c.facts.insert(key.into(), Type::Text);
                }
                let ty = if site["result"].is_null() {
                    Type::empty()
                } else {
                    r.type_descriptor(&b["data"][text(&site["result"])?]["type"]["body"])?
                };
                c.facts.insert("result".into(), ty);
            }
            "race" => {
                require(
                    field == "invariant"
                        && subject["site"].is_null()
                        && record["body"]["mode"] == "guarded",
                    "snapshot-subject",
                )?;
            }
            _ => return Err(err("snapshot-subject")),
        }
    }
    let mut scope = Some(text(&record["scope"])?);
    let mut visible = BTreeSet::new();
    while let Some(id) = scope {
        r.budget.tick(visible.len())?;
        require(visible.insert(id), "scope-cycle")?;
        scope = b["scopes"][id]["parent"].as_str();
    }
    for (id, d) in object(&b["data"])? {
        r.budget.tick(0)?;
        if visible.contains(text(&d["scope"])?) {
            c.cells
                .insert(id.clone(), r.type_descriptor(&d["type"]["body"])?);
        }
    }
    let ast = if map == "expressions" {
        &record[field]
    } else {
        &record["body"][field]
    };
    Ok((ast, c))
}

type SnapshotResult = std::result::Result<Value, (&'static str, Diagnostic)>;
fn evaluate_case(f: &Fixture, case: &Value, r: &mut Resources<'_>) -> SnapshotResult {
    let (ast, context) =
        select(&f.candidate["body"], &case["subject"], r).map_err(|e| ("catalogue", e))?;
    let original = r.expression(ast, &context).map_err(|e| ("static", e))?;
    require(original.ty == Type::Boolean, "snapshot-type").map_err(|e| ("static", e))?;
    let mut bindings = bindings::Bindings::new(r, &context, &f.catalogue["values"], case)
        .map_err(|e| ("binding", e))?;
    let bound = bindings.rebind(ast, r, 0).map_err(|e| ("binding", e))?;
    bindings.complete().map_err(|e| ("binding", e))?;
    let checked = r
        .expression(&bound, &bindings.context)
        .map_err(|e| ("static", e))?;
    require(
        checked.ty == original.ty && checked.reads.is_empty(),
        "snapshot-rebinding-type",
    )
    .map_err(|e| ("binding", e))?;
    let value = r
        .evaluate(&bound, &bindings.values, &bindings.context, 0)
        .map_err(|e| ("evaluation", e))?;
    r.literal(&original.ty, &value)
        .map_err(|e| ("evaluation", e))?;
    Ok(value)
}
fn outcome(result: SnapshotResult) -> Value {
    match result {
        Ok(value) => json!({"phase":"value","value":value}),
        Err((phase, e)) => json!({"phase":phase,"code":e.code}),
    }
}

#[test]
fn actual_rp01_guard_snapshots_match_finite_expected_values_and_faults() {
    let f = Fixture::new();
    for case in items(&f.catalogue["cases"]).unwrap() {
        let supplied = f.supplied();
        let mut r = Resources::new(&supplied).unwrap();
        assert_eq!(
            outcome(evaluate_case(&f, case, &mut r)),
            case["expect"],
            "{}",
            case["id"]
        );
    }
}
