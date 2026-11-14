// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Bounded declarations and conditional body proof; never actual history values.
use super::static_output::{push_row, reserve};
use choreoform_semantic_check::{
    FaultEntryReport, FaultSelection, Pin, ProviderFaultHistory, Report, Type,
};
use serde_json::{Value, json};
use std::collections::BTreeMap;
fn pin(p: &Pin) -> Value {
    json!({"id":p.id,"revision":p.revision})
}
fn selection(s: &FaultSelection) -> Value {
    json!({"owner":s.owner(),"policy":s.policy(),"fault":s.fault(),"handler":s.handler()})
}
pub(super) fn append_declared(
    report: &Report,
    value: &mut Value,
    remaining: &mut usize,
) -> Result<(), String> {
    if report.fault_entries().is_empty() {
        return Ok(());
    }
    let mut rows = Vec::new();
    let mut catalogues: BTreeMap<&Pin, &ProviderFaultHistory> = BTreeMap::new();
    for entry in report.fault_entries() {
        let h = entry.history_metadata();
        push_row(
            &mut rows,
            json!({"revision":entry.revision(),"handler":entry.handler(),"result":entry.result(),"history":pin(h.pin()),"hostRequirements":entry.host_requirements()}),
            remaining,
        )?;
        catalogues.entry(h.pin()).or_insert(h);
    }
    let mut histories = Vec::new();
    for h in catalogues.into_values() {
        let Type::Named(ty) = h.history_type() else {
            return Err("report-type".into());
        };
        push_row(
            &mut histories,
            json!({"history":pin(h.pin()),"profile":pin(h.profile()),"source":pin(h.source_metadata().pin()),"sourceProfile":pin(h.source_metadata().profile()),"provider":pin(h.source_metadata().provider()),"historyType":{"kind":"named","contract":pin(&ty.contract),"name":ty.name},"hostRequirements":h.remaining()}),
            remaining,
        )?;
    }
    value["declaredFaultEntries"] = rows.into();
    value["declaredFaultHistories"] = histories.into();
    Ok(())
}
pub(super) fn append_conditional(
    report: &FaultEntryReport,
    value: &mut Value,
    remaining: &mut usize,
) -> Result<(), String> {
    if report
        .origins()
        .faults()
        .static_report()
        .fault_entries()
        .is_empty()
    {
        return Ok(());
    }
    let mut view = json!({"coverageKind":"conditional","coveredEntries":report.covered_entries(),"uncoveredEntries":report.uncovered_entries(),"checkedNodes":report.checked_nodes(),"checkedScopes":report.checked_scopes(),"transfers":report.transfers(),"accessObligations":[],"invalidationObligations":[],"requiredProtectionProvenance":[],"incomingSources":[],"hostRequirements":report.remaining()});
    reserve(&view, remaining)?;
    let mut accesses = Vec::new();
    for a in report.accesses() {
        push_row(
            &mut accesses,
            json!({"site":a.site(),"cell":a.cell(),"operation":a.operation()}),
            remaining,
        )?;
    }
    let mut obligations = Vec::new();
    for o in report.obligations() {
        push_row(
            &mut obligations,
            json!({"site":o.site(),"cell":o.cell()}),
            remaining,
        )?;
    }
    let mut provenance = Vec::new();
    for p in report.required_provenance() {
        push_row(
            &mut provenance,
            json!({"owner":p.owner(),"kind":p.kind(),"site":p.site(),"providerRole":p.provider_role()}),
            remaining,
        )?;
    }
    let mut incoming = Vec::new();
    for r in report
        .origins()
        .routes()
        .filter(|r| report.covered_entries().contains(r.selected().handler()))
    {
        // Source metadata is already disclosed by the exact static use site;
        // keep full selection stacks without repeating provider/source payloads.
        let before: Vec<_> = r.active_before().map(selection).collect();
        let after: Vec<_> = r.active_after().map(selection).collect();
        push_row(
            &mut incoming,
            json!({"sourceUseSite":r.site(),"originScope":r.scope(),"ordinaryReachable":r.ordinary_reachable(),"selected":selection(r.selected()),"activeBefore":before,"activeAfter":after}),
            remaining,
        )?;
    }
    view["accessObligations"] = accesses.into();
    view["invalidationObligations"] = obligations.into();
    view["requiredProtectionProvenance"] = provenance.into();
    view["incomingSources"] = incoming.into();
    value["conditionalFaultEntries"] = view;
    Ok(())
}
