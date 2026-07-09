//! Milestone H4 control / resource helper implementations.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md` §20.5,
//! `PHASE-3-RUNTIME-HELPER-CONTRACTS.md` §8.6–§8.8,
//! `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md` §2, §5–§8,
//! `PHASE-3-RUNTIME-ERROR-REGISTRY.md` §4

use vm_core::error::language::{ErrorObj, ErrorStore};
use vm_core::error::raise::{type_error_for_invalid_raise, validate_raise_operand, RaiseValidation};
use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::ErrorHandle;
use vm_core::value::Value;

use crate::call::callable::{check_callable, CallableRegistry};
use crate::control::VmControl;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};
use crate::unwind::cleanup::ResourceCloseError;
use crate::unwind::combine::CleanupStepResult;
use crate::unwind::{UnwindContext, UnwindExecutor};

fn type_error() -> RuntimeFailure {
    RuntimeFailure::language(RuntimeErrorCode::TypeError)
}

fn require_arg<'a>(args: &'a [Value], index: usize) -> RuntimeResult<&'a Value> {
    args.get(index).ok_or_else(type_error)
}

fn int_arg(args: &[Value], index: usize) -> RuntimeResult<i64> {
    match args.get(index) {
        Some(Value::Int(v)) => Ok(*v),
        _ => Err(type_error()),
    }
}

fn error_handle(args: &[Value], index: usize) -> RuntimeResult<ErrorHandle> {
    match args.get(index) {
        Some(Value::Error(h)) => Ok(*h),
        _ => Err(type_error()),
    }
}

fn optional_label(args: &[Value], index: usize) -> String {
    match args.get(index) {
        Some(Value::String(s)) => s.clone(),
        _ => String::new(),
    }
}

fn cleanup_to_control(step: CleanupStepResult) -> VmControl {
    match step {
        CleanupStepResult::Normal => VmControl::Normal(None),
        CleanupStepResult::Return(v) => VmControl::Return(v),
        CleanupStepResult::Break(r) => VmControl::Break(r),
        CleanupStepResult::Continue(r) => VmControl::Continue(r),
        CleanupStepResult::Raise(h) => VmControl::Raise(h),
    }
}

/// Bootstrap: `args[0]` must be a live Error value → `VmControl::Raise`.
/// Non-Error operands materialize TypeError and raise that instead (registry §4).
pub fn helper_raise(args: &[Value], store: &mut ErrorStore) -> RuntimeResult<VmControl> {
    let value = require_arg(args, 0)?;
    let handle = match validate_raise_operand(value, store) {
        RaiseValidation::Accepted(h) => h,
        RaiseValidation::RejectedTypeError => type_error_for_invalid_raise(store),
    };
    Ok(VmControl::Raise(handle))
}

/// Bootstrap: `args[0]` primary Error, `args[1]` suppressed Error → Unit.
pub fn helper_attach_suppressed(args: &[Value], store: &mut ErrorStore) -> RuntimeResult<()> {
    let primary = error_handle(args, 0)?;
    let suppressed = error_handle(args, 1)?;
    if store.get(primary).is_none() || store.get(suppressed).is_none() {
        return Err(type_error());
    }
    crate::unwind::combine::attach_suppressed_to_primary(primary, suppressed, store);
    Ok(())
}

/// Bootstrap: optional `args[0]` String message → construct AssertionError and raise.
pub fn helper_assert_fail(args: &[Value], store: &mut ErrorStore) -> RuntimeResult<VmControl> {
    let message = match args.first() {
        Some(Value::String(s)) => s.clone(),
        Some(Value::None) | None => "assertion failed".to_string(),
        _ => return Err(type_error()),
    };
    let handle = store.allocate(ErrorObj::new(RuntimeErrorCode::AssertionError, message));
    Ok(VmControl::Raise(handle))
}

/// Bootstrap: `args[0]` Int callable_id (or ObjectRef id), optional `args[1]` String label.
/// Requires an active region frame; validates zero-arg callable when ObjectRef given.
pub fn helper_register_defer(
    args: &[Value],
    ctx: &mut UnwindContext,
    registry: &CallableRegistry,
) -> RuntimeResult<()> {
    let (callable_id, label) = match args.first() {
        Some(Value::Int(id)) if *id >= 0 => (*id as u32, optional_label(args, 1)),
        Some(Value::ObjectRef(id)) => {
            // Must be registered callable (arity-0 deferred callables use registry presence).
            check_callable(&Value::ObjectRef(*id), registry)?;
            (id.raw(), optional_label(args, 1))
        }
        _ => return Err(type_error()),
    };
    let frame = ctx.top_region_mut().ok_or_else(|| {
        RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidFrameStateError,
            "register_defer requires an active control region",
        )
    })?;
    frame.cleanup_state.register_defer(callable_id, label);
    Ok(())
}

/// Bootstrap: `args[0]` Int callable_id → invoke executor once; returns control outcome.
pub fn helper_execute_defer(
    args: &[Value],
    executor: &mut impl UnwindExecutor,
) -> RuntimeResult<VmControl> {
    let id = int_arg(args, 0)?;
    if id < 0 {
        return Err(type_error());
    }
    Ok(cleanup_to_control(executor.call_defer(id as u32)))
}

/// Bootstrap: `args[0]` Int resource_id, optional `args[1]` String label.
pub fn helper_register_resource(args: &[Value], ctx: &mut UnwindContext) -> RuntimeResult<()> {
    let id = int_arg(args, 0)?;
    if id < 0 {
        return Err(type_error());
    }
    let label = optional_label(args, 1);
    let frame = ctx.top_region_mut().ok_or_else(|| {
        RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidFrameStateError,
            "register_resource requires an active control region",
        )
    })?;
    frame.cleanup_state.register_resource(id as u32, label);
    Ok(())
}

/// Bootstrap: `args[0]` Int resource_id → exactly-once close via executor.
/// Returns VmControl from close result; double-close → ResourceStateError.
pub fn helper_close_resource(
    args: &[Value],
    ctx: &mut UnwindContext,
    executor: &mut impl UnwindExecutor,
) -> RuntimeResult<VmControl> {
    let id = int_arg(args, 0)?;
    if id < 0 {
        return Err(type_error());
    }
    let resource_id = id as u32;
    let frame = ctx.top_region_mut().ok_or_else(|| {
        RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidFrameStateError,
            "close_resource requires an active control region",
        )
    })?;
    let resource = frame
        .cleanup_state
        .resource_stack
        .iter_mut()
        .find(|r| r.resource_id == resource_id)
        .ok_or_else(|| RuntimeFailure::language(RuntimeErrorCode::ResourceStateError))?;

    resource.begin_close().map_err(|e| match e {
        ResourceCloseError::AlreadyClosing
        | ResourceCloseError::AlreadyClosed
        | ResourceCloseError::PreviouslyFailed => {
            RuntimeFailure::language(RuntimeErrorCode::ResourceStateError)
        }
    })?;

    let step = executor.close_resource(resource_id);
    match &step {
        CleanupStepResult::Normal | CleanupStepResult::Return(_)
        | CleanupStepResult::Break(_) | CleanupStepResult::Continue(_) => {
            resource.mark_closed();
        }
        CleanupStepResult::Raise(_) => {
            resource.mark_failed();
        }
    }
    Ok(cleanup_to_control(step))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call::callable::{CallableTarget, UserFunctionTarget};
    use crate::control::PendingControl;
    use crate::unwind::region::{ControlRegionKind, RuntimeRegionFrame};
    use vm_core::id::{ControlRegionId, EirFunctionId, FunctionId, ModuleId, ObjectId};

    struct OkExecutor;
    impl UnwindExecutor for OkExecutor {
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
    fn raise_accepts_error_value() {
        let mut store = ErrorStore::new();
        let h = store.allocate(ErrorObj::new(RuntimeErrorCode::KeyError, "k"));
        let control = helper_raise(&[Value::Error(h)], &mut store).expect("raise");
        assert_eq!(control, VmControl::Raise(h));
    }

    #[test]
    fn raise_non_error_becomes_type_error_raise() {
        let mut store = ErrorStore::new();
        let control = helper_raise(&[Value::Int(1)], &mut store).expect("raise");
        let VmControl::Raise(h) = control else {
            panic!("expected raise");
        };
        assert_eq!(
            store.get(h).expect("err").error_code,
            RuntimeErrorCode::TypeError
        );
    }

    #[test]
    fn attach_suppressed_appends() {
        let mut store = ErrorStore::new();
        let p = store.allocate(ErrorObj::new(RuntimeErrorCode::KeyError, "p"));
        let s = store.allocate(ErrorObj::new(RuntimeErrorCode::TypeError, "s"));
        helper_attach_suppressed(&[Value::Error(p), Value::Error(s)], &mut store).expect("attach");
        assert_eq!(store.get(p).expect("p").suppressed, vec![s]);
    }

    #[test]
    fn assert_fail_raises_assertion_error() {
        let mut store = ErrorStore::new();
        let control =
            helper_assert_fail(&[Value::String("boom".into())], &mut store).expect("assert");
        let VmControl::Raise(h) = control else {
            panic!("raise");
        };
        let err = store.get(h).expect("err");
        assert_eq!(err.error_code, RuntimeErrorCode::AssertionError);
        assert_eq!(err.message, "boom");
    }

    #[test]
    fn register_defer_on_active_region() {
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        ctx.push_region(RuntimeRegionFrame::new(
            ControlRegionId::new(0),
            ControlRegionKind::Block,
        ));
        let registry = CallableRegistry::new();
        helper_register_defer(&[Value::Int(7), Value::String("d".into())], &mut ctx, &registry)
            .expect("reg");
        assert_eq!(
            ctx.top_region().expect("top").cleanup_state.defer_stack.len(),
            1
        );
    }

    #[test]
    fn register_defer_rejects_without_region() {
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let registry = CallableRegistry::new();
        let err = helper_register_defer(&[Value::Int(1)], &mut ctx, &registry).expect_err("no region");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    #[test]
    fn register_defer_object_must_be_callable() {
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        ctx.push_region(RuntimeRegionFrame::new(
            ControlRegionId::new(0),
            ControlRegionKind::Block,
        ));
        let mut registry = CallableRegistry::new();
        let id = ObjectId::new(3);
        registry.register(
            id,
            CallableTarget::UserFunction(UserFunctionTarget {
                function_id: FunctionId::new(0),
                module_id: ModuleId::new(0),
                entry_eir_function: EirFunctionId::new(0),
                return_type: None,
                object_id: id,
            }),
        );
        helper_register_defer(&[Value::ObjectRef(id)], &mut ctx, &registry).expect("callable");
        let err = helper_register_defer(
            &[Value::ObjectRef(ObjectId::new(99))],
            &mut ctx,
            &registry,
        )
        .expect_err("not callable");
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::TypeError));
    }

    #[test]
    fn execute_defer_returns_normal() {
        let mut exec = OkExecutor;
        let control = helper_execute_defer(&[Value::Int(1)], &mut exec).expect("exec");
        assert_eq!(control, VmControl::Normal(None));
    }

    #[test]
    fn register_and_close_resource_once() {
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        ctx.push_region(RuntimeRegionFrame::new(
            ControlRegionId::new(1),
            ControlRegionKind::TryFinally,
        ));
        helper_register_resource(&[Value::Int(5), Value::String("r".into())], &mut ctx)
            .expect("reg");
        let mut exec = OkExecutor;
        let control = helper_close_resource(&[Value::Int(5)], &mut ctx, &mut exec).expect("close");
        assert_eq!(control, VmControl::Normal(None));
        let err = helper_close_resource(&[Value::Int(5)], &mut ctx, &mut exec).expect_err("double");
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::ResourceStateError)
        );
    }
}
