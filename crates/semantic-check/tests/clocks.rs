// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use choreoform_ir_probe_core::{Resource, digest};
use choreoform_semantic_check::{CALENDAR_PROFILE, CLOCK_PROFILE, Pin, Resources, Type};
use serde_json::{Value, json};

const PROFILE: &[u8] = include_bytes!(
    "../../../docs/ir/contracts/sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt"
);
const CALENDAR: &[u8] = include_bytes!(
    "../../../docs/ir/contracts/sha256-2ab8a6a78f2829ababa2d023f6743ac82d7a31acbfca1fb7a03340b7c929f35f.txt"
);
fn provider() -> Value {
    json!({"kind":"provider-contract","version":"0.1.0","id":"urn:test:clock-source","input":{"kind":"record","fields":{}},"output":{"kind":"integer"},"outcomes":{"tick":true},"resultProtection":{"policy":"protect","sensitivity":"internal","purposes":{"process":true},"participants":{"reviewer":true},"capabilities":{}},"observations":{"tick":{"state":"succeeded","result":{"kind":"integer"},"outcome":"tick"},"unknown":{"state":"unknown","result":null,"outcome":null}},"idempotency":"none","effectClass":"read-only"})
}
fn clock(source: &Value) -> Value {
    let bytes = serde_json::to_vec(source).unwrap();
    json!({"kind":"clock","version":"0.1.0","id":"urn:test:clock","profile":{"id":CLOCK_PROFILE.0,"revision":CLOCK_PROFILE.1},"source":{"id":source["id"],"revision":digest(&bytes)}})
}
fn inspect(source: Value, document: Value, f: impl FnOnce(&mut Resources<'_>, &Pin)) {
    let source_bytes = serde_json::to_vec(&source).unwrap();
    let clock_bytes = serde_json::to_vec(&document).unwrap();
    let revision = digest(&clock_bytes);
    let source_revision = digest(&source_bytes);
    let pin = Pin {
        id: "urn:test:clock".into(),
        revision: revision.clone(),
    };
    let supplied = [
        Resource {
            id: &pin.id,
            revision: &revision,
            bytes: &clock_bytes,
        },
        Resource {
            id: CLOCK_PROFILE.0,
            revision: CLOCK_PROFILE.1,
            bytes: PROFILE,
        },
        Resource {
            id: source["id"].as_str().unwrap(),
            revision: &source_revision,
            bytes: &source_bytes,
        },
    ];
    f(&mut Resources::new(&supplied).unwrap(), &pin);
}

#[test]
fn clocks_bind_exact_profile_and_typed_source_without_timer_authority() {
    assert_eq!(digest(PROFILE), CLOCK_PROFILE.1);
    assert_eq!(
        PROFILE,
        include_bytes!("../../../docs/clocks/instance-microseconds-v0.1.md")
    );
    let source = provider();
    inspect(source.clone(), clock(&source), |r, pin| {
        let clock = r.clock(pin).unwrap();
        assert_eq!(clock.pin(), pin);
        assert_eq!(
            (
                clock.profile().id.as_str(),
                clock.profile().revision.as_str()
            ),
            CLOCK_PROFILE
        );
        assert_eq!(clock.source().id, "urn:test:clock-source");
        assert_eq!(
            clock.source_contract().input(),
            &Type::Record(Default::default())
        );
        assert_eq!(clock.source_contract().output(), &Type::Integer);
        assert_eq!(clock.source_contract().protection()["policy"], "protect");
        assert_eq!(r.clock(pin).unwrap().source(), clock.source());
    });
}

#[test]
fn clocks_refuse_unknown_profiles_shapes_and_safe_flags() {
    let source = provider();
    for mutation in [
        "kind",
        "version",
        "id",
        "profile-id",
        "profile-revision",
        "extra",
        "flag",
        "pin",
    ] {
        let mut doc = clock(&source);
        match mutation {
            "kind" => doc["kind"] = "machine-clock".into(),
            "version" => doc["version"] = "0.2.0".into(),
            "id" => doc["id"] = "urn:test:other".into(),
            "profile-id" => doc["profile"]["id"] = "urn:test:wall-clock".into(),
            "profile-revision" => {
                doc["profile"]["revision"] = format!("sha256:{}", "0".repeat(64)).into()
            }
            "extra" => doc["origin"] = "trust-me".into(),
            "flag" => doc["safe"] = true.into(),
            _ => doc["source"]["extra"] = true.into(),
        }
        inspect(source.clone(), doc, |r, pin| {
            assert!(r.clock(pin).is_err(), "{mutation}")
        });
    }
}

#[test]
fn clocks_require_read_only_integer_tick_contracts() {
    for mutation in [
        "input",
        "output",
        "outcome",
        "effect",
        "idempotency",
        "flag",
        "observation",
    ] {
        let mut source = provider();
        match mutation {
            "input" => source["input"] = json!({"kind":"text"}),
            "output" => {
                source["output"] = json!({"kind":"text"});
                source["observations"]["tick"]["result"] = json!({"kind":"text"});
            }
            "outcome" => {
                source["outcomes"] = json!({"other":true});
                source["observations"]["tick"]["outcome"] = "other".into();
            }
            "effect" => source["effectClass"] = "irreversible".into(),
            "idempotency" => source["idempotency"] = "stable-key".into(),
            "flag" => source["outcomes"]["tick"] = false.into(),
            _ => source["observations"]["unknown"]["result"] = json!({"kind":"integer"}),
        }
        inspect(source.clone(), clock(&source), |r, pin| {
            assert!(r.clock(pin).is_err(), "{mutation}")
        });
    }
}

#[test]
fn clocks_require_all_exact_bytes_and_reject_hashed_arbitrary_sources() {
    for mode in [
        "missing-clock",
        "missing-profile",
        "missing-source",
        "clock-digest",
        "profile-digest",
        "source-digest",
        "arbitrary-clock",
        "arbitrary-source",
    ] {
        let source = provider();
        let mut doc = clock(&source);
        let mut source_bytes = serde_json::to_vec(&source).unwrap();
        if mode == "arbitrary-source" {
            source_bytes =
                br#"{"kind":"other","version":"0.1.0","id":"urn:test:clock-source"}"#.to_vec();
            doc["source"]["revision"] = digest(&source_bytes).into();
        }
        let source_revision = doc["source"]["revision"].as_str().unwrap().to_owned();
        let mut bytes = serde_json::to_vec(&doc).unwrap();
        if mode == "arbitrary-clock" {
            bytes = b"arbitrary clock bytes".to_vec();
        }
        let revision = digest(&bytes);
        let pin = Pin {
            id: "urn:test:clock".into(),
            revision: revision.clone(),
        };
        let mut profile = PROFILE.to_vec();
        if mode == "clock-digest" {
            bytes.push(b' ');
        }
        if mode == "profile-digest" {
            profile.push(b' ');
        }
        if mode == "source-digest" {
            source_bytes.push(b' ');
        }
        let mut supplied = vec![
            Resource {
                id: &pin.id,
                revision: &revision,
                bytes: &bytes,
            },
            Resource {
                id: CLOCK_PROFILE.0,
                revision: CLOCK_PROFILE.1,
                bytes: &profile,
            },
            Resource {
                id: "urn:test:clock-source",
                revision: &source_revision,
                bytes: &source_bytes,
            },
        ];
        if mode == "missing-clock" {
            supplied.remove(0);
        }
        if mode == "missing-profile" {
            supplied.remove(1);
        }
        if mode == "missing-source" {
            supplied.remove(2);
        }
        let error = Resources::new(&supplied).unwrap().clock(&pin).unwrap_err();
        let expected = if mode.starts_with("missing") {
            "resource-missing"
        } else if mode.ends_with("digest") {
            "resource-digest"
        } else if mode == "arbitrary-clock" {
            "resource-transport"
        } else {
            "resource-format"
        };
        assert_eq!(error.code, expected, "{mode}: {error:?}");
    }
}

type Artifacts = Vec<(String, String, Vec<u8>)>;
fn artifact(artifacts: &mut Artifacts, id: &str, bytes: Vec<u8>) -> Pin {
    let pin = Pin {
        id: id.into(),
        revision: digest(&bytes),
    };
    artifacts.push((pin.id.clone(), pin.revision.clone(), bytes));
    pin
}
fn reference(pin: &Pin) -> Value {
    json!({"id":pin.id,"revision":pin.revision})
}
fn calendar_bundle(
    intervals: Value,
    mutate: impl FnOnce(&mut Value, &mut Value),
) -> (Artifacts, Pin) {
    let mut artifacts = vec![
        (
            CLOCK_PROFILE.0.into(),
            CLOCK_PROFILE.1.into(),
            PROFILE.to_vec(),
        ),
        (
            CALENDAR_PROFILE.0.into(),
            CALENDAR_PROFILE.1.into(),
            CALENDAR.to_vec(),
        ),
    ];
    let source = provider();
    artifact(
        &mut artifacts,
        "urn:test:clock-source",
        serde_json::to_vec(&source).unwrap(),
    );
    let clock = artifact(
        &mut artifacts,
        "urn:test:clock",
        serde_json::to_vec(&clock(&source)).unwrap(),
    );
    let mut rules = json!({"kind":"calendar-rules","version":"0.1.0","id":"urn:test:rules","profile":{"id":CALENDAR_PROFILE.0,"revision":CALENDAR_PROFILE.1},"clock":reference(&clock)});
    let mut calendar = json!({"kind":"calendar","version":"0.1.0","id":"urn:test:calendar","clock":reference(&clock),"zone":"instance-relative","zoneRules":null,"intervals":intervals});
    mutate(&mut rules, &mut calendar);
    let rules = artifact(
        &mut artifacts,
        "urn:test:rules",
        serde_json::to_vec(&rules).unwrap(),
    );
    calendar["zoneRules"] = reference(&rules);
    let calendar = artifact(
        &mut artifacts,
        "urn:test:calendar",
        serde_json::to_vec(&calendar).unwrap(),
    );
    (artifacts, calendar)
}
fn registry(artifacts: &Artifacts) -> Vec<Resource<'_>> {
    artifacts
        .iter()
        .map(|(id, revision, bytes)| Resource {
            id,
            revision,
            bytes,
        })
        .collect()
}

#[test]
fn instance_calendars_bind_exact_construction_and_test_half_open_coordinates_only() {
    assert_eq!(digest(CALENDAR), CALENDAR_PROFILE.1);
    assert_eq!(
        CALENDAR,
        include_bytes!("../../../docs/clocks/instance-calendar-v0.1.md")
    );
    let (artifacts, pin) = calendar_bundle(
        json!([{"start":i64::MIN.to_string(),"end":"-1"},{"start":"0","end":"2"},{"start":"2","end":"5"},{"start":(i64::MAX-1).to_string(),"end":i64::MAX.to_string()}]),
        |_, _| {},
    );
    let supplied = registry(&artifacts);
    let mut resources = Resources::new(&supplied).unwrap();
    let calendar = resources.instance_calendar(&pin).unwrap();
    assert_eq!(calendar.pin(), &pin);
    assert_eq!(calendar.profile().id, CALENDAR_PROFILE.0);
    assert_eq!(calendar.clock().id, "urn:test:clock");
    assert_eq!(calendar.rules().id, "urn:test:rules");
    assert_eq!(calendar.intervals().len(), 4);
    for (coordinate, contains) in [
        (i64::MIN, true),
        (-2, true),
        (-1, false),
        (0, true),
        (1, true),
        (2, true),
        (4, true),
        (5, false),
        (6, false),
        (i64::MAX - 1, true),
        (i64::MAX, false),
    ] {
        assert_eq!(
            calendar.contains_coordinate(coordinate),
            contains,
            "{coordinate}"
        );
    }
    let (artifacts, pin) = calendar_bundle(json!([]), |_, _| {});
    let supplied = registry(&artifacts);
    let calendar = Resources::new(&supplied)
        .unwrap()
        .instance_calendar(&pin)
        .unwrap();
    assert!(!calendar.contains_coordinate(0));
    assert!(!calendar.contains_coordinate(i64::MAX));
}

#[test]
fn instance_calendars_refuse_unknown_rules_profiles_clocks_and_trust_flags() {
    for (mutation, expected) in [
        ("zone", "calendar-zone-profile"),
        ("profile-id", "calendar-profile-unsupported"),
        ("profile-revision", "calendar-profile-unsupported"),
        ("rule-clock", "calendar-rules-clock"),
        ("safe", "fields"),
        ("kind", "resource-format"),
        ("version", "resource-format"),
        ("interval-order", "calendar-intervals"),
        ("zero-width", "calendar-intervals"),
    ] {
        let (artifacts, pin) = calendar_bundle(
            json!([{"start":"0","end":"10"}]),
            |rules, calendar| match mutation {
                "zone" => calendar["zone"] = "Etc/Unknown".into(),
                "profile-id" => rules["profile"]["id"] = "urn:test:unimplemented".into(),
                "profile-revision" => {
                    rules["profile"]["revision"] = format!("sha256:{}", "0".repeat(64)).into()
                }
                "rule-clock" => rules["clock"]["id"] = "urn:test:other-clock".into(),
                "safe" => rules["safe"] = true.into(),
                "kind" => rules["kind"] = "uploaded-schedule-approval".into(),
                "version" => rules["version"] = "0.2.0".into(),
                "interval-order" => {
                    calendar["intervals"] = json!([{"start":"3","end":"5"},{"start":"0","end":"2"}])
                }
                _ => calendar["intervals"] = json!([{"start":"0","end":"0"}]),
            },
        );
        let supplied = registry(&artifacts);
        let error = Resources::new(&supplied)
            .unwrap()
            .instance_calendar(&pin)
            .unwrap_err();
        assert_eq!(error.code, expected, "{mutation}: {error:?}");
    }
}

#[test]
fn instance_calendars_require_exact_rule_and_profile_bytes_after_generic_metadata_reading() {
    for mutation in [
        "missing-rules",
        "missing-profile",
        "corrupt-rules",
        "corrupt-profile",
        "opaque-rules",
    ] {
        let (mut artifacts, mut pin) =
            calendar_bundle(json!([{"start":"0","end":"10"}]), |_, _| {});
        match mutation {
            "missing-rules" => artifacts.retain(|(id, _, _)| id != "urn:test:rules"),
            "missing-profile" => artifacts.retain(|(id, _, _)| id != CALENDAR_PROFILE.0),
            "corrupt-rules" => artifacts
                .iter_mut()
                .find(|(id, _, _)| id == "urn:test:rules")
                .unwrap()
                .2
                .push(b' '),
            "corrupt-profile" => artifacts
                .iter_mut()
                .find(|(id, _, _)| id == CALENDAR_PROFILE.0)
                .unwrap()
                .2
                .push(b' '),
            _ => {
                let rules = artifacts
                    .iter_mut()
                    .find(|(id, _, _)| id == "urn:test:rules")
                    .unwrap();
                rules.2 = b"opaque provenance bytes".to_vec();
                rules.1 = digest(&rules.2);
                let revision = rules.1.clone();
                let calendar = artifacts
                    .iter_mut()
                    .find(|(id, _, _)| id == "urn:test:calendar")
                    .unwrap();
                let mut value: Value = serde_json::from_slice(&calendar.2).unwrap();
                value["zoneRules"]["revision"] = revision.into();
                calendar.2 = serde_json::to_vec(&value).unwrap();
                calendar.1 = digest(&calendar.2);
                pin.revision = calendar.1.clone();
            }
        }
        let supplied = registry(&artifacts);
        let mut resources = Resources::new(&supplied).unwrap();
        if mutation == "opaque-rules" {
            resources.calendar(&pin).unwrap();
        }
        let error = resources.instance_calendar(&pin).unwrap_err();
        let expected = if mutation.starts_with("missing") {
            "resource-missing"
        } else if mutation.starts_with("corrupt") {
            "resource-digest"
        } else {
            "resource-transport"
        };
        assert_eq!(error.code, expected, "{mutation}: {error:?}");
    }
}

#[test]
fn instance_calendar_metadata_cache_shares_wide_tables_without_live_approval() {
    let intervals = (0..10000)
        .map(|i| json!({"start":(i*2).to_string(),"end":(i*2+1).to_string()}))
        .collect::<Vec<_>>();
    let (artifacts, pin) = calendar_bundle(intervals.into(), |_, _| {});
    let supplied = registry(&artifacts);
    let mut resources = Resources::new(&supplied).unwrap();
    let first = resources.instance_calendar(&pin).unwrap();
    for _ in 0..1000 {
        assert!(std::sync::Arc::ptr_eq(
            &first,
            &resources.instance_calendar(&pin).unwrap()
        ));
    }
    assert_eq!(first.intervals().len(), 10000);
    assert!(first.contains_coordinate(19998));
    assert!(!first.contains_coordinate(19999));
}
