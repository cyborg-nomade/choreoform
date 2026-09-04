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
        write_output(&mut io::stdout().lock(), value.canonical.as_bytes())
            .map_err(|e| e.to_string())?;
        return Ok(());
    }
    Err("usage: choreoform-portability suite [report.json] | inspect < definition.json".into())
}

/// Preserve exact bytes and report buffered-output errors before CLI success.
fn write_output(output: &mut impl Write, bytes: &[u8]) -> io::Result<()> {
    output.write_all(bytes)?;
    output.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestWriter {
        bytes: Vec<u8>,
        fail_write: bool,
        fail_flush: bool,
        flushed: bool,
    }

    impl Write for TestWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.fail_write {
                return Err(io::Error::new(io::ErrorKind::BrokenPipe, "write failed"));
            }
            // Exercise write_all's handling of short writes as well.
            let count = bytes.len().min(2);
            self.bytes.extend_from_slice(&bytes[..count]);
            Ok(count)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.flushed = true;
            if self.fail_flush {
                return Err(io::Error::new(io::ErrorKind::BrokenPipe, "flush failed"));
            }
            Ok(())
        }
    }

    #[test]
    fn output_preserves_bytes_and_flushes() {
        let mut writer = TestWriter {
            bytes: Vec::new(),
            fail_write: false,
            fail_flush: false,
            flushed: false,
        };
        write_output(&mut writer, b"canonical").unwrap();
        assert_eq!(writer.bytes, b"canonical");
        assert!(writer.flushed);
    }

    #[test]
    fn output_propagates_write_failure() {
        let mut writer = TestWriter {
            bytes: Vec::new(),
            fail_write: true,
            fail_flush: false,
            flushed: false,
        };
        assert_eq!(
            write_output(&mut writer, b"canonical").unwrap_err().kind(),
            io::ErrorKind::BrokenPipe
        );
        assert!(writer.bytes.is_empty());
        assert!(!writer.flushed);
    }

    #[test]
    fn output_propagates_flush_failure_after_successful_write() {
        let mut writer = TestWriter {
            bytes: Vec::new(),
            fail_write: false,
            fail_flush: true,
            flushed: false,
        };
        assert_eq!(
            write_output(&mut writer, b"canonical")
                .unwrap_err()
                .to_string(),
            "flush failed"
        );
        assert_eq!(writer.bytes, b"canonical");
        assert!(writer.flushed);
    }
}
