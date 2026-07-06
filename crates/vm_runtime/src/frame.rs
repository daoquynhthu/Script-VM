//! Frame and slot storage.
//!
//! Spec: `PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md` §2, `PHASE-3-VM-RUNTIME-ROUND1.md`

use vm_core::error::registry::{RuntimeErrorCode, VmStructuralErrorCode};
use vm_core::id::{BindingId, CellId, FrameId, SlotId};
use vm_core::runtime_plan::schema::Mutability;
use vm_core::value::Value;

use crate::binding_cell::{
    BindingCell, BindingCellRef, BindingCellStore, BindingState, CellOwner,
};
use crate::call::contract::TypeContractChecker;
use crate::control::PendingControl;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};
use crate::runtime_value::RuntimeValue;
use crate::write_barrier::{value_may_mutate_heap, WriteBarrierHook};

/// Policy for whether uninitialized slots may be read (§2.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SlotReadPolicy {
    #[default]
    RequireInitialized,
    /// Operation explicitly permits reading uninitialized value slots as `Value::None`.
    PermitUninitialized,
}

/// Slot initialization state.
#[derive(Debug, Clone, PartialEq)]
pub enum SlotState {
    Uninitialized,
    Value(Value),
    Cell(BindingCellRef),
    RuntimeInternal(RuntimeValue),
}

/// Fixed-size slot array for an interpreter frame.
#[derive(Debug, Clone, PartialEq)]
pub struct SlotArray {
    slots: Vec<SlotState>,
    cells: BindingCellStore,
}

impl SlotArray {
    #[must_use]
    pub fn with_capacity(count: usize) -> Self {
        Self {
            slots: vec![SlotState::Uninitialized; count],
            cells: BindingCellStore::new(),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    /// Ordinary binding write: stores a value in a value slot.
    ///
    /// Cell and runtime-internal slots reject user-visible writes (§2.4).
    pub fn write(&mut self, slot_id: SlotId, value: Value) -> RuntimeResult<()> {
        let index = slot_index(slot_id, self.slots.len())?;
        match &self.slots[index] {
            SlotState::Uninitialized | SlotState::Value(_) => {
                self.slots[index] = SlotState::Value(value);
                Ok(())
            }
            SlotState::Cell(_) => Err(slot_kind_mismatch("cannot write value directly to cell slot")),
            SlotState::RuntimeInternal(_) => Err(user_visible_runtime_internal_access()),
        }
    }

    /// Ordinary binding read with default `RequireInitialized` policy (§2.3, §2.5).
    pub fn read(&self, slot_id: SlotId) -> RuntimeResult<Value> {
        self.read_with_policy(slot_id, SlotReadPolicy::RequireInitialized)
    }

    /// Binding read with explicit initialization policy (§2.3).
    pub fn read_with_policy(
        &self,
        slot_id: SlotId,
        policy: SlotReadPolicy,
    ) -> RuntimeResult<Value> {
        let index = slot_index(slot_id, self.slots.len())?;
        match &self.slots[index] {
            SlotState::Uninitialized => match policy {
                SlotReadPolicy::RequireInitialized => Err(RuntimeFailure::language(
                    RuntimeErrorCode::UninitializedBindingError,
                )),
                SlotReadPolicy::PermitUninitialized => Ok(Value::None),
            },
            SlotState::Value(value) => Ok(value.clone()),
            SlotState::Cell(cell_ref) => {
                self.read_cell_value(cell_ref.cell_id, policy)
            }
            SlotState::RuntimeInternal(_) => Err(user_visible_runtime_internal_access()),
        }
    }

    pub fn state(&self, slot_id: SlotId) -> RuntimeResult<&SlotState> {
        let index = slot_index(slot_id, self.slots.len())?;
        Ok(&self.slots[index])
    }

    /// Bind a cell into `slot_id` and return the allocated `CellId`.
    pub fn bind_cell(
        &mut self,
        slot_id: SlotId,
        binding_id: BindingId,
        mutability: Mutability,
        owner: CellOwner,
    ) -> RuntimeResult<CellId> {
        let index = slot_index(slot_id, self.slots.len())?;
        let cell_id = self
            .cells
            .allocate(BindingCell::new(binding_id, mutability, owner));
        self.slots[index] = SlotState::Cell(BindingCellRef { cell_id });
        Ok(cell_id)
    }

    /// Install a fully described cell at `slot_id` (bootstrap/test helper).
    pub fn install_cell(
        &mut self,
        slot_id: SlotId,
        cell: BindingCell,
    ) -> RuntimeResult<CellId> {
        let index = slot_index(slot_id, self.slots.len())?;
        let cell_id = self.cells.allocate(cell);
        self.slots[index] = SlotState::Cell(BindingCellRef { cell_id });
        Ok(cell_id)
    }

    /// Install a cell slot with an initialized value (test/bootstrap helper).
    pub fn bind_cell_with_value(
        &mut self,
        slot_id: SlotId,
        binding_id: BindingId,
        mutability: Mutability,
        owner: CellOwner,
        value: Value,
    ) -> RuntimeResult<CellId> {
        let index = slot_index(slot_id, self.slots.len())?;
        let cell_id = self.cells.allocate(BindingCell::with_value(
            binding_id,
            mutability,
            owner,
            value,
        ));
        self.slots[index] = SlotState::Cell(BindingCellRef { cell_id });
        Ok(cell_id)
    }

    /// `StoreCell` / cell write path (§5.3): mutability, type contract, write barrier.
    pub fn write_cell(
        &mut self,
        cell_slot: SlotId,
        value: Value,
        checker: &impl TypeContractChecker,
        barrier: &mut impl WriteBarrierHook,
    ) -> RuntimeResult<()> {
        let index = slot_index(cell_slot, self.slots.len())?;
        let cell_id = match &self.slots[index] {
            SlotState::Cell(cell_ref) => cell_ref.cell_id,
            SlotState::Uninitialized | SlotState::Value(_) => {
                return Err(slot_kind_mismatch("store cell requires cell slot"));
            }
            SlotState::RuntimeInternal(_) => return Err(user_visible_runtime_internal_access()),
        };

        let cell = self.cells.get_mut(cell_id).ok_or_else(|| {
            RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidSlotError,
                "cell id not found in cell store",
            )
        })?;

        if cell.mutability == Mutability::Immutable {
            return Err(assignment_to_immutable_cell());
        }

        if let Some(type_id) = cell.type_contract {
            if !checker.value_matches_type(&value, type_id) {
                return Err(RuntimeFailure::language(RuntimeErrorCode::TypeContractError));
            }
        }

        let previous = match &cell.state {
            BindingState::Uninitialized => None,
            BindingState::Initialized(old) => Some(old),
        };

        if value_may_mutate_heap(previous, &value) {
            barrier.on_heap_mutation(previous, &value);
        }

        cell.state = BindingState::Initialized(value);
        Ok(())
    }

    /// Explicit cell value read (`LoadCell` semantics).
    pub fn read_cell(&self, cell_slot: SlotId) -> RuntimeResult<Value> {
        let index = slot_index(cell_slot, self.slots.len())?;
        let cell_id = match &self.slots[index] {
            SlotState::Cell(cell_ref) => cell_ref.cell_id,
            _ => return Err(slot_kind_mismatch("load cell requires cell slot")),
        };
        self.read_cell_value(cell_id, SlotReadPolicy::RequireInitialized)
    }

    /// Install a runtime-internal slot (hidden from ordinary binding access).
    pub fn install_runtime_internal(
        &mut self,
        slot_id: SlotId,
        value: RuntimeValue,
    ) -> RuntimeResult<()> {
        let index = slot_index(slot_id, self.slots.len())?;
        self.slots[index] = SlotState::RuntimeInternal(value);
        Ok(())
    }

    /// VM-internal read of a runtime-internal slot.
    pub fn read_runtime_internal(&self, slot_id: SlotId) -> RuntimeResult<RuntimeValue> {
        let index = slot_index(slot_id, self.slots.len())?;
        match &self.slots[index] {
            SlotState::RuntimeInternal(value) => Ok(value.clone()),
            _ => Err(slot_kind_mismatch("runtime-internal read requires runtime-internal slot")),
        }
    }

    /// VM-internal write of a runtime-internal slot.
    pub fn write_runtime_internal(
        &mut self,
        slot_id: SlotId,
        value: RuntimeValue,
    ) -> RuntimeResult<()> {
        let index = slot_index(slot_id, self.slots.len())?;
        match &self.slots[index] {
            SlotState::RuntimeInternal(_) => {
                self.slots[index] = SlotState::RuntimeInternal(value);
                Ok(())
            }
            _ => Err(slot_kind_mismatch(
                "runtime-internal write requires runtime-internal slot",
            )),
        }
    }

    fn read_cell_value(
        &self,
        cell_id: CellId,
        policy: SlotReadPolicy,
    ) -> RuntimeResult<Value> {
        let cell = self.cells.get(cell_id).ok_or_else(|| {
            RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidSlotError,
                "cell id not found in cell store",
            )
        })?;
        match &cell.state {
            BindingState::Uninitialized => match policy {
                SlotReadPolicy::RequireInitialized => Err(RuntimeFailure::language(
                    RuntimeErrorCode::UninitializedBindingError,
                )),
                // LoadCell still requires initialized cell even when value slots permit uninit.
                SlotReadPolicy::PermitUninitialized => Err(RuntimeFailure::language(
                    RuntimeErrorCode::UninitializedBindingError,
                )),
            },
            BindingState::Initialized(value) => Ok(value.clone()),
        }
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

fn slot_kind_mismatch(message: &'static str) -> RuntimeFailure {
    RuntimeFailure::structural(VmStructuralErrorCode::InvalidSlotError, message)
}

fn user_visible_runtime_internal_access() -> RuntimeFailure {
    RuntimeFailure::language(RuntimeErrorCode::InternalVMError)
}

/// Immutable cell / const assignment (§2.5, validation matrix: const assignment rejected).
fn assignment_to_immutable_cell() -> RuntimeFailure {
    RuntimeFailure::language(RuntimeErrorCode::ReadOnlyError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call::contract::StubTypeContractChecker;
    use crate::write_barrier::NoopWriteBarrierHook;
    use vm_core::id::{ObjectId, TypeId};

    struct RecordingBarrier {
        invocations: u32,
    }

    impl WriteBarrierHook for RecordingBarrier {
        fn on_heap_mutation(&mut self, _previous: Option<&Value>, _new_value: &Value) {
            self.invocations += 1;
        }
    }

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
    fn permit_uninitialized_read_returns_none_for_value_slot() {
        let frame = Frame::new(FrameId::new(0), 1);
        let value = frame
            .slots
            .read_with_policy(SlotId::new(0), SlotReadPolicy::PermitUninitialized)
            .expect("permitted read");
        assert_eq!(value, Value::None);
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

    #[test]
    fn cell_slot_ordinary_read_returns_current_cell_value() {
        let mut frame = Frame::new(FrameId::new(0), 1);
        frame
            .slots
            .bind_cell_with_value(
                SlotId::new(0),
                BindingId::new(1),
                Mutability::Mutable,
                CellOwner::LocalCapture,
                Value::Int(7),
            )
            .expect("bind");
        let value = frame.slots.read(SlotId::new(0)).expect("ordinary read");
        assert_eq!(value, Value::Int(7));
    }

    #[test]
    fn mutable_cell_write_updates_value_visible_on_read() {
        let mut frame = Frame::new(FrameId::new(0), 1);
        let mut barrier = NoopWriteBarrierHook;
        let checker = StubTypeContractChecker::new();
        frame
            .slots
            .bind_cell_with_value(
                SlotId::new(0),
                BindingId::new(1),
                Mutability::Mutable,
                CellOwner::LocalCapture,
                Value::Int(1),
            )
            .expect("bind");
        frame
            .slots
            .write_cell(SlotId::new(0), Value::Int(99), &checker, &mut barrier)
            .expect("write cell");
        assert_eq!(
            frame.slots.read(SlotId::new(0)).expect("read"),
            Value::Int(99)
        );
    }

    #[test]
    fn immutable_cell_write_is_rejected_with_read_only_error() {
        let mut frame = Frame::new(FrameId::new(0), 1);
        let mut barrier = NoopWriteBarrierHook;
        let checker = StubTypeContractChecker::new();
        frame
            .slots
            .bind_cell_with_value(
                SlotId::new(0),
                BindingId::new(1),
                Mutability::Immutable,
                CellOwner::LocalCapture,
                Value::Int(1),
            )
            .expect("bind");
        let err = frame
            .slots
            .write_cell(SlotId::new(0), Value::Int(2), &checker, &mut barrier)
            .unwrap_err();
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::ReadOnlyError)
        );
        assert_eq!(
            frame.slots.read(SlotId::new(0)).expect("unchanged"),
            Value::Int(1)
        );
    }

    #[test]
    fn cell_write_type_contract_mismatch_is_rejected() {
        let mut frame = Frame::new(FrameId::new(0), 1);
        let mut barrier = NoopWriteBarrierHook;
        let mut checker = StubTypeContractChecker::new();
        checker.declare_int_type(TypeId::new(1));

        let cell = BindingCell::with_value(
            BindingId::new(1),
            Mutability::Mutable,
            CellOwner::LocalCapture,
            Value::Int(1),
        )
        .with_type_contract(TypeId::new(1));
        frame
            .slots
            .install_cell(SlotId::new(0), cell)
            .expect("install");

        let err = frame
            .slots
            .write_cell(SlotId::new(0), Value::Bool(true), &checker, &mut barrier)
            .unwrap_err();
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::TypeContractError)
        );
    }

    #[test]
    fn cell_write_heap_ref_invokes_write_barrier_hook() {
        let mut frame = Frame::new(FrameId::new(0), 1);
        let mut barrier = RecordingBarrier { invocations: 0 };
        let checker = StubTypeContractChecker::new();
        frame
            .slots
            .bind_cell_with_value(
                SlotId::new(0),
                BindingId::new(1),
                Mutability::Mutable,
                CellOwner::LocalCapture,
                Value::Int(1),
            )
            .expect("bind");
        frame
            .slots
            .write_cell(
                SlotId::new(0),
                Value::ObjectRef(ObjectId::new(9)),
                &checker,
                &mut barrier,
            )
            .expect("write cell");
        assert_eq!(barrier.invocations, 1);
    }

    #[test]
    fn uninitialized_cell_read_raises_uninitialized_binding_error() {
        let mut frame = Frame::new(FrameId::new(0), 1);
        frame
            .slots
            .bind_cell(
                SlotId::new(0),
                BindingId::new(1),
                Mutability::Mutable,
                CellOwner::LocalCapture,
            )
            .expect("bind");
        let err = frame.slots.read(SlotId::new(0)).unwrap_err();
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::UninitializedBindingError)
        );
    }

    #[test]
    fn ordinary_write_to_cell_slot_is_rejected() {
        let mut frame = Frame::new(FrameId::new(0), 1);
        frame
            .slots
            .bind_cell(
                SlotId::new(0),
                BindingId::new(1),
                Mutability::Mutable,
                CellOwner::LocalCapture,
            )
            .expect("bind");
        let err = frame
            .slots
            .write(SlotId::new(0), Value::Int(1))
            .unwrap_err();
        assert!(matches!(
            err,
            RuntimeFailure::Structural(ref e) if e.code == VmStructuralErrorCode::InvalidSlotError
        ));
    }

    #[test]
    fn runtime_internal_rejects_ordinary_read_and_write() {
        let mut frame = Frame::new(FrameId::new(0), 1);
        frame
            .slots
            .install_runtime_internal(SlotId::new(0), RuntimeValue::Temp(Value::Int(3)))
            .expect("install");

        let read_err = frame.slots.read(SlotId::new(0)).unwrap_err();
        assert_eq!(
            read_err,
            RuntimeFailure::language(RuntimeErrorCode::InternalVMError)
        );

        let write_err = frame
            .slots
            .write(SlotId::new(0), Value::Int(4))
            .unwrap_err();
        assert_eq!(
            write_err,
            RuntimeFailure::language(RuntimeErrorCode::InternalVMError)
        );
    }

    #[test]
    fn runtime_internal_internal_api_round_trips() {
        let mut frame = Frame::new(FrameId::new(0), 1);
        frame
            .slots
            .install_runtime_internal(SlotId::new(0), RuntimeValue::Temp(Value::Int(8)))
            .expect("install");
        let current = frame
            .slots
            .read_runtime_internal(SlotId::new(0))
            .expect("internal read");
        assert_eq!(current, RuntimeValue::Temp(Value::Int(8)));

        frame
            .slots
            .write_runtime_internal(SlotId::new(0), RuntimeValue::Temp(Value::Int(9)))
            .expect("internal write");
        let updated = frame
            .slots
            .read_runtime_internal(SlotId::new(0))
            .expect("internal read");
        assert_eq!(updated, RuntimeValue::Temp(Value::Int(9)));
    }
}