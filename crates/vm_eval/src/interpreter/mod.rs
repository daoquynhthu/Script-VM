//! Fast EIR interpreter minimal execution path.
//!
//! Spec: `PHASE-3-EIR-SCHEMA-CLOSURE.md`, `PHASE-3-CONTROL-STATE-MODEL.md`,
//! `PHASE-3-RUNTIME-HELPER-REGISTRY.md`, `PHASE-3-GC-METADATA-OWNERSHIP.md`

mod diagnostics;
mod error;
mod fixtures;
mod helpers;
mod ops;
mod state;
mod terminators;

pub use error::InterpreterError;
pub use fixtures::{
    binary_add_module, branch_non_bool_module, branch_true_module,
    helper_perform_unwind_module, literal_return_module, loop_backedge_module,
    raise_error_module, slot_copy_module, undispatched_helper_module,
};
pub use helpers::BootstrapUnwindExecutor;
pub use state::{InterpreterFrame, InterpreterState, SafepointPollState};

use vm_core::control::ControlState;
use vm_core::eir::schema::{EirFunction, EirModule};
use vm_core::id::EirFunctionId;
use vm_core::error::language::{ErrorObj, ErrorStore};
use vm_core::error::registry::VmStructuralErrorCode;
use vm_core::error::VmError;
use vm_core::id::ErrorHandle;
use vm_runtime::control::VmControl;

use self::ops::{execute_op, OpOutcome};
use self::terminators::{execute_terminator, TerminatorOutcome};

/// Default slot capacity for bootstrap interpreter frames.
const DEFAULT_SLOT_COUNT: usize = 16;

/// Maximum interpreter steps before forced halt (bootstrap guard).
const MAX_STEPS: u32 = 10_000;

/// EIR fast interpreter.
#[derive(Debug)]
pub struct Interpreter {
    state: InterpreterState,
    executor: BootstrapUnwindExecutor,
}

impl Interpreter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: InterpreterState::new(Default::default()),
            executor: BootstrapUnwindExecutor::default(),
        }
    }

    #[must_use]
    pub fn state(&self) -> &InterpreterState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut InterpreterState {
        &mut self.state
    }

    /// Seed a language error object for raise tests.
    pub fn seed_error(&mut self, error: ErrorObj) -> ErrorHandle {
        self.state.error_store.allocate(error)
    }

    /// Execute the entry function of an EIR module.
    pub fn run_module(
        &mut self,
        module: &EirModule,
        entry: EirFunctionId,
    ) -> ControlState {
        self.state.constants = module.constants.clone();
        let function = match module.functions.iter().find(|f| f.eir_function_id == entry) {
            Some(f) => f,
            None => {
                return ControlState::VmError(VmError::new(
                    VmStructuralErrorCode::InvalidEirError,
                    format!("unknown EIR function {}", entry.raw()),
                ));
            }
        };
        self.run_function(function)
    }

    /// Execute a single EIR function to completion.
    pub fn run_function(&mut self, function: &EirFunction) -> ControlState {
        self.state.halted = false;
        self.state.push_frame(function, DEFAULT_SLOT_COUNT);
        match self.execute_until_halt() {
            Ok(control) => control,
            Err(err) => error_to_control_state(err),
        }
    }

    fn execute_until_halt(&mut self) -> Result<ControlState, InterpreterError> {
        let mut steps = 0u32;
        loop {
            if self.state.halted || self.state.current_frame().is_none() {
                return Ok(ControlState::Halt);
            }
            steps += 1;
            if steps > MAX_STEPS {
                return Err(InterpreterError::structural(
                    VmStructuralErrorCode::InvalidFrameStateError,
                    "interpreter step limit exceeded",
                ));
            }

            let block = {
                let frame = self.state.current_frame().expect("frame");
                match frame.current_block() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(InterpreterError::structural(
                            VmStructuralErrorCode::InvalidEirError,
                            "current block not found",
                        ));
                    }
                }
            };

            for op in &block.ops {
                match execute_op(op, &mut self.state, &mut self.executor)? {
                    OpOutcome::Continue => {}
                    OpOutcome::Control(control) => {
                        return Ok(control_to_control_state(control));
                    }
                }
            }

            match execute_terminator(&mut self.state, &block, &block.terminator)? {
                TerminatorOutcome::JumpTo(target) => {
                    let frame = self.state.current_frame_mut().expect("frame");
                    frame.current_block = target;
                }
                TerminatorOutcome::Return(value) => {
                    self.state.pop_frame();
                    if self.state.current_frame().is_some() {
                        return Err(InterpreterError::structural(
                            VmStructuralErrorCode::InvalidFrameStateError,
                            "nested return not supported in bootstrap interpreter",
                        ));
                    }
                    self.state.halted = true;
                    return Ok(ControlState::Return(value));
                }
                TerminatorOutcome::Raise(handle) => {
                    self.state.halted = true;
                    return Ok(ControlState::Raise(handle));
                }
                TerminatorOutcome::Halt => {
                    self.state.halted = true;
                    return Ok(ControlState::Halt);
                }
            }
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

fn control_to_control_state(control: VmControl) -> ControlState {
    match control {
        VmControl::Normal(v) => ControlState::Normal(v),
        VmControl::Return(v) => ControlState::Return(v),
        VmControl::Break(r) => ControlState::Break(r),
        VmControl::Continue(r) => ControlState::Continue(r),
        VmControl::Raise(h) => ControlState::Raise(h),
    }
}

fn error_to_control_state(err: InterpreterError) -> ControlState {
    match err {
        InterpreterError::Language(code, message) => {
            let mut store = ErrorStore::new();
            let handle = store.allocate(ErrorObj::new(code, message));
            ControlState::Raise(handle)
        }
        InterpreterError::Structural(code, message) => {
            ControlState::VmError(VmError::new(code, message))
        }
        InterpreterError::VmError(e) => ControlState::VmError(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::error::language::ErrorObj;
    use vm_core::value::Value;
    use vm_core::error::registry::RuntimeErrorCode;
    use vm_core::id::EirFunctionId;
    use vm_runtime::control::PendingControl;
    use vm_runtime::unwind::region::{ControlRegionKind, RuntimeRegionFrame};
    use vm_core::id::ControlRegionId;

    #[test]
    fn literal_execution_returns_constant() {
        let module = literal_return_module();
        let mut interp = Interpreter::new();
        let result = interp.run_module(&module, EirFunctionId::new(0));
        assert_eq!(result, ControlState::Return(Some(Value::Int(42))));
        assert_eq!(interp.state().last_source_span, Some(vm_diag::source_span::SourceSpanId::new(1)));
    }

    #[test]
    fn slot_load_store_round_trips() {
        let module = slot_copy_module();
        let mut interp = Interpreter::new();
        let result = interp.run_module(&module, EirFunctionId::new(0));
        assert_eq!(result, ControlState::Return(Some(Value::Int(7))));
    }

    #[test]
    fn branch_executes_then_path_for_true() {
        let module = branch_true_module();
        let mut interp = Interpreter::new();
        let result = interp.run_module(&module, EirFunctionId::new(0));
        assert_eq!(result, ControlState::Return(Some(Value::Int(1))));
    }

    #[test]
    fn branch_rejects_non_bool_condition() {
        let module = branch_non_bool_module();
        let mut interp = Interpreter::new();
        let result = interp.run_module(&module, EirFunctionId::new(0));
        assert!(matches!(result, ControlState::Raise(_)));
    }

    #[test]
    fn binary_add_computes_sum() {
        let module = binary_add_module();
        let mut interp = Interpreter::new();
        let result = interp.run_module(&module, EirFunctionId::new(0));
        assert_eq!(result, ControlState::Return(Some(Value::Int(7))));
    }

    #[test]
    fn loop_backedge_records_safepoint_poll() {
        let module = loop_backedge_module();
        let mut interp = Interpreter::new();
        let result = interp.run_module(&module, EirFunctionId::new(0));
        assert_eq!(result, ControlState::Return(Some(Value::Int(99))));
        assert_eq!(interp.state().safepoint_polls.loop_backedge_count, 1);
    }

    #[test]
    fn undispatched_helper_propagates_structural_error() {
        let module = undispatched_helper_module();
        let mut interp = Interpreter::new();
        let result = interp.run_module(&module, EirFunctionId::new(0));
        assert!(matches!(result, ControlState::VmError(_)));
    }

    #[test]
    fn raise_terminator_produces_control_state_raise() {
        let mut interp = Interpreter::new();
        let handle = interp.seed_error(ErrorObj::new(RuntimeErrorCode::AssertionError, "failed"));
        let module = raise_error_module(Value::Error(handle));
        let result = interp.run_module(&module, EirFunctionId::new(0));
        assert_eq!(result, ControlState::Raise(handle));
    }

    #[test]
    fn helper_perform_unwind_resolves_pending_return() {
        let module = helper_perform_unwind_module();
        let mut interp = Interpreter::new();
        interp.state_mut().unwind_ctx =
            vm_runtime::unwind::UnwindContext::with_pending(PendingControl::Return(Some(Value::Int(9))));
        interp.state_mut().unwind_ctx.push_region(RuntimeRegionFrame::new(
            ControlRegionId::new(0),
            ControlRegionKind::Function,
        ));
        let result = interp.run_module(&module, EirFunctionId::new(0));
        assert_eq!(result, ControlState::Return(Some(Value::Int(9))));
    }
}