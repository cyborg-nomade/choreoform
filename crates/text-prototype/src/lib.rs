// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

//! Disposable ADR-0011 syntax prototype. Lowering produces UNVALIDATED IR.
//! No IO, registry lookup, policy interpretation, or execution in this library.

use choreoform_ir_probe_core::{digest, semantic_bytes, transport};
use serde_json::{Map, Value, json};
use std::{collections::BTreeSet, ops::Range};

pub const MAX_BYTES: usize = 1024 * 1024;
const SECTIONS: [&str; 8] = [
    "scopes",
    "data",
    "expressions",
    "actors",
    "capabilities",
    "policies",
    "nodes",
    "flows",
];
const VALUES: [&str; 5] = ["id", "semantics", "dialects", "root", "annotations"];

/// Prototype-local diagnostic; spans are half-open UTF-8 byte offsets.
/// JSON payload errors cover the whole payload; codes are not a stable API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub message: String,
    pub span: Range<usize>,
}

type Result<T> = std::result::Result<T, Diagnostic>;

fn error(message: impl Into<String>, span: Range<usize>) -> Diagnostic {
    Diagnostic {
        message: message.into(),
        span,
    }
}

/// Stable declaration identity and its original-source record extent.
#[derive(Debug)]
pub struct RecordSpan {
    pub id: String,
    pub span: Range<usize>,
}

/// Source-level item, distinct from an IR declaration or runtime occurrence.
#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub span: Range<usize>,
    pub records: Vec<RecordSpan>,
    value: Value,
}

/// Retains the exact borrowed source (including trivia) and parsed item spans.
/// It is immutable through this API. No incremental editing/recovery is implied.
#[derive(Debug)]
pub struct Syntax<'a> {
    source: &'a str,
    items: Vec<Item>,
}

/// Persist these two identities with any extracted spans. Never reuse a map
/// after either the original source bytes or semantic revision changes.
#[derive(Debug, PartialEq, Eq)]
pub struct SourceBinding {
    pub source_digest: String,
    pub semantic_revision: String,
}

impl Syntax<'_> {
    /// Bind source spans to exact source bytes and the candidate's semantic
    /// revision. Returns an error if the complete lowered envelope exceeds limits.
    pub fn binding(&self) -> Result<SourceBinding> {
        let document = self.lower()?;
        Ok(SourceBinding {
            source_digest: digest(self.source.as_bytes()),
            semantic_revision: document["revision"]
                .as_str()
                .expect("lower computes revision")
                .to_owned(),
        })
    }
    /// Return the original source verbatim, including comments and whitespace.
    pub fn source(&self) -> &str {
        self.source
    }
    /// Return immutable items in original source order, with byte spans.
    pub fn items(&self) -> &[Item] {
        &self.items
    }

    /// Produce a revision-bearing candidate, NOT a validation/admission result.
    /// Source comments/spans stay in Syntax; supplied annotations survive exactly.
    pub fn lower(&self) -> Result<Value> {
        let mut body = Map::new();
        let mut annotations = Value::Null;
        for item in &self.items {
            if item.name == "annotations" {
                annotations = item.value.clone();
            } else {
                body.insert(item.name.clone(), item.value.clone());
            }
        }
        let mut document = json!({
            "format": "choreoform-ir", "version": "0.1.0", "kind": "definition",
            "revision": format!("sha256:{}", "0".repeat(64)),
            "body": body, "annotations": annotations
        });
        let raw = serde_json::to_vec(&document).expect("JSON values serialize");
        let canonical = semantic_bytes(&raw).map_err(|e| {
            error(
                format!("lowered IR: {}", e.category()),
                0..self.source.len(),
            )
        })?;
        document["revision"] = Value::String(digest(canonical.as_bytes()));
        Ok(document)
    }
}

struct Parser<'a> {
    source: &'a str,
    pos: usize,
}

impl Parser<'_> {
    fn skip(&mut self) {
        loop {
            while self
                .source
                .as_bytes()
                .get(self.pos)
                .is_some_and(|b| matches!(b, b' ' | b'\n' | b'\r' | b'\t'))
            {
                self.pos += 1;
            }
            if self.source[self.pos..].starts_with("//") {
                self.pos += self.source[self.pos..]
                    .find('\n')
                    .unwrap_or(self.source.len() - self.pos);
            } else {
                break;
            }
        }
    }

    fn expect(&mut self, token: &str) -> Result<()> {
        self.skip();
        if !self.source[self.pos..].starts_with(token) {
            return Err(error(format!("expected {token}"), self.pos..self.pos));
        }
        self.pos += token.len();
        Ok(())
    }

    fn word(&mut self) -> Result<String> {
        self.skip();
        let start = self.pos;
        while self
            .source
            .as_bytes()
            .get(self.pos)
            .is_some_and(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
        {
            self.pos += 1;
        }
        let word = &self.source[start..self.pos];
        if word.len() > 64 || !word.as_bytes().first().is_some_and(u8::is_ascii_alphabetic) {
            return Err(error(
                "expected ASCII identifier (1–64 bytes)",
                start..self.pos,
            ));
        }
        Ok(word.to_owned())
    }

    // Find the semicolon outside strings/containers, then reuse strict JSON
    // admission. This scanner does not recursively build untrusted structures.
    fn payload(&mut self) -> Result<Value> {
        self.skip();
        let start = self.pos;
        let mut depth = 0usize;
        let mut string = false;
        let mut escaped = false;
        while let Some(&byte) = self.source.as_bytes().get(self.pos) {
            if string {
                if escaped {
                    escaped = false;
                } else if byte == b'\\' {
                    escaped = true;
                } else if byte == b'"' {
                    string = false;
                }
            } else {
                match byte {
                    b'"' => string = true,
                    b'{' | b'[' => {
                        depth += 1;
                        if depth > 64 {
                            return Err(error("payload nesting limit", start..self.pos + 1));
                        }
                    }
                    b'}' | b']' => {
                        depth = depth.checked_sub(1).ok_or_else(|| {
                            error("unmatched closing delimiter", start..self.pos + 1)
                        })?;
                    }
                    b';' if depth == 0 => {
                        let end = self.pos;
                        self.pos += 1;
                        return transport::decode(&self.source.as_bytes()[start..end]).map_err(
                            |e| error(format!("JSON payload: {}", e.category()), start..end),
                        );
                    }
                    _ => {}
                }
            }
            self.pos += 1;
        }
        Err(error(
            "unterminated JSON payload; expected semicolon",
            start..self.pos,
        ))
    }
}

/// Parse a complete source artifact. Unknown/missing/duplicate sections and
/// globally duplicate declaration IDs are errors; record semantics are opaque.
pub fn parse(raw: &[u8]) -> Result<Syntax<'_>> {
    if raw.len() > MAX_BYTES {
        return Err(error("source size limit", 0..raw.len()));
    }
    let source = std::str::from_utf8(raw)
        .map_err(|e| error("invalid UTF-8", e.valid_up_to()..e.valid_up_to()))?;
    let mut p = Parser { source, pos: 0 };
    if p.word()? != "choreoform" {
        return Err(error("expected choreoform header", 0..p.pos));
    }
    if p.payload()? != "0.1.0" {
        return Err(error("unsupported textual version", 0..p.pos));
    }
    let mut names = BTreeSet::new();
    let mut ids = BTreeSet::new();
    let mut items = Vec::new();
    loop {
        p.skip();
        let start = p.pos;
        if start == source.len() {
            break;
        }
        let name = p.word()?;
        if !names.insert(name.clone()) {
            return Err(error("duplicate item", start..p.pos));
        }
        let mut records = Vec::new();
        let value = if SECTIONS.contains(&name.as_str()) {
            p.expect("{")?;
            let mut map = Map::new();
            loop {
                p.skip();
                if p.source[p.pos..].starts_with('}') {
                    p.pos += 1;
                    break;
                }
                let record_start = p.pos;
                let id = p.word()?;
                if !ids.insert(id.clone()) {
                    return Err(error("duplicate declaration ID", record_start..p.pos));
                }
                p.expect("=")?;
                let value = p.payload()?;
                if !value.is_object() {
                    return Err(error("record must be a JSON object", record_start..p.pos));
                }
                records.push(RecordSpan {
                    id: id.clone(),
                    span: record_start..p.pos,
                });
                map.insert(id, value);
            }
            Value::Object(map)
        } else if VALUES.contains(&name.as_str()) {
            p.expect("=")?;
            let value = p.payload()?;
            let valid = match name.as_str() {
                "id" | "root" => value.as_str().is_some_and(|s| !s.is_empty()),
                _ => value.is_object(),
            };
            if !valid {
                return Err(error("wrong metadata value shape", start..p.pos));
            }
            value
        } else {
            return Err(error("unknown item", start..p.pos));
        };
        items.push(Item {
            name,
            span: start..p.pos,
            records,
            value,
        });
    }
    for name in VALUES.iter().chain(SECTIONS.iter()) {
        if !names.contains(*name) {
            return Err(error(
                format!("missing item {name}"),
                source.len()..source.len(),
            ));
        }
    }
    Ok(Syntax { source, items })
}

/// Explicit normalized IR-to-text export. Rejects revision mismatch and fields
/// this syntax cannot preserve. Comments from a previous source are NOT restored.
/// This is not a formatter or an overwrite operation, nor semantic validation.
pub fn export(raw: &[u8]) -> Result<String> {
    let canonical = semantic_bytes(raw).map_err(|e| error(e.category(), 0..raw.len()))?;
    let document = transport::decode(raw).map_err(|e| error(e.category(), 0..raw.len()))?;
    if document["revision"] != digest(canonical.as_bytes()) {
        return Err(error("revision mismatch", 0..raw.len()));
    }
    let body = document["body"]
        .as_object()
        .ok_or_else(|| error("body must be object", 0..raw.len()))?;
    if body.len() != 12
        || body
            .keys()
            .any(|k| !SECTIONS.contains(&k.as_str()) && !VALUES[..4].contains(&k.as_str()))
    {
        return Err(error("unknown or missing body field", 0..raw.len()));
    }
    let mut source = String::from("choreoform \"0.1.0\";\n");
    for name in VALUES {
        let value = if name == "annotations" {
            &document[name]
        } else {
            &body[name]
        };
        source.push_str(&format!(
            "{name} = {};\n",
            serde_json::to_string_pretty(value).expect("JSON values serialize")
        ));
    }
    for name in SECTIONS {
        let map = body[name]
            .as_object()
            .ok_or_else(|| error("section must be object", 0..raw.len()))?;
        source.push_str(&format!("\n{name} {{\n"));
        for (id, value) in map {
            source.push_str(&format!(
                "  {id} = {};\n",
                serde_json::to_string_pretty(value).expect("JSON values serialize")
            ));
        }
        source.push_str("}\n");
    }
    // Pretty-printing can expand bounded IR beyond the source limit. This is
    // resource refusal, not evidence that the artifact loses information.
    if source.len() > MAX_BYTES {
        return Err(error("source size limit", 0..raw.len()));
    }
    // Any export diagnostic belongs to the supplied IR artifact, not to byte
    // offsets in an internal generated source the caller has never received.
    let roundtrip = parse(source.as_bytes())
        .and_then(|syntax| syntax.lower())
        .map_err(|e| {
            error(
                format!("unrepresentable text export: {}", e.message),
                0..raw.len(),
            )
        })?;
    if roundtrip != document {
        return Err(error("export would lose information", 0..raw.len()));
    }
    Ok(source)
}
