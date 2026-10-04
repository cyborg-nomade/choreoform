// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

//! Bounded Candidate A frontend. Produces candidate IR, never semantic admission.
//! No filesystem/network, implicit ID allocation, IO or execution in the library.

mod ledger;
mod lower;
mod syntax;
pub use ledger::Ledger;
pub use lower::{Candidate, Integrity, Package};
pub use syntax::{Syntax, Tree, parse, parse_fragment};

use std::ops::Range;

pub const MAX_BYTES: usize = 1024 * 1024;
pub const MAX_DEPTH: usize = 64;
pub const MAX_TOKENS: usize = 4096;
pub const MAX_WORK: usize = 100_000;

/// Frontend-local codes and half-open UTF-8 byte spans, not validator diagnostics.
/// Messages never reproduce a caller's source literal or companion value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub span: Range<usize>,
}

pub type Result<T> = std::result::Result<T, Diagnostic>;

pub(crate) fn error(code: &'static str, span: Range<usize>) -> Diagnostic {
    Diagnostic { code, span }
}
