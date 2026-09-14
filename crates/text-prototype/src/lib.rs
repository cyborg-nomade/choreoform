// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

//! Disposable ADR-0011 syntax prototype. Lowering produces UNVALIDATED IR.
//! No IO, registry lookup, policy interpretation, or execution in this library.

use choreoform_ir_probe_core::bounded::LimitedWriter;
use choreoform_ir_probe_core::{DefinitionEnvelope, digest, transport};
use serde_json::{Map, Value};
use std::{collections::BTreeSet, io::Write, ops::Range};

pub const MAX_BYTES: usize = transport::MAX_BYTES;

// One fixed spelling table per vocabulary, shared by parsing and export.
macro_rules! vocabulary {
    ($name:ident { $($variant:ident => $word:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name { $($variant),+ }
        impl $name {
            const ALL: &'static [Self] = &[$(Self::$variant),+];
            pub fn as_str(self) -> &'static str { match self { $(Self::$variant => $word),+ } }
            fn parse(word: &str) -> Option<Self> { match word { $($word => Some(Self::$variant)),+, _ => None } }
        }
    };
}
vocabulary!(MetadataKind { Id => "id", Semantics => "semantics", Dialects => "dialects", Root => "root", Annotations => "annotations" });
vocabulary!(SectionKind { Scopes => "scopes", Data => "data", Expressions => "expressions", Actors => "actors", Capabilities => "capabilities", Policies => "policies", Nodes => "nodes", Flows => "flows" });

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
#[derive(Debug, Clone)]
pub struct Record<'a> {
    pub id: &'a str,
    pub span: Range<usize>,
    payload: Value,
}

/// Source-level item, distinct from an IR declaration or runtime occurrence.
#[derive(Debug, Clone)]
pub enum Item<'a> {
    Metadata {
        kind: MetadataKind,
        value: Value,
        span: Range<usize>,
    },
    Section {
        kind: SectionKind,
        records: Vec<Record<'a>>,
        span: Range<usize>,
    },
}

impl<'a> Item<'a> {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Metadata { kind, .. } => kind.as_str(),
            Self::Section { kind, .. } => kind.as_str(),
        }
    }
    pub fn span(&self) -> &Range<usize> {
        match self {
            Self::Metadata { span, .. } | Self::Section { span, .. } => span,
        }
    }
    pub fn records(&self) -> &[Record<'a>] {
        match self {
            Self::Metadata { .. } => &[],
            Self::Section { records, .. } => records,
        }
    }
}

/// Retains the exact borrowed source (including trivia) and parsed item spans.
/// It is immutable through this API. No incremental editing/recovery is implied.
#[derive(Debug, Clone)]
pub struct Syntax<'a> {
    source: &'a str,
    items: Vec<Item<'a>>,
}

/// Original-source locations retained when payloads are moved into candidate IR.
#[derive(Debug)]
pub struct ItemSpan<'a> {
    pub name: &'static str,
    pub span: Range<usize>,
    pub records: Vec<(&'a str, Range<usize>)>,
}

/// One lowering operation returns the candidate, both identities and source map.
/// These are evidence artifacts, not a semantic validation certificate.
#[derive(Debug)]
pub struct Lowered<'a> {
    pub document: Value,
    pub binding: SourceBinding,
    pub source: &'a str,
    pub spans: Vec<ItemSpan<'a>>,
}

/// Persist these two identities with any extracted spans. Never reuse a map
/// after either the original source bytes or semantic revision changes.
#[derive(Debug, PartialEq, Eq)]
pub struct SourceBinding {
    pub source_digest: String,
    pub semantic_revision: String,
}

impl<'a> Syntax<'a> {
    /// Bind source spans to exact source bytes and the candidate's semantic
    /// revision. Returns an error if the complete lowered envelope exceeds limits.
    pub fn binding(&self) -> Result<SourceBinding> {
        Ok(self.lower_with_binding()?.binding)
    }
    /// Return the original source verbatim, including comments and whitespace.
    pub fn source(&self) -> &str {
        self.source
    }
    /// Return immutable items in original source order, with byte spans.
    pub fn items(&self) -> &[Item<'a>] {
        &self.items
    }

    /// Produce a revision-bearing candidate, NOT a validation/admission result.
    /// Source comments/spans stay in Syntax; supplied annotations survive exactly.
    pub fn lower(&self) -> Result<Value> {
        Ok(self.lower_with_binding()?.document)
    }

    /// Retain this immutable syntax while producing candidate and binding once.
    pub fn lower_with_binding(&self) -> Result<Lowered<'a>> {
        self.clone().into_lowered()
    }

    /// Move decoded payloads without cloning them; retain original source/spans.
    pub fn into_lowered(self) -> Result<Lowered<'a>> {
        let mut body = Map::new();
        let mut annotations = Value::Null;
        let mut spans = Vec::new();
        for item in self.items {
            let location = ItemSpan {
                name: item.name(),
                span: item.span().clone(),
                records: item
                    .records()
                    .iter()
                    .map(|r| (r.id, r.span.clone()))
                    .collect(),
            };
            match item {
                Item::Metadata {
                    kind: MetadataKind::Annotations,
                    value,
                    ..
                } => annotations = value,
                Item::Metadata { kind, value, .. } => {
                    body.insert(kind.as_str().into(), value);
                }
                Item::Section { kind, records, .. } => {
                    body.insert(
                        kind.as_str().into(),
                        Value::Object(
                            records
                                .into_iter()
                                .map(|r| (r.id.to_owned(), r.payload))
                                .collect(),
                        ),
                    );
                }
            }
            spans.push(location);
        }
        let document = DefinitionEnvelope::from_parts(Value::Object(body), annotations)
            .map_err(|e| {
                error(
                    format!("lowered IR: {}", e.category()),
                    0..self.source.len(),
                )
            })?
            .into_document();
        let binding = SourceBinding {
            source_digest: digest(self.source.as_bytes()),
            semantic_revision: document["revision"]
                .as_str()
                .expect("constructor computes revision")
                .to_owned(),
        };
        Ok(Lowered {
            document,
            binding,
            source: self.source,
            spans,
        })
    }
}

struct Parser<'a> {
    source: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
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

    fn word(&mut self) -> Result<&'a str> {
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
        Ok(word)
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
        if !names.insert(name) {
            return Err(error("duplicate item", start..p.pos));
        }
        let item = if let Some(kind) = SectionKind::parse(name) {
            p.expect("{")?;
            let mut records = Vec::new();
            loop {
                p.skip();
                if p.source[p.pos..].starts_with('}') {
                    p.pos += 1;
                    break;
                }
                let record_start = p.pos;
                let id = p.word()?;
                if !ids.insert(id) {
                    return Err(error("duplicate declaration ID", record_start..p.pos));
                }
                p.expect("=")?;
                let value = p.payload()?;
                if !value.is_object() {
                    return Err(error("record must be a JSON object", record_start..p.pos));
                }
                records.push(Record {
                    id,
                    span: record_start..p.pos,
                    payload: value,
                });
            }
            Item::Section {
                kind,
                records,
                span: start..p.pos,
            }
        } else if let Some(kind) = MetadataKind::parse(name) {
            p.expect("=")?;
            let value = p.payload()?;
            let valid = match kind {
                MetadataKind::Id | MetadataKind::Root => {
                    value.as_str().is_some_and(|s| !s.is_empty())
                }
                _ => value.is_object(),
            };
            if !valid {
                return Err(error("wrong metadata value shape", start..p.pos));
            }
            Item::Metadata {
                kind,
                value,
                span: start..p.pos,
            }
        } else {
            return Err(error("unknown item", start..p.pos));
        };
        items.push(item);
    }
    for name in MetadataKind::ALL
        .iter()
        .map(|k| k.as_str())
        .chain(SectionKind::ALL.iter().map(|k| k.as_str()))
    {
        if !names.contains(name) {
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
    let admitted =
        DefinitionEnvelope::decode(raw).map_err(|e| error(e.category(), 0..raw.len()))?;
    let canonical = admitted.canonical();
    let document = admitted.document();
    if document["revision"] != digest(canonical.as_bytes()) {
        return Err(error("revision mismatch", 0..raw.len()));
    }
    let body = document["body"]
        .as_object()
        .ok_or_else(|| error("body must be object", 0..raw.len()))?;
    if body.len() != 12
        || body.keys().any(|k| {
            SectionKind::parse(k).is_none()
                && !matches!(
                    MetadataKind::parse(k),
                    Some(
                        MetadataKind::Id
                            | MetadataKind::Semantics
                            | MetadataKind::Dialects
                            | MetadataKind::Root
                    )
                )
        })
    {
        return Err(error("unknown or missing body field", 0..raw.len()));
    }
    // Buffer under a byte budget; nothing reaches the host until the complete
    // export and its self-round-trip guard succeed.
    let mut output = LimitedWriter::new(Vec::new(), MAX_BYTES);
    let mut emit = || -> std::result::Result<(), serde_json::Error> {
        output
            .write_all(b"choreoform \"0.1.0\";\n")
            .map_err(serde_json::Error::io)?;
        for kind in MetadataKind::ALL {
            let name = kind.as_str();
            let value = if name == "annotations" {
                &document[name]
            } else {
                &body[name]
            };
            write!(output, "{name} = ").map_err(serde_json::Error::io)?;
            serde_json::to_writer_pretty(&mut output, value)?;
            output.write_all(b";\n").map_err(serde_json::Error::io)?;
        }
        for kind in SectionKind::ALL {
            let name = kind.as_str();
            let map = body[name]
                .as_object()
                .expect("section shape checked before writing");
            write!(output, "\n{name} {{\n").map_err(serde_json::Error::io)?;
            for (id, value) in map {
                write!(output, "  {id} = ").map_err(serde_json::Error::io)?;
                serde_json::to_writer_pretty(&mut output, value)?;
                output.write_all(b";\n").map_err(serde_json::Error::io)?;
            }
            output.write_all(b"}\n").map_err(serde_json::Error::io)?;
        }
        Ok(())
    };
    // Check shapes before the closure's infallible indexing, without generating
    // oversized intermediates or converting semantic fields to defaults.
    for kind in SectionKind::ALL {
        let name = kind.as_str();
        if !body[name].is_object() {
            return Err(error("section must be object", 0..raw.len()));
        }
    }
    if let Err(e) = emit() {
        return Err(error(
            if output.exceeded() {
                "source size limit".to_owned()
            } else {
                e.to_string()
            },
            0..raw.len(),
        ));
    }
    let source =
        String::from_utf8(output.into_inner()).expect("writer emits UTF-8 JSON and ASCII syntax");
    // Any export diagnostic belongs to the supplied IR artifact, not to byte
    // offsets in an internal generated source the caller has never received.
    let roundtrip = parse(source.as_bytes())
        .and_then(|syntax| syntax.into_lowered())
        .map_err(|e| {
            error(
                format!("unrepresentable text export: {}", e.message),
                0..raw.len(),
            )
        })?;
    if &roundtrip.document != document {
        return Err(error("export would lose information", 0..raw.len()));
    }
    Ok(source)
}
