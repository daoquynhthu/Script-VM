//! Pending-control combination and finally override rules.
//!
//! Spec: `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md` §6–§8

use vm_core::error::language::ErrorStore;
use vm_core::id::{ControlRegionId, ErrorHandle};
use vm_core::value::Value;

use crate::control::PendingControl;

/// Result of a defer, resource close, or finally step.
#[derive(Debug, Clone, PartialEq)]
pub enum CleanupStepResult {
    Normal,
    Return(Option<Value>),
    Break(ControlRegionId),
    Continue(ControlRegionId),
    Raise(ErrorHandle),
}

impl CleanupStepResult {
    #[must_use]
    pub fn is_normal(&self) -> bool {
        matches!(self, Self::Normal)
    }

    #[must_use]
    pub fn into_pending_control(self) -> Option<PendingControl> {
        match self {
            Self::Normal => None,
            Self::Return(value) => Some(PendingControl::Return(value)),
            Self::Break(region) => Some(PendingControl::Break(region)),
            Self::Continue(region) => Some(PendingControl::Continue(region)),
            Self::Raise(handle) => Some(PendingControl::Raise(handle)),
        }
    }
}

/// Combine a cleanup step result with existing pending control (defer/resource paths).
pub fn combine_cleanup_result(
    pending: PendingControl,
    step: CleanupStepResult,
    store: &mut ErrorStore,
) -> PendingControl {
    match step {
        CleanupStepResult::Normal => pending,
        CleanupStepResult::Raise(new_handle) => match pending {
            PendingControl::Raise(primary) => {
                attach_suppressed_to_primary(primary, new_handle, store);
                PendingControl::Raise(primary)
            }
            _ => PendingControl::Raise(new_handle),
        },
        non_raise => {
            if let Some(replacement) = non_raise.into_pending_control() {
                replacement
            } else {
                pending
            }
        }
    }
}

/// Finally override rule: non-normal finally result replaces pending control.
pub fn finally_override(
    pending: PendingControl,
    finally_result: CleanupStepResult,
    store: &mut ErrorStore,
) -> PendingControl {
    match finally_result {
        CleanupStepResult::Normal => pending,
        CleanupStepResult::Raise(new_handle) => {
            if let PendingControl::Raise(old) = pending {
                attach_suppressed_to_primary(new_handle, old, store);
            }
            PendingControl::Raise(new_handle)
        }
        replacement => replacement
            .into_pending_control()
            .unwrap_or(pending),
    }
}

/// Attach a suppressed error to the primary error object.
pub fn attach_suppressed_to_primary(
    primary: ErrorHandle,
    suppressed: ErrorHandle,
    store: &mut ErrorStore,
) {
    if let Some(obj) = store.get_mut(primary) {
        obj.attach_suppressed(suppressed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::error::language::ErrorObj;
    use vm_core::error::registry::RuntimeErrorCode;

    #[test]
    fn defer_raise_during_pending_raise_is_suppressed() {
        let mut store = ErrorStore::new();
        let primary = store.allocate(ErrorObj::new(RuntimeErrorCode::KeyError, "primary"));
        let defer_err = store.allocate(ErrorObj::new(RuntimeErrorCode::TypeError, "defer"));

        let pending = combine_cleanup_result(
            PendingControl::Raise(primary),
            CleanupStepResult::Raise(defer_err),
            &mut store,
        );

        assert_eq!(pending, PendingControl::Raise(primary));
        assert_eq!(store.get(primary).expect("primary").suppressed, vec![defer_err]);
    }

    #[test]
    fn defer_raise_during_pending_return_becomes_raise() {
        let mut store = ErrorStore::new();
        let defer_err = store.allocate(ErrorObj::new(RuntimeErrorCode::TypeError, "defer"));

        let pending = combine_cleanup_result(
            PendingControl::Return(Some(Value::Int(1))),
            CleanupStepResult::Raise(defer_err),
            &mut store,
        );

        assert_eq!(pending, PendingControl::Raise(defer_err));
    }

    #[test]
    fn finally_raise_overrides_pending_return() {
        let mut store = ErrorStore::new();
        let finally_err = store.allocate(ErrorObj::new(RuntimeErrorCode::AssertionError, "finally"));

        let pending = finally_override(
            PendingControl::Return(Some(Value::Int(1))),
            CleanupStepResult::Raise(finally_err),
            &mut store,
        );

        assert_eq!(pending, PendingControl::Raise(finally_err));
    }

    #[test]
    fn finally_return_overrides_pending_raise_with_suppression() {
        let mut store = ErrorStore::new();
        let old = store.allocate(ErrorObj::new(RuntimeErrorCode::KeyError, "old"));
        let finally_err = store.allocate(ErrorObj::new(RuntimeErrorCode::TypeError, "finally"));

        let pending = finally_override(
            PendingControl::Raise(old),
            CleanupStepResult::Raise(finally_err),
            &mut store,
        );

        assert_eq!(pending, PendingControl::Raise(finally_err));
        assert_eq!(
            store.get(finally_err).expect("finally").suppressed,
            vec![old]
        );
    }

    #[test]
    fn finally_normal_preserves_pending_break() {
        let pending = finally_override(
            PendingControl::Break(ControlRegionId::new(2)),
            CleanupStepResult::Normal,
            &mut ErrorStore::new(),
        );
        assert_eq!(pending, PendingControl::Break(ControlRegionId::new(2)));
    }

    #[test]
    fn finally_continue_overrides_pending_break() {
        let pending = finally_override(
            PendingControl::Break(ControlRegionId::new(2)),
            CleanupStepResult::Continue(ControlRegionId::new(2)),
            &mut ErrorStore::new(),
        );
        assert_eq!(pending, PendingControl::Continue(ControlRegionId::new(2)));
    }
}