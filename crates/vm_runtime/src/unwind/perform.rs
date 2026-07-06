//! `perform_unwind` entry point.
//!
//! Spec: `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md` §5

use vm_core::error::language::ErrorStore;
use vm_core::id::ErrorHandle;

use super::catch::{dispatch_catch_handlers, CatchDispatchResult};
use super::cleanup::{CleanupProgress, FinallyState, ResourceCloseError};
use super::combine::{combine_cleanup_result, finally_override, CleanupStepResult};
use super::region::{resolved_inside_region, target_resolved_by_region, UnwindContext};
use crate::control::{PendingControl, VmControl};

/// Callback surface for defer/resource/finally/catch execution during unwind.
pub trait UnwindExecutor {
    fn call_defer(&mut self, callable_id: u32) -> CleanupStepResult;
    fn close_resource(&mut self, resource_id: u32) -> CleanupStepResult;
    fn run_finally(&mut self, action_id: u32) -> CleanupStepResult;

    fn catch_matches(
        &mut self,
        entry: &super::catch::CatchEntry,
        error: ErrorHandle,
        store: &ErrorStore,
    ) -> Result<bool, CleanupStepResult> {
        let _ = (entry, error, store);
        Ok(false)
    }

    fn run_catch_body(&mut self, action_id: u32) -> CleanupStepResult {
        let _ = action_id;
        CleanupStepResult::Normal
    }
}

/// Outcome of a completed unwind pass.
#[derive(Debug, Clone, PartialEq)]
pub enum UnwindOutcome {
    Resolved(VmControl),
    Propagated(PendingControl),
}

/// Canonical structured unwinding loop.
pub fn perform_unwind<E: UnwindExecutor>(
    ctx: &mut UnwindContext,
    executor: &mut E,
    store: &mut ErrorStore,
) -> UnwindOutcome {
    while let Some(region_id) = ctx.top_region().map(|frame| frame.region_id) {
        let frame_snapshot = ctx.top_region().expect("top").clone();

        if resolved_inside_region(&ctx.pending, &frame_snapshot) {
            return resolve_pending(&ctx.pending);
        }

        let mut advanced = false;

        if let Some(frame) = ctx.top_region_mut() {
            if frame.cleanup_state.cleanup_progress == CleanupProgress::NotStarted {
                frame.cleanup_state.cleanup_progress = CleanupProgress::RunningDefers;
            }
        }

        if let Some(frame) = ctx.top_region() {
            if frame.cleanup_state.cleanup_progress == CleanupProgress::RunningDefers {
                advanced = true;
                run_defer_phase(ctx, executor, store);
            }
        }

        if let Some(frame) = ctx.top_region() {
            if frame.cleanup_state.cleanup_progress == CleanupProgress::RunningResources {
                advanced = true;
                run_resource_phase(ctx, executor, store);
            }
        }

        if let Some(frame) = ctx.top_region() {
            if frame.cleanup_state.cleanup_progress == CleanupProgress::RunningFinally {
                advanced = true;
                run_finally_phase(ctx, executor, store);
            }
        }

        if let Some(frame) = ctx.top_region() {
            if frame.cleanup_state.cleanup_progress == CleanupProgress::Complete {
                let frame = ctx.top_region().expect("top").clone();
                if frame.region_kind == super::region::ControlRegionKind::TryCatch {
                    if let PendingControl::Raise(error) = ctx.pending {
                        match dispatch_catch_handlers(
                            &frame.catch_entries,
                            error,
                            executor,
                            store,
                        ) {
                            CatchDispatchResult::Handled(outcome) => return outcome,
                            CatchDispatchResult::NotHandled => {}
                        }
                    }
                }
                let frame = ctx.pop_region().expect("pop complete region");
                if target_resolved_by_region(&ctx.pending, &frame) {
                    return resolve_pending(&ctx.pending);
                }
                let _ = region_id;
                advanced = true;
            }
        }

        if !advanced {
            break;
        }
    }

    UnwindOutcome::Propagated(ctx.pending.clone())
}

fn run_defer_phase<E: UnwindExecutor>(
    ctx: &mut UnwindContext,
    executor: &mut E,
    store: &mut ErrorStore,
) {
    loop {
        let defer_callable = ctx
            .top_region_mut()
            .and_then(|frame| frame.cleanup_state.defer_stack.pop());
        let Some(defer) = defer_callable else {
            if let Some(frame) = ctx.top_region_mut() {
                frame.cleanup_state.cleanup_progress = CleanupProgress::RunningResources;
            }
            break;
        };
        let result = executor.call_defer(defer.callable_id);
        if !result.is_normal() {
            ctx.pending = combine_cleanup_result(ctx.pending.clone(), result, store);
        }
    }
}

fn run_resource_phase<E: UnwindExecutor>(
    ctx: &mut UnwindContext,
    executor: &mut E,
    store: &mut ErrorStore,
) {
    loop {
        let resource_entry = ctx
            .top_region_mut()
            .and_then(|frame| frame.cleanup_state.resource_stack.pop());
        let Some(mut resource) = resource_entry else {
            if let Some(frame) = ctx.top_region_mut() {
                frame.cleanup_state.cleanup_progress = CleanupProgress::RunningFinally;
            }
            break;
        };

        match resource.begin_close() {
            Ok(()) => {
                let result = executor.close_resource(resource.resource_id);
                match &result {
                    CleanupStepResult::Normal => resource.mark_closed(),
                    CleanupStepResult::Raise(_) => resource.mark_failed(),
                    _ => resource.mark_failed(),
                }
                if !result.is_normal() {
                    ctx.pending = combine_cleanup_result(ctx.pending.clone(), result, store);
                }
            }
            Err(ResourceCloseError::AlreadyClosed) | Err(ResourceCloseError::PreviouslyFailed) => {
                continue;
            }
            Err(ResourceCloseError::AlreadyClosing) => {
                resource.mark_failed();
            }
        }
    }
}

fn run_finally_phase<E: UnwindExecutor>(
    ctx: &mut UnwindContext,
    executor: &mut E,
    store: &mut ErrorStore,
) {
    let finally_action = ctx.top_region().and_then(|frame| match frame.cleanup_state.finally_state {
        FinallyState::Pending { action_id } => Some(action_id),
        _ => None,
    });

    if let Some(action_id) = finally_action {
        let result = executor.run_finally(action_id);
        ctx.pending = finally_override(ctx.pending.clone(), result, store);
        if let Some(frame) = ctx.top_region_mut() {
            frame.cleanup_state.finally_state = FinallyState::Complete;
        }
    }

    if let Some(frame) = ctx.top_region_mut() {
        frame.cleanup_state.cleanup_progress = CleanupProgress::Complete;
    }
}

pub(super) fn resolve_pending(pending: &PendingControl) -> UnwindOutcome {
    let control = match pending {
        PendingControl::Return(value) => VmControl::Return(value.clone()),
        PendingControl::Break(region) => VmControl::Break(*region),
        PendingControl::Continue(region) => VmControl::Continue(*region),
        PendingControl::Raise(handle) => VmControl::Raise(*handle),
    };
    UnwindOutcome::Resolved(control)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unwind::cleanup::CleanupState;
    use crate::unwind::region::{ControlRegionKind, RuntimeRegionFrame};
    use vm_core::error::language::ErrorObj;
    use vm_core::error::registry::RuntimeErrorCode;
    use vm_core::id::ControlRegionId;
    use vm_core::value::Value;

    struct ScriptExecutor {
        defer_results: Vec<CleanupStepResult>,
        resource_results: Vec<CleanupStepResult>,
        finally_result: CleanupStepResult,
        log: Vec<String>,
    }

    impl ScriptExecutor {
        fn new(
            defer_results: Vec<CleanupStepResult>,
            resource_results: Vec<CleanupStepResult>,
            finally_result: CleanupStepResult,
        ) -> Self {
            Self {
                defer_results,
                resource_results,
                finally_result,
                log: Vec::new(),
            }
        }
    }

    impl UnwindExecutor for ScriptExecutor {
        fn call_defer(&mut self, callable_id: u32) -> CleanupStepResult {
            self.log.push(format!("defer:{callable_id}"));
            self.defer_results
                .get(callable_id as usize)
                .cloned()
                .unwrap_or(CleanupStepResult::Normal)
        }

        fn close_resource(&mut self, resource_id: u32) -> CleanupStepResult {
            self.log.push(format!("resource:{resource_id}"));
            self.resource_results
                .get(resource_id as usize)
                .cloned()
                .unwrap_or(CleanupStepResult::Normal)
        }

        fn run_finally(&mut self, action_id: u32) -> CleanupStepResult {
            self.log.push(format!("finally:{action_id}"));
            self.finally_result.clone()
        }
    }

    fn region_with_cleanup(region_id: u32, kind: ControlRegionKind) -> RuntimeRegionFrame {
        let mut cleanup = CleanupState::new();
        cleanup.register_defer(0, "d0");
        cleanup.register_defer(1, "d1");
        cleanup.register_resource(0, "r0");
        cleanup.set_finally(7);
        RuntimeRegionFrame::new(ControlRegionId::new(region_id), kind).with_cleanup(cleanup)
    }

    #[test]
    fn return_through_finally_preserves_return_when_finally_normal() {
        let mut ctx = UnwindContext::with_pending(PendingControl::Return(Some(Value::Int(9))));
        ctx.push_region(region_with_cleanup(1, ControlRegionKind::Function));

        let mut exec = ScriptExecutor::new(
            vec![CleanupStepResult::Normal, CleanupStepResult::Normal],
            vec![CleanupStepResult::Normal],
            CleanupStepResult::Normal,
        );
        let mut store = ErrorStore::new();

        let outcome = perform_unwind(&mut ctx, &mut exec, &mut store);
        assert_eq!(
            outcome,
            UnwindOutcome::Resolved(VmControl::Return(Some(Value::Int(9))))
        );
        assert_eq!(exec.log, vec!["defer:1", "defer:0", "resource:0", "finally:7"]);
    }

    #[test]
    fn raise_through_defer_replaces_pending_return() {
        let mut store = ErrorStore::new();
        let defer_err = store.allocate(ErrorObj::new(RuntimeErrorCode::TypeError, "defer"));

        let mut ctx = UnwindContext::with_pending(PendingControl::Return(Some(Value::Int(1))));
        let mut cleanup = CleanupState::new();
        cleanup.register_defer(0, "d0");
        ctx.push_region(
            RuntimeRegionFrame::new(ControlRegionId::new(1), ControlRegionKind::Block)
                .with_cleanup(cleanup),
        );

        let mut exec = ScriptExecutor::new(
            vec![CleanupStepResult::Raise(defer_err)],
            vec![],
            CleanupStepResult::Normal,
        );

        let outcome = perform_unwind(&mut ctx, &mut exec, &mut store);
        assert_eq!(
            outcome,
            UnwindOutcome::Propagated(PendingControl::Raise(defer_err))
        );
    }

    #[test]
    fn resource_close_raise_during_pending_raise_is_suppressed() {
        let mut store = ErrorStore::new();
        let primary = store.allocate(ErrorObj::new(RuntimeErrorCode::KeyError, "primary"));
        let close_err = store.allocate(ErrorObj::new(RuntimeErrorCode::TypeError, "close"));

        let mut ctx = UnwindContext::with_pending(PendingControl::Raise(primary));
        let mut cleanup = CleanupState::new();
        cleanup.register_resource(0, "r0");
        ctx.push_region(
            RuntimeRegionFrame::new(ControlRegionId::new(1), ControlRegionKind::Block)
                .with_cleanup(cleanup),
        );

        let mut exec = ScriptExecutor::new(vec![], vec![CleanupStepResult::Raise(close_err)], CleanupStepResult::Normal);
        let outcome = perform_unwind(&mut ctx, &mut exec, &mut store);

        assert_eq!(
            outcome,
            UnwindOutcome::Propagated(PendingControl::Raise(primary))
        );
        assert_eq!(store.get(primary).expect("primary").suppressed, vec![close_err]);
    }

    #[test]
    fn break_cleanup_runs_before_loop_resolution() {
        let target = ControlRegionId::new(5);
        let mut ctx = UnwindContext::with_pending(PendingControl::Break(target));
        ctx.push_region(region_with_cleanup(5, ControlRegionKind::Loop));

        let mut exec = ScriptExecutor::new(
            vec![CleanupStepResult::Normal, CleanupStepResult::Normal],
            vec![CleanupStepResult::Normal],
            CleanupStepResult::Normal,
        );
        let mut store = ErrorStore::new();

        let outcome = perform_unwind(&mut ctx, &mut exec, &mut store);
        assert_eq!(outcome, UnwindOutcome::Resolved(VmControl::Break(target)));
        assert!(exec.log.contains(&"defer:1".to_string()));
        assert!(exec.log.contains(&"finally:7".to_string()));
    }

    #[test]
    fn finally_override_replaces_pending_return_with_raise() {
        let mut store = ErrorStore::new();
        let finally_err = store.allocate(ErrorObj::new(RuntimeErrorCode::AssertionError, "finally"));

        let mut ctx = UnwindContext::with_pending(PendingControl::Return(Some(Value::Int(3))));
        let mut cleanup = CleanupState::new();
        cleanup.set_finally(1);
        ctx.push_region(
            RuntimeRegionFrame::new(ControlRegionId::new(1), ControlRegionKind::Function)
                .with_cleanup(cleanup),
        );

        let mut exec = ScriptExecutor::new(vec![], vec![], CleanupStepResult::Raise(finally_err));
        let outcome = perform_unwind(&mut ctx, &mut exec, &mut store);

        assert_eq!(
            outcome,
            UnwindOutcome::Propagated(PendingControl::Raise(finally_err))
        );
    }

    #[test]
    fn pending_raise_handled_by_try_catch_region() {
        use super::super::catch::CatchEntry;

        let mut store = ErrorStore::new();
        let err = store.allocate(ErrorObj::new(RuntimeErrorCode::KeyError, "missing"));

        struct CatchExec;
        impl UnwindExecutor for CatchExec {
            fn call_defer(&mut self, _: u32) -> CleanupStepResult {
                CleanupStepResult::Normal
            }

            fn close_resource(&mut self, _: u32) -> CleanupStepResult {
                CleanupStepResult::Normal
            }

            fn run_finally(&mut self, _: u32) -> CleanupStepResult {
                CleanupStepResult::Normal
            }

            fn catch_matches(
                &mut self,
                entry: &super::super::catch::CatchEntry,
                error: ErrorHandle,
                store: &ErrorStore,
            ) -> Result<bool, CleanupStepResult> {
                Ok(entry.matched_error.is_some_and(|code| {
                    store
                        .get(error)
                        .is_some_and(|obj| obj.error_code == code)
                }))
            }
        }

        let mut ctx = UnwindContext::with_pending(PendingControl::Raise(err));
        ctx.push_region(
            RuntimeRegionFrame::new(ControlRegionId::new(1), ControlRegionKind::TryCatch)
                .with_catch_entries(vec![CatchEntry {
                    catch_index: 0,
                    matched_error: Some(RuntimeErrorCode::KeyError),
                    body_action_id: 0,
                }]),
        );

        let outcome = perform_unwind(&mut ctx, &mut CatchExec, &mut store);
        assert_eq!(
            outcome,
            UnwindOutcome::Resolved(VmControl::Normal(None))
        );
    }
}