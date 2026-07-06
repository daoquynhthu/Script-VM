//! Runtime error layering and registry.
//!
//! Spec: `PHASE-3-RUNTIME-ERROR-REGISTRY.md`

pub mod language;
pub mod raise;
pub mod registry;
pub mod structural;

pub use language::{ErrorObj, ErrorStore};
pub use raise::{type_error_for_invalid_raise, validate_raise_operand, RaiseValidation};
pub use registry::{ErrorLayer, RuntimeErrorCode, VmStructuralErrorCode};
pub use structural::VmError;