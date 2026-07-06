//! Frame and slot storage.
//!
//! Spec: `PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md` §2, `PHASE-3-VM-RUNTIME-ROUND1.md`

use vm_core::error::registry::{RuntimeErrorCode, VmStructuralErrorCode};
use vm_core::id::{FrameId, SlotId};
use vm_core::value::Value;

use crate::control::PendingControl;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};

/// Slot initialization state.
#[derive(Debug, Clone, PartialEq)]
pub enum SlotState {
    Uninitialized,
    Value(Value),
}

/// Fixed-size slot array for an interpreter frame.
#[derive(Debug, Clone, PartialEq)]
pub struct SlotArray {
    slots: Vec<SlotState>,
}

impl SlotArray {
    #[must_use]
    pub fn with_capacity(count: usize) -> Self {
        Self {
            slots: vec![SlotState::Uninitialized; count],
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn write(&mut self, slot_id: SlotId, value: Value) -> RuntimeResult<()> {
        let index = slot_index(slot_id, self.slots.len())?;
        self.slots[index] = SlotState::Value(value);
        Ok(())
    }

    pub fn read(&self, slot_id: SlotId) -> RuntimeResult<Value> {
        let index = slot_index(slot_id, self.slots.len())?;
        match &self.slots[index] {
            SlotState::Uninitialized => Err(RuntimeFailure::language(
                RuntimeErrorCode::UninitializedBindingError,
            )),
            SlotState::Value(value) => Ok(value.clone()),
        }
    }

    pub fn state(&self, slot_id: SlotId) -> RuntimeResult<&SlotState> {
        let index = slot_index(slot_id, self.slots.len())?;
        Ok(&self.slots[index])
    }
}

/// Interpreter activation record.
#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub frame_id: FrameId,
    pub slots: SlotArray,
    pub pending_control: Option<PendingControl>,
}

impl Frame {
    #[must_use]
    pub fn new(frame_id: FrameId, slot_count: usize) -> Self {
        Self {
            frame_id,
            slots: SlotArray::with_capacity(slot_count),
            pending_control: None,
        }
    }

    pub fn set_pending_control(&mut self, pending: PendingControl) {
        self.pending_control = Some(pending);
    }
}

fn slot_index(slot_id: SlotId, len: usize) -> RuntimeResult<usize> {
    if !slot_id.is_valid() {
        return Err(RuntimeFailure::structural(
            VmStructuralErrorCode::InvalidSlotError,
            "invalid slot id sentinel",
        ));
    }
    let index = slot_id.raw() as usize;
    if index >= len {
        return Err(RuntimeFailure::structural(
            VmStructuralErrorCode::InvalidSlotError,
            "slot index out of frame layout",
        ));
    }
    Ok(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uninitialized_slot_read_is_rejected() {
        let frame = Frame::new(FrameId::new(0), 2);
        let err = frame.slots.read(SlotId::new(0)).unwrap_err();
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::UninitializedBindingError)
        );
    }

    #[test]
    fn invalid_slot_id_is_rejected() {
        let frame = Frame::new(FrameId::new(0), 2);
        let err = frame.slots.read(SlotId::INVALID).unwrap_err();
        assert!(matches!(
            err,
            RuntimeFailure::Structural(ref e) if e.code == VmStructuralErrorCode::InvalidSlotError
        ));
    }

    #[test]
    fn slot_write_then_read_round_trips() {
        let mut frame = Frame::new(FrameId::new(0), 1);
        frame
            .slots
            .write(SlotId::new(0), Value::Int(5))
            .expect("write");
        let value = frame.slots.read(SlotId::new(0)).expect("read");
        assert_eq!(value, Value::Int(5));
    }
}