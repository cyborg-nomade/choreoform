// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

pub const CLOCK_PROFILE: (&str, &str) = (
    "urn:choreoform:clock-profile:instance-microseconds:0.1.0",
    "sha256:8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9",
);

/// Immutable clock metadata, never an authenticated source or execution capability.
#[derive(Debug)]
pub struct Clock {
    pin: Pin,
    profile: Pin,
    source: Pin,
    provider: Arc<Provider>,
}
impl Clock {
    pub fn pin(&self) -> &Pin {
        &self.pin
    }
    pub fn profile(&self) -> &Pin {
        &self.profile
    }
    pub fn source(&self) -> &Pin {
        &self.source
    }
    /// Result protection still needs definition-context access/flow validation.
    pub fn source_contract(&self) -> &Provider {
        &self.provider
    }
}
impl Resources<'_> {
    /// Decode one exact implemented clock profile and its typed source contract.
    /// This neither reads a clock nor admits a timer or authenticates a host.
    pub fn clock(&mut self, pin: &Pin) -> Result<Clock> {
        let v = self.load(pin, "clock")?;
        fields(&v, &["kind", "version", "id", "profile", "source"])?;
        let profile = Pin::decode(&v["profile"])?;
        require(
            (profile.id.as_str(), profile.revision.as_str()) == CLOCK_PROFILE,
            "clock-profile-unsupported",
        )?;
        self.verify(&profile)?;
        let source = Pin::decode(&v["source"])?;
        let provider = self.provider(&source)?;
        require(
            provider.input == Type::empty()
                && provider.output == Type::Integer
                && provider.outcomes == BTreeSet::from(["tick".into()])
                && provider.effect == "read-only"
                && provider.idempotency == "none",
            "clock-source-contract",
        )?;
        Ok(Clock {
            pin: pin.clone(),
            profile,
            source,
            provider,
        })
    }
}
