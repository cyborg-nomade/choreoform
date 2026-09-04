// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

//! Test adapter: fixed fixture/resource inputs, shared by native and browser.
//! Embedded contracts belong here, never in the portable core's host boundary.

use choreoform_ir_probe_core::{self as core, Resource, SUPPORTED_CONTRACTS};
use serde_json::{Value, json};

const FIXTURES: [(&str, &[u8], &str); 3] = [
    (
        "reimbursement",
        include_bytes!("../../../examples/ir/01-reimbursement.json"),
        "sha256:4c9c8bbad9f0c05523262979eca8f8b6449e3f310004d83c39ffaee1670eae62",
    ),
    (
        "order",
        include_bytes!("../../../examples/ir/03-order.json"),
        "sha256:4089e28a5c025ce4814f7f518a1d9619ac48a940e9f82c9d2400956312e67faf",
    ),
    (
        "incident",
        include_bytes!("../../../examples/ir/08-incident.json"),
        "sha256:a085966fc7e9077f8d590ea0ac7e859de25786f673a6e6b7753d48c37abc87e7",
    ),
];
const ARTIFACTS: [&[u8]; 2] = [
    include_bytes!(
        "../../../docs/ir/contracts/sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt"
    ),
    include_bytes!(
        "../../../docs/ir/contracts/sha256-282dd3c02983141da13e8f700e852d5472ffc4af176a53ca3578c418482a09b9.txt"
    ),
];

pub fn resources() -> Vec<Resource<'static>> {
    SUPPORTED_CONTRACTS
        .into_iter()
        .zip(ARTIFACTS)
        .map(|((id, revision), bytes)| Resource {
            id,
            revision,
            bytes,
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Operation {
    Inspect,
    Decode,
    Projection,
}

struct Case {
    name: String,
    raw: Vec<u8>,
    operation: Operation,
    expected: &'static str,
    revision: Option<&'static str>,
    canonical: Option<String>,
    registry: &'static str,
}

fn case(name: &str, raw: impl Into<Vec<u8>>, expected: &'static str) -> Case {
    Case {
        name: name.into(),
        raw: raw.into(),
        operation: Operation::Inspect,
        expected,
        revision: None,
        canonical: None,
        registry: "normal",
    }
}

fn changed(name: &str, expected: &'static str, edit: impl FnOnce(&mut Value)) -> Case {
    let mut document = core::transport::decode(FIXTURES[0].1).expect("fixture JSON");
    edit(&mut document);
    case(
        name,
        serde_json::to_vec(&document).expect("fixture serialization"),
        expected,
    )
}

fn revised(name: &str, edit: impl FnOnce(&mut Value)) -> Case {
    let mut c = changed(name, "ok", edit);
    let canonical = core::semantic_bytes(&c.raw).expect("valid test envelope");
    let mut document = core::transport::decode(&c.raw).expect("valid test JSON");
    document["revision"] = json!(core::digest(canonical.as_bytes()));
    c.raw = serde_json::to_vec(&document).expect("fixture serialization");
    c
}

fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (name, raw, revision) in FIXTURES {
        let mut c = case(name, raw, "ok");
        c.revision = Some(revision);
        cases.push(c);
    }
    let base = std::str::from_utf8(FIXTURES[0].1).expect("UTF8 fixture");
    cases.push(case(
        "duplicate-envelope",
        base.replacen("{", "{\"format\":\"choreoform-ir\",", 1),
        "duplicate-key",
    ));
    cases.push(case(
        "duplicate-escaped-key",
        br#"{"body":{"a":0,"\u0061":1}}"#,
        "duplicate-key",
    ));
    cases.push(case(
        "duplicate-inside-array",
        br#"[{"x":0,"x":1}]"#,
        "duplicate-key",
    ));
    cases.push(case(
        "fraction-in-annotations",
        base.replacen("\"annotations\": {", "\"annotations\": {\"bad\":1.0,", 1),
        "number-token",
    ));
    cases.push(case(
        "exponent-in-dialect",
        base.replacen(
            "\"name\": \"Expense\"",
            "\"name\": \"Expense\", \"bad\":1e0",
            1,
        ),
        "number-token",
    ));
    for (name, raw, expected) in [
        ("fraction", "1.0", "number-token"),
        ("exponent", "1e0", "number-token"),
        ("negative-exponent", "-1E+3", "number-token"),
        ("negative-zero-fraction", "-0.0", "number-token"),
        ("unsafe-positive", "9007199254740992", "integer-range"),
        ("unsafe-negative", "-9007199254740992", "integer-range"),
        (
            "integer-overflow",
            "9999999999999999999999999999999999999999",
            "integer-range",
        ),
        ("nan", "NaN", "json"),
        ("infinity", "Infinity", "json"),
        ("negative-infinity", "-Infinity", "json"),
        ("leading-zero", "01", "json"),
        ("plus-sign", "+1", "json"),
        ("bare-minus", "-", "json"),
        ("bom", "\u{feff}{}", "json"),
        ("high-surrogate", r#""\ud800""#, "json"),
        ("low-surrogate", r#""\udfff""#, "json"),
        ("surrogate-key", r#"{"\ud800":0}"#, "json"),
        ("bad-escape", r#""\q""#, "json"),
        ("raw-control", "\"\n\"", "json"),
        ("trailing-comma", "[1,]", "json"),
        ("trailing-object-comma", "{\"a\":0,}", "json"),
        ("trailing-data", "{}{}", "json"),
        ("comment", "/*x*/{}", "json"),
        ("empty", "", "json"),
        ("truncated-string", "\"x\\", "json"),
    ] {
        cases.push(case(name, raw, expected));
    }
    cases.push(case("invalid-utf8", vec![b'"', 0xff, b'"'], "utf8"));
    cases.push(case(
        "encoded-surrogate-utf8",
        vec![b'"', 0xed, 0xa0, 0x80, b'"'],
        "utf8",
    ));
    cases.push(case("overlong-utf8", vec![0xc0, 0xaf], "utf8"));
    cases.push(case(
        "oversize",
        vec![b' '; core::transport::MAX_BYTES + 1],
        "size",
    ));
    cases.push(case(
        "too-deep",
        format!("{}0{}", "[".repeat(65), "]".repeat(65)),
        "depth",
    ));
    cases.push(case(
        "too-deep-key",
        format!("{}{{\"k\":0}}{}", "[".repeat(64), "]".repeat(64)),
        "depth",
    ));
    for (name, raw) in [
        (
            "depth-boundary",
            format!("{}0{}", "[".repeat(64), "]".repeat(64)),
        ),
        (
            "size-boundary",
            format!("0{}", " ".repeat(core::transport::MAX_BYTES - 1)),
        ),
        (
            "safe-integers",
            "[-9007199254740991,9007199254740991,-0]".into(),
        ),
        ("paired-surrogate", r#""\ud83d\ude00""#.into()),
        (
            "all-json-values",
            r#"[null,true,false,{},[],"\\\"\/\b\f\n\r\t\u0000"]"#.into(),
        ),
    ] {
        let mut c = case(name, raw, "ok");
        c.operation = Operation::Decode;
        cases.push(c);
    }
    for (name, pointer, value, expected) in [
        ("unsupported-version", "/version", json!("0.2.0"), "version"),
        ("wrong-format", "/format", json!("other"), "envelope"),
        ("wrong-artifact", "/kind", json!("plan"), "envelope"),
        (
            "bad-revision-spelling",
            "/revision",
            json!("sha256:AB"),
            "envelope",
        ),
        ("null-annotations", "/annotations", Value::Null, "envelope"),
        (
            "unknown-semantics",
            "/body/semantics/id",
            json!("https://invalid.example/no-fetch"),
            "contract-unsupported",
        ),
        (
            "unknown-contract-revision",
            "/body/semantics/revision",
            json!(format!("sha256:{}", "0".repeat(64))),
            "contract-unsupported",
        ),
        (
            "unknown-dialect",
            "/body/dialects/illustrative/id",
            json!("urn:unknown"),
            "contract-unsupported",
        ),
        ("dangling-root", "/body/root", json!("absent"), "reference"),
        (
            "wrong-kind-root",
            "/body/root",
            json!("expense"),
            "reference",
        ),
        (
            "scope-cycle",
            "/body/scopes/root/parent",
            json!("root"),
            "scope",
        ),
        (
            "dangling-node-scope",
            "/body/nodes/review/scope",
            json!("absent"),
            "reference",
        ),
        (
            "unknown-node-kind",
            "/body/nodes/review/kind",
            json!("extension"),
            "unknown-node-kind",
        ),
        (
            "wrong-kind-actor",
            "/body/nodes/review/target",
            json!("expense"),
            "reference",
        ),
        (
            "dangling-actor",
            "/body/nodes/review/target",
            json!("absent"),
            "reference",
        ),
        (
            "non-set",
            "/body/nodes/review/reads/expense",
            json!(false),
            "shape",
        ),
        (
            "missing-purpose",
            "/body/data/expense/protection/purposes",
            json!({}),
            "shape",
        ),
        (
            "wrong-kind-protection-policy",
            "/body/data/expense/protection/policy",
            json!("operator"),
            "reference",
        ),
        (
            "missing-type-dialect",
            "/body/data/expense/type/dialect",
            json!("unknown"),
            "reference",
        ),
        (
            "tampered-semantic-body",
            "/body/id",
            json!("urn:edited"),
            "revision",
        ),
    ] {
        cases.push(changed(name, expected, |d| {
            *d.pointer_mut(pointer).expect("test path") = value
        }));
    }
    cases.push(changed("unknown-envelope-field", "envelope", |d| {
        d["extra"] = json!(0)
    }));
    cases.push(changed("missing-envelope-field", "envelope", |d| {
        d.as_object_mut().unwrap().remove("annotations");
    }));
    cases.push(changed("unknown-body-field", "shape", |d| {
        d["body"]["extra"] = json!(0)
    }));
    cases.push(changed("unknown-node-field", "shape", |d| {
        d["body"]["nodes"]["review"]["extra"] = json!(0)
    }));
    cases.push(changed("missing-protection", "shape", |d| {
        d["body"]["data"]["expense"]
            .as_object_mut()
            .unwrap()
            .remove("protection");
    }));
    cases.push(changed("duplicate-declaration-id", "reference", |d| {
        d["body"]["data"]["review"] = d["body"]["data"]["expense"].clone()
    }));
    cases.push(changed("annotation-only", "ok", |d| {
        d["annotations"] = json!({"label":"different", "unicode":"😀"})
    }));
    cases.last_mut().unwrap().revision = Some(FIXTURES[0].2);
    let compact = core::transport::decode(FIXTURES[0].1).unwrap();
    let mut reordered = case("map-order-and-whitespace", reverse_json(&compact), "ok");
    reordered.revision = Some(FIXTURES[0].2);
    cases.push(reordered);
    for (registry, expected) in [
        ("missing", "contract-missing"),
        ("corrupt", "contract-digest"),
        ("duplicate", "contract-digest"),
        ("missing-dialect", "contract-missing"),
        ("corrupt-dialect", "contract-digest"),
        ("duplicate-dialect", "contract-digest"),
        ("reversed", "ok"),
    ] {
        let mut c = case(&format!("registry-{registry}"), FIXTURES[0].1, expected);
        c.registry = registry;
        cases.push(c);
    }
    cases.push(changed(
        "swapped-contract-role",
        "contract-unsupported",
        |d| d["body"]["semantics"] = d["body"]["dialects"]["illustrative"].clone(),
    ));
    cases.push(revised("empty-access-retained", |d| {
        d["body"]["data"]["expense"]["protection"]["participants"] = json!({});
        d["body"]["data"]["expense"]["protection"]["capabilities"] = json!({});
    }));
    cases.push(revised("policy-edit", |d| {
        d["body"]["policies"]["p_protection"]["body"]["rule"] = json!("changed-policy")
    }));
    for values in [json!([1, 2, 3]), json!([3, 2, 1])] {
        cases.push(revised(&format!("ordered-array-{values}"), |d| {
            d["body"]["policies"]["p_protection"]["body"]["ordered"] = values
        }));
    }
    // Deliberately partial graph fixtures exercise the remaining closed variants;
    // these are NOT claimed to be schema-valid or executable process examples.
    for (name, extra) in [
        (
            "compute",
            json!({"assignments":{"expense":"expense_input"}}),
        ),
        (
            "invoke",
            json!({"body":"root", "inputs":{"expense":"expense_input"}, "outputs":{}}),
        ),
        (
            "decision",
            json!({"guards":{"approved":"expense_input"}, "default":null}),
        ),
        (
            "repeat",
            json!({"body":"root", "condition":"expense_input"}),
        ),
    ] {
        cases.push(revised(&format!("typed-{name}"), |d| {
            let old = &d["body"]["nodes"]["review"];
            let mut node = json!({"kind":name,"scope":old["scope"],"outcomes":old["outcomes"],"reads":old["reads"],"writes":old["writes"]});
            node.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
            d["body"]["nodes"]["review"] = node;
        }));
    }
    // Literal expected JCS vector, independently of the implementation under test.
    let vector_body = r#"{"\ue000":1,"\ud83d\ude00":2,"z":"\u000f\n\"\\/","numbers":[-0,9007199254740991,-9007199254740991],"combining":["é","é"]}"#;
    let raw = format!(
        r#"{{"format":"choreoform-ir","version":"0.1.0","kind":"definition","revision":"sha256:{}","annotations":{{}},"body":{vector_body}}}"#,
        "0".repeat(64)
    );
    let mut c = case("jcs-utf16-escapes-no-normalization", raw, "ok");
    c.operation = Operation::Projection;
    c.canonical = Some("{\"body\":{\"combining\":[\"é\",\"é\"],\"numbers\":[0,9007199254740991,-9007199254740991],\"z\":\"\\u000f\\n\\\"\\\\/\",\"😀\":2,\"\u{e000}\":1},\"format\":\"choreoform-ir\",\"kind\":\"definition\",\"version\":\"0.1.0\"}".into());
    cases.push(c);
    cases
}

fn reverse_json(value: &Value) -> String {
    match value {
        Value::Object(map) => format!(
            "{{ {} }}",
            map.iter()
                .rev()
                .map(|(k, v)| format!("{} : {}", json!(k), reverse_json(v)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Value::Array(list) => format!(
            "[{}]",
            list.iter().map(reverse_json).collect::<Vec<_>>().join(",")
        ),
        _ => value.to_string(),
    }
}

fn run(c: &Case) -> Value {
    let mut registry = resources();
    match c.registry {
        "missing" => registry.clear(),
        "corrupt" => registry[0].bytes = b"corrupt",
        "duplicate" => registry.push(resources().remove(0)),
        "missing-dialect" => {
            registry.remove(1);
        }
        "corrupt-dialect" => registry[1].bytes = b"corrupt",
        "duplicate-dialect" => registry.push(resources().remove(1)),
        "reversed" => registry.reverse(),
        _ => {}
    }
    let result = match c.operation {
        Operation::Inspect => inspect_output(&c.raw, &registry),
        Operation::Decode => {
            core::transport::decode(&c.raw).map(|v| json!({"category":"ok", "decoded":v}))
        }
        Operation::Projection => core::semantic_bytes(&c.raw).map(
            |v| json!({"category":"ok", "revision":core::digest(v.as_bytes()), "canonical":v}),
        ),
    };
    result.unwrap_or_else(|error| json!({"category":error.category()}))
}

use std::collections::BTreeSet;

fn inspect_output(raw: &[u8], registry: &[Resource<'_>]) -> core::Result<Value> {
    core::inspect(raw, registry).map(|v| {
        let kinds: BTreeSet<_> = v.graph.nodes.values().map(|n| n.kind.name()).collect();
        json!({"category":"ok", "canonical":v.canonical, "revision":v.revision, "document":v.document, "nodeKinds":kinds})
    })
}

/// Browser byte-input boundary using the test adapter's explicit pinned resources.
/// The host must bound transfers before calling; the core also enforces its bound.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn inspect_bytes(raw: &[u8]) -> String {
    inspect_output(raw, &resources())
        .unwrap_or_else(|error| json!({"category":error.category()}))
        .to_string()
}

/// ABI is a JSON report string, not Rust memory layout. Versioned for the probe.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn run_suite() -> String {
    let results: Vec<_> = cases()
        .into_iter()
        .map(|c| {
            let actual = run(&c);
            let passed = actual["category"] == c.expected
                && c.revision.is_none_or(|r| actual["revision"] == r)
                && c.canonical
                    .as_ref()
                    .is_none_or(|v| actual["canonical"] == *v);
            json!({"name":c.name, "expected":c.expected, "passed":passed, "actual":actual})
        })
        .collect();
    let passed = results.iter().all(|r| r["passed"] == true);
    json!({"reportVersion":"0.1.0", "passed":passed, "cases":results}).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn shared_suite_passes() {
        let report: serde_json::Value = serde_json::from_str(&super::run_suite()).unwrap();
        for row in report["cases"].as_array().unwrap() {
            assert_eq!(row["passed"], true, "{}: {}", row["name"], row["actual"]);
        }
    }
}
