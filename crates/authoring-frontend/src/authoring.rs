// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Immutable checked authoring state. Caller authorization remains external.
use crate::{Candidate, MAX_BYTES, Package};
use choreoform_ir_probe_core::{Resource, bounded::LimitedWriter, digest, transport};
use serde_json::{Map, Value, json};
use std::ops::Range;
mod rename;
mod resources;

const DOCUMENT_BYTES: usize = 16 * MAX_BYTES;
const LAYOUT_BYTES: usize = 256 * 1024;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthoringComponent {
    Document,
    Source,
    Bindings,
    Layout,
    Change,
    Resources,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoringDiagnostic {
    pub code: &'static str,
    pub component: AuthoringComponent,
    pub path: &'static str,
    pub span: Range<usize>,
    pub entity: Option<String>,
}
pub type AuthoringResult<T> = std::result::Result<T, AuthoringDiagnostic>;
fn diagnostic(
    code: &'static str,
    component: AuthoringComponent,
    path: &'static str,
) -> AuthoringDiagnostic {
    AuthoringDiagnostic {
        code,
        component,
        path,
        span: 0..0,
        entity: None,
    }
}
fn located((bindings, error): (bool, crate::Diagnostic)) -> AuthoringDiagnostic {
    let mut d = diagnostic(
        error.code,
        if bindings {
            AuthoringComponent::Bindings
        } else {
            AuthoringComponent::Source
        },
        "",
    );
    d.span = error.span;
    d
}
fn source_error(error: crate::Diagnostic) -> AuthoringDiagnostic {
    located((false, error))
}
fn decode(
    raw: &[u8],
    maximum: usize,
    component: AuthoringComponent,
    code: &'static str,
) -> AuthoringResult<Value> {
    transport::decode_with_limits(
        raw,
        transport::DecodeLimits {
            max_bytes: maximum,
            max_containers: 64,
            max_items: 262_144,
            reject_negative_zero: true,
        },
    )
    .map_err(|_| diagnostic(code, component, ""))
}
fn fields<'a>(
    value: &'a Value,
    names: &[&str],
    component: AuthoringComponent,
    code: &'static str,
) -> AuthoringResult<&'a Map<String, Value>> {
    value
        .as_object()
        .filter(|m| m.len() == names.len() && names.iter().all(|n| m.contains_key(*n)))
        .ok_or_else(|| diagnostic(code, component, ""))
}
// All document/layout/change/inspection member names are closed ASCII fields.
// serde_json's pinned BTreeMap object representation therefore has JCS ordering;
// arbitrary companion annotations use the existing native UTF-16 JCS writer.
fn bytes(value: &Value, maximum: usize, component: AuthoringComponent) -> AuthoringResult<Vec<u8>> {
    let mut out = LimitedWriter::new(Vec::new(), maximum);
    serde_json::to_writer(&mut out, value)
        .map_err(|_| diagnostic("authoring-size", component, ""))?;
    Ok(out.into_inner())
}
fn integrity(package: &Package<'_>) -> Value {
    let i = package.integrity();
    json!({"source": i.source, "bindings": i.bindings, "package": i.package})
}
fn projection(candidate: &Candidate) -> Value {
    let ir = candidate.document();
    json!({"format":ir["format"],"version":ir["version"],"definition":ir["body"]["id"],"revision":ir["revision"]})
}
fn layout(value: &Value, ledger: &crate::Ledger) -> AuthoringResult<()> {
    use AuthoringComponent::Layout;
    let fail = || diagnostic("layout-format", Layout, "/layout");
    fields(
        value,
        &["format", "version", "entries"],
        Layout,
        "layout-format",
    )?;
    if value["format"] != "choreoform-authoring-layout" || value["version"] != "0.1.0" {
        return Err(fail());
    }
    let entries = value["entries"]
        .as_array()
        .filter(|a| a.len() <= 10000)
        .ok_or_else(fail)?;
    bytes(value, LAYOUT_BYTES, Layout)?;
    let mut previous: Option<&str> = None;
    let generated: std::collections::BTreeSet<_> =
        ledger.slots.iter().map(|r| r.id.as_str()).collect();
    for row in entries {
        fields(
            row,
            &["entity", "x", "y", "collapsed"],
            Layout,
            "layout-format",
        )?;
        let id = row["entity"]
            .as_str()
            .filter(|id| crate::ledger::valid_id(id))
            .ok_or_else(fail)?;
        for field in ["x", "y"] {
            if !row[field]
                .as_i64()
                .is_some_and(|v| (-1_000_000..=1_000_000).contains(&v))
            {
                return Err(fail());
            }
        }
        if !row["collapsed"].is_boolean() {
            return Err(fail());
        }
        if previous.is_some_and(|p| p >= id)
            || !(ledger.declarations.contains_key(id)
                || ledger.symbols.contains_key(id)
                || generated.contains(id))
        {
            let mut d = diagnostic("layout-reference", Layout, "/layout/entries");
            d.entity = Some(id.into());
            return Err(d);
        }
        previous = Some(id);
    }
    Ok(())
}
fn components<'a>(source: &'a [u8], bindings: &[u8]) -> AuthoringResult<Package<'a>> {
    for (raw, component) in [
        (source, AuthoringComponent::Source),
        (bindings, AuthoringComponent::Bindings),
    ] {
        if raw.len() > MAX_BYTES {
            return Err(diagnostic("component-limit", component, ""));
        }
    }
    let package = Package::parse_located(source, bindings).map_err(located)?;
    if package.syntax().profile() != "0.19.0" || package.ledger().document()["version"] != "0.18.0"
    {
        return Err(diagnostic(
            "component-profile",
            AuthoringComponent::Document,
            "",
        ));
    }
    Ok(package)
}

/// An immutable authoring projection, never executable admission or access authority.
/// ```compile_fail
/// use choreoform_authoring_frontend::AuthoringDocument;
/// fn tamper(d: &mut AuthoringDocument) { d.document()["source"] = "forged".into(); }
/// ```
/// ```compile_fail
/// use choreoform_authoring_frontend::AuthoringDocument;
/// fn detach(d: &mut AuthoringDocument) { d.candidate = todo!(); }
/// ```
#[derive(Debug)]
pub struct AuthoringDocument {
    document: Value,
    candidate: Candidate,
    authoring_digest: String,
}
impl AuthoringDocument {
    pub fn admit(raw: &[u8], supplied: &[Resource<'_>]) -> AuthoringResult<Self> {
        use AuthoringComponent::Document;
        let document = decode(raw, DOCUMENT_BYTES, Document, "document-format")?;
        fields(
            &document,
            &[
                "format",
                "version",
                "source",
                "bindings",
                "integrity",
                "projection",
                "layout",
            ],
            Document,
            "document-format",
        )?;
        if document["format"] != "choreoform-authoring-document" || document["version"] != "0.1.0" {
            return Err(diagnostic("document-format", Document, ""));
        }
        let s = document["source"]
            .as_str()
            .ok_or_else(|| diagnostic("document-format", Document, "/source"))?;
        let b = document["bindings"]
            .as_str()
            .ok_or_else(|| diagnostic("document-format", Document, "/bindings"))?;
        let p = components(s.as_bytes(), b.as_bytes())?;
        if document["integrity"] != integrity(&p) {
            return Err(diagnostic("integrity-mismatch", Document, "/integrity"));
        }
        let resources = resources::registry(supplied)?;
        p.ledger().verify_resources(&resources).map_err(|e| {
            let mut d = diagnostic(e.code, AuthoringComponent::Resources, "");
            d.span = e.span;
            d
        })?;
        layout(&document["layout"], p.ledger())?;
        let candidate = p.lower(&resources).map_err(source_error)?;
        if document["projection"] != projection(&candidate) {
            return Err(diagnostic("projection-mismatch", Document, "/projection"));
        }
        Self::checked(document, candidate)
    }
    fn checked(document: Value, candidate: Candidate) -> AuthoringResult<Self> {
        let raw = bytes(&document, DOCUMENT_BYTES, AuthoringComponent::Document)?;
        let mut framed = b"choreoform-authoring-document:0.1.0\0".to_vec();
        framed.extend_from_slice(&raw);
        Ok(Self {
            document,
            candidate,
            authoring_digest: digest(&framed),
        })
    }
    pub fn from_package(
        source: &[u8],
        bindings: &[u8],
        placement: &[u8],
        supplied: &[Resource<'_>],
    ) -> AuthoringResult<Self> {
        let p = components(source, bindings)?;
        let placement = decode(
            placement,
            MAX_BYTES,
            AuthoringComponent::Layout,
            "layout-format",
        )?;
        layout(&placement, p.ledger())?;
        let resources = resources::registry(supplied)?;
        p.ledger()
            .verify_resources(&resources)
            .map_err(|e| diagnostic(e.code, AuthoringComponent::Resources, ""))?;
        let candidate = p.lower(&resources).map_err(source_error)?;
        // Parsing has already established exact UTF-8; retain the original bytes.
        let document = json!({"format":"choreoform-authoring-document","version":"0.1.0","source":p.syntax().source(),"bindings":std::str::from_utf8(bindings).map_err(|_| diagnostic("bindings-format",AuthoringComponent::Bindings,""))?,"integrity":integrity(&p),"projection":projection(&candidate),"layout":placement});
        Self::checked(document, candidate)
    }
    pub fn document(&self) -> &Value {
        &self.document
    }
    pub fn candidate(&self) -> &Candidate {
        &self.candidate
    }
    pub fn authoring_digest(&self) -> &str {
        &self.authoring_digest
    }
    pub fn source(&self) -> &str {
        self.document["source"].as_str().expect("admitted source")
    }
    pub fn bindings(&self) -> &str {
        self.document["bindings"]
            .as_str()
            .expect("admitted bindings")
    }
    pub fn export(&self) -> AuthoringResult<Vec<u8>> {
        bytes(&self.document, DOCUMENT_BYTES, AuthoringComponent::Document)
    }
    pub fn apply(&self, raw: &[u8], supplied: &[Resource<'_>]) -> AuthoringResult<Self> {
        use AuthoringComponent::Change;
        let request = decode(raw, MAX_BYTES, Change, "change-format")?;
        fields(
            &request,
            &["format", "version", "base", "operation"],
            Change,
            "change-format",
        )?;
        if request["format"] != "choreoform-authoring-change" || request["version"] != "0.1.0" {
            return Err(diagnostic("change-format", Change, ""));
        }
        let operation = &request["operation"];
        let kind = operation["kind"]
            .as_str()
            .ok_or_else(|| diagnostic("change-format", Change, "/operation"))?;
        let keys: &[&str] = match kind {
            "format" => &["kind"],
            "rename" => &["kind", "target", "name"],
            "replace-layout" => &["kind", "layout"],
            _ => return Err(diagnostic("change-format", Change, "/operation")),
        };
        fields(operation, keys, Change, "change-format")?;
        let base = request["base"]
            .as_str()
            .filter(|s| {
                s.len() == 71
                    && s.starts_with("sha256:")
                    && s.as_bytes()[7..]
                        .iter()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(b))
            })
            .ok_or_else(|| diagnostic("change-format", Change, "/base"))?;
        if kind == "rename" {
            rename::validate(operation)?;
        }
        let p = components(self.source().as_bytes(), self.bindings().as_bytes())?;
        if kind == "replace-layout" {
            layout(&operation["layout"], p.ledger())?;
        }
        if base != self.authoring_digest() {
            return Err(diagnostic("stale-base", Change, "/base"));
        }
        let resources = resources::registry(supplied)?;
        p.ledger()
            .verify_resources(&resources)
            .map_err(|e| diagnostic(e.code, AuthoringComponent::Resources, ""))?;
        let (source, bindings, placement) = match kind {
            "format" => {
                let (s, b) = p.formatted().map_err(source_error)?;
                (s, b, self.document["layout"].clone())
            }
            "replace-layout" => {
                layout(&operation["layout"], p.ledger())?;
                (
                    self.source().into(),
                    self.bindings().into(),
                    operation["layout"].clone(),
                )
            }
            "rename" => {
                let (s, b) = rename::rename(&p, operation, self.bindings())?;
                (s, b, self.document["layout"].clone())
            }
            _ => unreachable!("checked kind"),
        };
        let placement = bytes(&placement, LAYOUT_BYTES, AuthoringComponent::Layout)?;
        let result =
            Self::from_package(source.as_bytes(), bindings.as_bytes(), &placement, supplied)?;
        if result.candidate.document() != self.candidate.document() {
            return Err(diagnostic("projection-mismatch", Change, "/operation"));
        }
        Ok(result)
    }
}
