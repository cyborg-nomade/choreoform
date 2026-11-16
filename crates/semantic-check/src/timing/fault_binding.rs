// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

/// Eligible declaration only; no accepted observation or actual fault receipt.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RetryFaultDeclaration {
    evidence: FaultEvidence,
    observations: BTreeSet<String>,
}

impl RetryFaultDeclaration {
    pub fn evidence(&self) -> FaultEvidence {
        self.evidence
    }
    pub fn observations(&self) -> &BTreeSet<String> {
        &self.observations
    }
}

/// Revision-bound declared source/use relation, never actual retry eligibility.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RetryFaultBinding {
    revision: String,
    site: String,
    source: Pin,
    profile: Pin,
    provider: Pin,
    faults: BTreeMap<String, RetryFaultDeclaration>,
}
impl RetryFaultBinding {
    pub fn revision(&self) -> &str {
        &self.revision
    }
    pub fn site(&self) -> &str {
        &self.site
    }
    pub fn source(&self) -> &Pin {
        &self.source
    }
    pub fn profile(&self) -> &Pin {
        &self.profile
    }
    pub fn provider(&self) -> &Pin {
        &self.provider
    }
    pub fn faults(&self) -> &BTreeMap<String, RetryFaultDeclaration> {
        &self.faults
    }
    pub fn host_requirements(&self) -> &'static [&'static str] {
        &[
            "supported-authenticated-provider-observation-and-non-engine-fault-emission",
            "exact-original-attempt-occurrence-request-key-and-input-revision-correlation",
            "current-work-target-source-protection-and-access-authority",
            "actual-stable-key-or-definitive-correlated-no-effect-as-required",
            "accepted-eligible-fault-budget-delay-and-unchanged-basis",
            "atomic-no-retry-after-success-cancellation-or-ordinary-closure",
            "owned-reconciliation-deduplication-contradictions-and-terminal-settlement",
        ]
    }
}

pub(super) fn collect(
    resources: &mut Resources<'_>,
    retry: &Value,
    site: &str,
    revision: &str,
) -> Result<RetryFaultBinding> {
    let source = resources.provider_fault_source(&Pin::decode(&retry["faultSource"])?)?;
    let eligible = names(&retry["faults"])?;
    resources.budget.charge(eligible.len() + 6)?;
    let mut faults = BTreeMap::new();
    for fault in eligible {
        let emission = source
            .emissions()
            .get(&fault)
            .ok_or_else(|| err("retry-fault-source-name"))?;
        resources.budget.charge(emission.observations().len() + 2)?;
        faults.insert(
            fault,
            RetryFaultDeclaration {
                evidence: emission.evidence(),
                observations: emission.observations().clone(),
            },
        );
    }
    Ok(RetryFaultBinding {
        revision: revision.into(),
        site: site.into(),
        source: source.pin().clone(),
        profile: source.profile().clone(),
        provider: source.provider().clone(),
        faults,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn typed_evidence_copying_consumes_the_same_remaining_resource_budget() {
        let provider = serde_json::json!({"kind":"provider-contract","version":"0.1.0","id":"urn:test:binding-provider","input":{"kind":"record","fields":{}},"output":{"kind":"record","fields":{}},"outcomes":{"done":true},"resultProtection":{"policy":"protect","sensitivity":"internal","purposes":{"test":true},"participants":{},"capabilities":{}},"observations":{"done":{"state":"succeeded","result":{"kind":"record","fields":{}},"outcome":"done"},"unknown":{"state":"unknown","result":null,"outcome":null}},"idempotency":"stable-key","effectClass":"irreversible"});
        let raw = serde_json::to_vec(&provider).unwrap();
        let provider_pin = Pin {
            id: "urn:test:binding-provider".into(),
            revision: choreoform_ir_probe_core::digest(&raw),
        };
        let source = serde_json::json!({"kind":"provider-fault-source","version":"0.1.0","id":"urn:test:binding-source","profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":{"id":provider_pin.id,"revision":provider_pin.revision},"emissions":{"uncertain":{"observations":{"unknown":true},"evidence":"unresolved"}}});
        let source_raw = serde_json::to_vec(&source).unwrap();
        let source_pin = Pin {
            id: "urn:test:binding-source".into(),
            revision: choreoform_ir_probe_core::digest(&source_raw),
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
                id: FAULT_SOURCE_PROFILE.0,
                revision: FAULT_SOURCE_PROFILE.1,
                bytes: include_bytes!(
                    "../../../../docs/ir/contracts/sha256-f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0.txt"
                ),
            },
        ];
        let mut resources = Resources::new(&supplied).unwrap();
        resources.provider_fault_source(&source_pin).unwrap();
        let retry = serde_json::json!({"faultSource":{"id":source_pin.id,"revision":source_pin.revision},"faults":{"uncertain":true}});
        resources.budget.work = 999_999;
        assert_eq!(
            collect(&mut resources, &retry, "site", "test-only-revision")
                .unwrap_err()
                .code,
            "budget"
        );
        resources.budget.work = 0;
        let evidence = collect(&mut resources, &retry, "site", "test-only-revision").unwrap();
        assert_eq!(evidence.site(), "site");
        assert_eq!(
            evidence.faults()["uncertain"].evidence(),
            FaultEvidence::Unresolved
        );
    }
}
