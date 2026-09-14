// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

//! Informational stage timings, never CI pass/fail thresholds. Run in release.
use choreoform_ir_probe_core::{digest, semantic_bytes};
use choreoform_text_prototype::{export, parse};
use serde_json::{Value, json};
use std::{hint::black_box, time::Instant};

fn measure(mut task: impl FnMut()) -> u128 {
    task();
    let mut samples = Vec::new();
    for _ in 0..5 {
        let start = Instant::now();
        for _ in 0..50 {
            task();
        }
        samples.push(start.elapsed().as_nanos() / 50);
    }
    samples.sort_unstable();
    samples[2]
}

fn main() {
    let base: Value =
        serde_json::from_slice(include_bytes!("../../../examples/ir/01-reimbursement.json"))
            .unwrap();
    for name in ["fixture", "semantic-array"] {
        let mut document = base.clone();
        if name == "semantic-array" {
            document["body"]["policies"]
                .as_object_mut()
                .unwrap()
                .values_mut()
                .next()
                .unwrap()["body"]["profile"] = json!(vec![0; 10_000]);
            document["revision"] = digest(
                semantic_bytes(&serde_json::to_vec(&document).unwrap())
                    .unwrap()
                    .as_bytes(),
            )
            .into();
        }
        let wire = serde_json::to_vec(&document).unwrap();
        let text = export(&wire).unwrap();
        let syntax = parse(text.as_bytes()).unwrap();
        println!("{name}: source_bytes={}", text.len());
        println!(
            "parse_ns={}",
            measure(|| {
                black_box(parse(black_box(text.as_bytes())).unwrap());
            })
        );
        println!(
            "lower_ns={}",
            measure(|| {
                black_box(syntax.lower().unwrap());
            })
        );
        println!(
            "lower_then_binding_ns={}",
            measure(|| {
                black_box(syntax.lower().unwrap());
                black_box(syntax.binding().unwrap());
            })
        );
        println!(
            "export_ns={}",
            measure(|| {
                black_box(export(black_box(&wire)).unwrap());
            })
        );
        println!(
            "combined_lowering_ns={}",
            measure(|| {
                black_box(syntax.lower_with_binding().unwrap());
            })
        );
        println!(
            "parse_and_consume_ns={}",
            measure(|| {
                black_box(
                    parse(black_box(text.as_bytes()))
                        .unwrap()
                        .into_lowered()
                        .unwrap(),
                );
            })
        );
    }
}
