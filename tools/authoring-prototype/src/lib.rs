// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! One native/browser adapter. No IO, authentication or execution authority.
use choreoform_authoring_frontend::{AuthoringComponent, AuthoringDiagnostic, AuthoringDocument};
use choreoform_ir_probe_core::{Resource, bounded::LimitedWriter, transport};
use serde_json::Value;
const MIB: usize = 1024 * 1024;
fn fields(v: &Value, keys: &[&str]) -> bool {
    v.as_object()
        .is_some_and(|m| m.len() == keys.len() && keys.iter().all(|k| m.contains_key(*k)))
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
    let entity = d.entity.map_or(String::new(), |s| format!("; entity {s}"));
    format!(
        "{} at {}{}{}; source bytes {}..{}",
        d.code, component, d.path, entity, d.span.start, d.span.end
    )
}
fn encode(v: &Value, max: usize) -> Result<String, String> {
    let mut w = LimitedWriter::new(Vec::new(), max);
    serde_json::to_writer(&mut w, v).map_err(|_| "view-limit")?;
    String::from_utf8(w.into_inner()).map_err(|_| "view-format".into())
}
fn header(raw: &[u8]) -> Result<Value, String> {
    let v = transport::decode_with_limits(
        raw,
        transport::DecodeLimits {
            max_bytes: 4 * MIB,
            max_containers: 64,
            max_items: 262144,
            reject_negative_zero: true,
        },
    )
    .map_err(|_| "registry-format")?;
    if !fields(&v, &["format", "version", "resources"])
        || v["format"] != "choreoform-authoring-resource-registry"
        || v["version"] != "0.1.0"
    {
        return Err("registry-format".into());
    }
    encode(&v, 4 * MIB)?;
    Ok(v)
}
fn resources<'a>(header: &'a Value, blob: &'a [u8]) -> Result<Vec<Resource<'a>>, String> {
    if blob.len() > 8 * MIB {
        return Err("registry-limit".into());
    }
    let rows = header["resources"]
        .as_array()
        .filter(|a| a.len() <= 128)
        .ok_or("registry-format")?;
    let mut cursor = 0usize;
    let mut result = Vec::new();
    for row in rows {
        if !fields(row, &["contract", "digest", "start", "length"]) {
            return Err("registry-format".into());
        }
        let id = row["contract"].as_str().ok_or("registry-format")?;
        let revision = row["digest"].as_str().ok_or("registry-format")?;
        let start = row["start"]
            .as_u64()
            .filter(|n| *n <= 8 * MIB as u64)
            .ok_or("registry-range")? as usize;
        let length = row["length"]
            .as_u64()
            .filter(|n| *n <= MIB as u64)
            .ok_or("registry-range")? as usize;
        if start != cursor {
            return Err("registry-range".into());
        }
        cursor = start
            .checked_add(length)
            .filter(|n| *n <= blob.len())
            .ok_or("registry-range")?;
        result.push(Resource {
            id,
            revision,
            bytes: &blob[start..cursor],
        });
    }
    if cursor != blob.len() {
        return Err("registry-range".into());
    }
    Ok(result)
}
fn with_document(
    raw: &[u8],
    registry: &[u8],
    blob: &[u8],
    f: impl FnOnce(AuthoringDocument, &[Resource<'_>]) -> Result<String, String>,
) -> Result<String, String> {
    if raw.len() > 16 * MIB {
        return Err("document-format".into());
    }
    if blob.len() > 8 * MIB {
        return Err("registry-limit".into());
    }
    let header = header(registry)?;
    let resources = resources(&header, blob)?;
    let document = AuthoringDocument::admit(raw, &resources).map_err(diagnostic)?;
    f(document, &resources)
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn canonical_document(
    document: &[u8],
    registry: &[u8],
    resources: &[u8],
) -> Result<String, String> {
    with_document(document, registry, resources, |d, _| {
        String::from_utf8(d.export().map_err(diagnostic)?).map_err(|_| "document-format".into())
    })
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn change_document(
    document: &[u8],
    change: &[u8],
    registry: &[u8],
    resources: &[u8],
) -> Result<String, String> {
    if change.len() > MIB {
        return Err("change-format".into());
    }
    with_document(document, registry, resources, |d, r| {
        String::from_utf8(
            d.apply(change, r)
                .map_err(diagnostic)?
                .export()
                .map_err(diagnostic)?,
        )
        .map_err(|_| "document-format".into())
    })
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn view_document(document: &[u8], registry: &[u8], resources: &[u8]) -> Result<String, String> {
    with_document(document, registry, resources, |d, _| {
        encode(&d.view().map_err(diagnostic)?, 16 * MIB)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    const DOC: &[u8] = include_bytes!("../../../docs/authoring/paired-0.1/document.json");
    const HEADER: &[u8] = include_bytes!("../../../docs/authoring/paired-0.1/registry.json");
    const BLOB: &[u8] = include_bytes!("../../../docs/authoring/examples/service.named-types.json");
    #[test]
    fn native_view_is_exact_authored_golden_with_real_locations_and_complete_identity() {
        let actual: Value =
            serde_json::from_str(&view_document(DOC, HEADER, BLOB).unwrap()).unwrap();
        let expected: Value = serde_json::from_slice(include_bytes!(
            "../../../docs/authoring/paired-0.1/view.json"
        ))
        .unwrap();
        assert_eq!(actual, expected);
        let canonical = canonical_document(DOC, HEADER, BLOB).unwrap();
        assert_eq!(
            canonical_document(canonical.as_bytes(), HEADER, BLOB).unwrap(),
            canonical
        );
    }
    #[test]
    fn registry_ranges_authority_claims_pins_and_opaque_invalid_input_refuse_privately() {
        assert_eq!(
            view_document(
                DOC,
                include_bytes!("../../../docs/authoring/paired-0.1/registry-gap.json"),
                BLOB
            )
            .unwrap_err(),
            "registry-range"
        );
        assert_eq!(
            view_document(
                DOC,
                include_bytes!("../../../docs/authoring/paired-0.1/registry-authority-claim.json"),
                BLOB
            )
            .unwrap_err(),
            "registry-format"
        );
        let mut header: Value = serde_json::from_slice(HEADER).unwrap();
        header["resources"][0]["length"] = json!(BLOB.len() - 1);
        assert_eq!(
            view_document(DOC, &serde_json::to_vec(&header).unwrap(), BLOB).unwrap_err(),
            "registry-range"
        );
        let mut changed = BLOB.to_vec();
        changed[0] = b'x';
        let error = view_document(DOC, HEADER, &changed).unwrap_err();
        assert!(error.starts_with("resource-integrity"));
        for raw in [
            b"{\"SECRET\":0,\"SECRET\":1}".as_slice(),
            b"{\"x\":-0}",
            b"{\"x\":1.0}",
            b"[]",
        ] {
            let error = view_document(DOC, raw, BLOB).unwrap_err();
            assert!(!error.contains("SECRET"));
        }
    }
    #[test]
    fn all_published_actual_changes_and_views_keep_one_semantic_projection() {
        let d: Value = serde_json::from_slice(DOC).unwrap();
        let before: Value =
            serde_json::from_str(&view_document(DOC, HEADER, BLOB).unwrap()).unwrap();
        let original = &before["ir"];
        for operation in [
            json!({"kind":"format"}),
            json!({"kind":"rename","target":"same","name":"New test"}),
            json!({"kind":"rename","target":"same_value","name":"Operand"}),
            json!({"kind":"replace-layout","layout":{"format":"choreoform-authoring-layout","version":"0.1.0","entries":[{"entity":"same","x":-10,"y":20,"collapsed":true}]}}),
        ] {
            let request = json!({"format":"choreoform-authoring-change","version":"0.1.0","base":before["binding"]["authoring"],"operation":operation});
            let result =
                change_document(DOC, &serde_json::to_vec(&request).unwrap(), HEADER, BLOB).unwrap();
            let view: Value =
                serde_json::from_str(&view_document(result.as_bytes(), HEADER, BLOB).unwrap())
                    .unwrap();
            assert_eq!(&view["ir"], original);
            assert_ne!(view["binding"]["authoring"], before["binding"]["authoring"]);
            assert_eq!(view["entities"].as_array().unwrap().len(), 21);
        }
        assert_eq!(
            d["projection"]["revision"],
            before["binding"]["semanticRevision"]
        );
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn existing_admitted_component_limits_do_not_silently_expand_view_limits() {
        let d: Value = serde_json::from_slice(include_bytes!(
            "../../../docs/authoring/reversible-0.1/base.json"
        ))
        .unwrap();
        let mut ledger: Value = serde_json::from_str(d["bindings"].as_str().unwrap()).unwrap();
        ledger["annotations"]["many"] = json!(vec![0; 270000]);
        let admitted = AuthoringDocument::from_package(
            d["source"].as_str().unwrap().as_bytes(),
            &serde_json::to_vec(&ledger).unwrap(),
            br#"{"format":"choreoform-authoring-layout","version":"0.1.0","entries":[]}"#,
            &[],
        )
        .unwrap();
        assert!(admitted.export().is_ok());
        assert_eq!(admitted.view().unwrap_err().code, "view-limit");
    }
}
