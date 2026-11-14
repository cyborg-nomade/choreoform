// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Optional declared emitter contexts, not event receipts or handler snapshots.
use super::*;
use std::sync::Arc;

#[derive(Debug)]
struct Site {
    id: String,
    scope: usize,
    reachable: bool,
    binding: Option<usize>,
    unsupported: &'static str,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Route {
    site: usize,
    before: usize,
    after: usize,
    selected: usize,
}
#[derive(Debug)]
pub(super) struct Records {
    sites: Vec<Site>,
    contexts: Vec<Vec<usize>>,
    routes: BTreeSet<Route>,
    unhandled: usize,
    unsupported_unhandled: usize,
}

/// Declared source/activity/selection cases; never initialized or live handlers.
#[derive(Debug)]
pub struct FaultOriginReport {
    faults: Arc<FaultReport>,
    records: Records,
}
impl FaultOriginReport {
    pub(super) fn new(faults: FaultReport, records: Records) -> Self {
        Self {
            faults: Arc::new(faults),
            records,
        }
    }
    pub(crate) fn shared_faults(&self) -> Arc<FaultReport> {
        self.faults.clone()
    }
    pub fn revision(&self) -> &str {
        self.faults.revision()
    }
    pub fn faults(&self) -> &FaultReport {
        &self.faults
    }
    pub fn declared_sites(&self) -> usize {
        self.records.sites.len()
    }
    pub fn sites(&self) -> impl Iterator<Item = FaultOriginSite<'_>> {
        self.records
            .sites
            .iter()
            .map(|site| FaultOriginSite { report: self, site })
    }
    /// Sites whose scope has no explored abstract activation context.
    pub fn uncovered_sites(&self) -> impl Iterator<Item = &str> {
        self.records
            .sites
            .iter()
            .filter(|s| {
                !self
                    .faults
                    .checked_scopes()
                    .contains(&self.faults.scopes[s.scope])
            })
            .map(|s| s.id.as_str())
    }
    /// Syntactically unvisited nodes remain conservative declared origin cases
    /// when their scope is explored. Neither category implies actual activation.
    pub fn ordinary_unvisited_sites(&self) -> impl Iterator<Item = &str> {
        self.records
            .sites
            .iter()
            .filter(|s| !s.reachable)
            .map(|s| s.id.as_str())
    }
    pub fn unhandled_cases(&self) -> usize {
        self.records.unhandled
    }
    pub fn unsupported_unhandled_cases(&self) -> usize {
        self.records.unsupported_unhandled
    }
    pub fn routes(&self) -> impl Iterator<Item = FaultOriginRoute<'_>> {
        self.records.routes.iter().map(|record| FaultOriginRoute {
            report: self,
            record,
        })
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "declared-origin-contexts-only-no-actual-emission-or-activation",
            "unsupported-and-uncovered-origins-have-no-typed-history-entry-proof",
            "truthful-authenticated-non-engine-source-attempt-cause-input-and-request-correlation",
            "complete-source-input-fault-existence-control-and-creator-owner-provenance",
            "actual-nearest-live-handler-selection-exclusion-and-current-authority",
            "reviewed-entry-wire-fresh-readonly-initialization-access-flow-strict-invalidation",
            "monotonic-cancellation-permitted-owned-continuation-handoff-all-settlement",
            "all-prior-validator-benchmark-host-user-browser-independent-and-phase-gates",
        ]
    }
}
/// All activity declarations, including unsupported, unhandled and uncovered sites.
pub struct FaultOriginSite<'a> {
    report: &'a FaultOriginReport,
    site: &'a Site,
}
impl FaultOriginSite<'_> {
    pub fn site(&self) -> &str {
        &self.site.id
    }
    pub fn scope(&self) -> &str {
        &self.report.faults.scopes[self.site.scope]
    }
    pub fn ordinary_reachable(&self) -> bool {
        self.site.reachable
    }
    pub fn scope_explored(&self) -> bool {
        self.report.faults.checked_scopes().contains(self.scope())
    }
    pub fn source_use(&self) -> Option<&DeclaredFaultSourceUse> {
        self.site
            .binding
            .map(|i| &self.report.faults.static_report().fault_source_uses()[i])
    }
    pub fn unsupported_reason(&self) -> Option<&str> {
        self.site.binding.is_none().then_some(self.site.unsupported)
    }
}
/// Borrowed indices into one retained static/routing report; no metadata copies.
pub struct FaultOriginRoute<'a> {
    report: &'a FaultOriginReport,
    record: &'a Route,
}
impl FaultOriginRoute<'_> {
    pub fn site(&self) -> &str {
        &self.report.records.sites[self.record.site].id
    }
    pub fn scope(&self) -> &str {
        &self.report.faults.scopes[self.report.records.sites[self.record.site].scope]
    }
    pub fn ordinary_reachable(&self) -> bool {
        self.report.records.sites[self.record.site].reachable
    }
    pub fn source_use(&self) -> Option<&DeclaredFaultSourceUse> {
        self.report.records.sites[self.record.site]
            .binding
            .map(|i| &self.report.faults.static_report().fault_source_uses()[i])
    }
    pub fn unsupported_reason(&self) -> Option<&str> {
        let site = &self.report.records.sites[self.record.site];
        site.binding.is_none().then_some(site.unsupported)
    }
    pub fn selected(&self) -> &FaultSelection {
        &self.report.faults.selections[self.record.selected]
    }
    pub fn active_before(&self) -> impl Iterator<Item = &FaultSelection> {
        self.report.records.contexts[self.record.before]
            .iter()
            .map(|i| &self.report.faults.selections[*i])
    }
    pub fn active_after(&self) -> impl Iterator<Item = &FaultSelection> {
        self.report.records.contexts[self.record.after]
            .iter()
            .map(|i| &self.report.faults.selections[*i])
    }
}

pub(super) struct Builder {
    records: Records,
    by_scope: Vec<Vec<usize>>,
    contexts: BTreeMap<Vec<usize>, usize>,
}
impl Builder {
    pub fn new(
        b: &Value,
        scopes: &BTreeMap<&str, usize>,
        reachable: &BTreeMap<&str, BTreeSet<&str>>,
        sources: &[DeclaredFaultSourceUse],
        budget: &mut Budget,
    ) -> Result<Self> {
        budget.charge(scopes.len())?;
        let mut by_scope = vec![Vec::new(); scopes.len()];
        let mut sites = Vec::new();
        let mut bindings = BTreeMap::new();
        for (i, binding) in sources.iter().enumerate() {
            budget.charge(2)?;
            bindings.insert(binding.site(), i);
        }
        for (id, n) in object(&b["nodes"])? {
            budget.tick(0)?;
            if n["kind"] != "activity" {
                continue;
            }
            let scope = text(&n["scope"])?;
            budget.charge(7)?;
            by_scope[scopes[scope]].push(sites.len());
            sites.push(Site {
                id: id.clone(),
                scope: scopes[scope],
                reachable: reachable[scope].contains(id.as_str()),
                binding: bindings.get(id.as_str()).copied(),
                unsupported: if n["mode"] == "human" {
                    "human-origin-unproved"
                } else {
                    "general-source-binding-unproved"
                },
            });
        }
        Ok(Self {
            records: Records {
                sites,
                contexts: Vec::new(),
                routes: BTreeSet::new(),
                unhandled: 0,
                unsupported_unhandled: 0,
            },
            by_scope,
            contexts: BTreeMap::new(),
        })
    }
    fn context(&mut self, active: &[usize], budget: &mut Budget) -> Result<usize> {
        budget.charge(active.len() + 1)?;
        if let Some(index) = self.contexts.get(active) {
            return Ok(*index);
        }
        budget.charge(2 * (active.len() + 1))?;
        let index = self.records.contexts.len();
        self.records.contexts.push(active.to_vec());
        self.contexts.insert(active.to_vec(), index);
        Ok(index)
    }
    pub fn record(
        &mut self,
        scope: usize,
        fault: &str,
        before: &[usize],
        selected: Option<(usize, &[usize])>,
        sources: &[DeclaredFaultSourceUse],
        budget: &mut Budget,
    ) -> Result<()> {
        for position in 0..self.by_scope[scope].len() {
            budget.charge(2)?;
            let site = self.by_scope[scope][position];
            let binding = self.records.sites[site].binding;
            if let Some(index) = binding {
                budget.tick(0)?;
                if !sources[index]
                    .source_metadata()
                    .emissions()
                    .contains_key(fault)
                {
                    continue;
                }
            }
            if let Some((selected, after)) = selected {
                let before = self.context(before, budget)?;
                let after = self.context(after, budget)?;
                budget.charge(5)?;
                self.records.routes.insert(Route {
                    site,
                    before,
                    after,
                    selected,
                });
            } else {
                budget.tick(0)?;
                self.records.unhandled += 1;
                if binding.is_none() {
                    self.records.unsupported_unhandled += 1;
                }
            }
        }
        Ok(())
    }
    pub fn finish(self) -> Records {
        self.records
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn builder() -> Builder {
        Builder {
            records: Records {
                sites: vec![Site {
                    id: "unbound".into(),
                    scope: 0,
                    reachable: true,
                    binding: None,
                    unsupported: "general-source-binding-unproved",
                }],
                contexts: Vec::new(),
                routes: BTreeSet::new(),
                unhandled: 0,
                unsupported_unhandled: 0,
            },
            by_scope: vec![vec![0]],
            contexts: BTreeMap::new(),
        }
    }
    #[test]
    fn shared_stack_interning_charges_cached_work_and_refuses_before_new_copy() {
        let mut b = builder();
        let mut budget = Budget::default();
        let context = b.context(&[0, 1, 2], &mut budget).unwrap();
        let before = budget.work;
        assert_eq!(b.context(&[0, 1, 2], &mut budget).unwrap(), context);
        assert!(budget.work > before);
        assert_eq!(b.records.contexts.len(), 1);
        budget.work = 999_990;
        assert_eq!(b.context(&[3; 64], &mut budget).unwrap_err().code, "budget");
        assert_eq!(b.records.contexts.len(), 1);
    }
    #[test]
    fn deduplicated_routes_and_unhandled_counts_share_one_cumulative_allowance() {
        let mut b = builder();
        let mut budget = Budget::default();
        b.record(0, "f", &[], Some((0, &[0])), &[], &mut budget)
            .unwrap();
        let before = budget.work;
        b.record(0, "f", &[], Some((0, &[0])), &[], &mut budget)
            .unwrap();
        assert_eq!(b.records.routes.len(), 1);
        assert!(budget.work > before);
        assert_eq!(b.records.contexts.len(), 2);
        b.record(0, "f", &[], None, &[], &mut budget).unwrap();
        assert_eq!(b.records.unhandled, 1);
        assert_eq!(b.records.unsupported_unhandled, 1);
        budget.work = 999_999;
        assert_eq!(
            b.record(0, "f", &[], Some((0, &[0])), &[], &mut budget)
                .unwrap_err()
                .code,
            "budget"
        );
        assert_eq!(b.records.routes.len(), 1);
    }
}
