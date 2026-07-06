//! Host function wrapper and descriptor.
//!
//! Spec: `PHASE-3-HOST-BOUNDARY-CONTRACT.md` §3

use vm_core::id::{CapabilityId, EffectId};
use vm_diag::source_span::SourceSpanId;

/// Host function identity at the VM-controlled boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HostFunctionId(pub u32);

/// Arity shape for host function descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArityShape {
    pub min: u32,
    pub max: Option<u32>,
}

/// Host parameter marshaling policy placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostParameterPolicy {
    VmValues,
}

/// Host result marshaling policy placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostResultPolicy {
    VmValue,
    VmControl,
}

/// Descriptor metadata for a host function wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFunctionDescriptor {
    pub arity: ArityShape,
    pub parameter_policy: HostParameterPolicy,
    pub result_policy: HostResultPolicy,
    pub may_allocate: bool,
    pub may_raise: bool,
    pub may_block: bool,
    pub may_reenter_vm: bool,
    pub requires_roots_visible: bool,
}

/// VM-controlled host function wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFunctionWrapper {
    pub host_function_id: HostFunctionId,
    pub descriptor: HostFunctionDescriptor,
    pub capability: Option<CapabilityId>,
    pub effect: Option<EffectId>,
    pub source_span: Option<SourceSpanId>,
}

/// Raw host invocation result before normalization.
#[derive(Debug, Clone, PartialEq)]
pub enum HostCallResult {
    Return(vm_core::value::Value),
    Error(String),
}

/// Host-implemented callable body.
pub trait HostCallable {
    fn invoke(&self, args: &[vm_core::value::Value]) -> HostCallResult;
}