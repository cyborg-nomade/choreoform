// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

//! Byte budget for a writer supplied by the caller; no ambient IO.
use std::io::{self, Write};

/// Refuse an entire write that would exceed the budget, before forwarding it.
/// A sink can count bytes without allocation; a Vec buffers output until success.
/// This bounds bytes written, not allocator capacity or total process memory.
pub struct LimitedWriter<W> {
    inner: W,
    remaining: usize,
    exceeded: bool,
}

impl<W> LimitedWriter<W> {
    pub fn new(inner: W, limit: usize) -> Self {
        Self {
            inner,
            remaining: limit,
            exceeded: false,
        }
    }
    pub fn exceeded(&self) -> bool {
        self.exceeded
    }
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write> Write for LimitedWriter<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.exceeded || bytes.len() > self.remaining {
            self.exceeded = true;
            return Err(io::Error::other("byte limit"));
        }
        let count = self.inner.write(bytes)?;
        self.remaining -= count;
        Ok(count)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn budget_is_checked_before_forwarding_and_stays_failed() {
        let mut writer = LimitedWriter::new(Vec::new(), 4);
        writer.write_all(b"1234").unwrap();
        assert!(writer.write_all(b"5").is_err());
        assert!(writer.exceeded());
        assert!(writer.write(b"").is_err());
        assert_eq!(writer.into_inner(), b"1234");
        let mut writer = LimitedWriter::new(Vec::new(), 4);
        assert!(writer.write_all(b"12345").is_err());
        assert!(writer.into_inner().is_empty());
    }

    struct ShortWriter {
        bytes: Vec<u8>,
        fail_write: bool,
    }
    impl Write for ShortWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.fail_write {
                return Err(io::ErrorKind::BrokenPipe.into());
            }
            let n = bytes.len().min(2);
            self.bytes.extend_from_slice(&bytes[..n]);
            Ok(n)
        }
        fn flush(&mut self) -> io::Result<()> {
            Err(io::ErrorKind::BrokenPipe.into())
        }
    }
    #[test]
    fn short_writes_and_sink_failures_are_not_size_errors() {
        let mut writer = LimitedWriter::new(
            ShortWriter {
                bytes: vec![],
                fail_write: false,
            },
            4,
        );
        writer.write_all(b"1234").unwrap();
        assert_eq!(
            writer.flush().unwrap_err().kind(),
            io::ErrorKind::BrokenPipe
        );
        assert!(!writer.exceeded());
        assert_eq!(writer.into_inner().bytes, b"1234");
        let mut writer = LimitedWriter::new(
            ShortWriter {
                bytes: vec![],
                fail_write: true,
            },
            4,
        );
        assert_eq!(
            writer.write_all(b"x").unwrap_err().kind(),
            io::ErrorKind::BrokenPipe
        );
        assert!(!writer.exceeded());
    }
}
