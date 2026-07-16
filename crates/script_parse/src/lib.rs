//! Phase 1 parser and AST (bootstrap subset).
//!
//! Spec: `PHASE-1-LANGUAGE-SPEC.md` (grammar as implementation guide; AST not normative).
//! Depends on `script_lex` (WP-21).

pub mod ast;
pub mod error;
pub mod parser;

pub use ast::{
    AugOp, BinaryOp, Block, CallArg, CatchClause, Decl, Expr, ImportName, Item, Module, RecordField,
    Stmt, UnaryOp,
};
pub use error::ParseError;
pub use parser::parse_module;
