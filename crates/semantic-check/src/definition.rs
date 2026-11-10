// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource, digest};
mod capture;
mod creation;
mod fault_entry;
mod fault_source;
mod retained_request;
pub use capture::DeclaredWaitCapture;
pub use fault_entry::DeclaredFaultEntry;
pub use fault_source::DeclaredFaultSourceUse;
pub use retained_request::DeclaredRetainedRequest;
mod lifecycle;
mod nodes;
mod policies;

/// Static stage evidence only. This type has no plan/execute methods or authority.
#[derive(Debug)]
pub struct Report {
    revision: String,
    expressions: usize,
    policies: usize,
    captures: Vec<DeclaredWaitCapture>,
    fault_sources: Vec<DeclaredFaultSourceUse>,
    fault_entries: Vec<DeclaredFaultEntry>,
    retained_requests: Vec<DeclaredRetainedRequest>,
}
impl Report {
    pub fn revision(&self) -> &str {
        &self.revision
    }
    pub fn expressions(&self) -> usize {
        self.expressions
    }
    pub fn policies(&self) -> usize {
        self.policies
    }
    /// Every declared capture, including unvisited sites; no activation/acceptance.
    pub fn capture_bindings(&self) -> &[DeclaredWaitCapture] {
        &self.captures
    }
    /// All declared general provider-source uses, even unvisited; no actual faults.
    pub fn fault_source_uses(&self) -> &[DeclaredFaultSourceUse] {
        &self.fault_sources
    }
    /// Reserved local history declarations, not initialized values or entry proof.
    pub fn fault_entries(&self) -> &[DeclaredFaultEntry] {
        &self.fault_entries
    }
    /// Immutable original-input declarations, not actual retained values.
    pub fn retained_requests(&self) -> &[DeclaredRetainedRequest] {
        &self.retained_requests
    }
    /// Explicit residual obligations: a static-stage report cannot close these.
    pub fn remaining(&self) -> &'static [&'static str] {
        &[
            "complete-control-flow-and-definite-output-analysis",
            "complete-information-flow-and-access-authorization",
            "race-and-invalidation-proofs",
            "host-trust-and-atomic-runtime-enforcement",
            "full-benchmark-and-conformance-evidence",
        ]
    }
}

/// Check closed suite payloads and selected links/types/dependencies. Never return
/// an executable/admitted definition; see Report::remaining for residual gates.
pub fn check(envelope: &DefinitionEnvelope, supplied: &[Resource<'_>]) -> Result<Report> {
    envelope.check_structure().map_err(|_| err("structure"))?;
    require(
        envelope.document()["revision"] == digest(envelope.canonical().as_bytes()),
        "revision",
    )?;
    let b = &envelope.document()["body"];
    let mut resources = Resources::new(supplied)?;
    let semantics = Pin::decode(&b["semantics"])?;
    require(
        (semantics.id.as_str(), semantics.revision.as_str()) == SEMANTICS,
        "semantic-contract",
    )?;
    resources.verify(&semantics)?;
    let advanced_continuation = continuation_core(b);
    for pin in object(&b["dialects"])?.values() {
        let p = Pin::decode(pin)?;
        // Wire 0.3 entry fields must never be ignored by an older dialect.
        // The new core is admitted only once its exact contract is implemented.
        if envelope.document()["version"] == "0.4.0" {
            require(
                p.id == if advanced_continuation {
                    CORE_V9.0
                } else {
                    CORE_V8.0
                },
                "retained-request-wire-version",
            )?;
        }
        if envelope.document()["version"] == "0.3.0" {
            require(p.id == CORE_V7.0, "fault-entry-wire-version")?;
        }
        let contract = (p.id.as_str(), p.revision.as_str());
        let level = CORE_PROFILES
            .iter()
            .position(|known| *known == contract)
            .ok_or_else(|| err("dialect-contract"))?;
        resources.verify(&p)?;
        if p.id == CORE_V8.0 || p.id == CORE_V9.0 {
            require(
                envelope.document()["version"] == "0.4.0",
                "retained-request-wire-version",
            )?;
        } else if p.id == CORE_V7.0 {
            require(
                envelope.document()["version"] == "0.3.0",
                "fault-entry-wire-version",
            )?;
        } else if level >= 4 {
            require(
                envelope.document()["version"] == "0.2.0",
                "capture-wire-version",
            )?;
        }
        // Exact closed list, newest inherited artifact first; preserve old order.
        for base in CORE_PROFILES[..level].iter().rev() {
            resources.verify(&Pin {
                id: base.0.into(),
                revision: base.1.into(),
            })?;
        }
    }
    let mut c = Check {
        b,
        resources,
        cells: BTreeMap::new(),
        expressions: BTreeMap::new(),
        policy_reads: BTreeMap::new(),
        protection_refs: BTreeMap::new(),
        creation_reads: BTreeMap::new(),
        provider_refs: BTreeMap::new(),
        captures: Vec::new(),
        capture_types: BTreeSet::new(),
        fault_sources: Vec::new(),
        fault_entries: Vec::new(),
        retained_requests: Vec::new(),
    };
    c.run()?;
    if matches!(
        envelope.document()["version"].as_str(),
        Some("0.3.0" | "0.4.0")
    ) {
        c.fault_entry_declarations()?;
    }
    if envelope.document()["version"] == "0.4.0" {
        c.retained_request_declarations()?;
    }
    Ok(Report {
        revision: text(&envelope.document()["revision"])?.into(),
        expressions: c.expressions.len(),
        policies: c.policy_reads.len(),
        captures: c.capture_bindings(text(&envelope.document()["revision"])?)?,
        fault_sources: c.fault_source_bindings(text(&envelope.document()["revision"])?)?,
        fault_entries: c.fault_entry_bindings(text(&envelope.document()["revision"])?)?,
        retained_requests: c.retained_request_bindings(text(&envelope.document()["revision"])?)?,
    })
}
struct Check<'a, 'r> {
    b: &'a Value,
    resources: Resources<'r>,
    cells: BTreeMap<String, Type>,
    expressions: BTreeMap<String, Judgment>,
    policy_reads: BTreeMap<String, BTreeSet<String>>,
    protection_refs: BTreeMap<String, BTreeSet<String>>,
    creation_reads: BTreeMap<String, BTreeSet<String>>,
    provider_refs: BTreeMap<String, BTreeMap<Pin, BTreeSet<String>>>,
    captures: Vec<capture::PendingCapture>,
    fault_sources: Vec<fault_source::PendingFaultSource>,
    fault_entries: Vec<fault_entry::PendingFaultEntry>,
    capture_types: BTreeSet<(capture::CaptureTypeSubject, Pin, Pin)>,
    retained_requests: Vec<retained_request::PendingRetainedRequest>,
}
impl Check<'_, '_> {
    fn run(&mut self) -> Result<()> {
        // Structural inspection guarantees all fixed record fields/maps exist.
        for (id, record) in object(&self.b["data"])? {
            let ty = self.wrapped(&record["type"]).map_err(|mut e| {
                e.path = format!("/body/data/{id}/type");
                e
            })?;
            self.cells.insert(id.clone(), ty);
        }
        for scope in object(&self.b["scopes"])?.keys() {
            (|| require(self.ancestors(scope)?.len() <= MAX_DEPTH, "scope-depth"))()
                .map_err(|e| e.at_path(format!("/body/scopes/{scope}/parent")))?;
        }
        let mut fanout_parameters: BTreeMap<String, Type> = BTreeMap::new();
        for node in object(&self.b["nodes"])?
            .values()
            .filter(|n| n["kind"] == "fanout")
        {
            let item = text(&node["item"])?;
            let key = text(&node["itemKey"])?;
            let ty = self.cells[item].clone();
            if let Some(old) = fanout_parameters.insert(key.into(), ty.clone()) {
                require(old == ty, "fanout-parameter-type")?;
            }
        }
        for (id, r) in object(&self.b["expressions"])? {
            let result = (|| {
                let scope = text(&r["scope"])?;
                self.dialect(r)?;
                let mut context = self.context(scope)?;
                for (name, ty) in object(&r["parameters"])? {
                    context.parameters.insert(name.clone(), self.wrapped(ty)?);
                }
                if let Some(item) = fanout_parameters.get(id) {
                    require(
                        context.parameters == BTreeMap::from([("item".into(), item.clone())]),
                        "parameter-binding",
                    )?;
                } else {
                    require(context.parameters.is_empty(), "parameter-binding")?;
                }
                let inferred = self.resources.expression(&r["body"], &context)?;
                require(
                    inferred.ty == self.wrapped(&r["resultType"])?,
                    "expression-result",
                )?;
                require(inferred.reads == names(&r["reads"])?, "expression-reads")?;
                if fanout_parameters.contains_key(id) {
                    require(inferred.reads.is_empty(), "fanout-key-reads")?;
                }
                Ok(inferred)
            })();
            self.expressions.insert(
                id.clone(),
                result.map_err(|mut e: Diagnostic| {
                    e.path = format!("/body/expressions/{id}");
                    e
                })?,
            );
        }
        self.policies()?;
        self.nodes()?;
        self.lifecycle_links()?;
        Ok(())
    }
    fn ancestors(&self, scope: &str) -> Result<BTreeSet<String>> {
        let mut cursor = Some(scope);
        let mut result = BTreeSet::new();
        while let Some(s) = cursor {
            require(
                result.insert(s.into()) && result.len() <= MAX_DEPTH,
                "scope-cycle",
            )?;
            cursor = self.b["scopes"][s]["parent"].as_str();
        }
        Ok(result)
    }
    fn visible(&self, scope: &str, map: &str, id: &Value) -> Result<String> {
        let id = text(id)?;
        let record = object(&self.b[map])?
            .get(id)
            .ok_or_else(|| err("reference"))?;
        require(
            self.ancestors(scope)?.contains(text(&record["scope"])?),
            "visibility",
        )?;
        Ok(id.into())
    }
    fn dialect(&self, r: &Value) -> Result<()> {
        require(
            object(&self.b["dialects"])?.contains_key(text(&r["dialect"])?),
            "dialect-reference",
        )
    }
    fn wrapped(&mut self, v: &Value) -> Result<Type> {
        fields(v, &["dialect", "body"])?;
        self.dialect(v)?;
        self.resources.type_descriptor(&v["body"])
    }
    fn context(&self, scope: &str) -> Result<Context> {
        let owners = self.ancestors(scope)?;
        Ok(Context {
            cells: self
                .cells
                .iter()
                .filter(|(id, _)| {
                    self.b["data"][*id]["scope"]
                        .as_str()
                        .is_some_and(|s| owners.contains(s))
                })
                .map(|(id, t)| (id.clone(), t.clone()))
                .collect(),
            ..Context::default()
        })
    }
    fn policy_kind(&self, scope: &str, id: &Value, kind: &str) -> Result<String> {
        let id = self.visible(scope, "policies", id)?;
        require(
            self.b["policies"][&id]["body"]["kind"] == kind,
            "policy-kind",
        )?;
        Ok(id)
    }
    fn actor(&self, scope: &str, id: &Value) -> Result<String> {
        let id = self.visible(scope, "actors", id)?;
        self.policy_kind(
            text(&self.b["actors"][&id]["scope"])?,
            &self.b["actors"][&id]["requirement"],
            "actor",
        )
    }
    fn protection(&mut self, scope: &str, p: &Value) -> Result<BTreeSet<String>> {
        fields(
            p,
            &[
                "policy",
                "sensitivity",
                "purposes",
                "participants",
                "capabilities",
            ],
        )?;
        let count = object(&p["participants"])?.len() + object(&p["capabilities"])?.len();
        let members = object(&p["purposes"])?.len();
        self.resources
            .budget
            .charge(count.saturating_mul(4) + members + 1)?;
        name(&p["sensitivity"])?;
        require(!names(&p["purposes"])?.is_empty(), "protection-purposes")?;
        let mut dependencies =
            BTreeSet::from([self.policy_kind(scope, &p["policy"], "protection")?]);
        for id in names(&p["participants"])? {
            dependencies.insert(self.actor(scope, &Value::from(id))?);
        }
        for id in names(&p["capabilities"])? {
            dependencies.extend(self.capability_refs(scope, &Value::from(id))?);
        }
        Ok(dependencies)
    }
    fn data_protection(&mut self, id: &str) -> Result<BTreeSet<String>> {
        let refs = self
            .protection_refs
            .get(id)
            .ok_or_else(|| err("protection-reference"))?;
        self.resources.budget.charge(refs.len() + 1)?;
        Ok(refs.clone())
    }
    fn provider_protection(&mut self, scope: &str, pin: &Pin) -> Result<BTreeSet<String>> {
        if self
            .provider_refs
            .get(scope)
            .and_then(|sources| sources.get(pin))
            .is_none()
        {
            let provider = self.resources.provider(pin)?;
            let refs = self.protection(scope, provider.protection())?;
            self.provider_refs
                .entry(scope.into())
                .or_default()
                .insert(pin.clone(), refs);
        }
        // Only immutable definition-local references are cached, by source AND
        // lexical scope. No predicate truth or current authority is cached.
        let refs = &self.provider_refs[scope][pin];
        self.resources.budget.charge(refs.len() + 1)?;
        Ok(refs.clone())
    }
    fn capability_refs(&self, scope: &str, id: &Value) -> Result<BTreeSet<String>> {
        let id = self.visible(scope, "capabilities", id)?;
        let c = &self.b["capabilities"][&id];
        let scope = text(&c["scope"])?;
        Ok(BTreeSet::from([
            self.policy_kind(scope, &c["contract"], "capability")?,
            self.policy_kind(scope, &c["effects"], "effect")?,
            self.actor(scope, &c["authority"])?,
        ]))
    }
}
