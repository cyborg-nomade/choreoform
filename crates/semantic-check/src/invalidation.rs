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

// Shared immutable analysis is private: advisory data is never constructed as a
// strict InvalidationReport, even temporarily.
#[derive(Debug)]
struct Analysis {
    protection: ProtectionReport,
    checked: BTreeSet<String>,
    uncovered: BTreeSet<String>,
    obligations: BTreeSet<InvalidationObligation>,
    historical: BTreeSet<HistoricalOrigin>,
    scope_contexts: crate::protection::DependencyMap,
}

/// Revision-bound checked declarations, without values or an executable plan.
#[derive(Debug)]
pub struct InvalidationReport {
    analysis: Analysis,
}
impl InvalidationReport {
    pub(crate) fn scope_contexts(&self) -> &crate::protection::DependencyMap {
        &self.analysis.scope_contexts
    }
    pub fn revision(&self) -> &str {
        self.analysis.protection.revision()
    }
    pub fn protection(&self) -> &ProtectionReport {
        &self.analysis.protection
    }
    pub fn checked_nodes(&self) -> &BTreeSet<String> {
        &self.analysis.checked
    }
    pub fn uncovered_nodes(&self) -> &BTreeSet<String> {
        &self.analysis.uncovered
    }
    pub fn obligations(&self) -> &BTreeSet<InvalidationObligation> {
        &self.analysis.obligations
    }
    pub fn historical_origins(&self) -> &BTreeSet<HistoricalOrigin> {
        &self.analysis.historical
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        REMAINING
    }
}

/// Ordinary requirements only. Missing links may exist; this is not a passed
/// invalidation stage and cannot be converted into an InvalidationReport.
///
/// ```compile_fail
/// use choreoform_semantic_check::{InvalidationReport, InvalidationRequirements};
/// fn needs_checked(_: &InvalidationReport) {}
/// fn cannot_promote(advice: &InvalidationRequirements) {
///     needs_checked(advice);
/// }
/// ```
#[derive(Debug)]
pub struct InvalidationRequirements {
    analysis: Analysis,
    missing: BTreeSet<InvalidationObligation>,
}
impl InvalidationRequirements {
    pub fn revision(&self) -> &str {
        self.analysis.protection.revision()
    }
    pub fn protection(&self) -> &ProtectionReport {
        &self.analysis.protection
    }
    pub fn analyzed_nodes(&self) -> &BTreeSet<String> {
        &self.analysis.checked
    }
    pub fn uncovered_nodes(&self) -> &BTreeSet<String> {
        &self.analysis.uncovered
    }
    pub fn obligations(&self) -> &BTreeSet<InvalidationObligation> {
        &self.analysis.obligations
    }
    pub fn missing(&self) -> &BTreeSet<InvalidationObligation> {
        &self.missing
    }
    pub fn historical_origins(&self) -> &BTreeSet<HistoricalOrigin> {
        &self.analysis.historical
    }
    /// Required even when the current missing set is empty; no checked report is
    /// inferred from this advisory object or reused after a definition edit.
    pub fn required_check(&self) -> &'static str {
        "check-invalidation-at-final-definition-revision"
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        REMAINING
    }
}
const REMAINING: &[&str] = &[
    "parallel-and-exceptional-availability-proofs",
    "exceptional-and-observable-channel-information-flow",
    "current-principal-purpose-capability-and-policy-enforcement",
    "concurrency-and-irreversible-race-safety-proofs",
    "exceptional-and-non-data-invalidation-and-lifecycle-enforcement",
    "occurrence-bound-ordinary-closure-and-snapshot-enforcement",
    "host-trust-and-atomic-runtime-enforcement",
    "full-benchmark-and-conformance-evidence",
];

enum Coverage {
    Required,
    Advisory,
}

/// Derive all ordinary requirements or refuse an unproved context. This never
/// edits the envelope or declares its coverage complete/passed.
pub fn derive_invalidation_requirements(
    envelope: &DefinitionEnvelope,
    supplied: &[Resource<'_>],
) -> Result<InvalidationRequirements> {
    let (analysis, missing) = analyze(envelope, supplied, Coverage::Advisory)?;
    Ok(InvalidationRequirements { analysis, missing })
}

/// Require visible pre-result dependencies to declare invalidation of visited
/// decision/work/wait sites. Conservative refusal can mean unproved, not invalid.
pub fn check_invalidation(
    envelope: &DefinitionEnvelope,
    supplied: &[Resource<'_>],
) -> Result<InvalidationReport> {
    let (analysis, _) = analyze(envelope, supplied, Coverage::Required)?;
    Ok(InvalidationReport { analysis })
}

fn analyze(
    envelope: &DefinitionEnvelope,
    supplied: &[Resource<'_>],
    coverage: Coverage,
) -> Result<(Analysis, BTreeSet<InvalidationObligation>)> {
    let (protection, dependencies) = crate::protection::analyze(envelope, supplied, true)?;
    let b = &envelope.document()["body"];
    let mut budget = Budget::default();
    let (obligations, missing) = requirements(b, &dependencies.nodes, coverage, &mut budget)?;
    let checked = dependencies.nodes.into_keys().collect::<BTreeSet<_>>();
    let mut uncovered = BTreeSet::new();
    for (id, n) in object(&b["nodes"])? {
        budget.tick(0)?;
        if matches!(text(&n["kind"])?, "decision" | "activity" | "wait") && !checked.contains(id) {
            uncovered.insert(id.clone());
        }
    }
    Ok((
        Analysis {
            protection,
            checked,
            uncovered,
            obligations,
            scope_contexts: dependencies.scopes,
            historical: dependencies.historical,
        },
        missing,
    ))
}

/// Reuse strict coverage for explicit conditional entries; never advisory promotion.
pub(crate) fn require_entries(
    b: &Value,
    nodes: &crate::protection::DependencyMap,
    budget: &mut Budget,
) -> Result<BTreeSet<InvalidationObligation>> {
    requirements(b, nodes, Coverage::Required, budget)
        .map(|(required, _)| required)
        .map_err(|mut e| {
            e.code = match e.code {
                "invalidation-coverage" => "owned-local-invalidation-coverage",
                "invalidation-context-unproved" => "owned-local-invalidation-context-unproved",
                other => other,
            };
            e
        })
}

fn requirements(
    b: &Value,
    nodes: &crate::protection::DependencyMap,
    coverage: Coverage,
    budget: &mut Budget,
) -> Result<(
    BTreeSet<InvalidationObligation>,
    BTreeSet<InvalidationObligation>,
)> {
    let mut declared = BTreeMap::new();
    for (id, d) in object(&b["data"])? {
        budget.charge(object(&d["invalidates"])?.len() + 1)?;
        declared.insert(id.as_str(), names(&d["invalidates"])?);
    }
    let mut obligations = BTreeSet::new();
    let mut missing = BTreeSet::new();
    for (site, cells) in nodes {
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
            let obligation = InvalidationObligation {
                site: site.clone(),
                cell: cell.clone(),
            };
            if !declared[cell.as_str()].contains(site) {
                if matches!(coverage, Coverage::Required) {
                    return Err(Diagnostic {
                        code: "invalidation-coverage",
                        path: format!("/body/data/{cell}/invalidates"),
                    });
                }
                budget.charge(2)?;
                missing.insert(obligation.clone());
            }
            obligations.insert(obligation);
        }
    }
    Ok((obligations, missing))
}
