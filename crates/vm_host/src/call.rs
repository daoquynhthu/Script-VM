//! Host call protocol execution shell.
//!
//! Spec: `PHASE-3-HOST-BOUNDARY-CONTRACT.md` §6–§7

use vm_core::error::registry::RuntimeErrorCode;
use vm_core::error::language::ErrorStore;
use vm_core::value::Value;

use vm_runtime::control::VmControl;
use vm_runtime::module::resolver::CapabilitySet;
use vm_runtime::runtime_error::RuntimeFailure;

use crate::error::{normalize_host_call_result, NormalizedHostError};
use crate::host_function::{HostCallable, HostFunctionWrapper};
use crate::host_root::{HostBoundaryId, HostRootRegistry};

/// Context for a single host call invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct HostCallContext {
    pub capabilities: CapabilitySet,
    pub boundary_id: HostBoundaryId,
    pub root_registry: HostRootRegistry,
    pub call_frame_registered: bool,
}

impl HostCallContext {
    #[must_use]
    pub fn new(boundary_id: HostBoundaryId) -> Self {
        Self {
            capabilities: CapabilitySet::new(),
            boundary_id,
            root_registry: HostRootRegistry::new(),
            call_frame_registered: false,
        }
    }
}

/// Execute host call protocol steps 1–9 (bootstrap shell).
pub fn execute_host_call<C: HostCallable>(
    wrapper: &HostFunctionWrapper,
    callable: &C,
    args: &[Value],
    ctx: &mut HostCallContext,
    store: &mut ErrorStore,
) -> Result<VmControl, NormalizedHostError> {
    if let Some(cap) = wrapper.capability {
        if !ctx.capabilities.has(cap) {
            return Err(NormalizedHostError::Raise(host_capability_error(store)));
        }
    }

    if wrapper.descriptor.requires_roots_visible {
        ctx.call_frame_registered = true;
    }

    let raw = callable.invoke(args);
    let value = normalize_host_call_result(raw, store)?;
    ctx.root_registry.unregister_call_scoped();
    Ok(VmControl::Normal(Some(value)))
}

fn host_capability_error(store: &mut ErrorStore) -> vm_core::id::ErrorHandle {
    store.allocate(vm_core::error::language::ErrorObj::new(
        RuntimeErrorCode::CapabilityError,
        "host call missing required capability",
    ))
}

/// Map normalized host errors to runtime failures for boundary callers.
#[must_use]
pub fn runtime_failure_from_host_error(err: NormalizedHostError) -> RuntimeFailure {
    match err {
        NormalizedHostError::Raise(_) => {
            RuntimeFailure::language(RuntimeErrorCode::CapabilityError)
        }
        NormalizedHostError::Structural(vm_err) => RuntimeFailure::Structural(vm_err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host_function::{
        ArityShape, HostFunctionDescriptor, HostFunctionId, HostCallResult, HostParameterPolicy,
        HostResultPolicy,
    };

    struct CapHost;

    impl HostCallable for CapHost {
        fn invoke(&self, _: &[Value]) -> HostCallResult {
            HostCallResult::Return(Value::Int(1))
        }
    }

    fn wrapper_with_cap(cap: vm_core::id::CapabilityId) -> HostFunctionWrapper {
        HostFunctionWrapper {
            host_function_id: HostFunctionId(0),
            descriptor: HostFunctionDescriptor {
                arity: ArityShape { min: 0, max: None },
                parameter_policy: HostParameterPolicy::VmValues,
                result_policy: HostResultPolicy::VmValue,
                may_allocate: false,
                may_raise: false,
                may_block: false,
                may_reenter_vm: false,
                requires_roots_visible: false,
            },
            capability: Some(cap),
            effect: None,
            source_span: None,
        }
    }

    #[test]
    fn host_call_without_capability_rejected() {
        let wrapper = wrapper_with_cap(vm_core::id::CapabilityId::new(5));
        let mut ctx = HostCallContext::new(HostBoundaryId(0));
        let mut store = ErrorStore::new();
        let err = execute_host_call(&wrapper, &CapHost, &[], &mut ctx, &mut store)
            .expect_err("no cap");
        assert!(matches!(err, NormalizedHostError::Raise(_)));
    }

    #[test]
    fn host_call_with_capability_succeeds() {
        let cap = vm_core::id::CapabilityId::new(5);
        let wrapper = wrapper_with_cap(cap);
        let mut ctx = HostCallContext::new(HostBoundaryId(0));
        ctx.capabilities.grant(cap);
        let mut store = ErrorStore::new();
        let control = execute_host_call(&wrapper, &CapHost, &[], &mut ctx, &mut store)
            .expect("ok");
        assert_eq!(control, VmControl::Normal(Some(Value::Int(1))));
    }
}