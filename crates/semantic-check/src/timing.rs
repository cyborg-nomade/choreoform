// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Typed timer uses and required live host judgments; no clock read or eligibility.
use super::*;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};

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

#[derive(Debug)]
pub struct TimingReport {
    concurrency: ConcurrencyReport,
    uses: BTreeSet<TimerUse>,
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
            "live-calendar-activation-and-retry-clock-basis-bindings",
            "live-clock-pause-reschedule-membership-and-lifecycle-enforcement",
            "exceptional-and-non-data-invalidation-and-lifecycle-enforcement",
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
            return Err(Diagnostic {
                code: "retry-clock-basis-unproved",
                path: format!("/body/policies/{policy}/body/retry"),
            });
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
        checked,
        uncovered,
    })
}
