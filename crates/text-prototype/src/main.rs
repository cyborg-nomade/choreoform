// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

//! Native adapter. Reads one bounded artifact from stdin; writes stdout only.
use std::io::{self, Read, Write};

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 1 || !matches!(args[0].as_str(), "lower" | "export") {
        return Err("usage: choreoform-text-prototype <lower|export> < input; lower emits UNVALIDATED IR; export normalizes and does not restore comments".into());
    }
    let mut bytes = Vec::new();
    io::stdin()
        .lock()
        .take(choreoform_text_prototype::MAX_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    let output = if args[0] == "lower" {
        let syntax = choreoform_text_prototype::parse(&bytes)
            .map_err(|e| format!("{} at bytes {:?}", e.message, e.span))?;
        let document = syntax
            .lower()
            .map_err(|e| format!("{} at bytes {:?}", e.message, e.span))?;
        serde_json::to_string_pretty(&document).map_err(|e| e.to_string())?
    } else {
        choreoform_text_prototype::export(&bytes)
            .map_err(|e| format!("{} at bytes {:?}", e.message, e.span))?
    };
    let mut stdout = io::stdout().lock();
    stdout
        .write_all(output.as_bytes())
        .and_then(|()| stdout.flush())
        .map_err(|e| e.to_string())
}

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            std::process::ExitCode::FAILURE
        }
    }
}
