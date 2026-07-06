//! Binding cell storage for cell slots.
//!
//! Spec: `PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md` §2.5,
//! `PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md` §7,
//! `PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md` §6.2

use std::collections::BTreeMap;

use vm_core::id::{BindingId, CellId, TypeId};
use vm_core::runtime_plan::schema::Mutability;
use vm_core::value::Value;

/// Reference to a binding cell held in a `SlotState::Cell` slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BindingCellRef {
    pub cell_id: CellId,
}

/// Owner category for GC root tracing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellOwner {
    LocalCapture,
    ModuleBinding,
    ExportBinding,
    ClosureCapture,
    RuntimeInternal,
}

/// Initialization state of a binding cell.
#[derive(Debug, Clone, PartialEq)]
pub enum BindingState {
    Uninitialized,
    Initialized(Value),
}

/// Runtime binding cell (mutable or immutable).
#[derive(Debug, Clone, PartialEq)]
pub struct BindingCell {
    pub binding_id: BindingId,
    pub state: BindingState,
    pub mutability: Mutability,
    pub type_contract: Option<TypeId>,
    pub owner: CellOwner,
}

impl BindingCell {
    #[must_use]
    pub fn new(binding_id: BindingId, mutability: Mutability, owner: CellOwner) -> Self {
        Self {
            binding_id,
            state: BindingState::Uninitialized,
            mutability,
            type_contract: None,
            owner,
        }
    }

    #[must_use]
    pub fn with_value(
        binding_id: BindingId,
        mutability: Mutability,
        owner: CellOwner,
        value: Value,
    ) -> Self {
        Self {
            binding_id,
            state: BindingState::Initialized(value),
            mutability,
            type_contract: None,
            owner,
        }
    }

    pub fn with_type_contract(mut self, type_contract: TypeId) -> Self {
        self.type_contract = Some(type_contract);
        self
    }
}

/// Per-frame (or shared) cell table keyed by `CellId`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BindingCellStore {
    cells: BTreeMap<u32, BindingCell>,
    next_id: u32,
}

impl BindingCellStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allocate(&mut self, cell: BindingCell) -> CellId {
        let id = CellId::new(self.next_id);
        self.next_id = self.next_id.saturating_add(1);
        self.cells.insert(id.raw(), cell);
        id
    }

    pub fn get(&self, cell_id: CellId) -> Option<&BindingCell> {
        self.cells.get(&cell_id.raw())
    }

    pub fn get_mut(&mut self, cell_id: CellId) -> Option<&mut BindingCell> {
        self.cells.get_mut(&cell_id.raw())
    }
}