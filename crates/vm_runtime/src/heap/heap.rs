//! Arena-backed heap with generational handles.
//!
//! Spec: `PHASE-3-VM-RUNTIME-ROUND1.md` §3, `PHASE-3-GC-METADATA-OWNERSHIP.md`

use vm_core::error::registry::VmStructuralErrorCode;
use vm_core::id::{CaseIndex, EnumId, ObjectId};
use vm_core::value::Value;

use super::obj_ref::ObjRef;
use super::object::HeapObject;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};
use crate::value::value_to_key;

#[derive(Debug)]
struct HeapSlot {
    generation: u32,
    object: HeapObject,
}

/// Bootstrap heap (non-moving; GC hooks preserved for later phases).
#[derive(Debug, Default)]
pub struct Heap {
    slots: Vec<HeapSlot>,
}

impl Heap {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn alloc_object(&mut self, object: HeapObject) -> ObjRef<()> {
        let index = self.slots.len() as u32;
        self.slots.push(HeapSlot {
            generation: 0,
            object,
        });
        ObjRef::new(ObjectId::new(index), 0)
    }

    pub fn alloc_list(&mut self, elements: Vec<Value>, readonly: bool) -> RuntimeResult<ObjRef<()>> {
        Ok(self.alloc_object(HeapObject::List { elements, readonly }))
    }

    pub fn alloc_map(&mut self, readonly: bool) -> RuntimeResult<ObjRef<()>> {
        Ok(self.alloc_object(HeapObject::Map {
            entries: Vec::new(),
            readonly,
        }))
    }

    pub fn alloc_readonly_view(
        &mut self,
        target: Value,
        source_span: Option<vm_diag::source_span::SourceSpanId>,
    ) -> RuntimeResult<ObjRef<()>> {
        Ok(self.alloc_object(HeapObject::ReadOnlyView(
            crate::readonly::ReadOnlyViewObj { target, source_span },
        )))
    }

    pub fn alloc_enum_value(
        &mut self,
        enum_id: EnumId,
        case_index: CaseIndex,
        payload: Option<Value>,
    ) -> RuntimeResult<ObjRef<()>> {
        Ok(self.alloc_object(HeapObject::EnumValue {
            enum_id,
            case_index,
            payload,
        }))
    }

    pub fn alloc_record_instance(
        &mut self,
        fields: Vec<Value>,
        readonly: bool,
    ) -> RuntimeResult<ObjRef<()>> {
        Ok(self.alloc_object(HeapObject::RecordInstance { fields, readonly }))
    }

    pub fn get(&self, id: ObjectId) -> RuntimeResult<&HeapObject> {
        let slot = self.resolve_slot(id, 0)?;
        Ok(&slot.object)
    }

    pub fn get_mut(&mut self, id: ObjectId) -> RuntimeResult<&mut HeapObject> {
        let index = Self::slot_index(id)?;
        if !id.is_valid() {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidObjectHandleError,
                "invalid object id",
            ));
        }
        let slot = self
            .slots
            .get_mut(index)
            .ok_or_else(|| {
                RuntimeFailure::structural(
                    VmStructuralErrorCode::InvalidObjectHandleError,
                    "object id out of range",
                )
            })?;
        Ok(&mut slot.object)
    }

    pub fn resolve(&self, obj: ObjRef<()>) -> RuntimeResult<&HeapObject> {
        let slot = self.resolve_slot(obj.id(), obj.generation())?;
        Ok(&slot.object)
    }

    pub fn resolve_mut(&mut self, obj: ObjRef<()>) -> RuntimeResult<&mut HeapObject> {
        let index = Self::slot_index(obj.id())?;
        let slot = self
            .slots
            .get_mut(index)
            .ok_or_else(|| {
                RuntimeFailure::structural(
                    VmStructuralErrorCode::InvalidObjectHandleError,
                    "object id out of range",
                )
            })?;
        if slot.generation != obj.generation() {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidObjectHandleError,
                "stale object generation",
            ));
        }
        Ok(&mut slot.object)
    }

    pub fn map_insert(
        &mut self,
        map: ObjRef<()>,
        key: Value,
        value: Value,
    ) -> RuntimeResult<()> {
        crate::readonly::assert_mutable_map_target(map, self)?;
        let canonical = value_to_key(&key, self)?;
        let object = self.resolve_mut(map)?;
        let HeapObject::Map { entries, .. } = object else {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidObjectHandleError,
                "expected map object",
            ));
        };

        if let Some((_, existing)) = entries.iter_mut().find(|(k, _)| k == &canonical) {
            *existing = value;
        } else {
            entries.push((canonical, value));
        }
        Ok(())
    }

    pub fn list_set(
        &mut self,
        list: ObjRef<()>,
        index: usize,
        value: Value,
    ) -> RuntimeResult<()> {
        crate::readonly::assert_mutable_list_target(list, self)?;
        let object = self.resolve_mut(list)?;
        let HeapObject::List { elements, .. } = object else {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidObjectHandleError,
                "expected list object",
            ));
        };
        let element = elements.get_mut(index).ok_or_else(|| {
            RuntimeFailure::language(vm_core::error::registry::RuntimeErrorCode::IndexError)
        })?;
        *element = value;
        Ok(())
    }

    pub fn record_set_field(
        &mut self,
        record: ObjRef<()>,
        field_index: vm_core::id::FieldIndex,
        value: Value,
    ) -> RuntimeResult<()> {
        crate::readonly::assert_mutable_record_target(record, self)?;
        let object = self.resolve_mut(record)?;
        let HeapObject::RecordInstance { fields, .. } = object else {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidObjectHandleError,
                "expected record instance",
            ));
        };
        let idx = field_index.0 as usize;
        let field = fields.get_mut(idx).ok_or_else(|| {
            RuntimeFailure::language(vm_core::error::registry::RuntimeErrorCode::FieldError)
        })?;
        *field = value;
        Ok(())
    }

    fn resolve_slot(&self, id: ObjectId, generation: u32) -> RuntimeResult<&HeapSlot> {
        if !id.is_valid() {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidObjectHandleError,
                "invalid object id sentinel",
            ));
        }
        let index = Self::slot_index(id)?;
        let slot = self.slots.get(index).ok_or_else(|| {
            RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidObjectHandleError,
                "object id out of range",
            )
        })?;
        if slot.generation != generation {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidObjectHandleError,
                "stale object generation",
            ));
        }
        Ok(slot)
    }

    fn slot_index(id: ObjectId) -> RuntimeResult<usize> {
        usize::try_from(id.raw()).map_err(|_| {
            RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidObjectHandleError,
                "object id does not fit platform usize",
            )
        })
    }

    #[cfg(test)]
    pub fn invalidate_for_test(&mut self, id: ObjectId) {
        if let Ok(index) = Self::slot_index(id) {
            if let Some(slot) = self.slots.get_mut(index) {
                slot.generation = slot.generation.wrapping_add(1);
            }
        }
    }
}

/// Whether a value refers to a mutable aggregate (directly or through a view).
#[must_use]
pub fn value_is_mutable_aggregate(value: &Value, heap: &Heap) -> bool {
    match value {
        Value::ObjectRef(id) => heap
            .get(*id)
            .ok()
            .is_some_and(|obj| match obj {
                HeapObject::ReadOnlyView(view) => value_is_mutable_aggregate(&view.target, heap),
                other => other.kind().is_mutable_aggregate(),
            }),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::value::Value;

    #[test]
    fn invalid_object_handle_is_rejected() {
        let heap = Heap::new();
        let err = heap.get(ObjectId::INVALID).unwrap_err();
        assert!(matches!(
            err,
            RuntimeFailure::Structural(ref e) if e.code == VmStructuralErrorCode::InvalidObjectHandleError
        ));
    }

    #[test]
    fn stale_generation_is_rejected() {
        let mut heap = Heap::new();
        let list = heap.alloc_list(vec![], false).expect("alloc");
        heap.invalidate_for_test(list.id());
        let err = heap.resolve(list).unwrap_err();
        assert!(matches!(
            err,
            RuntimeFailure::Structural(ref e) if e.code == VmStructuralErrorCode::InvalidObjectHandleError
        ));
    }

    #[test]
    fn map_insert_rejects_non_hashable_key() {
        let mut heap = Heap::new();
        let map = heap.alloc_map(false).expect("alloc");
        let list = heap.alloc_list(vec![], false).expect("list");
        let err = heap
            .map_insert(map, Value::ObjectRef(list.id()), Value::Int(1))
            .unwrap_err();
        assert_eq!(
            err,
            RuntimeFailure::language(vm_core::error::registry::RuntimeErrorCode::TypeError)
        );
    }
}