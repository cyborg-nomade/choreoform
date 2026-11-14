// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Exact immutable original-input declarations, never stored request values.
use super::*;
use std::sync::Arc;
#[derive(Debug)]
pub struct DeclaredRetainedRequest {
    revision: Arc<str>,
    scope: String,
    origin: String,
    result: String,
    input_type: Arc<Type>,
}
impl DeclaredRetainedRequest {
    pub fn revision(&self) -> &str {
        &self.revision
    }
    pub fn scope(&self) -> &str {
        &self.scope
    }
    pub fn origin(&self) -> &str {
        &self.origin
    }
    pub fn result(&self) -> &str {
        &self.result
    }
    pub fn input_type(&self) -> &Type {
        &self.input_type
    }
    pub fn host_requirements(&self) -> &'static [&'static str] {
        &[
            "actual-original-accepted-typed-request-not-current-expression-reevaluation",
            "exact-definition-instance-owner-source-occurrence-input-value-revision-digest-key-effect-attempt-correlation",
            "fresh-occurrence-owned-readonly-storage-before-owner-disposal-and-no-sharing",
            "complete-original-input-policy-source-control-provider-provenance-retained",
            "current-source-target-data-management-authority-and-atomic-accepted-single-owner-handoff",
            "unchanged-logical-request-across-retries-changed-business-input-is-new-work",
            "live-cancellation-closure-cutoffs-subscriptions-and-all-obligation-settlement",
        ]
    }
}
pub(super) struct PendingRetainedRequest {
    scope: String,
    origin: String,
    result: String,
    input_type: Arc<Type>,
}
impl Check<'_, '_> {
    pub(super) fn retained_request_declarations(&mut self) -> Result<()> {
        if !object(&self.b["scopes"])?.values().any(|s| {
            s["retainedRequests"]
                .as_object()
                .is_some_and(|m| !m.is_empty())
        }) {
            return Ok(());
        }
        let advanced_continuation = continuation_core(self.b);
        let source_waits = if pre_capture_core(self.b) {
            Some(crate::source_wait::SourceWaits::new(
                self.b,
                &mut self.resources.budget,
            )?)
        } else {
            None
        };
        let mut children: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        let mut nested = BTreeSet::new();
        let mut handlers = BTreeSet::new();
        let mut creators = BTreeSet::new();
        let mut writers = BTreeSet::new();
        let mut captures: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for (id, s) in object(&self.b["scopes"])? {
            self.resources.budget.tick(0)?;
            if let Some(p) = s["parent"].as_str() {
                nested.insert(p);
                if advanced_continuation {
                    self.resources.budget.charge(2)?;
                    children.entry(p).or_default().insert(id.as_str());
                }
            }
            let policy = &self.b["policies"][text(&s["faults"])?]["body"];
            for target in object(&policy["handlers"])?.values() {
                self.resources.budget.tick(0)?;
                handlers.insert(text(target)?);
            }
        }
        for n in object(&self.b["nodes"])?.values() {
            self.resources.budget.tick(0)?;
            if matches!(n["kind"].as_str(), Some("invoke" | "repeat" | "fanout")) {
                creators.insert(text(&n["body"])?);
            }
            if n["kind"] == "split" {
                for target in object(&n["children"])?.values() {
                    self.resources.budget.tick(0)?;
                    creators.insert(text(target)?);
                }
            }
            for cell in object(&n["writes"])?.keys() {
                self.resources.budget.tick(0)?;
                writers.insert(cell.as_str());
            }
            if n["kind"] == "wait" && !n["capture"].is_null() {
                self.resources.budget.charge(2)?;
                captures
                    .entry(text(&n["scope"])?)
                    .or_default()
                    .push(text(&n["capture"]["origin"])?);
            }
        }
        for (scope, s) in object(&self.b["scopes"])? {
            self.resources.budget.tick(0)?;
            let requests = object(&s["retainedRequests"])?;
            if requests.is_empty() {
                continue;
            }
            let validate = (|| {
                require(requests.len() == 1, "retained-request-cardinality")?;
                let owner = text(&s["parent"]).map_err(|_| err("retained-request-owner"))?;
                let closure =
                    &self.b["policies"][text(&self.b["scopes"][owner]["closure"])?]["body"];
                require(
                    closure["mode"] == "reconcile" && closure["followup"] == *scope,
                    "retained-request-owner",
                )?;
                let own_closure = &self.b["policies"][text(&s["closure"])?]["body"];
                let tree_child = advanced_continuation
                    && own_closure["mode"] == "reconcile"
                    && children.get(scope.as_str()).is_some_and(|cs| {
                        cs.len() == 1
                            && own_closure["followup"]
                                .as_str()
                                .is_some_and(|target| cs.contains(target))
                    });
                require(
                    !creators.contains(scope.as_str())
                        && (!nested.contains(scope.as_str()) || tree_child),
                    "retained-request-creator",
                )?;
                require(
                    !handlers.contains(scope.as_str()) && s["faultInput"].is_null(),
                    "retained-request-handler",
                )?;
                require(
                    object(&s["inputs"])?.is_empty() && object(&s["outputs"])?.is_empty(),
                    "retained-request-ports",
                )?;
                require(
                    own_closure["mode"] == "terminal" || tree_child,
                    "retained-request-closure",
                )?;
                let (result, origin) = requests
                    .iter()
                    .next()
                    .ok_or_else(|| err("retained-request-cardinality"))?;
                let origin = text(origin)?;
                let n = &self.b["nodes"][origin];
                require(
                    n["kind"] == "activity" && n["mode"] == "capability" && n["scope"] == owner,
                    "retained-request-origin",
                )?;
                let entry = &self.b["nodes"][text(&s["entry"])?];
                let capturing_entry =
                    entry["kind"] == "wait" && entry["capture"]["origin"] == origin;
                if !capturing_entry {
                    require(
                        tree_child
                            && source_waits.is_some()
                            && matches!(
                                entry["kind"].as_str(),
                                Some("compute" | "decision" | "activity")
                            ),
                        "retained-request-capture",
                    )?;
                    source_waits.as_ref().unwrap().capture(
                        scope,
                        origin,
                        &mut self.resources.budget,
                    )?;
                }
                require(
                    captures
                        .get(scope.as_str())
                        .is_some_and(|cs| cs.iter().all(|c| *c == origin)),
                    "retained-request-capture",
                )?;
                require(
                    self.b["data"][result]["scope"] == *scope,
                    "retained-request-local",
                )?;
                require(
                    self.b["data"][result].get("initial").is_none()
                        && !writers.contains(result.as_str()),
                    "retained-request-readonly",
                )?;
                let cap = &self.b["capabilities"][text(&n["target"])?];
                let input = self.wrapped(&cap["input"])?;
                require(self.cells[result] == input, "retained-request-type")?;
                let provider =
                    Pin::decode(&self.b["policies"][text(&cap["contract"])?]["body"]["provider"])?;
                self.provider_protection(scope, &provider)?;
                self.resources.budget.charge(4)?;
                self.retained_requests.push(PendingRetainedRequest {
                    scope: scope.clone(),
                    origin: origin.into(),
                    result: result.clone(),
                    input_type: Arc::new(input),
                });
                Ok(())
            })();
            validate.map_err(|e: Diagnostic| {
                e.at_path(format!("/body/scopes/{scope}/retainedRequests"))
            })?;
        }
        Ok(())
    }
    pub(super) fn retained_request_bindings(
        &mut self,
        revision: &str,
    ) -> Result<Vec<DeclaredRetainedRequest>> {
        if self.retained_requests.is_empty() {
            return Ok(Vec::new());
        }
        self.resources
            .budget
            .charge(self.retained_requests.len() * 4 + 1)?;
        let revision: Arc<str> = revision.into();
        Ok(std::mem::take(&mut self.retained_requests)
            .into_iter()
            .map(|p| DeclaredRetainedRequest {
                revision: revision.clone(),
                scope: p.scope,
                origin: p.origin,
                result: p.result,
                input_type: p.input_type,
            })
            .collect())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn publication_is_cumulative_and_never_drains_a_partial_proof() {
        let b = serde_json::json!({});
        let mut c = Check {
            interference_selection: false,
            interference_choices: BTreeMap::new(),
            pending_interference: Vec::new(),
            compensation_identity: false,
            b: &b,
            resources: Resources::new(&[]).unwrap(),
            cells: BTreeMap::new(),
            expressions: BTreeMap::new(),
            policy_reads: BTreeMap::new(),
            protection_refs: BTreeMap::new(),
            creation_reads: BTreeMap::new(),
            provider_refs: BTreeMap::new(),
            captures: Vec::new(),
            capture_types: BTreeSet::new(),
            fault_sources: Vec::new(),
            fault_entries: Vec::new(),
            retained_requests: Vec::new(),
        };
        assert!(c.retained_request_bindings("revision").unwrap().is_empty());
        c.retained_requests.push(PendingRetainedRequest {
            scope: "followup".into(),
            origin: "request".into(),
            result: "original".into(),
            input_type: Arc::new(Type::Text),
        });
        while c.resources.budget.tick(0).is_ok() {}
        assert_eq!(
            c.retained_request_bindings("revision").unwrap_err().code,
            "budget"
        );
        assert_eq!(c.retained_requests.len(), 1);
    }
}
