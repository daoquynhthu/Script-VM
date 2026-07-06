//! Import resolution and circular-import checks.
//!
//! Spec: `PHASE-3-MODULE-RUNTIME-CONTRACT.md` §5–§10

use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::SlotId;
use vm_core::value::Value;

use crate::runtime_error::{RuntimeFailure, RuntimeResult};

use super::instance::ModuleInstance;
use super::state::ModuleState;

/// Import failure when provider module is in `Failed` state.
pub fn reject_failed_provider(provider: &ModuleInstance) -> RuntimeResult<()> {
    if provider.state.blocks_ordinary_import() {
        return Err(RuntimeFailure::language(RuntimeErrorCode::ImportError));
    }
    Ok(())
}

/// Read a named export value, enforcing circular-import access rules.
pub fn read_named_export(
    provider: &ModuleInstance,
    export_name: &str,
) -> RuntimeResult<Value> {
    reject_failed_provider(provider)?;

    let entry = provider.export_table.get(export_name).ok_or_else(|| {
        RuntimeFailure::language(RuntimeErrorCode::ImportError)
    })?;

    if provider.state == ModuleState::Initializing && !entry.initialized {
        return Err(RuntimeFailure::language(RuntimeErrorCode::ImportCycleError));
    }

    if !entry.initialized {
        return Err(RuntimeFailure::language(RuntimeErrorCode::ImportError));
    }

    provider.module_slots.read(entry.slot_id)
}

/// Bind a named import into the importer's local slot.
pub fn bind_named_import(
    importer: &mut ModuleInstance,
    provider: &ModuleInstance,
    export_name: &str,
    local_slot: SlotId,
) -> RuntimeResult<()> {
    let value = read_named_export(provider, export_name)?;
    importer.module_slots.write(local_slot, value)
}

/// Bind a whole-module import (module object reference).
pub fn bind_whole_module_import(
    importer: &mut ModuleInstance,
    provider: &ModuleInstance,
    local_slot: SlotId,
) -> RuntimeResult<()> {
    reject_failed_provider(provider)?;
    if !provider.state.allows_export_reads() {
        return Err(RuntimeFailure::language(RuntimeErrorCode::ImportError));
    }
    importer
        .module_slots
        .write(local_slot, Value::ObjectRef(provider.module_object.id()))
}

/// Check whether accessing an export during initialization would violate cycle rules.
#[must_use]
pub fn is_uninitialized_circular_access(
    provider_state: ModuleState,
    export_initialized: bool,
) -> bool {
    provider_state == ModuleState::Initializing && !export_initialized
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::heap::ObjRef;
    use crate::module::export::{ExportEntry, ExportTable};
    use vm_core::id::{BindingId, EirFunctionId, ModuleId, ObjectId};
    use vm_core::runtime_plan::schema::{ExportPlan, ModulePlan};
    use vm_diag::source_span::SourceSpanId;

    fn provider_with_export(name: &str, initialized: bool, state: ModuleState) -> ModuleInstance {
        let mut export_table = ExportTable::new();
        let entry = ExportEntry {
            name: name.to_string(),
            binding_id: BindingId::new(0),
            slot_id: SlotId::new(0),
            initialized,
            type_id: None,
            source_span: SourceSpanId::new(0),
        };
        export_table.insert_entry(entry).expect("insert");
        let mut instance = ModuleInstance {
            module_id: ModuleId::new(1),
            state,
            module_object: ObjRef::new(ObjectId::new(1), 0),
            module_slots: crate::frame::SlotArray::with_capacity(1),
            export_table,
            interface_descriptor: Default::default(),
            initialization_error: None,
            initialization_function: EirFunctionId::new(0),
            source_span: None,
        };
        if initialized {
            instance
                .module_slots
                .write(SlotId::new(0), Value::Int(42))
                .expect("write");
        }
        instance
    }

    #[test]
    fn uninitialized_circular_export_raises_import_cycle_error() {
        let provider = provider_with_export("foo", false, ModuleState::Initializing);
        let err = read_named_export(&provider, "foo").expect_err("cycle");
        assert!(matches!(
            err,
            RuntimeFailure::Language(RuntimeErrorCode::ImportCycleError)
        ));
    }

    #[test]
    fn initialized_export_during_initializing_is_allowed() {
        let provider = provider_with_export("foo", true, ModuleState::Initializing);
        let value = read_named_export(&provider, "foo").expect("ok");
        assert_eq!(value, Value::Int(42));
    }

    #[test]
    fn failed_module_import_rejected() {
        let mut provider = provider_with_export("foo", true, ModuleState::Failed);
        provider.initialization_error = Some(vm_core::id::ErrorHandle::new(0));
        let err = read_named_export(&provider, "foo").expect_err("failed");
        assert!(matches!(
            err,
            RuntimeFailure::Language(RuntimeErrorCode::ImportError)
        ));
    }

    #[test]
    fn missing_export_rejected() {
        let provider = provider_with_export("foo", true, ModuleState::Initialized);
        let err = read_named_export(&provider, "missing").expect_err("missing");
        assert!(matches!(
            err,
            RuntimeFailure::Language(RuntimeErrorCode::ImportError)
        ));
    }

    #[test]
    fn bind_named_import_writes_local_slot() {
        let provider = provider_with_export("x", true, ModuleState::Initialized);
        let plan = ModulePlan {
            module_id: ModuleId::new(2),
            module_slot_layout: vm_core::id::SlotLayoutId::new(0),
            initialization_function: EirFunctionId::new(0),
            import_plan: Default::default(),
            export_plan: ExportPlan {
                exports: vec![],
                seal_after_init: true,
            },
            module_state_slot: SlotId::new(0),
            module_object_slot: SlotId::new(1),
            source_order: vec![],
            source_span: None,
        };
        let mut importer =
            ModuleInstance::from_plan(&plan, 2, ObjRef::new(ObjectId::new(2), 0)).expect("plan");
        bind_named_import(&mut importer, &provider, "x", SlotId::new(0)).expect("bind");
        assert_eq!(
            importer.module_slots.read(SlotId::new(0)).expect("read"),
            Value::Int(42)
        );
    }
}