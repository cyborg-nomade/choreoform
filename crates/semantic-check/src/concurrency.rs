// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Ordinary parallel independence only; no arbitrary invariant or host safety proof.
use super::*;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};
mod owned;
pub use owned::{OwnedLocalFollowup, OwnedLocalReport};
mod summary;
mod survivors;
use summary::Summary;
pub use survivors::IsolatedFollowup;

#[derive(Debug)]
pub struct ConcurrencyReport {
    invalidation: InvalidationReport,
    checked: BTreeSet<String>,
    uncovered: BTreeSet<String>,
    isolated: BTreeSet<IsolatedFollowup>,
    owned_local: Option<OwnedLocalReport>,
}
impl ConcurrencyReport {
    pub fn revision(&self) -> &str {
        self.invalidation.revision()
    }
    pub fn invalidation(&self) -> &InvalidationReport {
        &self.invalidation
    }
    pub fn checked_forks(&self) -> &BTreeSet<String> {
        &self.checked
    }
    pub fn uncovered_forks(&self) -> &BTreeSet<String> {
        &self.uncovered
    }
    pub fn isolated_followups(&self) -> &BTreeSet<IsolatedFollowup> {
        &self.isolated
    }
    /// Conditional fresh-local checks, without changing ordinary coverage.
    pub fn owned_local(&self) -> Option<&OwnedLocalReport> {
        self.owned_local.as_ref()
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "parallel-and-exceptional-availability-proofs",
            "exceptional-and-observable-channel-information-flow",
            "current-principal-purpose-capability-and-policy-enforcement",
            "domain-and-irreversible-race-invariant-proofs",
            "exceptional-and-reconciliation-concurrency",
            "live-membership-sealing-and-lifecycle-enforcement",
            "conditional-isolated-followup-ownership-correlation-and-terminal-settlement",
            "exceptional-and-non-data-invalidation-and-lifecycle-enforcement",
            "occurrence-bound-ordinary-closure-and-snapshot-enforcement",
            "host-trust-and-atomic-runtime-enforcement",
            "full-benchmark-and-conformance-evidence",
        ]
    }
}

pub fn check_concurrency(
    envelope: &DefinitionEnvelope,
    supplied: &[Resource<'_>],
) -> Result<ConcurrencyReport> {
    let invalidation = check_invalidation(envelope, supplied)?;
    let b = &envelope.document()["body"];
    let mut budget = Budget::default();
    let summaries = Summary::build(b, invalidation.scope_contexts(), &mut budget)?;
    let covered = invalidation.protection().control().covered_nodes();
    let mut checked = BTreeSet::new();
    let mut uncovered = BTreeSet::new();
    let mut isolated = BTreeSet::new();
    let mut survivor_index = None;
    let mut owned_candidates = None;
    for (id, n) in object(&b["nodes"])? {
        budget.tick(0)?;
        let kind = text(&n["kind"])?;
        if !covered.contains(id) {
            if matches!(kind, "split" | "fanout") {
                uncovered.insert(id.clone());
            }
            continue;
        }
        if matches!(kind, "invoke" | "repeat") {
            let body = text(&n["body"])?;
            let closure = text(&b["scopes"][body]["closure"])?;
            if b["policies"][closure]["body"]["mode"] != "terminal" {
                if survivor_index.is_none() {
                    survivor_index = Some(survivors::Index::build(b, &mut budget)?);
                }
                let index = survivor_index.as_mut().unwrap();
                if index.has_storage_followup(b, body)? {
                    if owned_candidates.is_none() {
                        owned_candidates = Some(owned::Candidates::new(b, &mut budget)?);
                    }
                    owned_candidates.as_mut().unwrap().consider(
                        b,
                        index.graph(),
                        body,
                        id,
                        &mut budget,
                    )?;
                } else {
                    isolated.insert(index.check(b, body, id, &mut budget)?);
                }
            }
        }
        if !matches!(kind, "split" | "fanout") {
            continue;
        }
        let children: Vec<_> = if kind == "split" {
            object(&n["children"])?
                .values()
                .map(text)
                .collect::<Result<_>>()?
        } else {
            vec![text(&n["body"])?]
        };
        budget.charge(children.len())?;
        let join = text(&n["join"])?;
        let population = (kind == "split").then_some(children.len());
        if !requires_all(&b["nodes"][join]["predicate"], population, &mut budget, 0)? {
            return Err(at("parallel-lifetime-unproved", join));
        }
        let mut previous = Summary::default();
        for child in children {
            terminal(b, child, id)?;
            let shared = summaries[child].shared(b, child, &mut budget)?;
            if kind == "fanout" {
                independent(&shared, &shared, id, &mut budget)?;
            } else {
                // Comparing against accumulated sets detects exactly the same
                // pair hazards without quadratic scans of empty/read-only peers.
                independent(&shared, &previous, id, &mut budget)?;
                previous.reads.extend(shared.reads);
                previous.writes.extend(shared.writes);
                previous.observations |= shared.observations;
                previous.effects |= shared.effects;
            }
        }
        checked.insert(id.clone());
    }
    let owned_local = owned_candidates
        .map(|c| c.check(envelope, supplied, invalidation.protection(), &mut budget))
        .transpose()?;
    Ok(ConcurrencyReport {
        invalidation,
        checked,
        uncovered,
        isolated,
        owned_local,
    })
}

fn terminal(b: &Value, child: &str, site: &str) -> Result<()> {
    let policy = text(&b["scopes"][child]["closure"])?;
    if b["policies"][policy]["body"]["mode"] != "terminal" {
        return Err(at("scope-lifetime-unproved", site));
    }
    Ok(())
}
fn independent(a: &Summary, b: &Summary, site: &str, budget: &mut Budget) -> Result<()> {
    budget.charge(2 * a.reads.len() + 3 * a.writes.len() + 1)?;
    if a.writes
        .iter()
        .any(|cell| b.writes.contains(cell) || b.reads.contains(cell))
        || a.reads.iter().any(|cell| b.writes.contains(cell))
    {
        return Err(at("parallel-storage-conflict-unproved", site));
    }
    if (a.effects && b.observations) || (b.effects && a.observations) {
        return Err(at("parallel-effects-unproved", site));
    }
    Ok(())
}
fn requires_all(
    p: &Value,
    population: Option<usize>,
    budget: &mut Budget,
    depth: usize,
) -> Result<bool> {
    budget.tick(depth)?;
    match text(&p["op"])? {
        "all" => Ok(true),
        "any" => Ok(population == Some(1)),
        "atLeast" => {
            Ok(population
                .is_some_and(|n| p["count"].as_u64().is_some_and(|count| count >= n as u64)))
        }
        "and" => {
            for term in object(&p["terms"])?.values() {
                if requires_all(term, population, budget, depth + 1)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        "or" => {
            for term in object(&p["terms"])?.values() {
                if !requires_all(term, population, budget, depth + 1)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        _ => Err(err("join-predicate")),
    }
}
fn at(code: &'static str, site: &str) -> Diagnostic {
    Diagnostic {
        code,
        path: format!("/body/nodes/{site}"),
    }
}
