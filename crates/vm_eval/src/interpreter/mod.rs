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
    binary_add_module, branch_non_bool_module, branch_true_module, generic_call_mid_block_module,
    generic_call_nested_module, helper_alloc_object_module, helper_perform_unwind_module,
    literal_return_module, loop_backedge_module, module_init_body_module, raise_error_module,
    slot_copy_module, undispatched_helper_module,
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
/// SIR→EIR lowering may allocate many temps (short-circuit, loops, display).
const DEFAULT_SLOT_COUNT: usize = 128;

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
        self.state.load_functions(&module.functions);
        let function = match module.functions.iter().find(|f| f.eir_function_id == entry) {
            Some(f) => f.clone(),
            None => {
                return ControlState::VmError(VmError::new(
                    VmStructuralErrorCode::InvalidEirError,
                    format!("unknown EIR function {}", entry.raw()),
                ));
            }
        };
        self.run_function(&function)
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

    /// Run module initialization EIR body (ISSUE-002 path).
    pub fn run_module_init_function(
        &mut self,
        module: &EirModule,
        init: EirFunctionId,
    ) -> ControlState {
        self.state.constants = module.constants.clone();
        self.state.load_functions(&module.functions);
        let function = match module.functions.iter().find(|f| f.eir_function_id == init) {
            Some(f) => f.clone(),
            None => {
                return ControlState::VmError(VmError::new(
                    VmStructuralErrorCode::InvalidEirError,
                    format!("unknown init EIR function {}", init.raw()),
                ));
            }
        };
        self.run_function(&function)
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

            let (block, start_op) = {
                let frame = self.state.current_frame().expect("frame");
                match frame.current_block() {
                    Some(b) => (b.clone(), frame.next_op_index),
                    None => {
                        return Err(InterpreterError::structural(
                            VmStructuralErrorCode::InvalidEirError,
                            "current block not found",
                        ));
                    }
                }
            };

            for (idx, op) in block.ops.iter().enumerate().skip(start_op) {
                // Advance resume point before execute so nested call resumes after this op.
                if let Some(frame) = self.state.current_frame_mut() {
                    frame.next_op_index = idx + 1;
                }
                match execute_op(op, &mut self.state, &mut self.executor)? {
                    OpOutcome::Continue => {}
                    OpOutcome::Control(control) => {
                        return Ok(control_to_control_state(control));
                    }
                    OpOutcome::EnterUserCall { prepared, dest } => {
                        self.enter_user_call(prepared, dest)?;
                        // Nested call finished; continue remaining ops from next_op_index.
                    }
                }
            }

            match execute_terminator(&mut self.state, &block, &block.terminator)? {
                TerminatorOutcome::JumpTo(target) => {
                    let frame = self.state.current_frame_mut().expect("frame");
                    frame.current_block = target;
                    frame.next_op_index = 0;
                }
                TerminatorOutcome::Return(value) => {
                    let returning = self.state.pop_frame();
                    if let Some(parent) = self.state.current_frame_mut() {
                        if let Some(frame) = returning {
                            if let Some(dest) = frame.return_dest {
                                if let Some(v) = value {
                                    parent
                                        .slots
                                        .write(dest, v)
                                        .map_err(InterpreterError::from_runtime_failure)?;
                                }
                            }
                        }
                        // Resume parent at next_op_index (already advanced past call site).
                        continue;
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

    /// Push callee frame, bind prepared args, run until nested return writes parent dest.
    fn enter_user_call(
        &mut self,
        prepared: vm_runtime::helpers::h3::PreparedUserCall,
        dest: Option<vm_core::id::SlotId>,
    ) -> Result<(), InterpreterError> {
        use vm_runtime::call::callable::CallableTarget;
        let (entry, function_id) = match &prepared.target {
            CallableTarget::UserFunction(u) => (u.entry_eir_function, Some(u.function_id)),
            CallableTarget::BoundMethod(m) => {
                // Resolve via FunctionId only.
                (
                    EirFunctionId::new(m.function_id.raw()),
                    Some(m.function_id),
                )
            }
            _ => {
                return Err(InterpreterError::structural(
                    VmStructuralErrorCode::InvalidFrameStateError,
                    "enter_user_call requires UserFunction or BoundMethod",
                ));
            }
        };
        let function = self
            .state
            .resolve_user_function(entry, function_id)
            .cloned()
            .ok_or_else(|| {
                InterpreterError::structural(
                    VmStructuralErrorCode::InvalidEirError,
                    format!("no EIR body for call target {}", entry.raw()),
                )
            })?;

        let mut frame = InterpreterFrame::new(&function, DEFAULT_SLOT_COUNT);
        frame.return_dest = dest;
        for (slot, value) in prepared.bound {
            frame
                .slots
                .write(slot, value)
                .map_err(InterpreterError::from_runtime_failure)?;
        }
        self.state.frames.push(frame);

        // Run nested frames until we return to parent depth.
        let parent_depth = self.state.frames.len() - 1;
        self.execute_nested_until_return(parent_depth)
    }

    /// Execute while frame stack is deeper than `parent_depth`.
    fn execute_nested_until_return(
        &mut self,
        parent_depth: usize,
    ) -> Result<(), InterpreterError> {
        let mut steps = 0u32;
        while self.state.frames.len() > parent_depth {
            steps += 1;
            if steps > MAX_STEPS {
                return Err(InterpreterError::structural(
                    VmStructuralErrorCode::InvalidFrameStateError,
                    "nested call step limit exceeded",
                ));
            }
            let (block, start_op) = {
                let frame = self.state.current_frame().expect("frame");
                match frame.current_block() {
                    Some(b) => (b.clone(), frame.next_op_index),
                    None => {
                        return Err(InterpreterError::structural(
                            VmStructuralErrorCode::InvalidEirError,
                            "current block not found",
                        ));
                    }
                }
            };
            for (idx, op) in block.ops.iter().enumerate().skip(start_op) {
                if let Some(frame) = self.state.current_frame_mut() {
                    frame.next_op_index = idx + 1;
                }
                match execute_op(op, &mut self.state, &mut self.executor)? {
                    OpOutcome::Continue => {}
                    OpOutcome::Control(control) => {
                        // Nested raise/control propagates by converting to raise/return on top.
                        match control {
                            VmControl::Raise(h) => {
                                // Pop nested frames down to parent and re-raise.
                                while self.state.frames.len() > parent_depth {
                                    self.state.pop_frame();
                                }
                                return Err(InterpreterError::VmError(VmError::new(
                                    VmStructuralErrorCode::InvalidFrameStateError,
                                    format!("nested raise {}", h.raw()),
                                )));
                            }
                            other => {
                                return Err(InterpreterError::structural(
                                    VmStructuralErrorCode::InvalidFrameStateError,
                                    format!("unsupported nested control {other:?}"),
                                ));
                            }
                        }
                    }
                    OpOutcome::EnterUserCall { prepared, dest } => {
                        self.enter_user_call(prepared, dest)?;
                    }
                }
            }
            match execute_terminator(&mut self.state, &block, &block.terminator)? {
                TerminatorOutcome::JumpTo(target) => {
                    let frame = self.state.current_frame_mut().expect("frame");
                    frame.current_block = target;
                    frame.next_op_index = 0;
                }
                TerminatorOutcome::Return(value) => {
                    let returning = self.state.pop_frame().expect("frame");
                    if self.state.frames.len() == parent_depth {
                        if let (Some(dest), Some(v)) = (returning.return_dest, value) {
                            let parent = self.state.current_frame_mut().expect("parent");
                            parent
                                .slots
                                .write(dest, v)
                                .map_err(InterpreterError::from_runtime_failure)?;
                        }
                        return Ok(());
                    }
                    if let Some(parent) = self.state.current_frame_mut() {
                        if let Some(dest) = returning.return_dest {
                            if let Some(v) = value {
                                parent
                                    .slots
                                    .write(dest, v)
                                    .map_err(InterpreterError::from_runtime_failure)?;
                            }
                        }
                    }
                }
                TerminatorOutcome::Raise(handle) => {
                    while self.state.frames.len() > parent_depth {
                        self.state.pop_frame();
                    }
                    return Err(InterpreterError::VmError(VmError::new(
                        VmStructuralErrorCode::InvalidFrameStateError,
                        format!("nested raise {}", handle.raw()),
                    )));
                }
                TerminatorOutcome::Halt => {
                    return Err(InterpreterError::structural(
                        VmStructuralErrorCode::InvalidFrameStateError,
                        "nested halt",
                    ));
                }
            }
        }
        Ok(())
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
    use vm_core::error::registry::RuntimeErrorCode;
    use vm_core::id::{ControlRegionId, EirFunctionId, FunctionId, ModuleId, ObjectId};
    use vm_core::value::Value;
    use vm_runtime::call::callable::{CallableTarget, UserFunctionTarget};
    use vm_runtime::control::PendingControl;
    use vm_runtime::unwind::region::{ControlRegionKind, RuntimeRegionFrame};

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
    fn helper_alloc_object_integration_returns_object_ref() {
        let module = helper_alloc_object_module();
        let mut interp = Interpreter::new();
        let result = interp.run_module(&module, EirFunctionId::new(0));
        assert!(matches!(result, ControlState::Return(Some(Value::ObjectRef(_)))));
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

    /// ISSUE-001: generic_call prepare + nested EIR body returns callee arg.
    #[test]
    fn generic_call_enters_user_function_body() {
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

    /// ISSUE-002 path: module init EIR body executes via run_module_init_function.
    #[test]
    fn module_init_body_executes() {
        let module = module_init_body_module();
        let mut interp = Interpreter::new();
        let result = interp.run_module_init_function(&module, EirFunctionId::new(0));
        assert_eq!(result, ControlState::Return(Some(Value::Int(99))));
    }

    /// Mid-block resume: ops after generic_call still run (copy result then return).
    #[test]
    fn generic_call_resumes_ops_after_call_site() {
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
}