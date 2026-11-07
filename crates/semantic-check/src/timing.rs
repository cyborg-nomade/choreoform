// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Typed timer uses and required live host judgments; no clock read or eligibility.
use super::*;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};
mod fault_binding;
pub use fault_binding::{RetryFaultBinding, RetryFaultDeclaration};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimerUse {
    site: String,
    policy: String,
    role: &'static str,
    clock: Pin,
    source: Pin,
    basis: String,
    pause_authority: String,
    calendar: Option<CalendarUse>,
}
impl TimerUse {
    pub fn site(&self) -> &str {
        &self.site
    }
    pub fn policy(&self) -> &str {
        &self.policy
    }
    pub fn role(&self) -> &str {
        self.role
    }
    pub fn clock(&self) -> &Pin {
        &self.clock
    }
    pub fn source(&self) -> &Pin {
        &self.source
    }
    pub fn basis(&self) -> &str {
        &self.basis
    }
    pub fn pause_authority(&self) -> &str {
        &self.pause_authority
    }
    pub fn calendar(&self) -> Option<&CalendarUse> {
        self.calendar.as_ref()
    }
    pub fn host_requirements(&self) -> &'static [&'static str] {
        &[
            "authenticated-source-instance-domain-and-origin-binding",
            "truthful-microseconds-and-durable-monotonic-observation-order",
            "exact-observation-occurrence-and-timer-revision-identity",
            "current-clock-source-result-protection-and-authority",
            "due-and-basis-calculation-in-the-registered-clock-domain",
            "current-basis-revisions-and-access",
            "pause-resume-reschedule-only-with-current-pause-authority",
            "pause-reschedule-overflow-and-replay-enforcement",
            "atomic-fire-cancellation-and-owned-settlement",
        ]
    }
}

/// Exact construction references and required live binding, never its approval.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CalendarUse {
    pin: Pin,
    rules: Pin,
    profile: Pin,
}
impl CalendarUse {
    pub fn pin(&self) -> &Pin {
        &self.pin
    }
    pub fn rules(&self) -> &Pin {
        &self.rules
    }
    pub fn profile(&self) -> &Pin {
        &self.profile
    }
    pub fn host_requirements(&self) -> &'static [&'static str] {
        &[
            "trusted-host-calendar-construction-approval",
            "exact-calendar-rules-profile-clock-and-instance-origin-binding",
            "current-definition-timer-occurrence-and-basis-revisions",
            "durable-binding-recovery-without-origin-reset",
            "calendar-change-only-by-authorized-reschedule",
            "finite-half-open-table-gating-without-fallback",
        ]
    }
}

/// Explicit retry binding metadata, never proof that an attempt may dispatch.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RetryUse {
    site: String,
    policy: String,
    clock: Pin,
    source: Pin,
    basis: String,
    authority: String,
    max_attempts: u64,
    delays: Vec<i64>,
    fault_source: Option<RetryFaultBinding>,
}
impl RetryUse {
    pub fn site(&self) -> &str {
        &self.site
    }
    pub fn policy(&self) -> &str {
        &self.policy
    }
    pub fn clock(&self) -> &Pin {
        &self.clock
    }
    pub fn source(&self) -> &Pin {
        &self.source
    }
    pub fn basis(&self) -> &str {
        &self.basis
    }
    pub fn authority(&self) -> &str {
        &self.authority
    }
    pub fn max_attempts(&self) -> u64 {
        self.max_attempts
    }
    pub fn delays(&self) -> &[i64] {
        &self.delays
    }
    /// None for legacy cores; no source proof is inferred from eligible labels.
    pub fn fault_source(&self) -> Option<&RetryFaultBinding> {
        self.fault_source.as_ref()
    }
    pub fn host_requirements(&self) -> &'static [&'static str] {
        &[
            "authenticated-retry-clock-source-instance-and-registered-origin",
            "accepted-eligible-fault-attempt-correlation-and-domain-stamp",
            "checked-delay-addition-and-durable-monotonic-due-observations",
            "unchanged-original-request-input-and-basis-revisions",
            "current-work-target-source-protection-and-access-authority",
            "stable-key-or-definitive-correlated-no-effect-evidence-as-required",
            "no-retry-after-success-process-cancellation-or-ordinary-closure",
            "atomic-durable-attempt-budget-dispatch-and-owned-reconciliation",
        ]
    }
}

#[derive(Debug)]
pub struct TimingReport {
    concurrency: ConcurrencyReport,
    uses: BTreeSet<TimerUse>,
    retries: BTreeSet<RetryUse>,
    checked: BTreeSet<String>,
    uncovered: BTreeSet<String>,
}
impl TimingReport {
    pub fn revision(&self) -> &str {
        self.concurrency.revision()
    }
    pub fn concurrency(&self) -> &ConcurrencyReport {
        &self.concurrency
    }
    pub fn timer_uses(&self) -> &BTreeSet<TimerUse> {
        &self.uses
    }
    pub fn retry_uses(&self) -> &BTreeSet<RetryUse> {
        &self.retries
    }
    pub fn checked_nodes(&self) -> &BTreeSet<String> {
        &self.checked
    }
    pub fn uncovered_nodes(&self) -> &BTreeSet<String> {
        &self.uncovered
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "parallel-and-exceptional-availability-proofs",
            "exceptional-and-observable-channel-information-flow",
            "current-principal-purpose-capability-and-policy-enforcement",
            "domain-and-irreversible-race-invariant-proofs",
            "exceptional-and-reconciliation-concurrency",
            "live-calendar-activation-and-retry-domain-fault-basis-bindings",
            "live-clock-pause-reschedule-membership-and-lifecycle-enforcement",
            "exceptional-and-non-data-invalidation-and-lifecycle-enforcement",
            "occurrence-bound-ordinary-closure-and-snapshot-enforcement",
            "host-trust-and-atomic-runtime-enforcement",
            "full-benchmark-and-conformance-evidence",
        ]
    }
}

/// Check typed ordinary timer uses and enumerate required runtime judgments.
/// No clock observations, authentication or current host facts are accepted here.
pub fn check_timing(
    envelope: &DefinitionEnvelope,
    supplied: &[Resource<'_>],
) -> Result<TimingReport> {
    let concurrency = check_concurrency(envelope, supplied)?;
    let b = &envelope.document()["body"];
    let covered = concurrency
        .invalidation()
        .protection()
        .control()
        .covered_nodes();
    let mut resources = Resources::new(supplied)?;
    let mut uses = BTreeSet::new();
    let mut retries = BTreeSet::new();
    let mut checked = BTreeSet::new();
    let mut uncovered = BTreeSet::new();
    for (id, n) in object(&b["nodes"])? {
        resources.budget.tick(0)?;
        let role = match n["kind"].as_str() {
            Some("activity") => "timeout",
            Some("wait") => "timer",
            _ => continue,
        };
        let policy = text(&n["policy"])?;
        let p = &b["policies"][policy]["body"];
        let retry = role == "timeout" && p["retry"]["maxAttempts"].as_u64().is_some_and(|n| n > 1);
        if p[role].is_null() && !retry {
            continue;
        }
        if !covered.contains(id) {
            uncovered.insert(id.clone());
            continue;
        }
        if retry {
            if p["retry"]["clock"].is_null() {
                return Err(Diagnostic {
                    code: "retry-clock-basis-unproved",
                    path: format!("/body/policies/{policy}/body/retry"),
                });
            }
            let r = &p["retry"];
            let pin = Pin::decode(&r["clock"])?;
            let clock = resources.clock(&pin)?;
            let mut delays = Vec::new();
            for delay in items(&r["delays"])? {
                resources.budget.tick(0)?;
                delays.push(integer(delay)?);
            }
            let core = &b["dialects"][text(&b["policies"][policy]["dialect"])?]["id"];
            let fault_source = if core == CORE_V4.0 || core == CORE_V5.0 {
                Some(fault_binding::collect(
                    &mut resources,
                    r,
                    id,
                    concurrency.revision(),
                )?)
            } else {
                None
            };
            retries.insert(RetryUse {
                site: id.clone(),
                policy: policy.into(),
                clock: pin,
                source: clock.source().clone(),
                basis: text(&r["basis"])?.into(),
                authority: text(&p["authority"])?.into(),
                max_attempts: r["maxAttempts"]
                    .as_u64()
                    .ok_or_else(|| err("retry-attempts"))?,
                delays,
                fault_source,
            });
            checked.insert(id.clone());
        }
        if p[role].is_null() {
            continue;
        }
        let timer = &p[role];
        let pin = Pin::decode(&timer["clock"])?;
        let clock = resources.clock(&pin)?;
        let calendar = if timer["calendar"].is_null() {
            None
        } else {
            let calendar = resources.instance_calendar(&Pin::decode(&timer["calendar"])?)?;
            Some(CalendarUse {
                pin: calendar.pin().clone(),
                rules: calendar.rules().clone(),
                profile: calendar.profile().clone(),
            })
        };
        uses.insert(TimerUse {
            site: id.clone(),
            policy: policy.into(),
            role,
            clock: pin,
            source: clock.source().clone(),
            basis: text(&timer["basis"])?.into(),
            pause_authority: text(&timer["pauseAuthority"])?.into(),
            calendar,
        });
        checked.insert(id.clone());
    }
    Ok(TimingReport {
        concurrency,
        uses,
        retries,
        checked,
        uncovered,
    })
}
