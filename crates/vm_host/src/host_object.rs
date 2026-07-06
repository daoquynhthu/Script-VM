//! Host object boundary scaffold.
//!
//! Spec: `PHASE-3-HOST-BOUNDARY-CONTRACT.md`

use vm_core::id::ObjectId;

/// Host-managed object reference at the capability-gated boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HostObjectRef {
    pub object_id: ObjectId,
}