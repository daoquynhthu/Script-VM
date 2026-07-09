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
use crate::module::resolver::{CapabilitySet, HostModuleResolver};
use crate::module::runtime::ModuleRuntime;
use crate::runtime_error::RuntimeFailure;
use crate::unwind::{perform_unwind, UnwindContext, UnwindExecutor, UnwindOutcome};
use crate::write_barrier::WriteBarrierHook;

use super::h1::{
    helper_alloc_object, helper_check_callable, helper_check_hashable,
    helper_check_type_contract, helper_construct_error, helper_write_barrier,
};
use super::h2::{
    helper_compare, helper_construct_enum, helper_construct_map, helper_construct_record,
    helper_display, helper_get_attribute, helper_index_read, helper_index_write,
    helper_numeric_binary, helper_set_attribute, helper_slice_read,
};
use super::h3::{
    helper_bind_method, helper_call_builtin, helper_check_arity, helper_generic_call,
    CallSiteFeedback,
};
use super::h4::{
    helper_assert_fail, helper_attach_suppressed, helper_close_resource, helper_execute_defer,
    helper_raise, helper_register_defer, helper_register_resource,
};
use super::h5::{
    helper_import_module, helper_import_named, helper_initialize_module, helper_resolve_module,
    helper_seal_exports,
};
use super::h6::{
    helper_check_capability, helper_enter_host_call, helper_exit_host_call, HostBoundarySession,
};

/// Canonical helper id for `helper_alloc_object` (registry §3 table order).
pub const HELPER_ALLOC_OBJECT_ID: RuntimeHelperId = RuntimeHelperId::new(0);
/// Canonical helper id for `helper_write_barrier`.
pub const HELPER_WRITE_BARRIER_ID: RuntimeHelperId = RuntimeHelperId::new(1);
/// Canonical helper id for `helper_construct_error`.
pub const HELPER_CONSTRUCT_ERROR_ID: RuntimeHelperId = RuntimeHelperId::new(2);
/// Canonical helper id for `helper_raise`.
pub const HELPER_RAISE_ID: RuntimeHelperId = RuntimeHelperId::new(3);
/// Canonical helper id for `helper_attach_suppressed`.
pub const HELPER_ATTACH_SUPPRESSED_ID: RuntimeHelperId = RuntimeHelperId::new(4);
/// Canonical helper id for `helper_assert_fail`.
pub const HELPER_ASSERT_FAIL_ID: RuntimeHelperId = RuntimeHelperId::new(5);
/// Canonical helper id for `helper_check_type_contract`.
pub const HELPER_CHECK_TYPE_CONTRACT_ID: RuntimeHelperId = RuntimeHelperId::new(6);
/// Canonical helper id for `helper_check_callable`.
pub const HELPER_CHECK_CALLABLE_ID: RuntimeHelperId = RuntimeHelperId::new(7);
/// Canonical helper id for `helper_check_hashable`.
pub const HELPER_CHECK_HASHABLE_ID: RuntimeHelperId = RuntimeHelperId::new(8);
/// Canonical helper id for `helper_numeric_binary`.
pub const HELPER_NUMERIC_BINARY_ID: RuntimeHelperId = RuntimeHelperId::new(11);
/// Canonical helper id for `helper_compare`.
pub const HELPER_COMPARE_ID: RuntimeHelperId = RuntimeHelperId::new(12);
/// Canonical helper id for `helper_get_attribute`.
pub const HELPER_GET_ATTRIBUTE_ID: RuntimeHelperId = RuntimeHelperId::new(13);
/// Canonical helper id for `helper_set_attribute`.
pub const HELPER_SET_ATTRIBUTE_ID: RuntimeHelperId = RuntimeHelperId::new(14);
/// Canonical helper id for `helper_index_read`.
pub const HELPER_INDEX_READ_ID: RuntimeHelperId = RuntimeHelperId::new(16);
/// Canonical helper id for `helper_index_write`.
pub const HELPER_INDEX_WRITE_ID: RuntimeHelperId = RuntimeHelperId::new(17);
/// Canonical helper id for `helper_slice_read`.
pub const HELPER_SLICE_READ_ID: RuntimeHelperId = RuntimeHelperId::new(18);
/// Canonical helper id for `helper_construct_map`.
pub const HELPER_CONSTRUCT_MAP_ID: RuntimeHelperId = RuntimeHelperId::new(21);
/// Canonical helper id for `helper_construct_record`.
pub const HELPER_CONSTRUCT_RECORD_ID: RuntimeHelperId = RuntimeHelperId::new(22);
/// Canonical helper id for `helper_construct_enum`.
pub const HELPER_CONSTRUCT_ENUM_ID: RuntimeHelperId = RuntimeHelperId::new(23);
/// Canonical helper id for `helper_generic_call`.
pub const HELPER_GENERIC_CALL_ID: RuntimeHelperId = RuntimeHelperId::new(25);
/// Canonical helper id for `helper_call_builtin`.
pub const HELPER_CALL_BUILTIN_ID: RuntimeHelperId = RuntimeHelperId::new(26);
/// Canonical helper id for `helper_check_arity`.
pub const HELPER_CHECK_ARITY_ID: RuntimeHelperId = RuntimeHelperId::new(27);
/// Canonical helper id for `helper_bind_method`.
pub const HELPER_BIND_METHOD_ID: RuntimeHelperId = RuntimeHelperId::new(15);
/// Canonical helper id for `helper_perform_unwind` (registry §3 table order).
pub const HELPER_PERFORM_UNWIND_ID: RuntimeHelperId = RuntimeHelperId::new(29);
/// Canonical helper id for `helper_register_defer`.
pub const HELPER_REGISTER_DEFER_ID: RuntimeHelperId = RuntimeHelperId::new(30);
/// Canonical helper id for `helper_execute_defer`.
pub const HELPER_EXECUTE_DEFER_ID: RuntimeHelperId = RuntimeHelperId::new(31);
/// Canonical helper id for `helper_register_resource`.
pub const HELPER_REGISTER_RESOURCE_ID: RuntimeHelperId = RuntimeHelperId::new(32);
/// Canonical helper id for `helper_close_resource`.
pub const HELPER_CLOSE_RESOURCE_ID: RuntimeHelperId = RuntimeHelperId::new(33);
/// Canonical helper id for `helper_resolve_module`.
pub const HELPER_RESOLVE_MODULE_ID: RuntimeHelperId = RuntimeHelperId::new(34);
/// Canonical helper id for `helper_initialize_module`.
pub const HELPER_INITIALIZE_MODULE_ID: RuntimeHelperId = RuntimeHelperId::new(35);
/// Canonical helper id for `helper_import_named`.
pub const HELPER_IMPORT_NAMED_ID: RuntimeHelperId = RuntimeHelperId::new(36);
/// Canonical helper id for `helper_import_module`.
pub const HELPER_IMPORT_MODULE_ID: RuntimeHelperId = RuntimeHelperId::new(37);
/// Canonical helper id for `helper_seal_exports`.
pub const HELPER_SEAL_EXPORTS_ID: RuntimeHelperId = RuntimeHelperId::new(38);
/// Canonical helper id for `helper_check_capability`.
pub const HELPER_CHECK_CAPABILITY_ID: RuntimeHelperId = RuntimeHelperId::new(39);
/// Canonical helper id for `helper_enter_host_call`.
pub const HELPER_ENTER_HOST_CALL_ID: RuntimeHelperId = RuntimeHelperId::new(40);
/// Canonical helper id for `helper_exit_host_call`.
pub const HELPER_EXIT_HOST_CALL_ID: RuntimeHelperId = RuntimeHelperId::new(41);
/// Canonical helper id for `helper_display`.
pub const HELPER_DISPLAY_ID: RuntimeHelperId = RuntimeHelperId::new(42);

/// Default max logical call depth for bootstrap stack-overflow checks.
pub const DEFAULT_MAX_CALL_DEPTH: u32 = 64;

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
    pub callable_registry: &'a mut CallableRegistry,
    pub capabilities: &'a CapabilitySet,
    pub call_site_feedback: Option<&'a mut CallSiteFeedback>,
    pub call_depth: u32,
    pub max_call_depth: u32,
    pub module_runtime: Option<&'a mut ModuleRuntime>,
    pub module_resolver: Option<&'a dyn HostModuleResolver>,
    pub host_session: Option<&'a mut HostBoundarySession>,
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

    // H1
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

    // H2
    if helper_id == HELPER_NUMERIC_BINARY_ID {
        return helper_numeric_binary(args).map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_COMPARE_ID {
        return helper_compare(args).map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_GET_ATTRIBUTE_ID {
        return helper_get_attribute(args, env.heap).map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_SET_ATTRIBUTE_ID {
        helper_set_attribute(args, env.heap, env.write_barrier)?;
        return Ok(HelperDispatchOutcome::Unit);
    }
    if helper_id == HELPER_INDEX_READ_ID {
        return helper_index_read(args, env.heap).map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_INDEX_WRITE_ID {
        helper_index_write(args, env.heap, env.write_barrier)?;
        return Ok(HelperDispatchOutcome::Unit);
    }
    if helper_id == HELPER_SLICE_READ_ID {
        return helper_slice_read(args, env.heap).map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_CONSTRUCT_MAP_ID {
        return helper_construct_map(args, env.heap).map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_CONSTRUCT_RECORD_ID {
        return helper_construct_record(args, env.heap).map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_CONSTRUCT_ENUM_ID {
        return helper_construct_enum(args, env.heap).map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_DISPLAY_ID {
        return helper_display(args, env.heap).map(HelperDispatchOutcome::Value);
    }

    // H3 call engine
    if helper_id == HELPER_BIND_METHOD_ID {
        return helper_bind_method(args, env.heap, env.callable_registry)
            .map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_CHECK_ARITY_ID {
        helper_check_arity(args)?;
        return Ok(HelperDispatchOutcome::Unit);
    }
    if helper_id == HELPER_GENERIC_CALL_ID {
        let control = helper_generic_call(
            args,
            env.callable_registry,
            env.type_checker,
            env.capabilities,
            env.call_site_feedback.as_deref_mut(),
            env.call_depth,
            env.max_call_depth,
        )?;
        return Ok(HelperDispatchOutcome::VmControl(control));
    }
    if helper_id == HELPER_CALL_BUILTIN_ID {
        let control = helper_call_builtin(
            args,
            env.callable_registry,
            env.capabilities,
            env.call_site_feedback.as_deref_mut(),
        )?;
        return Ok(HelperDispatchOutcome::VmControl(control));
    }

    // H4 control / resource
    if helper_id == HELPER_RAISE_ID {
        return helper_raise(args, env.error_store).map(HelperDispatchOutcome::VmControl);
    }
    if helper_id == HELPER_ATTACH_SUPPRESSED_ID {
        helper_attach_suppressed(args, env.error_store)?;
        return Ok(HelperDispatchOutcome::Unit);
    }
    if helper_id == HELPER_ASSERT_FAIL_ID {
        return helper_assert_fail(args, env.error_store).map(HelperDispatchOutcome::VmControl);
    }
    if helper_id == HELPER_REGISTER_DEFER_ID {
        helper_register_defer(args, env.unwind_ctx, env.callable_registry)?;
        return Ok(HelperDispatchOutcome::Unit);
    }
    if helper_id == HELPER_EXECUTE_DEFER_ID {
        return helper_execute_defer(args, env.executor).map(HelperDispatchOutcome::VmControl);
    }
    if helper_id == HELPER_REGISTER_RESOURCE_ID {
        helper_register_resource(args, env.unwind_ctx)?;
        return Ok(HelperDispatchOutcome::Unit);
    }
    if helper_id == HELPER_CLOSE_RESOURCE_ID {
        return helper_close_resource(args, env.unwind_ctx, env.executor)
            .map(HelperDispatchOutcome::VmControl);
    }

    // H5 module
    if helper_id == HELPER_RESOLVE_MODULE_ID {
        return helper_resolve_module(args, env.module_runtime.as_deref_mut(), env.module_resolver)
            .map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_INITIALIZE_MODULE_ID {
        return helper_initialize_module(args, env.module_runtime.as_deref_mut())
            .map(HelperDispatchOutcome::VmControl);
    }
    if helper_id == HELPER_IMPORT_NAMED_ID {
        return helper_import_named(args, env.module_runtime.as_deref_mut())
            .map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_IMPORT_MODULE_ID {
        return helper_import_module(args, env.module_runtime.as_deref_mut())
            .map(HelperDispatchOutcome::Value);
    }
    if helper_id == HELPER_SEAL_EXPORTS_ID {
        helper_seal_exports(args, env.module_runtime.as_deref_mut())?;
        return Ok(HelperDispatchOutcome::Unit);
    }

    // H6 capability / host boundary
    if helper_id == HELPER_CHECK_CAPABILITY_ID {
        helper_check_capability(args, env.capabilities)?;
        return Ok(HelperDispatchOutcome::Unit);
    }
    if helper_id == HELPER_ENTER_HOST_CALL_ID {
        helper_enter_host_call(args, env.host_session.as_deref_mut())?;
        return Ok(HelperDispatchOutcome::Unit);
    }
    if helper_id == HELPER_EXIT_HOST_CALL_ID {
        return helper_exit_host_call(args, env.host_session.as_deref_mut(), env.error_store)
            .map(HelperDispatchOutcome::VmControl);
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
    use crate::helpers::h2::{
        COMPARE_OP_LT, NUMERIC_OP_ADD, NUMERIC_OP_DIV, NUMERIC_OP_MUL,
    };
    use crate::module::resolver::CapabilitySet;
    use crate::unwind::combine::CleanupStepResult;
    use crate::unwind::region::{ControlRegionKind, RuntimeRegionFrame};
    use crate::write_barrier::NoopWriteBarrierHook;
    use vm_core::error::registry::RuntimeErrorCode;
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
        registry: &'a mut CallableRegistry,
        capabilities: &'a CapabilitySet,
        feedback: Option<&'a mut CallSiteFeedback>,
        barrier: &'a mut NoopWriteBarrierHook,
        unwind_ctx: &'a mut UnwindContext,
        executor: &'a mut NoopExecutor,
    ) -> HelperDispatchEnv<'a, NoopExecutor> {
        HelperDispatchEnv {
            heap,
            error_store: store,
            type_checker: checker,
            callable_registry: registry,
            capabilities,
            call_site_feedback: feedback,
            call_depth: 0,
            max_call_depth: DEFAULT_MAX_CALL_DEPTH,
            module_runtime: None,
            module_resolver: None,
            host_session: None,
            write_barrier: barrier,
            source_span: None,
            unwind_ctx,
            executor,
        }
    }

    fn with_env<F, T>(f: F) -> T
    where
        F: FnOnce(&mut HelperDispatchEnv<'_, NoopExecutor>) -> T,
    {
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let capabilities = CapabilitySet::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExecutor;
        let mut env = test_env(
            &mut heap,
            &mut store,
            &checker,
            &mut registry,
            &capabilities,
            None,
            &mut barrier,
            &mut ctx,
            &mut executor,
        );
        f(&mut env)
    }

    // --- H1 regression ---

    #[test]
    fn helper_perform_unwind_dispatches_to_perform_unwind() {
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let capabilities = CapabilitySet::new();
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
            &mut registry,
            &capabilities,
            None,
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
    fn dispatch_alloc_object_returns_heap_handle() {
        with_env(|env| {
            let outcome = dispatch_helper(HELPER_ALLOC_OBJECT_ID, &[], env).expect("alloc");
            assert!(matches!(outcome, HelperDispatchOutcome::Value(Value::ObjectRef(_))));
        });
    }

    #[test]
    fn dispatch_write_barrier_returns_unit() {
        with_env(|env| {
            let outcome =
                dispatch_helper(HELPER_WRITE_BARRIER_ID, &[Value::Int(1)], env).expect("barrier");
            assert_eq!(outcome, HelperDispatchOutcome::Unit);
        });
    }

    #[test]
    fn dispatch_construct_error_returns_error_value() {
        with_env(|env| {
            let outcome = dispatch_helper(
                HELPER_CONSTRUCT_ERROR_ID,
                &[Value::Int(0), Value::String("e".into())],
                env,
            )
            .expect("construct");
            assert!(matches!(outcome, HelperDispatchOutcome::Value(Value::Error(_))));
        });
    }

    #[test]
    fn dispatch_check_type_contract_returns_value_on_match() {
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let mut checker = StubTypeContractChecker::new();
        checker.declare_int_type(vm_core::id::TypeId::new(1));
        let mut registry = CallableRegistry::new();
        let capabilities = CapabilitySet::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExecutor;
        let mut env = test_env(
            &mut heap,
            &mut store,
            &checker,
            &mut registry,
            &capabilities,
            None,
            &mut barrier,
            &mut ctx,
            &mut executor,
        );
        let outcome = dispatch_helper(
            HELPER_CHECK_TYPE_CONTRACT_ID,
            &[Value::Int(7), Value::Int(1)],
            &mut env,
        )
        .expect("type contract");
        assert_eq!(outcome, HelperDispatchOutcome::Value(Value::Int(7)));
    }

    #[test]
    fn dispatch_check_type_contract_rejects_mismatch() {
        with_env(|env| {
            let err = dispatch_helper(
                HELPER_CHECK_TYPE_CONTRACT_ID,
                &[Value::Int(7), Value::Int(1)],
                env,
            )
            .expect_err("reject");
            assert_eq!(
                err,
                RuntimeFailure::language(RuntimeErrorCode::TypeContractError)
            );
        });
    }

    #[test]
    fn dispatch_check_callable_returns_callee_on_success() {
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let capabilities = CapabilitySet::new();
        let object_id = vm_core::id::ObjectId::new(1);
        registry.register(
            object_id,
            crate::call::callable::CallableTarget::UserFunction(
                crate::call::callable::UserFunctionTarget {
                    function_id: vm_core::id::FunctionId::new(0),
                    module_id: vm_core::id::ModuleId::new(0),
                    entry_eir_function: vm_core::id::EirFunctionId::new(0),
                    return_type: None,
                    object_id,
                },
            ),
        );
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExecutor;
        let mut env = test_env(
            &mut heap,
            &mut store,
            &checker,
            &mut registry,
            &capabilities,
            None,
            &mut barrier,
            &mut ctx,
            &mut executor,
        );
        let callee = Value::ObjectRef(object_id);
        let outcome =
            dispatch_helper(HELPER_CHECK_CALLABLE_ID, &[callee.clone()], &mut env).expect("callable");
        assert_eq!(outcome, HelperDispatchOutcome::Value(callee));
    }

    #[test]
    fn dispatch_check_hashable_returns_value_on_success() {
        with_env(|env| {
            let outcome =
                dispatch_helper(HELPER_CHECK_HASHABLE_ID, &[Value::Int(3)], env).expect("hashable");
            assert_eq!(outcome, HelperDispatchOutcome::Value(Value::Int(3)));
        });
    }

    #[test]
    fn dispatch_check_hashable_rejects_non_hashable_object() {
        with_env(|env| {
            let list = env.heap.alloc_list(vec![], false).expect("list");
            let err = dispatch_helper(
                HELPER_CHECK_HASHABLE_ID,
                &[Value::ObjectRef(list.id())],
                env,
            )
            .expect_err("reject");
            assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::TypeError));
        });
    }

    // --- H2 dispatch boundary ---

    #[test]
    fn dispatch_get_attribute_reads_record_field() {
        with_env(|env| {
            let rec = env
                .heap
                .alloc_record_instance(vec![Value::Int(11)], false)
                .expect("rec");
            let outcome = dispatch_helper(
                HELPER_GET_ATTRIBUTE_ID,
                &[Value::ObjectRef(rec.id()), Value::Int(0)],
                env,
            )
            .expect("get");
            assert_eq!(outcome, HelperDispatchOutcome::Value(Value::Int(11)));
        });
    }

    #[test]
    fn dispatch_get_attribute_rejects_unknown_field() {
        with_env(|env| {
            let rec = env
                .heap
                .alloc_record_instance(vec![Value::Int(1)], false)
                .expect("rec");
            let err = dispatch_helper(
                HELPER_GET_ATTRIBUTE_ID,
                &[Value::ObjectRef(rec.id()), Value::Int(3)],
                env,
            )
            .expect_err("field");
            assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::FieldError));
        });
    }

    #[test]
    fn dispatch_set_attribute_writes_field() {
        with_env(|env| {
            let rec = env
                .heap
                .alloc_record_instance(vec![Value::Int(0)], false)
                .expect("rec");
            let outcome = dispatch_helper(
                HELPER_SET_ATTRIBUTE_ID,
                &[Value::ObjectRef(rec.id()), Value::Int(0), Value::Int(99)],
                env,
            )
            .expect("set");
            assert_eq!(outcome, HelperDispatchOutcome::Unit);
            let read = dispatch_helper(
                HELPER_GET_ATTRIBUTE_ID,
                &[Value::ObjectRef(rec.id()), Value::Int(0)],
                env,
            )
            .expect("get");
            assert_eq!(read, HelperDispatchOutcome::Value(Value::Int(99)));
        });
    }

    #[test]
    fn dispatch_set_attribute_rejects_readonly_record() {
        with_env(|env| {
            let rec = env
                .heap
                .alloc_record_instance(vec![Value::Int(0)], true)
                .expect("rec");
            let err = dispatch_helper(
                HELPER_SET_ATTRIBUTE_ID,
                &[Value::ObjectRef(rec.id()), Value::Int(0), Value::Int(1)],
                env,
            )
            .expect_err("readonly");
            assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::ReadOnlyError));
        });
    }

    #[test]
    fn dispatch_index_read_list_and_map() {
        with_env(|env| {
            let list = env
                .heap
                .alloc_list(vec![Value::Int(5), Value::Int(6)], false)
                .expect("list");
            let list_out = dispatch_helper(
                HELPER_INDEX_READ_ID,
                &[Value::ObjectRef(list.id()), Value::Int(1)],
                env,
            )
            .expect("list idx");
            assert_eq!(list_out, HelperDispatchOutcome::Value(Value::Int(6)));

            let map = env.heap.alloc_map(false).expect("map");
            env.heap
                .map_insert(map, Value::String("k".into()), Value::Int(8))
                .expect("insert");
            let map_out = dispatch_helper(
                HELPER_INDEX_READ_ID,
                &[Value::ObjectRef(map.id()), Value::String("k".into())],
                env,
            )
            .expect("map idx");
            assert_eq!(map_out, HelperDispatchOutcome::Value(Value::Int(8)));
        });
    }

    #[test]
    fn dispatch_index_read_rejects_oob_and_missing_key() {
        with_env(|env| {
            let list = env.heap.alloc_list(vec![Value::Int(1)], false).expect("list");
            let err = dispatch_helper(
                HELPER_INDEX_READ_ID,
                &[Value::ObjectRef(list.id()), Value::Int(9)],
                env,
            )
            .expect_err("oob");
            assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::IndexError));

            let map = env.heap.alloc_map(false).expect("map");
            let err = dispatch_helper(
                HELPER_INDEX_READ_ID,
                &[Value::ObjectRef(map.id()), Value::String("missing".into())],
                env,
            )
            .expect_err("key");
            assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::KeyError));
        });
    }

    #[test]
    fn dispatch_index_write_list_and_map() {
        with_env(|env| {
            let list = env
                .heap
                .alloc_list(vec![Value::Int(0)], false)
                .expect("list");
            let unit = dispatch_helper(
                HELPER_INDEX_WRITE_ID,
                &[Value::ObjectRef(list.id()), Value::Int(0), Value::Int(3)],
                env,
            )
            .expect("list write");
            assert_eq!(unit, HelperDispatchOutcome::Unit);
            let read = dispatch_helper(
                HELPER_INDEX_READ_ID,
                &[Value::ObjectRef(list.id()), Value::Int(0)],
                env,
            )
            .expect("read");
            assert_eq!(read, HelperDispatchOutcome::Value(Value::Int(3)));

            let map = env.heap.alloc_map(false).expect("map");
            dispatch_helper(
                HELPER_INDEX_WRITE_ID,
                &[
                    Value::ObjectRef(map.id()),
                    Value::Int(1),
                    Value::String("v".into()),
                ],
                env,
            )
            .expect("map write");
            let map_read = dispatch_helper(
                HELPER_INDEX_READ_ID,
                &[Value::ObjectRef(map.id()), Value::Int(1)],
                env,
            )
            .expect("map read");
            assert_eq!(
                map_read,
                HelperDispatchOutcome::Value(Value::String("v".into()))
            );
        });
    }

    #[test]
    fn dispatch_slice_read_list_and_string() {
        with_env(|env| {
            let list = env
                .heap
                .alloc_list(
                    vec![Value::Int(1), Value::Int(2), Value::Int(3)],
                    false,
                )
                .expect("list");
            let sliced = dispatch_helper(
                HELPER_SLICE_READ_ID,
                &[Value::ObjectRef(list.id()), Value::Int(1), Value::Int(3)],
                env,
            )
            .expect("slice list");
            let HelperDispatchOutcome::Value(Value::ObjectRef(id)) = sliced else {
                panic!("expected list object");
            };
            let out = dispatch_helper(
                HELPER_INDEX_READ_ID,
                &[Value::ObjectRef(id), Value::Int(0)],
                env,
            )
            .expect("elem0");
            assert_eq!(out, HelperDispatchOutcome::Value(Value::Int(2)));

            let s = dispatch_helper(
                HELPER_SLICE_READ_ID,
                &[Value::String("abcd".into()), Value::Int(1), Value::Int(3)],
                env,
            )
            .expect("slice str");
            assert_eq!(s, HelperDispatchOutcome::Value(Value::String("bc".into())));
        });
    }

    #[test]
    fn dispatch_slice_read_rejects_negative_and_oob() {
        with_env(|env| {
            let err = dispatch_helper(
                HELPER_SLICE_READ_ID,
                &[Value::String("ab".into()), Value::Int(-1), Value::Int(1)],
                env,
            )
            .expect_err("neg");
            assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::IndexError));

            let err = dispatch_helper(
                HELPER_SLICE_READ_ID,
                &[Value::String("ab".into()), Value::Int(0), Value::Int(9)],
                env,
            )
            .expect_err("oob");
            assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::IndexError));
        });
    }

    #[test]
    fn dispatch_construct_record_enum_map() {
        with_env(|env| {
            let rec = dispatch_helper(
                HELPER_CONSTRUCT_RECORD_ID,
                &[Value::Int(1), Value::Int(2)],
                env,
            )
            .expect("record");
            let HelperDispatchOutcome::Value(Value::ObjectRef(rec_id)) = rec else {
                panic!("record ref");
            };
            let f0 = dispatch_helper(
                HELPER_GET_ATTRIBUTE_ID,
                &[Value::ObjectRef(rec_id), Value::Int(0)],
                env,
            )
            .expect("f0");
            assert_eq!(f0, HelperDispatchOutcome::Value(Value::Int(1)));

            let en = dispatch_helper(
                HELPER_CONSTRUCT_ENUM_ID,
                &[Value::Int(3), Value::Int(1), Value::String("p".into())],
                env,
            )
            .expect("enum");
            assert!(matches!(en, HelperDispatchOutcome::Value(Value::ObjectRef(_))));

            let map = dispatch_helper(
                HELPER_CONSTRUCT_MAP_ID,
                &[Value::String("a".into()), Value::Int(10)],
                env,
            )
            .expect("map");
            let HelperDispatchOutcome::Value(Value::ObjectRef(map_id)) = map else {
                panic!("map ref");
            };
            let v = dispatch_helper(
                HELPER_INDEX_READ_ID,
                &[Value::ObjectRef(map_id), Value::String("a".into())],
                env,
            )
            .expect("map get");
            assert_eq!(v, HelperDispatchOutcome::Value(Value::Int(10)));
        });
    }

    #[test]
    fn dispatch_construct_map_rejects_odd_args_and_non_hashable_key() {
        with_env(|env| {
            let err = dispatch_helper(
                HELPER_CONSTRUCT_MAP_ID,
                &[Value::Int(1)],
                env,
            )
            .expect_err("odd");
            assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::TypeError));

            let list = env.heap.alloc_list(vec![], false).expect("list");
            let err = dispatch_helper(
                HELPER_CONSTRUCT_MAP_ID,
                &[Value::ObjectRef(list.id()), Value::Int(1)],
                env,
            )
            .expect_err("non-hashable");
            assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::TypeError));
        });
    }

    #[test]
    fn dispatch_numeric_binary_success_and_raises() {
        with_env(|env| {
            let add = dispatch_helper(
                HELPER_NUMERIC_BINARY_ID,
                &[
                    Value::Int(NUMERIC_OP_ADD),
                    Value::Int(4),
                    Value::Int(5),
                ],
                env,
            )
            .expect("add");
            assert_eq!(add, HelperDispatchOutcome::Value(Value::Int(9)));

            let div0 = dispatch_helper(
                HELPER_NUMERIC_BINARY_ID,
                &[
                    Value::Int(NUMERIC_OP_DIV),
                    Value::Int(1),
                    Value::Int(0),
                ],
                env,
            )
            .expect_err("div0");
            assert_eq!(
                div0,
                RuntimeFailure::language(RuntimeErrorCode::DivisionByZeroError)
            );

            let overflow = dispatch_helper(
                HELPER_NUMERIC_BINARY_ID,
                &[
                    Value::Int(NUMERIC_OP_MUL),
                    Value::Int(i64::MAX),
                    Value::Int(2),
                ],
                env,
            )
            .expect_err("overflow");
            assert_eq!(
                overflow,
                RuntimeFailure::language(RuntimeErrorCode::NumericOverflowError)
            );

            let coerce = dispatch_helper(
                HELPER_NUMERIC_BINARY_ID,
                &[
                    Value::Int(NUMERIC_OP_ADD),
                    Value::Int(1),
                    Value::Float(2.0),
                ],
                env,
            )
            .expect_err("no coerce");
            assert_eq!(coerce, RuntimeFailure::language(RuntimeErrorCode::TypeError));
        });
    }

    #[test]
    fn dispatch_compare_success_and_unsupported() {
        with_env(|env| {
            let lt = dispatch_helper(
                HELPER_COMPARE_ID,
                &[Value::Int(COMPARE_OP_LT), Value::Int(1), Value::Int(2)],
                env,
            )
            .expect("lt");
            assert_eq!(lt, HelperDispatchOutcome::Value(Value::Bool(true)));

            let bad = dispatch_helper(
                HELPER_COMPARE_ID,
                &[Value::Int(COMPARE_OP_LT), Value::Int(1), Value::Bool(true)],
                env,
            )
            .expect_err("unsupported");
            assert_eq!(bad, RuntimeFailure::language(RuntimeErrorCode::TypeError));
        });
    }

    #[test]
    fn dispatch_display_returns_string() {
        with_env(|env| {
            let out = dispatch_helper(HELPER_DISPLAY_ID, &[Value::Int(42)], env).expect("display");
            assert_eq!(
                out,
                HelperDispatchOutcome::Value(Value::String("42".into()))
            );

            let missing = dispatch_helper(HELPER_DISPLAY_ID, &[], env).expect_err("args");
            assert_eq!(
                missing,
                RuntimeFailure::language(RuntimeErrorCode::TypeError)
            );
        });
    }

    // --- H3 call-engine dispatch boundary ---

    #[test]
    fn dispatch_check_arity_success_and_reject() {
        with_env(|env| {
            let ok = dispatch_helper(
                HELPER_CHECK_ARITY_ID,
                &[Value::Int(2), Value::Int(1), Value::Int(2)],
                env,
            )
            .expect("arity ok");
            assert_eq!(ok, HelperDispatchOutcome::Unit);
            let err = dispatch_helper(
                HELPER_CHECK_ARITY_ID,
                &[Value::Int(1), Value::Int(1), Value::Int(2)],
                env,
            )
            .expect_err("arity bad");
            assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::ArityError));
        });
    }

    #[test]
    fn dispatch_bind_method_registers_bound_callable() {
        with_env(|env| {
            let receiver = env.heap.alloc_list(vec![], false).expect("recv");
            let outcome = dispatch_helper(
                HELPER_BIND_METHOD_ID,
                &[
                    Value::ObjectRef(receiver.id()),
                    Value::Int(4),
                    Value::String("run".into()),
                ],
                env,
            )
            .expect("bind");
            let HelperDispatchOutcome::Value(Value::ObjectRef(bound_id)) = outcome else {
                panic!("bound ref");
            };
            let resolved = env
                .callable_registry
                .resolve(&Value::ObjectRef(bound_id))
                .expect("resolve");
            match resolved {
                crate::call::callable::CallableTarget::BoundMethod(m) => {
                    assert_eq!(m.receiver_id, receiver.id());
                    assert_eq!(m.method_name, "run");
                }
                other => panic!("unexpected {other:?}"),
            }
        });
    }

    #[test]
    fn dispatch_bind_method_rejects_bad_args() {
        with_env(|env| {
            let err = dispatch_helper(
                HELPER_BIND_METHOD_ID,
                &[Value::Int(1), Value::Int(2), Value::String("m".into())],
                env,
            )
            .expect_err("bad receiver");
            assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::TypeError));
        });
    }

    #[test]
    fn dispatch_generic_call_prepares_user_function() {
        with_env(|env| {
            let shell = env.heap.alloc_function().expect("fn");
            let object_id = shell.id();
            env.callable_registry.register(
                object_id,
                crate::call::callable::CallableTarget::UserFunction(
                    crate::call::callable::UserFunctionTarget {
                        function_id: vm_core::id::FunctionId::new(8),
                        module_id: vm_core::id::ModuleId::new(0),
                        entry_eir_function: vm_core::id::EirFunctionId::new(0),
                        return_type: None,
                        object_id,
                    },
                ),
            );
            let outcome = dispatch_helper(
                HELPER_GENERIC_CALL_ID,
                &[Value::ObjectRef(object_id), Value::Int(1), Value::Int(2)],
                env,
            )
            .expect("generic");
            assert_eq!(
                outcome,
                HelperDispatchOutcome::VmControl(VmControl::Normal(Some(Value::ObjectRef(
                    object_id
                ))))
            );
        });
    }

    #[test]
    fn dispatch_generic_call_rejects_non_callable() {
        with_env(|env| {
            let err = dispatch_helper(
                HELPER_GENERIC_CALL_ID,
                &[Value::Int(1)],
                env,
            )
            .expect_err("not callable");
            assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::TypeError));
        });
    }

    #[test]
    fn dispatch_generic_call_stack_overflow() {
        with_env(|env| {
            env.call_depth = 64;
            env.max_call_depth = 64;
            let shell = env.heap.alloc_function().expect("fn");
            env.callable_registry.register(
                shell.id(),
                crate::call::callable::CallableTarget::UserFunction(
                    crate::call::callable::UserFunctionTarget {
                        function_id: vm_core::id::FunctionId::new(0),
                        module_id: vm_core::id::ModuleId::new(0),
                        entry_eir_function: vm_core::id::EirFunctionId::new(0),
                        return_type: None,
                        object_id: shell.id(),
                    },
                ),
            );
            let err = dispatch_helper(
                HELPER_GENERIC_CALL_ID,
                &[Value::ObjectRef(shell.id())],
                env,
            )
            .expect_err("overflow");
            assert_eq!(
                err,
                RuntimeFailure::language(RuntimeErrorCode::StackOverflowError)
            );
        });
    }

    #[test]
    fn dispatch_call_builtin_success_and_capability_reject() {
        with_env(|env| {
            let shell = env.heap.alloc_function().expect("fn");
            let object_id = shell.id();
            env.callable_registry.register(
                object_id,
                crate::call::callable::CallableTarget::BuiltinFunction(
                    crate::call::callable::BuiltinFunctionTarget {
                        builtin_id: 3,
                        arity: 1,
                        required_capabilities: vec![vm_core::id::CapabilityId::new(9)],
                    },
                ),
            );
            // no capability granted
            let err = dispatch_helper(
                HELPER_CALL_BUILTIN_ID,
                &[Value::ObjectRef(object_id), Value::Int(1)],
                env,
            )
            .expect_err("cap");
            assert_eq!(
                err,
                RuntimeFailure::language(RuntimeErrorCode::CapabilityError)
            );
        });

        // success with capability granted
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let mut capabilities = CapabilitySet::new();
        capabilities.grant(vm_core::id::CapabilityId::new(9));
        let mut feedback = CallSiteFeedback::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExecutor;
        let shell = heap.alloc_function().expect("fn");
        let object_id = shell.id();
        registry.register(
            object_id,
            crate::call::callable::CallableTarget::BuiltinFunction(
                crate::call::callable::BuiltinFunctionTarget {
                    builtin_id: 3,
                    arity: 1,
                    required_capabilities: vec![vm_core::id::CapabilityId::new(9)],
                },
            ),
        );
        let mut env = test_env(
            &mut heap,
            &mut store,
            &checker,
            &mut registry,
            &capabilities,
            Some(&mut feedback),
            &mut barrier,
            &mut ctx,
            &mut executor,
        );
        let outcome = dispatch_helper(
            HELPER_CALL_BUILTIN_ID,
            &[Value::ObjectRef(object_id), Value::Int(1)],
            &mut env,
        )
        .expect("builtin ok");
        assert_eq!(
            outcome,
            HelperDispatchOutcome::VmControl(VmControl::Normal(Some(Value::ObjectRef(object_id))))
        );
        assert_eq!(feedback.last_builtin_id, Some(3));
        assert_eq!(feedback.last_callee_kind, Some("BuiltinFunction"));
    }

    #[test]
    fn dispatch_call_builtin_rejects_arity() {
        with_env(|env| {
            let shell = env.heap.alloc_function().expect("fn");
            let object_id = shell.id();
            env.callable_registry.register(
                object_id,
                crate::call::callable::CallableTarget::BuiltinFunction(
                    crate::call::callable::BuiltinFunctionTarget {
                        builtin_id: 1,
                        arity: 2,
                        required_capabilities: vec![],
                    },
                ),
            );
            let err = dispatch_helper(
                HELPER_CALL_BUILTIN_ID,
                &[Value::ObjectRef(object_id), Value::Int(1)],
                env,
            )
            .expect_err("arity");
            assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::ArityError));
        });
    }

    // --- H4 control / resource dispatch boundary ---

    #[test]
    fn dispatch_raise_error_and_non_error() {
        with_env(|env| {
            let h = env
                .error_store
                .allocate(vm_core::error::language::ErrorObj::new(
                    RuntimeErrorCode::KeyError,
                    "k",
                ));
            let outcome = dispatch_helper(HELPER_RAISE_ID, &[Value::Error(h)], env).expect("raise");
            assert_eq!(
                outcome,
                HelperDispatchOutcome::VmControl(VmControl::Raise(h))
            );

            let outcome =
                dispatch_helper(HELPER_RAISE_ID, &[Value::Int(1)], env).expect("type raise");
            let HelperDispatchOutcome::VmControl(VmControl::Raise(th)) = outcome else {
                panic!("expected raise");
            };
            assert_eq!(
                env.error_store.get(th).expect("e").error_code,
                RuntimeErrorCode::TypeError
            );
        });
    }

    #[test]
    fn dispatch_attach_suppressed() {
        with_env(|env| {
            let p = env
                .error_store
                .allocate(vm_core::error::language::ErrorObj::new(
                    RuntimeErrorCode::KeyError,
                    "p",
                ));
            let s = env
                .error_store
                .allocate(vm_core::error::language::ErrorObj::new(
                    RuntimeErrorCode::TypeError,
                    "s",
                ));
            let outcome = dispatch_helper(
                HELPER_ATTACH_SUPPRESSED_ID,
                &[Value::Error(p), Value::Error(s)],
                env,
            )
            .expect("attach");
            assert_eq!(outcome, HelperDispatchOutcome::Unit);
            assert_eq!(env.error_store.get(p).expect("p").suppressed, vec![s]);
        });
    }

    #[test]
    fn dispatch_assert_fail() {
        with_env(|env| {
            let outcome = dispatch_helper(
                HELPER_ASSERT_FAIL_ID,
                &[Value::String("nope".into())],
                env,
            )
            .expect("assert");
            let HelperDispatchOutcome::VmControl(VmControl::Raise(h)) = outcome else {
                panic!("raise");
            };
            assert_eq!(
                env.error_store.get(h).expect("e").error_code,
                RuntimeErrorCode::AssertionError
            );
        });
    }

    #[test]
    fn dispatch_register_and_execute_defer() {
        with_env(|env| {
            env.unwind_ctx.push_region(RuntimeRegionFrame::new(
                ControlRegionId::new(0),
                ControlRegionKind::Block,
            ));
            let unit = dispatch_helper(
                HELPER_REGISTER_DEFER_ID,
                &[Value::Int(3), Value::String("d".into())],
                env,
            )
            .expect("reg");
            assert_eq!(unit, HelperDispatchOutcome::Unit);
            assert_eq!(
                env.unwind_ctx
                    .top_region()
                    .expect("top")
                    .cleanup_state
                    .defer_stack
                    .len(),
                1
            );
            let control = dispatch_helper(HELPER_EXECUTE_DEFER_ID, &[Value::Int(3)], env)
                .expect("exec");
            assert_eq!(
                control,
                HelperDispatchOutcome::VmControl(VmControl::Normal(None))
            );
        });
    }

    #[test]
    fn dispatch_register_defer_without_region_fails() {
        with_env(|env| {
            let err = dispatch_helper(HELPER_REGISTER_DEFER_ID, &[Value::Int(1)], env)
                .expect_err("no region");
            assert!(matches!(err, RuntimeFailure::Structural(_)));
        });
    }

    #[test]
    fn dispatch_register_and_close_resource() {
        with_env(|env| {
            env.unwind_ctx.push_region(RuntimeRegionFrame::new(
                ControlRegionId::new(2),
                ControlRegionKind::TryFinally,
            ));
            dispatch_helper(
                HELPER_REGISTER_RESOURCE_ID,
                &[Value::Int(9), Value::String("res".into())],
                env,
            )
            .expect("reg");
            let close = dispatch_helper(HELPER_CLOSE_RESOURCE_ID, &[Value::Int(9)], env)
                .expect("close");
            assert_eq!(
                close,
                HelperDispatchOutcome::VmControl(VmControl::Normal(None))
            );
            let err = dispatch_helper(HELPER_CLOSE_RESOURCE_ID, &[Value::Int(9)], env)
                .expect_err("double");
            assert_eq!(
                err,
                RuntimeFailure::language(RuntimeErrorCode::ResourceStateError)
            );
        });
    }

    // --- H5 module dispatch boundary ---

    fn sample_module_plan(id: u32, export: &str) -> vm_core::runtime_plan::schema::ModulePlan {
        use vm_core::id::{BindingId, EirFunctionId, SlotId};
        use vm_core::runtime_plan::schema::{ExportPlan, ExportPlanEntry};
        use vm_diag::source_span::SourceSpanId;
        vm_core::runtime_plan::schema::ModulePlan {
            module_id: vm_core::id::ModuleId::new(id),
            module_slot_layout: vm_core::id::SlotLayoutId::new(0),
            initialization_function: EirFunctionId::new(0),
            import_plan: Default::default(),
            export_plan: ExportPlan {
                exports: vec![ExportPlanEntry {
                    exported_name: export.to_string(),
                    binding_id: BindingId::new(0),
                    slot_id: SlotId::new(0),
                    interface_type: None,
                    source_span: SourceSpanId::new(0),
                }],
                seal_after_init: true,
            },
            module_state_slot: SlotId::new(0),
            module_object_slot: SlotId::new(1),
            source_order: vec![],
            source_span: None,
        }
    }

    #[test]
    fn dispatch_resolve_initialize_seal_import() {
        use crate::heap::ObjRef;
        use crate::module::resolver::StubModuleResolver;
        use crate::module::runtime::ModuleRuntime;
        use crate::module::state::ModuleState;
        use vm_core::id::{CapabilityId, ModuleId, ObjectId, SlotId};

        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let capabilities = CapabilitySet::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExecutor;

        let mut module_rt = ModuleRuntime::new(CapabilityId::new(1));
        module_rt.capabilities.grant(CapabilityId::new(1));
        let mut resolver = StubModuleResolver::default();
        resolver.map("lib.core", ModuleId::new(1));
        module_rt
            .register_module(
                &sample_module_plan(1, "x"),
                2,
                ObjRef::new(ObjectId::new(1), 0),
            )
            .expect("reg provider");
        module_rt
            .register_module(
                &sample_module_plan(2, "y"),
                2,
                ObjRef::new(ObjectId::new(2), 0),
            )
            .expect("reg importer");

        let mut env = HelperDispatchEnv {
            heap: &mut heap,
            error_store: &mut store,
            type_checker: &checker,
            callable_registry: &mut registry,
            capabilities: &capabilities,
            call_site_feedback: None,
            call_depth: 0,
            max_call_depth: DEFAULT_MAX_CALL_DEPTH,
            module_runtime: Some(&mut module_rt),
            module_resolver: Some(&resolver),
            host_session: None,
            write_barrier: &mut barrier,
            source_span: None,
            unwind_ctx: &mut ctx,
            executor: &mut executor,
        };

        let resolved = dispatch_helper(
            HELPER_RESOLVE_MODULE_ID,
            &[Value::String("lib.core".into())],
            &mut env,
        )
        .expect("resolve");
        assert_eq!(resolved, HelperDispatchOutcome::Value(Value::Int(1)));

        let init = dispatch_helper(HELPER_INITIALIZE_MODULE_ID, &[Value::Int(1)], &mut env)
            .expect("init");
        assert_eq!(
            init,
            HelperDispatchOutcome::VmControl(VmControl::Normal(None))
        );
        assert_eq!(
            env.module_runtime
                .as_ref()
                .unwrap()
                .registry
                .get(ModuleId::new(1))
                .unwrap()
                .state,
            ModuleState::Initializing
        );

        // mark export initialized and write value for import
        {
            let rt = env.module_runtime.as_mut().unwrap();
            rt.registry
                .get_mut(ModuleId::new(1))
                .unwrap()
                .export_table
                .mark_initialized("x")
                .expect("mark");
            rt.registry
                .get_mut(ModuleId::new(1))
                .unwrap()
                .module_slots
                .write(SlotId::new(0), Value::Int(42))
                .expect("write");
            rt.complete_initialization(ModuleId::new(1)).expect("done");
        }

        let named = dispatch_helper(
            HELPER_IMPORT_NAMED_ID,
            &[
                Value::Int(2),
                Value::Int(1),
                Value::String("x".into()),
                Value::Int(0),
            ],
            &mut env,
        )
        .expect("import named");
        assert_eq!(named, HelperDispatchOutcome::Value(Value::Int(42)));

        let whole = dispatch_helper(
            HELPER_IMPORT_MODULE_ID,
            &[Value::Int(2), Value::Int(1), Value::Int(1)],
            &mut env,
        )
        .expect("import module");
        assert!(matches!(
            whole,
            HelperDispatchOutcome::Value(Value::ObjectRef(_))
        ));

        // re-init path not needed; seal provider again would fail if already sealed via finish
        // seal a fresh module 2 exports
        let seal = dispatch_helper(HELPER_SEAL_EXPORTS_ID, &[Value::Int(2)], &mut env).expect("seal");
        assert_eq!(seal, HelperDispatchOutcome::Unit);
        assert!(env
            .module_runtime
            .as_ref()
            .unwrap()
            .registry
            .get(ModuleId::new(2))
            .unwrap()
            .export_table
            .is_sealed());
    }

    #[test]
    fn dispatch_resolve_capability_and_missing_runtime() {
        with_env(|env| {
            let err = dispatch_helper(
                HELPER_RESOLVE_MODULE_ID,
                &[Value::String("x".into())],
                env,
            )
            .expect_err("no runtime");
            assert!(matches!(err, RuntimeFailure::Structural(_)));
        });
    }

    #[test]
    fn dispatch_import_cycle_negative() {
        use crate::heap::ObjRef;
        use crate::module::runtime::ModuleRuntime;
        use vm_core::id::{CapabilityId, ModuleId, ObjectId};

        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let capabilities = CapabilitySet::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExecutor;
        let mut module_rt = ModuleRuntime::new(CapabilityId::new(0));
        module_rt
            .register_module(
                &sample_module_plan(1, "x"),
                2,
                ObjRef::new(ObjectId::new(1), 0),
            )
            .expect("p");
        module_rt
            .register_module(
                &sample_module_plan(2, "y"),
                2,
                ObjRef::new(ObjectId::new(2), 0),
            )
            .expect("i");
        module_rt.begin_loading(ModuleId::new(1)).expect("l");
        module_rt.begin_initializing(ModuleId::new(1)).expect("i");

        let mut env = HelperDispatchEnv {
            heap: &mut heap,
            error_store: &mut store,
            type_checker: &checker,
            callable_registry: &mut registry,
            capabilities: &capabilities,
            call_site_feedback: None,
            call_depth: 0,
            max_call_depth: DEFAULT_MAX_CALL_DEPTH,
            module_runtime: Some(&mut module_rt),
            module_resolver: None,
            host_session: None,
            write_barrier: &mut barrier,
            source_span: None,
            unwind_ctx: &mut ctx,
            executor: &mut executor,
        };
        let err = dispatch_helper(
            HELPER_IMPORT_NAMED_ID,
            &[
                Value::Int(2),
                Value::Int(1),
                Value::String("x".into()),
                Value::Int(0),
            ],
            &mut env,
        )
        .expect_err("cycle");
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::ImportCycleError)
        );
    }

    // --- H6 capability / host boundary ---

    #[test]
    fn dispatch_check_capability_and_host_enter_exit() {
        use crate::helpers::h6::HostBoundarySession;
        use vm_core::id::{CapabilityId, ObjectId};

        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let mut capabilities = CapabilitySet::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExecutor;
        let mut host_session = HostBoundarySession::new();

        let mut env = HelperDispatchEnv {
            heap: &mut heap,
            error_store: &mut store,
            type_checker: &checker,
            callable_registry: &mut registry,
            capabilities: &capabilities,
            call_site_feedback: None,
            call_depth: 0,
            max_call_depth: DEFAULT_MAX_CALL_DEPTH,
            module_runtime: None,
            module_resolver: None,
            host_session: Some(&mut host_session),
            write_barrier: &mut barrier,
            source_span: None,
            unwind_ctx: &mut ctx,
            executor: &mut executor,
        };

        let miss = dispatch_helper(HELPER_CHECK_CAPABILITY_ID, &[Value::Int(4)], &mut env)
            .expect_err("miss");
        assert_eq!(
            miss,
            RuntimeFailure::language(RuntimeErrorCode::CapabilityError)
        );

        // grant via separate mutable capabilities is awkward with shared ref;
        // rebuild env with granted capability
        drop(env);
        capabilities.grant(CapabilityId::new(4));
        let mut env = HelperDispatchEnv {
            heap: &mut heap,
            error_store: &mut store,
            type_checker: &checker,
            callable_registry: &mut registry,
            capabilities: &capabilities,
            call_site_feedback: None,
            call_depth: 0,
            max_call_depth: DEFAULT_MAX_CALL_DEPTH,
            module_runtime: None,
            module_resolver: None,
            host_session: Some(&mut host_session),
            write_barrier: &mut barrier,
            source_span: None,
            unwind_ctx: &mut ctx,
            executor: &mut executor,
        };
        let ok = dispatch_helper(HELPER_CHECK_CAPABILITY_ID, &[Value::Int(4)], &mut env)
            .expect("cap");
        assert_eq!(ok, HelperDispatchOutcome::Unit);

        let enter = dispatch_helper(
            HELPER_ENTER_HOST_CALL_ID,
            &[Value::Int(1), Value::ObjectRef(ObjectId::new(9))],
            &mut env,
        )
        .expect("enter");
        assert_eq!(enter, HelperDispatchOutcome::Unit);
        assert!(env.host_session.as_ref().unwrap().active);
        assert_eq!(env.host_session.as_ref().unwrap().call_scoped_roots.len(), 1);

        let exit = dispatch_helper(
            HELPER_EXIT_HOST_CALL_ID,
            &[Value::Int(0), Value::String("ok".into())],
            &mut env,
        )
        .expect("exit");
        assert_eq!(
            exit,
            HelperDispatchOutcome::VmControl(VmControl::Normal(Some(Value::String("ok".into()))))
        );
        assert!(!env.host_session.as_ref().unwrap().active);
        assert!(env.host_session.as_ref().unwrap().call_scoped_roots.is_empty());
    }

    #[test]
    fn dispatch_exit_host_error_paths() {
        use crate::helpers::h6::HostBoundarySession;

        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let capabilities = CapabilitySet::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExecutor;
        let mut host_session = HostBoundarySession::new();
        let mut env = HelperDispatchEnv {
            heap: &mut heap,
            error_store: &mut store,
            type_checker: &checker,
            callable_registry: &mut registry,
            capabilities: &capabilities,
            call_site_feedback: None,
            call_depth: 0,
            max_call_depth: DEFAULT_MAX_CALL_DEPTH,
            module_runtime: None,
            module_resolver: None,
            host_session: Some(&mut host_session),
            write_barrier: &mut barrier,
            source_span: None,
            unwind_ctx: &mut ctx,
            executor: &mut executor,
        };
        dispatch_helper(HELPER_ENTER_HOST_CALL_ID, &[], &mut env).expect("enter");
        let raise = dispatch_helper(
            HELPER_EXIT_HOST_CALL_ID,
            &[Value::Int(1), Value::String("boom".into())],
            &mut env,
        )
        .expect("lang err");
        assert!(matches!(
            raise,
            HelperDispatchOutcome::VmControl(VmControl::Raise(_))
        ));

        dispatch_helper(HELPER_ENTER_HOST_CALL_ID, &[], &mut env).expect("enter2");
        let structural = dispatch_helper(
            HELPER_EXIT_HOST_CALL_ID,
            &[Value::Int(2), Value::String("structural:x".into())],
            &mut env,
        )
        .expect_err("struct");
        assert!(matches!(structural, RuntimeFailure::Structural(_)));
    }

    #[test]
    fn undispatched_helper_returns_invalid_helper_error() {
        // id 28 = helper_match_pattern remains outside H1–H6 milestones.
        with_env(|env| {
            let err = dispatch_helper(RuntimeHelperId::new(28), &[], env).expect_err("reject");
            assert!(matches!(err, RuntimeFailure::Structural(_)));
        });
    }
}
