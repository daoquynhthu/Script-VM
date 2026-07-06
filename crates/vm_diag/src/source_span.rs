//! Diagnostic source-span attachment.
//!
//! Spec: `PHASE-2-IR-SPEC.md` §7, `PHASE-3-RUNTIME-ERROR-REGISTRY.md` §6

/// Stable identifier for a source span in the diagnostic/runtime mapping layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceSpanId(pub u32);

impl SourceSpanId {
    pub const INVALID: Self = Self(u32::MAX);

    #[must_use]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    #[must_use]
    pub const fn raw(self) -> u32 {
        self.0
    }

    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 != u32::MAX
    }
}

/// File or module identifier for diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceFileId(pub u32);

impl SourceFileId {
    pub const INVALID: Self = Self(u32::MAX);

    #[must_use]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }
}

/// One-based line/column position for human-readable diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    pub line: u32,
    pub column: u32,
}

impl SourceLocation {
    #[must_use]
    pub const fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

/// Span used by validator and runtime diagnostic reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiagnosticSpan {
    pub file: SourceFileId,
    pub start: SourceLocation,
    pub end: SourceLocation,
}

impl DiagnosticSpan {
    #[must_use]
    pub const fn new(file: SourceFileId, start: SourceLocation, end: SourceLocation) -> Self {
        Self { file, start, end }
    }
}