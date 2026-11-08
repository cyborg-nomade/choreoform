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
mod fault_entry;
mod faults;
mod initialization;
mod invalidation;
mod policy_reads;
mod protection;
mod reconciliation;
mod reconciliation_entry;
mod resources;
mod timing;
mod types;
pub use concurrency::{
    ConcurrencyReport, IsolatedFollowup, OwnedLocalFollowup, OwnedLocalReport,
    OwnedRequestSnapshot, check_concurrency,
};
pub use control::{ControlReport, check_control};
pub use definition::{
    DeclaredFaultEntry, DeclaredFaultSourceUse, DeclaredRetainedRequest, DeclaredWaitCapture,
    Report, check,
};
pub use exceptional::{ExceptionalReport, check_exceptional_availability};
pub use expression::{Context, Judgment};
pub use fault_entry::{FaultEntryReport, check_fault_entries};
pub use faults::{
    FaultOriginReport, FaultOriginRoute, FaultOriginSite, FaultReport, FaultRoute, FaultSelection,
    check_fault_origins, check_fault_routes,
};
pub use invalidation::{
    HistoricalOrigin, InvalidationObligation, InvalidationReport, InvalidationRequirements,
    check_invalidation, derive_invalidation_requirements,
};
pub use policy_reads::{PolicyReadClosure, PolicyReadDependencies, policy_read_closure};
pub use protection::{
    AccessObligation, ProtectionReport, RequiredProtectionProvenance, check_protection,
};
pub use reconciliation::{ReconciliationReport, ReconciliationUse, check_reconciliation_links};
pub use reconciliation_entry::{
    ReconciliationEntry, ReconciliationEntryReport, check_reconciliation_availability,
};
pub use resources::{
    CALENDAR_PROFILE, CLOCK_PROFILE, Calendar, Clock, FAULT_HISTORY_PROFILE, FAULT_HISTORY_TYPES,
    FAULT_SOURCE_PROFILE, FaultEmission, FaultEvidence, InstanceCalendar, Provider,
    ProviderFaultHistory, ProviderFaultSource, ProviderObservationState, ProviderTerminalSource,
    Resources, TERMINAL_OBSERVATION_PROFILE, TerminalFactTag, TerminalObservation,
};
pub use timing::{
    CalendarUse, RetryFaultBinding, RetryFaultDeclaration, RetryUse, TimerUse, TimingReport,
    check_timing,
};
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
pub const CORE_V3: (&str, &str) = (
    "urn:choreoform:contracts:core-profile:0.3.0",
    "sha256:30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3",
);
pub const CORE_V4: (&str, &str) = (
    "urn:choreoform:contracts:core-profile:0.4.0",
    "sha256:0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3",
);

pub const CORE_V5: (&str, &str) = (
    "urn:choreoform:contracts:core-profile:0.5.0",
    "sha256:50007ac05e5e71df91bb9997a40a06c7a8993df04b1a673421e4a987014299d7",
);

pub const CORE_V6: (&str, &str) = (
    "urn:choreoform:contracts:core-profile:0.6.0",
    "sha256:27bce3fde9a0393abb6bf760a8588af9268de65069b73f4cb946144f5d30cb85",
);
pub const CORE_V7: (&str, &str) = (
    "urn:choreoform:contracts:core-profile:0.7.0",
    "sha256:e286e54441b3d785ae823eef359c12d64fb8405829c70709227282e5af62f54b",
);
pub const CORE_V8: (&str, &str) = (
    "urn:choreoform:contracts:core-profile:0.8.0",
    "sha256:edc8195ba0c31536ab2cdc95089e8aecc3ff402489639b4435277f0a38e7b603",
);
const CORE_PROFILES: &[(&str, &str)] = &[
    CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, CORE_V6, CORE_V7, CORE_V8,
];
fn general_source_core(id: &Value) -> bool {
    matches!(id.as_str(), Some(id) if id==CORE_V6.0 || id==CORE_V7.0 || id==CORE_V8.0)
}
fn retry_source_core(id: &Value) -> bool {
    matches!(id.as_str(), Some(id) if id==CORE_V4.0 || id==CORE_V5.0 || id==CORE_V6.0 || id==CORE_V7.0 || id==CORE_V8.0)
}
fn capture_core(id: &Value) -> bool {
    matches!(id.as_str(), Some(id) if id==CORE_V5.0 || id==CORE_V6.0 || id==CORE_V7.0 || id==CORE_V8.0)
}

fn metadata_core(id: &Value) -> bool {
    matches!(id.as_str(), Some(id) if id == CORE_V3.0 || id == CORE_V4.0 || id == CORE_V5.0 || id == CORE_V6.0 || id == CORE_V7.0 || id == CORE_V8.0)
}

const MAX_DEPTH: usize = 64;
const MAX_ITEMS: usize = 10_000;

/// Fixed codes and declaration-level paths, never literal values or host facts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub path: String,
}
impl Diagnostic {
    fn at_path(mut self, path: String) -> Self {
        if self.path.is_empty() {
            self.path = path;
        }
        self
    }
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
