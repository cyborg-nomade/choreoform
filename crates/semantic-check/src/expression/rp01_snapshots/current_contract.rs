// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Pure synthetic expression inputs, not accepted provider events or execution.
use super::*;

#[test]
fn late_category_reuses_receipt_predicate_and_matches_its_retained_request() {
    let f = Fixture::new();
    let corpus: Value = serde_json::from_slice(include_bytes!(
        "../../../tests/conformance/rp01-late-classification.json"
    ))
    .unwrap();
    let supplied = f.supplied();
    let mut r = Resources::new(&supplied).unwrap();
    let ty = r
        .type_descriptor(
            &f.candidate["body"]["policies"]["payment_wait"]["body"]["observationType"],
        )
        .unwrap();
    let request_type = r
        .type_descriptor(
            &f.candidate["body"]["capabilities"]["payment_capability"]["input"]["body"],
        )
        .unwrap();
    let mut context = Context::default();
    context
        .cells
        .insert("audit_payment_capture".into(), ty.clone());
    context
        .cells
        .insert("audit_payment_request".into(), request_type.clone());
    let judgment = r.expression(&corpus["expression"], &context).unwrap();
    assert_eq!(judgment.ty, Type::Text);
    assert_eq!(
        judgment.reads,
        BTreeSet::from([
            "audit_payment_capture".into(),
            "audit_payment_request".into()
        ])
    );
    fn bind(v: &mut Value) {
        if v["op"] == "read" {
            let name = match v["cell"].as_str().unwrap() {
                "audit_payment_capture" => "capture",
                "audit_payment_request" => "request",
                _ => panic!("unexpected live cell"),
            };
            *v = json!({"op":"param","name":name});
        } else {
            match v {
                Value::Object(m) => {
                    for child in m.values_mut() {
                        if child.is_object() || child.is_array() {
                            bind(child);
                        }
                    }
                }
                Value::Array(a) => {
                    for child in a {
                        bind(child);
                    }
                }
                _ => {}
            }
        }
    }
    let mut bound = corpus["expression"].clone();
    bind(&mut bound);
    let mut context = Context::default();
    context.parameters.insert("capture".into(), ty.clone());
    context
        .parameters
        .insert("request".into(), request_type.clone());
    assert_eq!(r.expression(&bound, &context).unwrap().ty, Type::Text);
    // Genuine counterexample to the initial tag-only experiment: a typed paid
    // payload with the wrong amount would still be called paid without the
    // rebound original receipt predicate. This is a pure test, not an event.
    let mut tag_only = bound.clone();
    tag_only["then"] = tag_only["then"]["then"].clone();
    assert_eq!(r.expression(&tag_only, &context).unwrap().ty, Type::Text);
    for case in corpus["cases"].as_array().unwrap() {
        r.value(&ty, &case["value"], 0).unwrap();
        let request = case.get("request").unwrap_or(&corpus["request"]);
        r.value(&request_type, request, 0).unwrap();
        let params = BTreeMap::from([
            ("capture".into(), case["value"].clone()),
            ("request".into(), request.clone()),
        ]);
        if case["id"] == "confirmed-paid_wrong_amount" {
            assert_eq!(
                r.evaluate(&tag_only, &params, &context, 0).unwrap(),
                json!("paid")
            );
            assert_eq!(case["expected"], "mismatched-evidence");
        }
        assert_eq!(
            r.evaluate(&bound, &params, &context, 0).unwrap(),
            case["expected"],
            "{}",
            case["id"]
        );
    }
}
