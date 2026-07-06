//! Interpreter execution errors.

use vm_core::error::registry::{RuntimeErrorCode, VmStructuralErrorCode};
use vm_core::error::VmError;
use vm_runtime::runtime_error::RuntimeFailure;

/// Failure during interpreter execution.
#[derive(Debug, Clone, PartialEq)]
pub enum InterpreterError {
    Language(RuntimeErrorCode, String),
    Structural(VmStructuralErrorCode, String),
    VmError(VmError),
}

impl InterpreterError {
    #[must_use]
    pub fn language(code: RuntimeErrorCode, message: impl Into<String>) -> Self {
        Self::Language(code, message.into())
    }

    #[must_use]
    pub fn structural(code: VmStructuralErrorCode, message: impl Into<String>) -> Self {
        Self::Structural(code, message.into())
    }

    #[must_use]
    pub fn vm_error(err: VmError) -> Self {
        Self::VmError(err)
    }

    pub fn from_runtime_failure(err: RuntimeFailure) -> Self {
        match err {
            RuntimeFailure::Language(code) => {
                Self::Language(code, code.name().to_string())
            }
            RuntimeFailure::Structural(vm_err) => Self::VmError(vm_err),
        }
    }
}