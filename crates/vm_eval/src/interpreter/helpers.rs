//! Runtime helper bridge for interpreter dispatch.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-REGISTRY.md`, `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md` §15

use vm_core::eir::schema::RuntimeHelperOp;
use vm_core::error::registry::VmStructuralErrorCode;
use vm_core::id::SlotId;

use vm_runtime::control::VmControl;
use vm_runtime::helpers::{
    dispatch_helper, HelperDispatchEnv, HelperDispatchOutcome,
};
use vm_runtime::unwind::UnwindExecutor;

use super::error::InterpreterError;
use super::state::InterpreterState;

/// Result of helper dispatch inside the interpreter.
#[derive(Debug, Clone, PartialEq)]
pub enum HelperBridgeOutcome {
    Value(vm_core::value::Value),
    Unit,
    Control(VmControl),
}

/// Bootstrap unwind executor for helper calls without full defer/resource wiring.
#[derive(Debug, Default)]
pub struct BootstrapUnwindExecutor;

impl UnwindExecutor for BootstrapUnwindExecutor {
    fn call_defer(&mut self, _: u32) -> vm_runtime::unwind::combine::CleanupStepResult {
        vm_runtime::unwind::combine::CleanupStepResult::Normal
    }

    fn close_resource(&mut self, _: u32) -> vm_runtime::unwind::combine::CleanupStepResult {
        vm_runtime::unwind::combine::CleanupStepResult::Normal
    }

    fn run_finally(&mut self, _: u32) -> vm_runtime::unwind::combine::CleanupStepResult {
        vm_runtime::unwind::combine::CleanupStepResult::Normal
    }
}

/// Dispatch a `RuntimeHelperOp` through the VM helper boundary.
pub fn dispatch_runtime_helper(
    op: &RuntimeHelperOp,
    state: &mut InterpreterState,
    executor: &mut impl UnwindExecutor,
) -> Result<HelperBridgeOutcome, InterpreterError> {
    let args: Vec<vm_core::value::Value> = op
        .args
        .iter()
        .map(|slot| read_slot(state, *slot))
        .collect::<Result<_, _>>()?;

    let source_span = state.last_source_span;
    let mut env = HelperDispatchEnv {
        heap: &mut state.heap,
        error_store: &mut state.error_store,
        type_checker: &state.type_checker,
        callable_registry: &state.callable_registry,
        write_barrier: &mut state.write_barrier,
        source_span,
        unwind_ctx: &mut state.unwind_ctx,
        executor,
    };

    match dispatch_helper(op.helper_id, &args, &mut env) {
        Ok(HelperDispatchOutcome::Value(value)) => Ok(HelperBridgeOutcome::Value(value)),
        Ok(HelperDispatchOutcome::Unit) => Ok(HelperBridgeOutcome::Unit),
        Ok(HelperDispatchOutcome::VmControl(control)) => Ok(HelperBridgeOutcome::Control(control)),
        Err(err) => Err(InterpreterError::from_runtime_failure(err)),
    }
}

fn read_slot(state: &InterpreterState, slot: SlotId) -> Result<vm_core::value::Value, InterpreterError> {
    let frame = state
        .current_frame()
        .ok_or_else(|| InterpreterError::structural(
            VmStructuralErrorCode::InvalidFrameStateError,
            "no active frame for helper args",
        ))?;
    frame
        .slots
        .read(slot)
        .map_err(InterpreterError::from_runtime_failure)
}