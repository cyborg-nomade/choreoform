// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

impl Check<'_, '_> {
    pub(super) fn creation_dependencies(&mut self) -> Result<()> {
        let mut seeds: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for (scope, s) in object(&self.b["scopes"])? {
            self.resources.budget.tick(0)?;
            let mut cells = BTreeSet::new();
            for cell in object(&s["inputs"])?.values() {
                self.resources.budget.tick(0)?;
                cells.insert(text(cell)?.to_owned());
            }
            seeds.insert(scope.clone(), cells);
            self.creation_reads.insert(scope.clone(), BTreeSet::new());
        }
        for (cell, data) in object(&self.b["data"])? {
            self.resources.budget.tick(0)?;
            if let Some(initial) = data.get("initial") {
                let scope = text(&data["scope"])?;
                let reads = &self.expressions[text(initial)?].reads;
                self.resources.budget.charge(reads.len() * 2 + 1)?;
                self.creation_reads
                    .get_mut(scope)
                    .unwrap()
                    .extend(reads.iter().cloned());
                let cells = seeds.get_mut(scope).unwrap();
                cells.insert(cell.clone());
                cells.extend(reads.iter().cloned());
            }
        }
        for (scope, cells) in seeds {
            let mut reads = self.creation_reads.remove(&scope).unwrap();
            self.resources.budget.charge(cells.len())?;
            let mut queue: Vec<_> = cells.into_iter().collect();
            let mut seen = BTreeSet::new();
            while let Some(cell) = queue.pop() {
                self.resources.budget.tick(0)?;
                if !seen.insert(cell.clone()) {
                    continue;
                }
                for policy in self.data_protection(&cell)? {
                    let deps = &self.policy_reads[&policy];
                    self.resources.budget.charge(deps.len())?;
                    for read in deps {
                        if reads.insert(read.clone()) {
                            queue.push(read.clone());
                        }
                    }
                }
            }
            self.resources.budget.charge(reads.len())?;
            // Fresh locals are checked in their own occurrence, never caller storage.
            reads.retain(|cell| self.b["data"][cell]["scope"] != scope);
            self.creation_reads.insert(scope, reads);
        }
        Ok(())
    }
}
