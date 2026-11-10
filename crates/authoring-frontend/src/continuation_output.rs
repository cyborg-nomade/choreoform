// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Bounded conditional declarations, never live ownership or accepted host gates.
use super::static_output::{push_row as push, reserve};
use choreoform_semantic_check::OwnedContinuationReport;
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub(super) fn append(report: &OwnedContinuationReport, value: &mut Value) -> Result<(), String> {
    if value["revision"] != report.revision() {
        return Err("report-revision".into());
    }
    let mut remaining = super::MAX_BYTES;
    reserve(value, &mut remaining)?;
    let mut roots = Vec::new();
    let mut scopes = Vec::new();
    let mut seen = BTreeSet::new();
    for root in report.continuations() {
        push(
            &mut roots,
            json!({"creator":root.creator(),"owner":root.owner()}),
            &mut remaining,
        )?;
        for scope in root.scopes() {
            if seen.contains(scope.scope()) {
                continue;
            }
            push(
                &mut scopes,
                json!({"rootOwner":root.owner(),"owner":scope.owner(),"scope":scope.scope(),"closure":scope.closure(),"source":scope.source(),"request":scope.request(),"conditionalSource":scope.conditional_source(),"query":scope.query(),"child":scope.child()}),
                &mut remaining,
            )?;
            seen.insert(scope.scope().to_owned());
        }
    }
    let mut gates = Vec::new();
    for gate in report.gates() {
        push(
            &mut gates,
            json!({"kind":gate.kind(),"owner":gate.owner(),"source":gate.source(),"receiver":gate.receiver(),"managementActor":gate.management_actor(),"hostRequirements":gate.host_requirements()}),
            &mut remaining,
        )?;
    }
    let mut accesses = Vec::new();
    for access in report.accesses() {
        push(
            &mut accesses,
            json!({"site":access.site(),"cell":access.cell(),"operation":access.operation()}),
            &mut remaining,
        )?;
    }
    let mut obligations = Vec::new();
    for obligation in report.obligations() {
        push(
            &mut obligations,
            json!({"site":obligation.site(),"cell":obligation.cell()}),
            &mut remaining,
        )?;
    }
    let mut provenance = Vec::new();
    for origin in report.required_provenance() {
        push(
            &mut provenance,
            json!({"owner":origin.owner(),"kind":origin.kind(),"site":origin.site(),"providerRole":origin.provider_role()}),
            &mut remaining,
        )?;
    }
    let mut checks = json!({"revision":report.revision(),"conditional":true,"checkedNodes":report.checked_nodes(),"checkedScopes":report.checked_scopes(),"transfers":report.transfers(),"remaining":report.remaining()});
    reserve(&checks, &mut remaining)?;
    checks["accessObligations"] = accesses.into();
    checks["invalidationObligations"] = obligations.into();
    checks["requiredProtectionProvenance"] = provenance.into();
    checks["gates"] = gates.into();
    value["ownedContinuations"] = roots.into();
    value["ownedContinuationScopes"] = scopes.into();
    value["conditionalOwnedContinuationChecks"] = checks;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};
    #[test]
    fn inherited_output_and_shared_scope_catalogue_are_bounded_without_partial_mutation() {
        let raw: Value = serde_json::from_slice(include_bytes!(
            "../../semantic-check/tests/conformance/owned-readonly-definition.json"
        ))
        .unwrap();
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let artifacts: Vec<_> = raw["normative"]
            .as_array()
            .unwrap()
            .iter()
            .chain(raw["resources"].as_array().unwrap())
            .map(|r| {
                let revision = r["revision"].as_str().unwrap().to_owned();
                let bytes = if let Some(s) = r["bytes"].as_str() {
                    s.as_bytes().to_vec()
                } else {
                    std::fs::read(root.join(format!(
                        "docs/ir/contracts/{}.txt",
                        revision.replace(':', "-")
                    )))
                    .unwrap()
                };
                (r["id"].as_str().unwrap().to_owned(), revision, bytes)
            })
            .collect();
        let supplied: Vec<_> = artifacts
            .iter()
            .map(|(id, revision, bytes)| Resource {
                id,
                revision,
                bytes,
            })
            .collect();
        let mut b = raw["definition"]["body"].clone();
        b["nodes"]["run_again"] = b["nodes"]["run_owner"].clone();
        b["flows"]["main_next"]["target"] = "run_again".into();
        b["flows"]["again_next"] =
            json!({"source":"run_again","outcome":"complete","target":"main_done"});
        let e = DefinitionEnvelope::from_parts_versioned("0.4.0", b, json!({})).unwrap();
        let r = choreoform_semantic_check::check_concurrency(&e, &supplied).unwrap();
        let tree = r.owned_continuations().unwrap();
        let mut value = json!({"revision":tree.revision(),"executable":false});
        append(tree, &mut value).unwrap();
        assert_eq!(value["ownedContinuations"].as_array().unwrap().len(), 2);
        assert_eq!(
            value["ownedContinuationScopes"].as_array().unwrap().len(),
            2
        );
        assert_eq!(
            value["conditionalOwnedContinuationChecks"]["gates"]
                .as_array()
                .unwrap()
                .len(),
            7
        );
        let mut value = json!({"revision":tree.revision(),"inherited":"x".repeat(super::super::MAX_BYTES-1000)});
        let before = value.clone();
        assert_eq!(append(tree, &mut value).unwrap_err(), "report-budget");
        assert_eq!(value, before);
        let mut value = json!({"revision":"other"});
        assert_eq!(append(tree, &mut value).unwrap_err(), "report-revision");
    }
}
