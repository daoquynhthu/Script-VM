//! Runtime helper dispatch shell.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-REGISTRY.md`, `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md` §15

use vm_core::error::language::ErrorStore;
use vm_core::error::registry::VmStructuralErrorCode;
use vm_core::error::VmError;
use vm_core::id::RuntimeHelperId;
use vm_core::value::Value;
use vm_diag::source_span::SourceSpanId;

use crate::call::callable::CallableRegistry;
use crate::call::contract::TypeContractChecker;
use crate::control::VmControl;
use crate::heap::Heap;
use crate::runtime_error::RuntimeFailure;
use crate::unwind::{perform_unwind, UnwindContext, UnwindExecutor, UnwindOutcome};
use crate::write_barrier::WriteBarrierHook;

use super::h1::{
    helper_alloc_object, helper_check_callable, helper_check_hashable,
    helper_check_type_contract, helper_construct_error, helper_write_barrier,
};

/// Canonical helper id for `helper_alloc_object` (registry §3 table order).
pub const HELPER_ALLOC_OBJECT_ID: RuntimeHelperId = RuntimeHelperId::new(0);
/// Canonical helper id for `helper_write_barrier`.
pub const HELPER_WRITE_BARRIER_ID: RuntimeHelperId = RuntimeHelperId::new(1);
/// Canonical helper id for `helper_construct_error`.
pub const HELPER_CONSTRUCT_ERROR_ID: RuntimeHelperId = RuntimeHelperId::new(2);
/// Canonical helper id for `helper_check_type_contract`.
pub const HELPER_CHECK_TYPE_CONTRACT_ID: RuntimeHelperId = RuntimeHelperId::new(6);
/// Canonical helper id for `helper_check_callable`.
pub const HELPER_CHECK_CALLABLE_ID: RuntimeHelperId = RuntimeHelperId::new(7);
/// Canonical helper id for `helper_check_hashable`.
pub const HELPER_CHECK_HASHABLE_ID: RuntimeHelperId = RuntimeHelperId::new(8);
/// Canonical helper id for `helper_perform_unwind` (registry §3 table order).
pub const HELPER_PERFORM_UNWIND_ID: RuntimeHelperId = RuntimeHelperId::new(29);

/// Normalized helper dispatch result per implementation plan §5.3.
#[derive(Debug, Clone, PartialEq)]
pub enum HelperDispatchOutcome {
    Value(Value),
    Unit,
    VmControl(VmControl),
}

/// Execution environment for helper dispatch.
pub struct HelperDispatchEnv<'a, E> {
    pub heap: &'a mut Heap,
    pub error_store: &'a mut ErrorStore,
    pub type_checker: &'a dyn TypeContractChecker,
    pub callable_registry: &'a CallableRegistry,
    pub write_barrier: &'a mut dyn WriteBarrierHook,
    pub source_span: Option<SourceSpanId>,
    pub unwind_ctx: &'a mut UnwindContext,
    pub executor: &'a mut E,
}

/// Dispatch a registered helper by id through the shipped boundary.
pub fn dispatch_helper<E: UnwindExecutor>(
    helper_id: RuntimeHelperId,
    args: &[Value],
    env: &mut HelperDispatchEnv<'_, E>,
) -> Result<HelperDispatchOutcome, RuntimeFailure> {
    if helper_id == HELPER_PERFORM_UNWIND_ID {
        let outcome = perform_unwind(env.unwind_ctx, env.executor, env.error_store);
        return Ok(match outcome {
            UnwindOutcome::Resolved(control) => HelperDispatchOutcome::VmControl(control),
            UnwindOutcome::Propagated(_) => {
                return Err(RuntimeFailure::structural(
                    VmStructuralErrorCode::InvalidFrameStateError,
                    "helper left pending unwind state",
                ));
            }
        });
    }

    if helper_id == HELPER_ALLOC_OBJECT_ID {
        return helper_alloc_object(args, env.heap).map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_WRITE_BARRIER_ID {
        helper_write_barrier(args, env.write_barrier)?;
        return Ok(HelperDispatchOutcome::Unit);
    }
    if helper_id == HELPER_CONSTRUCT_ERROR_ID {
        return helper_construct_error(args, env.error_store, env.source_span)
            .map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_CHECK_TYPE_CONTRACT_ID {
        return helper_check_type_contract(args, env.type_checker).map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_CHECK_CALLABLE_ID {
        return helper_check_callable(args, env.callable_registry).map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_CHECK_HASHABLE_ID {
        return helper_check_hashable(args, env.heap).map(HelperDispatchOutcome::Value);
    }

    Err(RuntimeFailure::structural(
        VmStructuralErrorCode::InvalidHelperError,
        format!("helper {} is not dispatched", helper_id.raw()),
    ))
}

/// Legacy unwind-only entry retained for narrow integration tests.
pub fn dispatch_helper_unwind_only(
    helper_id: RuntimeHelperId,
    unwind_ctx: &mut UnwindContext,
    executor: &mut impl UnwindExecutor,
    store: &mut ErrorStore,
) -> Result<UnwindOutcome, VmError> {
    if helper_id != HELPER_PERFORM_UNWIND_ID {
        return Err(VmError::new(
            VmStructuralErrorCode::InvalidHelperError,
            format!("helper {} is not dispatched in bootstrap shell", helper_id.raw()),
        ));
    }
    Ok(perform_unwind(unwind_ctx, executor, store))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call::contract::StubTypeContractChecker;
    use crate::control::PendingControl;
    use crate::unwind::combine::CleanupStepResult;
    use crate::unwind::region::{ControlRegionKind, RuntimeRegionFrame};
    use crate::write_barrier::NoopWriteBarrierHook;
    use vm_core::id::ControlRegionId;

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

    fn test_env<'a>(
        heap: &'a mut Heap,
        store: &'a mut ErrorStore,
        checker: &'a StubTypeContractChecker,
        registry: &'a CallableRegistry,
        barrier: &'a mut NoopWriteBarrierHook,
        unwind_ctx: &'a mut UnwindContext,
        executor: &'a mut NoopExecutor,
    ) -> HelperDispatchEnv<'a, NoopExecutor> {
        HelperDispatchEnv {
            heap,
            error_store: store,
            type_checker: checker,
            callable_registry: registry,
            write_barrier: barrier,
            source_span: None,
            unwind_ctx,
            executor,
        }
    }

    #[test]
    fn helper_perform_unwind_dispatches_to_perform_unwind() {
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let registry = CallableRegistry::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(Some(Value::Int(1))));
        ctx.push_region(RuntimeRegionFrame::new(
            ControlRegionId::new(0),
            ControlRegionKind::Function,
        ));
        let mut executor = NoopExecutor;
        let mut env = test_env(
            &mut heap,
            &mut store,
            &checker,
            &registry,
            &mut barrier,
            &mut ctx,
            &mut executor,
        );
        let outcome = dispatch_helper(HELPER_PERFORM_UNWIND_ID, &[], &mut env).expect("dispatch");
        assert_eq!(
            outcome,
            HelperDispatchOutcome::VmControl(VmControl::Return(Some(Value::Int(1))))
        );
    }

    #[test]
    fn h1_helpers_dispatch_through_central_boundary() {
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let registry = CallableRegistry::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExecutor;
        let mut env = test_env(
            &mut heap,
            &mut store,
            &checker,
            &registry,
            &mut barrier,
            &mut ctx,
            &mut executor,
        );

        assert!(matches!(
            dispatch_helper(HELPER_ALLOC_OBJECT_ID, &[], &mut env).expect("alloc"),
            HelperDispatchOutcome::Value(Value::ObjectRef(_))
        ));
        assert_eq!(
            dispatch_helper(HELPER_WRITE_BARRIER_ID, &[Value::Int(1)], &mut env).expect("barrier"),
            HelperDispatchOutcome::Unit
        );
        assert!(matches!(
            dispatch_helper(
                HELPER_CONSTRUCT_ERROR_ID,
                &[Value::Int(0), Value::String("e".into())],
                &mut env
            )
            .expect("construct"),
            HelperDispatchOutcome::Value(Value::Error(_))
        ));
    }

    #[test]
    fn undispatched_helper_returns_invalid_helper_error() {
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let registry = CallableRegistry::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExecutor;
        let mut env = test_env(
            &mut heap,
            &mut store,
            &checker,
            &registry,
            &mut barrier,
            &mut ctx,
            &mut executor,
        );
        let err = dispatch_helper(RuntimeHelperId::new(15), &[], &mut env).expect_err("reject");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }
}