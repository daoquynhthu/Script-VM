//! Positive conformance matrix (Stage 13 / WP-18).
//!
//! Spec: PHASE-3-VALIDATION-MATRIX.md, TRACEABILITY-MATRIX.md (WP-18)

#[cfg(test)]
mod tests {
    use vm_core::control::ControlState;
    use vm_core::eir::fixtures::{minimal_eir_validation_context, minimal_valid_eir_module};
    use vm_core::eir::validate::{validate_eir_module, EirModuleInput};
    use vm_core::id::{CapabilityId, EirFunctionId, ModuleId, ObjectId, ShapeId};
    use vm_core::runtime_plan::fixtures::minimal_valid_plan;
    use vm_core::runtime_plan::validate_runtime_plan;
    use vm_core::value::Value;
    use vm_eval::interpreter::Interpreter;
    use vm_runtime::call::callable::CallableRegistry;
    use vm_runtime::call::contract::StubTypeContractChecker;
    use vm_runtime::control::PendingControl;
    use vm_runtime::heap::Heap;
    use vm_runtime::helpers::dispatch::{
        dispatch_helper, HelperDispatchEnv, HelperDispatchOutcome, DEFAULT_MAX_CALL_DEPTH,
        HELPER_CHECK_SHAPE_ID, HELPER_EXIT_HOST_CALL_ID, HELPER_ENTER_HOST_CALL_ID,
        HELPER_CHECK_CAPABILITY_ID, HELPER_INITIALIZE_MODULE_ID, HELPER_RESOLVE_MODULE_ID,
    };
    use vm_runtime::helpers::h6::HostBoundarySession;
    use vm_runtime::helpers::h7::{ShapeKind, ShapeRegistry};
    use vm_runtime::module::resolver::{CapabilitySet, StubModuleResolver};
    use vm_runtime::module::runtime::ModuleRuntime;
    use vm_runtime::module::state::ModuleState;
    use vm_runtime::readonly::{readonly_view, assert_mutable_list_target};
    use vm_runtime::heap::ObjRef;
    use vm_runtime::unwind::{UnwindContext, UnwindExecutor};
    use vm_runtime::unwind::combine::CleanupStepResult;
    use vm_runtime::write_barrier::NoopWriteBarrierHook;
    use vm_core::error::language::ErrorStore;
    use vm_core::runtime_plan::schema::{ExportPlan, ExportPlanEntry, ModulePlan};
    use vm_core::id::{BindingId, SlotId, SlotLayoutId};
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

    /// CF-01: minimal EIR validates.
    #[test]
    fn cf01_minimal_eir_module_validates() {
        let ctx = minimal_eir_validation_context();
        let module = minimal_valid_eir_module();
        validate_eir_module(EirModuleInput::Resolved(&module), &ctx).expect("CF-01");
    }

    /// CF-02: minimal RuntimePlan validates.
    #[test]
    fn cf02_minimal_runtime_plan_validates() {
        validate_runtime_plan(&minimal_valid_plan()).expect("CF-02");
    }

    /// CF-03: interpreter returns constant from fixture module.
    #[test]
    fn cf03_interpreter_minimal_execution_returns_constant() {
        let mut interpreter = Interpreter::new();
        let module = minimal_valid_eir_module();
        let state = interpreter.run_module(&module, EirFunctionId::new(0));
        assert_eq!(state, ControlState::Return(Some(Value::Int(0))));
    }

    /// CF-04: dispatched shape check succeeds for matching record.
    #[test]
    fn cf04_helper_check_shape_positive() {
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let capabilities = CapabilitySet::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExec;
        let mut shapes = ShapeRegistry::new();
        shapes.register(ShapeId::new(1), ShapeKind::Record { field_count: 1 });
        let rec = heap
            .alloc_record_instance(vec![Value::Int(1)], false)
            .expect("rec");
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
            shape_registry: Some(&shapes),            cell_slots: None, prepared_call: None, write_barrier: &mut barrier,
            source_span: None,
            unwind_ctx: &mut ctx,
            executor: &mut executor,
        };
        let out = dispatch_helper(
            HELPER_CHECK_SHAPE_ID,
            &[Value::ObjectRef(rec.id()), Value::Int(1)],
            &mut env,
        )
        .expect("CF-04");
        assert_eq!(out, HelperDispatchOutcome::Value(Value::Bool(true)));
    }

    /// CF-05: module resolve + initialize advances to Initializing.
    #[test]
    fn cf05_module_resolve_and_initialize() {
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let capabilities = CapabilitySet::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExec;
        let mut module_rt = ModuleRuntime::new(CapabilityId::new(1));
        module_rt.capabilities.grant(CapabilityId::new(1));
        let mut resolver = StubModuleResolver::default();
        resolver.map("app.main", ModuleId::new(1));
        let plan = ModulePlan {
            module_id: ModuleId::new(1),
            module_slot_layout: SlotLayoutId::new(0),
            initialization_function: EirFunctionId::new(0),
            import_plan: Default::default(),
            export_plan: ExportPlan {
                exports: vec![ExportPlanEntry {
                    exported_name: "x".into(),
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
            .register_module(&plan, 2, ObjRef::new(ObjectId::new(1), 0))
            .expect("reg");
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
            shape_registry: None,            cell_slots: None, prepared_call: None, write_barrier: &mut barrier,
            source_span: None,
            unwind_ctx: &mut ctx,
            executor: &mut executor,
        };
        let resolved = dispatch_helper(
            HELPER_RESOLVE_MODULE_ID,
            &[Value::String("app.main".into())],
            &mut env,
        )
        .expect("resolve");
        assert_eq!(resolved, HelperDispatchOutcome::Value(Value::Int(1)));
        dispatch_helper(HELPER_INITIALIZE_MODULE_ID, &[Value::Int(1)], &mut env).expect("init");
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
    }

    /// CF-06: host enter/exit with capability check.
    #[test]
    fn cf06_host_boundary_capability_enter_exit() {
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut registry = CallableRegistry::new();
        let mut capabilities = CapabilitySet::new();
        capabilities.grant(CapabilityId::new(2));
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExec;
        let mut host = HostBoundarySession::new();
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
            host_session: Some(&mut host),
            shape_registry: None,            cell_slots: None, prepared_call: None, write_barrier: &mut barrier,
            source_span: None,
            unwind_ctx: &mut ctx,
            executor: &mut executor,
        };
        dispatch_helper(HELPER_CHECK_CAPABILITY_ID, &[Value::Int(2)], &mut env).expect("cap");
        dispatch_helper(HELPER_ENTER_HOST_CALL_ID, &[Value::Int(0)], &mut env).expect("enter");
        let out = dispatch_helper(
            HELPER_EXIT_HOST_CALL_ID,
            &[Value::Int(0), Value::Int(42)],
            &mut env,
        )
        .expect("exit");
        assert!(matches!(
            out,
            HelperDispatchOutcome::VmControl(vm_runtime::control::VmControl::Normal(Some(
                Value::Int(42)
            )))
        ));
    }

    /// CF-07: ReadOnlyView rejects mutation of viewed list.
    #[test]
    fn cf07_readonly_view_rejects_mutation() {
        let mut heap = Heap::new();
        let list = heap.alloc_list(vec![Value::Int(1)], false).expect("list");
        let view = readonly_view(&mut heap, Value::ObjectRef(list.id()), None).expect("view");
        let err = assert_mutable_list_target(view, &heap).expect_err("readonly");
        assert!(matches!(
            err,
            vm_runtime::runtime_error::RuntimeFailure::Language(
                vm_core::error::registry::RuntimeErrorCode::ReadOnlyError
            )
        ));
    }

    /// CF-10: `readonly(x) is x` is false; `readonly(x) == x` may be true (ISSUE-009).
    #[test]
    fn cf10_readonly_identity_and_equality() {
        use vm_runtime::value::{values_equal, values_identical};
        let mut heap = Heap::new();
        let list = heap.alloc_list(vec![Value::Int(5)], false).expect("list");
        let target = Value::ObjectRef(list.id());
        let view = readonly_view(&mut heap, target.clone(), None).expect("view");
        let view_val = Value::ObjectRef(view.id());
        assert!(!values_identical(&view_val, &target));
        assert!(values_equal(&view_val, &target, &heap).expect("eq"));
    }

    /// CF-12: distinct maps with same entries are equal, not identical.
    #[test]
    fn cf12_map_structural_equality() {
        use vm_runtime::value::{values_equal, values_identical};
        let mut heap = Heap::new();
        let m1 = heap.alloc_map(false).expect("m1");
        heap.map_insert(m1, Value::String("k".into()), Value::Int(1))
            .expect("ins");
        let m2 = heap.alloc_map(false).expect("m2");
        heap.map_insert(m2, Value::String("k".into()), Value::Int(1))
            .expect("ins");
        let a = Value::ObjectRef(m1.id());
        let b = Value::ObjectRef(m2.id());
        assert!(values_equal(&a, &b, &heap).expect("eq"));
        assert!(!values_identical(&a, &b));
    }

    /// CF-11: all 47 helpers resolve through registry + central dispatch negative for id 99.
    #[test]
    fn cf11_helper_registry_has_47_and_out_of_range_rejected() {
        use vm_core::id::RuntimeHelperId;
        use vm_runtime::helpers::dispatch::{dispatch_helper, DEFAULT_MAX_CALL_DEPTH};
        use vm_runtime::helpers::RuntimeHelperRegistry;
        use vm_runtime::runtime_error::RuntimeFailure;
        let registry = RuntimeHelperRegistry::canonical().expect("reg");
        assert_eq!(registry.helper_ids().count(), 47);
        let mut heap = Heap::new();
        let mut store = ErrorStore::new();
        let checker = StubTypeContractChecker::new();
        let mut callable = CallableRegistry::new();
        let capabilities = CapabilitySet::new();
        let mut barrier = NoopWriteBarrierHook;
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(None));
        let mut executor = NoopExec;
        let mut env = HelperDispatchEnv {
            heap: &mut heap,
            error_store: &mut store,
            type_checker: &checker,
            callable_registry: &mut callable,
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

    /// CF-13: map equality holds when one side is a ReadOnlyView of the other (TR-007 §5).
    #[test]
    fn cf13_map_equality_through_readonly_view() {
        use vm_runtime::value::{values_equal, values_identical};
        let mut heap = Heap::new();
        let m = heap.alloc_map(false).expect("map");
        heap.map_insert(m, Value::String("k".into()), Value::Int(1))
            .expect("ins");
        let target = Value::ObjectRef(m.id());
        let view = readonly_view(&mut heap, target.clone(), None).expect("view");
        let view_val = Value::ObjectRef(view.id());
        assert!(values_equal(&view_val, &target, &heap).expect("eq"));
        assert!(!values_identical(&view_val, &target));
    }

    /// CF-14: string scalar length and in-bounds slice (TR-006).
    #[test]
    fn cf14_string_scalar_len_and_slice() {
        use vm_runtime::value::{string_scalar_len, string_slice};
        assert_eq!(string_scalar_len("a🙂b"), 3);
        assert_eq!(string_slice("a🙂b", 1, 2).expect("slice"), "🙂");
        assert_eq!(string_slice("abc", 0, 3).expect("full"), "abc");
    }

    /// CF-15: mutating original aggregate is visible through ReadOnlyView read (TR-007).
    #[test]
    fn cf15_readonly_view_reflects_original_mutation() {
        use vm_core::id::FieldIndex;
        use vm_runtime::readonly::readonly_read_field;
        let mut heap = Heap::new();
        let rec = heap
            .alloc_record_instance(vec![Value::Int(1)], false)
            .expect("rec");
        let view = readonly_view(&mut heap, Value::ObjectRef(rec.id()), None).expect("view");
        assert_eq!(
            readonly_read_field(&heap, view, FieldIndex(0)).expect("r0"),
            Value::Int(1)
        );
        // Mutate original record field storage via heap resolve_mut path used by helpers.
        {
            use vm_runtime::heap::object::HeapObject;
            let object = heap.resolve_mut(rec).expect("mut");
            let HeapObject::RecordInstance { fields, .. } = object else {
                panic!("record");
            };
            fields[0] = Value::Int(99);
        }
        assert_eq!(
            readonly_read_field(&heap, view, FieldIndex(0)).expect("r1"),
            Value::Int(99)
        );
    }

    /// CF-16: float NaN equals NaN under values_equal; identity uses bit pattern.
    #[test]
    fn cf16_float_nan_equality() {
        use vm_runtime::value::{values_equal, values_identical};
        let heap = Heap::new();
        let a = Value::Float(f64::NAN);
        let b = Value::Float(f64::NAN);
        assert!(values_equal(&a, &b, &heap).expect("eq"));
        // Both NaNs share quiet-NaN bit pattern from f64::NAN on this platform path.
        assert!(values_identical(&a, &b));
        assert!(!values_equal(&a, &Value::Float(1.0), &heap).expect("neq"));
    }

    /// CF-17: argument binding success path (TR-011 positive counterpart).
    #[test]
    fn cf17_bind_arguments_positional_and_optional_default() {
        use vm_core::id::SlotId;
        use vm_runtime::call::{bind_arguments, ParameterSpec};
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
        let binding = bind_arguments(&params, &[Value::Int(1)], &[]).expect("CF-17");
        assert_eq!(binding.bound, vec![(SlotId::new(0), Value::Int(1))]);
        assert_eq!(binding.pending_default_indices, vec![1]);
    }

    /// CF-18: cache digest inputs include canonical helper registry (TR-014).
    #[test]
    fn cf18_cache_digest_includes_helper_registry() {
        use vm_core::runtime_plan::fixtures::minimal_valid_plan;
        use vm_runtime::cache_compat::{collect_digest_inputs, reject_public_bytecode_cache_claim};
        use vm_runtime::helpers::RuntimeHelperRegistry;
        let inputs = collect_digest_inputs(&minimal_valid_plan());
        let reg = RuntimeHelperRegistry::canonical().expect("reg");
        assert_eq!(inputs.helper_registry_digest, reg.digest());
        reject_public_bytecode_cache_claim(false).expect("internal ok");
    }
}
