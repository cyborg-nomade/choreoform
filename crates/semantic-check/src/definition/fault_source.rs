// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use std::sync::Arc;

/// Exact declaration only: neither an emitted/accepted fault nor retry authority.
#[derive(Debug)]
pub struct DeclaredFaultSourceUse {
    revision: Arc<str>,
    site: String,
    policy: String,
    metadata: Arc<ProviderFaultSource>,
}
impl DeclaredFaultSourceUse {
    pub fn revision(&self) -> &str {
        &self.revision
    }
    pub fn site(&self) -> &str {
        &self.site
    }
    pub fn policy(&self) -> &str {
        &self.policy
    }
    pub fn source(&self) -> &Pin {
        self.metadata.pin()
    }
    pub fn profile(&self) -> &Pin {
        self.metadata.profile()
    }
    pub fn provider(&self) -> &Pin {
        self.metadata.provider()
    }
    /// All declared emission names/classes, distinct from retry eligibility.
    pub fn source_metadata(&self) -> &ProviderFaultSource {
        &self.metadata
    }
    pub fn host_requirements(&self) -> &'static [&'static str] {
        &[
            "supported-authenticated-provider-and-actual-non-engine-emission-classification",
            "exact-live-source-occurrence-attempt-logical-effect-request-key-cause-input-revisions",
            "current-source-work-access-protection-and-management-authority",
            "actual-eligible-evidence-delay-basis-budget-and-cancellation-cutoff-for-any-retry",
            "accepted-exact-nearest-handler-selection-exclusion-and-fresh-entry-snapshot",
            "retained-source-control-provenance-and-owned-atomic-settlement",
            "typed-fault-data-and-permitted-continuation-not-supplied-by-this-declaration",
        ]
    }
}
pub(super) struct PendingFaultSource {
    site: String,
    policy: String,
    metadata: Arc<ProviderFaultSource>,
}
impl Check<'_, '_> {
    pub(super) fn general_fault_source(
        &mut self,
        site: &str,
        n: &Value,
        work: &Value,
    ) -> Result<()> {
        if self.b["dialects"][text(&work["dialect"])?]["id"] != CORE_V6.0
            || work["body"]["faultSource"].is_null()
        {
            return Ok(());
        }
        require(n["mode"] == "capability", "fault-source-context-unproved")?;
        let source = self
            .resources
            .provider_fault_source(&Pin::decode(&work["body"]["faultSource"])?)?;
        let cap = self.visible(text(&n["scope"])?, "capabilities", &n["target"])?;
        let provider = Pin::decode(
            &self.b["policies"][text(&self.b["capabilities"][cap]["contract"])?]["body"]["provider"],
        )?;
        require(source.provider() == &provider, "fault-source-provider")?;
        self.resources.budget.charge(4)?;
        self.fault_sources.push(PendingFaultSource {
            site: site.into(),
            policy: text(&n["policy"])?.into(),
            metadata: source,
        });
        Ok(())
    }
    pub(super) fn fault_source_bindings(
        &mut self,
        revision: &str,
    ) -> Result<Vec<DeclaredFaultSourceUse>> {
        if self.fault_sources.is_empty() {
            return Ok(Vec::new());
        }
        self.resources.budget.charge(1)?;
        let revision: Arc<str> = revision.into();
        Ok(std::mem::take(&mut self.fault_sources)
            .into_iter()
            .map(|p| DeclaredFaultSourceUse {
                revision: revision.clone(),
                site: p.site,
                policy: p.policy,
                metadata: p.metadata,
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use choreoform_ir_probe_core::{Resource, digest};
    #[test]
    fn general_declaration_rows_share_budget_and_empty_legacy_adds_no_work() {
        let provider = serde_json::json!({"kind":"provider-contract","version":"0.1.0","id":"urn:test:general-provider","input":{"kind":"record","fields":{}},"output":{"kind":"record","fields":{}},"outcomes":{"done":true},"resultProtection":{"policy":"protect","sensitivity":"internal","purposes":{"test":true},"participants":{},"capabilities":{}},"observations":{"done":{"state":"succeeded","result":{"kind":"record","fields":{}},"outcome":"done"},"unknown":{"state":"unknown","result":null,"outcome":null}},"idempotency":"stable-key","effectClass":"irreversible"});
        let provider_raw = serde_json::to_vec(&provider).unwrap();
        let provider_rev = digest(&provider_raw);
        let source = serde_json::json!({"kind":"provider-fault-source","version":"0.1.0","id":"urn:test:general-source","profile":{"id":FAULT_SOURCE_PROFILE.0,"revision":FAULT_SOURCE_PROFILE.1},"provider":{"id":provider["id"],"revision":provider_rev},"emissions":{"uncertain":{"observations":{"unknown":true},"evidence":"unresolved"}}});
        let source_raw = serde_json::to_vec(&source).unwrap();
        let source_rev = digest(&source_raw);
        let supplied = [
            Resource {
                id: provider["id"].as_str().unwrap(),
                revision: &provider_rev,
                bytes: &provider_raw,
            },
            Resource {
                id: source["id"].as_str().unwrap(),
                revision: &source_rev,
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
        let metadata = resources
            .provider_fault_source(&Pin {
                id: source["id"].as_str().unwrap().into(),
                revision: source_rev.clone(),
            })
            .unwrap();
        let b = serde_json::json!({});
        let mut c = Check {
            b: &b,
            resources,
            cells: BTreeMap::new(),
            expressions: BTreeMap::new(),
            policy_reads: BTreeMap::new(),
            protection_refs: BTreeMap::new(),
            creation_reads: BTreeMap::new(),
            provider_refs: BTreeMap::new(),
            captures: Vec::new(),
            capture_types: BTreeSet::new(),
            fault_sources: Vec::new(),
        };
        c.resources.budget.work = 1_000_000;
        assert!(
            c.fault_source_bindings("revision-test-only")
                .unwrap()
                .is_empty()
        );
        assert_eq!(c.resources.budget.work, 1_000_000);
        c.fault_sources.push(PendingFaultSource {
            site: "site".into(),
            policy: "work".into(),
            metadata,
        });
        assert_eq!(
            c.fault_source_bindings("revision-test-only")
                .unwrap_err()
                .code,
            "budget"
        );
        assert_eq!(c.fault_sources.len(), 1);
    }
}
