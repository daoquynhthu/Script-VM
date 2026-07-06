//! Module runtime orchestration shell.
//!
//! Spec: `PHASE-3-MODULE-RUNTIME-CONTRACT.md` §4, §8, §14

use vm_core::id::{CapabilityId, ModuleId, SlotId};
use vm_core::runtime_plan::schema::ModulePlan;

use crate::control::VmControl;
use crate::heap::ObjRef;
use crate::runtime_error::RuntimeResult;

use super::import::{bind_named_import, bind_whole_module_import};
use super::instance::ModuleInstance;
use super::registry::ModuleRegistry;
use super::resolver::{resolve_with_capability, CapabilitySet, HostModuleResolver, ModuleResolverRequest};
use super::state::ModuleState;
use super::validate::reject_top_level_control;

/// Canonical helper ids for module operations (registry §3 table order).
pub const HELPER_RESOLVE_MODULE_ID: vm_core::id::RuntimeHelperId =
    vm_core::id::RuntimeHelperId::new(34);
pub const HELPER_INITIALIZE_MODULE_ID: vm_core::id::RuntimeHelperId =
    vm_core::id::RuntimeHelperId::new(35);
pub const HELPER_IMPORT_NAMED_ID: vm_core::id::RuntimeHelperId =
    vm_core::id::RuntimeHelperId::new(36);
pub const HELPER_IMPORT_MODULE_ID: vm_core::id::RuntimeHelperId =
    vm_core::id::RuntimeHelperId::new(37);
pub const HELPER_SEAL_EXPORTS_ID: vm_core::id::RuntimeHelperId =
    vm_core::id::RuntimeHelperId::new(38);

/// Coordinates module registry, resolver capability boundary, and init lifecycle.
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleRuntime {
    pub registry: ModuleRegistry,
    pub capabilities: CapabilitySet,
    pub resolver_capability: CapabilityId,
}

impl ModuleRuntime {
    #[must_use]
    pub fn new(resolver_capability: CapabilityId) -> Self {
        Self {
            registry: ModuleRegistry::new(),
            capabilities: CapabilitySet::new(),
            resolver_capability,
        }
    }

    pub fn register_module(
        &mut self,
        plan: &ModulePlan,
        slot_count: usize,
        module_object: ObjRef<()>,
    ) -> RuntimeResult<()> {
        let instance = ModuleInstance::from_plan(plan, slot_count, module_object)?;
        self.registry.insert(instance)
    }

    pub fn resolve_module<R: HostModuleResolver>(
        &self,
        resolver: &R,
        request: &ModuleResolverRequest,
    ) -> RuntimeResult<ModuleId> {
        resolve_with_capability(
            resolver,
            &self.capabilities,
            self.resolver_capability,
            request,
        )
    }

    pub fn begin_loading(&mut self, module_id: ModuleId) -> RuntimeResult<()> {
        self.registry
            .transition(module_id, ModuleState::Loading, false)
    }

    pub fn begin_initializing(&mut self, module_id: ModuleId) -> RuntimeResult<()> {
        self.registry
            .transition(module_id, ModuleState::Initializing, false)
    }

    pub fn complete_initialization(&mut self, module_id: ModuleId) -> RuntimeResult<()> {
        self.registry
            .transition(module_id, ModuleState::Initialized, false)
    }

    pub fn seal_exports(&mut self, module_id: ModuleId) -> RuntimeResult<()> {
        let instance = self.registry.get_mut(module_id).ok_or_else(|| {
            crate::runtime_error::RuntimeFailure::structural(
                vm_core::error::registry::VmStructuralErrorCode::InvalidRuntimePlanError,
                format!("unknown module: {}", module_id.raw()),
            )
        })?;
        instance.export_table.seal()
    }

    pub fn finish_module_init(
        &mut self,
        module_id: ModuleId,
        control: &VmControl,
    ) -> RuntimeResult<()> {
        reject_top_level_control(control)?;
        match control {
            VmControl::Raise(handle) => {
                self.registry.fail_initialization(module_id, *handle)
            }
            _ => {
                self.seal_exports(module_id)?;
                self.complete_initialization(module_id)
            }
        }
    }

    pub fn import_named(
        &mut self,
        importer_id: ModuleId,
        provider_id: ModuleId,
        export_name: &str,
        local_slot: SlotId,
    ) -> RuntimeResult<()> {
        let provider = self
            .registry
            .get(provider_id)
            .ok_or_else(|| missing_module(provider_id))?
            .clone();
        let importer = self.registry.get_mut(importer_id).ok_or_else(|| missing_module(importer_id))?;
        bind_named_import(importer, &provider, export_name, local_slot)
    }

    pub fn import_whole_module(
        &mut self,
        importer_id: ModuleId,
        provider_id: ModuleId,
        local_slot: SlotId,
    ) -> RuntimeResult<()> {
        let provider = self
            .registry
            .get(provider_id)
            .ok_or_else(|| missing_module(provider_id))?
            .clone();
        let importer = self.registry.get_mut(importer_id).ok_or_else(|| missing_module(importer_id))?;
        bind_whole_module_import(importer, &provider, local_slot)
    }

    pub fn explicit_retry(&mut self, module_id: ModuleId) -> RuntimeResult<()> {
        self.registry.explicit_retry(module_id)
    }

    pub fn attempt_automatic_retry(&mut self, module_id: ModuleId) -> RuntimeResult<()> {
        self.registry
            .transition(module_id, ModuleState::Loading, false)
    }
}

fn missing_module(module_id: ModuleId) -> crate::runtime_error::RuntimeFailure {
    crate::runtime_error::RuntimeFailure::structural(
        vm_core::error::registry::VmStructuralErrorCode::InvalidRuntimePlanError,
        format!("unknown module: {}", module_id.raw()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::heap::ObjRef;
    use vm_core::id::{BindingId, EirFunctionId, ObjectId};
    use vm_core::runtime_plan::schema::{ExportPlan, ExportPlanEntry, ModulePlan};
    use vm_diag::source_span::SourceSpanId;

    fn sample_plan(id: u32, export_name: &str) -> ModulePlan {
        ModulePlan {
            module_id: ModuleId::new(id),
            module_slot_layout: vm_core::id::SlotLayoutId::new(0),
            initialization_function: EirFunctionId::new(0),
            import_plan: Default::default(),
            export_plan: ExportPlan {
                exports: vec![ExportPlanEntry {
                    exported_name: export_name.to_string(),
                    binding_id: BindingId::new(0),
                    slot_id: SlotId::new(0),
                    interface_type: None,
                    source_span: SourceSpanId::new(0),
                }],
                seal_after_init: true,
            },
            module_state_slot: SlotId::new(0),
            module_object_slot: SlotId::new(1),
            source_order: vec![],
            source_span: None,
        }
    }

    #[test]
    fn finish_init_seals_and_marks_initialized() {
        let mut rt = ModuleRuntime::new(CapabilityId::new(0));
        let plan = sample_plan(1, "e");
        rt.register_module(&plan, 2, ObjRef::new(ObjectId::new(1), 0))
            .expect("register");
        rt.begin_loading(ModuleId::new(1)).expect("load");
        rt.begin_initializing(ModuleId::new(1)).expect("init");
        rt.finish_module_init(ModuleId::new(1), &VmControl::Normal(None))
            .expect("finish");
        let instance = rt.registry.get(ModuleId::new(1)).expect("get");
        assert_eq!(instance.state, ModuleState::Initialized);
        assert!(instance.export_table.is_sealed());
    }

    #[test]
    fn automatic_retry_after_failed_rejected() {
        let mut rt = ModuleRuntime::new(CapabilityId::new(0));
        let plan = sample_plan(1, "e");
        rt.register_module(&plan, 2, ObjRef::new(ObjectId::new(1), 0))
            .expect("register");
        rt.begin_loading(ModuleId::new(1)).expect("load");
        rt.begin_initializing(ModuleId::new(1)).expect("init");
        rt.registry
            .fail_initialization(ModuleId::new(1), vm_core::id::ErrorHandle::new(1))
            .expect("fail");
        assert!(rt.attempt_automatic_retry(ModuleId::new(1)).is_err());
        assert!(rt.explicit_retry(ModuleId::new(1)).is_ok());
    }
}