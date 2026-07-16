//! Milestone H2 helper implementations (access, construction, numeric, display).
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md` §20.3,
//! `PHASE-3-RUNTIME-HELPER-CONTRACTS.md` §8.2–§8.3, §8.13–§8.14

use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::{CaseIndex, EnumId, FieldIndex, ObjectId};
use vm_core::value::Value;

use crate::heap::obj_ref::ObjRef;
use crate::heap::object::HeapObject;
use crate::heap::Heap;
use crate::readonly::readonly_read_field;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};
use crate::value::value_to_key;
use crate::write_barrier::{value_may_mutate_heap, WriteBarrierHook};

/// Binary numeric op tags for bootstrap argument layout.
pub const NUMERIC_OP_ADD: i64 = 0;
pub const NUMERIC_OP_SUB: i64 = 1;
pub const NUMERIC_OP_MUL: i64 = 2;
pub const NUMERIC_OP_DIV: i64 = 3;
pub const NUMERIC_OP_MOD: i64 = 4;

/// Compare op tags for bootstrap argument layout.
pub const COMPARE_OP_EQ: i64 = 0;
pub const COMPARE_OP_NE: i64 = 1;
pub const COMPARE_OP_LT: i64 = 2;
pub const COMPARE_OP_LE: i64 = 3;
pub const COMPARE_OP_GT: i64 = 4;
pub const COMPARE_OP_GE: i64 = 5;

fn type_error() -> RuntimeFailure {
    RuntimeFailure::language(RuntimeErrorCode::TypeError)
}

fn object_ref(args: &[Value], index: usize) -> RuntimeResult<ObjectId> {
    match args.get(index) {
        Some(Value::ObjectRef(id)) => Ok(*id),
        _ => Err(type_error()),
    }
}

fn int_arg(args: &[Value], index: usize) -> RuntimeResult<i64> {
    match args.get(index) {
        Some(Value::Int(v)) => Ok(*v),
        _ => Err(type_error()),
    }
}

fn require_arg<'a>(args: &'a [Value], index: usize) -> RuntimeResult<&'a Value> {
    args.get(index).ok_or_else(type_error)
}

fn obj_handle(id: ObjectId) -> ObjRef<()> {
    // Bootstrap heap always allocates generation 0.
    ObjRef::new(id, 0)
}

fn maybe_write_barrier(
    previous: Option<&Value>,
    new_value: &Value,
    hook: &mut dyn WriteBarrierHook,
) {
    if value_may_mutate_heap(previous, new_value) {
        hook.on_heap_mutation(previous, new_value);
    }
}

/// Bootstrap: `args[0]` receiver ObjectRef, `args[1]` Int field index.
/// Supports RecordInstance and ReadOnlyView over RecordInstance.
pub fn helper_get_attribute(args: &[Value], heap: &Heap) -> RuntimeResult<Value> {
    let receiver = object_ref(args, 0)?;
    let field_raw = int_arg(args, 1)?;
    if field_raw < 0 {
        return Err(RuntimeFailure::language(RuntimeErrorCode::FieldError));
    }
    let field_index = FieldIndex(field_raw as u32);
    let object = heap.get(receiver)?;
    match object {
        HeapObject::RecordInstance { fields, .. } => {
            let idx = field_index.0 as usize;
            fields
                .get(idx)
                .cloned()
                .ok_or_else(|| RuntimeFailure::language(RuntimeErrorCode::FieldError))
        }
        HeapObject::ReadOnlyView(_) => {
            readonly_read_field(heap, obj_handle(receiver), field_index)
        }
        _ => Err(type_error()),
    }
}

/// Bootstrap: `args[0]` receiver, `args[1]` Int field index, `args[2]` new value.
pub fn helper_set_attribute(
    args: &[Value],
    heap: &mut Heap,
    write_barrier: &mut dyn WriteBarrierHook,
) -> RuntimeResult<()> {
    let receiver = object_ref(args, 0)?;
    let field_raw = int_arg(args, 1)?;
    if field_raw < 0 {
        return Err(RuntimeFailure::language(RuntimeErrorCode::FieldError));
    }
    let new_value = require_arg(args, 2)?.clone();
    let field_index = FieldIndex(field_raw as u32);
    let handle = obj_handle(receiver);

    // Capture previous for barrier without holding a borrow across mutation.
    let previous = match heap.get(receiver)? {
        HeapObject::RecordInstance { fields, .. } => {
            let idx = field_index.0 as usize;
            fields.get(idx).cloned()
        }
        _ => None,
    };
    let previous_ref = previous.as_ref();
    heap.record_set_field(handle, field_index, new_value.clone())?;
    maybe_write_barrier(previous_ref, &new_value, write_barrier);
    Ok(())
}

/// Bootstrap: `args[0]` list ObjectRef → Int length.
pub fn helper_list_len(args: &[Value], heap: &Heap) -> RuntimeResult<Value> {
    let container = object_ref(args, 0)?;
    match heap.get(container)? {
        HeapObject::List { elements, .. } => Ok(Value::Int(elements.len() as i64)),
        _ => Err(type_error()),
    }
}

/// Bootstrap: `args[0]` container, `args[1]` index/key.
/// List[Int] and Map[Hashable] only.
pub fn helper_index_read(args: &[Value], heap: &Heap) -> RuntimeResult<Value> {
    let container = object_ref(args, 0)?;
    let key = require_arg(args, 1)?;
    match heap.get(container)? {
        HeapObject::List { elements, .. } => {
            let index = match key {
                Value::Int(i) if *i >= 0 => *i as usize,
                Value::Int(_) => {
                    return Err(RuntimeFailure::language(RuntimeErrorCode::IndexError));
                }
                _ => return Err(type_error()),
            };
            elements
                .get(index)
                .cloned()
                .ok_or_else(|| RuntimeFailure::language(RuntimeErrorCode::IndexError))
        }
        HeapObject::Map { entries, .. } => {
            let canonical = value_to_key(key, heap)?;
            entries
                .iter()
                .find(|(k, _)| k == &canonical)
                .map(|(_, v)| v.clone())
                .ok_or_else(|| RuntimeFailure::language(RuntimeErrorCode::KeyError))
        }
        _ => Err(type_error()),
    }
}

/// Bootstrap: `args[0]` container, `args[1]` index/key, `args[2]` value.
pub fn helper_index_write(
    args: &[Value],
    heap: &mut Heap,
    write_barrier: &mut dyn WriteBarrierHook,
) -> RuntimeResult<()> {
    let container = object_ref(args, 0)?;
    let key = require_arg(args, 1)?.clone();
    let new_value = require_arg(args, 2)?.clone();
    let handle = obj_handle(container);

    let previous = match heap.get(container)? {
        HeapObject::List { elements, .. } => {
            let index = match &key {
                Value::Int(i) if *i >= 0 => *i as usize,
                Value::Int(_) => {
                    return Err(RuntimeFailure::language(RuntimeErrorCode::IndexError));
                }
                _ => return Err(type_error()),
            };
            elements.get(index).cloned()
        }
        HeapObject::Map { entries, .. } => {
            let canonical = value_to_key(&key, heap)?;
            entries
                .iter()
                .find(|(k, _)| k == &canonical)
                .map(|(_, v)| v.clone())
        }
        _ => return Err(type_error()),
    };

    match heap.get(container)? {
        HeapObject::List { .. } => {
            let index = match &key {
                Value::Int(i) if *i >= 0 => *i as usize,
                _ => unreachable!("validated above"),
            };
            heap.list_set(handle, index, new_value.clone())?;
        }
        HeapObject::Map { .. } => {
            heap.map_insert(handle, key, new_value.clone())?;
        }
        _ => return Err(type_error()),
    }
    maybe_write_barrier(previous.as_ref(), &new_value, write_barrier);
    Ok(())
}

/// Bootstrap: `args[0]` list ObjectRef or String, `args[1]` start Int, `args[2]` end Int.
/// Half-open `[start, end)`; negative bounds and out-of-range raise IndexError.
pub fn helper_slice_read(args: &[Value], heap: &mut Heap) -> RuntimeResult<Value> {
    let start = int_arg(args, 1)?;
    let end = int_arg(args, 2)?;
    if start < 0 || end < 0 {
        return Err(RuntimeFailure::language(RuntimeErrorCode::IndexError));
    }
    let start_u = start as usize;
    let end_u = end as usize;
    if start_u > end_u {
        return Err(RuntimeFailure::language(RuntimeErrorCode::IndexError));
    }

    match args.first() {
        Some(Value::String(text)) => {
            let len = text.chars().count();
            if end_u > len {
                return Err(RuntimeFailure::language(RuntimeErrorCode::IndexError));
            }
            let sliced: String = text.chars().skip(start_u).take(end_u - start_u).collect();
            Ok(Value::String(sliced))
        }
        Some(Value::ObjectRef(id)) => {
            let elements = match heap.get(*id)? {
                HeapObject::List { elements, .. } => elements.clone(),
                _ => return Err(type_error()),
            };
            if end_u > elements.len() {
                return Err(RuntimeFailure::language(RuntimeErrorCode::IndexError));
            }
            let slice = elements[start_u..end_u].to_vec();
            let list = heap.alloc_list(slice, false)?;
            Ok(Value::ObjectRef(list.id()))
        }
        _ => Err(type_error()),
    }
}

/// Bootstrap: all args are field values in shape order; constructs mutable record.
pub fn helper_construct_record(args: &[Value], heap: &mut Heap) -> RuntimeResult<Value> {
    let fields = args.to_vec();
    let record = heap.alloc_record_instance(fields, false)?;
    Ok(Value::ObjectRef(record.id()))
}

/// Bootstrap: `args[0]` Int enum_id, `args[1]` Int case_index, optional `args[2]` payload.
pub fn helper_construct_enum(args: &[Value], heap: &mut Heap) -> RuntimeResult<Value> {
    let enum_raw = int_arg(args, 0)?;
    let case_raw = int_arg(args, 1)?;
    if enum_raw < 0 || case_raw < 0 {
        return Err(type_error());
    }
    let payload = args.get(2).cloned();
    let enum_value = heap.alloc_enum_value(
        EnumId::new(enum_raw as u32),
        CaseIndex(case_raw as u32),
        payload,
    )?;
    Ok(Value::ObjectRef(enum_value.id()))
}

/// Bootstrap: alternating key/value pairs; later value replaces earlier at first position.
pub fn helper_construct_map(args: &[Value], heap: &mut Heap) -> RuntimeResult<Value> {
    if args.len() % 2 != 0 {
        return Err(type_error());
    }
    let map = heap.alloc_map(false)?;
    for pair in args.chunks_exact(2) {
        heap.map_insert(map, pair[0].clone(), pair[1].clone())?;
    }
    Ok(Value::ObjectRef(map.id()))
}

/// Bootstrap: `args[0]` Int op tag, `args[1]` lhs, `args[2]` rhs.
/// No implicit coercion; checked Int overflow; division by zero raises.
pub fn helper_numeric_binary(args: &[Value]) -> RuntimeResult<Value> {
    let op = int_arg(args, 0)?;
    let lhs = require_arg(args, 1)?;
    let rhs = require_arg(args, 2)?;
    match (lhs, rhs) {
        (Value::Int(a), Value::Int(b)) => numeric_int(op, *a, *b),
        (Value::Float(a), Value::Float(b)) => numeric_float(op, *a, *b),
        _ => Err(type_error()),
    }
}

fn numeric_int(op: i64, a: i64, b: i64) -> RuntimeResult<Value> {
    let result = match op {
        NUMERIC_OP_ADD => a
            .checked_add(b)
            .ok_or(RuntimeFailure::language(RuntimeErrorCode::NumericOverflowError))?,
        NUMERIC_OP_SUB => a
            .checked_sub(b)
            .ok_or(RuntimeFailure::language(RuntimeErrorCode::NumericOverflowError))?,
        NUMERIC_OP_MUL => a
            .checked_mul(b)
            .ok_or(RuntimeFailure::language(RuntimeErrorCode::NumericOverflowError))?,
        NUMERIC_OP_DIV => {
            if b == 0 {
                return Err(RuntimeFailure::language(RuntimeErrorCode::DivisionByZeroError));
            }
            a.checked_div(b)
                .ok_or(RuntimeFailure::language(RuntimeErrorCode::NumericOverflowError))?
        }
        NUMERIC_OP_MOD => {
            if b == 0 {
                return Err(RuntimeFailure::language(RuntimeErrorCode::DivisionByZeroError));
            }
            a.checked_rem(b)
                .ok_or(RuntimeFailure::language(RuntimeErrorCode::NumericOverflowError))?
        }
        _ => return Err(type_error()),
    };
    Ok(Value::Int(result))
}

fn numeric_float(op: i64, a: f64, b: f64) -> RuntimeResult<Value> {
    let result = match op {
        NUMERIC_OP_ADD => a + b,
        NUMERIC_OP_SUB => a - b,
        NUMERIC_OP_MUL => a * b,
        NUMERIC_OP_DIV => {
            if b == 0.0 {
                return Err(RuntimeFailure::language(RuntimeErrorCode::DivisionByZeroError));
            }
            a / b
        }
        NUMERIC_OP_MOD => {
            if b == 0.0 {
                return Err(RuntimeFailure::language(RuntimeErrorCode::DivisionByZeroError));
            }
            a % b
        }
        _ => return Err(type_error()),
    };
    Ok(Value::Float(result))
}

/// Bootstrap: `args[0]` Int compare op, `args[1]` lhs, `args[2]` rhs → Bool.
/// Unsupported comparisons raise TypeError.
pub fn helper_compare(args: &[Value]) -> RuntimeResult<Value> {
    let op = int_arg(args, 0)?;
    let lhs = require_arg(args, 1)?;
    let rhs = require_arg(args, 2)?;
    let ordered = match (lhs, rhs) {
        (Value::Int(a), Value::Int(b)) => compare_ord(op, a.cmp(b))?,
        (Value::Float(a), Value::Float(b)) => {
            if a.is_nan() || b.is_nan() {
                // Equality with NaN is false; ordering involving NaN is TypeError-like unsupported.
                match op {
                    COMPARE_OP_EQ => false,
                    COMPARE_OP_NE => true,
                    _ => return Err(type_error()),
                }
            } else {
                compare_ord(op, a.partial_cmp(b).expect("finite floats comparable"))?
            }
        }
        (Value::Bool(a), Value::Bool(b)) => match op {
            COMPARE_OP_EQ => a == b,
            COMPARE_OP_NE => a != b,
            _ => return Err(type_error()),
        },
        (Value::String(a), Value::String(b)) => compare_ord(op, a.cmp(b))?,
        (Value::None, Value::None) => match op {
            COMPARE_OP_EQ => true,
            COMPARE_OP_NE => false,
            _ => return Err(type_error()),
        },
        _ => match op {
            COMPARE_OP_EQ => false,
            COMPARE_OP_NE => true,
            _ => return Err(type_error()),
        },
    };
    Ok(Value::Bool(ordered))
}

fn compare_ord(op: i64, ord: std::cmp::Ordering) -> RuntimeResult<bool> {
    use std::cmp::Ordering::*;
    Ok(match op {
        COMPARE_OP_EQ => ord == Equal,
        COMPARE_OP_NE => ord != Equal,
        COMPARE_OP_LT => ord == Less,
        COMPARE_OP_LE => matches!(ord, Less | Equal),
        COMPARE_OP_GT => ord == Greater,
        COMPARE_OP_GE => matches!(ord, Greater | Equal),
        _ => return Err(type_error()),
    })
}

/// Bootstrap: `args[0]` value → display String (no operator coercion side effects).
pub fn helper_display(args: &[Value], heap: &Heap) -> RuntimeResult<Value> {
    let value = require_arg(args, 0)?;
    Ok(Value::String(display_value(value, heap)?))
}

fn display_value(value: &Value, heap: &Heap) -> RuntimeResult<String> {
    Ok(match value {
        Value::None => "None".to_string(),
        Value::Bool(b) => if *b { "true" } else { "false" }.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::Error(_) => "<Error>".to_string(),
        Value::ObjectRef(id) => match heap.get(*id)? {
            HeapObject::List { elements, .. } => {
                let parts: RuntimeResult<Vec<_>> =
                    elements.iter().map(|e| display_value(e, heap)).collect();
                format!("[{}]", parts?.join(", "))
            }
            HeapObject::Map { entries, .. } => {
                format!("{{map:{} entries}}", entries.len())
            }
            HeapObject::RecordInstance { fields, .. } => {
                format!("{{record:{} fields}}", fields.len())
            }
            HeapObject::ReadOnlyView(view) => {
                format!("readonly({})", display_value(&view.target, heap)?)
            }
            HeapObject::EnumValue {
                enum_id,
                case_index,
                ..
            } => format!("Enum({}.{})", enum_id.raw(), case_index.0),
            HeapObject::Function => "<Function>".to_string(),
            HeapObject::Module => "<Module>".to_string(),
            HeapObject::Resource => "<Resource>".to_string(),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::write_barrier::NoopWriteBarrierHook;

    #[test]
    fn get_attribute_reads_record_field() {
        let mut heap = Heap::new();
        let rec = heap
            .alloc_record_instance(vec![Value::Int(9), Value::String("x".into())], false)
            .expect("record");
        let v = helper_get_attribute(&[Value::ObjectRef(rec.id()), Value::Int(0)], &heap)
            .expect("get");
        assert_eq!(v, Value::Int(9));
    }

    #[test]
    fn set_attribute_writes_record_field() {
        let mut heap = Heap::new();
        let mut barrier = NoopWriteBarrierHook;
        let rec = heap
            .alloc_record_instance(vec![Value::Int(1)], false)
            .expect("record");
        helper_set_attribute(
            &[Value::ObjectRef(rec.id()), Value::Int(0), Value::Int(42)],
            &mut heap,
            &mut barrier,
        )
        .expect("set");
        let v = helper_get_attribute(&[Value::ObjectRef(rec.id()), Value::Int(0)], &heap)
            .expect("get");
        assert_eq!(v, Value::Int(42));
    }

    #[test]
    fn numeric_binary_add_int() {
        let v = helper_numeric_binary(&[
            Value::Int(NUMERIC_OP_ADD),
            Value::Int(2),
            Value::Int(3),
        ])
        .expect("add");
        assert_eq!(v, Value::Int(5));
    }

    #[test]
    fn numeric_binary_div_zero() {
        let err = helper_numeric_binary(&[
            Value::Int(NUMERIC_OP_DIV),
            Value::Int(1),
            Value::Int(0),
        ])
        .expect_err("div0");
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::DivisionByZeroError)
        );
    }

    #[test]
    fn compare_int_lt() {
        let v = helper_compare(&[
            Value::Int(COMPARE_OP_LT),
            Value::Int(1),
            Value::Int(2),
        ])
        .expect("lt");
        assert_eq!(v, Value::Bool(true));
    }

    #[test]
    fn display_int() {
        let heap = Heap::new();
        let v = helper_display(&[Value::Int(7)], &heap).expect("display");
        assert_eq!(v, Value::String("7".into()));
    }
}
