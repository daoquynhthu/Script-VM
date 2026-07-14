//! Runtime value semantics.
//!
//! Spec: `PHASE-3-VALUE-KEY-STRING-SEMANTICS.md`, `PHASE-3-VM-RUNTIME-ROUND1.md` §2,
//! `PHASE-3-READONLY-VIEW-SEMANTICS.md` §4–§5

pub mod key;
pub mod string;

pub use key::{hash_key, keys_equal, value_to_key, EnumKey, FloatKey, ValueKey};
pub use string::{string_scalar_len, string_slice};

use vm_core::value::Value;

use crate::heap::object::HeapObject;
use crate::heap::Heap;
use crate::runtime_error::RuntimeResult;

/// Language-level identity comparison (`is`).
///
/// Heap objects compare by handle identity. Immediate values compare by content.
/// A `ReadOnlyView` handle is never identical to its target handle (spec §4).
#[must_use]
pub fn values_identical(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::None, Value::None) => true,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Int(x), Value::Int(y)) => x == y,
        (Value::Float(x), Value::Float(y)) => x.to_bits() == y.to_bits(),
        (Value::String(x), Value::String(y)) => x == y,
        (Value::ObjectRef(x), Value::ObjectRef(y)) => x == y,
        (Value::Error(x), Value::Error(y)) => x == y,
        _ => false,
    }
}

/// Language-level equality (`==`) with ReadOnlyView unwrap for shallow equality.
///
/// Spec: equality through ReadOnlyView delegates to target equality (§5).
/// Identity (`is`) is separate — use [`values_identical`].
pub fn values_equal(a: &Value, b: &Value, heap: &Heap) -> RuntimeResult<bool> {
    let a_u = unwrap_readonly_shallow(a, heap)?;
    let b_u = unwrap_readonly_shallow(b, heap)?;
    Ok(match (&a_u, &b_u) {
        (Value::None, Value::None) => true,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Int(x), Value::Int(y)) => x == y,
        (Value::Float(x), Value::Float(y)) => x == y || (x.is_nan() && y.is_nan()),
        (Value::String(x), Value::String(y)) => x == y,
        (Value::ObjectRef(x), Value::ObjectRef(y)) => {
            if x == y {
                true
            } else {
                // Distinct handles: compare shallow structure for bootstrap aggregates.
                match (heap.get(*x)?, heap.get(*y)?) {
                    (
                        HeapObject::List { elements: ea, .. },
                        HeapObject::List { elements: eb, .. },
                    ) => list_equal(ea, eb, heap)?,
                    (HeapObject::RecordInstance { fields: fa, .. }, HeapObject::RecordInstance { fields: fb, .. }) => {
                        list_equal(fa, fb, heap)?
                    }
                    (
                        HeapObject::EnumValue {
                            enum_id: e1,
                            case_index: c1,
                            payload: p1,
                        },
                        HeapObject::EnumValue {
                            enum_id: e2,
                            case_index: c2,
                            payload: p2,
                        },
                    ) => {
                        e1 == e2
                            && c1 == c2
                            && match (p1, p2) {
                                (None, None) => true,
                                (Some(a), Some(b)) => values_equal(a, b, heap)?,
                                _ => false,
                            }
                    }
                    _ => false,
                }
            }
        }
        (Value::Error(x), Value::Error(y)) => x == y,
        _ => false,
    })
}

fn unwrap_readonly_shallow(value: &Value, heap: &Heap) -> RuntimeResult<Value> {
    match value {
        Value::ObjectRef(id) => match heap.get(*id)? {
            HeapObject::ReadOnlyView(view) => unwrap_readonly_shallow(&view.target, heap),
            _ => Ok(value.clone()),
        },
        other => Ok(other.clone()),
    }
}

fn list_equal(a: &[Value], b: &[Value], heap: &Heap) -> RuntimeResult<bool> {
    if a.len() != b.len() {
        return Ok(false);
    }
    for (x, y) in a.iter().zip(b.iter()) {
        if !values_equal(x, y, heap)? {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::id::ObjectId;

    #[test]
    fn object_refs_identical_only_same_handle() {
        assert!(values_identical(
            &Value::ObjectRef(ObjectId::new(1)),
            &Value::ObjectRef(ObjectId::new(1))
        ));
        assert!(!values_identical(
            &Value::ObjectRef(ObjectId::new(1)),
            &Value::ObjectRef(ObjectId::new(2))
        ));
    }
}