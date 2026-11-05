// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Definition-site links only; live ownership/correlation require the host.
use super::*;

impl Check<'_, '_> {
    pub(super) fn lifecycle_links(&mut self) -> Result<()> {
        for (id, s) in object(&self.b["scopes"])? {
            self.resources.budget.tick(0)?;
            let faults = &self.b["policies"][text(&s["faults"])?]["body"];
            for child in object(&faults["handlers"])?.values() {
                self.resources.budget.tick(0)?;
                if self.b["scopes"][text(child)?]["parent"] != *id {
                    return Err(site("fault-handler-context", "scopes", id, "faults"));
                }
            }
            let closure = &self.b["policies"][text(&s["closure"])?]["body"];
            if closure["mode"] == "reconcile"
                && self.b["scopes"][text(&closure["followup"])?]["parent"] != *id
            {
                return Err(site("closure-followup-context", "scopes", id, "closure"));
            }
        }
        let mut users: BTreeMap<&str, usize> = BTreeMap::new();
        for n in object(&self.b["nodes"])?.values() {
            self.resources.budget.tick(0)?;
            if n["kind"] == "wait" {
                *users.entry(text(&n["policy"])?).or_default() += 1;
            }
        }
        for (id, p) in object(&self.b["policies"])? {
            self.resources.budget.tick(0)?;
            if p["body"]["kind"] != "effect" {
                continue;
            }
            let wait = text(&p["body"]["reconciliation"])?;
            let count = users.get(wait).copied().unwrap_or(0);
            let code = match count {
                0 => Some("reconciliation-owner-missing"),
                1 => None,
                _ => Some("reconciliation-owner-unproved"),
            };
            if let Some(code) = code {
                return Err(site(code, "policies", id, "body/reconciliation"));
            }
        }
        Ok(())
    }
}
fn site(code: &'static str, map: &str, id: &str, field: &str) -> Diagnostic {
    Diagnostic {
        code,
        path: format!("/body/{map}/{id}/{field}"),
    }
}
