// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

//! Native test adapter; explicit source/companion paths, no discovery or writes.
use choreoform_authoring_frontend::{Candidate, MAX_BYTES, Package, parse_fragment};
use choreoform_ir_probe_core::{Resource, SUPPORTED_CONTRACTS};
use std::io::{Read, Write};

mod capture_output;
use capture_output::capture_output;
mod retry_output;
use retry_output::timing_output;

const CORE_ID: &str = "urn:choreoform:contracts:core-profile:0.1.0";
const CORE_REV: &str = "sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58";
fn semantic_error(
    candidate: &Candidate,
    diagnostic: choreoform_semantic_check::Diagnostic,
) -> String {
    match candidate.source_span(&diagnostic) {
        Some(span) => format!(
            "{} at {}; source bytes {}..{}",
            diagnostic.code, diagnostic.path, span.start, span.end
        ),
        None => format!("{} at {}", diagnostic.code, diagnostic.path),
    }
}
// A bounded native view of conditional declaration evidence, never values or
// actual closure/occurrence receipts. Library callers retain the typed report.
fn invalidation_output(
    report: &choreoform_semantic_check::Report,
    historical: &std::collections::BTreeSet<choreoform_semantic_check::HistoricalOrigin>,
    mut value: serde_json::Value,
) -> Result<String, String> {
    let mut remaining = MAX_BYTES;
    let mut origins = Vec::new();
    for h in historical {
        let mut record = serde_json::json!({"site":h.site(),"cell":h.cell(),"boundary":h.boundary(),"scope":h.scope(),"closure":h.closure(),"condition":h.condition()});
        if let Some(scope) = h.context_scope() {
            record["contextScope"] = scope.into();
        }
        let size = serde_json::to_vec(&record)
            .map_err(|_| "report-budget".to_owned())?
            .len();
        remaining = remaining
            .checked_sub(size)
            .ok_or_else(|| "report-budget".to_owned())?;
        origins.push(record);
    }
    value["historicalOrigins"] = origins.into();
    capture_output(report, value)
}
fn concurrency_output(
    report: &choreoform_semantic_check::ConcurrencyReport,
    mut value: serde_json::Value,
) -> Result<String, String> {
    let mut remaining = MAX_BYTES;
    let mut records = Vec::new();
    for p in report.isolated_followups() {
        let record = serde_json::json!({"creator":p.creator(),"body":p.body(),"closure":p.closure(),"followup":p.followup(),"waits":p.waits(),"hostRequirements":p.host_requirements()});
        let size = serde_json::to_vec(&record)
            .map_err(|_| "report-budget".to_owned())?
            .len();
        remaining = remaining
            .checked_sub(size)
            .ok_or_else(|| "report-budget".to_owned())?;
        records.push(record);
    }
    value["isolatedFollowups"] = records.into();
    invalidation_output(
        report.invalidation().protection().control().static_report(),
        report.invalidation().historical_origins(),
        value,
    )
}
fn read(input: impl Read) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    input
        .take(MAX_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "input-read".to_owned())?;
    if bytes.len() > MAX_BYTES {
        return Err("size".into());
    }
    Ok(bytes)
}
fn run() -> Result<String, String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() == 2 && args[0] == "parse-fragment" {
        let raw = read(std::io::stdin().lock())?;
        let syntax =
            parse_fragment(&raw, &args[1]).map_err(|e| format!("{} at {:?}", e.code, e.span))?;
        return Ok(
            serde_json::json!({"rule":syntax.tree().rule,"span":syntax.span(syntax.tree())})
                .to_string(),
        );
    }
    if args.len() < 3
        || !matches!(
            args[0].as_str(),
            "lower"
                | "integrity"
                | "format"
                | "check"
                | "check-control"
                | "check-protection"
                | "check-invalidation"
                | "invalidation-requirements"
                | "check-concurrency"
                | "check-timing"
                | "check-fault-routes"
                | "check-exceptional-availability"
                | "check-reconciliation-links"
                | "check-reconciliation-availability"
        )
        || args.len() > 3 + 4 * 254
        || !(args.len() - 3).is_multiple_of(4)
        || (args[0] == "integrity" && args.len() != 3)
    {
        return Err("usage: choreoform-authoring-frontend <lower|check|check-control|check-protection|check-invalidation|invalidation-requirements|check-concurrency|check-timing|check-fault-routes|check-exceptional-availability|check-reconciliation-links|check-reconciliation-availability|format|integrity> SOURCE COMPANION [--resource ID REVISION PATH]...; checking grants NO execution authority; or parse-fragment RULE < stdin".into());
    }
    let source = read(std::fs::File::open(&args[1]).map_err(|_| "source-read".to_owned())?)?;
    let bindings = read(std::fs::File::open(&args[2]).map_err(|_| "companion-read".to_owned())?)?;
    let package =
        Package::parse(&source, &bindings).map_err(|e| format!("{} at {:?}", e.code, e.span))?;
    if args[0] == "integrity" {
        let i = package.integrity();
        return Ok(
            serde_json::json!({"source":i.source,"bindings":i.bindings,"package":i.package})
                .to_string(),
        );
    }
    let mut extra = Vec::new();
    let mut total = 0usize;
    for arg in args[3..].as_chunks::<4>().0 {
        if arg[0] != "--resource" {
            return Err("resource-arguments".into());
        }
        let bytes = read(std::fs::File::open(&arg[3]).map_err(|_| "resource-read".to_owned())?)?;
        total = total
            .checked_add(bytes.len())
            .ok_or_else(|| "resource-budget".to_owned())?;
        if total > 8 * MAX_BYTES {
            return Err("resource-budget".into());
        }
        extra.push((&arg[1], &arg[2], bytes));
    }
    let mut resources = vec![
        Resource {
            id: SUPPORTED_CONTRACTS[0].0,
            revision: SUPPORTED_CONTRACTS[0].1,
            bytes: include_bytes!(
                "../../../docs/ir/contracts/sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt"
            ),
        },
        Resource {
            id: CORE_ID,
            revision: CORE_REV,
            bytes: include_bytes!(
                "../../../docs/ir/contracts/sha256-26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58.txt"
            ),
        },
    ];
    if package.syntax().profile() != "0.2.0" {
        resources.push(Resource {
            id: choreoform_semantic_check::CORE_V2.0,
            revision: choreoform_semantic_check::CORE_V2.1,
            bytes: include_bytes!("../../../docs/ir/contracts/sha256-5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623.txt"),
        });
    }
    if matches!(package.syntax().profile(), "0.6.0" | "0.7.0" | "0.8.0") {
        resources.push(Resource {
            id: choreoform_semantic_check::CORE_V3.0,
            revision: choreoform_semantic_check::CORE_V3.1,
            bytes: include_bytes!("../../../docs/ir/contracts/sha256-30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3.txt"),
        });
    }
    if matches!(package.syntax().profile(), "0.7.0" | "0.8.0") {
        resources.push(Resource {
            id: choreoform_semantic_check::CORE_V4.0,
            revision: choreoform_semantic_check::CORE_V4.1,
            bytes: include_bytes!("../../../docs/ir/contracts/sha256-0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3.txt"),
        });
    }
    if package.syntax().profile() == "0.8.0" {
        resources.push(Resource {
            id: choreoform_semantic_check::CORE_V5.0,
            revision: choreoform_semantic_check::CORE_V5.1,
            bytes: include_bytes!("../../../docs/ir/contracts/sha256-50007ac05e5e71df91bb9997a40a06c7a8993df04b1a673421e4a987014299d7.txt"),
        });
    }
    resources.extend(extra.iter().map(|(id, revision, bytes)| Resource {
        id,
        revision,
        bytes,
    }));
    let candidate = package
        .lower(&resources)
        .map_err(|e| format!("{} at {:?}", e.code, e.span))?;
    if args[0] == "check" {
        let report = candidate
            .check_static(&resources)
            .map_err(|e| semantic_error(&candidate, e))?;
        return capture_output(
            &report,
            serde_json::json!({"status":"static-checks-passed","executable":false,"revision":report.revision(),"expressions":report.expressions(),"policies":report.policies(),"remaining":report.remaining()}),
        );
    }
    if args[0] == "check-control" {
        let report = candidate
            .check_control(&resources)
            .map_err(|e| semantic_error(&candidate, e))?;
        return capture_output(
            report.static_report(),
            serde_json::json!({"status":"control-checks-passed","executable":false,"revision":report.revision(),"coveredNodes":report.covered_nodes(),"uncoveredNodes":report.uncovered_nodes(),"coveredScopes":report.covered_scopes(),"requiredInputs":report.required_inputs(),"remaining":report.remaining()}),
        );
    }
    if args[0] == "check-protection" {
        let report = candidate
            .check_protection(&resources)
            .map_err(|e| semantic_error(&candidate, e))?;
        let accesses: Vec<_> = report
            .accesses()
            .iter()
            .map(|a| serde_json::json!({"site":a.site(),"cell":a.cell(),"operation":a.operation()}))
            .collect();
        return capture_output(
            report.control().static_report(),
            serde_json::json!({"status":"protection-checks-passed","executable":false,"revision":report.revision(),"transfers":report.transfers(),"accessObligations":accesses,"coveredNodes":report.control().covered_nodes(),"uncoveredNodes":report.control().uncovered_nodes(),"requiredInputs":report.control().required_inputs(),"remaining":report.remaining()}),
        );
    }
    if args[0] == "invalidation-requirements" {
        let report = candidate
            .invalidation_requirements(&resources)
            .map_err(|e| semantic_error(&candidate, e))?;
        let pairs = |set: &std::collections::BTreeSet<
            choreoform_semantic_check::InvalidationObligation,
        >| {
            set.iter()
                .map(|o| serde_json::json!({"site":o.site(),"cell":o.cell()}))
                .collect::<Vec<_>>()
        };
        return invalidation_output(
            report.protection().control().static_report(),
            report.historical_origins(),
            serde_json::json!({"status":"invalidation-requirements-derived","advisory":true,"executable":false,"revision":report.revision(),"analyzedInvalidationNodes":report.analyzed_nodes(),"uncoveredInvalidationNodes":report.uncovered_nodes(),"requiredInvalidationObligations":pairs(report.obligations()),"missingInvalidationObligations":pairs(report.missing()),"requiredInputs":report.protection().control().required_inputs(),"remaining":report.remaining(),"requiredNextCheck":report.required_check()}),
        );
    }
    if args[0] == "check-invalidation" {
        let report = candidate
            .check_invalidation(&resources)
            .map_err(|e| semantic_error(&candidate, e))?;
        let obligations: Vec<_> = report
            .obligations()
            .iter()
            .map(|o| serde_json::json!({"site":o.site(),"cell":o.cell()}))
            .collect();
        return invalidation_output(
            report.protection().control().static_report(),
            report.historical_origins(),
            serde_json::json!({"status":"invalidation-checks-passed","executable":false,"revision":report.revision(),"checkedInvalidationNodes":report.checked_nodes(),"uncoveredInvalidationNodes":report.uncovered_nodes(),"invalidationObligations":obligations,"coveredNodes":report.protection().control().covered_nodes(),"uncoveredNodes":report.protection().control().uncovered_nodes(),"requiredInputs":report.protection().control().required_inputs(),"remaining":report.remaining()}),
        );
    }
    if args[0] == "check-concurrency" {
        let report = candidate
            .check_concurrency(&resources)
            .map_err(|e| semantic_error(&candidate, e))?;
        let control = report.invalidation().protection().control();
        return concurrency_output(
            &report,
            serde_json::json!({"status":"concurrency-checks-passed","executable":false,"revision":report.revision(),"checkedForks":report.checked_forks(),"uncoveredForks":report.uncovered_forks(),"coveredNodes":control.covered_nodes(),"uncoveredNodes":control.uncovered_nodes(),"requiredInputs":control.required_inputs(),"checkedInvalidationNodes":report.invalidation().checked_nodes(),"uncoveredInvalidationNodes":report.invalidation().uncovered_nodes(),"remaining":report.remaining()}),
        );
    }
    if args[0] == "check-timing" {
        let report = candidate
            .check_timing(&resources)
            .map_err(|e| semantic_error(&candidate, e))?;
        let control = report.concurrency().invalidation().protection().control();
        let uses: Vec<_> = report.timer_uses().iter().map(|t| {
            let calendar = t.calendar().map(|c| serde_json::json!({"id":c.pin().id,"revision":c.pin().revision,"rules":{"id":c.rules().id,"revision":c.rules().revision},"profile":{"id":c.profile().id,"revision":c.profile().revision},"hostRequirements":c.host_requirements()}));
            serde_json::json!({"site":t.site(),"policy":t.policy(),"role":t.role(),"clock":{"id":t.clock().id,"revision":t.clock().revision},"source":{"id":t.source().id,"revision":t.source().revision},"basis":t.basis(),"pauseAuthority":t.pause_authority(),"calendar":calendar,"hostRequirements":t.host_requirements()})
        }).collect();
        let retries: Vec<_> = report.retry_uses().iter().map(|r| serde_json::json!({"site":r.site(),"policy":r.policy(),"clock":{"id":r.clock().id,"revision":r.clock().revision},"source":{"id":r.source().id,"revision":r.source().revision},"basis":r.basis(),"authority":r.authority(),"maxAttempts":r.max_attempts(),"delays":r.delays().iter().map(i64::to_string).collect::<Vec<_>>(),"hostRequirements":r.host_requirements()})).collect();
        return timing_output(
            &report,
            serde_json::json!({"status":"timing-checks-passed","executable":false,"revision":report.revision(),"timerUses":uses,"retryUses":retries,"checkedTimingNodes":report.checked_nodes(),"uncoveredTimingNodes":report.uncovered_nodes(),"coveredNodes":control.covered_nodes(),"uncoveredNodes":control.uncovered_nodes(),"requiredInputs":control.required_inputs(),"remaining":report.remaining()}),
        );
    }
    if args[0] == "check-fault-routes" {
        let report = candidate
            .check_fault_routes(&resources)
            .map_err(|e| semantic_error(&candidate, e))?;
        let control = report
            .timing()
            .concurrency()
            .invalidation()
            .protection()
            .control();
        let selection = |s: &choreoform_semantic_check::FaultSelection| serde_json::json!({"owner":s.owner(),"policy":s.policy(),"fault":s.fault(),"handler":s.handler()});
        let mut remaining = MAX_BYTES;
        let mut bounded = |value: serde_json::Value| -> Result<serde_json::Value, String> {
            let size = serde_json::to_vec(&value)
                .map_err(|_| "report-budget".to_owned())?
                .len();
            remaining = remaining
                .checked_sub(size)
                .ok_or_else(|| "report-budget".to_owned())?;
            Ok(value)
        };
        let routes:Vec<_> = report.routes().map(|r|serde_json::json!({"originScope":r.origin(),"from":r.from().map(selection),"selected":selection(r.selected())})).map(&mut bounded).collect::<Result<_,_>>()?;
        let checked: Vec<_> = report
            .covered_handlers()
            .map(selection)
            .map(&mut bounded)
            .collect::<Result<_, _>>()?;
        let uncovered: Vec<_> = report
            .uncovered_handlers()
            .map(selection)
            .map(&mut bounded)
            .collect::<Result<_, _>>()?;
        return timing_output(
            report.timing(),
            serde_json::json!({"status":"fault-route-checks-passed","executable":false,"revision":report.revision(),"faultRoutes":routes,"checkedHandlerSelections":checked,"uncoveredHandlerSelections":uncovered,"checkedFaultScopes":report.checked_scopes(),"uncoveredFaultScopes":report.uncovered_scopes(),"analysisContexts":report.contexts(),"unhandledFaultCases":report.unhandled_cases(),"coveredNodes":control.covered_nodes(),"uncoveredNodes":control.uncovered_nodes(),"requiredInputs":control.required_inputs(),"remaining":report.remaining(),"priorTimingObligations":report.timing().remaining()}),
        );
    }
    if args[0] == "check-exceptional-availability" {
        let report = candidate
            .check_exceptional_availability(&resources)
            .map_err(|e| semantic_error(&candidate, e))?;
        let control = report
            .faults()
            .timing()
            .concurrency()
            .invalidation()
            .protection()
            .control();
        let mut remaining = MAX_BYTES;
        let entries: Vec<_> = report
            .entry_assumptions()
            .iter()
            .map(|(handler, cells)| {
                let value = serde_json::json!({"handler":handler,"requiredAncestorInputs":cells});
                let size = serde_json::to_vec(&value)
                    .map_err(|_| "report-budget".to_owned())?
                    .len();
                remaining = remaining
                    .checked_sub(size)
                    .ok_or_else(|| "report-budget".to_owned())?;
                Ok(value)
            })
            .collect::<Result<_, String>>()?;
        return timing_output(
            report.faults().timing(),
            serde_json::json!({"status":"exceptional-availability-checks-passed","executable":false,"revision":report.revision(),"handlerEntries":entries,"checkedExceptionalNodes":report.checked_nodes(),"checkedExceptionalScopes":report.checked_scopes(),"uncoveredNodes":report.uncovered_nodes(),"ordinaryCoveredNodes":control.covered_nodes(),"ordinaryUncoveredNodes":control.uncovered_nodes(),"requiredInputs":control.required_inputs(),"checkedFaultScopes":report.faults().checked_scopes(),"uncoveredFaultScopes":report.faults().uncovered_scopes(),"remaining":report.remaining(),"priorFaultRouteObligations":report.faults().remaining(),"priorTimingObligations":report.faults().timing().remaining()}),
        );
    }
    if args[0] == "check-reconciliation-links" {
        let report = candidate
            .check_reconciliation_links(&resources)
            .map_err(|e| semantic_error(&candidate, e))?;
        let control = report
            .exceptional()
            .faults()
            .timing()
            .concurrency()
            .invalidation()
            .protection()
            .control();
        let mut remaining = MAX_BYTES;
        let uses:Vec<_>=report.uses().iter().map(|u| {
            let value=serde_json::json!({"site":u.site(),"scope":u.scope(),"capability":u.capability(),"effects":u.effects(),"closure":u.closure(),"followup":u.followup(),"waitPolicy":u.wait_policy(),"waitNode":u.wait_node(),"hostRequirements":u.host_requirements()});
            let size=serde_json::to_vec(&value).map_err(|_|"report-budget".to_owned())?.len();
            remaining=remaining.checked_sub(size).ok_or_else(||"report-budget".to_owned())?;
            Ok(value)
        }).collect::<Result<_,String>>()?;
        return timing_output(
            report.exceptional().faults().timing(),
            serde_json::json!({"status":"reconciliation-links-checked","executable":false,"revision":report.revision(),"reconciliationUses":uses,"ordinaryUnvisitedReconciliationSites":report.ordinary_unvisited_sites(),"unusedEffectContexts":report.unused_effects(),"ordinaryCoveredNodes":control.covered_nodes(),"conditionalExceptionalNodes":report.exceptional().checked_nodes(),"uncoveredNodes":report.exceptional().uncovered_nodes(),"remaining":report.remaining(),"priorExceptionalObligations":report.exceptional().remaining(),"priorFaultRouteObligations":report.exceptional().faults().remaining(),"priorTimingObligations":report.exceptional().faults().timing().remaining()}),
        );
    }
    if args[0] == "check-reconciliation-availability" {
        let report = candidate
            .check_reconciliation_availability(&resources)
            .map_err(|e| semantic_error(&candidate, e))?;
        let control = report
            .links()
            .exceptional()
            .faults()
            .timing()
            .concurrency()
            .invalidation()
            .protection()
            .control();
        let mut remaining = MAX_BYTES;
        let entries: Vec<_> = report.entries().iter().map(|e| {
            let value = serde_json::json!({"owner":e.owner(),"closure":e.closure(),"followup":e.followup(),"inheritedCells":[],"hostRequirements":e.host_requirements()});
            let size = serde_json::to_vec(&value).map_err(|_| "report-budget".to_owned())?.len();
            remaining = remaining.checked_sub(size).ok_or_else(|| "report-budget".to_owned())?;
            Ok(value)
        }).collect::<Result<_, String>>()?;
        return timing_output(
            report.links().exceptional().faults().timing(),
            serde_json::json!({"status":"reconciliation-availability-checks-passed","executable":false,"revision":report.revision(),"reconciliationEntries":entries,"checkedReconciliationNodes":report.checked_nodes(),"checkedReconciliationScopes":report.checked_scopes(),"uncoveredNodes":report.uncovered_nodes(),"ordinaryCoveredNodes":control.covered_nodes(),"conditionalExceptionalNodes":report.links().exceptional().checked_nodes(),"priorReconciliationObligations":report.links().remaining(),"remaining":report.remaining()}),
        );
    }
    if args[0] == "format" {
        let (source, companion) = package.formatted().map_err(|e| e.code.to_owned())?;
        return Ok(serde_json::json!({"source":source,"companion":companion,"semanticRevision":candidate.document()["revision"]}).to_string());
    }
    serde_json::to_string(candidate.document()).map_err(|_| "output-json".into())
}
fn main() -> std::process::ExitCode {
    match run() {
        Ok(output) => match std::io::stdout()
            .lock()
            .write_all(output.as_bytes())
            .and_then(|()| std::io::stdout().flush())
        {
            Ok(()) => std::process::ExitCode::SUCCESS,
            Err(_) => {
                eprintln!("output-write");
                std::process::ExitCode::FAILURE
            }
        },
        Err(message) => {
            eprintln!("{message}");
            std::process::ExitCode::FAILURE
        }
    }
}
