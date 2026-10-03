// SPDX-License-Identifier: GPL-3.0-only

//! Source locations and the contract's rejection classes (grammar §6):
//! a [`Diag`] names where a guest program failed admission and which of
//! the classes it belongs to, so a conformance report can tell invalid
//! syntax, invalid typing/ownership, and host-valid-but-excluded forms
//! apart before any effect runs.

/// A source location: one-based line and column of the first character
/// and the span's length in characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    /// One-based line number.
    pub line: u32,
    /// One-based column number of the span's first character.
    pub col: u32,
    /// Span length in characters.
    pub len: u32,
}

impl Span {
    /// Builds a span at the given position.
    #[must_use]
    pub const fn new(line: u32, col: u32, len: u32) -> Self {
        Self { line, col, len }
    }

    /// The span of a single character at this span's start.
    #[must_use]
    pub const fn point(self) -> Self {
        Self {
            line: self.line,
            col: self.col,
            len: 1,
        }
    }
}

/// The rejection classes of grammar §6. A successful admission has no
/// diagnostic at all; every failure is exactly one of these.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagKind {
    /// Invalid Rust syntax: the pinned parser rejects it. No evaluator
    /// is entered.
    Syntax,
    /// Invalid Rust typing or ownership: unresolved or incompatible
    /// types, immutable assignment, overlapping borrows, dangling
    /// references, invalid closure capture.
    Type,
    /// Ownership or borrow violation: a move conflict, an exclusive
    /// borrow clash, or a capture that Rust forbids.
    Ownership,
    /// Host-valid but excluded syntax, type, or effect: report the
    /// location, never a fabricated type error and never a runtime
    /// failure.
    Unsupported,
}

/// One admission failure: its class, its location, and the stable
/// message the teaching engine reports. Compiler diagnostic wording is
/// never compared; this message is the edition's own contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diag {
    /// The rejection class.
    pub kind: DiagKind,
    /// Where the rejection was found.
    pub span: Span,
    /// The edition's stable message.
    pub message: String,
}

impl Diag {
    /// Builds a diagnostic.
    #[must_use]
    pub fn new(kind: DiagKind, span: Span, message: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            message: message.into(),
        }
    }

    /// Builds a syntax diagnostic.
    #[must_use]
    pub fn syntax(span: Span, message: impl Into<String>) -> Self {
        Self::new(DiagKind::Syntax, span, message)
    }

    /// Builds a typing diagnostic.
    #[must_use]
    pub fn type_error(span: Span, message: impl Into<String>) -> Self {
        Self::new(DiagKind::Type, span, message)
    }

    /// Builds an ownership diagnostic.
    #[must_use]
    pub fn ownership(span: Span, message: impl Into<String>) -> Self {
        Self::new(DiagKind::Ownership, span, message)
    }

    /// Builds an unsupported-construct diagnostic.
    #[must_use]
    pub fn unsupported(span: Span, message: impl Into<String>) -> Self {
        Self::new(DiagKind::Unsupported, span, message)
    }
}
