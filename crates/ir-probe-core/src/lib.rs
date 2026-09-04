// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

//! Bounded ADR-0010 evidence, NOT a production structural/semantic validator.
//! No filesystem, network, clock, randomness, threads or execution in this API.

pub mod model;
pub mod transport;

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

pub type Result<T> = std::result::Result<T, Error>;

/// Probe-local categories, not the future language diagnostics contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Size,
    Depth,
    Utf8,
    Json,
    DuplicateKey,
    NumberToken,
    IntegerRange,
    Envelope,
    Version,
    Shape,
    UnknownNodeKind,
    Reference,
    Scope,
    ContractUnsupported,
    ContractMissing,
    ContractDigest,
    Revision,
}

impl Error {
    pub fn category(self) -> &'static str {
        match self {
            Self::Size => "size",
            Self::Depth => "depth",
            Self::Utf8 => "utf8",
            Self::Json => "json",
            Self::DuplicateKey => "duplicate-key",
            Self::NumberToken => "number-token",
            Self::IntegerRange => "integer-range",
            Self::Envelope => "envelope",
            Self::Version => "version",
            Self::Shape => "shape",
            Self::UnknownNodeKind => "unknown-node-kind",
            Self::Reference => "reference",
            Self::Scope => "scope",
            Self::ContractUnsupported => "contract-unsupported",
            Self::ContractMissing => "contract-missing",
            Self::ContractDigest => "contract-digest",
            Self::Revision => "revision",
        }
    }
}

/// Host supplies artifact bytes, never a URL resolver or implicit fetch.
pub struct Resource<'a> {
    pub id: &'a str,
    pub revision: &'a str,
    pub bytes: &'a [u8],
}

pub const SUPPORTED_CONTRACTS: [(&str, &str); 2] = [
    (
        "urn:choreoform:semantics:adr-0008",
        "sha256:0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141",
    ),
    (
        "urn:choreoform:examples:illustrative-dialect",
        "sha256:282dd3c02983141da13e8f700e852d5472ffc4af176a53ca3578c418482a09b9",
    ),
];

/// Lossless decoded wire data plus a deliberately partial typed graph view.
/// Passing inspection grants no authority to plan, execute or enforce policy.
#[derive(Debug)]
pub struct Inspected {
    pub document: Value,
    pub graph: model::Graph,
    pub canonical: String,
    pub revision: String,
}

pub fn digest(bytes: &[u8]) -> String {
    let hex: String = Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    format!("sha256:{hex}")
}

pub(crate) fn object(value: &Value) -> Result<&Map<String, Value>> {
    value.as_object().ok_or(Error::Shape)
}

pub(crate) fn string(value: &Value) -> Result<&str> {
    value.as_str().ok_or(Error::Shape)
}

pub(crate) fn fields(value: &Value, required: &[&str], optional: &[&str]) -> Result<()> {
    let map = object(value)?;
    if required.iter().any(|key| !map.contains_key(*key))
        || map
            .keys()
            .any(|key| !required.contains(&key.as_str()) && !optional.contains(&key.as_str()))
    {
        return Err(Error::Shape);
    }
    Ok(())
}

fn envelope(document: &Value) -> Result<()> {
    fields(
        document,
        &[
            "format",
            "version",
            "kind",
            "revision",
            "body",
            "annotations",
        ],
        &[],
    )
    .map_err(|_| Error::Envelope)?;
    if document["format"] != "choreoform-ir" || document["kind"] != "definition" {
        return Err(Error::Envelope);
    }
    if document["version"] != "0.1.0" {
        return Err(Error::Version);
    }
    object(&document["annotations"]).map_err(|_| Error::Envelope)?;
    let revision = string(&document["revision"]).map_err(|_| Error::Envelope)?;
    if revision.len() != 71
        || !revision.starts_with("sha256:")
        || !revision[7..]
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(Error::Envelope);
    }
    Ok(())
}

fn projection(document: &Value) -> Value {
    Value::Object(
        ["format", "version", "kind", "body"]
            .into_iter()
            .map(|key| (key.to_owned(), document[key].clone()))
            .collect(),
    )
}

/// Transport and envelope only; useful to prepare mutated evidence fixtures.
/// This intentionally does NOT certify even the selected graph/contract checks.
pub fn semantic_bytes(raw: &[u8]) -> Result<String> {
    let document = transport::decode(raw)?;
    envelope(&document)?;
    Ok(transport::canonical(&projection(&document)))
}

pub fn inspect(raw: &[u8], resources: &[Resource<'_>]) -> Result<Inspected> {
    let document = transport::decode(raw)?;
    envelope(&document)?;
    let body = &document["body"];
    fields(
        body,
        &[
            "id",
            "semantics",
            "dialects",
            "root",
            "scopes",
            "data",
            "expressions",
            "actors",
            "capabilities",
            "policies",
            "nodes",
            "flows",
        ],
        &[],
    )?;
    if string(&body["id"])?.is_empty() {
        return Err(Error::Shape);
    }
    for binding in std::iter::once(&body["semantics"]).chain(object(&body["dialects"])?.values()) {
        fields(binding, &["id", "revision"], &[])?;
        let pair = (string(&binding["id"])?, string(&binding["revision"])?);
        if !SUPPORTED_CONTRACTS.contains(&pair) {
            return Err(Error::ContractUnsupported);
        }
        let matches: Vec<_> = resources
            .iter()
            .filter(|r| (r.id, r.revision) == pair)
            .collect();
        if matches.is_empty() {
            return Err(Error::ContractMissing);
        }
        // Ambiguous or corrupt registry entries are never selected by order.
        if matches.len() != 1 || digest(matches[0].bytes) != pair.1 {
            return Err(Error::ContractDigest);
        }
    }
    // Semantic and dialect roles are not interchangeable, even when supported.
    if body["semantics"]["id"] != SUPPORTED_CONTRACTS[0].0
        || object(&body["dialects"])?
            .values()
            .any(|v| v["id"] != SUPPORTED_CONTRACTS[1].0)
    {
        return Err(Error::ContractUnsupported);
    }
    let graph = model::Graph::read(body)?;
    let canonical = transport::canonical(&projection(&document));
    let revision = digest(canonical.as_bytes());
    if document["revision"] != revision {
        return Err(Error::Revision);
    }
    Ok(Inspected {
        document,
        graph,
        canonical,
        revision,
    })
}
