//! Pending-control root visibility metadata.
//!
//! Spec: `PHASE-3-GC-METADATA-OWNERSHIP.md` §1, `PHASE-3-CONTROL-STATE-MODEL.md`

use crate::control::PendingControl;

use super::root_location::RootLocation;

/// Metadata linking pending control to GC root enumeration.
#[derive(Debug, Clone, PartialEq)]
pub struct PendingControlRootMetadata {
    pub pending: PendingControl,
    pub root_location: RootLocation,
}

/// Collect root locations that must be visible while pending control is active.
#[must_use]
pub fn pending_control_roots(pending: &PendingControl) -> Vec<PendingControlRootMetadata> {
    if !pending.contains_heap_reference() {
        return Vec::new();
    }
    vec![PendingControlRootMetadata {
        pending: pending.clone(),
        root_location: RootLocation::PendingControl,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::id::{ControlRegionId, ErrorHandle, ObjectId};
    use vm_core::value::Value;

    #[test]
    fn pending_return_with_heap_value_is_root_visible() {
        let pending = PendingControl::Return(Some(Value::ObjectRef(ObjectId::new(1))));
        let roots = pending_control_roots(&pending);
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].root_location, RootLocation::PendingControl);
    }

    #[test]
    fn pending_break_has_no_pending_control_roots() {
        let pending = PendingControl::Break(ControlRegionId::new(0));
        assert!(pending_control_roots(&pending).is_empty());
    }

    #[test]
    fn pending_raise_is_root_visible() {
        let pending = PendingControl::Raise(ErrorHandle::new(0));
        let roots = pending_control_roots(&pending);
        assert_eq!(roots.len(), 1);
    }
}