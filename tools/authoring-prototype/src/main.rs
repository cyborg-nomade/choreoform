// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use std::io::{Read, Write};
fn read(path: &str, max: usize) -> Result<Vec<u8>, String> {
    let mut b = Vec::new();
    std::fs::File::open(path)
        .map_err(|_| "read")?
        .take(max as u64 + 1)
        .read_to_end(&mut b)
        .map_err(|_| "read")?;
    if b.len() > max {
        return Err("size".into());
    }
    Ok(b)
}
fn run() -> Result<String, String> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    let n = match a.first().map(String::as_str) {
        Some("canonical" | "view") => 4,
        Some("change") => 5,
        _ => return Err("arguments".into()),
    };
    if a.len() != n {
        return Err("arguments".into());
    }
    let d = read(&a[1], 16 * 1024 * 1024)?;
    let h = read(&a[n - 2], 4 * 1024 * 1024)?;
    let b = read(&a[n - 1], 8 * 1024 * 1024)?;
    match a[0].as_str() {
        "canonical" => choreoform_authoring_prototype::canonical_document(&d, &h, &b),
        "view" => choreoform_authoring_prototype::view_document(&d, &h, &b),
        "change" => {
            let c = read(&a[2], 1024 * 1024)?;
            choreoform_authoring_prototype::change_document(&d, &c, &h, &b)
        }
        _ => unreachable!(),
    }
}
fn main() -> std::process::ExitCode {
    match run() {
        Ok(s) => match std::io::stdout()
            .lock()
            .write_all(s.as_bytes())
            .and_then(|()| std::io::stdout().flush())
        {
            Ok(()) => std::process::ExitCode::SUCCESS,
            Err(_) => {
                eprintln!("write");
                std::process::ExitCode::FAILURE
            }
        },
        Err(e) => {
            eprintln!("{e}");
            std::process::ExitCode::FAILURE
        }
    }
}
