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
            shape_registry: None,            cell_slots: None, prepared_call: None, write_barrier: &mut barrier,
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
            shape_registry: None,            cell_slots: None, prepared_call: None, write_barrier: &mut barrier,
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
            shape_registry: None,            cell_slots: None, prepared_call: None, write_barrier: &mut barrier,
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

    /// NG-06: NaN float cannot be used as map key (TR-006).
    #[test]
    fn ng06_nan_map_key_rejected() {
        let mut heap = Heap::new();
        let map = heap.alloc_map(false).expect("map");
        let err = heap
            .map_insert(map, Value::Float(f64::NAN), Value::Int(1))
            .expect_err("NG-06");
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::TypeError));
    }

    /// NG-07: string slice bounds / negative start → IndexError (TR-006).
    #[test]
    fn ng07_string_slice_bounds_rejected() {
        use vm_runtime::value::string_slice;
        assert_eq!(
            string_slice("abc", -1, 1).unwrap_err(),
            RuntimeFailure::language(RuntimeErrorCode::IndexError)
        );
        assert_eq!(
            string_slice("abc", 0, 4).unwrap_err(),
            RuntimeFailure::language(RuntimeErrorCode::IndexError)
        );
        assert_eq!(
            string_slice("abc", 2, 1).unwrap_err(),
            RuntimeFailure::language(RuntimeErrorCode::IndexError)
        );
    }

    /// NG-08: uninitialized slot read rejected (TR-008).
    #[test]
    fn ng08_uninitialized_slot_read_rejected() {
        use vm_core::id::{FrameId, SlotId};
        use vm_runtime::frame::Frame;
        let frame = Frame::new(FrameId::new(0), 1);
        let err = frame.slots.read(SlotId::new(0)).expect_err("NG-08");
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::UninitializedBindingError)
        );
    }

    /// NG-09: wrong arity rejected (TR-011).
    #[test]
    fn ng09_wrong_arity_rejected() {
        use vm_core::id::SlotId;
        use vm_runtime::call::{bind_arguments, ParameterSpec};
        let params = [ParameterSpec {
            name: "a".into(),
            slot_id: SlotId::new(0),
            required: true,
            default_index: None,
            type_id: None,
        }];
        let err = bind_arguments(&params, &[Value::Int(1), Value::Int(2)], &[])
            .expect_err("NG-09");
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::ArityError));
    }

    /// NG-10: duplicate named argument rejected (TR-011).
    #[test]
    fn ng10_duplicate_named_argument_rejected() {
        use vm_core::id::SlotId;
        use vm_runtime::call::{bind_arguments, NamedArgumentValue, ParameterSpec};
        let params = [
            ParameterSpec {
                name: "a".into(),
                slot_id: SlotId::new(0),
                required: true,
                default_index: None,
                type_id: None,
            },
            ParameterSpec {
                name: "b".into(),
                slot_id: SlotId::new(1),
                required: false,
                default_index: Some(0),
                type_id: None,
            },
        ];
        let named = [
            NamedArgumentValue {
                name: "b".into(),
                value: Value::Int(2),
            },
            NamedArgumentValue {
                name: "b".into(),
                value: Value::Int(3),
            },
        ];
        let err = bind_arguments(&params, &[Value::Int(1)], &named).expect_err("NG-10");
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::ArityError));
    }

    /// NG-11: public-bytecode cache claim rejected (TR-014).
    #[test]
    fn ng11_public_bytecode_cache_claim_rejected() {
        use vm_runtime::cache_compat::reject_public_bytecode_cache_claim;
        let err = reject_public_bytecode_cache_claim(true).expect_err("NG-11");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    /// NG-12: helper registry digest mismatch rejected (TR-014).
    #[test]
    fn ng12_helper_registry_digest_mismatch_rejected() {
        use vm_core::digest::Digest;
        use vm_runtime::cache_compat::{
            reject_helper_registry_mismatch, collect_digest_inputs,
        };
        use vm_core::runtime_plan::fixtures::minimal_valid_plan;
        let expected = collect_digest_inputs(&minimal_valid_plan()).helper_registry_digest;
        let err =
            reject_helper_registry_mismatch(Digest(0xDEAD), expected).expect_err("NG-12");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    /// NG-13: branch condition must be Bool (TR-015).
    #[test]
    fn ng13_interpreter_branch_non_bool_raises() {
        use vm_core::control::ControlState;
        use vm_core::id::EirFunctionId;
        use vm_eval::interpreter::{branch_non_bool_module, Interpreter};
        let mut interpreter = Interpreter::new();
        let state = interpreter.run_module(&branch_non_bool_module(), EirFunctionId::new(0));
        assert!(matches!(state, ControlState::Raise(_)));
    }

    /// NG-14: duplicate export name rejected (TR-010).
    #[test]
    fn ng14_duplicate_export_name_rejected() {
        use vm_core::id::{BindingId, SlotId};
        use vm_core::runtime_plan::schema::{ExportPlan, ExportPlanEntry};
        use vm_runtime::module::validate::validate_export_plan;
        let plan = ExportPlan {
            exports: vec![
                ExportPlanEntry {
                    exported_name: "dup".into(),
                    binding_id: BindingId::new(0),
                    slot_id: SlotId::new(0),
                    interface_type: None,
                    source_span: SourceSpanId::new(0),
                },
                ExportPlanEntry {
                    exported_name: "dup".into(),
                    binding_id: BindingId::new(1),
                    slot_id: SlotId::new(1),
                    interface_type: None,
                    source_span: SourceSpanId::new(1),
                },
            ],
            seal_after_init: true,
        };
        let err = validate_export_plan(&plan).expect_err("NG-14");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    /// NG-15: may-collect helper at safepoint without RootMap rejected (TR-013).
    #[test]
    fn ng15_may_collect_without_root_map_rejected() {
        use vm_core::eir::fixtures::{
            eir_module_with_may_collect_without_root_map, minimal_eir_validation_context,
        };
        use vm_core::eir::validate::{validate_eir_module, EirModuleInput, EirValidationError};
        use vm_core::id::SafepointId;
        use vm_runtime::helpers::eir_validation_view;
        use vm_runtime::helpers::RuntimeHelperRegistry;
        let registry = RuntimeHelperRegistry::canonical().expect("reg");
        let mut ctx = minimal_eir_validation_context();
        ctx.gc_may_run = true;
        ctx.safepoint_ids.insert(SafepointId::new(0));
        ctx.helper_registry = eir_validation_view(&registry);
        let module = eir_module_with_may_collect_without_root_map();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert_eq!(err, EirValidationError::MayCollectWithoutRootMap);
    }

    /// NG-16: host retained heap value without root rejected (TR-012).
    #[test]
    fn ng16_host_retained_value_without_root_rejected() {
        use vm_core::error::registry::VmStructuralErrorCode;
        use vm_host::HostRootRegistry;
        let value = Value::ObjectRef(ObjectId::new(1));
        let err = HostRootRegistry::validate_retention(&value, true).expect_err("NG-16");
        assert_eq!(err.code, VmStructuralErrorCode::BackendViolationError);
    }
}
