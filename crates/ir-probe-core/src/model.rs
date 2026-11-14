// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

//! A partial typed projection, never a replacement serialization or execution model.
//!
//! Distinct identifiers cannot be silently interchanged:
//! ```compile_fail
//! use choreoform_ir_probe_core::model::{NodeId, DataId};
//! fn node(_: NodeId) {}
//! fn wrong(id: DataId) { node(id); }
//! ```
//! ```compile_fail
//! use choreoform_ir_probe_core::model::{NodeId, OccurrenceId};
//! fn node(_: NodeId) {}
//! fn wrong(id: OccurrenceId) { node(id); }
//! ```
//! Closed variants require exhaustive handling (no fallback extension node):
//! ```compile_fail
//! use choreoform_ir_probe_core::model::NodeKind;
//! fn incomplete(node: NodeKind) {
//!     match node { NodeKind::Finish { .. } => () }
//! }
//! ```

use crate::{Error, Result, fields, object, string};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

macro_rules! ids {
    ($($name:ident),+) => {$ (
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(String);
        impl $name { pub fn as_str(&self) -> &str { &self.0 } }
    )+};
}
ids!(
    NodeId,
    DataId,
    ScopeId,
    ExpressionId,
    ActorId,
    CapabilityId,
    PolicyId,
    OccurrenceId
);

/// A structural external reference, not verified resource bytes or host support.
#[derive(Debug)]
pub struct ArtifactRef {
    id: String,
    revision: String,
}
impl ArtifactRef {
    fn read(value: &Value) -> Result<Self> {
        fields(value, &["id", "revision"], &[])?;
        let artifact_id = string(&value["id"])?;
        let revision = string(&value["revision"])?;
        if artifact_id.is_empty()
            || revision.len() != 71
            || !revision.starts_with("sha256:")
            || !revision[7..]
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(Error::Shape);
        }
        Ok(Self {
            id: artifact_id.into(),
            revision: revision.into(),
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn revision(&self) -> &str {
        &self.revision
    }
}
/// Structural handler-local history reference; no initialized value or authority.
#[derive(Debug)]
pub struct FaultInput {
    pub history: ArtifactRef,
    pub result: DataId,
}
#[derive(Debug)]
pub struct WaitCapture {
    pub source: ArtifactRef,
    pub origin: NodeId,
    pub result: DataId,
}

#[derive(Debug)]
pub enum ActivityTarget {
    Human(ActorId),
    Capability(CapabilityId),
}

#[derive(Debug)]
pub enum NodeKind {
    Activity {
        target: ActivityTarget,
        input: ExpressionId,
        result: Option<DataId>,
        policy: PolicyId,
    },
    Compute {
        assignments: BTreeMap<DataId, ExpressionId>,
    },
    Invoke {
        body: ScopeId,
        inputs: BTreeMap<String, ExpressionId>,
        outputs: BTreeMap<String, DataId>,
    },
    Decision {
        guards: BTreeMap<String, ExpressionId>,
        default: Option<String>,
    },
    Split {
        children: BTreeMap<String, ScopeId>,
        join: NodeId,
    },
    Join {
        source: NodeId,
        remaining: PolicyId,
    },
    Wait {
        policy: PolicyId,
        capture: Option<WaitCapture>,
    },
    Repeat {
        body: ScopeId,
        condition: ExpressionId,
    },
    Fanout {
        collection: ExpressionId,
        item_key: ExpressionId,
        body: ScopeId,
        item: DataId,
        join: NodeId,
        seal: ExpressionId,
        changes: PolicyId,
    },
    Finish {
        outcome: String,
    },
}

impl NodeKind {
    /// Intentionally no wildcard: adding a variant makes this match fail to compile.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Activity { .. } => "activity",
            Self::Compute { .. } => "compute",
            Self::Invoke { .. } => "invoke",
            Self::Decision { .. } => "decision",
            Self::Split { .. } => "split",
            Self::Join { .. } => "join",
            Self::Wait { .. } => "wait",
            Self::Repeat { .. } => "repeat",
            Self::Fanout { .. } => "fanout",
            Self::Finish { .. } => "finish",
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub scope: ScopeId,
    pub reads: BTreeSet<DataId>,
    pub writes: BTreeSet<DataId>,
    pub kind: NodeKind,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: BTreeMap<NodeId, Node>,
    pub fault_inputs: BTreeMap<ScopeId, FaultInput>,
    pub retained_requests: BTreeMap<ScopeId, BTreeMap<DataId, NodeId>>,
}

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

fn id(text: &str) -> Result<String> {
    if text.is_empty()
        || text.len() > 64
        || !text.as_bytes()[0].is_ascii_alphabetic()
        || !text
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        return Err(Error::Shape);
    }
    Ok(text.to_owned())
}

fn symbols(value: &Value) -> Result<Vec<String>> {
    object(value)?
        .iter()
        .map(|(key, value)| {
            if value != true {
                return Err(Error::Shape);
            }
            id(key)
        })
        .collect()
}

fn reference(body: &Value, map: &str, value: &Value) -> Result<String> {
    let key = id(string(value)?)?;
    if !object(&body[map])?.contains_key(&key) {
        return Err(Error::Reference);
    }
    Ok(key)
}

fn refs(body: &Value, map: &str, value: &Value) -> Result<Vec<String>> {
    symbols(value)?
        .iter()
        .map(|key| reference(body, map, &Value::String(key.clone())))
        .collect()
}

fn bindings<T>(
    value: &Value,
    mut read: impl FnMut(&Value) -> Result<T>,
) -> Result<BTreeMap<String, T>> {
    object(value)?
        .iter()
        .map(|(key, value)| Ok((id(key)?, read(value)?)))
        .collect()
}

fn dialect(body: &Value, value: &Value) -> Result<()> {
    fields(value, &["dialect", "body"], &[])?;
    reference(body, "dialects", &value["dialect"])?;
    object(&value["body"])?;
    Ok(())
}

fn predicate(value: &Value) -> Result<()> {
    match string(&value["op"])? {
        "all" | "any" => {
            fields(value, &["op", "outcomes"], &[])?;
            if symbols(&value["outcomes"])?.is_empty() {
                return Err(Error::Shape);
            }
        }
        "atLeast" => {
            fields(value, &["op", "count", "outcomes"], &[])?;
            if value["count"].as_u64().unwrap_or(0) == 0 || symbols(&value["outcomes"])?.is_empty()
            {
                return Err(Error::Shape);
            }
        }
        "and" | "or" => {
            fields(value, &["op", "terms"], &[])?;
            if object(&value["terms"])?.is_empty() {
                return Err(Error::Shape);
            }
            bindings(&value["terms"], predicate)?;
        }
        _ => return Err(Error::Shape),
    }
    Ok(())
}

impl Graph {
    pub(crate) fn read(body: &Value, version: &str) -> Result<Self> {
        if !matches!(version, "0.1.0" | "0.2.0" | "0.3.0" | "0.4.0") {
            return Err(Error::Version);
        }
        let v2 = version != "0.1.0";
        let v3 = matches!(version, "0.3.0" | "0.4.0");
        let v4 = version == "0.4.0";
        let mut retained_requests = BTreeMap::new();
        let mut fault_inputs = BTreeMap::new();
        let mut all_ids = BTreeSet::new();
        for map in MAPS {
            for key in object(&body[map])?.keys() {
                if !all_ids.insert(id(key)?) {
                    return Err(Error::Reference);
                }
            }
        }
        for key in object(&body["dialects"])?.keys() {
            id(key)?;
        }
        let root = reference(body, "scopes", &body["root"])?;
        if !body["scopes"][&root]["parent"].is_null() {
            return Err(Error::Scope);
        }
        let mut rooted = BTreeSet::from([root.clone()]);
        for (key, scope) in object(&body["scopes"])? {
            let required: &[&str] = if v3 {
                &[
                    "parent",
                    "entry",
                    "inputs",
                    "outputs",
                    "outcomes",
                    "cancellation",
                    "faults",
                    "closure",
                    "race",
                    "faultInput",
                ]
            } else {
                &[
                    "parent",
                    "entry",
                    "inputs",
                    "outputs",
                    "outcomes",
                    "cancellation",
                    "faults",
                    "closure",
                    "race",
                ]
            };
            if v4 {
                let mut extended = required.to_vec();
                extended.push("retainedRequests");
                fields(scope, &extended, &[])?;
            } else {
                fields(scope, required, &[])?;
            }
            if v4 {
                let mut requests = BTreeMap::new();
                for (target, origin) in object(&scope["retainedRequests"])? {
                    let result = reference(body, "data", &Value::from(target.as_str()))?;
                    if body["data"][&result]["scope"] != *key {
                        return Err(Error::Scope);
                    }
                    let source = reference(body, "nodes", origin)?;
                    requests.insert(DataId(result), NodeId(source));
                }
                if !requests.is_empty() {
                    retained_requests.insert(ScopeId(key.clone()), requests);
                }
            }
            if v3 && !scope["faultInput"].is_null() {
                let binding = &scope["faultInput"];
                fields(binding, &["history", "result"], &[])?;
                let history = ArtifactRef::read(&binding["history"])?;
                let result = reference(body, "data", &binding["result"])?;
                if body["data"][&result]["scope"] != *key {
                    return Err(Error::Scope);
                }
                fault_inputs.insert(
                    ScopeId(key.clone()),
                    FaultInput {
                        history,
                        result: DataId(result),
                    },
                );
            }
            trace_scope(body, key, &mut rooted)?;
            let entry = reference(body, "nodes", &scope["entry"])?;
            reference(body, "scopes", &body["nodes"][&entry]["scope"])?;
            if body["nodes"][entry]["scope"] != *key {
                return Err(Error::Scope);
            }
            symbols(&scope["outcomes"])?;
            for field in ["inputs", "outputs"] {
                for cell in bindings(&scope[field], |v| reference(body, "data", v))?.values() {
                    if body["data"][cell]["scope"] != *key {
                        return Err(Error::Scope);
                    }
                }
            }
            for field in ["cancellation", "faults", "closure", "race"] {
                reference(body, "policies", &scope[field])?;
            }
        }
        // Closed declaration record shapes. Dialect payload objects remain opaque.
        for (map, required, optional) in [
            (
                "data",
                vec!["scope", "type", "protection", "invalidates"],
                vec!["initial"],
            ),
            (
                "expressions",
                vec![
                    "scope",
                    "dialect",
                    "resultType",
                    "parameters",
                    "reads",
                    "body",
                ],
                vec![],
            ),
            ("actors", vec!["scope", "requirement"], vec![]),
            (
                "capabilities",
                vec![
                    "scope",
                    "contract",
                    "input",
                    "output",
                    "authority",
                    "effects",
                ],
                vec![],
            ),
            ("policies", vec!["scope", "dialect", "body"], vec![]),
        ] {
            for record in object(&body[map])?.values() {
                fields(record, &required, &optional)?;
                reference(body, "scopes", &record["scope"])?;
            }
        }
        for record in object(&body["data"])?.values() {
            dialect(body, &record["type"])?;
            refs(body, "nodes", &record["invalidates"])?;
            if let Some(initial) = record.get("initial") {
                reference(body, "expressions", initial)?;
            }
            let p = &record["protection"];
            fields(
                p,
                &[
                    "sensitivity",
                    "purposes",
                    "participants",
                    "capabilities",
                    "policy",
                ],
                &[],
            )?;
            if string(&p["sensitivity"])?.is_empty() || symbols(&p["purposes"])?.is_empty() {
                return Err(Error::Shape);
            }
            refs(body, "actors", &p["participants"])?;
            refs(body, "capabilities", &p["capabilities"])?;
            reference(body, "policies", &p["policy"])?;
        }
        for record in object(&body["expressions"])?.values() {
            reference(body, "dialects", &record["dialect"])?;
            object(&record["body"])?;
            dialect(body, &record["resultType"])?;
            bindings(&record["parameters"], |v| dialect(body, v))?;
            refs(body, "data", &record["reads"])?;
        }
        for record in object(&body["policies"])?.values() {
            reference(body, "dialects", &record["dialect"])?;
            object(&record["body"])?;
        }
        for record in object(&body["actors"])?.values() {
            reference(body, "policies", &record["requirement"])?;
        }
        for record in object(&body["capabilities"])?.values() {
            for field in ["contract", "effects"] {
                reference(body, "policies", &record[field])?;
            }
            reference(body, "actors", &record["authority"])?;
            for field in ["input", "output"] {
                dialect(body, &record[field])?;
            }
        }
        let mut nodes = BTreeMap::new();
        for (key, node) in object(&body["nodes"])? {
            let scope = ScopeId(reference(body, "scopes", &node["scope"])?);
            symbols(&node["outcomes"])?;
            let reads = refs(body, "data", &node["reads"])?
                .into_iter()
                .map(DataId)
                .collect();
            let writes = refs(body, "data", &node["writes"])?
                .into_iter()
                .map(DataId)
                .collect();
            let kind = read_kind(body, node, v2)?;
            nodes.insert(
                NodeId(key.clone()),
                Node {
                    scope,
                    reads,
                    writes,
                    kind,
                },
            );
        }
        for flow in object(&body["flows"])?.values() {
            fields(flow, &["source", "outcome", "target"], &[])?;
            let source = reference(body, "nodes", &flow["source"])?;
            let target = reference(body, "nodes", &flow["target"])?;
            let outcome = id(string(&flow["outcome"])?)?;
            if body["nodes"][&source]["scope"] != body["nodes"][&target]["scope"] {
                return Err(Error::Scope);
            }
            if body["nodes"][source]["outcomes"].get(outcome).is_none() {
                return Err(Error::Reference);
            }
        }
        Ok(Self {
            nodes,
            fault_inputs,
            retained_requests,
        })
    }
}

// Memoize only paths proven to reach the sole root. Every non-root parent is
// resolved once across a successful graph check, independent of traversal order.
// The returned edge count lets tests check scaling without wall-clock thresholds.
fn trace_scope(body: &Value, start: &str, rooted: &mut BTreeSet<String>) -> Result<usize> {
    let mut path = BTreeSet::new();
    let mut cursor = start.to_owned();
    while !rooted.contains(&cursor) {
        if !path.insert(cursor.clone()) {
            return Err(Error::Scope);
        }
        let parent = &body["scopes"][&cursor]["parent"];
        if parent.is_null() {
            return Err(Error::Scope);
        }
        cursor = reference(body, "scopes", parent)?;
    }
    let edges = path.len();
    rooted.extend(path);
    Ok(edges)
}

fn read_kind(body: &Value, node: &Value, v2: bool) -> Result<NodeKind> {
    let name = string(&node["kind"])?;
    let extra: &[&str] = match name {
        "activity" => &["mode", "target", "input", "result", "policy"],
        "compute" => &["assignments"],
        "invoke" => &["body", "inputs", "outputs"],
        "decision" => &["guards", "default"],
        "split" => &["children", "join"],
        "join" => &["source", "predicate", "remaining"],
        "wait" if v2 => &["policy", "capture"],
        "wait" => &["policy"],
        "repeat" => &["body", "condition"],
        "fanout" => &[
            "collection",
            "itemKey",
            "body",
            "item",
            "join",
            "seal",
            "changes",
        ],
        "finish" => &["outcome"],
        _ => return Err(Error::UnknownNodeKind),
    };
    let required: Vec<_> = ["kind", "scope", "outcomes", "reads", "writes"]
        .iter()
        .chain(extra)
        .copied()
        .collect();
    fields(node, &required, &[])?;
    let r = |map, field: &str| reference(body, map, &node[field]);
    Ok(match name {
        "activity" => NodeKind::Activity {
            target: match string(&node["mode"])? {
                "human" => ActivityTarget::Human(ActorId(r("actors", "target")?)),
                "capability" => {
                    ActivityTarget::Capability(CapabilityId(r("capabilities", "target")?))
                }
                _ => return Err(Error::Shape),
            },
            input: ExpressionId(r("expressions", "input")?),
            result: if node["result"].is_null() {
                None
            } else {
                Some(DataId(r("data", "result")?))
            },
            policy: PolicyId(r("policies", "policy")?),
        },
        "compute" => {
            if object(&node["assignments"])?.is_empty() {
                return Err(Error::Shape);
            }
            let assignments = object(&node["assignments"])?
                .iter()
                .map(|(key, value)| {
                    Ok((
                        DataId(reference(body, "data", &Value::String(key.clone()))?),
                        ExpressionId(reference(body, "expressions", value)?),
                    ))
                })
                .collect::<Result<_>>()?;
            NodeKind::Compute { assignments }
        }
        "invoke" => NodeKind::Invoke {
            body: ScopeId(r("scopes", "body")?),
            inputs: bindings(&node["inputs"], |v| {
                Ok(ExpressionId(reference(body, "expressions", v)?))
            })?,
            outputs: bindings(&node["outputs"], |v| {
                Ok(DataId(reference(body, "data", v)?))
            })?,
        },
        "decision" => NodeKind::Decision {
            guards: bindings(&node["guards"], |v| {
                Ok(ExpressionId(reference(body, "expressions", v)?))
            })?,
            default: if node["default"].is_null() {
                None
            } else {
                Some(id(string(&node["default"])?)?)
            },
        },
        "split" => {
            if object(&node["children"])?.is_empty() {
                return Err(Error::Shape);
            }
            NodeKind::Split {
                children: bindings(&node["children"], |v| {
                    Ok(ScopeId(reference(body, "scopes", v)?))
                })?,
                join: NodeId(r("nodes", "join")?),
            }
        }
        "join" => {
            predicate(&node["predicate"])?;
            NodeKind::Join {
                source: NodeId(r("nodes", "source")?),
                remaining: PolicyId(r("policies", "remaining")?),
            }
        }
        "wait" => {
            let capture = if v2 && !node["capture"].is_null() {
                let c = &node["capture"];
                fields(c, &["source", "origin", "result"], &[])?;
                let source = ArtifactRef::read(&c["source"])?;
                Some(WaitCapture {
                    source,
                    origin: NodeId(reference(body, "nodes", &c["origin"])?),
                    result: DataId(reference(body, "data", &c["result"])?),
                })
            } else {
                None
            };
            NodeKind::Wait {
                policy: PolicyId(r("policies", "policy")?),
                capture,
            }
        }
        "repeat" => NodeKind::Repeat {
            body: ScopeId(r("scopes", "body")?),
            condition: ExpressionId(r("expressions", "condition")?),
        },
        "fanout" => NodeKind::Fanout {
            collection: ExpressionId(r("expressions", "collection")?),
            item_key: ExpressionId(r("expressions", "itemKey")?),
            body: ScopeId(r("scopes", "body")?),
            item: DataId(r("data", "item")?),
            join: NodeId(r("nodes", "join")?),
            seal: ExpressionId(r("expressions", "seal")?),
            changes: PolicyId(r("policies", "changes")?),
        },
        "finish" => NodeKind::Finish {
            outcome: id(string(&node["outcome"])?)?,
        },
        _ => return Err(Error::UnknownNodeKind),
    })
}

#[cfg(test)]
mod scope_tests {
    use super::*;
    use serde_json::json;

    fn chain(n: usize) -> Value {
        let mut body = json!({"scopes": {"s0": {"parent": null}}});
        for i in 1..n {
            body["scopes"][format!("s{i}")] = json!({"parent":format!("s{}", i-1)});
        }
        body
    }

    #[test]
    fn parent_edges_are_visited_once_even_deepest_first() {
        for n in [128, 512, 2048] {
            let body = chain(n);
            let mut rooted = BTreeSet::from(["s0".to_owned()]);
            let mut edges = 0;
            for i in (0..n).rev() {
                edges += trace_scope(&body, &format!("s{i}"), &mut rooted).unwrap();
            }
            assert_eq!(edges, n - 1);
            assert_eq!(rooted.len(), n);
        }
    }

    #[test]
    fn invalid_deep_paths_are_not_cached() {
        for (parent, expected) in [
            (json!("s2047"), Error::Scope),
            (Value::Null, Error::Scope),
            (json!("missing"), Error::Reference),
        ] {
            let mut body = chain(2048);
            body["scopes"]["s1"]["parent"] = parent;
            let mut rooted = BTreeSet::from(["s0".to_owned()]);
            assert_eq!(trace_scope(&body, "s2047", &mut rooted), Err(expected));
            assert_eq!(rooted.len(), 1);
        }
    }
}
