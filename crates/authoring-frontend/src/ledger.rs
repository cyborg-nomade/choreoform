// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

use crate::{MAX_BYTES, MAX_DEPTH, Result, error};
use choreoform_ir_probe_core::{Resource, digest, transport};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) const SEMANTICS: (&str, &str) = choreoform_ir_probe_core::SUPPORTED_CONTRACTS[0];
pub(crate) const CORE: (&str, &str) = (
    "urn:choreoform:contracts:core-profile:0.1.0",
    "sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58",
);

pub(crate) fn object(value: &Value) -> Result<&Map<String, Value>> {
    value.as_object().ok_or_else(|| error("ledger-shape", 0..0))
}
pub(crate) fn string(value: &Value) -> Result<&str> {
    value.as_str().ok_or_else(|| error("ledger-shape", 0..0))
}
fn array(value: &Value) -> Result<&Vec<Value>> {
    value
        .as_array()
        .filter(|v| v.len() <= 10000)
        .ok_or_else(|| error("ledger-shape", 0..0))
}
fn fields(value: &Value, keys: &[&str]) -> Result<()> {
    let map = object(value)?;
    if map.len() != keys.len() || keys.iter().any(|k| !map.contains_key(*k)) {
        return Err(error("ledger-fields", 0..0));
    }
    Ok(())
}
pub(crate) fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id.as_bytes()[0].is_ascii_alphabetic()
        && id
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-'))
}
fn id(value: &Value) -> Result<String> {
    let s = string(value)?;
    if !valid_id(s) {
        return Err(error("identity", 0..0));
    }
    Ok(s.into())
}
fn nullable_id(value: &Value) -> Result<Option<String>> {
    if value.is_null() {
        Ok(None)
    } else {
        id(value).map(Some)
    }
}
fn name(value: &Value) -> Result<String> {
    let s = string(value)?;
    if s.is_empty()
        || s.chars().count() > 4096
        || s.chars()
            .any(|c| c.is_control() || matches!(c, '\u{2028}' | '\u{2029}'))
    {
        return Err(error("binding-name", 0..0));
    }
    Ok(s.into())
}

#[derive(Debug)]
pub(crate) struct Declaration {
    pub id: String,
    pub kind: String,
    pub scope: Option<String>,
    pub name: String,
}
#[derive(Debug)]
pub(crate) struct Symbol {
    pub id: String,
    pub owner: String,
    pub role: String,
    pub name: String,
    pub wire: String,
}
#[derive(Debug)]
pub(crate) struct Slot {
    pub id: String,
    pub owner: String,
    pub role: String,
    pub key: Option<String>,
}

/// Strict, immutable identity ledger. This does not compare its inventory to source.
#[derive(Debug)]
pub struct Ledger {
    pub(crate) document: Value,
    pub(crate) declarations: BTreeMap<String, Declaration>,
    pub(crate) symbols: BTreeMap<String, Symbol>,
    pub(crate) slots: Vec<Slot>,
    canonical: String,
    names: BTreeMap<(Option<String>, String), String>,
    symbol_names: BTreeMap<(String, String, String), String>,
    slot_keys: BTreeMap<(String, String, Option<String>), usize>,
}

impl Ledger {
    /// Decode the entire portable ledger; reject unknown fields and identity defects.
    pub fn decode(raw: &[u8]) -> Result<Self> {
        // Legacy IR permits numeric -0; this companion profile deliberately does not.
        let mut quoted = false;
        let mut escape = false;
        for (i, &c) in raw.iter().enumerate() {
            if quoted {
                if escape {
                    escape = false;
                } else if c == b'\\' {
                    escape = true;
                } else if c == b'"' {
                    quoted = false;
                }
            } else if c == b'"' {
                quoted = true;
            } else if c == b'-' && raw.get(i + 1) == Some(&b'0') {
                return Err(error("ledger-negative-zero", 0..0));
            }
        }
        let (document, canonical) =
            transport::decode_canonical(raw).map_err(|_| error("ledger-transport", 0..0))?;
        if canonical.len() > MAX_BYTES {
            return Err(error("size", 0..0));
        }
        fields(
            &document,
            &[
                "format",
                "version",
                "profile",
                "definition",
                "root",
                "contracts",
                "declarations",
                "generated",
                "symbols",
                "retired",
                "resources",
                "annotations",
            ],
        )?;
        if document["format"] != "choreoform-authoring-bindings"
            || document["version"] != "0.1.0"
            || document["profile"] != "0.2.0"
        {
            return Err(error("ledger-profile", 0..0));
        }
        let mut used = BTreeSet::from([id(&document["definition"])?]);
        let root = id(&document["root"])?;
        object(&document["annotations"])?;
        fields(&document["contracts"], &["semantics", "core"])?;
        for (kind, expected) in [("semantics", SEMANTICS), ("core", CORE)] {
            let pin = &document["contracts"][kind];
            fields(
                pin,
                if kind == "core" {
                    &["contract", "digest", "dialect"]
                } else {
                    &["contract", "digest"]
                },
            )?;
            if pin["contract"] != expected.0 || pin["digest"] != expected.1 {
                return Err(error("contract-pin", 0..0));
            }
            if kind == "core" {
                id(&pin["dialect"])?;
            }
        }
        let mut declarations = BTreeMap::new();
        let mut names = BTreeMap::new();
        for row in array(&document["declarations"])? {
            fields(row, &["id", "kind", "scope", "name"])?;
            let declaration = Declaration {
                id: id(&row["id"])?,
                kind: string(&row["kind"])?.into(),
                scope: nullable_id(&row["scope"])?,
                name: name(&row["name"])?,
            };
            if ![
                "scope",
                "data",
                "expression",
                "actor",
                "capability",
                "policy",
                "node",
                "resource",
            ]
            .contains(&declaration.kind.as_str())
            {
                return Err(error("binding-kind", 0..0));
            }
            if !used.insert(declaration.id.clone())
                || names
                    .insert(
                        (declaration.scope.clone(), declaration.name.clone()),
                        declaration.id.clone(),
                    )
                    .is_some()
            {
                return Err(error("binding-duplicate", 0..0));
            }
            declarations.insert(declaration.id.clone(), declaration);
        }
        if !declarations
            .get(&root)
            .is_some_and(|r| r.kind == "scope" && r.scope.is_none())
        {
            return Err(error("root", 0..0));
        }
        for row in declarations.values().filter(|r| r.id != root) {
            let mut parent = row.scope.as_deref();
            let mut seen = BTreeSet::new();
            while let Some(owner) = parent {
                if owner == row.id || !seen.insert(owner) || seen.len() > MAX_DEPTH {
                    return Err(error("scope-cycle", 0..0));
                }
                let ancestor = declarations
                    .get(owner)
                    .filter(|r| r.kind == "scope")
                    .ok_or_else(|| error("scope", 0..0))?;
                if names
                    .get(&(Some(owner.to_owned()), row.name.clone()))
                    .is_some_and(|id| id != &row.id)
                {
                    return Err(error("shadowing", 0..0));
                }
                parent = ancestor.scope.as_deref();
            }
            if !seen.contains(root.as_str()) {
                return Err(error("scope-disconnected", 0..0));
            }
        }
        let mut symbols = BTreeMap::new();
        let mut symbol_names = BTreeSet::new();
        let mut symbol_wires = BTreeSet::new();
        for row in array(&document["symbols"])? {
            fields(row, &["id", "owner", "role", "name", "wire"])?;
            let symbol = Symbol {
                id: id(&row["id"])?,
                owner: id(&row["owner"])?,
                role: string(&row["role"])?.into(),
                name: name(&row["name"])?,
                wire: id(&row["wire"])?,
            };
            let kind = declarations.get(&symbol.owner).map(|r| r.kind.as_str());
            let valid = match symbol.role.as_str() {
                "outcome" => matches!(kind, Some("scope" | "node")),
                "branch" => kind == Some("node"),
                "input" | "output" => kind == Some("scope"),
                "purpose" => kind == Some("data"),
                _ => false,
            };
            if !valid {
                return Err(error("symbol-owner", 0..0));
            }
            if !used.insert(symbol.id.clone())
                || !symbol_names.insert((
                    symbol.owner.clone(),
                    symbol.role.clone(),
                    symbol.name.clone(),
                ))
                || !symbol_wires.insert((
                    symbol.owner.clone(),
                    symbol.role.clone(),
                    symbol.wire.clone(),
                ))
            {
                return Err(error("symbol-duplicate", 0..0));
            }
            symbols.insert(symbol.id.clone(), symbol);
        }
        let mut slots = Vec::new();
        let mut keys = BTreeSet::new();
        for row in array(&document["generated"])? {
            fields(row, &["id", "owner", "role", "key"])?;
            let slot = Slot {
                id: id(&row["id"])?,
                owner: id(&row["owner"])?,
                role: string(&row["role"])?.into(),
                key: nullable_id(&row["key"])?,
            };
            let kind = declarations.get(&slot.owner).map(|r| r.kind.as_str());
            let valid = match slot.role.as_str() {
                "initial" => kind == Some("data") && slot.key.is_none(),
                "provider-policy" => kind == Some("capability") && slot.key.is_none(),
                "assignment" => {
                    kind == Some("node")
                        && slot
                            .key
                            .as_ref()
                            .and_then(|k| declarations.get(k))
                            .is_some_and(|r| r.kind == "data")
                }
                "flow" => {
                    kind == Some("node")
                        && slot
                            .key
                            .as_ref()
                            .and_then(|k| symbols.get(k))
                            .is_some_and(|s| s.owner == slot.owner && s.role == "outcome")
                }
                _ => false,
            };
            if !valid
                || !used.insert(slot.id.clone())
                || !keys.insert((slot.owner.clone(), slot.role.clone(), slot.key.clone()))
            {
                return Err(error("generated-slot", 0..0));
            }
            slots.push(slot);
        }
        let mut retired = BTreeSet::new();
        for row in array(&document["retired"])? {
            let identity = id(row)?;
            if used.contains(&identity) || !retired.insert(identity) {
                return Err(error("retired-identity", 0..0));
            }
        }
        let mut resources = BTreeSet::new();
        for row in array(&document["resources"])? {
            fields(row, &["binding", "kind", "contract", "digest"])?;
            let binding = id(&row["binding"])?;
            if !declarations
                .get(&binding)
                .is_some_and(|r| r.kind == "resource")
                || !resources.insert(binding)
            {
                return Err(error("resource-binding", 0..0));
            }
            if !["provider", "named-type", "clock", "calendar"].contains(&string(&row["kind"])?) {
                return Err(error("resource-kind", 0..0));
            }
            let contract = string(&row["contract"])?;
            let revision = string(&row["digest"])?;
            if contract.is_empty()
                || contract.chars().count() > 4096
                || revision.len() != 71
                || !revision.starts_with("sha256:")
                || !revision[7..]
                    .bytes()
                    .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
            {
                return Err(error("resource-pin", 0..0));
            }
        }
        if resources
            != declarations
                .values()
                .filter(|r| r.kind == "resource")
                .map(|r| r.id.clone())
                .collect()
        {
            return Err(error("resource-inventory", 0..0));
        }
        let symbol_names = symbols
            .values()
            .map(|s| {
                (
                    (s.owner.clone(), s.role.clone(), s.name.clone()),
                    s.id.clone(),
                )
            })
            .collect();
        let slot_keys = slots
            .iter()
            .enumerate()
            .map(|(i, s)| ((s.owner.clone(), s.role.clone(), s.key.clone()), i))
            .collect();
        Ok(Self {
            document,
            declarations,
            symbols,
            slots,
            canonical,
            names,
            symbol_names,
            slot_keys,
        })
    }

    pub fn canonical(&self) -> &str {
        &self.canonical
    }
    pub fn document(&self) -> &Value {
        &self.document
    }
    pub(crate) fn root(&self) -> &str {
        self.document["root"].as_str().expect("checked ledger root")
    }
    pub(crate) fn dialect(&self) -> &str {
        self.document["contracts"]["core"]["dialect"]
            .as_str()
            .expect("checked dialect")
    }

    /// Verify exact bytes for every pin using an explicit host registry, without IO.
    /// A matching digest is not proof that a resource's policies can be enforced.
    pub fn verify_resources(&self, supplied: &[Resource<'_>]) -> Result<()> {
        for pin in self.document["contracts"]
            .as_object()
            .expect("checked contracts")
            .values()
            .chain(
                self.document["resources"]
                    .as_array()
                    .expect("checked resources"),
            )
        {
            let matches: Vec<_> = supplied
                .iter()
                .filter(|r| pin["contract"] == r.id && pin["digest"] == r.revision)
                .collect();
            if matches.len() != 1 {
                return Err(error("resource-missing-or-ambiguous", 0..0));
            }
            if digest(matches[0].bytes) != matches[0].revision {
                return Err(error("resource-digest", 0..0));
            }
        }
        Ok(())
    }

    pub(crate) fn declaration(
        &self,
        scope: Option<&str>,
        name: &str,
        kind: &str,
    ) -> Result<&Declaration> {
        self.names
            .get(&(scope.map(str::to_owned), name.into()))
            .and_then(|id| self.declarations.get(id))
            .filter(|r| r.kind == kind)
            .ok_or_else(|| error("binding-mismatch", 0..0))
    }
    pub(crate) fn resolve<'a>(
        &'a self,
        mut scope: &'a str,
        parts: &[&str],
        kind: &str,
    ) -> Result<&'a Declaration> {
        loop {
            if let Some(row) = self
                .names
                .get(&(Some(scope.to_owned()), parts[0].into()))
                .and_then(|id| self.declarations.get(id))
            {
                if row.kind != kind
                    && !(kind == "outcome-owner" && matches!(row.kind.as_str(), "scope" | "node"))
                {
                    return Err(error("reference-kind", 0..0));
                }
                if parts.len() > 1 {
                    let mut chain = Vec::new();
                    let mut parent = Some(scope);
                    while let Some(owner) = parent {
                        let row = &self.declarations[owner];
                        chain.push(row.name.as_str());
                        parent = row.scope.as_deref();
                    }
                    if parts[1..] != chain {
                        return Err(error("reference-qualification", 0..0));
                    }
                }
                return Ok(row);
            }
            let row = self
                .declarations
                .get(scope)
                .ok_or_else(|| error("scope", 0..0))?;
            let Some(parent) = row.scope.as_deref() else {
                break;
            };
            scope = parent;
        }
        Err(error("reference-missing", 0..0))
    }
    pub(crate) fn symbol(&self, owner: &str, role: &str, name: &str) -> Result<&Symbol> {
        self.symbol_names
            .get(&(owner.into(), role.into(), name.into()))
            .and_then(|id| self.symbols.get(id))
            .ok_or_else(|| error("symbol-mismatch", 0..0))
    }
    pub(crate) fn slot(&self, owner: &str, role: &str, key: Option<&str>) -> Result<&Slot> {
        self.slot_keys
            .get(&(owner.into(), role.into(), key.map(str::to_owned)))
            .and_then(|i| self.slots.get(*i))
            .ok_or_else(|| error("slot-mismatch", 0..0))
    }
    pub(crate) fn pin(&self, identity: &str, kind: &str) -> Result<Value> {
        let row = self.document["resources"]
            .as_array()
            .expect("checked resources")
            .iter()
            .find(|r| r["binding"] == identity && r["kind"] == kind)
            .ok_or_else(|| error("resource-kind", 0..0))?;
        Ok(serde_json::json!({"id": row["contract"], "revision": row["digest"]}))
    }

    fn chain<'a>(&'a self, row: &'a Declaration) -> Vec<&'a str> {
        let mut chain = vec![row.name.as_str()];
        let mut parent = row.scope.as_deref();
        while let Some(owner) = parent {
            let row = &self.declarations[owner];
            chain.push(row.name.as_str());
            parent = row.scope.as_deref();
        }
        chain
    }

    // These two exceptions do not broaden ordinary lexical visibility.
    pub(crate) fn ancestor<'a>(&'a self, scope: &str, parts: &[&str]) -> Result<&'a str> {
        let mut parent = self.declarations[scope].scope.as_deref();
        while let Some(owner) = parent {
            let row = &self.declarations[owner];
            if parts == [row.name.as_str()] || parts == self.chain(row) {
                return Ok(&row.id);
            }
            parent = row.scope.as_deref();
        }
        Err(error("settlement-ancestor", 0..0))
    }

    pub(crate) fn invalidation(&self, scope: &str, parts: &[&str]) -> Result<String> {
        if parts.len() == 1 {
            return self.resolve(scope, parts, "node").map(|r| r.id.clone());
        }
        let row = self
            .declarations
            .values()
            .find(|r| r.kind == "node" && self.chain(r) == parts)
            .ok_or_else(|| error("invalidation-qualification", 0..0))?;
        let mut owner = row.scope.as_deref();
        while let Some(id) = owner {
            if id == scope {
                return Ok(row.id.clone());
            }
            owner = self.declarations[id].scope.as_deref();
        }
        // Qualified references to lexical ancestors remain ordinary references.
        self.resolve(scope, parts, "node").map(|r| r.id.clone())
    }
}
