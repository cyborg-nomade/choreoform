// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Conditional finite ownership declarations; never values, grants or receipts.
use super::*;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};
use std::sync::Arc;
mod index;
pub(crate) use index::Candidates;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Entry {
    pub owner: String,
    pub scope: String,
    pub source: String,
    pub request: String,
    pub conditional_source: bool,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Level {
    entry: Entry,
    closure: String,
    query: Option<String>,
    child: Option<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Template {
    owner: String,
    levels: Vec<Level>,
}
/// One creator and immutable declaration template, not one live occurrence.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnedContinuation {
    creator: String,
    template: Arc<Template>,
}
impl OwnedContinuation {
    pub fn creator(&self) -> &str {
        &self.creator
    }
    pub fn owner(&self) -> &str {
        &self.template.owner
    }
    pub fn scopes(&self) -> impl Iterator<Item = OwnedContinuationScope<'_>> {
        self.template
            .levels
            .iter()
            .map(|level| OwnedContinuationScope { level })
    }
}
/// Borrowed source/query/child relation under a distinct conditional proof.
pub struct OwnedContinuationScope<'a> {
    level: &'a Level,
}
impl OwnedContinuationScope<'_> {
    pub fn owner(&self) -> &str {
        &self.level.entry.owner
    }
    pub fn scope(&self) -> &str {
        &self.level.entry.scope
    }
    pub fn source(&self) -> &str {
        &self.level.entry.source
    }
    pub fn request(&self) -> &str {
        &self.level.entry.request
    }
    pub fn conditional_source(&self) -> bool {
        self.level.entry.conditional_source
    }
    /// The source owner's incoming reconcile policy, not this body's own closure.
    pub fn closure(&self) -> &str {
        &self.level.closure
    }
    pub fn query(&self) -> Option<&str> {
        self.level.query.as_deref()
    }
    pub fn child(&self) -> Option<&str> {
        self.level.child.as_deref()
    }
}
/// A mandatory future host gate at a declaration, never an accepted transition.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnedContinuationGate {
    kind: &'static str,
    owner: String,
    source: Option<String>,
    receiver: Option<String>,
    management_actor: String,
}
impl OwnedContinuationGate {
    pub fn kind(&self) -> &str {
        self.kind
    }
    pub fn owner(&self) -> &str {
        &self.owner
    }
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }
    pub fn receiver(&self) -> Option<&str> {
        self.receiver.as_deref()
    }
    pub fn management_actor(&self) -> &str {
        &self.management_actor
    }
    pub fn host_requirements(&self) -> &'static [&'static str] {
        &[
            "actual-current-data-work-capability-and-source-receiver-management-authority",
            "open-noncancelled-registration-and-monotonic-cancellation-closure-cutoff",
            "exact-definition-instance-owner-source-occurrence-request-value-input-revision-digest-effect-key-attempt-correlation",
            "fresh-occurrence-owned-immutable-storage-and-authenticated-complete-provenance",
            "receiver-acceptance-and-atomic-single-accountable-owner-handoff",
            "failed-or-missing-acceptance-keeps-original-owner-accountable-and-closure-pending",
            "all-actually-activated-transitive-effects-subscriptions-children-settled-before-terminal-disposal",
            "late-duplicate-stale-contradictory-history-no-parent-resumption-or-terminal-reopening",
        ]
    }
}
#[derive(Debug)]
pub struct OwnedContinuationReport {
    revision: String,
    continuations: BTreeSet<OwnedContinuation>,
    nodes: BTreeSet<String>,
    scopes: BTreeSet<String>,
    gates: BTreeSet<OwnedContinuationGate>,
    transfers: usize,
    accesses: BTreeSet<AccessObligation>,
    obligations: BTreeSet<InvalidationObligation>,
    provenance: BTreeSet<RequiredProtectionProvenance>,
}
impl OwnedContinuationReport {
    pub fn revision(&self) -> &str {
        &self.revision
    }
    pub fn continuations(&self) -> &BTreeSet<OwnedContinuation> {
        &self.continuations
    }
    pub fn checked_nodes(&self) -> &BTreeSet<String> {
        &self.nodes
    }
    pub fn checked_scopes(&self) -> &BTreeSet<String> {
        &self.scopes
    }
    pub fn gates(&self) -> &BTreeSet<OwnedContinuationGate> {
        &self.gates
    }
    pub fn transfers(&self) -> usize {
        self.transfers
    }
    pub fn accesses(&self) -> &BTreeSet<AccessObligation> {
        &self.accesses
    }
    pub fn obligations(&self) -> &BTreeSet<InvalidationObligation> {
        &self.obligations
    }
    pub fn required_provenance(&self) -> &BTreeSet<RequiredProtectionProvenance> {
        &self.provenance
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "conditional-declarations-only-no-actual-events-values-storage-grants-or-accepted-gates",
            "all-registration-handoff-ordinary-closure-terminal-settlement-host-requirements",
            "actual-supported-provider-behavior-current-authority-and-complete-source-control-provenance",
            "exact-occurrence-request-key-effect-correlation-and-atomic-live-ledger-accounting",
            "ordinary-only-history-entry-contexts-and-unhandled-unsupported-fault-obligations",
            "observable-channels-general-exceptional-protection-command-compensation-and-richer-ownership",
            "no-runtime-liveness-termination-or-eventual-settlement-guarantee",
            "full-validator-benchmark-source-native-editing-and-external-phase-evidence",
        ]
    }
}
fn at(code: &'static str, scope: &str) -> Diagnostic {
    Diagnostic {
        code,
        path: format!("/body/scopes/{scope}"),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub(crate) type Artifacts = Vec<(String, String, Vec<u8>)>;
    pub(crate) fn fixture() -> (DefinitionEnvelope, Artifacts) {
        let v: Value = serde_json::from_slice(include_bytes!(
            "../tests/conformance/owned-readonly-definition.json"
        ))
        .unwrap();
        let doc = &v["definition"];
        let e = DefinitionEnvelope::from_parts_versioned(
            "0.4.0",
            doc["body"].clone(),
            doc["annotations"].clone(),
        )
        .unwrap();
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let artifacts = v["normative"]
            .as_array()
            .unwrap()
            .iter()
            .chain(v["resources"].as_array().unwrap())
            .map(|r| {
                let id = r["id"].as_str().unwrap().to_owned();
                let revision = r["revision"].as_str().unwrap().to_owned();
                let bytes = if let Some(raw) = r["bytes"].as_str() {
                    raw.as_bytes().to_vec()
                } else {
                    std::fs::read(root.join(format!(
                        "docs/ir/contracts/{}.txt",
                        revision.replace(':', "-")
                    )))
                    .unwrap()
                };
                (id, revision, bytes)
            })
            .collect();
        (e, artifacts)
    }
    pub(crate) fn resources(owned: &Artifacts) -> Vec<Resource<'_>> {
        owned
            .iter()
            .map(|(id, revision, bytes)| Resource {
                id,
                revision,
                bytes,
            })
            .collect()
    }
}
