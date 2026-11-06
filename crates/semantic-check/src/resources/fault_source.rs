// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

pub const FAULT_SOURCE_PROFILE: (&str, &str) = (
    "urn:choreoform:fault-source-profile:provider-observations:0.1.0",
    "sha256:f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0",
);

/// Declared evidence class only; no accepted fact or retry permission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultEvidence {
    DefinitiveNoEffect,
    Unresolved,
}

impl FaultEvidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DefinitiveNoEffect => "definitive-no-effect",
            Self::Unresolved => "unresolved",
        }
    }
    fn permits(self, state: ProviderObservationState) -> bool {
        match self {
            Self::DefinitiveNoEffect => matches!(
                state,
                ProviderObservationState::Rejected
                    | ProviderObservationState::Cancelled
                    | ProviderObservationState::NoEffect
            ),
            Self::Unresolved => state == ProviderObservationState::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct FaultEmission {
    observations: BTreeSet<String>,
    evidence: FaultEvidence,
}
impl FaultEmission {
    pub fn observations(&self) -> &BTreeSet<String> {
        &self.observations
    }
    pub fn evidence(&self) -> FaultEvidence {
        self.evidence
    }
}

/// Immutable mapping declarations, not a definition binding or fault receipt.
#[derive(Debug)]
pub struct ProviderFaultSource {
    pin: Pin,
    profile: Pin,
    provider: Pin,
    contract: Arc<Provider>,
    emissions: BTreeMap<String, FaultEmission>,
}
impl ProviderFaultSource {
    pub fn pin(&self) -> &Pin {
        &self.pin
    }
    pub fn profile(&self) -> &Pin {
        &self.profile
    }
    pub fn provider(&self) -> &Pin {
        &self.provider
    }
    pub fn provider_contract(&self) -> &Provider {
        &self.contract
    }
    pub fn emissions(&self) -> &BTreeMap<String, FaultEmission> {
        &self.emissions
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "explicit-supported-definition-retry-and-actual-provider-use-binding",
            "authenticated-supported-provider-observation-and-fault-emission",
            "exact-attempt-occurrence-request-key-and-input-revision-correlation",
            "current-protection-authority-and-accepted-fault-routing",
            "stable-key-or-definitive-correlated-no-effect-check-before-retry",
            "atomic-no-retry-after-success-cancellation-or-ordinary-closure",
            "accepted-owned-reconciliation-and-all-obligation-settlement",
        ]
    }
}
impl Resources<'_> {
    /// Decode mapping metadata only. Existing definitions do not implicitly bind
    /// it; no observation is accepted and no retry/dispatch is authorized.
    pub fn provider_fault_source(&mut self, pin: &Pin) -> Result<Arc<ProviderFaultSource>> {
        self.budget.tick(0)?;
        if let Some(source) = self.fault_sources.get(pin) {
            return Ok(source.clone());
        }
        let v = self.load(pin, "provider-fault-source")?;
        fields(
            &v,
            &["kind", "version", "id", "profile", "provider", "emissions"],
        )?;
        let profile = Pin::decode(&v["profile"])?;
        require(
            (profile.id.as_str(), profile.revision.as_str()) == FAULT_SOURCE_PROFILE,
            "fault-source-profile-unsupported",
        )?;
        self.verify(&profile)?;
        let provider = Pin::decode(&v["provider"])?;
        let contract = self.provider(&provider)?;
        let declarations = object(&v["emissions"])?;
        require(
            !declarations.is_empty() && declarations.len() <= MAX_ITEMS,
            "fault-source-emissions",
        )?;
        let mut emissions = BTreeMap::new();
        let mut assigned = BTreeSet::new();
        for (fault, emission) in declarations {
            self.budget.tick(0)?;
            name(&Value::from(fault.as_str()))?;
            fields(emission, &["observations", "evidence"])?;
            let evidence = match text(&emission["evidence"])? {
                "definitive-no-effect" => FaultEvidence::DefinitiveNoEffect,
                "unresolved" => FaultEvidence::Unresolved,
                _ => return Err(err("fault-source-evidence")),
            };
            self.budget
                .charge(object(&emission["observations"])?.len())?;
            let observations = names(&emission["observations"])?;
            require(!observations.is_empty(), "fault-source-observations")?;
            for observation in &observations {
                self.budget.charge(2)?;
                let state = contract
                    .observations()
                    .get(observation)
                    .ok_or_else(|| err("fault-source-observation"))?;
                require(evidence.permits(*state), "fault-source-evidence")?;
                require(
                    assigned.insert(observation.clone()),
                    "fault-source-ambiguous",
                )?;
            }
            self.budget.tick(0)?;
            emissions.insert(
                fault.clone(),
                FaultEmission {
                    observations,
                    evidence,
                },
            );
        }
        let source = Arc::new(ProviderFaultSource {
            pin: pin.clone(),
            profile,
            provider,
            contract,
            emissions,
        });
        self.fault_sources.insert(pin.clone(), source.clone());
        Ok(source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn mapping_and_cached_lookup_charge_the_same_cumulative_resource_budget() {
        let provider = json!({"kind":"provider-contract","version":"0.1.0","id":"urn:test:fault-provider","input":{"kind":"record","fields":{}},"output":{"kind":"record","fields":{}},"outcomes":{"done":true},"resultProtection":{"policy":"protect","sensitivity":"internal","purposes":{"test":true},"participants":{},"capabilities":{}},"observations":{"done":{"state":"succeeded","result":{"kind":"record","fields":{}},"outcome":"done"},"unknown":{"state":"unknown","result":null,"outcome":null}},"idempotency":"none","effectClass":"read-only"});
        let provider_bytes = serde_json::to_vec(&provider).unwrap();
        let p = Pin {
            id: "urn:test:fault-provider".into(),
            revision: digest(&provider_bytes),
        };
        let source = json!({"kind":"provider-fault-source","version":"0.1.0","id":"urn:test:fault-source","profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":{"id":p.id,"revision":p.revision},"emissions":{"uncertain":{"observations":{"unknown":true},"evidence":"unresolved"}}});
        let bytes = serde_json::to_vec(&source).unwrap();
        let pin = Pin {
            id: "urn:test:fault-source".into(),
            revision: digest(&bytes),
        };
        let supplied = [
            Resource {
                id: &pin.id,
                revision: &pin.revision,
                bytes: &bytes,
            },
            Resource {
                id: &p.id,
                revision: &p.revision,
                bytes: &provider_bytes,
            },
            Resource {
                id: FAULT_SOURCE_PROFILE.0,
                revision: FAULT_SOURCE_PROFILE.1,
                bytes: include_bytes!(
                    "../../../../docs/ir/contracts/sha256-f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0.txt"
                ),
            },
        ];
        let mut r = Resources::new(&supplied).unwrap();
        r.budget.work = 999_999;
        assert_eq!(r.provider_fault_source(&pin).unwrap_err().code, "budget");
        assert!(r.fault_sources.is_empty());
        r.budget.work = 0;
        let a = r.provider_fault_source(&pin).unwrap();
        let prior = r.budget.work;
        let b = r.provider_fault_source(&pin).unwrap();
        assert!(Arc::ptr_eq(&a, &b));
        assert!(r.budget.work > prior);
        r.budget.work = 1_000_000;
        assert_eq!(r.provider_fault_source(&pin).unwrap_err().code, "budget");
    }
}
