//! Remaining bootstrap helpers outside H1–H7 milestones.
//!
//! Spec: PHASE-3-RUNTIME-HELPER-REGISTRY.md §3,
//! PHASE-3-RUNTIME-HELPER-CONTRACTS.md access/construction/display/numeric.

use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::{BindingId, SlotId as CoreSlotId};
use vm_core::runtime_plan::schema::Mutability;
use vm_core::value::Value;

use crate::binding_cell::CellOwner;
use crate::call::contract::TypeContractChecker;
use crate::frame::SlotArray;
use crate::heap::object::HeapObject;
use crate::heap::Heap;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};
use crate::value::value_to_key;
use crate::write_barrier::WriteBarrierHook;

fn type_error() -> RuntimeFailure {
    RuntimeFailure::language(RuntimeErrorCode::TypeError)
}

/// Bootstrap unary numeric: `args[0]` op Int (0=neg, 1=abs), `args[1]` operand.
pub fn helper_numeric_unary(args: &[Value]) -> RuntimeResult<Value> {
    let op = match args.first() {
        Some(Value::Int(v)) => *v,
        _ => return Err(type_error()),
    };
    match (op, args.get(1)) {
        (0, Some(Value::Int(v))) => Ok(Value::Int(v.checked_neg().ok_or_else(|| {
            RuntimeFailure::language(RuntimeErrorCode::NumericOverflowError)
        })?)),
        (0, Some(Value::Float(v))) => Ok(Value::Float(-*v)),
        (1, Some(Value::Int(v))) => Ok(Value::Int(v.saturating_abs())),
        (1, Some(Value::Float(v))) => Ok(Value::Float(v.abs())),
        _ => Err(type_error()),
    }
}

/// Bootstrap membership: `args[0]` container, `args[1]` item → Bool.
pub fn helper_membership(args: &[Value], heap: &Heap) -> RuntimeResult<Value> {
    let container = args.first().ok_or_else(type_error)?;
    let item = args.get(1).ok_or_else(type_error)?;
    match container {
        Value::String(s) => {
            let needle = match item {
                Value::String(n) => n.as_str(),
                _ => return Err(type_error()),
            };
            Ok(Value::Bool(s.contains(needle)))
        }
        Value::ObjectRef(id) => match heap.get(*id)? {
            HeapObject::List { elements, .. } => Ok(Value::Bool(elements.iter().any(|e| e == item))),
            HeapObject::Map { entries, .. } => {
                let key = value_to_key(item, heap)?;
                Ok(Value::Bool(entries.iter().any(|(k, _)| k == &key)))
            }
            _ => Err(type_error()),
        },
        _ => Err(type_error()),
    }
}

/// Bootstrap: all args are list elements.
pub fn helper_construct_list(args: &[Value], heap: &mut Heap) -> RuntimeResult<Value> {
    let list = heap.alloc_list(args.to_vec(), false)?;
    Ok(Value::ObjectRef(list.id()))
}

/// Bootstrap: no args → function shell object.
pub fn helper_construct_function(args: &[Value], heap: &mut Heap) -> RuntimeResult<Value> {
    if !args.is_empty() {
        return Err(type_error());
    }
    let f = heap.alloc_function()?;
    Ok(Value::ObjectRef(f.id()))
}

/// Bootstrap: `args[0]` lhs String, `args[1]` rhs String.
pub fn helper_string_concat(args: &[Value]) -> RuntimeResult<Value> {
    let a = match args.first() {
        Some(Value::String(s)) => s,
        _ => return Err(type_error()),
    };
    let b = match args.get(1) {
        Some(Value::String(s)) => s,
        _ => return Err(type_error()),
    };
    Ok(Value::String(format!("{a}{b}")))
}

/// Bootstrap load_cell: requires external SlotArray via optional pointer pattern —
/// dispatch uses env-less API: `args[0]` Int slot id against provided slots.
pub fn helper_load_cell(args: &[Value], slots: &SlotArray) -> RuntimeResult<Value> {
    let slot = match args.first() {
        Some(Value::Int(v)) if *v >= 0 => CoreSlotId::new(*v as u32),
        _ => return Err(type_error()),
    };
    slots.read_cell(slot)
}

/// Bootstrap store_cell: slot Int, value.
pub fn helper_store_cell(
    args: &[Value],
    slots: &mut SlotArray,
    checker: &(impl TypeContractChecker + ?Sized),
    barrier: &mut dyn WriteBarrierHook,
) -> RuntimeResult<()> {
    let slot = match args.first() {
        Some(Value::Int(v)) if *v >= 0 => CoreSlotId::new(*v as u32),
        _ => return Err(type_error()),
    };
    let value = args.get(1).cloned().ok_or_else(type_error)?;
    slots.write_cell(slot, value, checker, barrier)
}

/// Ensure a mutable cell exists on slot for tests / bootstrap.
pub fn ensure_mutable_cell(
    slots: &mut SlotArray,
    slot: CoreSlotId,
    initial: Value,
) -> RuntimeResult<()> {
    slots.bind_cell_with_value(
        slot,
        BindingId::new(slot.raw()),
        Mutability::Mutable,
        CellOwner::LocalCapture,
        initial,
    )?;
    Ok(())
}

/// Bootstrap: `args[0]` Int module_id, `args[1]` Int slot_id → value from module slots.
pub fn helper_load_module_slot(
    args: &[Value],
    runtime: Option<&crate::module::runtime::ModuleRuntime>,
) -> RuntimeResult<Value> {
    let runtime = runtime.ok_or_else(|| {
        RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidFrameStateError,
            "load_module_slot requires ModuleRuntime",
        )
    })?;
    let module_id = match args.first() {
        Some(Value::Int(v)) if *v >= 0 => vm_core::id::ModuleId::new(*v as u32),
        _ => return Err(type_error()),
    };
    let slot = match args.get(1) {
        Some(Value::Int(v)) if *v >= 0 => CoreSlotId::new(*v as u32),
        _ => return Err(type_error()),
    };
    let instance = runtime.registry.get(module_id).ok_or_else(|| {
        RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidRuntimePlanError,
            "unknown module",
        )
    })?;
    instance.module_slots.read(slot)
}

/// Bootstrap pattern match: `args[0]` subject, `args[1]` Int tag
/// 0=wildcard → true, 1=literal equals args[2], 2=type tag (0 none,1 bool,2 int,3 string).
/// Returns Bool (MatchCase-style); does not implement full destructuring raise mode.
pub fn helper_match_pattern(args: &[Value]) -> RuntimeResult<Value> {
    let subject = args.first().ok_or_else(type_error)?;
    let tag = match args.get(1) {
        Some(Value::Int(v)) => *v,
        _ => return Err(type_error()),
    };
    let matched = match tag {
        0 => true, // wildcard
        1 => args.get(2).is_some_and(|lit| lit == subject),
        2 => {
            let kind = match args.get(2) {
                Some(Value::Int(k)) => *k,
                _ => return Err(type_error()),
            };
            match (kind, subject) {
                (0, Value::None) => true,
                (1, Value::Bool(_)) => true,
                (2, Value::Int(_)) => true,
                (3, Value::String(_)) => true,
                _ => false,
            }
        }
        _ => return Err(type_error()),
    };
    Ok(Value::Bool(matched))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_unary_neg_and_abs() {
        assert_eq!(
            helper_numeric_unary(&[Value::Int(0), Value::Int(3)]).unwrap(),
            Value::Int(-3)
        );
        assert_eq!(
            helper_numeric_unary(&[Value::Int(1), Value::Int(-4)]).unwrap(),
            Value::Int(4)
        );
    }

    #[test]
    fn construct_list_and_membership() {
        let mut heap = Heap::new();
        let list = helper_construct_list(&[Value::Int(1), Value::Int(2)], &mut heap).unwrap();
        let Value::ObjectRef(id) = list else {
            panic!();
        };
        assert_eq!(
            helper_membership(&[Value::ObjectRef(id), Value::Int(2)], &heap).unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            helper_membership(&[Value::ObjectRef(id), Value::Int(9)], &heap).unwrap(),
            Value::Bool(false)
        );
    }

    #[test]
    fn string_concat() {
        assert_eq!(
            helper_string_concat(&[Value::String("a".into()), Value::String("b".into())]).unwrap(),
            Value::String("ab".into())
        );
    }
}
