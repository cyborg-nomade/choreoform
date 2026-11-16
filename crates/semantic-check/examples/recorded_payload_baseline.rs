// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Existing native literal API only. NOT a recorded-evidence reader or event proof.
use choreoform_ir_probe_core::{Resource, digest, transport};
use choreoform_semantic_check::Resources;
use serde_json::Value;
fn main() {
    let bytes = include_bytes!("../../../docs/evidence/recorded-0.1/claim.types.json");
    let revision = digest(bytes);
    let supplied = [Resource {
        id: "urn:choreoform:examples:recorded-claims:0.1.0",
        revision: &revision,
        bytes,
    }];
    let mut resources = Resources::new(&supplied).expect("fixed baseline registry");
    let example: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/evidence/recorded-0.1/example.json"
    ))
    .expect("authored example JSON");
    let mut checked = 0;
    for row in example["records"]
        .as_object()
        .expect("fixture records")
        .values()
    {
        let payload = &row["claim"]["payload"];
        let ty = resources
            .type_descriptor(&payload["type"])
            .expect("existing descriptor judgment");
        resources
            .literal(&ty, &payload["value"])
            .expect("existing literal judgment");
        checked += 1;
    }
    let invalid: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/evidence/recorded-0.1/untyped-value.json"
    ))
    .expect("authored invalid literal JSON");
    let payload = &invalid["records"]["hypothesis"]["claim"]["payload"];
    let ty = resources
        .type_descriptor(&payload["type"])
        .expect("same descriptor");
    assert!(resources.literal(&ty, &payload["value"]).is_err());
    let invalid: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/evidence/recorded-0.1/unsupported-type.json"
    ))
    .expect("authored unsupported type JSON");
    assert!(
        resources
            .type_descriptor(&invalid["records"]["hypothesis"]["claim"]["payload"]["type"])
            .is_err()
    );
    let limits = transport::DecodeLimits {
        max_bytes: 1024 * 1024,
        max_containers: 64,
        max_items: 262144,
        reject_negative_zero: true,
    };
    let malformed: [&[u8]; 8] = [
        br#"{"a":0,"\u0061":1}"#,
        br#"{"n":1.0}"#,
        br#"{"n":1e0}"#,
        br#"{"n":-0}"#,
        br#"{"n":9007199254740992}"#,
        b"\xef\xbb\xbf{}",
        br#"{"x":"\ud800"}"#,
        b"{} {}",
    ];
    for raw in malformed {
        assert!(transport::decode_with_limits(raw, limits).is_err());
    }
    // Transport alone accepts non-artifact JSON, demonstrating the stage boundary.
    assert!(transport::decode_with_limits(b"[]", limits).is_ok());
    println!(
        "Existing native payload literals: {checked} valid, 2 refused; no whole evidence reader, authentication or accepted event."
    );
    println!("Existing native raw transport: 8 malformed inputs refused; no artifact admission.");
}
