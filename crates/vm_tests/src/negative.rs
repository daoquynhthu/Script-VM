//! Negative conformance matrix (Stage 13 / WP-18).

#[cfg(test)]
mod tests {
    use vm_core::eir::fixtures::{
        eir_module_with_unknown_shape_id, field_bound_validation_context,
        minimal_eir_validation_context, shape_bound_validation_context,
    };
    use vm_core::eir::validate::{validate_eir_module, EirModuleInput, EirValidationError};
    use vm_core::error::language::ErrorStore;
    use vm_core::error::registry::RuntimeErrorCode;
    use vm_core::id::{CapabilityId, ModuleId, ObjectId, RuntimeHelperId};
    use vm_core::value::Value;
    use vm_runtime::call::callable::CallableRegistry;
    use vm_runtime::call::contract::StubTypeContractChecker;
    use vm_runtime::control::PendingControl;
    use vm_runtime::heap::Heap;
    use vm_runtime::heap::ObjRef;
    use vm_runtime::helpers::dispatch::{
        dispatch_helper, HelperDispatchEnv, DEFAULT_MAX_CALL_DEPTH, HELPER_CHECK_CAPABILITY_ID,
        HELPER_IMPORT_NAMED_ID,
    };
    use vm_runtime::module::resolver::CapabilitySet;
    use vm_runtime::module::runtime::ModuleRuntime;
    use vm_runtime::runtime_error::RuntimeFailure;
    use vm_runtime::unwind::combine::CleanupStepResult;
    use vm_runtime::unwind::{UnwindContext, UnwindExecutor};
    use vm_runtime::write_barrier::NoopWriteBarrierHook;
    use vm_core::id::{BindingId, EirFunctionId, SlotId, SlotLayoutId};
    use vm_core::runtime_plan::schema::{ExportPlan, ExportPlanEntry, ModulePlan};
    use vm_diag::source_span::SourceSpanId;

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

    /// NG-01: unknown shape id rejected by EIR validation.
    #[test]
    fn ng01_unknown_shape_id_rejected() {
        let ctx = shape_bound_validation_context();
        let module = eir_module_with_unknown_shape_id();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::UnknownShapeId(_)));
    }

    /// NG-02: undispatched helper id returns InvalidHelperError.
    #[test]
    fn ng02_undispatched_helper_rejected() {
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
            shape_registry: None,            cell_slots: None,            write_barrier: &mut barrier,
            source_span: None,
            unwind_ctx: &mut ctx,
            executor: &mut executor,
        };
        let err = dispatch_helper(RuntimeHelperId::new(99), &[], &mut env).expect_err("NG-02");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    /// NG-03: missing capability raises CapabilityError.
    #[test]
    fn ng03_missing_capability_rejected() {
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
            shape_registry: None,            cell_slots: None,            write_barrier: &mut barrier,
            source_span: None,
            unwind_ctx: &mut ctx,
            executor: &mut executor,
        };
        let err = dispatch_helper(HELPER_CHECK_CAPABILITY_ID, &[Value::Int(9)], &mut env)
            .expect_err("NG-03");
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::CapabilityError));
        let _ = CapabilityId::new(0);
    }

    /// NG-04: circular uninitialized export → ImportCycleError.
    #[test]
    fn ng04_import_cycle_rejected() {
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let capabilities = CapabilitySet::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExec;
        let mut module_rt = ModuleRuntime::new(CapabilityId::new(0));
        let plan = |id: u32, name: &str| ModulePlan {
            module_id: ModuleId::new(id),
            module_slot_layout: SlotLayoutId::new(0),
            initialization_function: EirFunctionId::new(0),
            import_plan: Default::default(),
            export_plan: ExportPlan {
                exports: vec![ExportPlanEntry {
                    exported_name: name.into(),
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
        };
        module_rt
            .register_module(&plan(1, "x"), 2, ObjRef::new(ObjectId::new(1), 0))
            .expect("p");
        module_rt
            .register_module(&plan(2, "y"), 2, ObjRef::new(ObjectId::new(2), 0))
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
            shape_registry: None,            cell_slots: None,            write_barrier: &mut barrier,
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
        .expect_err("NG-04");
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::ImportCycleError));
    }

    /// NG-05: non-hashable map key rejected.
    #[test]
    fn ng05_non_hashable_map_key_rejected() {
        let mut heap = Heap::new();
        let map = heap.alloc_map(false).expect("map");
        let list = heap.alloc_list(vec![], false).expect("list");
        let err = heap
            .map_insert(map, Value::ObjectRef(list.id()), Value::Int(1))
            .expect_err("NG-05");
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::TypeError));
        let _ = field_bound_validation_context();
        let _ = minimal_eir_validation_context();
    }
}
