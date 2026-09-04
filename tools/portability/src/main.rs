// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

use choreoform_ir_probe_core as core;
use std::{
    env, fs,
    io::{self, Read, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    match execute() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn execute() -> Result<(), String> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("suite") && args.len() <= 2 {
        let report = choreoform_portability::run_suite();
        let parsed: serde_json::Value = serde_json::from_str(&report).map_err(|e| e.to_string())?;
        if let Some(path) = args.get(1) {
            fs::write(path, &report).map_err(|e| e.to_string())?;
        } else {
            println!("{report}");
        }
        if parsed["passed"] != true {
            return Err("portability suite failed".into());
        }
        return Ok(());
    }
    if args.as_slice() == ["inspect"] {
        let mut bytes = Vec::new();
        io::stdin()
            .take((core::transport::MAX_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        let value = core::inspect(&bytes, &choreoform_portability::resources())
            .map_err(|e| e.category().to_string())?;
        io::stdout()
            .write_all(value.canonical.as_bytes())
            .map_err(|e| e.to_string())?;
        return Ok(());
    }
    Err("usage: choreoform-portability suite [report.json] | inspect < definition.json".into())
}
