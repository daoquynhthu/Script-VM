//! Diagnostic / source-mapping oriented tests (Stage 13 / WP-18).

#[cfg(test)]
mod tests {
    use vm_core::error::language::ErrorStore;
    use vm_core::error::registry::RuntimeErrorCode;
    use vm_core::value::Value;
    use vm_diag::source_span::SourceSpanId;
    use vm_runtime::call::callable::CallableRegistry;
    use vm_runtime::call::contract::StubTypeContractChecker;
    use vm_runtime::control::PendingControl;
    use vm_runtime::heap::Heap;
    use vm_runtime::helpers::dispatch::{
        dispatch_helper, HelperDispatchEnv, HelperDispatchOutcome, DEFAULT_MAX_CALL_DEPTH,
        HELPER_CONSTRUCT_ERROR_ID,
    };
    use vm_runtime::module::resolver::CapabilitySet;
    use vm_runtime::unwind::combine::CleanupStepResult;
    use vm_runtime::unwind::{UnwindContext, UnwindExecutor};
    use vm_runtime::write_barrier::NoopWriteBarrierHook;

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

    /// DG-01: construct_error attaches message and optional source span.
    #[test]
    fn dg01_construct_error_preserves_code_message_and_span() {
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let capabilities = CapabilitySet::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExec;
        // TypeError index in RuntimeErrorCode::ALL is 2
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
            shape_registry: None,            cell_slots: None, prepared_call: None, write_barrier: &mut barrier,
            source_span: Some(SourceSpanId::new(42)),
            unwind_ctx: &mut ctx,
            executor: &mut executor,
        };
        let out = dispatch_helper(
            HELPER_CONSTRUCT_ERROR_ID,
            &[Value::Int(2), Value::String("diagnostic".into())],
            &mut env,
        )
        .expect("DG-01");
        let HelperDispatchOutcome::Value(Value::Error(handle)) = out else {
            panic!("expected error value");
        };
        let err = store.get(handle).expect("stored");
        assert_eq!(err.error_code, RuntimeErrorCode::TypeError);
        assert_eq!(err.message, "diagnostic");
        assert_eq!(err.source_span, Some(SourceSpanId::new(42)));
    }
}
