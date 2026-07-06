//! Runtime operation failure types.
//!
//! Spec: `PHASE-3-RUNTIME-ERROR-REGISTRY.md`

use vm_core::error::registry::{RuntimeErrorCode, VmStructuralErrorCode};
use vm_core::error::VmError;

/// Unified failure for runtime substrate operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeFailure {
    Language(RuntimeErrorCode),
    Structural(VmError),
}

impl RuntimeFailure {
    #[must_use]
    pub fn language(code: RuntimeErrorCode) -> Self {
        Self::Language(code)
    }

    #[must_use]
    pub fn structural(code: VmStructuralErrorCode, message: impl Into<String>) -> Self {
        Self::Structural(VmError::new(code, message))
    }
}

pub type RuntimeResult<T> = Result<T, RuntimeFailure>;