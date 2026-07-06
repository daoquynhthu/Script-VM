//! Host error normalization shell.
//!
//! Spec: `PHASE-3-HOST-BOUNDARY-CONTRACT.md` §8

use vm_core::error::language::{ErrorObj, ErrorStore};
use vm_core::error::registry::{RuntimeErrorCode, VmStructuralErrorCode};
use vm_core::error::VmError;
use vm_core::id::ErrorHandle;

use crate::host_function::HostCallResult;

/// Normalized host failure outcomes.
#[derive(Debug, Clone, PartialEq)]
pub enum NormalizedHostError {
    Raise(ErrorHandle),
    Structural(VmError),
}

/// Normalize raw host call results to VM-controlled outcomes.
pub fn normalize_host_call_result(
    result: HostCallResult,
    store: &mut ErrorStore,
) -> Result<vm_core::value::Value, NormalizedHostError> {
    match result {
        HostCallResult::Return(value) => Ok(value),
        HostCallResult::Error(message) => {
            if message.starts_with("structural:") {
                Err(NormalizedHostError::Structural(VmError::new(
                    VmStructuralErrorCode::BackendViolationError,
                    message,
                )))
            } else {
                let handle = store.allocate(ErrorObj::new(
                    RuntimeErrorCode::InternalVMError,
                    message,
                ));
                Err(NormalizedHostError::Raise(handle))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_language_error_becomes_raise_handle() {
        let mut store = ErrorStore::new();
        let err = normalize_host_call_result(
            HostCallResult::Error("host failed".to_string()),
            &mut store,
        )
        .expect_err("error");
        match err {
            NormalizedHostError::Raise(handle) => {
                let obj = store.get(handle).expect("stored");
                assert_eq!(obj.error_code, RuntimeErrorCode::InternalVMError);
            }
            NormalizedHostError::Structural(_) => panic!("expected language raise"),
        }
    }

    #[test]
    fn host_structural_error_stays_structural() {
        let mut store = ErrorStore::new();
        let err = normalize_host_call_result(
            HostCallResult::Error("structural:bad host state".to_string()),
            &mut store,
        )
        .expect_err("error");
        assert!(matches!(err, NormalizedHostError::Structural(_)));
    }
}