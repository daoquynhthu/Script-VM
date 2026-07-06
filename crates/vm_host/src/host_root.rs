//! Host root registry for retained VM values.
//!
//! Spec: `PHASE-3-HOST-BOUNDARY-CONTRACT.md` §5

use std::collections::BTreeMap;

use vm_core::error::registry::VmStructuralErrorCode;
use vm_core::error::VmError;
use vm_core::id::CapabilityId;
use vm_core::value::Value;

/// Host boundary owner identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HostBoundaryId(pub u32);

/// Registered host root identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HostRootId(pub u32);

/// Root lifetime policy per frozen contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostRootLifetime {
    CallScoped,
    ResourceScoped,
    ExplicitHandle,
}

/// Single retained VM value visible to host boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct HostRootEntry {
    pub value: Value,
    pub owner: HostBoundaryId,
    pub lifetime: HostRootLifetime,
    pub capability: Option<CapabilityId>,
}

/// Registry of host-retained VM roots.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HostRootRegistry {
    roots: BTreeMap<u32, HostRootEntry>,
    next_id: u32,
    call_scoped: Vec<HostRootId>,
}

impl HostRootRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, entry: HostRootEntry) -> HostRootId {
        let id = HostRootId(self.next_id);
        self.next_id += 1;
        if entry.lifetime == HostRootLifetime::CallScoped {
            self.call_scoped.push(id);
        }
        self.roots.insert(id.0, entry);
        id
    }

    #[must_use]
    pub fn get(&self, id: HostRootId) -> Option<&HostRootEntry> {
        self.roots.get(&id.0)
    }

    /// Unregister call-scoped roots after host call exit (protocol step 8).
    pub fn unregister_call_scoped(&mut self) {
        for id in &self.call_scoped {
            self.roots.remove(&id.0);
        }
        self.call_scoped.clear();
    }

    /// Host MUST NOT retain VM values beyond a call without a registered root.
    pub fn validate_retention(
        value: &Value,
        retained_without_root: bool,
    ) -> Result<(), VmError> {
        if retained_without_root && value_may_need_root(value) {
            return Err(VmError::new(
                VmStructuralErrorCode::BackendViolationError,
                "host retained VM value without HostRootEntry",
            ));
        }
        Ok(())
    }
}

fn value_may_need_root(value: &Value) -> bool {
    matches!(value, Value::ObjectRef(_) | Value::Error(_))
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::id::ObjectId;

    #[test]
    fn retained_heap_value_without_root_rejected() {
        let value = Value::ObjectRef(ObjectId::new(1));
        let err = HostRootRegistry::validate_retention(&value, true).expect_err("root");
        assert_eq!(err.code, VmStructuralErrorCode::BackendViolationError);
    }

    #[test]
    fn call_scoped_roots_cleared_after_unregister() {
        let mut registry = HostRootRegistry::new();
        let id = registry.register(HostRootEntry {
            value: Value::Int(1),
            owner: HostBoundaryId(0),
            lifetime: HostRootLifetime::CallScoped,
            capability: None,
        });
        assert!(registry.get(id).is_some());
        registry.unregister_call_scoped();
        assert!(registry.get(id).is_none());
    }
}