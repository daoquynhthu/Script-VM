//! Source-level raise validation.
//!
//! Spec: `PHASE-3-RUNTIME-ERROR-REGISTRY.md` §4

use crate::error::language::{ErrorObj, ErrorStore};
use crate::error::registry::RuntimeErrorCode;
use crate::id::ErrorHandle;
use crate::value::Value;

/// Outcome of validating a source-level raise operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaiseValidation {
    Accepted(ErrorHandle),
    RejectedTypeError,
}

/// A source-level `raise` MUST raise only language Error values.
pub fn validate_raise_operand(value: &Value, store: &ErrorStore) -> RaiseValidation {
    match value {
        Value::Error(handle) if store.get(*handle).is_some() => {
            RaiseValidation::Accepted(*handle)
        }
        _ => RaiseValidation::RejectedTypeError,
    }
}

/// Materialize the TypeError required when a non-Error value is raised.
pub fn type_error_for_invalid_raise(store: &mut ErrorStore) -> ErrorHandle {
    store.allocate(ErrorObj::new(
        RuntimeErrorCode::TypeError,
        "exceptions must derive from Error",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::language::ErrorObj;

    #[test]
    fn non_error_raise_is_rejected_as_type_error() {
        let mut store = ErrorStore::new();
        let int_value = Value::Int(42);
        assert_eq!(
            validate_raise_operand(&int_value, &store),
            RaiseValidation::RejectedTypeError
        );

        let type_err = type_error_for_invalid_raise(&mut store);
        let produced = store.get(type_err).expect("type error");
        assert_eq!(produced.error_code, RuntimeErrorCode::TypeError);
    }

    #[test]
    fn error_value_raise_is_accepted() {
        let mut store = ErrorStore::new();
        let handle = store.allocate(ErrorObj::new(RuntimeErrorCode::AssertionError, "failed"));
        let value = Value::Error(handle);
        assert_eq!(
            validate_raise_operand(&value, &store),
            RaiseValidation::Accepted(handle)
        );
    }

    #[test]
    fn stale_error_handle_is_rejected() {
        let store = ErrorStore::new();
        let stale = Value::Error(ErrorHandle::new(99));
        assert_eq!(
            validate_raise_operand(&stale, &store),
            RaiseValidation::RejectedTypeError
        );
    }
}