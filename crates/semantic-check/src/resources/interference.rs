// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Standalone declarations only. No definition selection or physical independence.
use super::*;

pub const INTERFERENCE_PROFILE: (&str, &str) = (
    "urn:choreoform:interference-profile:provider-regions:0.1.0",
    "sha256:0b10eb4dbb19caace45b93f35dec44aa5f26458d631393932caddb5589210250",
);

/// Exact scalar type of one declared region-key component, never a live value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionKeyType {
    Text,
    Integer,
}
impl RegionKeyType {
    fn decode(v: &Value) -> Result<Self> {
        match text(v)? {
            "text" => Ok(Self::Text),
            "integer" => Ok(Self::Integer),
            _ => Err(err("interference-key-type")),
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Integer => "integer",
        }
    }
    fn matches(self, ty: &Type) -> bool {
        matches!(
            (self, ty),
            (Self::Text, Type::Text) | (Self::Integer, Type::Integer)
        )
    }
}

/// Closed typed path metadata. Present does not attest that an Option is present.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestSelectorStep {
    Field(String),
    Present,
    Representation,
}
#[derive(Debug)]
pub struct RequestSelector {
    path: Vec<RequestSelectorStep>,
    key_type: RegionKeyType,
}
impl RequestSelector {
    pub fn path(&self) -> &[RequestSelectorStep] {
        &self.path
    }
    pub fn key_type(&self) -> RegionKeyType {
        self.key_type
    }
}

/// Integrity-checked namespace declaration, not a supported physical normalizer.
#[derive(Debug)]
pub struct ExternalRegionNamespace {
    pin: Pin,
    profile: Pin,
    authority: Pin,
    key_types: Vec<RegionKeyType>,
}
impl ExternalRegionNamespace {
    pub fn pin(&self) -> &Pin {
        &self.pin
    }
    pub fn profile(&self) -> &Pin {
        &self.profile
    }
    pub fn authority(&self) -> &Pin {
        &self.authority
    }
    pub fn key_types(&self) -> &[RegionKeyType] {
        &self.key_types
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "registered-supported-truthful-common-canonicalization-authority",
            "exhaustive-cross-provider-namespace-key-alias-and-non-alias-evidence",
            "stable-physical-bindings-through-surviving-work",
        ]
    }
}
#[derive(Debug)]
pub struct ExternalRegion {
    namespace: Arc<ExternalRegionNamespace>,
    key: Vec<RequestSelector>,
}
impl ExternalRegion {
    pub fn namespace(&self) -> &Arc<ExternalRegionNamespace> {
        &self.namespace
    }
    pub fn key(&self) -> &[RequestSelector] {
        &self.key
    }
}
/// Exhaustive behavior claim only; finite does not imply independence or truth.
#[derive(Debug)]
pub enum ExternalFootprint {
    Opaque,
    Finite {
        reads: Vec<ExternalRegion>,
        writes: Vec<ExternalRegion>,
    },
}
#[derive(Debug)]
pub struct ProviderInterference {
    pin: Pin,
    profile: Pin,
    provider_profile: Pin,
    provider: Pin,
    contract: Arc<Provider>,
    activity: ExternalFootprint,
    terminal: ExternalFootprint,
}
impl ProviderInterference {
    pub fn pin(&self) -> &Pin {
        &self.pin
    }
    pub fn profile(&self) -> &Pin {
        &self.profile
    }
    pub fn provider_profile(&self) -> &Pin {
        &self.provider_profile
    }
    pub fn provider(&self) -> &Pin {
        &self.provider
    }
    pub fn provider_contract(&self) -> &Arc<Provider> {
        &self.contract
    }
    pub fn activity(&self) -> &ExternalFootprint {
        &self.activity
    }
    pub fn terminal(&self) -> &ExternalFootprint {
        &self.terminal
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "explicit-unique-semantic-definition-selection-and-dependency-closure",
            "generic-original-request-disjointness-for-all-overlapping-surviving-occurrences",
            "truthful-exhaustive-provider-activity-and-terminal-coverage",
            "supported-common-physical-canonicalization-and-exhaustive-alias-evidence",
            "accepted-typed-original-request-input-revision-occurrence-effect-attempt-correlation",
            "current-protection-authority-and-no-declassification",
            "uncertainty-all-attempt-quiescence-and-durable-business-obligations",
            "atomic-authorized-transitive-ownership-handoff-and-settlement",
            "all-other-semantic-host-browser-user-independent-and-phase-gates",
        ]
    }
}

fn bounded_name(v: &Value) -> Result<&str> {
    let value = text(v)?;
    require(!value.is_empty() && value.len() <= 256, "interference-name")?;
    Ok(value)
}
fn bounded_pin(v: &Value) -> Result<Pin> {
    let pin = Pin::decode(v)?;
    require(pin.id.len() <= 256, "interference-name")?;
    Ok(pin)
}
impl Resources<'_> {
    fn interference_profile(&mut self, v: &Value) -> Result<Pin> {
        let profile = bounded_pin(v)?;
        require(
            (profile.id.as_str(), profile.revision.as_str()) == INTERFERENCE_PROFILE,
            "interference-profile-unsupported",
        )?;
        self.verify(&profile)?;
        Ok(profile)
    }
    /// Decode an exact namespace and authority contract's integrity. Does not
    /// implement/register/trust the authority or map any physical resource.
    pub fn external_region_namespace(&mut self, pin: &Pin) -> Result<Arc<ExternalRegionNamespace>> {
        self.budget.tick(0)?;
        require(pin.id.len() <= 256, "interference-name")?;
        if let Some(namespace) = self.external_namespaces.get(pin) {
            return Ok(namespace.clone());
        }
        let v = self.load(pin, "external-region-namespace")?;
        fields(
            &v,
            &["kind", "version", "id", "profile", "authority", "keyTypes"],
        )?;
        let profile = self.interference_profile(&v["profile"])?;
        let authority = bounded_pin(&v["authority"])?;
        self.verify(&authority)?;
        let raw = self.supplied[&authority];
        require(
            !raw.is_empty() && std::str::from_utf8(raw).is_ok(),
            "interference-authority-contract",
        )?;
        let components = items(&v["keyTypes"])?;
        require(
            (1..=4).contains(&components.len()),
            "interference-key-arity",
        )?;
        self.budget.charge(components.len())?;
        let key_types = components
            .iter()
            .map(RegionKeyType::decode)
            .collect::<Result<_>>()?;
        let namespace = Arc::new(ExternalRegionNamespace {
            pin: pin.clone(),
            profile,
            authority,
            key_types,
        });
        self.external_namespaces
            .insert(pin.clone(), namespace.clone());
        Ok(namespace)
    }
    /// Decode both channels and every selector under the complete provider/type
    /// judgment. No request evaluation, definition binding or independence result.
    pub fn provider_interference(&mut self, pin: &Pin) -> Result<Arc<ProviderInterference>> {
        self.budget.tick(0)?;
        require(pin.id.len() <= 256, "interference-name")?;
        if let Some(descriptor) = self.interference.get(pin) {
            return Ok(descriptor.clone());
        }
        let v = self.load(pin, "external-interference")?;
        fields(
            &v,
            &[
                "kind",
                "version",
                "id",
                "profile",
                "providerProfile",
                "provider",
                "coverage",
                "activity",
                "terminal",
            ],
        )?;
        let profile = self.interference_profile(&v["profile"])?;
        let provider_profile = bounded_pin(&v["providerProfile"])?;
        require(
            (
                provider_profile.id.as_str(),
                provider_profile.revision.as_str(),
            ) == CORE_V12,
            "interference-provider-profile",
        )?;
        self.verify(&provider_profile)?;
        require(v["coverage"] == "exhaustive", "interference-coverage")?;
        let provider = bounded_pin(&v["provider"])?;
        let contract = self.provider(&provider)?;
        let activity = self.external_footprint(
            &v["activity"],
            &contract.input,
            contract.effect == "read-only",
        )?;
        let terminal = self.external_footprint(&v["terminal"], &contract.input, true)?;
        let descriptor = Arc::new(ProviderInterference {
            pin: pin.clone(),
            profile,
            provider_profile,
            provider,
            contract,
            activity,
            terminal,
        });
        self.interference.insert(pin.clone(), descriptor.clone());
        Ok(descriptor)
    }
    fn external_footprint(
        &mut self,
        v: &Value,
        input: &Type,
        readonly: bool,
    ) -> Result<ExternalFootprint> {
        self.budget.tick(0)?;
        if v == "opaque" {
            return Ok(ExternalFootprint::Opaque);
        }
        fields(v, &["reads", "writes"])?;
        let reads = items(&v["reads"])?;
        let writes = items(&v["writes"])?;
        require(
            reads.len() <= 16 && writes.len() <= 16,
            "interference-regions",
        )?;
        require(readonly == writes.is_empty(), "interference-class")?;
        let mut decode = |regions: &[Value]| {
            regions
                .iter()
                .map(|r| self.external_region(r, input))
                .collect::<Result<Vec<_>>>()
        };
        Ok(ExternalFootprint::Finite {
            reads: decode(reads)?,
            writes: decode(writes)?,
        })
    }
    fn external_region(&mut self, v: &Value, input: &Type) -> Result<ExternalRegion> {
        self.budget.tick(0)?;
        fields(v, &["namespace", "key"])?;
        let pin = bounded_pin(&v["namespace"])?;
        let namespace = self.external_region_namespace(&pin)?;
        let selectors = items(&v["key"])?;
        require(
            selectors.len() == namespace.key_types.len(),
            "interference-key-arity",
        )?;
        let mut key = Vec::with_capacity(selectors.len());
        for (selector, expected) in selectors.iter().zip(&namespace.key_types) {
            self.budget.tick(0)?;
            fields(selector, &["path", "type"])?;
            let key_type = RegionKeyType::decode(&selector["type"])?;
            require(key_type == *expected, "interference-key-type")?;
            let steps = items(&selector["path"])?;
            require(steps.len() <= 8, "interference-selector-depth")?;
            let mut path = Vec::with_capacity(steps.len());
            for step in steps {
                self.budget.tick(0)?;
                let op = text(&step["op"])?;
                path.push(match op {
                    "field" => {
                        fields(step, &["op", "name"])?;
                        RequestSelectorStep::Field(bounded_name(&step["name"])?.to_owned())
                    }
                    "present" => {
                        fields(step, &["op"])?;
                        RequestSelectorStep::Present
                    }
                    "representation" => {
                        fields(step, &["op"])?;
                        RequestSelectorStep::Representation
                    }
                    _ => return Err(err("interference-selector-step")),
                });
            }
            self.external_selector_type(input, &path, key_type, 0)?;
            key.push(RequestSelector { path, key_type });
        }
        Ok(ExternalRegion { namespace, key })
    }
    fn external_selector_type(
        &mut self,
        ty: &Type,
        path: &[RequestSelectorStep],
        leaf: RegionKeyType,
        depth: usize,
    ) -> Result<()> {
        self.budget.tick(depth)?;
        let Some((first, rest)) = path.split_first() else {
            return require(leaf.matches(ty), "interference-selector-type");
        };
        match (first, ty) {
            (RequestSelectorStep::Field(name), Type::Record(fields)) => {
                let next = fields
                    .get(name)
                    .ok_or_else(|| err("interference-selector-type"))?;
                self.external_selector_type(next, rest, leaf, depth + 1)
            }
            (RequestSelectorStep::Present, Type::Option(element)) => {
                self.external_selector_type(element, rest, leaf, depth + 1)
            }
            (RequestSelectorStep::Representation, Type::Named(name)) => {
                let definition = self.nominal(name, depth + 1)?;
                self.external_selector_type(&definition.representation, rest, leaf, depth + 1)
            }
            _ => Err(err("interference-selector-type")),
        }
    }
}
