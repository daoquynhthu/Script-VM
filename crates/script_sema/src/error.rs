//! Semantic analysis errors (Phase 1 binding/scope bootstrap).
//!
//! Spec: `PHASE-1-LANGUAGE-SPEC.md` §2.1, §2.2, declaration/assignment rules

use script_lex::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemaError {
    pub message: String,
    pub span: Span,
}

impl SemaError {
    #[must_use]
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

impl std::fmt::Display for SemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "semantic error: {}", self.message)
    }
}

impl std::error::Error for SemaError {}
