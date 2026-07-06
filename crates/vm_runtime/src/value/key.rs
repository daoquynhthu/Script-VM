//! ValueKey hashability and map key canonicalization.
//!
//! Spec: `PHASE-3-VALUE-KEY-STRING-SEMANTICS.md` §1–§7

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use vm_core::id::{CaseIndex, EnumId, ObjectId};
use vm_core::value::Value;

use crate::heap::object::HeapObject;
use crate::heap::{value_is_mutable_aggregate, Heap};
use crate::runtime_error::{RuntimeFailure, RuntimeResult};

/// Canonical map key representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueKey {
    Nil,
    Bool(bool),
    Int(i64),
    Float(FloatKey),
    String(String),
    Enum(EnumKey),
}

/// Canonical finite float key (NaN forbidden).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatKey(f64);

impl Eq for FloatKey {}

impl Hash for FloatKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        canonical_float_bits(self.0).hash(state);
    }
}

impl FloatKey {
    #[must_use]
    pub fn from_f64(value: f64) -> Option<Self> {
        if value.is_nan() {
            None
        } else {
            Some(Self(value))
        }
    }

    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// Enum identity key when payload is absent or hashable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumKey {
    pub enum_id: EnumId,
    pub case_index: CaseIndex,
    pub payload: Option<Box<ValueKey>>,
}

/// Convert a runtime value to a canonical map key.
pub fn value_to_key(value: &Value, heap: &Heap) -> RuntimeResult<ValueKey> {
    match value {
        Value::None => Ok(ValueKey::Nil),
        Value::Bool(b) => Ok(ValueKey::Bool(*b)),
        Value::Int(i) => Ok(ValueKey::Int(*i)),
        Value::Float(f) => FloatKey::from_f64(*f)
            .map(ValueKey::Float)
            .ok_or(RuntimeFailure::language(vm_core::error::registry::RuntimeErrorCode::TypeError)),
        Value::String(s) => Ok(ValueKey::String(s.clone())),
        Value::ObjectRef(id) => key_from_heap_object(*id, heap),
        Value::Error(_) => Err(RuntimeFailure::language(
            vm_core::error::registry::RuntimeErrorCode::TypeError,
        )),
    }
}

fn key_from_heap_object(id: ObjectId, heap: &Heap) -> RuntimeResult<ValueKey> {
    let object = heap.get(id)?;
    match object {
        HeapObject::EnumValue {
            enum_id,
            case_index,
            payload,
        } => {
            let payload_key = match payload {
                None => None,
                Some(value) => Some(Box::new(value_to_key(value, heap)?)),
            };
            Ok(ValueKey::Enum(EnumKey {
                enum_id: *enum_id,
                case_index: *case_index,
                payload: payload_key,
            }))
        }
        HeapObject::ReadOnlyView(view) => {
            if value_is_mutable_aggregate(&view.target, heap) {
                return Err(RuntimeFailure::language(
                    vm_core::error::registry::RuntimeErrorCode::TypeError,
                ));
            }
            value_to_key(&view.target, heap)
        }
        _ => Err(RuntimeFailure::language(
            vm_core::error::registry::RuntimeErrorCode::TypeError,
        )),
    }
}

/// Hash a canonical key for map storage.
#[must_use]
pub fn hash_key(key: &ValueKey) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

/// Keys compare equal iff canonical ValueKey equality holds.
#[must_use]
pub fn keys_equal(a: &ValueKey, b: &ValueKey) -> bool {
    a == b
}

fn canonical_float_bits(value: f64) -> u64 {
    if value == 0.0 {
        0.0f64.to_bits()
    } else {
        value.to_bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::value::Value;

    #[test]
    fn nil_bool_int_string_are_hashable() {
        let heap = Heap::new();
        assert!(value_to_key(&Value::None, &heap).is_ok());
        assert!(value_to_key(&Value::Bool(true), &heap).is_ok());
        assert!(value_to_key(&Value::Int(42), &heap).is_ok());
        assert!(value_to_key(&Value::String("k".into()), &heap).is_ok());
    }

    #[test]
    fn nan_float_key_is_rejected() {
        let heap = Heap::new();
        let err = value_to_key(&Value::Float(f64::NAN), &heap).unwrap_err();
        assert_eq!(
            err,
            RuntimeFailure::language(vm_core::error::registry::RuntimeErrorCode::TypeError)
        );
    }

    #[test]
    fn negative_zero_and_zero_hash_equally() {
        let zero = FloatKey::from_f64(0.0).expect("zero");
        let neg_zero = FloatKey::from_f64(-0.0).expect("neg zero");
        assert_eq!(hash_key(&ValueKey::Float(zero)), hash_key(&ValueKey::Float(neg_zero)));
    }

    #[test]
    fn list_object_is_non_hashable() {
        let mut heap = Heap::new();
        let list = heap.alloc_list(vec![Value::Int(1)], false).expect("alloc");
        let err = value_to_key(&Value::ObjectRef(list.id()), &heap).unwrap_err();
        assert_eq!(
            err,
            RuntimeFailure::language(vm_core::error::registry::RuntimeErrorCode::TypeError)
        );
    }

    #[test]
    fn equal_keys_imply_equal_hash() {
        let a = ValueKey::Int(7);
        let b = ValueKey::Int(7);
        assert!(keys_equal(&a, &b));
        assert_eq!(hash_key(&a), hash_key(&b));
    }
}