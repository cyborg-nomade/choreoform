// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Revision-bound declared selections, never physical independence or live calls.
use super::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct DeclaredInterferenceUse {
    definition_revision: String,
    policy: String,
    scope: String,
    descriptor: Arc<ProviderInterference>,
}
impl DeclaredInterferenceUse {
    pub fn definition_revision(&self) -> &str {
        &self.definition_revision
    }
    pub fn policy(&self) -> &str {
        &self.policy
    }
    pub fn scope(&self) -> &str {
        &self.scope
    }
    pub fn descriptor(&self) -> &Arc<ProviderInterference> {
        &self.descriptor
    }
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "generic-original-request-disjointness-for-all-overlapping-surviving-occurrences",
            "truthful-exhaustive-provider-activity-and-terminal-coverage",
            "supported-common-physical-canonicalization-and-exhaustive-alias-evidence",
            "accepted-typed-original-request-input-revision-occurrence-effect-attempt-correlation",
            "current-protection-authority-and-no-declassification",
            "uncertainty-all-attempt-quiescence-and-durable-business-obligations",
            "atomic-authorized-transitive-ownership-handoff-and-settlement",
            "all-other-semantic-host-browser-user-independent-and-phase-gates",
        ]
    }
}
pub(super) struct PendingInterference {
    policy: String,
    scope: String,
    descriptor: Arc<ProviderInterference>,
}
impl Check<'_, '_> {
    pub(super) fn interference_use(
        &mut self,
        policy: &str,
        scope: &str,
        provider: &Pin,
        selection: &Value,
    ) -> Result<()> {
        self.resources.budget.tick(0)?;
        let choice = if selection.is_null() {
            None
        } else {
            Some(Pin::decode(selection)?)
        };
        if let Some(previous) = self.interference_choices.get(provider) {
            require(*previous == choice, "interference-selection-conflict")?;
        }
        if let Some(pin) = &choice {
            let descriptor = self.resources.provider_interference(pin)?;
            require(
                descriptor.provider() == provider,
                "interference-provider-binding",
            )?;
            self.resources.budget.charge(1)?;
            self.pending_interference.push(PendingInterference {
                policy: policy.into(),
                scope: scope.into(),
                descriptor,
            });
        }
        self.interference_choices.insert(provider.clone(), choice);
        Ok(())
    }
    pub(super) fn interference_bindings(
        &mut self,
        revision: &str,
    ) -> Result<Vec<DeclaredInterferenceUse>> {
        self.resources
            .budget
            .charge(self.pending_interference.len())?;
        Ok(std::mem::take(&mut self.pending_interference)
            .into_iter()
            .map(|pending| DeclaredInterferenceUse {
                definition_revision: revision.into(),
                policy: pending.policy,
                scope: pending.scope,
                descriptor: pending.descriptor,
            })
            .collect())
    }
}
