// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Bounded static declarations, including unvisited sites; never accepted values.
use choreoform_ir_probe_core::bounded::LimitedWriter;
use choreoform_semantic_check::Report;
use serde_json::{Value, json};

pub(super) fn static_output(report: &Report, mut value: Value) -> Result<String, String> {
    let mut remaining = super::MAX_BYTES;
    let mut rows = Vec::new();
    for capture in report.capture_bindings() {
        let pin = |p: &choreoform_semantic_check::Pin| json!({"id":p.id,"revision":p.revision});
        let row = json!({"revision":capture.revision(),"site":capture.site(),"source":pin(capture.source()),"profile":pin(capture.profile()),"provider":pin(capture.provider()),"origin":capture.origin(),"result":capture.result(),"hostRequirements":capture.remaining()});
        push_row(&mut rows, row, &mut remaining)?;
    }
    if !rows.is_empty() {
        value["declaredWaitCaptures"] = rows.into();
    }
    super::fault_use_output::append(report, &mut value, &mut remaining)?;
    let mut writer = LimitedWriter::new(Vec::new(), super::MAX_BYTES);
    serde_json::to_writer(&mut writer, &value).map_err(|_| "report-budget".to_owned())?;
    String::from_utf8(writer.into_inner()).map_err(|_| "report-json".to_owned())
}

pub(super) fn push_row(
    rows: &mut Vec<Value>,
    row: Value,
    remaining: &mut usize,
) -> Result<(), String> {
    let size = serde_json::to_vec(&row)
        .map_err(|_| "report-budget".to_owned())?
        .len();
    *remaining = remaining
        .checked_sub(size)
        .ok_or_else(|| "report-budget".to_owned())?;
    rows.push(row);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use choreoform_ir_probe_core::{DefinitionEnvelope, Resource, digest};
    use choreoform_semantic_check::{
        CORE, CORE_V2, CORE_V3, CORE_V4, CORE_V5, SEMANTICS, TERMINAL_OBSERVATION_PROFILE,
    };
    #[test]
    fn final_combined_report_refuses_even_when_one_capture_row_is_small() {
        let corpus: Value = serde_json::from_slice(include_bytes!(
            "../../semantic-check/tests/conformance/wait-capture.json"
        ))
        .unwrap();
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut owned = Vec::new();
        for pin in [
            SEMANTICS,
            CORE,
            CORE_V2,
            CORE_V3,
            CORE_V4,
            CORE_V5,
            TERMINAL_OBSERVATION_PROFILE,
        ] {
            owned.push((
                pin.0.to_owned(),
                pin.1.to_owned(),
                std::fs::read(
                    root.join(format!("docs/ir/contracts/{}.txt", pin.1.replace(':', "-"))),
                )
                .unwrap(),
            ));
        }
        for kind in ["provider", "source"] {
            let v = &corpus["resources"][kind];
            let raw = serde_json::to_vec(v).unwrap();
            owned.push((v["id"].as_str().unwrap().to_owned(), digest(&raw), raw));
        }
        let supplied: Vec<_> = owned
            .iter()
            .map(|(id, revision, bytes)| Resource {
                id,
                revision,
                bytes,
            })
            .collect();
        let e = DefinitionEnvelope::from_parts_versioned(
            "0.2.0",
            corpus["definition"]["body"].clone(),
            json!({}),
        )
        .unwrap();
        let r = choreoform_semantic_check::check(&e, &supplied).unwrap();
        assert!(
            static_output(&r, json!({"executable":false}))
                .unwrap()
                .len()
                < 4096
        );
        let base = json!({"padding":"x".repeat(super::super::MAX_BYTES-100)});
        assert!(serde_json::to_vec(&base).unwrap().len() < super::super::MAX_BYTES);
        assert_eq!(static_output(&r, base).unwrap_err(), "report-budget");
    }
}
