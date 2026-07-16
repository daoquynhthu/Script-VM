//! VM integration tests
//!
//! This crate contains integration tests for the Script VM.
//! Stage 13 / WP-18 matrix modules: conformance, negative, diagnostics, regression.

#[cfg(test)]
mod conformance;
#[cfg(test)]
mod negative;
#[cfg(test)]
mod diagnostics;
#[cfg(test)]
mod regression;
#[cfg(test)]
mod gap_closure;
#[cfg(test)]
mod integration;

#[cfg(test)]
mod smoke {
    use sir_validate::validate::validate_sir_unit;
    use vm_core::eir::fixtures::{
        eir_module_with_may_collect_without_root_map,
        eir_module_with_write_barrier_without_source, minimal_eir_validation_context,
        minimal_valid_eir_module,
    };
    use vm_core::control::ControlState;
    use vm_core::id::EirFunctionId;
    use vm_core::eir::validate::{validate_eir_module, EirModuleInput, EirValidationError};
    use vm_core::id::SafepointId;
    use vm_core::runtime_plan::{fixtures::minimal_valid_plan, validate_runtime_plan};
    use vm_eval::interpreter::Interpreter;
    use vm_runtime::cache::runtime_plan_cache_key;
    use vm_runtime::helpers::{
        dispatch_helper_unwind_only, eir_validation_view, HELPER_PERFORM_UNWIND_ID,
        RuntimeHelperRegistry,
    };
    use vm_runtime::unwind::{UnwindContext, UnwindExecutor, UnwindOutcome};
    use vm_runtime::control::PendingControl;
    use vm_core::error::language::ErrorStore;
    use vm_core::id::ControlRegionId;
    use vm_core::value::Value;
    use vm_runtime::unwind::combine::CleanupStepResult;
    use vm_runtime::unwind::region::{ControlRegionKind, RuntimeRegionFrame};

    #[test]
    fn workspace_crates_link() {
        let plan = minimal_valid_plan();
        validate_runtime_plan(&plan).expect("minimal plan should validate");
        let cache = runtime_plan_cache_key(&plan);
        assert!(cache.helper_registry_digest.is_some());
        let mut interpreter = Interpreter::new();
        let module = minimal_valid_eir_module();
        let state = interpreter.run_module(&module, EirFunctionId::new(0));
        assert_eq!(state, ControlState::Return(Some(Value::Int(0))));
        let validation = validate_sir_unit();
        assert!(validation.is_valid());
    }

    #[test]
    fn eir_may_collect_rejects_missing_root_map_via_helper_registry() {
        let registry = RuntimeHelperRegistry::canonical().expect("canonical registry");
        let mut ctx = minimal_eir_validation_context();
        ctx.gc_may_run = true;
        ctx.safepoint_ids.insert(SafepointId::new(0));
        ctx.helper_registry = eir_validation_view(&registry);

        let module = eir_module_with_may_collect_without_root_map();
        let err = validate_eir_module(EirModuleInput::Resolved(&module), &ctx).unwrap_err();
        assert_eq!(err, EirValidationError::MayCollectWithoutRootMap);
    }

    #[test]
    fn non_may_raise_helper_without_source_passes_with_registry_view() {
        let registry = RuntimeHelperRegistry::canonical().expect("canonical registry");
        let mut ctx = minimal_eir_validation_context();
        ctx.helper_registry = eir_validation_view(&registry);

        let module = eir_module_with_write_barrier_without_source();
        assert!(validate_eir_module(EirModuleInput::Resolved(&module), &ctx).is_ok());
    }

    struct NoopUnwindExecutor;

    impl UnwindExecutor for NoopUnwindExecutor {
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
    fn helper_perform_unwind_integration_dispatches() {
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(Some(Value::Int(7))));
        ctx.push_region(RuntimeRegionFrame::new(
            ControlRegionId::new(0),
            ControlRegionKind::Function,
        ));
        let mut store = ErrorStore::new();
        let outcome = dispatch_helper_unwind_only(
            HELPER_PERFORM_UNWIND_ID,
            &mut ctx,
            &mut NoopUnwindExecutor,
            &mut store,
        )
        .expect("dispatch");
        assert!(matches!(outcome, UnwindOutcome::Resolved(_)));
    }
}