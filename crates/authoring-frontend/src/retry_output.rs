// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Bounded declaration disclosure; no accepted fault or retry permission.
use choreoform_semantic_check::TimingReport;
use serde_json::{Value, json};

pub(super) fn timing_output(report: &TimingReport, mut value: Value) -> Result<String, String> {
    let mut remaining = super::MAX_BYTES;
    let mut rows = Vec::new();
    for binding in report.retry_uses().iter().filter_map(|r| r.fault_source()) {
        let pin = |p: &choreoform_semantic_check::Pin| json!({"id":p.id,"revision":p.revision});
        let faults: serde_json::Map<String, Value> = binding
            .faults()
            .iter()
            .map(|(name, d)| {
                let evidence = match d.evidence() {
                    choreoform_semantic_check::FaultEvidence::DefinitiveNoEffect => {
                        "definitive-no-effect"
                    }
                    choreoform_semantic_check::FaultEvidence::Unresolved => "unresolved",
                };
                (
                    name.clone(),
                    json!({"observations":d.observations(),"evidence":evidence}),
                )
            })
            .collect();
        let row = json!({"revision":binding.revision(),"site":binding.site(),"source":pin(binding.source()),"profile":pin(binding.profile()),"provider":pin(binding.provider()),"faults":faults,"hostRequirements":binding.host_requirements()});
        let size = serde_json::to_vec(&row)
            .map_err(|_| "report-budget".to_owned())?
            .len();
        remaining = remaining
            .checked_sub(size)
            .ok_or_else(|| "report-budget".to_owned())?;
        rows.push(row);
    }
    // Omit on legacy reports, preserving their existing JSON structure.
    if !rows.is_empty() {
        value["retryFaultBindings"] = rows.into();
    }
    super::concurrency_output(report.concurrency(), value)
}
