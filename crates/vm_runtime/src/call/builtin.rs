//! Builtin call descriptor path.
//!
//! Spec: `PHASE-3-CALL-EXECUTION-PROTOCOL.md` §12

use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::CapabilityId;

use crate::call::bind::ArgumentBinding;
use crate::module::resolver::CapabilitySet;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};

/// Descriptor metadata for builtin invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltinCallDescriptor {
    pub builtin_id: u32,
    pub arity: u32,
    pub required_capabilities: Vec<CapabilityId>,
    pub may_raise: bool,
    pub may_allocate: bool,
}

/// Validate builtin call against descriptor and capability environment.
pub fn validate_builtin_call(
    descriptor: &BuiltinCallDescriptor,
    binding: &ArgumentBinding,
    capabilities: &CapabilitySet,
) -> RuntimeResult<()> {
    if binding.bound.len() as u32 != descriptor.arity {
        return Err(RuntimeFailure::language(RuntimeErrorCode::ArityError));
    }
    for cap in &descriptor.required_capabilities {
        if !capabilities.has(*cap) {
            return Err(RuntimeFailure::language(RuntimeErrorCode::CapabilityError));
        }
    }
    Ok(())
}

/// Canonical helper id for `helper_call_builtin` (registry §3 table order).
pub const HELPER_CALL_BUILTIN_ID: vm_core::id::RuntimeHelperId =
    vm_core::id::RuntimeHelperId::new(26);

/// Canonical helper id for `helper_generic_call`.
pub const HELPER_GENERIC_CALL_ID: vm_core::id::RuntimeHelperId =
    vm_core::id::RuntimeHelperId::new(25);

/// Canonical helper id for `helper_check_arity`.
pub const HELPER_CHECK_ARITY_ID: vm_core::id::RuntimeHelperId =
    vm_core::id::RuntimeHelperId::new(27);

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::id::SlotId;
    use vm_core::value::Value;

    #[test]
    fn effectful_builtin_without_capability_rejected() {
        let descriptor = BuiltinCallDescriptor {
            builtin_id: 1,
            arity: 1,
            required_capabilities: vec![CapabilityId::new(9)],
            may_raise: true,
            may_allocate: false,
        };
        let binding = ArgumentBinding {
            bound: vec![(SlotId::new(0), Value::Int(1))],
            pending_default_indices: vec![],
        };
        let caps = CapabilitySet::new();
        let err = validate_builtin_call(&descriptor, &binding, &caps).expect_err("cap");
        assert!(matches!(
            err,
            RuntimeFailure::Language(RuntimeErrorCode::CapabilityError)
        ));
    }
}