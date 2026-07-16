//! Source positions for Phase 1 frontend diagnostics.
//!
//! Spec: `PHASE-1-LANGUAGE-SPEC.md` §3 (source text model)

/// Zero-based UTF-8 byte offsets into a source buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    #[must_use]
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    #[must_use]
    pub const fn empty(at: u32) -> Self {
        Self {
            start: at,
            end: at,
        }
    }
}

/// One-based line/column (column counts Unicode scalar values from line start).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineCol {
    pub line: u32,
    pub column: u32,
}

impl LineCol {
    #[must_use]
    pub const fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}
