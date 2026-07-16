//! Phase 1 semantic analysis: binding and scope (bootstrap).
//!
//! Spec references:
//! - `PHASE-1-LANGUAGE-SPEC.md` §2.1 No Implicit New Binding by Assignment
//! - `PHASE-1-LANGUAGE-SPEC.md` §2.2 Block Scope Exists
//! - declaration / assignment immutability rules (`let` vs `const`/`def`)
//!
//! Non-goals (WP-23): full type contracts, import graph, record/enum members.

pub mod analyze;
pub mod binding;
pub mod error;

pub use analyze::{analyze_module, check_source, SemaResult};
pub use binding::{Binding, BindingKind, Scope, ScopeStack};
pub use error::SemaError;

/// Errors from combined parse + semantic check.
#[derive(Debug)]
pub enum CheckError {
    Parse(script_parse::ParseError),
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for CheckError {}
