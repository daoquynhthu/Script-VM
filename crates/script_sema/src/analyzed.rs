//! Stable frontend analysis product for T-P2 consumption (WP-L04).
//!
//! Spec: Phase 1 frontend output contract — analyzed module + diagnostics.
//! Does not expose public bytecode or RuntimePlan.

use script_lex::{line_col_at, LineCol, Span};
use script_parse::{parse_module, Module, ParseError};

use crate::analyze::{analyze_module, SemaResult};
use crate::binding::Binding;
use crate::error::SemaError;
use crate::CheckError;

/// Which frontend stage produced a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagStage {
    Lex,
    Parse,
    Sema,
}

/// Unified frontend diagnostic (source-oriented).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontendDiagnostic {
    pub stage: DiagStage,
    pub message: String,
    pub span: Span,
    pub start: LineCol,
    pub end: LineCol,
}

impl FrontendDiagnostic {
    #[must_use]
    pub fn new(stage: DiagStage, message: impl Into<String>, span: Span, source: &str) -> Self {
        Self {
            stage,
            message: message.into(),
            span,
            start: line_col_at(source, span.start),
            end: line_col_at(source, span.end),
        }
    }
}

impl std::fmt::Display for FrontendDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {}:{}-{}:{}: {}",
            self.stage,
            self.start.line,
            self.start.column,
            self.end.line,
            self.end.column,
            self.message
        )
    }
}

/// Result of a full frontend check: AST (if parse ok) + bindings + diagnostics.
#[derive(Debug, Clone)]
pub struct AnalyzedModule {
    /// Source text (owned for diagnostic line/col stability).
    pub source: String,
    /// Parsed module when parse succeeded.
    pub module: Option<Module>,
    /// Module-level bindings when sema ran (may be partial if errors).
    pub bindings: Vec<Binding>,
    /// All frontend diagnostics (parse and/or sema).
    pub diagnostics: Vec<FrontendDiagnostic>,
}

impl AnalyzedModule {
    #[must_use]
    pub fn ok(&self) -> bool {
        self.diagnostics.is_empty() && self.module.is_some()
    }

    /// True if parse produced an AST (sema may still have failed).
    #[must_use]
    pub fn has_ast(&self) -> bool {
        self.module.is_some()
    }
}

/// Parse + semantic analysis with unified diagnostics (primary T-P1 export API).
#[must_use]
pub fn analyze_source(source: &str) -> AnalyzedModule {
    match parse_module(source) {
        Err(e) => AnalyzedModule {
            source: source.to_string(),
            module: None,
            bindings: Vec::new(),
            diagnostics: vec![parse_diag(source, &e)],
        },
        Ok(module) => {
            let sema: SemaResult = analyze_module(&module);
            let diagnostics: Vec<_> = sema
                .errors
                .into_iter()
                .map(|e| sema_diag(source, &e))
                .collect();
            AnalyzedModule {
                source: source.to_string(),
                module: Some(module),
                bindings: sema.module_bindings,
                diagnostics,
            }
        }
    }
}

/// Alias used by the unified plan (`check_module`).
#[must_use]
pub fn check_module(source: &str) -> AnalyzedModule {
    analyze_source(source)
}

fn parse_diag(source: &str, e: &ParseError) -> FrontendDiagnostic {
    FrontendDiagnostic::new(DiagStage::Parse, e.message.clone(), e.span, source)
}

fn sema_diag(source: &str, e: &SemaError) -> FrontendDiagnostic {
    FrontendDiagnostic::new(DiagStage::Sema, e.message.clone(), e.span, source)
}

/// Bridge older `check_source` Result API.
pub fn check_source_result(source: &str) -> Result<SemaResult, CheckError> {
    crate::analyze::check_source(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyze_ok_fib() {
        let src = r#"
def fib(n):
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(10))
"#;
        let a = analyze_source(src);
        assert!(a.ok(), "{:?}", a.diagnostics);
        assert!(a.has_ast());
        assert!(a.bindings.iter().any(|b| b.name == "fib"));
    }

    #[test]
    fn analyze_parse_error_has_line_col() {
        let a = analyze_source("let = 1\n");
        assert!(!a.ok());
        assert!(a.module.is_none());
        assert_eq!(a.diagnostics[0].stage, DiagStage::Parse);
        assert!(a.diagnostics[0].start.line >= 1);
    }

    #[test]
    fn analyze_sema_error_has_line_col() {
        let a = analyze_source("x = 1\n");
        assert!(!a.ok());
        assert!(a.has_ast());
        assert_eq!(a.diagnostics[0].stage, DiagStage::Sema);
        assert!(a.diagnostics[0].message.contains("unbound"));
        // Display includes line numbers
        let s = a.diagnostics[0].to_string();
        assert!(s.contains(':'));
    }

    #[test]
    fn check_module_alias() {
        let a = check_module("let x = 1\n");
        assert!(a.ok());
    }
}
