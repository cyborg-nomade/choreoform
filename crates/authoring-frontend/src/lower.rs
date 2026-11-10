// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

use crate::{Ledger, MAX_DEPTH, Result, Syntax, Tree, error, parse};
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource, digest};
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};

/// Original source and strict companion. Construction grants no semantic authority.
#[derive(Debug)]
pub struct Package<'a> {
    syntax: Syntax<'a>,
    ledger: Ledger,
}

/// Immutable revision-bearing candidate; never a validated/executable definition.
/// Metadata cannot be replaced independently of the candidate document:
/// ```compile_fail
/// use choreoform_authoring_frontend::Candidate;
/// fn tamper(candidate: &mut Candidate) {
///     candidate.source_digest = String::new();
/// }
/// ```
/// The source map is exposed only as a shared borrow, even to an owning caller:
/// ```compile_fail
/// use choreoform_authoring_frontend::Candidate;
/// fn tamper(mut candidate: Candidate) {
///     candidate.spans().clear();
/// }
/// ```
#[derive(Debug)]
pub struct Candidate {
    envelope: DefinitionEnvelope,
    source_digest: String,
    bindings_digest: String,
    package_digest: String,
    spans: BTreeMap<String, std::ops::Range<usize>>,
    entry_spans: BTreeMap<String, std::ops::Range<usize>>,
    request_spans: BTreeMap<String, std::ops::Range<usize>>,
}
/// Package integrity only; independent of binding consistency and IR admission.
#[derive(Debug, PartialEq, Eq)]
pub struct Integrity {
    pub source: String,
    pub bindings: String,
    pub package: String,
}
impl Candidate {
    /// Borrow the unvalidated IR envelope; this does not authorize execution.
    pub fn document(&self) -> &Value {
        self.envelope.document()
    }
    /// Consume this candidate, returning its still-unvalidated IR envelope.
    pub fn into_document(self) -> Value {
        self.envelope.into_document()
    }
    /// Digest of the exact original UTF-8 source bytes, not its semantic revision.
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }
    /// Digest of the strict canonical companion used to lower this candidate.
    pub fn bindings_digest(&self) -> &str {
        &self.bindings_digest
    }
    /// Digest of the length-framed original source and canonical companion.
    pub fn package_digest(&self) -> &str {
        &self.package_digest
    }
    /// Borrow declaration/generated-record spans in the original source's UTF-8 bytes.
    pub fn spans(&self) -> &BTreeMap<String, std::ops::Range<usize>> {
        &self.spans
    }
    /// Locate a diagnostic from this candidate at its original declaration or
    /// generated-record span, or explicit fault-input/retained-request clause. Other fields do not
    /// have exact token locations. This is not proof
    /// that a diagnostic supplied by the caller belongs to this revision.
    /// Global, external-resource and missing-record paths have no guessed span.
    pub fn source_span(
        &self,
        diagnostic: &choreoform_semantic_check::Diagnostic,
    ) -> Option<std::ops::Range<usize>> {
        let mut parts = diagnostic.path.strip_prefix("/body/")?.split('/');
        let table = parts.next()?;
        if !matches!(
            table,
            "scopes"
                | "data"
                | "expressions"
                | "actors"
                | "capabilities"
                | "policies"
                | "nodes"
                | "flows"
        ) {
            return None;
        }
        let id = parts.next()?;
        self.document()["body"].get(table)?.as_object()?.get(id)?;
        let field = parts.next();
        if table == "scopes"
            && field == Some("faultInput")
            && let Some(span) = self.entry_spans.get(id)
        {
            return Some(span.clone());
        }
        if table == "scopes"
            && field == Some("retainedRequests")
            && let Some(span) = self.request_spans.get(id)
        {
            return Some(span.clone());
        }
        self.spans.get(id).cloned()
    }
    /// Run explicit static suite checks. A report still grants no planning or
    /// execution authority; full-definition and host obligations remain open.
    pub fn check_static(
        &self,
        resources: &[Resource<'_>],
    ) -> choreoform_semantic_check::Result<choreoform_semantic_check::Report> {
        choreoform_semantic_check::check(&self.envelope, resources)
    }
    /// Prove conservative ordinary control/initialization obligations. Root input
    /// cells are launch preconditions; this report never authorizes execution.
    pub fn check_control(
        &self,
        resources: &[Resource<'_>],
    ) -> choreoform_semantic_check::Result<choreoform_semantic_check::ControlReport> {
        choreoform_semantic_check::check_control(&self.envelope, resources)
    }
    /// Check conservative ordinary protection constraints and enumerate access
    /// obligations; this does not authenticate a principal or grant access.
    pub fn check_protection(
        &self,
        resources: &[Resource<'_>],
    ) -> choreoform_semantic_check::Result<choreoform_semantic_check::ProtectionReport> {
        choreoform_semantic_check::check_protection(&self.envelope, resources)
    }
    /// Ordinary advisory requirements; missing links never produce a checked report.
    pub fn invalidation_requirements(
        &self,
        supplied: &[Resource<'_>],
    ) -> choreoform_semantic_check::Result<choreoform_semantic_check::InvalidationRequirements>
    {
        choreoform_semantic_check::derive_invalidation_requirements(&self.envelope, supplied)
    }
    /// Check ordinary invalidation declarations; never execute withdrawal,
    /// authorize work or establish current runtime sufficiency.
    pub fn check_invalidation(
        &self,
        resources: &[Resource<'_>],
    ) -> choreoform_semantic_check::Result<choreoform_semantic_check::InvalidationReport> {
        choreoform_semantic_check::check_invalidation(&self.envelope, resources)
    }
    /// Check conservative ordinary parallel independence. This does not prove
    /// domain invariants or authorize concurrent effects.
    pub fn check_concurrency(
        &self,
        resources: &[Resource<'_>],
    ) -> choreoform_semantic_check::Result<choreoform_semantic_check::ConcurrencyReport> {
        choreoform_semantic_check::check_concurrency(&self.envelope, resources)
    }
    /// Check typed ordinary timer uses and expose live host requirements; no
    /// observations, current authority or execution permission are accepted.
    pub fn check_timing(
        &self,
        resources: &[Resource<'_>],
    ) -> choreoform_semantic_check::Result<choreoform_semantic_check::TimingReport> {
        choreoform_semantic_check::check_timing(&self.envelope, resources)
    }
    /// Check conservative possible handler selection, not initialized handler
    /// contexts, authenticated faults or runtime dispatch/ownership.
    pub fn check_fault_routes(
        &self,
        resources: &[Resource<'_>],
    ) -> choreoform_semantic_check::Result<choreoform_semantic_check::FaultReport> {
        choreoform_semantic_check::check_fault_routes(&self.envelope, resources)
    }
    /// Check selected handler bodies under explicit ancestor input contracts;
    /// never establish a live fault snapshot, current access or dispatch authority.
    pub fn check_exceptional_availability(
        &self,
        resources: &[Resource<'_>],
    ) -> choreoform_semantic_check::Result<choreoform_semantic_check::ExceptionalReport> {
        choreoform_semantic_check::check_exceptional_availability(&self.envelope, resources)
    }
    /// Check surviving reconciliation declaration links, never live ownership,
    /// provider facts, wait input availability or accepted atomic handoff.
    pub fn check_reconciliation_links(
        &self,
        resources: &[Resource<'_>],
    ) -> choreoform_semantic_check::Result<choreoform_semantic_check::ReconciliationReport> {
        choreoform_semantic_check::check_reconciliation_links(&self.envelope, resources)
    }
    /// Check conditional follow-up bodies with zero inherited values, never live
    /// storage, authority, accepted ownership, correlation or execution permission.
    pub fn check_reconciliation_availability(
        &self,
        resources: &[Resource<'_>],
    ) -> choreoform_semantic_check::Result<choreoform_semantic_check::ReconciliationEntryReport>
    {
        choreoform_semantic_check::check_reconciliation_availability(&self.envelope, resources)
    }
}
impl<'a> Package<'a> {
    pub fn parse(source: &'a [u8], companion: &[u8]) -> Result<Self> {
        let syntax = parse(source)?;
        let ledger = Ledger::decode(companion)?;
        if ledger.document["profile"] != syntax.profile() {
            return Err(error("package-profile", 0..0));
        }
        Ok(Self { syntax, ledger })
    }
    pub fn syntax(&self) -> &Syntax<'a> {
        &self.syntax
    }
    pub fn ledger(&self) -> &Ledger {
        &self.ledger
    }
    pub fn integrity(&self) -> Integrity {
        let source = self.syntax.source().as_bytes();
        let bindings = self.ledger.canonical().as_bytes();
        let mut framed =
            format!("choreoform-authoring-package:{}\0", self.syntax.profile()).into_bytes();
        framed.extend_from_slice(&(source.len() as u64).to_be_bytes());
        framed.extend_from_slice(source);
        framed.extend_from_slice(&(bindings.len() as u64).to_be_bytes());
        framed.extend_from_slice(bindings);
        Integrity {
            source: digest(source),
            bindings: digest(bindings),
            package: digest(&framed),
        }
    }
    /// Format this original source package. This is not an arbitrary IR exporter.
    pub fn formatted(&self) -> Result<(String, String)> {
        Ok((self.syntax.formatted()?, self.ledger.canonical().into()))
    }
    /// Bind exact inventories and references, then lower to UNVALIDATED candidate IR.
    /// All pins must have uniquely supplied matching bytes. No runtime policy follows.
    pub fn lower(&self, resources: &[Resource<'_>]) -> Result<Candidate> {
        self.ledger.verify_resources(resources)?;
        let mut lowering = Lower::new(&self.syntax, &self.ledger, resources)?;
        let root = self.syntax.tree().one("process")?;
        lowering.scope(root, None, 0)?;
        lowering.resolve_waits()?;
        for (tree, scope) in std::mem::take(&mut lowering.pending_steps) {
            lowering.step(tree, &scope)?;
        }
        lowering.validate_endpoints()?;
        lowering.finish_inventory()?;
        lowering.dependencies()?;
        let mut body = lowering.body;
        body.insert("id".into(), self.ledger.document["definition"].clone());
        body.insert("root".into(), self.ledger.document["root"].clone());
        let contracts = &self.ledger.document["contracts"];
        body.insert("semantics".into(), json!({"id": contracts["semantics"]["contract"], "revision": contracts["semantics"]["digest"]}));
        body.insert("dialects".into(), json!({ self.ledger.dialect(): {"id": contracts["core"]["contract"], "revision": contracts["core"]["digest"]} }));
        let envelope = DefinitionEnvelope::from_parts_versioned(
            if matches!(self.syntax.profile(), "0.11.0" | "0.12.0") {
                "0.4.0"
            } else if self.syntax.profile() == "0.10.0" {
                "0.3.0"
            } else if matches!(self.syntax.profile(), "0.8.0" | "0.9.0") {
                "0.2.0"
            } else {
                "0.1.0"
            },
            Value::Object(body),
            self.ledger.document["annotations"].clone(),
        )
        .map_err(|_| error("candidate-transport", 0..self.syntax.source().len()))?;
        let integrity = self.integrity();
        Ok(Candidate {
            envelope,
            source_digest: integrity.source,
            bindings_digest: integrity.bindings,
            package_digest: integrity.package,
            spans: lowering.spans,
            entry_spans: lowering.entry_spans,
            request_spans: lowering.request_spans,
        })
    }
}

mod creation;
mod declarations;
mod expressions;
mod graph;
mod nodes;
mod policies;
mod wait_endpoints;
use expressions::ast_reads;

struct Lower<'a, 's> {
    syntax: &'a Syntax<'s>,
    ledger: &'a Ledger,
    resources: choreoform_semantic_check::Resources<'a>,
    body: Map<String, Value>,
    declarations: BTreeSet<String>,
    symbols: BTreeSet<String>,
    slots: BTreeSet<String>,
    spans: BTreeMap<String, std::ops::Range<usize>>,
    entry_spans: BTreeMap<String, std::ops::Range<usize>>,
    request_spans: BTreeMap<String, std::ops::Range<usize>>,
    pending_steps: Vec<(&'a Tree, String)>,
    join_selections: BTreeMap<String, Vec<BTreeSet<String>>>,
    wait_owners: BTreeMap<String, String>,
    pending_waits: Vec<(&'a Tree, String, String)>,
    exported_endpoints: Vec<(String, String, String, std::ops::Range<usize>)>,
}

fn set(items: impl IntoIterator<Item = String>) -> Value {
    Value::Object(items.into_iter().map(|s| (s, Value::Bool(true))).collect())
}
fn insert(map: &mut Map<String, Value>, name: String, value: Value) -> Result<()> {
    if map.insert(name, value).is_some() {
        return Err(error("duplicate-map-key", 0..0));
    }
    Ok(())
}
fn unique_set(items: Vec<String>) -> Result<Value> {
    if items.iter().collect::<BTreeSet<_>>().len() != items.len() {
        return Err(error("duplicate-set-member", 0..0));
    }
    Ok(set(items))
}
fn integer(spelling: &str) -> Result<String> {
    let digits = spelling.strip_prefix('-').unwrap_or(spelling);
    if digits.is_empty()
        || !digits.bytes().all(|c| c.is_ascii_digit())
        || (digits.starts_with('0') && spelling != "0")
    {
        return Err(error("integer", 0..0));
    }
    spelling
        .parse::<i64>()
        .map(|n| n.to_string())
        .map_err(|_| error("integer-range", 0..0))
}
fn scale(spelling: &str) -> Result<u8> {
    spelling
        .parse::<u8>()
        .ok()
        .filter(|n| *n <= 18)
        .ok_or_else(|| error("decimal-scale", 0..0))
}
fn decimal(spelling: &str, places: u8) -> Result<String> {
    if spelling.split_once('.').map_or(0, |(_, tail)| tail.len()) != usize::from(places)
        || (places == 0 && spelling.contains('.'))
    {
        return Err(error("decimal-scale", 0..0));
    }
    let negative = spelling.starts_with('-');
    let digits = spelling.trim_start_matches('-').replace('.', "");
    let digits = digits.trim_start_matches('0');
    if digits.is_empty() {
        return if negative {
            Err(error("decimal-negative-zero", 0..0))
        } else {
            Ok("0".into())
        };
    }
    integer(&format!("{}{digits}", if negative { "-" } else { "" }))
}

impl<'a, 's> Lower<'a, 's> {
    fn new(
        syntax: &'a Syntax<'s>,
        ledger: &'a Ledger,
        supplied: &'a [Resource<'a>],
    ) -> Result<Self> {
        let mut resources =
            choreoform_semantic_check::Resources::new(supplied).map_err(|e| error(e.code, 0..0))?;
        for r in ledger.document["resources"]
            .as_array()
            .expect("checked resources")
        {
            let pin = choreoform_semantic_check::Pin {
                id: r["contract"].as_str().expect("contract").into(),
                revision: r["digest"].as_str().expect("digest").into(),
            };
            match r["kind"].as_str().expect("resource kind") {
                "named-type" => resources
                    .named_types(&pin)
                    .map_err(|e| error(e.code, 0..0))?,
                "provider" => {
                    resources.provider(&pin).map_err(|e| error(e.code, 0..0))?;
                }
                "clock" => {
                    resources.clock(&pin).map_err(|e| error(e.code, 0..0))?;
                }
                "calendar" => {
                    resources
                        .instance_calendar(&pin)
                        .map_err(|e| error(e.code, 0..0))?;
                }
                "provider-fault-source" => {
                    resources
                        .provider_fault_source(&pin)
                        .map_err(|e| error(e.code, 0..0))?;
                }
                "provider-fault-history" => {
                    resources
                        .provider_fault_history(&pin)
                        .map_err(|e| error(e.code, 0..0))?;
                }
                "provider-terminal-observation-source" => {
                    resources
                        .provider_terminal_source(&pin)
                        .map_err(|e| error(e.code, 0..0))?;
                }
                _ => return Err(error("external-contract-lowering-unsupported", 0..0)),
            }
        }
        Ok(Self {
            syntax,
            ledger,
            resources,
            body: [
                "scopes",
                "data",
                "expressions",
                "actors",
                "capabilities",
                "policies",
                "nodes",
                "flows",
            ]
            .into_iter()
            .map(|k| (k.into(), json!({})))
            .collect(),
            declarations: BTreeSet::new(),
            symbols: BTreeSet::new(),
            slots: BTreeSet::new(),
            spans: BTreeMap::new(),
            entry_spans: BTreeMap::new(),
            request_spans: BTreeMap::new(),
            pending_steps: Vec::new(),
            join_selections: BTreeMap::new(),
            wait_owners: BTreeMap::new(),
            pending_waits: Vec::new(),
            exported_endpoints: Vec::new(),
        })
    }
    fn put(&mut self, section: &str, id: String, record: Value) -> Result<()> {
        insert(
            self.body
                .get_mut(section)
                .and_then(Value::as_object_mut)
                .expect("known map"),
            id,
            record,
        )
    }
    fn names(&self, tree: &Tree) -> Result<Vec<String>> {
        tree.all("NAME")
            .iter()
            .map(|n| self.syntax.text(n).map(str::to_owned))
            .collect()
    }
    fn parts<'n>(&'n self, tree: &'n Tree) -> Result<Vec<&'n str>> {
        tree.all("NAME")
            .iter()
            .map(|n| self.syntax.text(n))
            .collect()
    }
    fn reference(&self, tree: &Tree, scope: &str, kind: &str) -> Result<String> {
        self.ledger
            .resolve(scope, &self.parts(tree)?, kind)
            .map(|r| r.id.clone())
            .map_err(|mut e| {
                e.span = self.syntax.span(tree);
                e
            })
    }
    fn r(&self, tree: &Tree, scope: &str, kind: &str, i: usize) -> Result<String> {
        self.reference(
            tree.all("reference")
                .get(i)
                .ok_or_else(|| error("internal-tree", self.syntax.span(tree)))?,
            scope,
            kind,
        )
    }
    fn declare(&mut self, tree: &Tree, parent: Option<&str>, kind: &str) -> Result<String> {
        let name = self.syntax.text(
            tree.all("NAME")
                .first()
                .ok_or_else(|| error("internal-tree", self.syntax.span(tree)))?,
        )?;
        let row = self
            .ledger
            .declaration(parent, name, kind)
            .map_err(|mut e| {
                e.span = self.syntax.span(tree);
                e
            })?;
        if !self.declarations.insert(row.id.clone()) {
            return Err(error("source-duplicate", self.syntax.span(tree)));
        }
        self.spans.insert(row.id.clone(), self.syntax.span(tree));
        Ok(row.id.clone())
    }
    fn symbol(&mut self, owner: &str, role: &str, name: &str) -> Result<(String, String)> {
        let row = self.ledger.symbol(owner, role, name)?;
        Ok((row.id.clone(), row.wire.clone()))
    }
    fn declared_symbol(&mut self, owner: &str, role: &str, name: &str) -> Result<(String, String)> {
        let symbol = self.symbol(owner, role, name)?;
        if !self.symbols.insert(symbol.0.clone()) {
            return Err(error("source-symbol-duplicate", 0..0));
        }
        Ok(symbol)
    }
    fn slot(&mut self, tree: &Tree, owner: &str, role: &str, key: Option<&str>) -> Result<String> {
        let row = self.ledger.slot(owner, role, key)?;
        if !self.slots.insert(row.id.clone()) {
            return Err(error("source-slot-duplicate", self.syntax.span(tree)));
        }
        self.spans.insert(row.id.clone(), self.syntax.span(tree));
        Ok(row.id.clone())
    }
    fn finish_inventory(&self) -> Result<()> {
        let expected: BTreeSet<_> = self
            .ledger
            .declarations
            .values()
            .filter(|r| r.kind != "resource")
            .map(|r| r.id.clone())
            .collect();
        if self.declarations != expected
            || self.symbols != self.ledger.symbols.keys().cloned().collect()
            || self.slots != self.ledger.slots.iter().map(|s| s.id.clone()).collect()
        {
            return Err(error(
                "source-ledger-inventory",
                0..self.syntax.source().len(),
            ));
        }
        Ok(())
    }
}
