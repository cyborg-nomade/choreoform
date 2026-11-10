// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! New-core work declarations, never dispatch authority or an accepted effect.
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OwnedWorkEffect {
    ReadOnly,
    Compensatable,
    Irreversible,
}
impl OwnedWorkEffect {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read-only",
            Self::Compensatable => "compensatable",
            Self::Irreversible => "irreversible",
        }
    }
}

/// Bound by the containing scope/report revision; contains no actual request.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnedContinuationWork {
    site: String,
    capability: String,
    input: String,
    effect: OwnedWorkEffect,
    compensates: Option<String>,
}
impl OwnedContinuationWork {
    pub fn site(&self) -> &str {
        &self.site
    }
    pub fn capability(&self) -> &str {
        &self.capability
    }
    pub fn input(&self) -> &str {
        &self.input
    }
    pub fn effect_class(&self) -> OwnedWorkEffect {
        self.effect
    }
    pub fn compensates(&self) -> Option<&str> {
        self.compensates.as_deref()
    }
    pub fn is_query(&self) -> bool {
        self.effect == OwnedWorkEffect::ReadOnly && self.compensates.is_none()
    }
    /// Conditional requirements apply according to class/link, never accepted facts.
    pub fn host_requirements(&self) -> &'static [&'static str] {
        &[
            "actual-accepted-P6-registration-atomic-handoff-current-grants-open-receiver-and-cutoff-before-entry",
            "new-business-work-and-compensation-use-new-logical-effect-key-only-technical-attempts-keep-identity",
            "implemented-exact-provider-and-declared-idempotency-promise-atomic-accepted-request-key-recording",
            "exact-typed-accepted-input-value-revisions-digest-occurrence-effect-key-attempt-fresh-immutable-storage-and-complete-provenance",
            "commands-and-explicit-compensators-require-actual-accepted-correlated-source-capture-before-dispatch",
            "appropriate-business-invariants-and-consequential-irreversible-conflict-acceptance",
            "if-explicit-compensation-actual-eligible-prior-effect-and-exact-accepted-occurrence-not-class-tag-or-declaration",
            "compensation-is-new-work-no-history-erasure-or-guaranteed-restoration",
            "failed-receiver-acceptance-keeps-old-owner-accountable-all-activated-transitive-effects-subscriptions-children-until-settlement",
            "no-metadata-activation-parent-resumption-original-source-settlement-terminal-reopening-or-liveness",
        ]
    }
}

/// Normal definition admission already checks pure input typing/read closure and
/// exact provider/effect/source compatibility. Conditional flow checks run next.
pub(super) fn check(
    b: &Value,
    site: &str,
    n: &Value,
    scope: &str,
    source: &str,
    budget: &mut Budget,
) -> Result<OwnedContinuationWork> {
    budget.tick(0)?;
    require(
        n["mode"] == "capability",
        "owned-continuation-work-unproved",
    )?;
    let capability = text(&n["target"])?;
    let cap = &b["capabilities"][capability];
    let effect = &b["policies"][text(&cap["effects"])?]["body"];
    let class = match text(&effect["class"])? {
        "read-only" => OwnedWorkEffect::ReadOnly,
        "compensatable" => OwnedWorkEffect::Compensatable,
        "irreversible" => OwnedWorkEffect::Irreversible,
        _ => return Err(at("owned-continuation-work-effect", scope)),
    };
    require(
        class == OwnedWorkEffect::ReadOnly || effect["idempotency"] == "stable-key",
        "owned-continuation-work-idempotency",
    )?;
    let policy = &b["policies"][text(&n["policy"])?]["body"];
    require(
        policy["retry"]["maxAttempts"] == 1
            && policy["timeout"].is_null()
            && !policy["faultSource"].is_null(),
        "owned-continuation-work-policy",
    )?;
    let compensates = policy["compensates"].as_str();
    require(
        compensates.is_none_or(|prior| prior == source),
        "owned-continuation-compensation-source",
    )?;
    budget.charge(object(&n["reads"])?.len() + 6 + usize::from(compensates.is_some()))?;
    Ok(OwnedContinuationWork {
        site: site.into(),
        capability: capability.into(),
        input: text(&n["input"])?.into(),
        effect: class,
        compensates: compensates.map(str::to_owned),
    })
}
