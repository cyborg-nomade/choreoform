// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Declared immutable local history target; never an initialized entry value.
use super::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct DeclaredFaultEntry {
    revision: Arc<str>,
    handler: String,
    result: String,
    metadata: Arc<ProviderFaultHistory>,
}
impl DeclaredFaultEntry {
    pub fn revision(&self) -> &str {
        &self.revision
    }
    pub fn handler(&self) -> &str {
        &self.handler
    }
    pub fn result(&self) -> &str {
        &self.result
    }
    pub fn history_metadata(&self) -> &ProviderFaultHistory {
        &self.metadata
    }
    pub fn host_requirements(&self) -> &'static [&'static str] {
        &[
            "actual-authenticated-supported-non-engine-fault-and-original-evidence",
            "exact-definition-instance-scope-node-attempt-input-cause-request-key-effect-correlation",
            "actual-nearest-live-handler-selection-and-failed-selection-exclusion",
            "complete-source-input-fault-existence-control-creator-owner-provenance",
            "current-source-work-access-management-authority-and-pre-entry-access-checks",
            "fresh-single-occurrence-readonly-history-storage-and-conditional-entry-proof",
            "monotonic-cancellation-permitted-owned-continuation-handoff-and-all-settlement",
        ]
    }
}
pub(super) struct PendingFaultEntry {
    handler: String,
    result: String,
    metadata: Arc<ProviderFaultHistory>,
}
impl Check<'_, '_> {
    pub(super) fn fault_entry_declarations(&mut self) -> Result<()> {
        if !object(&self.b["scopes"])?
            .values()
            .any(|s| !s["faultInput"].is_null())
        {
            return Ok(());
        }
        let mut handlers = BTreeSet::new();
        let mut creators = BTreeSet::new();
        let mut writers = BTreeSet::new();
        for s in object(&self.b["scopes"])?.values() {
            self.resources.budget.tick(0)?;
            let fault = &self.b["policies"][text(&s["faults"])?]["body"];
            for child in object(&fault["handlers"])?.values() {
                self.resources.budget.tick(0)?;
                handlers.insert(text(child)?);
            }
            let closure = &self.b["policies"][text(&s["closure"])?]["body"];
            if closure["mode"] == "reconcile" {
                self.resources.budget.tick(0)?;
                creators.insert(text(&closure["followup"])?);
            }
        }
        for n in object(&self.b["nodes"])?.values() {
            self.resources.budget.tick(0)?;
            if matches!(n["kind"].as_str(), Some("invoke" | "repeat" | "fanout")) {
                creators.insert(text(&n["body"])?);
            }
            if n["kind"] == "split" {
                for child in object(&n["children"])?.values() {
                    self.resources.budget.tick(0)?;
                    creators.insert(text(child)?);
                }
            }
            for cell in object(&n["writes"])?.keys() {
                self.resources.budget.tick(0)?;
                writers.insert(cell.as_str());
            }
        }
        let mut reserved = BTreeSet::new();
        for (scope, s) in object(&self.b["scopes"])? {
            self.resources.budget.tick(0)?;
            if s["faultInput"].is_null() {
                continue;
            }
            let result = text(&s["faultInput"]["result"])?;
            let validate = (|| {
                self.resources.budget.charge(5)?;
                require(
                    self.b["root"] != *scope && handlers.contains(scope.as_str()),
                    "fault-entry-handler",
                )?;
                require(!creators.contains(scope.as_str()), "fault-entry-creator")?;
                require(
                    object(&s["inputs"])?.is_empty() && object(&s["outputs"])?.is_empty(),
                    "handler-ports",
                )?;
                require(
                    self.b["data"][result]["scope"] == *scope,
                    "fault-entry-owner",
                )?;
                require(
                    self.b["data"][result].get("initial").is_none() && !writers.contains(result),
                    "fault-entry-readonly",
                )?;
                require(reserved.insert(result), "fault-entry-competing-binding")?;
                let metadata = self
                    .resources
                    .provider_fault_history(&Pin::decode(&s["faultInput"]["history"])?)?;
                require(
                    &self.cells[result] == metadata.history_type(),
                    "fault-entry-type",
                )?;
                self.provider_protection(scope, metadata.source_metadata().provider())?;
                self.fault_entries.push(PendingFaultEntry {
                    handler: scope.clone(),
                    result: result.into(),
                    metadata,
                });
                Ok(())
            })();
            validate
                .map_err(|e: Diagnostic| e.at_path(format!("/body/scopes/{scope}/faultInput")))?;
        }
        Ok(())
    }
    pub(super) fn fault_entry_bindings(
        &mut self,
        revision: &str,
    ) -> Result<Vec<DeclaredFaultEntry>> {
        if self.fault_entries.is_empty() {
            return Ok(Vec::new());
        }
        self.resources.budget.charge(1)?;
        let revision: Arc<str> = revision.into();
        Ok(std::mem::take(&mut self.fault_entries)
            .into_iter()
            .map(|p| DeclaredFaultEntry {
                revision: revision.clone(),
                handler: p.handler,
                result: p.result,
                metadata: p.metadata,
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use choreoform_ir_probe_core::digest;
    #[test]
    fn empty_legacy_adds_no_work_and_nonempty_publication_refuses_before_drain() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let manifest: Value = serde_json::from_slice(include_bytes!(
            "../../../../docs/benchmarks/rp01/manifest.json"
        ))
        .unwrap();
        let mut owned: Vec<_> = manifest["resources"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                (
                    Pin {
                        id: v["id"].as_str().unwrap().into(),
                        revision: v["revision"].as_str().unwrap().into(),
                    },
                    std::fs::read(root.join(v["path"].as_str().unwrap())).unwrap(),
                )
            })
            .collect();
        for(p,bytes)in[
            (FAULT_HISTORY_PROFILE,include_bytes!("../../../../docs/ir/contracts/sha256-2f580f46eb6ef9b7fff42906f6b0b5cafc6c5fa71ae14365828935bfe2da070c.txt").as_slice()),
            (FAULT_HISTORY_TYPES,include_bytes!("../../../../docs/ir/contracts/sha256-a2e177ce1dd4bad5b9cd68b74e6da7749ec2b04b68a39dae1bd72729e2aae7a7.txt")),
        ]{owned.push((Pin{id:p.0.into(),revision:p.1.into()},bytes.to_vec()));}
        let source = owned
            .iter()
            .find(|(p, _)| p.id.ends_with("payment-faults:0.1.0"))
            .unwrap()
            .0
            .clone();
        let raw=serde_json::to_vec(&serde_json::json!({"kind":"provider-fault-history","version":"0.1.0","id":"urn:test:entry-budget","profile":{"id":FAULT_HISTORY_PROFILE.0,"revision":FAULT_HISTORY_PROFILE.1},"source":{"id":source.id,"revision":source.revision}})).unwrap();
        let pin = Pin {
            id: "urn:test:entry-budget".into(),
            revision: digest(&raw),
        };
        owned.push((pin.clone(), raw));
        let supplied: Vec<_> = owned
            .iter()
            .map(|(p, bytes)| Resource {
                id: &p.id,
                revision: &p.revision,
                bytes,
            })
            .collect();
        let mut resources = Resources::new(&supplied).unwrap();
        let metadata = resources.provider_fault_history(&pin).unwrap();
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
            fault_sources: Vec::new(),
            fault_entries: Vec::new(),
            retained_requests: Vec::new(),
            capture_types: BTreeSet::new(),
        };
        c.resources.budget.work = 1_000_000;
        assert!(
            c.fault_entry_bindings("revision-test-only")
                .unwrap()
                .is_empty()
        );
        assert_eq!(c.resources.budget.work, 1_000_000);
        for i in 0..2 {
            c.fault_entries.push(PendingFaultEntry {
                handler: format!("handler-{i}"),
                result: format!("history-{i}"),
                metadata: metadata.clone(),
            });
        }
        assert_eq!(
            c.fault_entry_bindings("revision-test-only")
                .unwrap_err()
                .code,
            "budget"
        );
        assert_eq!(c.fault_entries.len(), 2);
        c.resources.budget.work = 999_999;
        let rows = c.fault_entry_bindings("revision-test-only").unwrap();
        assert_eq!(rows.len(), 2);
        assert!(std::ptr::eq(
            rows[0].revision().as_ptr(),
            rows[1].revision().as_ptr()
        ));
        assert!(std::ptr::eq(
            rows[0].history_metadata(),
            rows[1].history_metadata()
        ));
    }
}
