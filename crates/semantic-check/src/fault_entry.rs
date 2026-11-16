// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Conditional local entry under authenticated-event/provenance host premises.
use super::*;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};

#[derive(Debug)]
pub struct FaultEntryReport {
    origins: FaultOriginReport,
    covered: BTreeSet<String>,
    uncovered: BTreeSet<String>,
    nodes: BTreeSet<String>,
    scopes: BTreeSet<String>,
    transfers: usize,
    accesses: BTreeSet<AccessObligation>,
    obligations: BTreeSet<InvalidationObligation>,
    provenance: BTreeSet<RequiredProtectionProvenance>,
}
impl FaultEntryReport {
    pub fn revision(&self) -> &str {
        self.origins.revision()
    }
    pub fn origins(&self) -> &FaultOriginReport {
        &self.origins
    }
    pub fn covered_entries(&self) -> &BTreeSet<String> {
        &self.covered
    }
    pub fn uncovered_entries(&self) -> &BTreeSet<String> {
        &self.uncovered
    }
    /// Conditional handler bodies only; not ordinary or legacy exceptional sets.
    pub fn checked_nodes(&self) -> &BTreeSet<String> {
        &self.nodes
    }
    pub fn checked_scopes(&self) -> &BTreeSet<String> {
        &self.scopes
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
            "conditional-only-no-actual-handler-event-storage-value-or-current-grant",
            "all-prior-static-control-protection-invalidation-concurrency-timing-and-routing-obligations",
            "all-bound-history-source-actual-emission-correlation-selection-and-authority-requirements",
            "authenticated-complete-source-control-provenance-retained-at-actual-fresh-readonly-entry",
            "uncovered-and-unsupported-entry-contexts-have-no-body-proof",
            "general-exceptional-source-context-protection-invalidation-concurrency-and-lifetime",
            "monotonic-cancellation-permitted-owned-continuation-cutoff-handoff-all-settlement",
            "full-validator-benchmark-authoring-visual-host-user-browser-independent-phase-evidence",
        ]
    }
}
fn at(code: &'static str, scope: &str) -> Diagnostic {
    Diagnostic {
        code,
        path: format!("/body/scopes/{scope}/faultInput"),
    }
}

pub fn check_fault_entries(
    envelope: &DefinitionEnvelope,
    supplied: &[Resource<'_>],
) -> Result<FaultEntryReport> {
    let origins = check_fault_origins(envelope, supplied)?;
    let ordinary = origins
        .faults()
        .timing()
        .concurrency()
        .invalidation()
        .protection();
    let bindings = origins.faults().static_report().fault_entries();
    let b = &envelope.document()["body"];
    let mut budget = Budget::default();
    let mut by_handler = BTreeMap::new();
    for row in bindings {
        budget.charge(2)?;
        by_handler.insert(row.handler(), row);
    }
    // An unexplored scope has no complete active-stack premise. Do not drop
    // its possible incoming relation merely because the abstract walk omitted
    // that activation (including future surviving or exceptional contexts).
    for site in origins.sites().filter(|s| !s.scope_explored()) {
        let mut cursor = Some(site.scope());
        let mut depth = 0;
        while let Some(scope) = cursor {
            budget.tick(depth)?;
            depth += 1;
            let handlers = &b["policies"][text(&b["scopes"][scope]["faults"])?]["body"]["handlers"];
            for (fault, target) in object(handlers)? {
                budget.tick(0)?;
                let handler = text(target)?;
                if by_handler.contains_key(handler)
                    && site
                        .source_use()
                        .is_none_or(|u| u.source_metadata().emissions().contains_key(fault))
                {
                    return Err(at("fault-entry-source-control-unproved", handler));
                }
            }
            cursor = b["scopes"][scope]["parent"].as_str();
        }
    }
    let mut owners: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut sources: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for route in origins.routes() {
        budget.tick(0)?;
        let handler = route.selected().handler();
        let Some(binding) = by_handler.get(handler) else {
            continue;
        };
        let source = route
            .source_use()
            .ok_or_else(|| at("fault-entry-origin-unproved", handler))?;
        require(
            source.source() == binding.history_metadata().source_metadata().pin(),
            "fault-entry-source",
        )
        .map_err(|e| e.at_path(format!("/body/scopes/{handler}/faultInput")))?;
        if !route.ordinary_reachable()
            || route.active_before().next().is_some()
            || !ordinary.control().covered_nodes().contains(route.site())
        {
            return Err(at("fault-entry-source-control-unproved", handler));
        }
        budget.charge(4)?;
        owners
            .entry(handler.into())
            .or_default()
            .insert(route.scope().into());
        sources
            .entry(handler.into())
            .or_default()
            .insert(route.site().into());
    }
    budget.charge(bindings.len() + owners.len())?;
    let covered: BTreeSet<_> = owners.keys().cloned().collect();
    let uncovered = bindings
        .iter()
        .filter(|r| !covered.contains(r.handler()))
        .map(|r| r.handler().to_owned())
        .collect();
    let mut entries = BTreeMap::new();
    for handler in &covered {
        budget.charge(2)?;
        let binding = by_handler[handler.as_str()];
        require(
            b["policies"][text(&b["scopes"][handler]["closure"])?]["body"]["mode"] == "terminal",
            "fault-entry-body-unproved",
        )
        .map_err(|e| e.at_path(format!("/body/scopes/{handler}/faultInput")))?;
        entries.insert(handler.clone(), binding.result().to_owned());
    }
    if entries.is_empty() {
        // Metadata/uncovered bindings supply no body proof. Avoid another
        // graph/provider/dependency walk when there is no conditional entry.
        return Ok(FaultEntryReport {
            origins,
            covered,
            uncovered,
            nodes: BTreeSet::new(),
            scopes: BTreeSet::new(),
            transfers: 0,
            accesses: BTreeSet::new(),
            obligations: BTreeSet::new(),
            provenance: BTreeSet::new(),
        });
    }
    for scope in object(&b["scopes"])?.values() {
        budget.tick(0)?;
        if let Some(parent) = scope["parent"].as_str()
            && covered.contains(parent)
        {
            return Err(at("fault-entry-body-unproved", parent));
        }
    }
    for n in object(&b["nodes"])?.values() {
        budget.tick(0)?;
        let scope = text(&n["scope"])?;
        if !covered.contains(scope) {
            continue;
        }
        if !matches!(n["kind"].as_str(), Some("compute" | "decision" | "finish")) {
            return Err(at("fault-entry-body-unproved", scope));
        }
        for field in ["reads", "writes"] {
            for cell in object(&n[field])?.keys() {
                budget.tick(0)?;
                if b["data"][cell]["scope"] != scope {
                    return Err(at("fault-entry-storage-unproved", scope));
                }
            }
        }
    }
    // Initializers and access-policy reads are checked by the shared entry
    // availability analysis, which never inherits ancestor values.
    let coverage = control::check_history_entries(b, &entries, budget)?;
    let flow =
        protection::analyze_history_entries(b, supplied, &owners, &sources, &entries, ordinary)?;
    let obligations = invalidation::require_entries_with_codes(
        b,
        &flow.dependencies.nodes,
        &mut Budget::default(),
        "fault-entry-invalidation-coverage",
        "fault-entry-invalidation-context-unproved",
    )?;
    Ok(FaultEntryReport {
        origins,
        covered,
        uncovered,
        nodes: coverage.nodes,
        scopes: coverage.scopes,
        transfers: flow.transfers,
        accesses: flow.accesses,
        obligations,
        provenance: flow.provenance,
    })
}
