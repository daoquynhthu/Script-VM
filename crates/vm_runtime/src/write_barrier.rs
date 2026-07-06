//! Heap write barrier hook for slot/cell mutations.
//!
//! Spec: `PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md` §2.4–§2.5,
//! `PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md` §7.4

use vm_core::value::Value;

/// Hook invoked when a cell/slot write may mutate a heap reference.
pub trait WriteBarrierHook {
    fn on_heap_mutation(&mut self, previous: Option<&Value>, new_value: &Value);
}

/// Bootstrap no-op barrier (per performance architecture: may initially be no-op).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NoopWriteBarrierHook;

impl WriteBarrierHook for NoopWriteBarrierHook {
    fn on_heap_mutation(&mut self, _previous: Option<&Value>, _new_value: &Value) {}
}

/// Whether a write may require a write barrier (heap reference introduced or replaced).
#[must_use]
pub fn value_may_mutate_heap(previous: Option<&Value>, new_value: &Value) -> bool {
    matches!(new_value, Value::ObjectRef(_))
        || previous.is_some_and(|value| matches!(value, Value::ObjectRef(_)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::id::ObjectId;

    #[test]
    fn heap_ref_write_requires_barrier_hook() {
        assert!(value_may_mutate_heap(
            None,
            &Value::ObjectRef(ObjectId::new(1))
        ));
    }

    #[test]
    fn int_write_does_not_require_barrier_hook() {
        assert!(!value_may_mutate_heap(None, &Value::Int(1)));
    }
}