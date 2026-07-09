//! Regression matrix (Stage 13 / WP-18).

#[cfg(test)]
mod tests {
    use vm_core::error::language::ErrorStore;
    use vm_core::error::registry::RuntimeErrorCode;
    use vm_core::id::{BindingId, ControlRegionId, FrameId, SlotId};
    use vm_core::runtime_plan::schema::Mutability;
    use vm_core::value::Value;
    use vm_runtime::binding_cell::CellOwner;
    use vm_runtime::call::contract::StubTypeContractChecker;
    use vm_runtime::control::PendingControl;
    use vm_runtime::frame::Frame;
    use vm_runtime::runtime_error::RuntimeFailure;
    use vm_runtime::unwind::combine::CleanupStepResult;
    use vm_runtime::unwind::perform::{perform_unwind, UnwindExecutor, UnwindOutcome};
    use vm_runtime::unwind::region::{ControlRegionKind, RuntimeRegionFrame, UnwindContext};
    use vm_runtime::write_barrier::NoopWriteBarrierHook;

    struct LogExec {
        log: Vec<String>,
    }
    impl UnwindExecutor for LogExec {
        fn call_defer(&mut self, id: u32) -> CleanupStepResult {
            self.log.push(format!("defer:{id}"));
            CleanupStepResult::Normal
        }
        fn close_resource(&mut self, id: u32) -> CleanupStepResult {
            self.log.push(format!("resource:{id}"));
            CleanupStepResult::Normal
        }
        fn run_finally(&mut self, id: u32) -> CleanupStepResult {
            self.log.push(format!("finally:{id}"));
            CleanupStepResult::Normal
        }
    }

    /// RG-01: nested regions run inner defer before outer defer and finally.
    #[test]
    fn rg01_nested_unwind_lifo_order() {
        let mut store = ErrorStore::new();
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(Some(Value::Int(1))));
        let mut outer =
            RuntimeRegionFrame::new(ControlRegionId::new(0), ControlRegionKind::Function);
        outer.cleanup_state.register_defer(2, "outer");
        outer.cleanup_state.set_finally(3);
        let mut inner = RuntimeRegionFrame::new(ControlRegionId::new(1), ControlRegionKind::Block);
        inner.cleanup_state.register_defer(1, "inner");
        ctx.push_region(outer);
        ctx.push_region(inner);
        let mut exec = LogExec { log: Vec::new() };
        let outcome = perform_unwind(&mut ctx, &mut exec, &mut store);
        assert!(matches!(
            outcome,
            UnwindOutcome::Resolved(vm_runtime::control::VmControl::Return(Some(Value::Int(1))))
        ));
        assert_eq!(
            exec.log,
            vec![
                "defer:1".to_string(),
                "defer:2".to_string(),
                "finally:3".to_string()
            ]
        );
    }

    /// RG-02: immutable cell write raises ReadOnlyError.
    #[test]
    fn rg02_immutable_cell_write_readonly_error() {
        let mut frame = Frame::new(FrameId::new(0), 1);
        let mut barrier = NoopWriteBarrierHook;
        let checker = StubTypeContractChecker::new();
        frame
            .slots
            .bind_cell_with_value(
                SlotId::new(0),
                BindingId::new(1),
                Mutability::Immutable,
                CellOwner::LocalCapture,
                Value::Int(1),
            )
            .expect("bind");
        let err = frame
            .slots
            .write_cell(SlotId::new(0), Value::Int(2), &checker, &mut barrier)
            .expect_err("RG-02");
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::ReadOnlyError)
        );
    }
}
