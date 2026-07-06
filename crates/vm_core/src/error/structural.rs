//! VM structural errors.
//!
//! Spec: `PHASE-3-RUNTIME-ERROR-REGISTRY.md` §1.2, §4

use crate::error::registry::VmStructuralErrorCode;

/// VM invariant failure; not ordinary source-level control flow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmError {
    pub code: VmStructuralErrorCode,
    pub message: String,
}

impl VmError {
    #[must_use]
    pub fn new(code: VmStructuralErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// Structural VM failures MUST NOT be catchable as ordinary language errors.
    #[must_use]
    pub const fn is_catchable_as_language_error(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structural_errors_are_not_catchable() {
        let err = VmError::new(VmStructuralErrorCode::InvalidEirError, "bad eir");
        assert!(!err.is_catchable_as_language_error());
    }
}