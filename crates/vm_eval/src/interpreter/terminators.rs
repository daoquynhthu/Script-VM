//! EIR terminator handlers.
//!
//! Spec: `PHASE-3-EIR-SCHEMA-CLOSURE.md` §22, `PHASE-3-CONTROL-STATE-MODEL.md`

use vm_core::eir::schema::{Branch, EirBlock, EirTerminator, Jump, LoopBackedge, Raise, Return};
use vm_core::id::EirBlockId;
use vm_core::error::raise::{type_error_for_invalid_raise, validate_raise_operand};
use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::SlotId;
use vm_core::value::Value;

use super::diagnostics::record_block_span;
use super::error::InterpreterError;
use super::state::InterpreterState;

/// Outcome of terminator execution.
#[derive(Debug, Clone, PartialEq)]
pub enum TerminatorOutcome {
    /// Continue in the same frame at a new block.
    JumpTo(EirBlockId),
    /// Return from the current frame with a value.
    Return(Option<Value>),
    /// Source-level raise.
    Raise(vm_core::id::ErrorHandle),
    /// Frame stack exhausted; execution complete.
    Halt,
}

/// Execute a block terminator.
pub fn execute_terminator(
    state: &mut InterpreterState,
    block: &EirBlock,
    terminator: &EirTerminator,
) -> Result<TerminatorOutcome, InterpreterError> {
    record_block_span(state, block.source_span);
    match terminator {
        EirTerminator::Jump(j) => execute_jump(state, j),
        EirTerminator::Branch(b) => execute_branch(state, b),
        EirTerminator::Return(r) => execute_return(state, r),
        EirTerminator::Raise(r) => execute_raise(state, r),
        EirTerminator::LoopBackedge(l) => execute_loop_backedge(state, l),
        EirTerminator::Switch(_) | EirTerminator::Unwind(_) | EirTerminator::Unreachable(_) => {
            Err(InterpreterError::structural(
                vm_core::error::registry::VmStructuralErrorCode::InvalidEirError,
                "unsupported terminator in bootstrap interpreter",
            ))
        }
    }
}

fn execute_jump(state: &mut InterpreterState, jump: &Jump) -> Result<TerminatorOutcome, InterpreterError> {
    bind_block_args(state, jump.target, &jump.args)?;
    Ok(TerminatorOutcome::JumpTo(jump.target))
}

fn execute_branch(state: &mut InterpreterState, branch: &Branch) -> Result<TerminatorOutcome, InterpreterError> {
    let condition = read_slot(state, branch.condition)?;
    let target = match condition {
        Value::Bool(true) => branch.then_block,
        Value::Bool(false) => branch.else_block,
        _ => {
            return Err(InterpreterError::language(
                RuntimeErrorCode::TypeError,
                "branch condition must be bool",
            ));
        }
    };
    Ok(TerminatorOutcome::JumpTo(target))
}

fn execute_return(state: &mut InterpreterState, ret: &Return) -> Result<TerminatorOutcome, InterpreterError> {
    let value = match ret.value {
        Some(slot) => Some(read_slot(state, slot)?),
        None => None,
    };
    Ok(TerminatorOutcome::Return(value))
}

fn execute_raise(state: &mut InterpreterState, raise: &Raise) -> Result<TerminatorOutcome, InterpreterError> {
    let value = read_slot(state, raise.error)?;
    match validate_raise_operand(&value, &state.error_store) {
        vm_core::error::raise::RaiseValidation::Accepted(handle) => {
            Ok(TerminatorOutcome::Raise(handle))
        }
        vm_core::error::raise::RaiseValidation::RejectedTypeError => {
            let handle = type_error_for_invalid_raise(&mut state.error_store);
            Ok(TerminatorOutcome::Raise(handle))
        }
    }
}

fn execute_loop_backedge(
    state: &mut InterpreterState,
    edge: &LoopBackedge,
) -> Result<TerminatorOutcome, InterpreterError> {
    state.safepoint_polls.record_loop_backedge(edge.safepoint_id);
    bind_block_args(state, edge.target, &edge.args)?;
    Ok(TerminatorOutcome::JumpTo(edge.target))
}

fn bind_block_args(
    state: &mut InterpreterState,
    target: EirBlockId,
    args: &[SlotId],
) -> Result<(), InterpreterError> {
    let block = {
        let frame = state.current_frame().ok_or_else(|| {
            InterpreterError::structural(
                vm_core::error::registry::VmStructuralErrorCode::InvalidFrameStateError,
                "no active frame",
            )
        })?;
        frame.block(target).cloned().ok_or_else(|| {
            InterpreterError::structural(
                vm_core::error::registry::VmStructuralErrorCode::InvalidEirError,
                format!("unknown block {}", target.raw()),
            )
        })?
    };
    if block.parameters.len() != args.len() {
        return Err(InterpreterError::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidEirError,
            "block argument count mismatch",
        ));
    }
    let values: Vec<Value> = args
        .iter()
        .map(|slot| read_slot(state, *slot))
        .collect::<Result<_, _>>()?;
    let frame = state.current_frame_mut().expect("frame");
    for (param, value) in block.parameters.iter().zip(values) {
        frame
            .slots
            .write(param.slot, value)
            .map_err(InterpreterError::from_runtime_failure)?;
    }
    Ok(())
}

fn read_slot(state: &InterpreterState, slot: SlotId) -> Result<Value, InterpreterError> {
    let frame = state.current_frame().ok_or_else(|| {
        InterpreterError::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidFrameStateError,
            "no active frame",
        )
    })?;
    frame
        .slots
        .read(slot)
        .map_err(InterpreterError::from_runtime_failure)
}