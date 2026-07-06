//! ReadOnlyView runtime shell.
//!
//! Spec: `PHASE-3-READONLY-VIEW-SEMANTICS.md`

use vm_core::error::registry::RuntimeErrorCode;
use vm_core::value::Value;
use vm_diag::source_span::SourceSpanId;

use crate::heap::obj_ref::ObjRef;
use crate::heap::object::HeapObject;
use crate::heap::Heap;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};

/// Internal ReadOnlyView object payload.
#[derive(Debug, Clone, PartialEq)]
pub struct ReadOnlyViewObj {
    pub target: Value,
    pub source_span: Option<SourceSpanId>,
}

/// Construct a readonly view over a root-visible target value.
pub fn readonly_view(
    heap: &mut Heap,
    target: Value,
    source_span: Option<SourceSpanId>,
) -> RuntimeResult<ObjRef<()>> {
    heap.alloc_readonly_view(target, source_span)
}

/// Read through a view delegates to the target (shallow).
pub fn readonly_read_field(
    heap: &Heap,
    view: ObjRef<()>,
    field_index: vm_core::id::FieldIndex,
) -> RuntimeResult<Value> {
    let object = heap.resolve(view)?;
    let HeapObject::ReadOnlyView(view_obj) = object else {
        return Err(RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidObjectHandleError,
            "expected readonly view",
        ));
    };
    read_field_value(&view_obj.target, heap, field_index)
}

fn read_field_value(
    target: &Value,
    heap: &Heap,
    field_index: vm_core::id::FieldIndex,
) -> RuntimeResult<Value> {
    match target {
        Value::ObjectRef(id) => {
            let object = heap.get(*id)?;
            match object {
                HeapObject::RecordInstance { fields, .. } => {
                    let idx = field_index.0 as usize;
                    fields.get(idx).cloned().ok_or_else(|| {
                        RuntimeFailure::language(RuntimeErrorCode::FieldError)
                    })
                }
                HeapObject::ReadOnlyView(inner) => {
                    read_field_value(&inner.target, heap, field_index)
                }
                _ => Err(RuntimeFailure::structural(
                    vm_core::error::registry::VmStructuralErrorCode::InvalidObjectHandleError,
                    "field read requires record target",
                )),
            }
        }
        _ => Err(RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidObjectHandleError,
            "field read requires heap record target",
        )),
    }
}

pub fn assert_mutable_list_target(list: ObjRef<()>, heap: &Heap) -> RuntimeResult<()> {
    reject_readonly_path(list, heap, MutationKind::List)
}

pub fn assert_mutable_map_target(map: ObjRef<()>, heap: &Heap) -> RuntimeResult<()> {
    reject_readonly_path(map, heap, MutationKind::Map)
}

pub fn assert_mutable_record_target(record: ObjRef<()>, heap: &Heap) -> RuntimeResult<()> {
    reject_readonly_path(record, heap, MutationKind::Record)
}

enum MutationKind {
    List,
    Map,
    Record,
}

fn reject_readonly_path(
    obj: ObjRef<()>,
    heap: &Heap,
    kind: MutationKind,
) -> RuntimeResult<()> {
    let object = heap.resolve(obj)?;
    match (object, kind) {
        (HeapObject::ReadOnlyView(_), _) => {
            return Err(RuntimeFailure::language(RuntimeErrorCode::ReadOnlyError));
        }
        (HeapObject::List { readonly: true, .. }, MutationKind::List)
        | (HeapObject::Map { readonly: true, .. }, MutationKind::Map)
        | (HeapObject::RecordInstance { readonly: true, .. }, MutationKind::Record) => {
            return Err(RuntimeFailure::language(RuntimeErrorCode::ReadOnlyError));
        }
        (HeapObject::List { .. }, MutationKind::List)
        | (HeapObject::Map { .. }, MutationKind::Map)
        | (HeapObject::RecordInstance { .. }, MutationKind::Record) => Ok(()),
        _ => Err(RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidObjectHandleError,
            "mutation target kind mismatch",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::id::FieldIndex;
    use vm_core::value::Value;

    #[test]
    fn mutation_through_readonly_view_is_rejected() {
        let mut heap = Heap::new();
        let record_ref = heap
            .alloc_record_instance(vec![Value::Int(1)], false)
            .expect("record");
        let view = readonly_view(
            &mut heap,
            Value::ObjectRef(record_ref.id()),
            None,
        )
        .expect("view");

        let err = heap
            .record_set_field(view, FieldIndex(0), Value::Int(2))
            .unwrap_err();
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::ReadOnlyError)
        );
    }

    #[test]
    fn original_mutation_is_visible_through_view() {
        let mut heap = Heap::new();
        let record_ref = heap
            .alloc_record_instance(vec![Value::Int(1)], false)
            .expect("record");
        let view = readonly_view(
            &mut heap,
            Value::ObjectRef(record_ref.id()),
            None,
        )
        .expect("view");

        heap.record_set_field(record_ref, FieldIndex(0), Value::Int(9))
            .expect("mutate original");

        let read = readonly_read_field(&heap, view, FieldIndex(0)).expect("read");
        assert_eq!(read, Value::Int(9));
    }
}