// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

#[derive(Debug, Default)]
pub(super) struct Summary {
    pub reads: BTreeSet<String>,
    pub writes: BTreeSet<String>,
    pub observations: bool,
    pub effects: bool,
}
impl Summary {
    pub fn shared(&self, b: &Value, child: &str, budget: &mut Budget) -> Result<Self> {
        budget.charge(self.reads.len() + self.writes.len() + 1)?;
        // Each summary contains only its own storage and lexical ancestors.
        // Every occurrence gets fresh own storage, including reused templates.
        Ok(Self {
            reads: self
                .reads
                .iter()
                .filter(|id| b["data"][*id]["scope"] != child)
                .cloned()
                .collect(),
            writes: self
                .writes
                .iter()
                .filter(|id| b["data"][*id]["scope"] != child)
                .cloned()
                .collect(),
            observations: self.observations,
            effects: self.effects,
        })
    }
    pub fn build(
        b: &Value,
        contexts: &crate::protection::DependencyMap,
        budget: &mut Budget,
    ) -> Result<BTreeMap<String, Self>> {
        let mut direct = BTreeMap::new();
        let mut children: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        let mut order = Vec::new();
        for (scope, s) in object(&b["scopes"])? {
            budget.tick(0)?;
            let mut summary = Self::default();
            for cell in object(&s["inputs"])?.values() {
                budget.tick(0)?;
                let cell = text(cell)?;
                summary.writes.insert(cell.to_owned());
                summary
                    .reads
                    .extend(crate::access::cell_reads(b, cell, budget)?);
            }
            direct.insert(scope.clone(), summary);
            if let Some(parent) = s["parent"].as_str() {
                children.entry(parent).or_default().push(scope);
            }
            let mut depth = 0;
            let mut visible = BTreeSet::from([scope.as_str()]);
            let mut parent = s["parent"].as_str();
            while let Some(id) = parent {
                budget.tick(depth)?;
                depth += 1;
                visible.insert(id);
                parent = b["scopes"][id]["parent"].as_str();
            }
            order.push((depth, scope));
            if let Some(reads) = contexts.get(scope) {
                budget.charge(reads.len())?;
                for cell in reads {
                    if !visible.contains(text(&b["data"][cell]["scope"])?) {
                        return Err(Diagnostic {
                            code: "scope-context-unproved",
                            path: format!("/body/scopes/{scope}"),
                        });
                    }
                }
                direct
                    .get_mut(scope)
                    .unwrap()
                    .reads
                    .extend(reads.iter().cloned());
            }
        }
        for (cell, d) in object(&b["data"])? {
            budget.tick(0)?;
            if let Some(initial) = d.get("initial") {
                let record = &b["expressions"][text(initial)?]["reads"];
                budget.charge(object(record)?.len())?;
                let reads = names(record)?;
                let summary = direct.get_mut(text(&d["scope"])?).unwrap();
                summary.writes.insert(cell.clone());
                summary
                    .reads
                    .extend(crate::access::cell_reads(b, cell, budget)?);
                for read in &reads {
                    summary
                        .reads
                        .extend(crate::access::cell_reads(b, read, budget)?);
                }
                summary.reads.extend(reads);
            }
        }
        for n in object(&b["nodes"])?.values() {
            budget.charge(object(&n["reads"])?.len() + object(&n["writes"])?.len() + 1)?;
            let summary = direct.get_mut(text(&n["scope"])?).unwrap();
            summary.reads.extend(names(&n["reads"])?);
            summary.writes.extend(names(&n["writes"])?);
            if n["kind"] == "activity" {
                summary.observations = true;
                if n["mode"] == "human" {
                    summary.effects = true;
                } else {
                    let effects = text(&b["capabilities"][text(&n["target"])?]["effects"])?;
                    summary.effects |= b["policies"][effects]["body"]["class"] != "read-only";
                }
            }
        }
        order.sort_unstable();
        let mut summaries: BTreeMap<String, Self> = BTreeMap::new();
        for (_, scope) in order.into_iter().rev() {
            budget.tick(0)?;
            let mut summary = direct.remove(scope).unwrap();
            for child in children.get(scope.as_str()).into_iter().flatten() {
                let shared = summaries[*child].shared(b, child, budget)?;
                summary.reads.extend(shared.reads);
                summary.writes.extend(shared.writes);
                summary.observations |= shared.observations;
                summary.effects |= shared.effects;
            }
            summaries.insert(scope.clone(), summary);
        }
        Ok(summaries)
    }
}
