// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_semantic_check::Diagnostic;

fn diagnostic(path: &str) -> Diagnostic {
    Diagnostic {
        code: "illustrative-location",
        path: path.into(),
    }
}

#[test]
fn actual_static_and_control_errors_locate_original_declarations() {
    let source = REVIEW_SOURCE.replace("Complete when (true).", "Complete when (7).");
    let c = Package::parse(source.as_bytes(), REVIEW_LEDGER)
        .unwrap()
        .lower(&resources())
        .unwrap();
    let e = c.check_static(&resources()).unwrap_err();
    assert_eq!(e.code, "predicate-type");
    let span = c.source_span(&e).unwrap();
    assert!(source[span.clone()].starts_with("Rule for work \"Review work\"."));
    assert!(source[span].contains("Complete when (7)."));
    let source = REVIEW_SOURCE.replace(
        "Initially (\"Synthetic request\").",
        "Initially unavailable.",
    );
    let mut l: Value = serde_json::from_slice(REVIEW_LEDGER).unwrap();
    l["generated"]
        .as_array_mut()
        .unwrap()
        .retain(|r| !(r["owner"] == "review_request" && r["role"] == "initial"));
    let c = Package::parse(source.as_bytes(), &bytes(&l))
        .unwrap()
        .lower(&resources())
        .unwrap();
    let e = c.check_control(&resources()).unwrap_err();
    assert_eq!(e.code, "data-availability-unproved");
    let span = c.source_span(&e).unwrap();
    assert!(source[span].starts_with("Step \"Review\"."));
}

#[test]
fn generated_initializers_assignments_and_flows_locate_their_source_constructs() {
    for (source, raw, role, table, expected) in [
        (
            REVIEW_SOURCE,
            REVIEW_LEDGER,
            "initial",
            "expressions",
            "Synthetic request",
        ),
        (
            include_str!("../../../../docs/authoring/examples/control.choreo"),
            include_bytes!("../../../../docs/authoring/examples/control.bindings.json").as_slice(),
            "assignment",
            "expressions",
            "\"Result\" to (1)",
        ),
        (
            include_str!("../../../../docs/authoring/examples/control.choreo"),
            include_bytes!("../../../../docs/authoring/examples/control.bindings.json").as_slice(),
            "flow",
            "flows",
            "On \"first\", continue with \"Write first\".",
        ),
    ] {
        let c = Package::parse(source.as_bytes(), raw)
            .unwrap()
            .lower(&resources())
            .unwrap();
        let l: Value = serde_json::from_slice(raw).unwrap();
        let row = l["generated"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["role"] == role)
            .unwrap();
        let id = row["id"].as_str().unwrap();
        let span = c
            .source_span(&diagnostic(&format!("/body/{table}/{id}/body")))
            .unwrap();
        assert!(source[span].contains(expected), "{id}: {role}");
    }
}

#[test]
fn unknown_global_resource_and_wrong_table_paths_have_no_guessed_span() {
    let c = Package::parse(SOURCE.as_bytes(), LEDGER)
        .unwrap()
        .lower(&resources())
        .unwrap();
    for path in [
        "",
        "/body",
        "/body/nodes",
        "/body/nodes/missing",
        "/body/policies/node_done",
        "/body/flows/node_done",
        "body/nodes/node_done",
        "/annotations/node_done",
        "/body/resources/node_done",
        "/body/nodes/node_done~1other",
    ] {
        assert_eq!(c.source_span(&diagnostic(path)), None, "{path}");
    }
    // A missing field can still identify the existing containing declaration.
    assert_eq!(
        c.source_span(&diagnostic("/body/nodes/node_done/missing-field")),
        Some(c.spans()["node_done"].clone())
    );
    assert_eq!(
        c.source_span(&diagnostic("/body/nodes/node_done")),
        Some(c.spans()["node_done"].clone())
    );
}

#[test]
fn utf8_offsets_belong_to_the_original_source_and_recompute_after_formatting() {
    let source = format!(
        "# é 日本語\n{}",
        REVIEW_SOURCE.replace("Complete when (true).", "Complete when (7).")
    );
    let package = Package::parse(source.as_bytes(), REVIEW_LEDGER).unwrap();
    let c = package.lower(&resources()).unwrap();
    let e = c.check_static(&resources()).unwrap_err();
    let span = c.source_span(&e).unwrap();
    assert_eq!(
        span.start,
        source.find("Rule for work \"Review work\".").unwrap()
    );
    assert!(source.is_char_boundary(span.start) && source.is_char_boundary(span.end));
    let (formatted, companion) = package.formatted().unwrap();
    let next = Package::parse(formatted.as_bytes(), companion.as_bytes())
        .unwrap()
        .lower(&resources())
        .unwrap();
    assert_eq!(c.document(), next.document());
    let new_e = next.check_static(&resources()).unwrap_err();
    assert_eq!(e, new_e);
    let next_span = next.source_span(&new_e).unwrap();
    assert_ne!(span, next_span);
    assert!(formatted[next_span].starts_with("Rule for work \"Review work\" ."));
}
