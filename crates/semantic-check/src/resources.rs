// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_ir_probe_core::{Resource, digest, transport};
use std::sync::Arc;
mod clock;
pub use clock::{CLOCK_PROFILE, Clock};
mod instance_calendar;
pub use instance_calendar::{CALENDAR_PROFILE, InstanceCalendar};

#[derive(Debug)]
pub(crate) struct NamedDefinition {
    pub representation: Type,
    pub invariant: Value,
}
/// Decoded provider metadata only; no adapter authentication or dispatch authority.
#[derive(Debug)]
pub struct Provider {
    pub(crate) input: Type,
    pub(crate) output: Type,
    pub(crate) outcomes: BTreeSet<String>,
    pub(crate) protection: Value,
    pub(crate) effect: String,
    pub(crate) idempotency: String,
}
impl Provider {
    /// Exact declared input type, not an adapter-support attestation.
    pub fn input(&self) -> &Type {
        &self.input
    }
    pub fn output(&self) -> &Type {
        &self.output
    }
    pub fn outcomes(&self) -> &BTreeSet<String> {
        &self.outcomes
    }
    /// Definition-context references still require visibility and policy checking.
    pub fn protection(&self) -> &Value {
        &self.protection
    }
}

#[cfg(test)]
mod graph_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn synthetic_post_verification_nominal_graphs_refuse_cycles_and_depth() {
        // Exercise graph guards independently of cryptographic feasibility of a
        // content-hash cycle. Public registry verification is tested separately.
        for (count, cycle) in [(4, true), (66, false)] {
            let pin = Pin {
                id: "urn:test:graph".into(),
                revision: format!("sha256:{}", "0".repeat(64)),
            };
            let mut definitions = Map::new();
            for n in 0..count {
                let representation = if n + 1 == count && !cycle {
                    json!({"kind":"integer"})
                } else {
                    json!({"kind":"named","contract":{"id":pin.id,"revision":pin.revision},"name":format!("N{}",(n+1)%count)})
                };
                definitions.insert(format!("N{n}"),json!({"representation":representation,"invariant":{"op":"literal","type":{"kind":"boolean"},"value":true}}));
            }
            let mut r = Resources::new(&[]).unwrap();
            r.verified.insert(pin.clone());
            r.json.insert(pin.clone(),Arc::new(json!({"kind":"named-types","version":"0.1.0","id":pin.id,"definitions":definitions})));
            let code = r
                .nominal(
                    &Nominal {
                        contract: pin,
                        name: "N0".into(),
                    },
                    0,
                )
                .unwrap_err()
                .code;
            if cycle {
                assert_eq!(code, "nominal-cycle");
            } else {
                assert!(["budget", "nominal-depth"].contains(&code));
            }
            assert!(r.visiting.is_empty());
        }
    }
}
/// Finite table/provenance decoding, never clock support or schedule generation.
#[derive(Debug)]
pub struct Calendar {
    intervals: Vec<(i64, i64)>,
    clock: Pin,
    zone_rules: Pin,
}
impl Calendar {
    /// Mathematical membership only, without an observed/authenticated clock
    /// coordinate, live domain binding, due check or firing authority.
    pub fn contains_coordinate(&self, coordinate: i64) -> bool {
        let index = self
            .intervals
            .partition_point(|(_, end)| *end <= coordinate);
        self.intervals
            .get(index)
            .is_some_and(|(start, _)| *start <= coordinate)
    }
    pub fn intervals(&self) -> &[(i64, i64)] {
        &self.intervals
    }
    pub fn clock(&self) -> &Pin {
        &self.clock
    }
    pub fn zone_rules(&self) -> &Pin {
        &self.zone_rules
    }
}

/// Caller-supplied local bytes. No discovery, ambient trust or executable callbacks.
pub struct Resources<'a> {
    supplied: BTreeMap<Pin, &'a [u8]>,
    verified: BTreeSet<Pin>,
    json: BTreeMap<Pin, Arc<Value>>,
    named: BTreeMap<Nominal, Arc<NamedDefinition>>,
    visiting: BTreeSet<Nominal>,
    checking_named: bool,
    providers: BTreeMap<Pin, Arc<Provider>>,
    instance_calendars: BTreeMap<Pin, Arc<InstanceCalendar>>,
    pub(crate) budget: Budget,
}
impl<'a> Resources<'a> {
    /// Bound registry size before hashing/decoding; duplicate pins always refuse.
    pub fn new(supplied: &'a [Resource<'a>]) -> Result<Self> {
        require(supplied.len() <= 256, "resource-budget")?;
        let mut total = 0usize;
        let mut map = BTreeMap::new();
        for r in supplied {
            total = total
                .checked_add(r.bytes.len())
                .ok_or_else(|| err("resource-budget"))?;
            require(
                r.bytes.len() <= transport::MAX_BYTES && total <= 8 * transport::MAX_BYTES,
                "resource-budget",
            )?;
            let pin = Pin {
                id: r.id.into(),
                revision: r.revision.into(),
            };
            require(map.insert(pin, r.bytes).is_none(), "resource-ambiguous")?;
        }
        Ok(Self {
            supplied: map,
            verified: BTreeSet::new(),
            json: BTreeMap::new(),
            named: BTreeMap::new(),
            visiting: BTreeSet::new(),
            checking_named: false,
            providers: BTreeMap::new(),
            instance_calendars: BTreeMap::new(),
            budget: Budget::default(),
        })
    }
    /// Verify exact stored bytes, without claiming their format or host meaning.
    pub fn verify(&mut self, pin: &Pin) -> Result<()> {
        self.budget.tick(0)?;
        if !self.verified.contains(pin) {
            Pin::decode(&serde_json::json!({"id":pin.id,"revision":pin.revision}))?;
            let bytes = self
                .supplied
                .get(pin)
                .ok_or_else(|| err("resource-missing"))?;
            require(digest(bytes) == pin.revision, "resource-digest")?;
            self.verified.insert(pin.clone());
        }
        Ok(())
    }
    fn load(&mut self, pin: &Pin, kind: &str) -> Result<Arc<Value>> {
        self.verify(pin)?;
        if !self.json.contains_key(pin) {
            let value =
                transport::decode(self.supplied[pin]).map_err(|_| err("resource-transport"))?;
            self.json.insert(pin.clone(), Arc::new(value));
        }
        let v = self.json[pin].clone();
        require(
            v["kind"] == kind && v["version"] == "0.1.0" && v["id"] == pin.id,
            "resource-format",
        )?;
        Ok(v)
    }
    pub(crate) fn nominal(&mut self, key: &Nominal, depth: usize) -> Result<Arc<NamedDefinition>> {
        self.budget.tick(depth)?;
        if let Some(definition) = self.named.get(key) {
            return Ok(definition.clone());
        }
        require(self.visiting.len() < MAX_DEPTH, "nominal-depth")?;
        require(self.visiting.insert(key.clone()), "nominal-cycle")?;
        // Remove the active marker on failures too: no sticky, order-dependent errors.
        let result = (|| {
            let artifact = self.load(&key.contract, "named-types")?;
            fields(&artifact, &["kind", "version", "id", "definitions"])?;
            let defs = object(&artifact["definitions"])?;
            require(defs.len() <= MAX_ITEMS, "collection")?;
            let d = defs.get(&key.name).ok_or_else(|| err("nominal-missing"))?;
            fields(d, &["representation", "invariant"])?;
            let representation = self.ty(&d["representation"], depth + 1)?;
            let context = Context {
                parameters: BTreeMap::from([("value".into(), representation.clone())]),
                ..Context::default()
            };
            let judgment = self.expression(&d["invariant"], &context)?;
            require(
                judgment.ty == Type::Boolean && judgment.reads.is_empty(),
                "nominal-invariant",
            )?;
            Ok(Arc::new(NamedDefinition {
                representation,
                invariant: d["invariant"].clone(),
            }))
        })();
        self.visiting.remove(key);
        let definition = result?;
        self.named.insert(key.clone(), Arc::clone(&definition));
        Ok(definition)
    }
    /// Check every named declaration, including unused malformed definitions.
    pub fn named_types(&mut self, pin: &Pin) -> Result<()> {
        self.load(pin, "named-types")?;
        self.check_named_resources()
    }
    pub(crate) fn check_named_resources(&mut self) -> Result<()> {
        if self.checking_named || !self.visiting.is_empty() {
            return Ok(());
        }
        self.checking_named = true;
        let result = (|| {
            let mut done = BTreeSet::new();
            loop {
                let pending: Vec<_> = self
                    .json
                    .iter()
                    .filter(|(k, v)| v["kind"] == "named-types" && !done.contains(*k))
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                if pending.is_empty() {
                    break;
                }
                for (pin, v) in pending {
                    done.insert(pin.clone());
                    fields(&v, &["kind", "version", "id", "definitions"])?;
                    require(object(&v["definitions"])?.len() <= MAX_ITEMS, "collection")?;
                    for name in object(&v["definitions"])?.keys() {
                        super::name(&Value::from(name.as_str()))?;
                        self.nominal(
                            &Nominal {
                                contract: pin.clone(),
                                name: name.clone(),
                            },
                            0,
                        )?;
                    }
                }
            }
            Ok(())
        })();
        self.checking_named = false;
        result
    }
    /// Decode the full closed provider shape and check observations/result types.
    pub fn provider(&mut self, pin: &Pin) -> Result<Arc<Provider>> {
        self.budget.tick(0)?;
        if let Some(p) = self.providers.get(pin) {
            return Ok(p.clone());
        }
        let v = self.load(pin, "provider-contract")?;
        fields(
            &v,
            &[
                "kind",
                "version",
                "id",
                "input",
                "output",
                "outcomes",
                "resultProtection",
                "observations",
                "idempotency",
                "effectClass",
            ],
        )?;
        let input = self.type_descriptor(&v["input"])?;
        let output = self.type_descriptor(&v["output"])?;
        let outcomes = names(&v["outcomes"])?;
        require(!outcomes.is_empty(), "provider-outcomes")?;
        let p = &v["resultProtection"];
        fields(
            p,
            &[
                "policy",
                "sensitivity",
                "purposes",
                "participants",
                "capabilities",
            ],
        )?;
        name(&p["policy"])?;
        name(&p["sensitivity"])?;
        require(!names(&p["purposes"])?.is_empty(), "protection-purposes")?;
        names(&p["participants"])?;
        names(&p["capabilities"])?;
        let mut succeeded = BTreeSet::new();
        let observations = object(&v["observations"])?;
        require(observations.len() <= MAX_ITEMS, "collection")?;
        for (key, o) in observations {
            self.budget.tick(0)?;
            name(&Value::from(key.as_str()))?;
            fields(o, &["state", "result", "outcome"])?;
            let state = text(&o["state"])?;
            require(
                [
                    "acknowledged",
                    "succeeded",
                    "rejected",
                    "cancelled",
                    "unknown",
                    "no-effect",
                ]
                .contains(&state),
                "provider-observation",
            )?;
            if state == "succeeded" {
                let outcome = name(&o["outcome"])?;
                require(
                    outcomes.contains(outcome) && self.type_descriptor(&o["result"])? == output,
                    "provider-result",
                )?;
                succeeded.insert(outcome.to_owned());
            } else {
                require(
                    o["result"].is_null() && o["outcome"].is_null(),
                    "provider-result",
                )?;
            }
        }
        require(succeeded == outcomes, "provider-outcomes")?;
        let effect = text(&v["effectClass"])?;
        let idempotency = text(&v["idempotency"])?;
        require(
            ["read-only", "compensatable", "irreversible"].contains(&effect)
                && ["none", "stable-key"].contains(&idempotency),
            "provider-effect",
        )?;
        let provider = Arc::new(Provider {
            input,
            output,
            outcomes,
            protection: p.clone(),
            effect: effect.into(),
            idempotency: idempotency.into(),
        });
        self.providers.insert(pin.clone(), provider.clone());
        Ok(provider)
    }
    /// Decode finite intervals and verify both provenance pins. This does not
    /// interpret clock/zone-rule bytes, authenticate the schedule or enable timers.
    pub fn calendar(&mut self, pin: &Pin) -> Result<Calendar> {
        let v = self.load(pin, "calendar")?;
        fields(
            &v,
            &[
                "kind",
                "version",
                "id",
                "clock",
                "zone",
                "zoneRules",
                "intervals",
            ],
        )?;
        name(&v["zone"])?;
        let clock = Pin::decode(&v["clock"])?;
        let zone_rules = Pin::decode(&v["zoneRules"])?;
        self.verify(&clock)?;
        self.verify(&zone_rules)?;
        let mut intervals: Vec<(i64, i64)> = Vec::new();
        for interval in items(&v["intervals"])? {
            self.budget.tick(0)?;
            fields(interval, &["start", "end"])?;
            let start = integer(&interval["start"])?;
            let end = integer(&interval["end"])?;
            require(
                start < end && intervals.last().is_none_or(|(_, last)| *last <= start),
                "calendar-intervals",
            )?;
            intervals.push((start, end));
        }
        Ok(Calendar {
            intervals,
            clock,
            zone_rules,
        })
    }
}
