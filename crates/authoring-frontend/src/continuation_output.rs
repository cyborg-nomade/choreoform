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
            let mut row = json!({"rootOwner":root.owner(),"owner":scope.owner(),"scope":scope.scope(),"closure":scope.closure(),"source":scope.source(),"request":scope.request(),"conditionalSource":scope.conditional_source(),"query":scope.query(),"child":scope.child()});
            if let Some(entry) = scope.pre_capture_entry() {
                row["preCaptureEntry"] = json!({"entry":entry.entry(),"captureWait":entry.capture_wait(),"hostRequirements":entry.host_requirements()});
            }
            if let Some(work) = scope.work() {
                row["work"] = json!({"site":work.site(),"capability":work.capability(),"input":work.input(),"effectClass":work.effect_class().as_str(),"compensates":work.compensates(),"hostRequirements":work.host_requirements()});
            }
            push(&mut scopes, row, &mut remaining)?;
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
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Mode {
        Capturing,
        PreCapture,
        Work,
    }
    fn assert_bounded_catalogue(mode: Mode) {
        let pre_capture = mode == Mode::PreCapture;
        let raw: Value = if mode == Mode::Work {
            serde_json::from_slice(include_bytes!(
                "../tests/capture/finite-work.resources.json"
            ))
            .unwrap()
        } else {
            serde_json::from_slice(include_bytes!(
                "../../semantic-check/tests/conformance/owned-readonly-definition.json"
            ))
            .unwrap()
        };
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut artifacts: Vec<_> = raw["normative"]
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
        if pre_capture {
            use choreoform_semantic_check::CORE_V10;
            artifacts.push((
                CORE_V10.0.into(),
                CORE_V10.1.into(),
                std::fs::read(root.join(format!(
                    "docs/ir/contracts/{}.txt",
                    CORE_V10.1.replace(':', "-")
                )))
                .unwrap(),
            ));
        }
        let supplied: Vec<_> = artifacts
            .iter()
            .map(|(id, revision, bytes)| Resource {
                id,
                revision,
                bytes,
            })
            .collect();
        let mut b = if mode == Mode::Work {
            choreoform_authoring_frontend::Package::parse(
                include_bytes!("../tests/capture/finite-work.choreo"),
                include_bytes!("../tests/capture/finite-work.bindings.json"),
            )
            .unwrap()
            .lower(&supplied)
            .unwrap()
            .document()["body"]
                .clone()
        } else {
            raw["definition"]["body"].clone()
        };
        if pre_capture {
            use choreoform_semantic_check::CORE_V10;
            b["dialects"]["core"] = json!({"id":CORE_V10.0,"revision":CORE_V10.1});
            b["scopes"]["recovery"]["entry"] = "lookup".into();
            b["flows"]["lookup_next"]["target"] = "capture_wait".into();
            b["flows"]["read_yes"]["target"] = "done".into();
        }
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
        for row in value["ownedContinuationScopes"].as_array().unwrap() {
            if mode == Mode::Work && row["scope"] == "recovery" {
                assert!(row["query"].is_null());
                assert_eq!(row["work"]["site"], "lookup");
                assert_eq!(row["work"]["effectClass"], "irreversible");
                assert_eq!(row["work"]["compensates"], "origin");
                assert!(
                    !row["work"]["hostRequirements"]
                        .as_array()
                        .unwrap()
                        .is_empty()
                );
            } else {
                assert!(row.get("work").is_none());
            }
            if pre_capture && row["scope"] == "recovery" {
                assert_eq!(row["preCaptureEntry"]["entry"], "lookup");
                assert_eq!(row["preCaptureEntry"]["captureWait"], "capture_wait");
                assert!(
                    !row["preCaptureEntry"]["hostRequirements"]
                        .as_array()
                        .unwrap()
                        .is_empty()
                );
            } else {
                assert!(row.get("preCaptureEntry").is_none());
            }
        }
        let mut value = json!({"revision":tree.revision(),"inherited":"x".repeat(super::super::MAX_BYTES-1000)});
        let before = value.clone();
        assert_eq!(append(tree, &mut value).unwrap_err(), "report-budget");
        assert_eq!(value, before);
        let mut value = json!({"revision":"other"});
        assert_eq!(append(tree, &mut value).unwrap_err(), "report-revision");
    }
    #[test]
    fn inherited_output_and_shared_scope_catalogue_are_bounded_without_partial_mutation() {
        assert_bounded_catalogue(Mode::Capturing);
    }
    #[test]
    fn pre_capture_relations_require_host_acceptance_and_share_bounded_immutable_rows() {
        assert_bounded_catalogue(Mode::PreCapture);
    }
    #[test]
    fn new_work_relations_share_bounded_rows_without_accepted_host_authority() {
        assert_bounded_catalogue(Mode::Work);
    }
}
