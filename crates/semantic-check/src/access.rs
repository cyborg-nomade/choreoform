// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Static access-metadata dependencies only; no runtime authority or predicate truth.
use super::*;

pub(crate) fn policy_reads(b: &Value, p: &Value, budget: &mut Budget) -> Result<BTreeSet<String>> {
    let mut policies = BTreeSet::from([text(&p["policy"])?.to_owned()]);
    let actors = names(&p["participants"])?;
    let capabilities = names(&p["capabilities"])?;
    budget.charge(actors.len() + capabilities.len() + 1)?;
    for actor in actors {
        policies.insert(text(&b["actors"][actor]["requirement"])?.into());
    }
    for cap in capabilities {
        let c = &b["capabilities"][cap];
        for field in ["contract", "effects"] {
            policies.insert(text(&c[field])?.into());
        }
        policies.insert(text(&b["actors"][text(&c["authority"])?]["requirement"])?.into());
    }
    let mut reads = BTreeSet::new();
    for policy in policies {
        let set = names(&b["policies"][policy]["body"]["reads"])?;
        budget.charge(set.len() + 1)?;
        reads.extend(set);
    }
    Ok(reads)
}

pub(crate) fn cell_reads(b: &Value, cell: &str, budget: &mut Budget) -> Result<BTreeSet<String>> {
    let mut reads = BTreeSet::new();
    let mut seen = BTreeSet::new();
    let mut queue = vec![cell.to_owned()];
    while let Some(id) = queue.pop() {
        budget.tick(0)?;
        if !seen.insert(id.clone()) {
            continue;
        }
        for read in policy_reads(b, &b["data"][id]["protection"], budget)? {
            if reads.insert(read.clone()) {
                queue.push(read);
            }
        }
    }
    Ok(reads)
}
