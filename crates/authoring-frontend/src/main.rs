// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

//! Native test adapter; explicit source/companion paths, no discovery or writes.
use choreoform_authoring_frontend::{MAX_BYTES, Package, parse_fragment};
use choreoform_ir_probe_core::{Resource, SUPPORTED_CONTRACTS};
use std::io::{Read, Write};

const CORE_ID: &str = "urn:choreoform:contracts:core-profile:0.1.0";
const CORE_REV: &str = "sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58";
fn read(input: impl Read) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    input
        .take(MAX_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "input-read".to_owned())?;
    if bytes.len() > MAX_BYTES {
        return Err("size".into());
    }
    Ok(bytes)
}
fn run() -> Result<String, String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() == 2 && args[0] == "parse-fragment" {
        let raw = read(std::io::stdin().lock())?;
        let syntax =
            parse_fragment(&raw, &args[1]).map_err(|e| format!("{} at {:?}", e.code, e.span))?;
        return Ok(
            serde_json::json!({"rule":syntax.tree().rule,"span":syntax.span(syntax.tree())})
                .to_string(),
        );
    }
    if args.len() != 3 || !matches!(args[0].as_str(), "lower" | "integrity" | "format") {
        return Err("usage: choreoform-authoring-frontend <lower|integrity|format> SOURCE COMPANION; lower emits UNVALIDATED candidate IR; or parse-fragment RULE < stdin".into());
    }
    let source = read(std::fs::File::open(&args[1]).map_err(|_| "source-read".to_owned())?)?;
    let bindings = read(std::fs::File::open(&args[2]).map_err(|_| "companion-read".to_owned())?)?;
    let package =
        Package::parse(&source, &bindings).map_err(|e| format!("{} at {:?}", e.code, e.span))?;
    if args[0] == "integrity" {
        let i = package.integrity();
        return Ok(
            serde_json::json!({"source":i.source,"bindings":i.bindings,"package":i.package})
                .to_string(),
        );
    }
    let resources = [
        Resource {
            id: SUPPORTED_CONTRACTS[0].0,
            revision: SUPPORTED_CONTRACTS[0].1,
            bytes: include_bytes!(
                "../../../docs/ir/contracts/sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt"
            ),
        },
        Resource {
            id: CORE_ID,
            revision: CORE_REV,
            bytes: include_bytes!(
                "../../../docs/ir/contracts/sha256-26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58.txt"
            ),
        },
    ];
    let candidate = package
        .lower(&resources)
        .map_err(|e| format!("{} at {:?}", e.code, e.span))?;
    if args[0] == "format" {
        let (source, companion) = package.formatted().map_err(|e| e.code.to_owned())?;
        return Ok(serde_json::json!({"source":source,"companion":companion,"semanticRevision":candidate.document()["revision"]}).to_string());
    }
    serde_json::to_string(candidate.document()).map_err(|_| "output-json".into())
}
fn main() -> std::process::ExitCode {
    match run() {
        Ok(output) => match std::io::stdout()
            .lock()
            .write_all(output.as_bytes())
            .and_then(|()| std::io::stdout().flush())
        {
            Ok(()) => std::process::ExitCode::SUCCESS,
            Err(_) => {
                eprintln!("output-write");
                std::process::ExitCode::FAILURE
            }
        },
        Err(message) => {
            eprintln!("{message}");
            std::process::ExitCode::FAILURE
        }
    }
}
