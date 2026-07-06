//! Diagnostic reporting types.
//!
//! Spec: `PHASE-3-RUNTIME-ERROR-REGISTRY.md` §1.3, §6

use crate::source_span::{DiagnosticSpan, SourceSpanId};

/// Severity for pre-execution validation and compile-time diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Note,
}

/// Diagnostic produced before or around execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub code: String,
    pub message: String,
    pub span: Option<DiagnosticSpan>,
}

impl Diagnostic {
    #[must_use]
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            code: code.into(),
            message: message.into(),
            span: None,
        }
    }

    #[must_use]
    pub fn with_span(mut self, span: DiagnosticSpan) -> Self {
        self.span = Some(span);
        self
    }
}

/// Validation/compile-time error layer (DiagnosticError).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticError {
    pub diagnostic: Diagnostic,
}

impl DiagnosticError {
    #[must_use]
    pub fn new(diagnostic: Diagnostic) -> Self {
        Self { diagnostic }
    }

    #[must_use]
    pub fn prevents_execution(&self) -> bool {
        self.diagnostic.severity == DiagnosticSeverity::Error
    }
}

/// Source-oriented stack trace attachment for language errors.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StackTrace {
    pub frames: Vec<StackFrame>,
    pub hidden_helper_frames: bool,
}

/// One frame in a source-oriented stack trace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackFrame {
    pub label: String,
    pub span: Option<SourceSpanId>,
}

impl StackTrace {
    #[must_use]
    pub fn with_frame(mut self, label: impl Into<String>, span: Option<SourceSpanId>) -> Self {
        self.frames.push(StackFrame {
            label: label.into(),
            span,
        });
        self
    }
}

/// Execution error source mapping scaffold.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ErrorSourceMapping {
    pub source_span: Option<SourceSpanId>,
    pub sir_node_id: Option<u32>,
    pub eir_location: Option<String>,
    pub helper_context: Option<String>,
}