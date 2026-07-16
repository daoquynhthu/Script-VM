//! WP-19 / Stage 14 integration regression suite (TR-018).
//!
//! Cross-subsystem checks that the bootstrap Phase 3 VM remains a coherent
//! minimal candidate: validation, helpers, cache, host, freeze boundaries.

#[cfg(test)]
mod tests {
    use vm_core::control::ControlState;
    use vm_core::eir::fixtures::{minimal_eir_validation_context, minimal_valid_eir_module};
    use vm_core::eir::validate::{validate_eir_module, EirModuleInput};
    use vm_core::error::language::ErrorStore;
    use vm_core::error::registry::{RuntimeErrorCode, VmStructuralErrorCode};
    use vm_core::id::{CapabilityId, EirFunctionId, RuntimeHelperId};
    use vm_core::runtime_plan::fixtures::minimal_valid_plan;
    use vm_core::runtime_plan::validate_runtime_plan;
    use vm_core::value::Value;
    use vm_eval::interpreter::Interpreter;
    use vm_host::{
        execute_host_call, ArityShape, HostBoundaryId, HostCallContext, HostCallResult,
        HostCallable, HostFunctionDescriptor, HostFunctionId, HostFunctionWrapper,
        HostParameterPolicy, HostResultPolicy, NormalizedHostError,
    };
    use vm_runtime::cache_compat::{
        collect_digest_inputs, reject_helper_registry_mismatch, reject_profile_mismatch,
        reject_public_bytecode_cache_claim,
    };
    use vm_runtime::call::callable::CallableRegistry;
    use vm_runtime::call::contract::StubTypeContractChecker;
    use vm_runtime::control::PendingControl;
    use vm_runtime::heap::Heap;
    use vm_runtime::helpers::dispatch::{
        dispatch_helper, HelperDispatchEnv, DEFAULT_MAX_CALL_DEPTH, HELPER_CHECK_CAPABILITY_ID,
    };
    use vm_runtime::helpers::RuntimeHelperRegistry;
    use vm_runtime::module::resolver::CapabilitySet;
    use vm_runtime::runtime_error::RuntimeFailure;
    use vm_runtime::unwind::combine::CleanupStepResult;
    use vm_runtime::unwind::{UnwindContext, UnwindExecutor};
    use vm_runtime::write_barrier::NoopWriteBarrierHook;
    use vm_core::digest::Digest;

    struct NoopExec;
    impl UnwindExecutor for NoopExec {
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

    /// IG-01: full positive path — plan + EIR validate, interpreter returns.
    #[test]
    fn ig01_validated_eir_executes_on_interpreter() {
        validate_runtime_plan(&minimal_valid_plan()).expect("plan");
        let ctx = minimal_eir_validation_context();
        let module = minimal_valid_eir_module();
        validate_eir_module(EirModuleInput::Resolved(&module), &ctx).expect("eir");
        let mut interp = Interpreter::new();
        let state = interp.run_module(&module, EirFunctionId::new(0));
        assert_eq!(state, ControlState::Return(Some(Value::Int(0))));
    }

    /// IG-02: helper registry is complete (47) and digest is stable across collect.
    #[test]
    fn ig02_helper_registry_complete_and_digest_stable() {
        let reg = RuntimeHelperRegistry::canonical().expect("reg");
        assert_eq!(reg.helper_ids().count(), 47);
        let plan = minimal_valid_plan();
        let inputs = collect_digest_inputs(&plan);
        assert_eq!(inputs.helper_registry_digest, reg.digest());
        reject_helper_registry_mismatch(reg.digest(), inputs.helper_registry_digest)
            .expect("match");
    }

    /// IG-03: public bytecode cache claim forbidden (freeze boundary).
    #[test]
    fn ig03_public_bytecode_cache_forbidden() {
        reject_public_bytecode_cache_claim(false).expect("internal ok");
        let err = reject_public_bytecode_cache_claim(true).expect_err("public");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    /// IG-04: profile mismatch invalidates caches.
    #[test]
    fn ig04_profile_mismatch_rejects_cache_reuse() {
        let err = reject_profile_mismatch(0xA, 0xB).expect_err("mismatch");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    /// IG-05: unregistered helper id rejected at dispatch (no unregistered helper).
    #[test]
    fn ig05_unregistered_helper_rejected() {
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let capabilities = CapabilitySet::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExec;
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
            host_session: None,
            shape_registry: None,
            cell_slots: None,
            prepared_call: None,
            write_barrier: &mut barrier,
            source_span: None,
            unwind_ctx: &mut ctx,
            executor: &mut executor,
        };
        let err = dispatch_helper(RuntimeHelperId::new(99), &[], &mut env).expect_err("oor");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    /// IG-06: capability required for gated helper (no capability bypass).
    #[test]
    fn ig06_capability_required_for_gated_helper() {
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let capabilities = CapabilitySet::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExec;
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
            host_session: None,
            shape_registry: None,
            cell_slots: None,
            prepared_call: None,
            write_barrier: &mut barrier,
            source_span: None,
            unwind_ctx: &mut ctx,
            executor: &mut executor,
        };
        let err = dispatch_helper(HELPER_CHECK_CAPABILITY_ID, &[Value::Int(1)], &mut env)
            .expect_err("cap");
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::CapabilityError)
        );
    }

    /// IG-07: host call without capability rejected (no host boundary bypass).
    #[test]
    fn ig07_host_call_requires_capability() {
        struct CapHost;
        impl HostCallable for CapHost {
            fn invoke(&self, _: &[Value]) -> HostCallResult {
                HostCallResult::Return(Value::Int(1))
            }
        }
        let wrapper = HostFunctionWrapper {
            host_function_id: HostFunctionId(0),
            descriptor: HostFunctionDescriptor {
                arity: ArityShape { min: 0, max: None },
                parameter_policy: HostParameterPolicy::VmValues,
                result_policy: HostResultPolicy::VmValue,
                may_allocate: false,
                may_raise: false,
                may_block: false,
                may_reenter_vm: false,
                requires_roots_visible: false,
            },
            capability: Some(CapabilityId::new(7)),
            effect: None,
            source_span: None,
        };
        let mut ctx = HostCallContext::new(HostBoundaryId(0));
        let mut store = ErrorStore::new();
        let err =
            execute_host_call(&wrapper, &CapHost, &[], &mut ctx, &mut store).expect_err("no cap");
        assert!(matches!(err, NormalizedHostError::Raise(_)));
    }

    /// IG-08: language vs structural error layers remain distinct (registry discipline).
    #[test]
    fn ig08_error_layers_distinct() {
        assert_eq!(RuntimeErrorCode::ALL.len(), 19);
        assert_eq!(VmStructuralErrorCode::ALL.len(), 10);
        assert_ne!(
            format!("{:?}", RuntimeErrorCode::TypeError),
            format!("{:?}", VmStructuralErrorCode::InvalidEirError)
        );
    }

    /// IG-09: helper registry digest mismatch cannot silently reuse cache.
    #[test]
    fn ig09_stale_helper_digest_rejected() {
        let reg = RuntimeHelperRegistry::canonical().expect("reg");
        let err =
            reject_helper_registry_mismatch(Digest(0x1111), reg.digest()).expect_err("stale");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    /// IG-10: double full matrix smoke — interpreter still works after registry use.
    #[test]
    fn ig10_registry_then_interpreter_still_coherent() {
        let _ = RuntimeHelperRegistry::canonical().expect("reg");
        let mut interp = Interpreter::new();
        let state = interp.run_module(&minimal_valid_eir_module(), EirFunctionId::new(0));
        assert_eq!(state, ControlState::Return(Some(Value::Int(0))));
    }
}
