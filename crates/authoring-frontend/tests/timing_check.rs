// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use choreoform_authoring_frontend::Package;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource, digest};
use choreoform_semantic_check::{
    CALENDAR_PROFILE, CLOCK_PROFILE, CORE, Pin, SEMANTICS, check, check_control, check_protection,
    check_timing,
};
use serde_json::{Value, json};

#[path = "timing/retry.rs"]
mod retry;

const PROFILE: &[u8] = include_bytes!(
    "../../../docs/ir/contracts/sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt"
);
const PROVIDER_ID: &str = "urn:choreoform:examples:review-provider:0.1.0";
const CLOCK_ID: &str = "urn:test:clock";
const SOURCE_ID: &str = "urn:test:clock-source";
struct Fixture {
    b: Value,
    extra: Vec<(String, String, Vec<u8>)>,
    clock: Pin,
    prefix: &'static str,
}
impl Fixture {
    fn new(service: bool) -> Self {
        let mut f = Self {
            b: Value::Null,
            extra: Vec::new(),
            clock: Pin {
                id: CLOCK_ID.into(),
                revision: String::new(),
            },
            prefix: if service { "service" } else { "review" },
        };
        if service {
            f.artifact(
                PROVIDER_ID,
                include_bytes!("../../../docs/authoring/examples/service.provider.json").to_vec(),
            );
            f.artifact(
                "urn:choreoform:examples:number-types:0.1.0",
                include_bytes!("../../../docs/authoring/examples/service.named-types.json")
                    .to_vec(),
            );
        }
        let (source, ledger): (&[u8], &[u8]) = if service {
            (
                include_bytes!("../../../docs/authoring/examples/service.choreo"),
                include_bytes!("../../../docs/authoring/examples/service.bindings.json"),
            )
        } else {
            (
                include_bytes!("../../../docs/authoring/examples/review.choreo"),
                include_bytes!("../../../docs/authoring/examples/review.bindings.json"),
            )
        };
        f.b = Package::parse(source, ledger)
            .unwrap()
            .lower(&f.resources())
            .unwrap()
            .into_document()["body"]
            .clone();
        f.extra.push((
            CLOCK_PROFILE.0.into(),
            CLOCK_PROFILE.1.into(),
            PROFILE.to_vec(),
        ));
        let protection = f.b["data"][format!("{}_request", f.prefix)]["protection"].clone();
        let source = json!({"kind":"provider-contract","version":"0.1.0","id":SOURCE_ID,"input":{"kind":"record","fields":{}},"output":{"kind":"integer"},"outcomes":{"tick":true},"resultProtection":protection,"observations":{"tick":{"state":"succeeded","result":{"kind":"integer"},"outcome":"tick"},"unknown":{"state":"unknown","result":null,"outcome":null}},"idempotency":"none","effectClass":"read-only"});
        let source_pin = f.artifact(SOURCE_ID, serde_json::to_vec(&source).unwrap());
        let clock = json!({"kind":"clock","version":"0.1.0","id":CLOCK_ID,"profile":{"id":CLOCK_PROFILE.0,"revision":CLOCK_PROFILE.1},"source":{"id":source_pin.id,"revision":source_pin.revision}});
        f.clock = f.artifact(CLOCK_ID, serde_json::to_vec(&clock).unwrap());
        f
    }
    fn artifact(&mut self, id: &str, bytes: Vec<u8>) -> Pin {
        let pin = Pin {
            id: id.into(),
            revision: digest(&bytes),
        };
        self.extra.retain(|(key, _, _)| key != id);
        self.extra
            .push((pin.id.clone(), pin.revision.clone(), bytes));
        pin
    }
    fn resources(&self) -> Vec<Resource<'_>> {
        let mut resources = vec![
            Resource {
                id: SEMANTICS.0,
                revision: SEMANTICS.1,
                bytes: include_bytes!(
                    "../../../docs/ir/contracts/sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt"
                ),
            },
            Resource {
                id: CORE.0,
                revision: CORE.1,
                bytes: include_bytes!(
                    "../../../docs/ir/contracts/sha256-26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58.txt"
                ),
            },
        ];
        resources.extend(self.extra.iter().map(|(id, revision, bytes)| Resource {
            id,
            revision,
            bytes,
        }));
        resources
    }
    fn document(&self, id: &str) -> Value {
        serde_json::from_slice(&self.extra.iter().find(|(key, _, _)| key == id).unwrap().2).unwrap()
    }
    fn envelope(&self) -> DefinitionEnvelope {
        DefinitionEnvelope::from_parts(self.b.clone(), json!({})).unwrap()
    }
    fn timer(&self) -> Value {
        json!({"clock":{"id":self.clock.id,"revision":self.clock.revision},"basis":format!("{}_audit",self.prefix),"due":{"op":"literal","type":{"kind":"integer"},"value":"10"},"calendar":null,"pauseAuthority":format!("{}_actor",self.prefix)})
    }
    fn work_timer(&mut self) {
        self.b["policies"][format!("{}_work", self.prefix)]["body"]["timeout"] = self.timer();
        self.b["policies"][format!("{}_work", self.prefix)]["body"]["reads"] =
            json!({format!("{}_audit",self.prefix):true});
    }
    fn wait(&mut self) {
        let root = format!("scope_{}", self.prefix);
        let basis = format!("{}_audit", self.prefix);
        self.b["scopes"][&root]["outputs"] = json!({});
        self.b["scopes"][&root]["entry"] = "timed_wait".into();
        self.b["policies"]["timed_wait_policy"] = json!({"scope":root,"dialect":"core","body":{"kind":"wait","reads":{basis.clone():true},"observationType":{"kind":"record","fields":{}},"match":{"op":"literal","type":{"kind":"boolean"},"value":true},"authority":format!("{}_actor",self.prefix),"timer":self.timer(),"onObservation":"observed","onTimer":"expired"}});
        self.b["nodes"]["timed_wait"] = json!({"kind":"wait","scope":root,"policy":"timed_wait_policy","reads":{basis.clone():true},"writes":{},"outcomes":{"observed":true,"expired":true}});
        self.b["data"][basis]["invalidates"]["timed_wait"] = true.into();
        for outcome in ["observed", "expired"] {
            self.b["flows"][format!("wait_{outcome}")] = json!({"source":"timed_wait","outcome":outcome,"target":format!("{}_done",self.prefix)});
        }
    }
    fn clock_private(&mut self) {
        let mut source = self.document(SOURCE_ID);
        source["resultProtection"]["purposes"] = json!({"private":true});
        self.clock_source(source);
    }
    fn clock_source(&mut self, source: Value) {
        let source_pin = self.artifact(SOURCE_ID, serde_json::to_vec(&source).unwrap());
        let mut clock = self.document(CLOCK_ID);
        clock["source"]["revision"] = source_pin.revision.into();
        self.clock = self.artifact(CLOCK_ID, serde_json::to_vec(&clock).unwrap());
        for p in self.b["policies"].as_object_mut().unwrap().values_mut() {
            for field in ["timeout", "timer"] {
                if p["body"][field].is_object() {
                    p["body"][field]["clock"] =
                        json!({"id":self.clock.id,"revision":self.clock.revision});
                }
            }
        }
    }
    fn unit_service(&mut self, private: bool) {
        let mut provider = self.document(PROVIDER_ID);
        provider["output"] = json!({"kind":"record","fields":{}});
        for observation in provider["observations"]
            .as_object_mut()
            .unwrap()
            .values_mut()
        {
            if !observation["result"].is_null() {
                observation["result"] = json!({"kind":"record","fields":{}});
            }
        }
        if private {
            provider["resultProtection"]["purposes"] = json!({"private":true});
        }
        let pin = self.artifact(PROVIDER_ID, serde_json::to_vec(&provider).unwrap());
        let contract = self.b["capabilities"]["service_capability"]["contract"]
            .as_str()
            .unwrap()
            .to_owned();
        self.b["policies"][contract]["body"]["provider"]["revision"] = pin.revision.into();
        self.b["capabilities"]["service_capability"]["output"]["body"] =
            json!({"kind":"record","fields":{}});
        self.b["nodes"]["service_step"]["result"] = Value::Null;
        self.b["nodes"]["service_step"]["writes"] = json!({});
        self.b["scopes"]["scope_service"]["outputs"] = json!({});
    }
    fn public_after(&mut self, site: &str) {
        let target = format!("{}_request", self.prefix);
        self.b["nodes"]["publish_public"] = json!({"kind":"compute","scope":format!("scope_{}",self.prefix),"assignments":{target.clone():format!("{}_initial_request",self.prefix)},"reads":{},"writes":{target:true},"outcomes":{"next":true}});
        for flow in self.b["flows"].as_object_mut().unwrap().values_mut() {
            if flow["source"] == site {
                flow["target"] = "publish_public".into();
            }
        }
        self.b["flows"]["public_done"] = json!({"source":"publish_public","outcome":"next","target":format!("{}_done",self.prefix)});
    }
    fn relative_calendar(&mut self, clock: &Pin, intervals: Value) -> Pin {
        let profile = self.artifact(CALENDAR_PROFILE.0, include_bytes!("../../../docs/ir/contracts/sha256-2ab8a6a78f2829ababa2d023f6743ac82d7a31acbfca1fb7a03340b7c929f35f.txt").to_vec());
        assert_eq!(profile.revision, CALENDAR_PROFILE.1);
        let rules = json!({"kind":"calendar-rules","version":"0.1.0","id":"urn:test:rules","profile":{"id":profile.id,"revision":profile.revision},"clock":{"id":clock.id,"revision":clock.revision}});
        let rules = self.artifact("urn:test:rules", serde_json::to_vec(&rules).unwrap());
        let calendar = json!({"kind":"calendar","version":"0.1.0","id":"urn:test:calendar","clock":{"id":clock.id,"revision":clock.revision},"zone":"instance-relative","zoneRules":{"id":rules.id,"revision":rules.revision},"intervals":intervals});
        self.artifact("urn:test:calendar", serde_json::to_vec(&calendar).unwrap())
    }
}

#[test]
fn timing_reports_exact_work_and_wait_uses_without_runtime_facts() {
    for wait in [false, true] {
        let mut f = Fixture::new(false);
        if wait {
            f.wait();
        } else {
            f.work_timer();
        }
        let e = f.envelope();
        let r = f.resources();
        let report = check_timing(&e, &r).unwrap();
        assert_eq!(report.revision(), e.document()["revision"]);
        assert_eq!(report.timer_uses().len(), 1);
        let use_site = report.timer_uses().first().unwrap();
        assert_eq!(use_site.clock(), &f.clock);
        assert_eq!(use_site.basis(), "review_audit");
        assert_eq!(use_site.pause_authority(), "review_actor");
        assert_eq!(use_site.source().id, SOURCE_ID);
        assert_eq!(
            use_site.site(),
            if wait { "timed_wait" } else { "review_step" }
        );
        assert_eq!(
            use_site.policy(),
            if wait {
                "timed_wait_policy"
            } else {
                "review_work"
            }
        );
        assert_eq!(use_site.role(), if wait { "timer" } else { "timeout" });
        assert!(
            use_site
                .host_requirements()
                .contains(&"authenticated-source-instance-domain-and-origin-binding")
        );
        assert!(report.uncovered_nodes().is_empty());
    }
}

#[test]
fn private_clock_is_checked_before_a_timed_work_result_write() {
    let mut f = Fixture::new(false);
    f.work_timer();
    f.clock_private();
    let e = f.envelope();
    let r = f.resources();
    check_control(&e, &r).unwrap();
    let error = check_protection(&e, &r).unwrap_err();
    assert_eq!(error.code, "control-information-flow");
    assert_eq!(error.path, "/body/nodes/review_step");
}

#[test]
fn clock_and_activity_origins_coexist_without_result_publication() {
    for private_clock in [false, true] {
        let mut f = Fixture::new(true);
        f.work_timer();
        f.unit_service(!private_clock);
        if private_clock {
            f.clock_private();
        }
        f.public_after("service_step");
        let e = f.envelope();
        let r = f.resources();
        check_control(&e, &r).unwrap();
        let error = check_protection(&e, &r).unwrap_err();
        assert_eq!(error.code, "control-information-flow");
        assert_eq!(error.path, "/body/nodes/publish_public");
    }
}

#[test]
fn timed_wait_keeps_clock_origin_for_both_ordinary_outcomes() {
    let mut f = Fixture::new(false);
    f.wait();
    f.clock_private();
    f.public_after("timed_wait");
    let e = f.envelope();
    let r = f.resources();
    check_control(&e, &r).unwrap();
    let error = check_protection(&e, &r).unwrap_err();
    assert_eq!(error.code, "control-information-flow");
    assert_eq!(error.path, "/body/nodes/publish_public");
}

#[test]
fn timer_fields_due_facts_and_basis_reads_are_checked() {
    for (mutation, code) in [
        ("extra", "fields"),
        ("due", "timer-due-type"),
        ("fact", "policy-fact"),
        ("policy-read", "policy-reads"),
        ("node-read", "node-reads"),
    ] {
        let mut f = Fixture::new(false);
        f.work_timer();
        match mutation {
            "extra" => f.b["policies"]["review_work"]["body"]["timeout"]["safe"] = true.into(),
            "due" => {
                f.b["policies"]["review_work"]["body"]["timeout"]["due"] =
                    json!({"op":"literal","type":{"kind":"text"},"value":"SECRET-MUST-NOT-APPEAR"})
            }
            "fact" => {
                f.b["policies"]["review_work"]["body"]["timeout"]["due"] =
                    json!({"op":"fact","name":"principal"})
            }
            "policy-read" => f.b["policies"]["review_work"]["body"]["reads"] = json!({}),
            _ => {
                f.b["nodes"]["review_step"]["reads"]
                    .as_object_mut()
                    .unwrap()
                    .remove("review_audit");
            }
        }
        let error = check(&f.envelope(), &f.resources()).unwrap_err();
        assert_eq!(error.code, code, "{mutation}: {error:?}");
        assert!(!format!("{error:?}").contains("SECRET-MUST-NOT-APPEAR"));
    }
}

#[test]
fn timed_wait_outcome_presence_distinctness_and_exact_sets_are_checked() {
    for mutation in ["missing", "same", "node", "orphan"] {
        let mut f = Fixture::new(false);
        f.wait();
        match mutation {
            "missing" => f.b["policies"]["timed_wait_policy"]["body"]["onTimer"] = Value::Null,
            "same" => f.b["policies"]["timed_wait_policy"]["body"]["onTimer"] = "observed".into(),
            "node" => {
                f.b["nodes"]["timed_wait"]["outcomes"]
                    .as_object_mut()
                    .unwrap()
                    .remove("expired");
                f.b["flows"].as_object_mut().unwrap().remove("wait_expired");
            }
            _ => f.b["policies"]["timed_wait_policy"]["body"]["timer"] = Value::Null,
        }
        let error = check(&f.envelope(), &f.resources()).unwrap_err();
        assert_eq!(
            error.code,
            if mutation == "node" {
                "wait-outcomes"
            } else {
                "wait-timer-outcome"
            }
        );
    }
}

#[test]
fn timing_refuses_unbound_retry_clocks_after_earlier_stages_pass() {
    for timeout in [false, true] {
        let mut f = Fixture::new(false);
        if timeout {
            f.work_timer();
        }
        f.b["policies"]["review_work"]["body"]["retry"] =
            json!({"maxAttempts":2,"delays":["0"],"faults":{"transient":true}});
        let e = f.envelope();
        let r = f.resources();
        choreoform_semantic_check::check_concurrency(&e, &r).unwrap();
        let error = check_timing(&e, &r).unwrap_err();
        assert_eq!(error.code, "retry-clock-basis-unproved");
        assert!(e.document().pointer(&error.path).is_some());
    }
}

#[test]
fn timing_reports_unvisited_timer_sites_without_inventing_clock_facts() {
    for wait in [false, true] {
        let mut f = Fixture::new(false);
        if wait {
            f.wait();
        } else {
            f.b["scopes"]["scope_review"]["outputs"] = json!({});
            f.b["policies"]["review_work"]["body"]["retry"] =
                json!({"maxAttempts":2,"delays":["0"],"faults":{}});
        }
        f.b["scopes"]["scope_review"]["entry"] = "review_done".into();
        let report = check_timing(&f.envelope(), &f.resources()).unwrap();
        assert!(report.timer_uses().is_empty());
        assert!(report.checked_nodes().is_empty());
        assert!(
            report
                .uncovered_nodes()
                .contains(if wait { "timed_wait" } else { "review_step" })
        );
    }
}

#[test]
fn timer_closes_clock_source_participant_requirement_dependencies() {
    let mut f = Fixture::new(false);
    f.work_timer();
    f.b["actors"]["clock_actor"] =
        json!({"scope":"scope_review","requirement":"clock_requirement"});
    f.b["policies"]["clock_requirement"] = json!({"scope":"scope_review","dialect":"core","body":{"kind":"actor","reads":{"review_request":true},"permit":{"op":"eq","left":{"op":"read","cell":"review_request"},"right":{"op":"literal","type":{"kind":"text"},"value":"Synthetic request"}}}});
    let mut source = f.document(SOURCE_ID);
    source["resultProtection"]["participants"]["clock_actor"] = true.into();
    f.clock_source(source);
    f.b["policies"]["review_work"]["body"]["reads"] =
        json!({"review_request":true,"review_audit":true});
    check_timing(&f.envelope(), &f.resources()).unwrap();
    f.b["policies"]["review_work"]["body"]["reads"] = json!({"review_audit":true});
    assert_eq!(
        check(&f.envelope(), &f.resources()).unwrap_err().code,
        "policy-reads"
    );
}

#[test]
fn timer_calendars_need_supported_construction_rules_even_with_valid_finite_tables() {
    for matching in [true, false] {
        let mut f = Fixture::new(false);
        f.work_timer();
        let clock = if matching {
            f.clock.clone()
        } else {
            let mut document = f.document(CLOCK_ID);
            document["id"] = "urn:test:other-clock".into();
            f.artifact(
                "urn:test:other-clock",
                serde_json::to_vec(&document).unwrap(),
            )
        };
        let zone = f.artifact(
            "urn:test:zone",
            b"immutable precomputed zone provenance".to_vec(),
        );
        let calendar = json!({"kind":"calendar","version":"0.1.0","id":"urn:test:calendar","clock":{"id":clock.id,"revision":clock.revision},"zone":"Test/Zone","zoneRules":{"id":zone.id,"revision":zone.revision},"intervals":[{"start":"0","end":"100"}]});
        let calendar = f.artifact("urn:test:calendar", serde_json::to_vec(&calendar).unwrap());
        f.b["policies"]["review_work"]["body"]["timeout"]["calendar"] =
            json!({"id":calendar.id,"revision":calendar.revision});
        let e = f.envelope();
        let error = check(&e, &f.resources()).unwrap_err();
        assert_eq!(error.code, "calendar-zone-profile");
        assert!(e.document().pointer(&error.path).is_some());
    }
}

#[test]
fn source_work_timer_derives_basis_and_clock_source_reads_after_reparse() {
    let mut f = Fixture::new(false);
    let mut clock_source = f.document(SOURCE_ID);
    clock_source["resultProtection"]["participants"]["clock_actor"] = true.into();
    f.clock_source(clock_source);
    let source = String::from_utf8(include_bytes!("../../../docs/authoring/examples/review.choreo").to_vec()).unwrap()
        .replace("Timeout none.", "Timeout clock \"Elapsed clock\", basis \"Audit basis\", due (10), calendar none, pause authority \"Reviewer\".")
        .replace("    Also read [\"Audit basis\"].\n", "")
        .replace("End process.", "Require actor \"Clock observer\" under \"Clock observer requirement\". Rule for actor \"Clock observer requirement\". Permit when ((the value of \"Request\") equals (\"Synthetic request\")). End policy. End process.");
    let mut ledger: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/authoring/examples/review.bindings.json"
    ))
    .unwrap();
    ledger["declarations"].as_array_mut().unwrap().push(json!({"id":"clock_binding","kind":"resource","scope":"scope_review","name":"Elapsed clock"}));
    ledger["declarations"].as_array_mut().unwrap().extend([
        json!({"id":"clock_actor","kind":"actor","scope":"scope_review","name":"Clock observer"}),
        json!({"id":"clock_requirement","kind":"policy","scope":"scope_review","name":"Clock observer requirement"}),
    ]);
    ledger["resources"].as_array_mut().unwrap().push(json!({"binding":"clock_binding","kind":"clock","contract":f.clock.id,"digest":f.clock.revision}));
    let ledger = serde_json::to_vec(&ledger).unwrap();
    let resources = f.resources();
    let package = Package::parse(source.as_bytes(), &ledger).unwrap();
    let candidate = package.lower(&resources).unwrap();
    assert_eq!(
        candidate.document()["body"]["policies"]["review_work"]["body"]["reads"],
        json!({"review_audit":true,"review_request":true})
    );
    assert_eq!(
        candidate.document()["body"]["nodes"]["review_step"]["reads"],
        json!({"review_audit":true,"review_request":true})
    );
    let report = candidate.check_timing(&resources).unwrap();
    assert_eq!(report.timer_uses().len(), 1);
    let (formatted, companion) = package.formatted().unwrap();
    let roundtrip = Package::parse(formatted.as_bytes(), companion.as_bytes())
        .unwrap()
        .lower(&resources)
        .unwrap();
    assert_eq!(roundtrip.document(), candidate.document());
    assert_eq!(
        roundtrip.check_timing(&resources).unwrap().timer_uses(),
        report.timer_uses()
    );
}

#[test]
fn repeated_timer_policies_reuse_validated_source_metadata_with_bounded_copies() {
    let mut f = Fixture::new(false);
    f.work_timer();
    let mut source = f.document(SOURCE_ID);
    let purposes = source["resultProtection"]["purposes"]
        .as_object_mut()
        .unwrap();
    for i in 0..4000 {
        purposes.insert(format!("purpose_{i:04}"), true.into());
    }
    f.clock_source(source);
    for i in 0..700 {
        f.b["policies"][format!("unused_timer_{i:04}")] = f.b["policies"]["review_work"].clone();
    }
    let report = check_timing(&f.envelope(), &f.resources()).unwrap();
    assert_eq!(report.timer_uses().len(), 1);
}

#[test]
fn source_reference_cache_never_reuses_a_child_visibility_proof_in_its_parent() {
    let mut f = Fixture::new(false);
    f.work_timer();
    let mut child = f.b["scopes"]["scope_review"].clone();
    child["parent"] = "scope_review".into();
    child["entry"] = "child_done".into();
    child["inputs"] = json!({});
    child["outputs"] = json!({});
    f.b["scopes"]["child"] = child;
    f.b["nodes"]["child_done"] = json!({"kind":"finish","scope":"child","outcome":"complete","reads":{},"writes":{},"outcomes":{}});
    f.b["actors"]["child_clock_actor"] = json!({"scope":"child","requirement":"child_requirement"});
    f.b["policies"]["child_requirement"] = json!({"scope":"child","dialect":"core","body":{"kind":"actor","reads":{},"permit":{"op":"literal","type":{"kind":"boolean"},"value":true}}});
    let mut source = f.document(SOURCE_ID);
    source["resultProtection"]["participants"]["child_clock_actor"] = true.into();
    f.clock_source(source);
    f.b["policies"]["aaa_child_timer"] = f.b["policies"]["review_work"].clone();
    f.b["policies"]["aaa_child_timer"]["scope"] = "child".into();
    let error = check(&f.envelope(), &f.resources()).unwrap_err();
    assert_eq!(error.code, "visibility");
    assert_eq!(error.path, "/body/policies/review_work");
}

#[test]
fn calendar_timing_uses_expose_exact_construction_pins_and_required_live_binding() {
    for wait in [false, true] {
        let mut f = Fixture::new(false);
        if wait {
            f.wait();
        } else {
            f.work_timer();
        }
        let calendar = f.relative_calendar(&f.clock.clone(), json!([{"start":"0","end":"100"}]));
        let policy = if wait {
            "timed_wait_policy"
        } else {
            "review_work"
        };
        let role = if wait { "timer" } else { "timeout" };
        f.b["policies"][policy]["body"][role]["calendar"] =
            json!({"id":calendar.id,"revision":calendar.revision});
        let report = check_timing(&f.envelope(), &f.resources()).unwrap();
        let use_site = report.timer_uses().first().unwrap().calendar().unwrap();
        assert_eq!(use_site.pin(), &calendar);
        assert_eq!(use_site.rules().id, "urn:test:rules");
        assert_eq!(use_site.profile().revision, CALENDAR_PROFILE.1);
        assert!(
            use_site
                .host_requirements()
                .contains(&"trusted-host-calendar-construction-approval")
        );
        assert!(
            use_site
                .host_requirements()
                .contains(&"exact-calendar-rules-profile-clock-and-instance-origin-binding")
        );
    }
}

#[test]
fn known_calendar_with_another_clock_cannot_supply_a_timer_domain() {
    let mut f = Fixture::new(false);
    f.work_timer();
    let mut clock = f.document(CLOCK_ID);
    clock["id"] = "urn:test:other-clock".into();
    let clock = f.artifact("urn:test:other-clock", serde_json::to_vec(&clock).unwrap());
    let calendar = f.relative_calendar(&clock, json!([{"start":"0","end":"100"}]));
    f.b["policies"]["review_work"]["body"]["timeout"]["calendar"] =
        json!({"id":calendar.id,"revision":calendar.revision});
    let error = check(&f.envelope(), &f.resources()).unwrap_err();
    assert_eq!(error.code, "calendar-clock");
}

#[test]
fn initial_calendar_metadata_does_not_require_permission_to_pause_an_absolute_deadline() {
    let mut f = Fixture::new(false);
    f.work_timer();
    f.b["actors"]["no_pause"] = json!({"scope":"scope_review","requirement":"deny_pause"});
    f.b["policies"]["deny_pause"] = json!({"scope":"scope_review","dialect":"core","body":{"kind":"actor","reads":{},"permit":{"op":"literal","type":{"kind":"boolean"},"value":false}}});
    f.b["policies"]["review_work"]["body"]["timeout"]["pauseAuthority"] = "no_pause".into();
    let calendar = f.relative_calendar(&f.clock.clone(), json!([{"start":"0","end":"100"}]));
    f.b["policies"]["review_work"]["body"]["timeout"]["calendar"] =
        json!({"id":calendar.id,"revision":calendar.revision});
    let report = check_timing(&f.envelope(), &f.resources()).unwrap();
    let timer = report.timer_uses().first().unwrap();
    assert_eq!(timer.pause_authority(), "no_pause");
    assert!(
        timer
            .calendar()
            .unwrap()
            .host_requirements()
            .contains(&"trusted-host-calendar-construction-approval")
    );
}

#[test]
fn repeated_timer_calendars_share_a_wide_immutable_table_without_binding_it() {
    let mut f = Fixture::new(false);
    f.work_timer();
    let intervals = (0..10000)
        .map(|i| json!({"start":(i*2).to_string(),"end":(i*2+1).to_string()}))
        .collect::<Vec<_>>();
    let calendar = f.relative_calendar(&f.clock.clone(), intervals.into());
    f.b["policies"]["review_work"]["body"]["timeout"]["calendar"] =
        json!({"id":calendar.id,"revision":calendar.revision});
    for i in 0..700 {
        f.b["policies"][format!("unused_calendar_{i:04}")] = f.b["policies"]["review_work"].clone();
    }
    let report = check_timing(&f.envelope(), &f.resources()).unwrap();
    assert_eq!(report.timer_uses().len(), 1);
}
