//! Pending control and region stack runtime storage.
//!
//! Spec: `PHASE-3-CONTROL-STATE-MODEL.md` §2, §6–§7

use vm_core::control::ControlState;
use vm_core::id::{ControlRegionId, DeoptId, ErrorHandle};
use vm_core::value::Value;

/// Deferred control while cleanup regions are active.
#[derive(Debug, Clone, PartialEq)]
pub enum PendingControl {
    Return(Option<Value>),
    Break(ControlRegionId),
    Continue(ControlRegionId),
    Raise(ErrorHandle),
}

impl PendingControl {
    /// Pending control containing heap references must be root-visible at safepoints.
    #[must_use]
    pub fn contains_heap_reference(&self) -> bool {
        match self {
            Self::Return(Some(value)) => matches!(value, Value::ObjectRef(_)),
            Self::Raise(_) => true,
            Self::Return(None) | Self::Break(_) | Self::Continue(_) => false,
        }
    }

    pub fn to_control_state(&self) -> ControlState {
        match self {
            Self::Return(value) => ControlState::Return(value.clone()),
            Self::Break(region) => ControlState::Break(*region),
            Self::Continue(region) => ControlState::Continue(*region),
            Self::Raise(handle) => ControlState::Raise(*handle),
        }
    }
}

/// Language-level control result for helpers and frame execution.
#[derive(Debug, Clone, PartialEq)]
pub enum VmControl {
    Normal(Option<Value>),
    Return(Option<Value>),
    Break(ControlRegionId),
    Continue(ControlRegionId),
    Raise(ErrorHandle),
}

impl VmControl {
    #[must_use]
    pub fn to_control_state(&self) -> ControlState {
        match self {
            Self::Normal(value) => ControlState::Normal(value.clone()),
            Self::Return(value) => ControlState::Return(value.clone()),
            Self::Break(region) => ControlState::Break(*region),
            Self::Continue(region) => ControlState::Continue(*region),
            Self::Raise(handle) => ControlState::Raise(*handle),
        }
    }
}

/// Active control-region stack shell (cleanup/unwind linkage in WP-10).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RegionStack {
    regions: Vec<ControlRegionId>,
}

impl RegionStack {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, region: ControlRegionId) {
        self.regions.push(region);
    }

    pub fn pop(&mut self) -> Option<ControlRegionId> {
        self.regions.pop()
    }

    #[must_use]
    pub fn current(&self) -> Option<ControlRegionId> {
        self.regions.last().copied()
    }

    #[must_use]
    pub fn depth(&self) -> usize {
        self.regions.len()
    }
}

/// Map VmControl layers onto canonical ControlState (excludes Deopt/VmError).
#[must_use]
pub fn control_state_from_deopt(deopt_id: DeoptId) -> ControlState {
    ControlState::Deopt(deopt_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::id::ObjectId;

    #[test]
    fn pending_return_with_heap_value_is_root_visible() {
        let pending = PendingControl::Return(Some(Value::ObjectRef(ObjectId::new(0))));
        assert!(pending.contains_heap_reference());
    }

    #[test]
    fn pending_break_has_no_heap_reference() {
        let pending = PendingControl::Break(ControlRegionId::new(1));
        assert!(!pending.contains_heap_reference());
    }

    #[test]
    fn vm_control_maps_to_control_state() {
        let control = VmControl::Normal(Some(Value::Int(3)));
        assert_eq!(
            control.to_control_state(),
            ControlState::Normal(Some(Value::Int(3)))
        );
    }

    #[test]
    fn region_stack_push_pop() {
        let mut stack = RegionStack::new();
        stack.push(ControlRegionId::new(1));
        assert_eq!(stack.current(), Some(ControlRegionId::new(1)));
        assert_eq!(stack.pop(), Some(ControlRegionId::new(1)));
        assert_eq!(stack.depth(), 0);
    }
}