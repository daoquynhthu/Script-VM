//! Lexical errors.
//!
//! Spec: `PHASE-1-LANGUAGE-SPEC.md` §3–§6

use crate::span::Span;

/// Errors raised during Phase 1 lexical analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexError {
    /// Ill-formed UTF-8 (caller should only pass `str`; retained for API completeness).
    InvalidUtf8 { span: Span },
    /// Tab character in leading indentation.
    TabInIndentation { span: Span },
    /// Dedent level does not match a prior indent level.
    IndentationError { span: Span },
    /// Integer form rejected by grammar (leading zero, trailing/double underscore, …).
    InvalidInteger { span: Span, message: String },
    /// Float form rejected by grammar.
    InvalidFloat { span: Span, message: String },
    /// Unterminated string or bad escape.
    InvalidString { span: Span, message: String },
    /// Unexpected character.
    UnexpectedChar { span: Span, ch: char },
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUtf8 { .. } => write!(f, "ill-formed UTF-8 in source"),
            Self::TabInIndentation { .. } => {
                write!(f, "tabs are not allowed in leading indentation")
            }
            Self::IndentationError { .. } => write!(f, "inconsistent indentation"),
            Self::InvalidInteger { message, .. } => write!(f, "invalid integer: {message}"),
            Self::InvalidFloat { message, .. } => write!(f, "invalid float: {message}"),
            Self::InvalidString { message, .. } => write!(f, "invalid string: {message}"),
            Self::UnexpectedChar { ch, .. } => write!(f, "unexpected character {ch:?}"),
        }
    }
}

impl std::error::Error for LexError {}

impl LexError {
    #[must_use]
    pub fn span(&self) -> Span {
        match self {
            Self::InvalidUtf8 { span }
            | Self::TabInIndentation { span }
            | Self::IndentationError { span }
            | Self::InvalidInteger { span, .. }
            | Self::InvalidFloat { span, .. }
            | Self::InvalidString { span, .. }
            | Self::UnexpectedChar { span, .. } => *span,
        }
    }
}
