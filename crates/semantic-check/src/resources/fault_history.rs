// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Declared history schema and untrusted claim shape, never an accepted event.
use super::*;

pub const FAULT_HISTORY_PROFILE: (&str, &str) = (
    "urn:choreoform:fault-history-profile:provider-observations:0.1.0",
    "sha256:2f580f46eb6ef9b7fff42906f6b0b5cafc6c5fa71ae14365828935bfe2da070c",
);
pub const FAULT_HISTORY_TYPES: (&str, &str) = (
    "urn:choreoform:types:provider-fault-history:0.1.0",
    "sha256:a2e177ce1dd4bad5b9cd68b74e6da7749ec2b04b68a39dae1bd72729e2aae7a7",
);

/// Immutable source/type declaration only. No actual-use or handler entry proof.
#[derive(Debug)]
pub struct ProviderFaultHistory {
    pin: Pin,
    profile: Pin,
    source: Arc<ProviderFaultSource>,
    history_type: Arc<Type>,
}
impl ProviderFaultHistory {
    pub fn pin(&self) -> &Pin {
        &self.pin
    }
    pub fn profile(&self) -> &Pin {
        &self.profile
    }
    pub fn source_metadata(&self) -> &ProviderFaultSource {
        &self.source
    }
    pub fn history_type(&self) -> &Type {
        &self.history_type
    }
    /// Lower bound only: source inputs and occurrence/control provenance must
    /// also be retained and combined by a future actual-entry judgment.
    pub fn source_protection(&self) -> &Value {
        self.source.provider_contract().protection()
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "truthful-authenticated-supported-provider-observation-and-non-engine-emission",
            "actual-definition-instance-scope-node-occurrence-attempt-cause-input-correlation",
            "exact-logical-effect-original-request-key-and-evidence-record-correlation",
            "complete-source-input-fault-existence-control-and-creator-owner-provenance",
            "current-source-work-access-protection-and-management-authority",
            "actual-nearest-handler-selection-exclusion-and-reviewed-fresh-readonly-entry",
            "conditional-initialization-access-flow-and-strict-invalidation",
            "retry-evidence-budget-delay-original-input-authority-and-monotonic-cutoff",
            "permitted-owned-continuation-atomic-handoff-and-all-obligation-settlement",
        ]
    }
}
impl Resources<'_> {
    /// Decode exact metadata only; no fault value, receipt or entry is created.
    pub fn provider_fault_history(&mut self, pin: &Pin) -> Result<Arc<ProviderFaultHistory>> {
        self.budget.tick(0)?;
        if let Some(history) = self.fault_histories.get(pin) {
            return Ok(history.clone());
        }
        let v = self.load(pin, "provider-fault-history")?;
        fields(&v, &["kind", "version", "id", "profile", "source"])?;
        let profile = Pin::decode(&v["profile"])?;
        require(
            (profile.id.as_str(), profile.revision.as_str()) == FAULT_HISTORY_PROFILE,
            "fault-history-profile-unsupported",
        )?;
        self.verify(&profile)?;
        let source = self.provider_fault_source(&Pin::decode(&v["source"])?)?;
        let history_type = if let Some(ty) = &self.fault_history_type {
            ty.clone()
        } else {
            let ty = Arc::new(self.type_descriptor(&serde_json::json!({
                "kind":"named", "contract":{"id":FAULT_HISTORY_TYPES.0,"revision":FAULT_HISTORY_TYPES.1},
                "name":"Provider fault history"
            }))?);
            self.fault_history_type = Some(ty.clone());
            ty
        };
        self.budget.charge(4)?;
        let history = Arc::new(ProviderFaultHistory {
            pin: pin.clone(),
            profile,
            source,
            history_type,
        });
        self.fault_histories.insert(pin.clone(), history.clone());
        Ok(history)
    }

    /// Check a caller's untrusted claim representation and declared mapping.
    /// Success returns unit, never evidence that an event occurred or that a
    /// handler can initialize/read it. No definition, live cell or host is read.
    pub fn check_fault_history_claim(&mut self, descriptor: &Pin, claim: &Value) -> Result<()> {
        let history = self.provider_fault_history(descriptor)?;
        self.literal(history.history_type(), claim)?;
        self.budget.charge(9)?;
        let emission = history
            .source_metadata()
            .emissions()
            .get(text(&claim["type"])?)
            .ok_or_else(|| err("fault-history-source"))?;
        require(
            emission
                .observations()
                .contains(text(&claim["cause"]["observation"])?),
            "fault-history-source",
        )?;
        name(&claim["cause"]["record"])?;
        let origin = &claim["sourceOccurrence"];
        // Reuse the exact pin revision syntax, without implying a registry
        // resource or authentication for this caller-supplied definition hash.
        Pin::decode(
            &serde_json::json!({"id":"definition-claim","revision":origin["definitionRevision"]}),
        )?;
        for key in ["instance", "scopeOccurrence", "site", "occurrence"] {
            name(&origin[key])?;
        }
        require(
            (1..=1000).contains(&integer(&origin["attempt"])?),
            "fault-history-attempt",
        )?;
        let mut cells = BTreeSet::new();
        for input in items(&claim["inputRevisions"])? {
            self.budget.charge(3)?;
            let cell = name(&input["cell"])?;
            name(&input["revision"])?;
            require(cells.insert(cell), "fault-history-inputs")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Vec<(Pin, Vec<u8>)> {
        let mut artifacts = Vec::new();
        for raw in [
            include_bytes!("../../../../docs/benchmarks/rp01/types.json").as_slice(),
            include_bytes!("../../../../docs/benchmarks/rp01/payment.provider.json"),
            include_bytes!("../../../../docs/benchmarks/rp01/payment.fault-source.json"),
            include_bytes!("../../../../docs/dialects/provider-fault-history.types.json"),
        ] {
            let v: Value = serde_json::from_slice(raw).unwrap();
            artifacts.push((
                Pin {
                    id: v["id"].as_str().unwrap().into(),
                    revision: digest(raw),
                },
                raw.to_vec(),
            ));
        }
        for (p, bytes) in [
            (FAULT_SOURCE_PROFILE, include_bytes!("../../../../docs/ir/contracts/sha256-f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0.txt").as_slice()),
            (FAULT_HISTORY_PROFILE, include_bytes!("../../../../docs/ir/contracts/sha256-2f580f46eb6ef9b7fff42906f6b0b5cafc6c5fa71ae14365828935bfe2da070c.txt")),
        ] { artifacts.push((Pin{id:p.0.into(),revision:p.1.into()},bytes.to_vec())); }
        let source = artifacts[2].0.clone();
        for id in ["urn:test:history-budget", "urn:test:history-alias"] {
            let bytes=serde_json::to_vec(&serde_json::json!({"kind":"provider-fault-history","version":"0.1.0","id":id,"profile":{"id":FAULT_HISTORY_PROFILE.0,"revision":FAULT_HISTORY_PROFILE.1},"source":{"id":source.id,"revision":source.revision}})).unwrap();
            artifacts.push((
                Pin {
                    id: id.into(),
                    revision: digest(&bytes),
                },
                bytes,
            ));
        }
        artifacts
    }
    fn supplied(artifacts: &[(Pin, Vec<u8>)]) -> Vec<Resource<'_>> {
        artifacts
            .iter()
            .map(|(p, bytes)| Resource {
                id: &p.id,
                revision: &p.revision,
                bytes,
            })
            .collect()
    }
    #[test]
    fn shared_aliases_and_cached_lookups_never_refill_budget_or_publish_failed_descriptor() {
        let a = fixture();
        let supplied = supplied(&a);
        let mut r = Resources::new(&supplied).unwrap();
        let pin = &a[6].0;
        let alias = &a[7].0;
        r.provider_fault_history(pin).unwrap();
        let before = r.budget.work;
        r.provider_fault_history(pin).unwrap();
        assert!(r.budget.work > before);
        // Fail at the final construction charge, after all dependencies are
        // cached. Only the already verified shared schema may remain cached.
        r.budget.work = 999_994;
        assert_eq!(r.provider_fault_history(alias).unwrap_err().code, "budget");
        assert!(!r.fault_histories.contains_key(alias));
        r.budget.work = 1_000_000;
        assert_eq!(r.provider_fault_history(pin).unwrap_err().code, "budget");
        r.budget.work = 0;
        let x = r.provider_fault_history(pin).unwrap();
        let y = r.provider_fault_history(alias).unwrap();
        assert!(std::ptr::eq(x.history_type(), y.history_type()));
    }
    #[test]
    fn repeated_claims_and_each_input_row_consume_the_same_cumulative_allowance() {
        let a = fixture();
        let supplied = supplied(&a);
        let mut r = Resources::new(&supplied).unwrap();
        let pin = &a[6].0;
        let v: Value = serde_json::from_slice(include_bytes!(
            "../../tests/conformance/provider-fault-history.json"
        ))
        .unwrap();
        let mut c = v["base"].clone();
        r.check_fault_history_claim(pin, &c).unwrap();
        let before = r.budget.work;
        r.check_fault_history_claim(pin, &c).unwrap();
        let small = r.budget.work - before;
        c["inputRevisions"] = Value::Array(
            (0..200)
                .map(|i| serde_json::json!({"cell":format!("cell-{i}"),"revision":"opaque-v1"}))
                .collect(),
        );
        let before = r.budget.work;
        r.check_fault_history_claim(pin, &c).unwrap();
        assert!(r.budget.work - before > small + 600);
        r.budget.work = 999_900;
        assert_eq!(
            r.check_fault_history_claim(pin, &c).unwrap_err().code,
            "budget"
        );
    }
}
