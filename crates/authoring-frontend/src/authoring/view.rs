// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Experimental complete view; no new semantics, identities or authority.
use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Write};
const MAPS: [&str; 8] = [
    "scopes",
    "data",
    "expressions",
    "actors",
    "capabilities",
    "policies",
    "nodes",
    "flows",
];
fn fail(code: &'static str) -> AuthoringDiagnostic {
    diagnostic(code, AuthoringComponent::Document, "/view")
}
struct Count(usize);
impl Write for Count {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        self.0 = self
            .0
            .checked_add(b.len())
            .filter(|n| *n <= DOCUMENT_BYTES)
            .ok_or_else(|| io::Error::other("view-limit"))?;
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn size(v: &Value) -> AuthoringResult<usize> {
    let mut n = Count(0);
    serde_json::to_writer(&mut n, v).map_err(|_| fail("view-limit"))?;
    Ok(n.0)
}
struct Budget {
    items: usize,
    work: usize,
    bytes: usize,
}
impl Budget {
    fn work(&mut self, n: usize) -> AuthoringResult<()> {
        self.work = self
            .work
            .checked_add(n)
            .filter(|n| *n <= 1_000_000)
            .ok_or_else(|| fail("view-limit"))?;
        Ok(())
    }
    fn add(&mut self, v: &Value, depth: usize, comma: bool) -> AuthoringResult<()> {
        let mut pending = vec![(v, depth)];
        while let Some((v, d)) = pending.pop() {
            self.work(1)?;
            self.items += 1;
            if self.items > 262_144 {
                return Err(fail("view-limit"));
            }
            match v {
                Value::Object(m) => {
                    if d > 64 {
                        return Err(fail("view-limit"));
                    }
                    self.items += m.len();
                    self.work(m.len())?;
                    if self.items > 262_144 {
                        return Err(fail("view-limit"));
                    }
                    if self.items + pending.len() + m.len() > 262_144 {
                        return Err(fail("view-limit"));
                    }
                    pending.extend(m.values().map(|v| (v, d + 1)));
                }
                Value::Array(a) => {
                    if d > 64 {
                        return Err(fail("view-limit"));
                    }
                    if self.items + pending.len() + a.len() > 262_144 {
                        return Err(fail("view-limit"));
                    }
                    pending.extend(a.iter().map(|v| (v, d + 1)));
                }
                _ => {}
            }
        }
        self.bytes = self
            .bytes
            .checked_add(size(v)?)
            .and_then(|n| n.checked_add(usize::from(comma)))
            .filter(|n| *n <= DOCUMENT_BYTES)
            .ok_or_else(|| fail("view-limit"))?;
        Ok(())
    }
}
fn map_for(category: &str, role: &str) -> AuthoringResult<Option<&'static str>> {
    Ok(match (category, role) {
        ("declaration", "scope") => Some("scopes"),
        ("declaration", "data") => Some("data"),
        ("declaration", "expression") => Some("expressions"),
        ("declaration", "actor") => Some("actors"),
        ("declaration", "capability") => Some("capabilities"),
        ("declaration", "policy") => Some("policies"),
        ("declaration", "node") => Some("nodes"),
        ("declaration", "resource" | "type-alias" | "data-protection" | "predicate") => None,
        ("symbol", "outcome" | "branch" | "input" | "output" | "purpose" | "predicate-input") => {
            None
        }
        ("generated", "initial" | "assignment") => Some("expressions"),
        ("generated", "flow") => Some("flows"),
        ("generated", "provider-policy") => Some("policies"),
        _ => return Err(fail("view-unmapped")),
    })
}
fn generated_caption(role: &str) -> AuthoringResult<&'static str> {
    match role {
        "initial" => Ok("Initial value"),
        "assignment" => Ok("Assignment"),
        "flow" => Ok("Outcome route"),
        "provider-policy" => Ok("Provider requirements"),
        _ => Err(fail("view-unmapped")),
    }
}
pub(super) fn build(d: &AuthoringDocument) -> AuthoringResult<Value> {
    let ledger =
        crate::Ledger::decode(d.bindings().as_bytes()).map_err(|_| fail("view-binding"))?;
    let ld = ledger.document();
    let ir = d.candidate().document();
    let body = &ir["body"];
    let mut budget = Budget {
        items: 0,
        work: 0,
        bytes: 0,
    };
    // Preflight the existing bounded IR before cloning it into the new view.
    let mut result = json!({"format":"choreoform-authoring-view","version":"0.1.0","experimental":true,"status":"projection-matched","executable":false,"binding":{"definition":body["id"],"root":body["root"],"authoring":d.authoring_digest(),"source":d.candidate().source_digest(),"bindings":d.candidate().bindings_digest(),"semanticRevision":ir["revision"]},"source":d.source(),"entities":[],"flows":[],"ir":null});
    budget.add(&result, 1, false)?;
    budget.items -= 1;
    budget.bytes -= 4;
    budget.add(ir, 2, false)?;
    result["ir"] = ir.clone();
    let mut semantic = BTreeMap::new();
    for map in MAPS {
        let records = body[map].as_object().ok_or_else(|| fail("view-unmapped"))?;
        for id in records.keys() {
            budget.work(1)?;
            if semantic.insert(id.as_str(), map).is_some() {
                return Err(fail("view-coverage"));
            }
        }
    }
    let mut rows = BTreeMap::new();
    for (group, category) in [
        ("declarations", "declaration"),
        ("generated", "generated"),
        ("symbols", "symbol"),
    ] {
        for (index, row) in ld[group]
            .as_array()
            .ok_or_else(|| fail("view-coverage"))?
            .iter()
            .enumerate()
        {
            budget.work(1)?;
            let id = row["id"].as_str().ok_or_else(|| fail("view-coverage"))?;
            if rows.insert(id, (category, group, index, row)).is_some() {
                return Err(fail("view-coverage"));
            }
        }
    }
    if rows.len() > 30000 || semantic.keys().any(|id| !rows.contains_key(id)) {
        return Err(fail("view-coverage"));
    }
    let pins: BTreeMap<_, _> = ld["resources"]
        .as_array()
        .ok_or_else(|| fail("view-coverage"))?
        .iter()
        .map(|r| (r["binding"].as_str().expect("admitted pin"), r))
        .collect();
    let placements: BTreeMap<_, _> = d.document()["layout"]["entries"]
        .as_array()
        .expect("admitted layout")
        .iter()
        .map(|r| (r["entity"].as_str().expect("admitted placement"), r))
        .collect();
    let mut positions: BTreeMap<Option<&str>, usize> = BTreeMap::new();
    let mut entities = Vec::new();
    for (id, (category, group, index, row)) in &rows {
        budget.work(1)?;
        let role = row[if *category == "declaration" {
            "kind"
        } else {
            "role"
        }]
        .as_str()
        .ok_or_else(|| fail("view-unmapped"))?;
        let expected = map_for(category, role)?;
        if semantic.get(id).copied() != expected {
            return Err(fail("view-coverage"));
        }
        let owner = row[if *category == "declaration" {
            "scope"
        } else {
            "owner"
        }]
        .as_str();
        if owner.is_some_and(|o| !rows.contains_key(o)) {
            return Err(fail("view-coverage"));
        }
        let caption = if *category == "generated" {
            generated_caption(role)?
        } else {
            row["name"].as_str().ok_or_else(|| fail("view-coverage"))?
        };
        let basis = if role == "resource" {
            "companion"
        } else if *category == "symbol" || role == "provider-policy" {
            "owner"
        } else if *category == "generated" {
            "slot"
        } else {
            "declaration"
        };
        let span = if basis == "companion" {
            None
        } else {
            let location_id = if basis == "owner" {
                owner.ok_or_else(|| fail("view-location"))?
            } else {
                id
            };
            budget.work(1)?;
            let s = d
                .candidate()
                .spans()
                .get(location_id)
                .ok_or_else(|| fail("view-location"))?;
            if s.start > s.end
                || s.end > d.source().len()
                || !d.source().is_char_boundary(s.start)
                || !d.source().is_char_boundary(s.end)
            {
                return Err(fail("view-location"));
            }
            Some(s)
        };
        let n = positions.entry(owner).or_default();
        let placement = if let Some(p) = placements.get(id) {
            json!({"x":p["x"],"y":p["y"],"collapsed":p["collapsed"],"explicit":true})
        } else {
            json!({"x":(*n%8)*240,"y":(*n/8)*120,"collapsed":false,"explicit":false})
        };
        *n += 1;
        let node_kind = if expected == Some("nodes") {
            let node = &body["nodes"][*id];
            let k = node["kind"].as_str().ok_or_else(|| fail("view-unmapped"))?;
            if ![
                "activity", "compute", "invoke", "decision", "split", "join", "wait", "repeat",
                "fanout", "finish",
            ]
            .contains(&k)
                || (k == "activity"
                    && !matches!(node["mode"].as_str(), Some("human" | "capability")))
            {
                return Err(fail("view-unmapped"));
            }
            Some(k)
        } else {
            None
        };
        let pin = if role == "resource" {
            let p = pins.get(id).ok_or_else(|| fail("view-coverage"))?;
            json!({"kind":p["kind"],"contract":p["contract"],"digest":p["digest"]})
        } else {
            Value::Null
        };
        let entity = json!({"id":id,"category":category,"role":role,"owner":owner,"caption":caption,"semanticMap":expected,"nodeKind":node_kind,"wire":row.get("wire").unwrap_or(&Value::Null),"location":{"basis":basis,"start":span.map(|s|s.start),"end":span.map(|s|s.end),"companion":format!("/{group}/{index}")},"placement":placement,"renameAllowed":matches!(role,"predicate"|"type-alias"|"data-protection"|"predicate-input"),"pin":pin});
        budget.add(&entity, 3, !entities.is_empty())?;
        entities.push(entity);
    }
    let mut flows = Vec::new();
    let slots: BTreeMap<_, _> = ledger.slots.iter().map(|s| (s.id.as_str(), s)).collect();
    for (id, f) in body["flows"]
        .as_object()
        .ok_or_else(|| fail("view-unmapped"))?
    {
        budget.work(1)?;
        if flows.len() >= 10000 {
            return Err(fail("view-limit"));
        }
        let slot = slots
            .get(id.as_str())
            .filter(|s| s.role == "flow")
            .ok_or_else(|| fail("view-coverage"))?;
        let symbol = slot
            .key
            .as_ref()
            .and_then(|k| ledger.symbols.get(k))
            .ok_or_else(|| fail("view-coverage"))?;
        let source = f["source"].as_str().ok_or_else(|| fail("view-coverage"))?;
        let target = f["target"].as_str().ok_or_else(|| fail("view-coverage"))?;
        if slot.owner != source
            || symbol.owner != source
            || symbol.role != "outcome"
            || f["outcome"] != symbol.wire
            || !body["nodes"]
                .as_object()
                .expect("mapped nodes")
                .contains_key(target)
        {
            return Err(fail("view-coverage"));
        }
        let flow = json!({"id":id,"source":source,"outcome":f["outcome"],"target":target,"caption":symbol.name});
        budget.add(&flow, 3, !flows.is_empty())?;
        flows.push(flow);
    }
    result["entities"] = Value::Array(entities);
    result["flows"] = Value::Array(flows);
    // Coverage checks use explicit IDs; graph order and coordinates confer nothing.
    let represented: BTreeSet<_> = result["entities"]
        .as_array()
        .expect("built entities")
        .iter()
        .map(|e| e["id"].as_str().expect("built ID"))
        .collect();
    if represented.len() != rows.len() {
        return Err(fail("view-coverage"));
    }
    Ok(result)
}
