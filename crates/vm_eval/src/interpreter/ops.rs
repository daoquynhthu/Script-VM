//! EIR operation handlers.
//!
//! Spec: `PHASE-3-EIR-SCHEMA-CLOSURE.md` §6–§22

use vm_core::eir::schema::{
    BinaryOp, BinaryOperator, CheckOp, ConstantOp, EirOp, EirOpKind, LoadOp, RuntimeHelperOp,
    SafepointOp, StoreOp, UnaryOp, UnaryOperator,
};
use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::SlotId;
use vm_core::value::Value;
use vm_runtime::unwind::UnwindExecutor;

use super::diagnostics::record_op_span;
use super::error::InterpreterError;
use super::helpers::{dispatch_runtime_helper, HelperBridgeOutcome};
use super::state::InterpreterState;

/// Execute a single EIR operation.
pub fn execute_op(
    op: &EirOp,
    state: &mut InterpreterState,
    executor: &mut impl UnwindExecutor,
) -> Result<OpOutcome, InterpreterError> {
    record_op_span(state, &op.metadata);
    match &op.kind {
        EirOpKind::Constant(c) => execute_constant(state, c),
        EirOpKind::Load(l) => execute_load(state, l),
        EirOpKind::Store(s) => execute_store(state, s),
        EirOpKind::Unary(u) => execute_unary(state, u),
        EirOpKind::Binary(b) => execute_binary(state, b),
        EirOpKind::Check(c) => execute_check(state, c),
        EirOpKind::RuntimeHelper(h) => execute_runtime_helper(state, h, executor),
        EirOpKind::Safepoint(s) => execute_safepoint(state, s),
        other => Err(InterpreterError::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidEirError,
            format!("unsupported EIR op in bootstrap interpreter: {other:?}"),
        )),
    }
}

/// Outcome of a single op (most ops continue; helpers may produce control).
#[derive(Debug, Clone, PartialEq)]
pub enum OpOutcome {
    Continue,
    Control(vm_runtime::control::VmControl),
}

fn execute_constant(state: &mut InterpreterState, op: &ConstantOp) -> Result<OpOutcome, InterpreterError> {
    let entry = state
        .constants
        .constants
        .get(&op.constant.raw())
        .ok_or_else(|| {
            InterpreterError::structural(
                vm_core::error::registry::VmStructuralErrorCode::InvalidEirError,
                format!("unknown constant id {}", op.constant.raw()),
            )
        })?;
    write_slot(state, op.dest, entry.value.clone())?;
    Ok(OpOutcome::Continue)
}

fn execute_load(state: &mut InterpreterState, op: &LoadOp) -> Result<OpOutcome, InterpreterError> {
    match op {
        LoadOp::Slot(load) => {
            let value = read_slot(state, load.source)?;
            if load.require_initialized && matches!(value, Value::None) {
                return Err(InterpreterError::language(
                    RuntimeErrorCode::UninitializedBindingError,
                    "load from uninitialized slot",
                ));
            }
            write_slot(state, load.dest, value)?;
        }
        _ => {
            return Err(InterpreterError::structural(
                vm_core::error::registry::VmStructuralErrorCode::InvalidEirError,
                "unsupported load op in bootstrap interpreter",
            ));
        }
    }
    Ok(OpOutcome::Continue)
}

fn execute_store(state: &mut InterpreterState, op: &StoreOp) -> Result<OpOutcome, InterpreterError> {
    match op {
        StoreOp::Slot(store) => {
            let value = read_slot(state, store.value)?;
            write_slot(state, store.dest, value)?;
        }
        _ => {
            return Err(InterpreterError::structural(
                vm_core::error::registry::VmStructuralErrorCode::InvalidEirError,
                "unsupported store op in bootstrap interpreter",
            ));
        }
    }
    Ok(OpOutcome::Continue)
}

fn execute_unary(state: &mut InterpreterState, op: &UnaryOp) -> Result<OpOutcome, InterpreterError> {
    let operand = read_slot(state, op.operand)?;
    let result = match op.op {
        UnaryOperator::Minus => match operand {
            Value::Int(n) => Value::Int(-n),
            _ => {
                return Err(InterpreterError::language(
                    RuntimeErrorCode::TypeError,
                    "unary minus requires int",
                ));
            }
        },
        UnaryOperator::Not => match operand {
            Value::Bool(b) => Value::Bool(!b),
            _ => {
                return Err(InterpreterError::language(
                    RuntimeErrorCode::TypeError,
                    "unary not requires bool",
                ));
            }
        },
        UnaryOperator::Plus => operand,
    };
    write_slot(state, op.dest, result)?;
    Ok(OpOutcome::Continue)
}

fn execute_binary(state: &mut InterpreterState, op: &BinaryOp) -> Result<OpOutcome, InterpreterError> {
    let left = read_slot(state, op.left)?;
    let right = read_slot(state, op.right)?;
    let result = match op.op {
        BinaryOperator::Add => match (left, right) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            _ => {
                return Err(InterpreterError::language(
                    RuntimeErrorCode::TypeError,
                    "add requires int operands",
                ));
            }
        },
        BinaryOperator::Equal => Value::Bool(left == right),
        _ => {
            return Err(InterpreterError::structural(
                vm_core::error::registry::VmStructuralErrorCode::InvalidEirError,
                "unsupported binary op in bootstrap interpreter",
            ));
        }
    };
    write_slot(state, op.dest, result)?;
    Ok(OpOutcome::Continue)
}

fn execute_check(state: &mut InterpreterState, op: &CheckOp) -> Result<OpOutcome, InterpreterError> {
    match op {
        CheckOp::Bool(check) => {
            let value = read_slot(state, check.operand)?;
            if !matches!(value, Value::Bool(_)) {
                return Err(InterpreterError::language(
                    check.failure_error,
                    "expected bool",
                ));
            }
        }
        CheckOp::Type(check) => {
            let value = read_slot(state, check.operand)?;
            if matches!(value, Value::None) {
                return Err(InterpreterError::language(
                    check.failure_error,
                    "type check failed",
                ));
            }
            let _ = check.expected_type;
        }
        _ => {
            return Err(InterpreterError::structural(
                vm_core::error::registry::VmStructuralErrorCode::InvalidEirError,
                "unsupported check op in bootstrap interpreter",
            ));
        }
    }
    Ok(OpOutcome::Continue)
}

fn execute_runtime_helper(
    state: &mut InterpreterState,
    op: &RuntimeHelperOp,
    executor: &mut impl UnwindExecutor,
) -> Result<OpOutcome, InterpreterError> {
    match dispatch_runtime_helper(op, state, executor)? {
        HelperBridgeOutcome::Value(value) => {
            if let Some(dest) = op.dest {
                write_slot(state, dest, value)?;
            }
            Ok(OpOutcome::Continue)
        }
        HelperBridgeOutcome::Unit => Ok(OpOutcome::Continue),
        HelperBridgeOutcome::Control(control) => Ok(OpOutcome::Control(control)),
    }
}

fn execute_safepoint(state: &mut InterpreterState, op: &SafepointOp) -> Result<OpOutcome, InterpreterError> {
    let _ = op.kind;
    state.safepoint_polls.record_loop_backedge(op.safepoint_id);
    Ok(OpOutcome::Continue)
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

fn write_slot(state: &mut InterpreterState, slot: SlotId, value: Value) -> Result<(), InterpreterError> {
    let frame = state.current_frame_mut().ok_or_else(|| {
        InterpreterError::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidFrameStateError,
            "no active frame",
        )
    })?;
    frame
        .slots
        .write(slot, value)
        .map_err(InterpreterError::from_runtime_failure)
}