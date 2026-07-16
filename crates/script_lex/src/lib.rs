//! Phase 1 lexical analysis for the Script language.
//!
//! Spec references:
//! - `PHASE-1-LANGUAGE-SPEC.md` §3 Source Text
//! - `PHASE-1-LANGUAGE-SPEC.md` §4 Indentation and Blocks
//! - `PHASE-1-LANGUAGE-SPEC.md` §5 Lexical Tokens
//! - `PHASE-1-LANGUAGE-SPEC.md` §6 Literals (lexical forms)
//!
//! Non-goals (WP-21): parser, AST, SIR, VM execution.

pub mod error;
pub mod lexer;
pub mod span;
pub mod token;

pub use error::LexError;
pub use lexer::lex;
pub use span::{line_col_at, LineCol, Span};
pub use token::{Keyword, Token, TokenKind};
