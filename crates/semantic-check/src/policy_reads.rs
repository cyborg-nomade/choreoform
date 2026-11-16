// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Pure finite declaration closure. Callers validate pins, kinds and visibility.
use super::*;

/// Inputs to read derivation, not a validated policy or authorization decision.
#[derive(Debug, Default)]
pub struct PolicyReadDependencies {
    pub local: BTreeSet<String>,
    pub strong: BTreeSet<String>,
    pub metadata: BTreeSet<String>,
    /// Set only after validating that this record selects the supported core 0.3.
    pub metadata_cycles: bool,
}

/// Derived sets and charged work; contains no values, facts or execution authority.
#[derive(Debug)]
pub struct PolicyReadClosure {
    pub reads: BTreeMap<String, BTreeSet<String>>,
    pub work: usize,
}

struct Work {
    used: usize,
    limit: usize,
}
impl Work {
    fn charge(&mut self, n: usize) -> Result<()> {
        self.used = self.used.checked_add(n).ok_or_else(|| err("budget"))?;
        require(self.used <= self.limit, "budget")
    }
}

/// Compute least read sets with iterative SCC condensation, retaining strong
/// acyclicity and rejecting any cyclic component with a legacy member. This
/// helper does not validate dialect selection, resource bytes or policy payloads.
/// `work_limit` must fit the caller's remaining total budget and be <= 1,000,000;
/// charge the returned work against that same budget, rather than adding a quota.
pub fn policy_read_closure(
    nodes: &BTreeMap<String, PolicyReadDependencies>,
    work_limit: usize,
) -> Result<PolicyReadClosure> {
    require(nodes.len() <= MAX_ITEMS, "collection")?;
    require(work_limit <= 1_000_000, "budget")?;
    let mut work = Work {
        used: 0,
        limit: work_limit,
    };
    work.charge(nodes.len())?;
    let ids: Vec<_> = nodes.keys().collect();
    let index: BTreeMap<_, _> = ids
        .iter()
        .enumerate()
        .map(|(i, id)| (id.as_str(), i))
        .collect();
    let n = ids.len();
    let mut strong = vec![Vec::new(); n];
    let mut edges = vec![Vec::new(); n];
    let mut reverse = vec![Vec::new(); n];
    for (i, node) in nodes.values().enumerate() {
        require(
            node.local.len() <= MAX_ITEMS
                && node.strong.len() <= MAX_ITEMS
                && node.metadata.len() <= MAX_ITEMS,
            "collection",
        )?;
        work.charge(node.local.len() + node.strong.len() + node.metadata.len() + 1)?;
        // An ineligible caller cannot classify old references as weak edges.
        for target in node.strong.union(&node.metadata) {
            let j = *index
                .get(target.as_str())
                .ok_or_else(|| err("policy-reference"))?;
            edges[i].push(j);
            reverse[j].push(i);
            if !node.metadata_cycles || node.strong.contains(target) {
                strong[i].push(j);
            }
        }
    }
    dag_order(&strong, &mut work)?;

    // Iterative Kosaraju: arbitrary graph depth never becomes native recursion.
    let mut visited = vec![false; n];
    let mut finish = Vec::with_capacity(n);
    for start in 0..n {
        work.charge(1)?;
        if visited[start] {
            continue;
        }
        visited[start] = true;
        let mut stack = vec![(start, 0)];
        while let Some((node, cursor)) = stack.last_mut() {
            work.charge(1)?;
            if let Some(&next) = edges[*node].get(*cursor) {
                *cursor += 1;
                if !visited[next] {
                    visited[next] = true;
                    stack.push((next, 0));
                }
            } else {
                finish.push(stack.pop().expect("present frame").0);
            }
        }
    }
    let mut component_of = vec![usize::MAX; n];
    let mut count = 0;
    for &start in finish.iter().rev() {
        work.charge(1)?;
        if component_of[start] != usize::MAX {
            continue;
        }
        let component = count;
        component_of[start] = component;
        let mut size = 0;
        let mut eligible = true;
        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            work.charge(reverse[node].len() + 1)?;
            size += 1;
            eligible &= nodes[ids[node]].metadata_cycles;
            for &next in &reverse[node] {
                if component_of[next] == usize::MAX {
                    component_of[next] = component;
                    stack.push(next);
                }
            }
        }
        let cyclic = size > 1 || edges[start].contains(&start);
        if cyclic && !eligible {
            return Err(err("policy-cycle"));
        }
        count += 1;
    }
    let mut dependencies = vec![BTreeSet::new(); count];
    let mut reads = vec![BTreeSet::new(); count];
    for (i, node) in nodes.values().enumerate() {
        let c = component_of[i];
        work.charge(node.local.len() + edges[i].len() + 1)?;
        reads[c].extend(node.local.iter().cloned());
        for &target in &edges[i] {
            let d = component_of[target];
            if c != d {
                dependencies[c].insert(d);
            }
        }
    }
    let dependencies: Vec<Vec<usize>> = dependencies
        .into_iter()
        .map(|s| s.into_iter().collect())
        .collect();
    for c in dag_order(&dependencies, &mut work)? {
        let mut closure = std::mem::take(&mut reads[c]);
        for &d in &dependencies[c] {
            work.charge(reads[d].len() + 1)?;
            closure.extend(reads[d].iter().cloned());
        }
        require(closure.len() <= MAX_ITEMS, "collection")?;
        reads[c] = closure;
    }
    let mut result = BTreeMap::new();
    for (i, id) in ids.iter().enumerate() {
        work.charge(reads[component_of[i]].len() + 1)?;
        result.insert((*id).clone(), reads[component_of[i]].clone());
    }
    Ok(PolicyReadClosure {
        reads: result,
        work: work.used,
    })
}

/// Dependencies precede users; check depth using the same edge orientation.
fn dag_order(edges: &[Vec<usize>], work: &mut Work) -> Result<Vec<usize>> {
    let mut users = vec![Vec::new(); edges.len()];
    let mut degree = Vec::with_capacity(edges.len());
    for (i, deps) in edges.iter().enumerate() {
        work.charge(deps.len() + 1)?;
        degree.push(deps.len());
        for &target in deps {
            users[target].push(i);
        }
    }
    let mut ready: Vec<_> = degree
        .iter()
        .enumerate()
        .filter_map(|(i, &d)| (d == 0).then_some(i))
        .collect();
    let mut depths = vec![0; edges.len()];
    let mut order = Vec::with_capacity(edges.len());
    while let Some(node) = ready.pop() {
        work.charge(edges[node].len() + users[node].len() + 1)?;
        depths[node] = 1 + edges[node].iter().map(|&d| depths[d]).max().unwrap_or(0);
        require(depths[node] <= MAX_DEPTH, "policy-depth")?;
        order.push(node);
        for &user in &users[node] {
            degree[user] -= 1;
            if degree[user] == 0 {
                ready.push(user);
            }
        }
    }
    require(order.len() == edges.len(), "policy-cycle")?;
    Ok(order)
}
