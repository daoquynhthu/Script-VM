//! Canonical control-state scaffold.
//!
//! Spec: `PHASE-3-CONTROL-STATE-MODEL.md` §2

use crate::error::VmError;
use crate::id::{ControlRegionId, DeoptId, ErrorHandle};
use crate::value::Value;

/// Canonical control state for VM execution.
#[derive(Debug, Clone, PartialEq)]
pub enum ControlState {
    Normal(Option<Value>),
    Return(Option<Value>),
    Break(ControlRegionId),
    Continue(ControlRegionId),
    Raise(ErrorHandle),
    Halt,
    Deopt(DeoptId),
    VmError(VmError),
}

impl ControlState {
    #[must_use]
    pub const fn normal(value: Option<Value>) -> Self {
        Self::Normal(value)
    }
}