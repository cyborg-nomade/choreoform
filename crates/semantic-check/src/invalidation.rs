// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Conservative ordinary invalidation declarations, never runtime sufficiency.
use super::*;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvalidationObligation {
    site: String,
    cell: String,
}
impl InvalidationObligation {
    pub fn site(&self) -> &str {
        &self.site
    }
    pub fn cell(&self) -> &str {
        &self.cell
    }
}

/// A conditional old value/control influence, not an actual completion receipt.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HistoricalOrigin {
    pub(crate) site: String,
    pub(crate) cell: String,
    pub(crate) boundary: String,
    pub(crate) scope: String,
    pub(crate) closure: String,
    pub(crate) context_scope: Option<String>,
}
impl HistoricalOrigin {
    pub fn site(&self) -> &str {
        &self.site
    }
    pub fn cell(&self) -> &str {
        &self.cell
    }
    pub fn boundary(&self) -> &str {
        &self.boundary
    }
    pub fn scope(&self) -> &str {
        &self.scope
    }
    pub fn closure(&self) -> &str {
        &self.closure
    }
    /// Present when a creator carries this historical influence into a scope.
    pub fn context_scope(&self) -> Option<&str> {
        self.context_scope.as_deref()
    }
    pub fn condition(&self) -> &'static str {
        "ordinary-completion-with-no-declared-descendant-writer"
    }
}

/// Revision-bound declaration evidence, without values or an executable plan.
#[derive(Debug)]
pub struct InvalidationReport {
    protection: ProtectionReport,
    checked: BTreeSet<String>,
    uncovered: BTreeSet<String>,
    obligations: BTreeSet<InvalidationObligation>,
    historical: BTreeSet<HistoricalOrigin>,
    scope_contexts: crate::protection::DependencyMap,
}
impl InvalidationReport {
    pub(crate) fn scope_contexts(&self) -> &crate::protection::DependencyMap {
        &self.scope_contexts
    }
    pub fn revision(&self) -> &str {
        self.protection.revision()
    }
    pub fn protection(&self) -> &ProtectionReport {
        &self.protection
    }
    pub fn checked_nodes(&self) -> &BTreeSet<String> {
        &self.checked
    }
    /// Consequential declarations without an invented ordinary entry context.
    pub fn uncovered_nodes(&self) -> &BTreeSet<String> {
        &self.uncovered
    }
    pub fn obligations(&self) -> &BTreeSet<InvalidationObligation> {
        &self.obligations
    }
    pub fn historical_origins(&self) -> &BTreeSet<HistoricalOrigin> {
        &self.historical
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "parallel-and-exceptional-availability-proofs",
            "exceptional-and-observable-channel-information-flow",
            "current-principal-purpose-capability-and-policy-enforcement",
            "concurrency-and-irreversible-race-safety-proofs",
            "exceptional-and-non-data-invalidation-and-lifecycle-enforcement",
            "occurrence-bound-ordinary-closure-and-snapshot-enforcement",
            "host-trust-and-atomic-runtime-enforcement",
            "full-benchmark-and-conformance-evidence",
        ]
    }
}

/// Require visible pre-result dependencies to declare invalidation of visited
/// decision/work/wait sites. Conservative refusal can mean unproved, not invalid.
pub fn check_invalidation(
    envelope: &DefinitionEnvelope,
    supplied: &[Resource<'_>],
) -> Result<InvalidationReport> {
    let (protection, dependencies) = crate::protection::analyze(envelope, supplied, true)?;
    let b = &envelope.document()["body"];
    let mut budget = Budget::default();
    let mut declared = BTreeMap::new();
    for (id, d) in object(&b["data"])? {
        budget.charge(object(&d["invalidates"])?.len() + 1)?;
        declared.insert(id.as_str(), names(&d["invalidates"])?);
    }
    let mut obligations = BTreeSet::new();
    for (site, cells) in &dependencies.nodes {
        budget.tick(0)?;
        let mut ancestors = BTreeSet::new();
        let mut scope = Some(text(&b["nodes"][site]["scope"])?);
        while let Some(id) = scope {
            budget.tick(ancestors.len())?;
            require(ancestors.insert(id), "scope-cycle")?;
            scope = b["scopes"][id]["parent"].as_str();
        }
        for cell in cells {
            budget.tick(0)?;
            if !ancestors.contains(text(&b["data"][cell]["scope"])?) {
                return Err(Diagnostic {
                    code: "invalidation-context-unproved",
                    path: format!("/body/nodes/{site}"),
                });
            }
            if !declared[cell.as_str()].contains(site) {
                return Err(Diagnostic {
                    code: "invalidation-coverage",
                    path: format!("/body/data/{cell}/invalidates"),
                });
            }
            obligations.insert(InvalidationObligation {
                site: site.clone(),
                cell: cell.clone(),
            });
        }
    }
    let checked = dependencies.nodes.into_keys().collect::<BTreeSet<_>>();
    let mut uncovered = BTreeSet::new();
    for (id, n) in object(&b["nodes"])? {
        budget.tick(0)?;
        if matches!(text(&n["kind"])?, "decision" | "activity" | "wait") && !checked.contains(id) {
            uncovered.insert(id.clone());
        }
    }
    Ok(InvalidationReport {
        protection,
        checked,
        uncovered,
        obligations,
        scope_contexts: dependencies.scopes,
        historical: dependencies.historical,
    })
}
