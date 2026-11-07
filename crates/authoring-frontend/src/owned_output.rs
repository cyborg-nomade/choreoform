// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Explicit conditional checks, never a live owner or allocated storage receipt.
use choreoform_semantic_check::OwnedLocalReport;
use serde_json::{Value, json};
fn push(rows: &mut Vec<Value>, row: Value, remaining: &mut usize) -> Result<(), String> {
    let size = serde_json::to_vec(&row)
        .map_err(|_| "report-budget".to_owned())?
        .len();
    *remaining = remaining
        .checked_sub(size)
        .ok_or_else(|| "report-budget".to_owned())?;
    rows.push(row);
    Ok(())
}
pub(super) fn append(report: &OwnedLocalReport, value: &mut Value) -> Result<(), String> {
    let mut remaining = super::MAX_BYTES;
    let mut rows = Vec::new();
    for p in report.followups() {
        push(
            &mut rows,
            json!({"creator":p.creator(),"body":p.body(),"closure":p.closure(),"followup":p.followup(),"localCells":p.local_cells(),"captureSites":p.capture_sites(),"hostRequirements":p.host_requirements()}),
            &mut remaining,
        )?;
    }
    let mut accesses = Vec::new();
    for a in report.accesses() {
        push(
            &mut accesses,
            json!({"site":a.site(),"cell":a.cell(),"operation":a.operation()}),
            &mut remaining,
        )?;
    }
    let mut obligations = Vec::new();
    for o in report.obligations() {
        push(
            &mut obligations,
            json!({"site":o.site(),"cell":o.cell()}),
            &mut remaining,
        )?;
    }
    let mut provenance = Vec::new();
    for p in report.required_provenance() {
        push(
            &mut provenance,
            json!({"owner":p.owner(),"kind":p.kind(),"site":p.site(),"providerRole":p.provider_role()}),
            &mut remaining,
        )?;
    }
    value["ownedLocalFollowups"] = rows.into();
    value["conditionalOwnedLocalChecks"] = json!({"revision":report.revision(),"conditional":true,"checkedNodes":report.checked_nodes(),"checkedScopes":report.checked_scopes(),"transfers":report.transfers(),"accessObligations":accesses,"invalidationObligations":obligations,"remaining":report.remaining(),"requiredProtectionProvenance":provenance});
    Ok(())
}
