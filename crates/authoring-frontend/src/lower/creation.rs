// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

type Cells = BTreeSet<String>;
type Footprints = BTreeMap<String, Cells>;

pub(super) fn charge(work: &mut usize, count: usize) -> Result<()> {
    *work = work
        .checked_add(count)
        .filter(|n| *n <= crate::MAX_WORK)
        .ok_or_else(|| error("work-limit", 0..0))?;
    Ok(())
}

impl Lower<'_, '_> {
    pub(super) fn creation_dependencies(
        &self,
        closure: &Footprints,
        work: &mut usize,
    ) -> Result<Footprints> {
        let mut inputs: BTreeMap<String, (Cells, Cells)> = BTreeMap::new();
        for (scope, s) in self.body["scopes"]
            .as_object()
            .ok_or_else(|| error("scope-reference", 0..0))?
        {
            charge(work, 1)?;
            let mut seeds = Cells::new();
            for cell in s["inputs"]
                .as_object()
                .ok_or_else(|| error("scope-inputs", 0..0))?
                .values()
            {
                charge(work, 1)?;
                seeds.insert(
                    cell.as_str()
                        .ok_or_else(|| error("input-cell", 0..0))?
                        .into(),
                );
            }
            inputs.insert(scope.clone(), (Cells::new(), seeds));
        }
        for (cell, data) in self.body["data"]
            .as_object()
            .ok_or_else(|| error("data-reference", 0..0))?
        {
            charge(work, 1)?;
            if let Some(initial) = data.get("initial") {
                let scope = data["scope"]
                    .as_str()
                    .ok_or_else(|| error("scope-reference", 0..0))?;
                let expr = initial
                    .as_str()
                    .ok_or_else(|| error("expression-reference", 0..0))?;
                let deps = self.body["expressions"][expr]["reads"]
                    .as_object()
                    .ok_or_else(|| error("expression-reference", 0..0))?;
                charge(work, deps.len() * 2 + 1)?;
                let (reads, seeds) = inputs
                    .get_mut(scope)
                    .ok_or_else(|| error("scope-reference", 0..0))?;
                reads.extend(deps.keys().cloned());
                seeds.extend(deps.keys().cloned());
                seeds.insert(cell.clone());
            }
        }
        let mut footprints = Footprints::new();
        for (scope, (mut reads, seeds)) in inputs {
            charge(work, seeds.len())?;
            let mut queue: Vec<_> = seeds.into_iter().collect();
            let mut seen = Cells::new();
            while let Some(cell) = queue.pop() {
                charge(work, 1)?;
                if !seen.insert(cell.clone()) {
                    continue;
                }
                let protection = &self.body["data"][&cell]["protection"];
                for field in ["participants", "capabilities"] {
                    charge(
                        work,
                        protection[field]
                            .as_object()
                            .ok_or_else(|| error("protection-access", 0..0))?
                            .len()
                            * 4
                            + 1,
                    )?;
                }
                let policies = self.protection_policy_refs(protection)?;
                charge(work, policies.len())?;
                for policy in policies {
                    let deps = closure
                        .get(&policy)
                        .ok_or_else(|| error("policy-reference", 0..0))?;
                    charge(work, deps.len())?;
                    for read in deps {
                        if reads.insert(read.clone()) {
                            queue.push(read.clone());
                        }
                    }
                }
            }
            charge(work, reads.len())?;
            let mut ancestors = Cells::new();
            for cell in reads {
                let owner = self.body["data"][&cell]["scope"]
                    .as_str()
                    .ok_or_else(|| error("data-reference", 0..0))?;
                if owner != scope {
                    ancestors.insert(cell);
                }
            }
            footprints.insert(scope, ancestors);
        }
        Ok(footprints)
    }
}
