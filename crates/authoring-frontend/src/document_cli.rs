// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Explicit bounded IO adapter. Artifact authorization belongs to its caller.
use choreoform_authoring_frontend::{
    AuthoringComponent, AuthoringDiagnostic, AuthoringDocument, MAX_BYTES,
};
use choreoform_ir_probe_core::Resource;
use std::io::Read;
fn read(path: &str, limit: usize) -> Result<Vec<u8>, String> {
    let mut raw = Vec::new();
    std::fs::File::open(path)
        .map_err(|_| "read")?
        .take(limit as u64 + 1)
        .read_to_end(&mut raw)
        .map_err(|_| "read")?;
    if raw.len() > limit {
        return Err("authoring-size".into());
    }
    Ok(raw)
}
fn diagnostic(d: AuthoringDiagnostic) -> String {
    let component = match d.component {
        AuthoringComponent::Document => "document",
        AuthoringComponent::Source => "source",
        AuthoringComponent::Bindings => "bindings",
        AuthoringComponent::Layout => "layout",
        AuthoringComponent::Change => "change",
        AuthoringComponent::Resources => "resources",
    };
    let entity = d
        .entity
        .map_or(String::new(), |id| format!("; entity {id}"));
    format!(
        "{} at {}{}{}; source bytes {}..{}",
        d.code, component, d.path, entity, d.span.start, d.span.end
    )
}
pub(super) fn run(args: &[String]) -> Result<String, String> {
    let count = match args.first().map(String::as_str) {
        Some("document-inspect" | "document-export") => 2,
        Some("document-change") => 3,
        Some("document-import") => 4,
        _ => return Err("arguments".into()),
    };
    if args.len() < count || args.len() > count + 128 * 4 || !(args.len() - count).is_multiple_of(4)
    {
        return Err("arguments".into());
    }
    let mut extra = Vec::new();
    let mut total = 0usize;
    for row in args[count..].as_chunks::<4>().0 {
        if row[0] != "--resource" {
            return Err("arguments".into());
        }
        let raw = read(&row[3], MAX_BYTES)?;
        total = total.checked_add(raw.len()).ok_or("registry-limit")?;
        if total > 8 * MAX_BYTES {
            return Err("registry-limit".into());
        }
        extra.push((row[1].as_str(), row[2].as_str(), raw));
    }
    let resources: Vec<_> = extra
        .iter()
        .map(|(id, revision, bytes)| Resource {
            id,
            revision,
            bytes,
        })
        .collect();
    let document = if args[0] == "document-import" {
        let source = read(&args[1], MAX_BYTES)?;
        let bindings = read(&args[2], MAX_BYTES)?;
        let layout = read(&args[3], MAX_BYTES)?;
        AuthoringDocument::from_package(&source, &bindings, &layout, &resources)
            .map_err(diagnostic)?
    } else {
        let raw = read(&args[1], 16 * MAX_BYTES)?;
        let document = AuthoringDocument::admit(&raw, &resources).map_err(diagnostic)?;
        if args[0] == "document-change" {
            let request = read(&args[2], MAX_BYTES)?;
            document.apply(&request, &resources).map_err(diagnostic)?
        } else {
            document
        }
    };
    if args[0] == "document-inspect" {
        serde_json::to_string(&serde_json::json!({"format":"choreoform-authoring-inspection","version":"0.1.0","status":"projection-matched","executable":false,"definition":document.candidate().document()["body"]["id"],"semanticRevision":document.candidate().document()["revision"],"authoringDigest":document.authoring_digest(),"sourceBytes":document.source().len(),"bindingsBytes":document.bindings().len(),"layoutEntries":document.document()["layout"]["entries"].as_array().expect("checked layout").len()})).map_err(|_|"output-json".into())
    } else {
        String::from_utf8(document.export().map_err(diagnostic)?).map_err(|_| "output-json".into())
    }
}
