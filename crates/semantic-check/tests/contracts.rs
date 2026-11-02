// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use choreoform_ir_probe_core::{Resource, digest};
use choreoform_semantic_check::{Context, Pin, Resources, Type};
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn ty(kind: &str) -> Value {
    json!({"kind":kind})
}
fn lit(t: Value, v: Value) -> Value {
    json!({"op":"literal","type":t,"value":v})
}
fn i(n: i64) -> Value {
    lit(ty("integer"), json!(n.to_string()))
}
fn b(v: bool) -> Value {
    lit(ty("boolean"), json!(v))
}
fn bin(op: &str, a: Value, b: Value) -> Value {
    json!({"op":op,"left":a,"right":b})
}
fn param() -> Value {
    json!({"op":"param","name":"value"})
}
fn named(representation: Value, invariant: Value) -> Value {
    json!({"kind":"named-types","version":"0.1.0","id":"urn:test:numbers","definitions":{"Number":{"representation":representation,"invariant":invariant}}})
}
fn named_type(pin: &Pin) -> Value {
    json!({"kind":"named","contract":{"id":pin.id,"revision":pin.revision},"name":"Number"})
}
fn pin(id: &str, bytes: &[u8]) -> Pin {
    Pin {
        id: id.into(),
        revision: digest(bytes),
    }
}
fn with_artifact(doc: Value, f: impl FnOnce(&mut Resources<'_>, Pin)) {
    let bytes = serde_json::to_vec(&doc).unwrap();
    let p = pin(doc["id"].as_str().unwrap(), &bytes);
    let supplied = [Resource {
        id: &p.id,
        revision: &p.revision,
        bytes: &bytes,
    }];
    let mut r = Resources::new(&supplied).unwrap();
    f(&mut r, p.clone());
}

#[test]
fn exact_type_and_value_catalogue() {
    let mut r = Resources::new(&[]).unwrap();
    for (descriptor, value) in [
        (ty("boolean"), json!(true)),
        (ty("text"), json!("é\n")),
        (ty("integer"), json!("-9223372036854775808")),
        (
            json!({"kind":"decimal","scale":18}),
            json!("9223372036854775807"),
        ),
        (
            json!({"kind":"record","fields":{"x":ty("integer")}}),
            json!({"x":"0"}),
        ),
        (
            json!({"kind":"variant","cases":{"ok":ty("text")}}),
            json!({"tag":"ok","value":"yes"}),
        ),
        (
            json!({"kind":"list","element":ty("integer")}),
            json!(["0", "1"]),
        ),
        (
            json!({"kind":"option","element":ty("text")}),
            json!({"tag":"none"}),
        ),
        (
            json!({"kind":"option","element":ty("text")}),
            json!({"tag":"some","value":""}),
        ),
    ] {
        let t = r.type_descriptor(&descriptor).unwrap();
        r.literal(&t, &value).unwrap();
    }
    for v in [
        json!(0),
        json!(1.0),
        json!("-0"),
        json!("00"),
        json!("+1"),
        json!("1e1"),
        json!("9223372036854775808"),
    ] {
        assert!(r.literal(&Type::Integer, &v).is_err(), "{v}");
    }
    for d in [
        json!({"kind":"number"}),
        json!({"kind":"integer","extra":true}),
        json!({"kind":"decimal","scale":19}),
        json!({"kind":"variant","cases":{}}),
    ] {
        assert!(r.type_descriptor(&d).is_err());
    }
    let record = r
        .type_descriptor(&json!({"kind":"record","fields":{}}))
        .unwrap();
    assert!(r.literal(&record, &json!({"extra":true})).is_err());
    let option = Type::Option(Box::new(Type::Text));
    for v in [
        Value::Null,
        json!({"tag":"none","value":""}),
        json!({"tag":"some"}),
    ] {
        assert!(r.literal(&option, &v).is_err());
    }
}

#[test]
fn complete_closed_expression_operations_and_all_branch_dependencies() {
    let mut r = Resources::new(&[]).unwrap();
    let variant = json!({"kind":"variant","cases":{"ok":ty("integer")}});
    let list = json!({"op":"list","element":ty("integer"),"items":[i(1)]});
    let case = json!({"op":"variant","type":variant,"tag":"ok","value":i(1)});
    let some = json!({"op":"some","value":i(1)});
    let context = Context {
        cells: BTreeMap::from([("a".into(), Type::Integer), ("b".into(), Type::Integer)]),
        parameters: BTreeMap::from([("x".into(), Type::Text)]),
        ..Context::default()
    };
    let mut cases = vec![
        (i(1), Type::Integer),
        (json!({"op":"read","cell":"a"}), Type::Integer),
        (json!({"op":"param","name":"x"}), Type::Text),
        (
            json!({"op":"field","name":"x","value":{"op":"record","fields":{"x":i(1)}}}),
            Type::Integer,
        ),
        (
            json!({"op":"record","fields":{}}),
            Type::Record(BTreeMap::new()),
        ),
        (list.clone(), Type::List(Box::new(Type::Integer))),
        (case.clone(), r.type_descriptor(&variant).unwrap()),
        (
            json!({"op":"none","element":ty("integer")}),
            Type::Option(Box::new(Type::Integer)),
        ),
        (some.clone(), Type::Option(Box::new(Type::Integer))),
        (json!({"op":"isSome","value":some}), Type::Boolean),
        (
            json!({"op":"unwrap","value":{"op":"some","value":i(1)}}),
            Type::Integer,
        ),
        (json!({"op":"tag","value":case}), Type::Text),
        (
            json!({"op":"payload","tag":"ok","value":case}),
            Type::Integer,
        ),
        (
            json!({"op":"if","condition":b(true),"then":i(1),"else":i(2)}),
            Type::Integer,
        ),
        (json!({"op":"not","value":b(true)}), Type::Boolean),
        (json!({"op":"length","value":list}), Type::Integer),
        (
            json!({"op":"contains","list":list,"value":i(1)}),
            Type::Boolean,
        ),
        (
            json!({"op":"index","value":list,"index":i(0)}),
            Type::Integer,
        ),
        (
            json!({"op":"rescale","value":lit(json!({"kind":"decimal","scale":2}),json!("120")),"scale":3}),
            Type::Decimal(3),
        ),
    ];
    for op in ["and", "or"] {
        cases.push((bin(op, b(true), b(false)), Type::Boolean));
    }
    for op in ["eq", "lt"] {
        cases.push((bin(op, i(1), i(2)), Type::Boolean));
    }
    for op in ["add", "sub", "mul"] {
        cases.push((bin(op, i(1), i(2)), Type::Integer));
    }
    for (ast, expected) in cases {
        assert_eq!(r.expression(&ast, &context).unwrap().ty, expected, "{ast}");
    }
    let all = json!({"op":"if","condition":b(true),"then":{"op":"read","cell":"a"},"else":{"op":"read","cell":"b"}});
    assert_eq!(
        r.expression(&all, &context)
            .unwrap()
            .reads
            .into_iter()
            .collect::<Vec<_>>(),
        ["a", "b"]
    );
    let dead = json!({"op":"if","condition":b(true),"then":i(1),"else":bin("add",i(1),b(false))});
    assert_eq!(
        r.expression(&dead, &context).unwrap_err().code,
        "expression-type"
    );
    for ast in [
        json!({"op":"read","cell":"missing"}),
        json!({"op":"param","name":"missing"}),
        json!({"op":"fact","name":"principal"}),
        json!({"op":"eval","value":"secret"}),
        json!({"op":"literal","type":ty("integer"),"value":"1","extra":true}),
        bin(
            "mul",
            lit(json!({"kind":"decimal","scale":2}), json!("1")),
            lit(json!({"kind":"decimal","scale":2}), json!("1")),
        ),
    ] {
        assert!(r.expression(&ast, &context).is_err());
    }
    let mut facts = Context::default();
    facts.facts.insert("principal".into(), Type::Text);
    assert_eq!(
        r.expression(&json!({"op":"fact","name":"principal"}), &facts)
            .unwrap()
            .ty,
        Type::Text
    );
}

#[test]
fn nominal_invariants_are_checked_exactly_and_cannot_read_ambient_data() {
    let invariant = json!({"op":"not","value":bin("lt",param(),i(0))});
    with_artifact(named(ty("integer"), invariant), |r, p| {
        let t = r.type_descriptor(&named_type(&p)).unwrap();
        r.literal(&t, &json!("0")).unwrap();
        assert_eq!(
            r.literal(&t, &json!("-1")).unwrap_err().code,
            "invalid-nominal-value"
        );
        let wrap = json!({"op":"wrap","type":named_type(&p),"value":i(1)});
        assert_eq!(r.expression(&wrap, &Context::default()).unwrap().ty, t);
        assert_eq!(
            r.expression(
                &json!({"op":"unwrapNamed","value":wrap}),
                &Context::default()
            )
            .unwrap()
            .ty,
            Type::Integer
        );
        assert_ne!(t, Type::Integer);
    });
    for bad in [
        json!({"op":"read","cell":"secret"}),
        json!({"op":"fact","name":"principal"}),
        json!({"op":"param","name":"other"}),
        i(1),
    ] {
        with_artifact(named(ty("integer"), bad), |r, p| {
            assert!(r.named_types(&p).is_err());
        });
    }
    let mut doc = named(ty("integer"), b(true));
    doc["definitions"]["Unused"] =
        json!({"representation":ty("integer"),"invariant":{"op":"unknown"}});
    with_artifact(doc, |r, p| {
        assert!(r.type_descriptor(&named_type(&p)).is_err());
        assert!(r.type_descriptor(&named_type(&p)).is_err());
    });
}

#[test]
fn invariant_evaluation_covers_constructors_short_circuit_bounds_and_exact_arithmetic() {
    let list = json!({"op":"list","element":ty("integer"),"items":[i(2)]});
    let variant = json!({"op":"variant","type":{"kind":"variant","cases":{"ok":ty("integer")}},"tag":"ok","value":i(2)});
    for (value, expected) in [
        (bin("add", param(), i(1)), i(3)),
        (bin("sub", param(), i(1)), i(1)),
        (bin("mul", param(), i(2)), i(4)),
        (
            json!({"op":"field","name":"x","value":{"op":"record","fields":{"x":param()}}}),
            i(2),
        ),
        (
            json!({"op":"unwrap","value":{"op":"some","value":param()}}),
            i(2),
        ),
        (
            json!({"op":"isSome","value":{"op":"none","element":ty("integer")}}),
            b(false),
        ),
        (
            json!({"op":"tag","value":variant}),
            lit(ty("text"), json!("ok")),
        ),
        (json!({"op":"payload","value":variant,"tag":"ok"}), i(2)),
        (json!({"op":"index","value":list,"index":i(0)}), i(2)),
        (json!({"op":"length","value":list}), i(1)),
        (
            json!({"op":"contains","list":list,"value":param()}),
            b(true),
        ),
        (
            json!({"op":"if","condition":b(true),"then":param(),"else":{"op":"unwrap","value":{"op":"none","element":ty("integer")}}}),
            i(2),
        ),
        (
            bin(
                "and",
                b(false),
                bin("eq", bin("add", i(i64::MAX), i(1)), i(0)),
            ),
            b(false),
        ),
        (
            bin(
                "or",
                b(true),
                bin("eq", bin("add", i(i64::MAX), i(1)), i(0)),
            ),
            b(true),
        ),
    ] {
        with_artifact(named(ty("integer"), bin("eq", value, expected)), |r, p| {
            let t = r.type_descriptor(&named_type(&p)).unwrap();
            r.literal(&t, &json!("2")).unwrap();
        });
    }
    for (op, expected) in [
        (bin("add", i(i64::MAX), i(1)), "overflow"),
        (
            json!({"op":"index","value":list,"index":i(-1)}),
            "index-bounds",
        ),
        (
            json!({"op":"unwrap","value":{"op":"none","element":ty("integer")}}),
            "evaluation-tag",
        ),
    ] {
        with_artifact(named(ty("integer"), bin("eq", op, i(0))), |r, p| {
            let t = r.type_descriptor(&named_type(&p)).unwrap();
            assert_eq!(r.literal(&t, &json!("2")).unwrap_err().code, expected);
        });
    }
    for (input, expected) in [("120", Ok(())), ("121", Err("inexact-rescale"))] {
        let inv = bin(
            "eq",
            json!({"op":"rescale","value":param(),"scale":1}),
            lit(json!({"kind":"decimal","scale":1}), json!("12")),
        );
        with_artifact(named(json!({"kind":"decimal","scale":2}), inv), |r, p| {
            let t = r.type_descriptor(&named_type(&p)).unwrap();
            assert_eq!(r.literal(&t, &json!(input)).map_err(|e| e.code), expected);
        });
    }
}

fn provider() -> Value {
    json!({"kind":"provider-contract","version":"0.1.0","id":"urn:test:provider","input":ty("text"),"output":ty("integer"),"outcomes":{"ok":true},"resultProtection":{"policy":"protect","sensitivity":"internal","purposes":{"process":true},"participants":{"reviewer":true},"capabilities":{}},"observations":{"success":{"state":"succeeded","result":ty("integer"),"outcome":"ok"},"unknown":{"state":"unknown","result":null,"outcome":null}},"idempotency":"stable-key","effectClass":"read-only"})
}
#[test]
fn providers_require_closed_shapes_total_outcomes_and_exact_results() {
    with_artifact(provider(), |r, p| {
        let p = r.provider(&p).unwrap();
        assert_eq!(p.input(), &Type::Text);
        assert_eq!(p.output(), &Type::Integer);
        assert!(p.outcomes().contains("ok"));
    });
    for mutation in [
        "extra",
        "type",
        "missing-outcome",
        "undeclared-outcome",
        "unknown-result",
        "state",
        "id",
        "version",
        "effect",
        "idempotency",
        "protection",
    ] {
        let mut p = provider();
        match mutation {
            "extra" => p["extra"] = true.into(),
            "type" => p["observations"]["success"]["result"] = ty("text"),
            "missing-outcome" => p["outcomes"]["missing"] = true.into(),
            "undeclared-outcome" => p["observations"]["success"]["outcome"] = "other".into(),
            "unknown-result" => p["observations"]["unknown"]["result"] = ty("integer"),
            "state" => p["observations"]["unknown"]["state"] = "timeout-means-failure".into(),
            "id" => p["kind"] = "other".into(),
            "version" => p["version"] = "0.2.0".into(),
            "effect" => p["effectClass"] = "guaranteed".into(),
            "idempotency" => p["idempotency"] = "trust-me".into(),
            _ => p["resultProtection"]["purposes"] = json!({}),
        }
        with_artifact(p, |r, pin| assert!(r.provider(&pin).is_err(), "{mutation}"));
    }
}

#[test]
fn calendar_tables_verify_pins_but_do_not_enable_clock_support() {
    let clock = b"host clock bytes";
    let zone = b"pinned zone-rule bytes";
    let c = pin("urn:test:clock", clock);
    let z = pin("urn:test:zone", zone);
    for intervals in [
        json!([]),
        json!([{"start":"-1","end":"0"},{"start":"0","end":"3"}]),
        json!([{"start":"1","end":"1"}]),
        json!([{"start":"0","end":"3"},{"start":"2","end":"4"}]),
        json!([{"start":"-0","end":"1"}]),
    ] {
        let doc = json!({"kind":"calendar","version":"0.1.0","id":"urn:test:calendar","clock":{"id":c.id,"revision":c.revision},"zone":"Etc/Test","zoneRules":{"id":z.id,"revision":z.revision},"intervals":intervals});
        let bytes = serde_json::to_vec(&doc).unwrap();
        let p = pin("urn:test:calendar", &bytes);
        let supplied = [
            Resource {
                id: &p.id,
                revision: &p.revision,
                bytes: &bytes,
            },
            Resource {
                id: &c.id,
                revision: &c.revision,
                bytes: clock,
            },
            Resource {
                id: &z.id,
                revision: &z.revision,
                bytes: zone,
            },
        ];
        let mut r = Resources::new(&supplied).unwrap();
        let result = r.calendar(&p);
        if intervals.as_array().unwrap().is_empty() || intervals[0]["start"] == "-1" {
            let cal = result.unwrap();
            assert_eq!(cal.clock(), &c);
            assert_eq!(cal.zone_rules(), &z);
        } else {
            assert!(result.is_err());
        }
        let mut r = Resources::new(&supplied[..2]).unwrap();
        assert_eq!(r.calendar(&p).unwrap_err().code, "resource-missing");
    }
}

#[test]
fn resource_pins_are_unique_exact_bounded_and_do_not_trust_json_id_alone() {
    let doc = provider();
    let bytes = serde_json::to_vec(&doc).unwrap();
    let p = pin("urn:test:provider", &bytes);
    let supplied = [
        Resource {
            id: &p.id,
            revision: &p.revision,
            bytes: &bytes,
        },
        Resource {
            id: &p.id,
            revision: &p.revision,
            bytes: &bytes,
        },
    ];
    assert!(Resources::new(&supplied).is_err());
    assert_eq!(
        Resources::new(&[]).unwrap().provider(&p).unwrap_err().code,
        "resource-missing"
    );
    let corrupt = [Resource {
        id: &p.id,
        revision: &p.revision,
        bytes: b"{}",
    }];
    assert_eq!(
        Resources::new(&corrupt)
            .unwrap()
            .provider(&p)
            .unwrap_err()
            .code,
        "resource-digest"
    );
    let wrong = Pin {
        id: "urn:test:other".into(),
        revision: p.revision.clone(),
    };
    let supplied = [Resource {
        id: &wrong.id,
        revision: &wrong.revision,
        bytes: &bytes,
    }];
    assert_eq!(
        Resources::new(&supplied)
            .unwrap()
            .provider(&wrong)
            .unwrap_err()
            .code,
        "resource-format"
    );
    let oversized = vec![b' '; 1024 * 1024 + 1];
    let supplied = [Resource {
        id: "x",
        revision: "x",
        bytes: &oversized,
    }];
    assert!(Resources::new(&supplied).is_err());
}

#[test]
fn type_value_ast_and_registry_budgets_fail_closed() {
    let mut r = Resources::new(&[]).unwrap();
    let mut fields = serde_json::Map::new();
    for n in 0..10_001 {
        fields.insert(format!("x{n}"), i(0));
    }
    assert!(
        r.expression(&json!({"op":"record","fields":fields}), &Context::default())
            .is_err()
    );
    assert!(
        r.literal(
            &Type::List(Box::new(Type::Integer)),
            &json!(vec!["0"; 10_001])
        )
        .is_err()
    );
    let mut ast = i(0);
    for _ in 0..65 {
        ast = json!({"op":"some","value":ast});
    }
    assert!(r.expression(&ast, &Context::default()).is_err());
}

#[test]
fn nominal_revisions_are_distinct_and_nested_wrap_invariants_are_enforced() {
    let base = named(
        ty("integer"),
        json!({"op":"not","value":bin("lt",param(),i(0))}),
    );
    let bytes = serde_json::to_vec(&base).unwrap();
    let p = pin("urn:test:numbers", &bytes);
    let mut reformatted = bytes.clone();
    reformatted.push(b'\n');
    let alternate = pin(&p.id, &reformatted);
    let mut outer = named(
        ty("integer"),
        bin(
            "eq",
            json!({"op":"unwrapNamed","value":{"op":"wrap","type":named_type(&p),"value":param()}}),
            param(),
        ),
    );
    outer["id"] = "urn:test:outer".into();
    let outer_bytes = serde_json::to_vec(&outer).unwrap();
    let o = pin("urn:test:outer", &outer_bytes);
    let supplied = [
        Resource {
            id: &p.id,
            revision: &p.revision,
            bytes: &bytes,
        },
        Resource {
            id: &alternate.id,
            revision: &alternate.revision,
            bytes: &reformatted,
        },
        Resource {
            id: &o.id,
            revision: &o.revision,
            bytes: &outer_bytes,
        },
    ];
    let mut r = Resources::new(&supplied).unwrap();
    let exact = r.type_descriptor(&named_type(&p)).unwrap();
    let other = r.type_descriptor(&named_type(&alternate)).unwrap();
    assert_ne!(exact, other);
    let outer = r.type_descriptor(&named_type(&o)).unwrap();
    r.literal(&outer, &json!("2")).unwrap();
    assert_eq!(
        r.literal(&outer, &json!("-1")).unwrap_err().code,
        "invalid-nominal-value"
    );
}

#[test]
fn artifact_byte_mutations_refuse_or_return_without_panicking() {
    let bytes = serde_json::to_vec(&provider()).unwrap();
    for n in 0..bytes.len() {
        let mut changed = bytes.clone();
        changed[n] = b'!';
        let p = pin("urn:test:provider", &changed);
        let supplied = [Resource {
            id: &p.id,
            revision: &p.revision,
            bytes: &changed,
        }];
        assert!(
            std::panic::catch_unwind(|| {
                let mut r = Resources::new(&supplied).unwrap();
                let _ = r.provider(&p);
            })
            .is_ok(),
            "byte {n}"
        );
    }
}
