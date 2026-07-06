//! Source mapping references for SIR.
//!
//! Spec: `PHASE-2-IR-SPEC.md` §7

use crate::id::{ModuleId, NodeId};

/// One-based source position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourcePosition {
    pub line: u32,
    pub column: u32,
}

impl SourcePosition {
    #[must_use]
    pub const fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

/// Source span attached to IR nodes originating from source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    pub module: ModuleId,
    pub start: SourcePosition,
    pub end: SourcePosition,
}

impl SourceSpan {
    #[must_use]
    pub const fn new(module: ModuleId, start: SourcePosition, end: SourcePosition) -> Self {
        Self {
            module,
            start,
            end,
        }
    }
}

/// Whether a node was introduced during lowering rather than parsed from source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceOrigin {
    pub span: Option<SourceSpan>,
    pub synthetic: bool,
    pub origin_node: Option<NodeId>,
}

impl SourceOrigin {
    #[must_use]
    pub const fn from_source(span: SourceSpan) -> Self {
        Self {
            span: Some(span),
            synthetic: false,
            origin_node: None,
        }
    }

    #[must_use]
    pub const fn synthetic(origin_node: Option<NodeId>) -> Self {
        Self {
            span: None,
            synthetic: true,
            origin_node,
        }
    }
}