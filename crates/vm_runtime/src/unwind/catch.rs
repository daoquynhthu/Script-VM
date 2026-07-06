//! Catch-region dispatch during structured unwinding.
//!
//! Spec: `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md` §9

use vm_core::error::language::ErrorStore;
use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::ErrorHandle;

use super::combine::CleanupStepResult;
use super::perform::{resolve_pending, UnwindExecutor, UnwindOutcome};
use crate::control::VmControl;

/// Catch handler entry in source order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatchEntry {
    pub catch_index: u32,
    /// `None` matches any error (bare catch).
    pub matched_error: Option<RuntimeErrorCode>,
    pub body_action_id: u32,
}

/// Result of catch dispatch for a `PendingRaise` at a try/catch region.
#[derive(Debug, Clone, PartialEq)]
pub enum CatchDispatchResult {
    NotHandled,
    Handled(UnwindOutcome),
}

/// Attempt catch handlers in source order for `PendingRaise`.
pub fn dispatch_catch_handlers<E: UnwindExecutor>(
    entries: &[CatchEntry],
    error: ErrorHandle,
    executor: &mut E,
    store: &ErrorStore,
) -> CatchDispatchResult {
    for entry in entries {
        match executor.catch_matches(entry, error, store) {
            Ok(true) => {
                return CatchDispatchResult::Handled(outcome_from_catch_body(
                    executor.run_catch_body(entry.body_action_id),
                ));
            }
            Ok(false) => continue,
            Err(step) => {
                return CatchDispatchResult::Handled(outcome_from_cleanup_step(step));
            }
        }
    }
    CatchDispatchResult::NotHandled
}

fn outcome_from_catch_body(step: CleanupStepResult) -> UnwindOutcome {
    match step {
        CleanupStepResult::Normal => UnwindOutcome::Resolved(VmControl::Normal(None)),
        other => outcome_from_cleanup_step(other),
    }
}

fn outcome_from_cleanup_step(step: CleanupStepResult) -> UnwindOutcome {
    if let Some(pending) = step.into_pending_control() {
        resolve_pending(&pending)
    } else {
        UnwindOutcome::Resolved(VmControl::Normal(None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::error::language::ErrorObj;

    struct CatchExecutor {
        body_result: CleanupStepResult,
    }

    impl UnwindExecutor for CatchExecutor {
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
            entry: &CatchEntry,
            error: ErrorHandle,
            store: &ErrorStore,
        ) -> Result<bool, CleanupStepResult> {
            let Some(code) = entry.matched_error else {
                return Ok(true);
            };
            Ok(store
                .get(error)
                .is_some_and(|obj| obj.error_code == code))
        }

        fn run_catch_body(&mut self, _: u32) -> CleanupStepResult {
            self.body_result.clone()
        }
    }

    #[test]
    fn matching_catch_clears_pending_raise() {
        let mut store = ErrorStore::new();
        let err = store.allocate(ErrorObj::new(RuntimeErrorCode::KeyError, "missing"));
        let mut exec = CatchExecutor {
            body_result: CleanupStepResult::Normal,
        };
        let entries = vec![CatchEntry {
            catch_index: 0,
            matched_error: Some(RuntimeErrorCode::KeyError),
            body_action_id: 1,
        }];

        let result = dispatch_catch_handlers(&entries, err, &mut exec, &store);
        assert_eq!(
            result,
            CatchDispatchResult::Handled(UnwindOutcome::Resolved(VmControl::Normal(None)))
        );
    }

    #[test]
    fn non_matching_catch_is_skipped() {
        let mut store = ErrorStore::new();
        let err = store.allocate(ErrorObj::new(RuntimeErrorCode::TypeError, "bad"));
        let mut exec = CatchExecutor {
            body_result: CleanupStepResult::Normal,
        };
        let entries = vec![CatchEntry {
            catch_index: 0,
            matched_error: Some(RuntimeErrorCode::KeyError),
            body_action_id: 1,
        }];

        let result = dispatch_catch_handlers(&entries, err, &mut exec, &store);
        assert_eq!(result, CatchDispatchResult::NotHandled);
    }
}