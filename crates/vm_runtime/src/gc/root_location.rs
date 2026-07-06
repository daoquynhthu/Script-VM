//! Canonical root location enum.
//!
//! Spec: `PHASE-3-GC-METADATA-OWNERSHIP.md` §2.3, `PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md` §18.2

use vm_core::id::{ConstantId, ControlRegionId, ErrorHandle, SlotId};

/// Location of a GC root within interpreter/JIT state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RootLocation {
    Slot(SlotId),
    Cell(SlotId),
    Module(SlotId),
    Constant(ConstantId),
    Region(ControlRegionId),
    PendingControl,
    Error(ErrorHandle),
    HelperArg(u32),
    Host(u32),
    Jit(u32),
}

impl RootLocation {
    /// Slot id referenced by this root, if any.
    #[must_use]
    pub fn referenced_slot(&self) -> Option<SlotId> {
        match self {
            Self::Slot(id) | Self::Cell(id) | Self::Module(id) => Some(*id),
            Self::Constant(_)
            | Self::Region(_)
            | Self::PendingControl
            | Self::Error(_)
            | Self::HelperArg(_)
            | Self::Host(_)
            | Self::Jit(_) => None,
        }
    }

    /// Whether this root may reference heap objects.
    #[must_use]
    pub const fn may_reference_heap(&self) -> bool {
        matches!(
            self,
            Self::Slot(_)
                | Self::Cell(_)
                | Self::Module(_)
                | Self::PendingControl
                | Self::Error(_)
                | Self::HelperArg(_)
                | Self::Host(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_root_reports_referenced_slot() {
        let loc = RootLocation::Slot(SlotId::new(3));
        assert_eq!(loc.referenced_slot(), Some(SlotId::new(3)));
    }

    #[test]
    fn pending_control_may_reference_heap() {
        assert!(RootLocation::PendingControl.may_reference_heap());
    }
}