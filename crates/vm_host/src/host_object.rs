//! Host object wrapper shell.
//!
//! Spec: `PHASE-3-HOST-BOUNDARY-CONTRACT.md` §4

use vm_core::id::{CapabilityId, ObjectId};

/// Host object identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HostObjectId(pub u32);

/// Host object lifetime policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostObjectLifetime {
    CallScoped,
    ResourceScoped,
    ExplicitHandle,
}

/// Descriptor for host-managed object state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostObjectDescriptor {
    pub may_allocate: bool,
}

/// VM-controlled wrapper over native host state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostObjectWrapper {
    pub host_object_id: HostObjectId,
    pub descriptor: HostObjectDescriptor,
    pub capability_origin: Option<CapabilityId>,
    pub lifetime: HostObjectLifetime,
}

/// Host-managed object reference linked to a VM heap object id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HostObjectRef {
    pub object_id: ObjectId,
}