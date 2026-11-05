// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use choreoform_ir_probe_core::{Resource, digest};
use choreoform_semantic_check::{CLOCK_PROFILE, Pin, Resources, Type};
use serde_json::{Value, json};

const PROFILE: &[u8] = include_bytes!(
    "../../../docs/ir/contracts/sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt"
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
