//! Module instance registry and state transition application.
//!
//! Spec: `PHASE-3-MODULE-RUNTIME-CONTRACT.md` §1–§3, §10

use std::collections::BTreeMap;

use vm_core::error::registry::VmStructuralErrorCode;
use vm_core::id::{ErrorHandle, ModuleId};

use crate::runtime_error::{RuntimeFailure, RuntimeResult};

use super::instance::ModuleInstance;
use super::state::{validate_transition_with_retry, ModuleState};

/// Storage for live module instances keyed by `ModuleId`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModuleRegistry {
    instances: BTreeMap<u32, ModuleInstance>,
}

impl ModuleRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, instance: ModuleInstance) -> RuntimeResult<()> {
        let key = instance.module_id.raw();
        if self.instances.contains_key(&key) {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidRuntimePlanError,
                format!("duplicate module instance: {}", key),
            ));
        }
        self.instances.insert(key, instance);
        Ok(())
    }

    #[must_use]
    pub fn get(&self, module_id: ModuleId) -> Option<&ModuleInstance> {
        self.instances.get(&module_id.raw())
    }

    pub fn get_mut(&mut self, module_id: ModuleId) -> Option<&mut ModuleInstance> {
        self.instances.get_mut(&module_id.raw())
    }

    pub fn transition(
        &mut self,
        module_id: ModuleId,
        to: ModuleState,
        explicit_host_retry: bool,
    ) -> RuntimeResult<()> {
        let instance = self.get_mut(module_id).ok_or_else(|| {
            RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidRuntimePlanError,
                format!("unknown module: {}", module_id.raw()),
            )
        })?;
        validate_transition_with_retry(instance.state, to, explicit_host_retry)?;
        instance.state = to;
        Ok(())
    }

    pub fn fail_initialization(
        &mut self,
        module_id: ModuleId,
        error: ErrorHandle,
    ) -> RuntimeResult<()> {
        self.transition(module_id, ModuleState::Failed, false)?;
        let instance = self.get_mut(module_id).expect("just inserted");
        instance.initialization_error = Some(error);
        Ok(())
    }

    pub fn explicit_retry(&mut self, module_id: ModuleId) -> RuntimeResult<()> {
        self.transition(module_id, ModuleState::Loading, true)
    }
}