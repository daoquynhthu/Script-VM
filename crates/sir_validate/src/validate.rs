//! SIR validation entry points.
//!
//! Spec: `PHASE-3-VALIDATION-MATRIX.md`, `PHASE-2-IR-SPEC.md`

use vm_diag::diagnostic::Diagnostic;

/// Validation outcome for SIR units.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub diagnostics: Vec<Diagnostic>,
}

impl ValidationResult {
    #[must_use]
    pub fn ok() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.diagnostics
            .iter()
            .all(|d| d.severity != vm_diag::diagnostic::DiagnosticSeverity::Error)
    }
}

/// Placeholder SIR validation entry (WP-05/WP-06 integration).
pub fn validate_sir_unit() -> ValidationResult {
    ValidationResult::ok()
}