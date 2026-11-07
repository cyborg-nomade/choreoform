// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

pub const TERMINAL_OBSERVATION_PROFILE: (&str, &str) = (
    "urn:choreoform:observation-source-profile:provider-terminal-facts:0.1.0",
    "sha256:a224fc2b8318af2ff8dfc9955b55af03f4d5b7bd0de719de0384421698d20ead",
);

/// Normalized declaration tag, never an accepted provider/lifecycle fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TerminalFactTag {
    Confirmed,
    Rejected,
    Cancelled,
    NoEffect,
}
impl TerminalFactTag {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Confirmed => "confirmed",
            Self::Rejected => "rejected",
            Self::Cancelled => "cancelled",
            Self::NoEffect => "noeffect",
        }
    }
    fn from_state(state: ProviderObservationState) -> Option<Self> {
        match state {
            ProviderObservationState::Succeeded => Some(Self::Confirmed),
            ProviderObservationState::Rejected => Some(Self::Rejected),
            ProviderObservationState::Cancelled => Some(Self::Cancelled),
            ProviderObservationState::NoEffect => Some(Self::NoEffect),
            ProviderObservationState::Acknowledged | ProviderObservationState::Unknown => None,
        }
    }
}

#[derive(Debug)]
pub struct TerminalObservation {
    state: ProviderObservationState,
    tag: TerminalFactTag,
}
impl TerminalObservation {
    pub fn state(&self) -> ProviderObservationState {
        self.state
    }
    pub fn tag(&self) -> TerminalFactTag {
        self.tag
    }
}

/// Immutable exact metadata only; no wait binding, observation or permission.
#[derive(Debug)]
pub struct ProviderTerminalSource {
    pin: Pin,
    profile: Pin,
    provider: Pin,
    contract: Arc<Provider>,
    schema: Arc<TerminalSchema>,
}

#[derive(Debug)]
pub(super) struct TerminalSchema {
    terminal: BTreeMap<String, TerminalObservation>,
    observation_type: Type,
}
impl ProviderTerminalSource {
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
    pub fn terminal_observations(&self) -> &BTreeMap<String, TerminalObservation> {
        &self.schema.terminal
    }
    pub fn observation_type(&self) -> &Type {
        &self.schema.observation_type
    }
    /// Exact declared source envelope; actual visibility/access is still unproved.
    pub fn source_protection(&self) -> &Value {
        self.contract.protection()
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "explicit-supported-wait-use-and-capture-representation-binding",
            "truthful-authenticated-provider-and-implemented-normalizer",
            "validated-provider-observation-outcome-and-normalized-payload",
            "exact-effect-attempt-occurrence-request-key-input-and-observation-correlation",
            "current-authority-protection-and-no-declassification-transfer",
            "engine-acceptance-order-and-success-cancellation-closure-cutoffs",
            "durable-late-duplicate-and-contradictory-observation-retention",
            "live-owned-storage-subscriptions-and-all-obligation-settlement",
        ]
    }
}
impl Resources<'_> {
    /// Decode declared terminal normalization metadata without binding a wait or
    /// reading, accepting or authenticating any provider observation.
    pub fn provider_terminal_source(&mut self, pin: &Pin) -> Result<Arc<ProviderTerminalSource>> {
        self.budget.tick(0)?;
        if let Some(source) = self.terminal_sources.get(pin) {
            return Ok(source.clone());
        }
        let v = self.load(pin, "provider-terminal-observation-source")?;
        fields(&v, &["kind", "version", "id", "profile", "provider"])?;
        let profile = Pin::decode(&v["profile"])?;
        require(
            (profile.id.as_str(), profile.revision.as_str()) == TERMINAL_OBSERVATION_PROFILE,
            "terminal-source-profile-unsupported",
        )?;
        self.verify(&profile)?;
        let provider = Pin::decode(&v["provider"])?;
        let contract = self.provider(&provider)?;
        let key = (profile.clone(), provider.clone());
        self.budget.charge(2)?;
        let schema = if let Some(schema) = self.terminal_schemas.get(&key) {
            schema.clone()
        } else {
            let schema = Arc::new(derive_schema(&contract, &mut self.budget)?);
            self.terminal_schemas.insert(key, schema.clone());
            schema
        };
        self.budget.charge(5)?;
        let source = Arc::new(ProviderTerminalSource {
            pin: pin.clone(),
            profile,
            provider,
            contract,
            schema,
        });
        self.terminal_sources.insert(pin.clone(), source.clone());
        Ok(source)
    }
}

fn derive_schema(contract: &Provider, budget: &mut Budget) -> Result<TerminalSchema> {
    let mut terminal = BTreeMap::new();
    let mut tags = BTreeSet::new();
    for (name, state) in contract.observations() {
        budget.tick(0)?;
        if let Some(tag) = TerminalFactTag::from_state(*state) {
            budget.charge(3)?;
            terminal.insert(name.clone(), TerminalObservation { state: *state, tag });
            tags.insert(tag);
        }
    }
    require(!terminal.is_empty(), "terminal-source-empty")?;
    let mut cases = BTreeMap::new();
    for tag in tags {
        budget.charge(2)?;
        let ty = if tag == TerminalFactTag::Confirmed {
            // Charge every nested type/key/reference copy, including the
            // extra Variant layer. No separate allowance for cached providers.
            copy_type(contract.output(), budget, 1)?
        } else {
            budget.charge(7)?;
            Type::Record(
                ["logical effect", "request digest", "observation"]
                    .map(|k| (k.into(), Type::Text))
                    .into(),
            )
        };
        cases.insert(tag.as_str().into(), ty);
    }
    Ok(TerminalSchema {
        terminal,
        observation_type: Type::Variant(cases),
    })
}

fn copy_type(ty: &Type, budget: &mut Budget, depth: usize) -> Result<Type> {
    budget.tick(depth)?;
    Ok(match ty {
        Type::Record(map) | Type::Variant(map) => {
            let mut copied = BTreeMap::new();
            for (key, ty) in map {
                budget.charge(1)?;
                copied.insert(key.clone(), copy_type(ty, budget, depth + 1)?);
            }
            if matches!(ty, Type::Record(_)) {
                Type::Record(copied)
            } else {
                Type::Variant(copied)
            }
        }
        Type::List(ty) => Type::List(Box::new(copy_type(ty, budget, depth + 1)?)),
        Type::Option(ty) => Type::Option(Box::new(copy_type(ty, budget, depth + 1)?)),
        Type::Named(_) => {
            budget.charge(3)?;
            ty.clone()
        }
        _ => ty.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn derived_output_copy_preserves_names_and_exact_nominal_pins_with_shared_work() {
        let named = Type::Named(Nominal {
            contract: Pin {
                id: "urn:test:nominal".into(),
                revision: format!("sha256:{}", "1".repeat(64)),
            },
            name: "Amount".into(),
        });
        let ty = Type::Record(
            [(
                "name".into(),
                Type::Option(Box::new(Type::List(Box::new(named)))),
            )]
            .into(),
        );
        let mut budget = Budget::default();
        assert_eq!(copy_type(&ty, &mut budget, 1).unwrap(), ty);
        assert!(budget.work > 5);
        budget.work = 999_999;
        assert_eq!(copy_type(&ty, &mut budget, 1).unwrap_err().code, "budget");
    }
    #[test]
    fn failed_schema_is_not_cached_and_cached_lookups_keep_the_same_allowance() {
        let fields: serde_json::Map<String, Value> = (0..40)
            .map(|i| (format!("f{i}"), serde_json::json!({"kind":"text"})))
            .collect();
        let output = serde_json::json!({"kind":"record","fields":fields});
        let provider = serde_json::json!({"kind":"provider-contract","version":"0.1.0","id":"urn:test:terminal-budget-provider","input":{"kind":"record","fields":{}},"output":output,"outcomes":{"done":true},"resultProtection":{"policy":"protect","sensitivity":"internal","purposes":{"test":true},"participants":{},"capabilities":{}},"observations":{"done":{"state":"succeeded","result":output,"outcome":"done"}},"idempotency":"none","effectClass":"read-only"});
        let raw = serde_json::to_vec(&provider).unwrap();
        let provider_pin = Pin {
            id: provider["id"].as_str().unwrap().into(),
            revision: digest(&raw),
        };
        let source = serde_json::json!({"kind":"provider-terminal-observation-source","version":"0.1.0","id":"urn:test:terminal-budget-source","profile":{"id":TERMINAL_OBSERVATION_PROFILE.0,"revision":TERMINAL_OBSERVATION_PROFILE.1},"provider":{"id":provider_pin.id,"revision":provider_pin.revision}});
        let source_raw = serde_json::to_vec(&source).unwrap();
        let source_pin = Pin {
            id: source["id"].as_str().unwrap().into(),
            revision: digest(&source_raw),
        };
        let supplied = [
            Resource {
                id: &provider_pin.id,
                revision: &provider_pin.revision,
                bytes: &raw,
            },
            Resource {
                id: &source_pin.id,
                revision: &source_pin.revision,
                bytes: &source_raw,
            },
            Resource {
                id: TERMINAL_OBSERVATION_PROFILE.0,
                revision: TERMINAL_OBSERVATION_PROFILE.1,
                bytes: include_bytes!(
                    "../../../../docs/ir/contracts/sha256-a224fc2b8318af2ff8dfc9955b55af03f4d5b7bd0de719de0384421698d20ead.txt"
                ),
            },
        ];
        let mut resources = Resources::new(&supplied).unwrap();
        resources.provider(&provider_pin).unwrap();
        resources
            .load(&source_pin, "provider-terminal-observation-source")
            .unwrap();
        resources
            .verify(&Pin {
                id: TERMINAL_OBSERVATION_PROFILE.0.into(),
                revision: TERMINAL_OBSERVATION_PROFILE.1.into(),
            })
            .unwrap();
        resources.budget.work = 999_980;
        assert_eq!(
            resources
                .provider_terminal_source(&source_pin)
                .unwrap_err()
                .code,
            "budget"
        );
        assert!(resources.terminal_schemas.is_empty());
        assert!(resources.terminal_sources.is_empty());
        // Reset only this synthetic test's private work counter to inspect retry
        // after the failed derivation; the public API has no refill operation.
        resources.budget.work = 0;
        let source = resources.provider_terminal_source(&source_pin).unwrap();
        assert_eq!(source.terminal_observations().len(), 1);
        assert_eq!(resources.terminal_schemas.len(), 1);
        resources.budget.work = 1_000_000;
        assert_eq!(
            resources
                .provider_terminal_source(&source_pin)
                .unwrap_err()
                .code,
            "budget"
        );
    }
    #[test]
    fn extra_normalization_layer_respects_existing_type_depth() {
        let mut ty = Type::Text;
        for _ in 0..64 {
            ty = Type::List(Box::new(ty));
        }
        assert_eq!(
            copy_type(&ty, &mut Budget::default(), 1).unwrap_err().code,
            "budget"
        );
    }
}
