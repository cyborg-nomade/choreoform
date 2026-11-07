// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use crate::control::graph::Graph;
use std::borrow::Cow;
use std::collections::VecDeque;

struct Scope<'a> {
    id: &'a str,
    parent: Option<usize>,
    handlers: BTreeMap<usize, usize>,
    children: BTreeSet<usize>,
}
struct Selection<'a> {
    owner: usize,
    policy: &'a str,
    fault: usize,
    handler: usize,
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    scope: usize,
    active: Vec<usize>,
}
pub(super) struct Analysis<'a> {
    scopes: Vec<Scope<'a>>,
    names: Vec<Cow<'a, str>>,
    selections: Vec<Selection<'a>>,
    root: usize,
    budget: Budget,
}
impl<'a> Analysis<'a> {
    pub fn new(b: &'a Value, supplied: &[choreoform_ir_probe_core::Resource<'_>]) -> Result<Self> {
        let mut budget = Budget::default();
        let graph = Graph::build(b, &mut budget)?;
        let reachable = graph.reachable(b, &mut budget)?;
        let records = object(&b["scopes"])?;
        let index: BTreeMap<&str, usize> = records
            .keys()
            .enumerate()
            .map(|(i, id)| (id.as_str(), i))
            .collect();
        budget.charge(index.len())?;
        let mut names = BTreeSet::new();
        let mut resources = Resources::new(supplied)?;
        let mut sources = BTreeSet::new();
        for p in object(&b["policies"])?.values() {
            budget.tick(0)?;
            if p["body"]["kind"] == "faults" {
                for name in object(&p["body"]["handlers"])?.keys() {
                    budget.tick(0)?;
                    names.insert(Cow::Borrowed(name.as_str()));
                }
            }
            if !matches!(b["dialects"][text(&p["dialect"])?]["id"].as_str(),Some(id) if id==CORE_V4.0 || id==CORE_V5.0)
                || p["body"]["kind"] != "work"
                || p["body"]["retry"]["faultSource"].is_null()
            {
                continue;
            }
            let pin = Pin::decode(&p["body"]["retry"]["faultSource"])?;
            if sources.insert(pin.clone()) {
                // Resource decoding and graph/name copying share one allowance.
                resources.budget.work = budget.work;
                let decoded = resources.provider_fault_source(&pin);
                budget.work = resources.budget.work;
                let source = decoded?;
                for name in source.emissions().keys() {
                    budget.charge(2)?;
                    names.insert(Cow::Owned(name.clone()));
                }
            }
        }
        require(names.len() <= MAX_ITEMS, "budget")?;
        let names: Vec<_> = names.into_iter().collect();
        let fault_index: BTreeMap<&str, _> = names
            .iter()
            .enumerate()
            .map(|(i, n)| (n.as_ref(), i))
            .collect();
        let mut scopes = Vec::new();
        let mut selections = Vec::new();
        for (id, s) in records {
            budget.tick(0)?;
            let owner = index[id.as_str()];
            let policy = text(&s["faults"])?;
            let mut handlers = BTreeMap::new();
            for (fault, handler) in object(&b["policies"][policy]["body"]["handlers"])? {
                budget.tick(0)?;
                let fault = fault_index[fault.as_str()];
                handlers.insert(fault, selections.len());
                selections.push(Selection {
                    owner,
                    policy,
                    fault,
                    handler: index[text(handler)?],
                });
            }
            // Only syntactically reachable ordinary sites can create templates.
            // No data availability or predicate truth is assigned to this walk.
            let mut children = BTreeSet::new();
            for node in reachable
                .get(id.as_str())
                .ok_or_else(|| err("scope-entry"))?
            {
                budget.tick(0)?;
                let n = &b["nodes"][*node];
                match n["kind"].as_str() {
                    Some("invoke" | "repeat" | "fanout") => {
                        children.insert(index[text(&n["body"])?]);
                    }
                    Some("split") => {
                        for child in object(&n["children"])?.values() {
                            budget.tick(0)?;
                            children.insert(index[text(child)?]);
                        }
                    }
                    _ => (),
                }
            }
            scopes.push(Scope {
                id,
                parent: s["parent"].as_str().map(|p| index[p]),
                handlers,
                children,
            });
        }
        Ok(Self {
            scopes,
            names,
            selections,
            root: index[text(&b["root"])?],
            budget,
        })
    }
    fn enqueue(
        &mut self,
        state: State,
        seen: &mut BTreeSet<State>,
        pending: &mut VecDeque<State>,
    ) -> Result<()> {
        self.budget.charge(state.active.len() + 1)?;
        if !seen.contains(&state) {
            self.budget.charge(2 * (state.active.len() + 1))?;
            seen.insert(state.clone());
            pending.push_back(state);
        }
        Ok(())
    }
    fn select(
        &mut self,
        origin: usize,
        fault: usize,
        active: &[usize],
    ) -> Result<Option<(usize, Vec<usize>)>> {
        self.budget.charge(active.len() + 1)?;
        let mut retained = active.to_vec();
        let mut excluded = BTreeSet::new();
        let mut cursor = Some(origin);
        let mut depth = 0;
        while let Some(scope) = cursor {
            self.budget.tick(depth)?;
            depth += 1;
            if let Some(selection) = self.scopes[scope].handlers.get(&fault).copied()
                && !excluded.contains(&selection)
            {
                self.budget.tick(retained.len())?;
                retained.push(selection);
                return Ok(Some((selection, retained)));
            }
            // Local handling precedes escaping this handler root. Leaving it
            // fails this selection, which this propagation must not reselect.
            while let Some(selection) = retained.last().copied()
                && self.selections[selection].handler == scope
            {
                self.budget.tick(0)?;
                retained.pop();
                excluded.insert(selection);
            }
            cursor = self.scopes[scope].parent;
        }
        Ok(None)
    }
    fn acyclic(&mut self, edges: &[BTreeSet<usize>]) -> Result<()> {
        self.budget.charge(edges.len())?;
        let mut degrees = vec![0usize; edges.len()];
        for targets in edges {
            for to in targets {
                self.budget.tick(0)?;
                degrees[*to] += 1;
            }
        }
        let mut ready: Vec<_> = degrees
            .iter()
            .enumerate()
            .filter(|(_, n)| **n == 0)
            .map(|(i, _)| i)
            .collect();
        while let Some(from) = ready.pop() {
            self.budget.tick(0)?;
            for to in &edges[from] {
                self.budget.tick(0)?;
                degrees[*to] -= 1;
                if degrees[*to] == 0 {
                    ready.push(*to);
                }
            }
        }
        if let Some(i) = degrees.iter().position(|n| *n > 0) {
            return Err(Diagnostic {
                code: "fault-routing-cycle-unproved",
                path: format!(
                    "/body/scopes/{}/faults",
                    self.scopes[self.selections[i].owner].id
                ),
            });
        }
        Ok(())
    }
    pub fn run(mut self, timing: TimingReport) -> Result<FaultReport> {
        let mut seen = BTreeSet::new();
        let mut pending = VecDeque::new();
        self.enqueue(
            State {
                scope: self.root,
                active: Vec::new(),
            },
            &mut seen,
            &mut pending,
        )?;
        let mut covered = BTreeSet::new();
        let mut selected = BTreeSet::new();
        let mut routes = BTreeSet::new();
        let mut unhandled = 0;
        self.budget.charge(self.selections.len())?;
        let mut edges = vec![BTreeSet::new(); self.selections.len()];
        while let Some(state) = pending.pop_front() {
            self.budget.tick(0)?;
            covered.insert(state.scope);
            self.budget
                .charge(self.scopes[state.scope].children.len())?;
            let children: Vec<_> = self.scopes[state.scope].children.iter().copied().collect();
            for child in children {
                self.budget.charge(state.active.len() + 1)?;
                self.enqueue(
                    State {
                        scope: child,
                        active: state.active.clone(),
                    },
                    &mut seen,
                    &mut pending,
                )?;
            }
            for fault in 0..self.names.len() {
                self.budget.tick(0)?;
                if let Some((target, active)) = self.select(state.scope, fault, &state.active)? {
                    self.budget.tick(0)?;
                    let from = state.active.last().copied();
                    routes.insert(Route {
                        origin: state.scope,
                        from,
                        selected: target,
                    });
                    selected.insert(target);
                    if let Some(from) = from {
                        edges[from].insert(target);
                    }
                    self.enqueue(
                        State {
                            scope: self.selections[target].handler,
                            active,
                        },
                        &mut seen,
                        &mut pending,
                    )?;
                } else {
                    unhandled += 1;
                }
            }
        }
        self.acyclic(&edges)?;
        self.budget
            .charge(self.scopes.len() + self.selections.len())?;
        let all_scopes: Vec<_> = self.scopes.iter().map(|s| s.id.to_owned()).collect();
        let checked = covered.iter().map(|i| all_scopes[*i].clone()).collect();
        let uncovered = all_scopes
            .iter()
            .enumerate()
            .filter(|(i, _)| !covered.contains(i))
            .map(|(_, s)| s.clone())
            .collect();
        let selections = self
            .selections
            .iter()
            .map(|s| FaultSelection {
                owner: all_scopes[s.owner].clone(),
                policy: s.policy.into(),
                fault: self.names[s.fault].to_string(),
                handler: all_scopes[s.handler].clone(),
            })
            .collect();
        Ok(FaultReport {
            timing,
            scopes: all_scopes,
            selections,
            routes,
            covered: checked,
            uncovered,
            selected,
            contexts: seen.len(),
            unhandled,
        })
    }
}
