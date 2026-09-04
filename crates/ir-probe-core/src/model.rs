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
    pub(crate) fn read(body: &Value) -> Result<Self> {
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
        for (key, scope) in object(&body["scopes"])? {
            fields(
                scope,
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
                ],
                &[],
            )?;
            let mut seen = BTreeSet::new();
            let mut current = Some(key.clone());
            while let Some(cursor) = current {
                if !seen.insert(cursor.clone()) {
                    return Err(Error::Scope);
                }
                let parent = &body["scopes"][&cursor]["parent"];
                current = if parent.is_null() {
                    None
                } else {
                    Some(reference(body, "scopes", parent)?)
                };
            }
            if !seen.contains(&root) {
                return Err(Error::Scope);
            }
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
            let kind = read_kind(body, node)?;
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
        Ok(Self { nodes })
    }
}

fn read_kind(body: &Value, node: &Value) -> Result<NodeKind> {
    let name = string(&node["kind"])?;
    let extra: &[&str] = match name {
        "activity" => &["mode", "target", "input", "result", "policy"],
        "compute" => &["assignments"],
        "invoke" => &["body", "inputs", "outputs"],
        "decision" => &["guards", "default"],
        "split" => &["children", "join"],
        "join" => &["source", "predicate", "remaining"],
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
        "wait" => NodeKind::Wait {
            policy: PolicyId(r("policies", "policy")?),
        },
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
