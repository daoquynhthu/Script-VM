//! Phase 1 semantic analysis: binding, scope, and analyzed-module API.
//!
//! Spec references:
//! - `PHASE-1-LANGUAGE-SPEC.md` §2.1–2.3, §3.3
//! - declaration / assignment immutability; export visibility
//!
//! Primary entry (WP-L04): [`check_module`] / [`analyze_source`] → [`AnalyzedModule`].

pub mod analyze;
pub mod analyzed;
pub mod binding;
pub mod error;

pub use analyze::{analyze_module, check_source, SemaResult};
pub use analyzed::{
    analyze_source, check_module, AnalyzedModule, DiagStage, FrontendDiagnostic,
};
pub use binding::{nfc, Binding, BindingKind, Scope, ScopeStack};
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
