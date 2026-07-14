//! WP-18 gap-closure rows: remaining TRACEABILITY test obligations for
//! implemented subsystems (bootstrap Phase 3).
//!
//! Spec anchors: TRACEABILITY-MATRIX TR-002..TR-017, PHASE-3-VALIDATION-MATRIX.md

#[cfg(test)]
mod tests {
    use vm_core::control::ControlState;
    use vm_core::error::language::{ErrorObj, ErrorStore};
    use vm_core::error::registry::{
        ErrorLayer, RuntimeErrorCode, VmStructuralErrorCode,
    };
    use vm_core::error::raise::{validate_raise_operand, RaiseValidation};
    use vm_core::error::VmError;
    use vm_core::eir::fixtures::{
        eir_module_with_invalid_block_graph, eir_module_with_invalid_helper,
        eir_module_with_unknown_constant, minimal_eir_validation_context,
    };
    use vm_core::eir::validate::{validate_eir_module, EirModuleInput, EirValidationError};
    use vm_core::id::{
        EirFunctionId, FrameId, FunctionId, ModuleId, ObjectId, RuntimeHelperId, SlotId, TypeId,
    };
    use vm_core::runtime_plan::fixtures::minimal_valid_plan;
    use vm_core::runtime_plan::validate::{validate_runtime_plan, ValidationError};
    use vm_core::value::Value;
    use vm_eval::interpreter::{
        generic_call_mid_block_module, generic_call_nested_module, helper_alloc_object_module,
        module_init_body_module, slot_copy_module, Interpreter,
    };
    use vm_host::{
        execute_host_call, normalize_host_call_result, ArityShape, HostBoundaryId, HostCallContext,
        HostCallResult, HostCallable, HostFunctionDescriptor, HostFunctionId, HostFunctionWrapper,
        HostParameterPolicy, HostResultPolicy, NormalizedHostError,
    };
    use vm_runtime::call::callable::{CallableTarget, UserFunctionTarget};
    use vm_runtime::call::{
        check_return_contract, ArgumentBinding, ParameterSpec, StubTypeContractChecker,
    };
    use vm_runtime::control::PendingControl;
    use vm_runtime::frame::Frame;
    use vm_runtime::gc::{
        pending_control_roots, GcProfile, RootLocation, RootMap, RootMapTable,
    };
    use vm_runtime::gc::validate::validate_root_map_slots;
    use vm_runtime::helpers::{
        RuntimeHelperDescriptor, RuntimeHelperFamily, RuntimeHelperRegistry, RuntimeHelperSignature,
        HelperCallingConvention, HelperGcBehavior, HelperJitCallPolicy, HelperResultType,
        HelperSourceMappingPolicy, RegistryBuildError,
    };
    use vm_runtime::module::import::reject_failed_provider;
    use vm_runtime::module::instance::ModuleInstance;
    use vm_runtime::module::state::ModuleState;
    use vm_runtime::module::export::ExportTable;
    use vm_runtime::module::instance::ModuleInterfaceDescriptor;
    use vm_runtime::heap::ObjRef;
    use vm_runtime::readonly::readonly_view;
    use vm_runtime::runtime_error::RuntimeFailure;
    use vm_runtime::cache_compat::reject_profile_mismatch;
    use vm_runtime::value::value_to_key;
    use vm_runtime::heap::Heap;
    use vm_runtime::unwind::combine::{combine_cleanup_result, CleanupStepResult};
    use std::collections::BTreeSet;

    // --- CF-08 / CF-09 / CF-24..CF-31 (positive residual) ---

    /// CF-08: nested generic_call user body (TR-011 / TR-015).
    #[test]
    fn cf08_nested_generic_call_user_body() {
        let callee_id = ObjectId::new(1);
        let module = generic_call_nested_module(Value::ObjectRef(callee_id));
        let mut interp = Interpreter::new();
        interp.state_mut().callable_registry.register(
            callee_id,
            CallableTarget::UserFunction(UserFunctionTarget {
                function_id: FunctionId::new(1),
                module_id: ModuleId::new(0),
                entry_eir_function: EirFunctionId::new(1),
                return_type: None,
                object_id: callee_id,
            }),
        );
        let result = interp.run_module(&module, EirFunctionId::new(0));
        assert_eq!(result, ControlState::Return(Some(Value::Int(7))));
    }

    /// CF-09: module init body via run_module_init_function (TR-010 / TR-015).
    #[test]
    fn cf09_module_init_body_executes() {
        let mut interp = Interpreter::new();
        let result = interp.run_module_init_function(&module_init_body_module(), EirFunctionId::new(0));
        assert_eq!(result, ControlState::Return(Some(Value::Int(99))));
    }

    /// CF-24: mid-block resume after nested generic_call (TR-015).
    #[test]
    fn cf24_mid_block_resume_after_generic_call() {
        let callee_id = ObjectId::new(2);
        let module = generic_call_mid_block_module(Value::ObjectRef(callee_id));
        let mut interp = Interpreter::new();
        interp.state_mut().callable_registry.register(
            callee_id,
            CallableTarget::UserFunction(UserFunctionTarget {
                function_id: FunctionId::new(1),
                module_id: ModuleId::new(0),
                entry_eir_function: EirFunctionId::new(1),
                return_type: None,
                object_id: callee_id,
            }),
        );
        let result = interp.run_module(&module, EirFunctionId::new(0));
        assert_eq!(result, ControlState::Return(Some(Value::Int(11))));
    }

    /// CF-25: slot load/store round-trip (TR-015).
    #[test]
    fn cf25_slot_load_store_round_trip() {
        let mut interp = Interpreter::new();
        let result = interp.run_module(&slot_copy_module(), EirFunctionId::new(0));
        assert_eq!(result, ControlState::Return(Some(Value::Int(7))));
    }

    /// CF-26: helper alloc_object via interpreter (TR-005 / TR-015).
    #[test]
    fn cf26_helper_alloc_object_returns_ref() {
        let mut interp = Interpreter::new();
        let result = interp.run_module(&helper_alloc_object_module(), EirFunctionId::new(0));
        assert!(matches!(
            result,
            ControlState::Return(Some(Value::ObjectRef(_)))
        ));
    }

    /// CF-27: error registry language + structural code sets (TR-002).
    #[test]
    fn cf27_error_registry_language_and_structural_sets() {
        assert_eq!(RuntimeErrorCode::ALL.len(), 19);
        assert_eq!(VmStructuralErrorCode::ALL.len(), 10);
        for code in RuntimeErrorCode::ALL {
            assert_eq!(code.layer(), ErrorLayer::LanguageError);
        }
        for code in VmStructuralErrorCode::ALL {
            assert_eq!(code.layer(), ErrorLayer::VmStructuralError);
        }
    }

    /// CF-28: return contract success when type matches (TR-011).
    #[test]
    fn cf28_return_contract_success() {
        let mut checker = StubTypeContractChecker::new();
        checker.declare_int_type(TypeId::new(1));
        check_return_contract(&Value::Int(3), Some(TypeId::new(1)), &checker).expect("CF-28");
    }

    /// CF-29: moving GC accepts updateable precise RootMap (TR-013).
    #[test]
    fn cf29_moving_gc_accepts_updateable_root_map() {
        use vm_core::eir::schema::RootMapOwner;
        use vm_core::id::RootMapId;
        let profile = GcProfile::moving_compacting();
        let map = RootMap {
            root_map_id: RootMapId::new(0),
            owner: RootMapOwner::InterpreterFrame,
            safepoint_id: None,
            frame_map_id: None,
            roots: vec![RootLocation::Slot(SlotId::new(0))],
            source_span: None,
            updateable: true,
        };
        profile.validate_root_map_policy(&map).expect("CF-29");
    }

    /// CF-30: canonical helper registry may_raise / may_collect sets non-empty (TR-005).
    #[test]
    fn cf30_helper_may_raise_and_may_collect_policy() {
        let reg = RuntimeHelperRegistry::canonical().expect("reg");
        assert!(!reg.may_raise_helper_ids().is_empty());
        assert!(!reg.may_collect_helper_ids().is_empty());
        // may-collect helpers are a subset of helpers that can allocate/collect.
        for id in reg.may_collect_helper_ids() {
            let d = reg.lookup(id).expect("desc");
            assert!(d.may_collect());
        }
    }

    /// CF-31: host call with required capability succeeds (TR-012).
    #[test]
    fn cf31_host_call_with_capability_succeeds() {
        struct CapHost;
        impl HostCallable for CapHost {
            fn invoke(&self, _: &[Value]) -> HostCallResult {
                HostCallResult::Return(Value::Int(1))
            }
        }
        let cap = vm_core::id::CapabilityId::new(5);
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
            capability: Some(cap),
            effect: None,
            source_span: None,
        };
        let mut ctx = HostCallContext::new(HostBoundaryId(0));
        ctx.capabilities.grant(cap);
        let mut store = ErrorStore::new();
        let control = execute_host_call(&wrapper, &CapHost, &[], &mut ctx, &mut store).expect("ok");
        assert_eq!(
            control,
            vm_runtime::control::VmControl::Normal(Some(Value::Int(1)))
        );
    }

    /// CF-32: pending control with heap value is root-visible (TR-008 / TR-013).
    #[test]
    fn cf32_pending_control_heap_root_visible() {
        let pending = PendingControl::Return(Some(Value::ObjectRef(ObjectId::new(1))));
        let roots = pending_control_roots(&pending);
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].root_location, RootLocation::PendingControl);
    }

    // --- NG-17..NG-32 (negative residual) ---

    /// NG-17: non-Error raise rejected (TR-002).
    #[test]
    fn ng17_non_error_raise_rejected() {
        let store = ErrorStore::new();
        assert_eq!(
            validate_raise_operand(&Value::Int(42), &store),
            RaiseValidation::RejectedTypeError
        );
    }

    /// NG-18: RuntimePlan unresolved module id (TR-003).
    #[test]
    fn ng18_runtime_plan_unresolved_module() {
        let mut plan = minimal_valid_plan();
        if let Some(function) = plan.function_plans.functions.get_mut(&0) {
            function.module_id = ModuleId::new(99);
        }
        let err = validate_runtime_plan(&plan).unwrap_err();
        assert!(matches!(err, ValidationError::UnresolvedModuleId(_)));
    }

    /// NG-19: RuntimePlan cache profile mismatch (TR-003 / TR-014).
    #[test]
    fn ng19_runtime_plan_cache_profile_mismatch() {
        let mut plan = minimal_valid_plan();
        plan.target_profile.vm_version = vm_core::profile::Version::new(9, 9, 9);
        let err = validate_runtime_plan(&plan).unwrap_err();
        assert_eq!(err, ValidationError::CacheProfileMismatch);
    }

    /// NG-20: EIR invalid block graph (TR-004).
    #[test]
    fn ng20_eir_invalid_block_graph() {
        let module = eir_module_with_invalid_block_graph();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::InvalidBlockGraph(_)));
    }

    /// NG-21: EIR unknown constant id (TR-004).
    #[test]
    fn ng21_eir_unknown_constant() {
        let module = eir_module_with_unknown_constant();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::UnknownConstantId(_)));
    }

    /// NG-22: EIR unknown runtime helper id (TR-004 / TR-005).
    #[test]
    fn ng22_eir_unknown_runtime_helper() {
        let module = eir_module_with_invalid_helper();
        let ctx = minimal_eir_validation_context();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert!(matches!(err, EirValidationError::UnknownRuntimeHelperId(_)));
    }

    /// NG-23: note — unknown op/terminator wire tags covered by `vm_core` unit tests
    /// (`unknown_op_kind_is_rejected`, `block_without_terminator_is_rejected`).
    /// NG-24: duplicate helper id rejected at registry build (TR-005).
    #[test]
    fn ng24_duplicate_helper_id_rejected() {
        let d = RuntimeHelperDescriptor {
            helper_id: RuntimeHelperId::new(0),
            name: "a".into(),
            family: RuntimeHelperFamily::Error,
            signature: RuntimeHelperSignature {
                result: HelperResultType::Unit,
                calling_convention: HelperCallingConvention::InterpreterDirect,
            },
            may_allocate: false,
            may_raise: false,
            may_unwind: false,
            is_safepoint: false,
            requires_roots_visible: false,
            required_capability: None,
            effect: None,
            gc_behavior: HelperGcBehavior::NoAllocation,
            jit_call_policy: HelperJitCallPolicy::InterpreterOnly,
            source_mapping_policy: HelperSourceMappingPolicy::NotRequired,
        };
        let mut d2 = d.clone();
        d2.name = "b".into();
        let err = RuntimeHelperRegistry::from_descriptors(vec![d, d2]).unwrap_err();
        assert!(matches!(err, RegistryBuildError::DuplicateHelperId(_)));
    }

    /// NG-25: invalid slot id rejected (TR-008).
    #[test]
    fn ng25_invalid_slot_id_rejected() {
        let frame = Frame::new(FrameId::new(0), 2);
        let err = frame.slots.read(SlotId::INVALID).expect_err("NG-25");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    /// NG-26: return contract failure (TR-011).
    #[test]
    fn ng26_return_contract_failure() {
        let mut checker = StubTypeContractChecker::new();
        checker.declare_int_type(TypeId::new(1));
        let err = check_return_contract(&Value::String("x".into()), Some(TypeId::new(1)), &checker)
            .expect_err("NG-26");
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::TypeContractError)
        );
        let _ = ArgumentBinding {
            bound: vec![],
            pending_default_indices: vec![],
        };
        let _ = ParameterSpec {
            name: "x".into(),
            slot_id: SlotId::new(0),
            required: true,
            default_index: None,
            type_id: None,
        };
    }

    /// NG-27: host call without capability (TR-012).
    #[test]
    fn ng27_host_call_without_capability() {
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
            capability: Some(vm_core::id::CapabilityId::new(5)),
            effect: None,
            source_span: None,
        };
        let mut ctx = HostCallContext::new(HostBoundaryId(0));
        let mut store = ErrorStore::new();
        let err = execute_host_call(&wrapper, &CapHost, &[], &mut ctx, &mut store)
            .expect_err("NG-27");
        assert!(matches!(err, NormalizedHostError::Raise(_)));
    }

    /// NG-28: cache profile fingerprint mismatch (TR-014).
    #[test]
    fn ng28_cache_profile_mismatch() {
        let err = reject_profile_mismatch(1, 2).expect_err("NG-28");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    /// NG-29: moving GC rejects non-updateable RootMap (TR-013).
    #[test]
    fn ng29_moving_gc_rejects_non_updateable_root_map() {
        use vm_core::eir::schema::RootMapOwner;
        use vm_core::id::RootMapId;
        let profile = GcProfile::moving_compacting();
        let map = RootMap {
            root_map_id: RootMapId::new(0),
            owner: RootMapOwner::InterpreterFrame,
            safepoint_id: None,
            frame_map_id: None,
            roots: vec![RootLocation::Slot(SlotId::new(0))],
            source_span: None,
            updateable: false,
        };
        let err = profile.validate_root_map_policy(&map).expect_err("NG-29");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    /// NG-30: RootMap unknown slot rejected (TR-013).
    #[test]
    fn ng30_root_map_unknown_slot_rejected() {
        use vm_core::eir::schema::RootMapOwner;
        use vm_core::id::RootMapId;
        let mut table = RootMapTable::new();
        table.maps.insert(
            0,
            RootMap {
                root_map_id: RootMapId::new(0),
                owner: RootMapOwner::InterpreterFrame,
                safepoint_id: None,
                frame_map_id: None,
                roots: vec![RootLocation::Slot(SlotId::new(99))],
                source_span: None,
                updateable: true,
            },
        );
        let known = BTreeSet::from([SlotId::new(0)]);
        let err = validate_root_map_slots(&table, &known).expect_err("NG-30");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    /// NG-31: failed module import rejected (TR-010).
    #[test]
    fn ng31_failed_module_import_rejected() {
        let instance = ModuleInstance {
            module_id: ModuleId::new(1),
            state: ModuleState::Failed,
            module_object: ObjRef::new(ObjectId::new(1), 0),
            module_slots: vm_runtime::frame::SlotArray::with_capacity(1),
            export_table: ExportTable::new(),
            interface_descriptor: ModuleInterfaceDescriptor::default(),
            initialization_error: Some(vm_core::id::ErrorHandle::new(0)),
            initialization_function: EirFunctionId::new(0),
            source_span: None,
        };
        let err = reject_failed_provider(&instance).expect_err("NG-31");
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::ImportError));
    }

    /// NG-32: readonly view of list remains non-hashable as map key (TR-007).
    #[test]
    fn ng32_readonly_list_view_non_hashable() {
        let mut heap = Heap::new();
        let list = heap.alloc_list(vec![Value::Int(1)], false).expect("list");
        let view = readonly_view(&mut heap, Value::ObjectRef(list.id()), None).expect("view");
        let err = value_to_key(&Value::ObjectRef(view.id()), &heap).expect_err("NG-32");
        assert_eq!(err, RuntimeFailure::language(RuntimeErrorCode::TypeError));
    }

    /// NG-33: structural errors are not catchable as language errors (TR-002).
    #[test]
    fn ng33_structural_error_not_catchable() {
        let err = VmError::new(VmStructuralErrorCode::InvalidEirError, "bad eir");
        assert!(!err.is_catchable_as_language_error());
    }

    // --- RG residual unwind paths ---

    /// RG-06: raise during pending raise is suppressed (TR-009).
    #[test]
    fn rg06_defer_raise_suppressed_under_pending_raise() {
        let mut store = ErrorStore::new();
        let primary = store.allocate(ErrorObj::new(RuntimeErrorCode::KeyError, "primary"));
        let defer_err = store.allocate(ErrorObj::new(RuntimeErrorCode::TypeError, "defer"));
        let pending = combine_cleanup_result(
            PendingControl::Raise(primary),
            CleanupStepResult::Raise(defer_err),
            &mut store,
        );
        assert_eq!(pending, PendingControl::Raise(primary));
        assert_eq!(
            store.get(primary).expect("p").suppressed,
            vec![defer_err]
        );
    }

    /// RG-07: finally Normal preserves pending return (TR-009 return through finally).
    #[test]
    fn rg07_finally_normal_preserves_pending_return() {
        use vm_runtime::unwind::combine::finally_override;
        let pending = finally_override(
            PendingControl::Return(Some(Value::Int(7))),
            CleanupStepResult::Normal,
            &mut ErrorStore::new(),
        );
        assert_eq!(pending, PendingControl::Return(Some(Value::Int(7))));
    }

    // --- DG residual ---

    /// DG-03: host language error normalized to raise handle (TR-012).
    #[test]
    fn dg03_host_error_normalized_to_raise() {
        let mut store = ErrorStore::new();
        let err = normalize_host_call_result(HostCallResult::Error("host failed".into()), &mut store)
            .expect_err("DG-03");
        match err {
            NormalizedHostError::Raise(handle) => {
                let obj = store.get(handle).expect("stored");
                assert_eq!(obj.error_code, RuntimeErrorCode::InternalVMError);
            }
            NormalizedHostError::Structural(_) => panic!("expected language raise"),
        }
    }
}
