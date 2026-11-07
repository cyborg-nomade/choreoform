// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use std::sync::Arc;

/// Exact declared relation only: no accepted observation, occurrence or value.
#[derive(Debug)]
pub struct DeclaredWaitCapture {
    revision: Arc<str>,
    site: String,
    origin: String,
    result: String,
    metadata: Arc<ProviderTerminalSource>,
}
impl DeclaredWaitCapture {
    pub fn revision(&self) -> &str {
        &self.revision
    }
    pub fn site(&self) -> &str {
        &self.site
    }
    pub fn origin(&self) -> &str {
        &self.origin
    }
    pub fn result(&self) -> &str {
        &self.result
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
    pub fn source_metadata(&self) -> &ProviderTerminalSource {
        &self.metadata
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "truthful-authenticated-provider-and-implemented-normalizer",
            "exact-live-origin-occurrence-attempt-request-key-input-and-observation-correlation",
            "current-match-authority-source-and-destination-access",
            "atomic-single-accepted-observation-capture-and-on-observation-completion",
            "no-write-on-unmatched-duplicate-stale-nonterminal-or-ordinary-cancelled-input",
            "retained-contradiction-history-and-success-cancellation-closure-cutoffs",
            "live-owned-storage-subscriptions-and-all-obligation-settlement",
        ]
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum CaptureTypeSubject {
    Policy(String),
    Cell(String),
}
pub(super) struct PendingCapture {
    site: String,
    origin: String,
    result: String,
    metadata: Arc<ProviderTerminalSource>,
}
impl Check<'_, '_> {
    pub(super) fn wait_capture(
        &mut self,
        site: &str,
        n: &Value,
        policy: &str,
    ) -> Result<BTreeSet<String>> {
        let c = &n["capture"];
        if c.is_null() {
            return Ok(BTreeSet::new());
        }
        let p = &self.b["policies"][policy];
        require(
            self.b["dialects"][text(&p["dialect"])?]["id"] == CORE_V5.0,
            "wait-capture-dialect",
        )?;
        require(p["body"]["timer"].is_null(), "wait-capture-timer-unproved")?;
        let scope = text(&n["scope"])?;
        let origin = self.visible(scope, "nodes", &c["origin"])?;
        let o = &self.b["nodes"][&origin];
        require(
            o["kind"] == "activity" && o["mode"] == "capability",
            "wait-capture-origin-unproved",
        )?;
        let cap = &self.b["capabilities"][text(&o["target"])?];
        let provider =
            Pin::decode(&self.b["policies"][text(&cap["contract"])?]["body"]["provider"])?;
        let source = self
            .resources
            .provider_terminal_source(&Pin::decode(&c["source"])?)?;
        require(source.provider() == &provider, "wait-capture-provider")?;
        let target = self.visible(scope, "data", &c["result"])?;
        // Exact immutable type equality only, never scope/authority/value truth.
        // Source aliases derive one schema per exact profile/provider pair.
        let policy_key = (
            CaptureTypeSubject::Policy(policy.into()),
            source.profile().clone(),
            source.provider().clone(),
        );
        if !self.capture_types.contains(&policy_key) {
            self.resources.budget.charge(3)?;
            let observed = self
                .resources
                .type_descriptor(&p["body"]["observationType"])?;
            self.resources.valid_type(source.observation_type(), 0)?;
            require(source.observation_type() == &observed, "wait-capture-type")?;
            self.capture_types.insert(policy_key);
        }
        let cell_key = (
            CaptureTypeSubject::Cell(target.clone()),
            source.profile().clone(),
            source.provider().clone(),
        );
        if !self.capture_types.contains(&cell_key) {
            self.resources.budget.charge(3)?;
            self.resources.valid_type(source.observation_type(), 0)?;
            require(
                source.observation_type() == &self.cells[&target],
                "wait-capture-type",
            )?;
            self.capture_types.insert(cell_key);
        }
        require(
            names(&n["writes"])? == BTreeSet::from([target.clone()]),
            "wait-capture-writes",
        )?;
        self.flow(
            source.source_protection(),
            &self.b["data"][&target]["protection"],
        )?;
        let mut policies = self.provider_protection(scope, source.provider())?;
        policies.extend(self.data_protection(&target)?);
        self.resources.budget.charge(5)?;
        self.captures.push(PendingCapture {
            site: site.into(),
            origin,
            result: target,
            metadata: source,
        });
        Ok(policies)
    }
    pub(super) fn capture_bindings(&mut self, revision: &str) -> Result<Vec<DeclaredWaitCapture>> {
        if self.captures.is_empty() {
            return Ok(Vec::new());
        }
        self.resources.budget.charge(1)?;
        let revision: Arc<str> = revision.into();
        let rows = std::mem::take(&mut self.captures)
            .into_iter()
            .map(|p| DeclaredWaitCapture {
                revision: revision.clone(),
                site: p.site,
                origin: p.origin,
                result: p.result,
                metadata: p.metadata,
            })
            .collect();
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use choreoform_ir_probe_core::{Resource, digest};
    #[test]
    fn declaration_rows_obey_shared_remaining_work_without_changing_legacy_cost() {
        let corpus: Value =
            serde_json::from_slice(include_bytes!("../../tests/conformance/wait-capture.json"))
                .unwrap();
        let provider = &corpus["resources"]["provider"];
        let source = &corpus["resources"]["source"];
        let provider_raw = serde_json::to_vec(provider).unwrap();
        let source_raw = serde_json::to_vec(source).unwrap();
        let provider_rev = digest(&provider_raw);
        let source_rev = digest(&source_raw);
        let supplied = [
            Resource {
                id: text(&provider["id"]).unwrap(),
                revision: &provider_rev,
                bytes: &provider_raw,
            },
            Resource {
                id: text(&source["id"]).unwrap(),
                revision: &source_rev,
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
        let metadata = resources
            .provider_terminal_source(&Pin {
                id: text(&source["id"]).unwrap().into(),
                revision: source_rev.clone(),
            })
            .unwrap();
        let mut c = Check {
            b: &corpus["definition"]["body"],
            resources,
            cells: BTreeMap::new(),
            expressions: BTreeMap::new(),
            policy_reads: BTreeMap::new(),
            protection_refs: BTreeMap::new(),
            creation_reads: BTreeMap::new(),
            provider_refs: BTreeMap::new(),
            captures: Vec::new(),
            capture_types: BTreeSet::new(),
        };
        c.resources.budget.work = 1_000_000;
        assert!(c.capture_bindings("revision-test-only").unwrap().is_empty());
        assert_eq!(c.resources.budget.work, 1_000_000);
        c.captures.push(PendingCapture {
            site: "capture_wait".into(),
            origin: "origin".into(),
            result: "captured".into(),
            metadata,
        });
        assert_eq!(
            c.capture_bindings("revision-test-only").unwrap_err().code,
            "budget"
        );
        assert_eq!(c.captures.len(), 1); // No partial declaration report was returned.
    }
}
