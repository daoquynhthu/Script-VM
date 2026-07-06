//! Module instance storage.
//!
//! Spec: `PHASE-3-MODULE-RUNTIME-CONTRACT.md` §1

use vm_core::id::{EirFunctionId, ErrorHandle, InterfaceId, ModuleId};
use vm_core::runtime_plan::schema::ModulePlan;
use vm_diag::source_span::SourceSpanId;

use crate::frame::SlotArray;
use crate::heap::ObjRef;
use crate::runtime_error::RuntimeResult;

use super::export::ExportTable;
use super::state::ModuleState;

/// Provider interface descriptor for import compatibility checks.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ModuleInterfaceDescriptor {
    pub interface_id: Option<InterfaceId>,
}

/// Live module runtime object.
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleInstance {
    pub module_id: ModuleId,
    pub state: ModuleState,
    pub module_object: ObjRef<()>,
    pub module_slots: SlotArray,
    pub export_table: ExportTable,
    pub interface_descriptor: ModuleInterfaceDescriptor,
    pub initialization_error: Option<ErrorHandle>,
    pub initialization_function: EirFunctionId,
    pub source_span: Option<SourceSpanId>,
}

impl ModuleInstance {
    pub fn from_plan(
        plan: &ModulePlan,
        slot_count: usize,
        module_object: ObjRef<()>,
    ) -> RuntimeResult<Self> {
        Ok(Self {
            module_id: plan.module_id,
            state: ModuleState::Unloaded,
            module_object,
            module_slots: SlotArray::with_capacity(slot_count),
            export_table: ExportTable::from_export_plan(&plan.export_plan)?,
            interface_descriptor: ModuleInterfaceDescriptor::default(),
            initialization_error: None,
            initialization_function: plan.initialization_function,
            source_span: plan.source_span,
        })
    }
}