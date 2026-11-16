// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Bounded use rows plus one all-name catalogue per exact source, never receipts.
use super::static_output::push_row;
use choreoform_semantic_check::{Pin, ProviderFaultSource, Report};
use serde_json::{Value, json};
use std::collections::BTreeMap;

pub(super) fn append(
    report: &Report,
    value: &mut Value,
    remaining: &mut usize,
) -> Result<(), String> {
    if report.fault_source_uses().is_empty() {
        return Ok(());
    }
    let pin = |p: &Pin| json!({"id":p.id,"revision":p.revision});
    let mut catalogues: BTreeMap<&Pin, &ProviderFaultSource> = BTreeMap::new();
    let mut uses = Vec::new();
    for binding in report.fault_source_uses() {
        push_row(
            &mut uses,
            json!({"revision":binding.revision(),"site":binding.site(),"policy":binding.policy(),"source":pin(binding.source()),"profile":pin(binding.profile()),"provider":pin(binding.provider()),"hostRequirements":binding.host_requirements()}),
            remaining,
        )?;
        catalogues
            .entry(binding.source())
            .or_insert_with(|| binding.source_metadata());
    }
    let mut sources = Vec::new();
    for source in catalogues.into_values() {
        let emissions: serde_json::Map<String, Value> = source
            .emissions()
            .iter()
            .map(|(name, e)| {
                (
                    name.clone(),
                    json!({"observations":e.observations(),"evidence":e.evidence().as_str()}),
                )
            })
            .collect();
        push_row(
            &mut sources,
            json!({"source":pin(source.pin()),"profile":pin(source.profile()),"provider":pin(source.provider()),"emissions":emissions}),
            remaining,
        )?;
    }
    value["declaredFaultSourceUses"] = uses.into();
    value["declaredFaultSourceCatalogues"] = sources.into();
    Ok(())
}
