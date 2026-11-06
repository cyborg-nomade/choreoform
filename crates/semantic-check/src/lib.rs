// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

//! Closed ADR-0013 static judgments, NOT full-definition admission or a planner.
//! No IO, network, clock, authentication, effect dispatch or executable permission.
mod access;
mod concurrency;
mod control;
mod definition;
mod exceptional;
mod expression;
mod faults;
mod initialization;
mod invalidation;
mod protection;
mod reconciliation;
mod resources;
mod timing;
mod types;
pub use concurrency::{ConcurrencyReport, check_concurrency};
pub use control::{ControlReport, check_control};
pub use definition::{Report, check};
pub use exceptional::{ExceptionalReport, check_exceptional_availability};
pub use expression::{Context, Judgment};
pub use faults::{FaultReport, FaultRoute, FaultSelection, check_fault_routes};
pub use invalidation::{InvalidationObligation, InvalidationReport, check_invalidation};
pub use protection::{AccessObligation, ProtectionReport, check_protection};
pub use reconciliation::{ReconciliationReport, ReconciliationUse, check_reconciliation_links};
pub use resources::{
    CALENDAR_PROFILE, CLOCK_PROFILE, Calendar, Clock, InstanceCalendar, Provider, Resources,
};
pub use timing::{CalendarUse, RetryUse, TimerUse, TimingReport, check_timing};
pub use types::{Nominal, Pin, Type};

use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const CORE: (&str, &str) = (
    "urn:choreoform:contracts:core-profile:0.1.0",
    "sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58",
);
pub const SEMANTICS: (&str, &str) = choreoform_ir_probe_core::SUPPORTED_CONTRACTS[0];
pub const CORE_V2: (&str, &str) = (
    "urn:choreoform:contracts:core-profile:0.2.0",
    "sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623",
);
const MAX_DEPTH: usize = 64;
const MAX_ITEMS: usize = 10_000;

/// Fixed codes and declaration-level paths, never literal values or host facts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub path: String,
}
pub type Result<T> = std::result::Result<T, Diagnostic>;
fn err(code: &'static str) -> Diagnostic {
    Diagnostic {
        code,
        path: String::new(),
    }
}
fn require(yes: bool, code: &'static str) -> Result<()> {
    if yes { Ok(()) } else { Err(err(code)) }
}
fn object(v: &Value) -> Result<&Map<String, Value>> {
    v.as_object().ok_or_else(|| err("shape"))
}
fn text(v: &Value) -> Result<&str> {
    v.as_str().ok_or_else(|| err("shape"))
}
fn name(v: &Value) -> Result<&str> {
    let s = text(v)?;
    require(!s.is_empty() && s.chars().count() <= 4096, "name")?;
    Ok(s)
}
fn fields(v: &Value, keys: &[&str]) -> Result<()> {
    let m = object(v)?;
    require(
        m.len() == keys.len() && keys.iter().all(|k| m.contains_key(*k)),
        "fields",
    )
}
fn items(v: &Value) -> Result<&Vec<Value>> {
    v.as_array()
        .filter(|a| a.len() <= MAX_ITEMS)
        .ok_or_else(|| err("collection"))
}
fn names(v: &Value) -> Result<BTreeSet<String>> {
    let m = object(v)?;
    require(m.len() <= MAX_ITEMS, "collection")?;
    m.iter()
        .map(|(k, v)| {
            name(&Value::from(k.as_str()))?;
            require(v == true, "set")?;
            Ok(k.clone())
        })
        .collect()
}
fn integer(v: &Value) -> Result<i64> {
    let s = text(v)?;
    let digits = s.strip_prefix('-').unwrap_or(s);
    require(
        !digits.is_empty()
            && digits.bytes().all(|b| b.is_ascii_digit())
            && (!digits.starts_with('0') || s == "0"),
        "integer-encoding",
    )?;
    s.parse().map_err(|_| err("integer-range"))
}
fn scale(v: &Value) -> Result<u8> {
    v.as_u64()
        .filter(|n| *n <= 18)
        .map(|n| n as u8)
        .ok_or_else(|| err("decimal-scale"))
}

#[derive(Default)]
struct Budget {
    work: usize,
}
impl Budget {
    fn charge(&mut self, amount: usize) -> Result<()> {
        self.work = self.work.checked_add(amount).ok_or_else(|| err("budget"))?;
        require(self.work <= 1_000_000, "budget")
    }
    fn tick(&mut self, depth: usize) -> Result<()> {
        require(depth <= MAX_DEPTH, "budget")?;
        self.charge(1)
    }
}
