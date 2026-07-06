//! Runtime helper dispatch shell.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-REGISTRY.md`, `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md` §15

use vm_core::error::language::ErrorStore;
use vm_core::error::registry::VmStructuralErrorCode;
use vm_core::error::VmError;
use vm_core::id::RuntimeHelperId;

use crate::unwind::{perform_unwind, UnwindContext, UnwindExecutor, UnwindOutcome};

/// Canonical helper id for `helper_perform_unwind` (registry §3 table order).
pub const HELPER_PERFORM_UNWIND_ID: RuntimeHelperId = RuntimeHelperId::new(29);

/// Dispatch a registered helper by id.
pub fn dispatch_helper(
    helper_id: RuntimeHelperId,
    unwind_ctx: &mut UnwindContext,
    executor: &mut impl UnwindExecutor,
    store: &mut ErrorStore,
) -> Result<UnwindOutcome, VmError> {
    if helper_id == HELPER_PERFORM_UNWIND_ID {
        Ok(perform_unwind(unwind_ctx, executor, store))
    } else {
        Err(VmError::new(
            VmStructuralErrorCode::InvalidHelperError,
            format!("helper {} is not dispatched in bootstrap shell", helper_id.raw()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::PendingControl;
    use crate::unwind::combine::CleanupStepResult;
    use crate::unwind::region::{ControlRegionKind, RuntimeRegionFrame};
    use vm_core::id::ControlRegionId;
    use vm_core::value::Value;

    struct NoopExecutor;

    impl UnwindExecutor for NoopExecutor {
        fn call_defer(&mut self, _: u32) -> CleanupStepResult {
            CleanupStepResult::Normal
        }

        fn close_resource(&mut self, _: u32) -> CleanupStepResult {
            CleanupStepResult::Normal
        }

        fn run_finally(&mut self, _: u32) -> CleanupStepResult {
            CleanupStepResult::Normal
        }
    }

    #[test]
    fn helper_perform_unwind_dispatches_to_perform_unwind() {
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(Some(Value::Int(1))));
        ctx.push_region(RuntimeRegionFrame::new(
            ControlRegionId::new(0),
            ControlRegionKind::Function,
        ));
        let mut store = ErrorStore::new();
        let outcome = dispatch_helper(HELPER_PERFORM_UNWIND_ID, &mut ctx, &mut NoopExecutor, &mut store)
            .expect("dispatch");
        assert_eq!(
            outcome,
            UnwindOutcome::Resolved(crate::control::VmControl::Return(Some(Value::Int(1))))
        );
    }
}