//! Internal runtime values for `SlotState::RuntimeInternal` slots.
//!
//! Spec: `PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md` §2.2,
//! `PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md` §6.2

use vm_core::value::Value;

/// VM-internal slot payload not exposed through ordinary binding read/write.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    /// Hidden temporary holding a runtime value.
    Temp(Value),
}

impl RuntimeValue {
    #[must_use]
    pub fn as_value(&self) -> Option<&Value> {
        match self {
            Self::Temp(value) => Some(value),
        }
    }
}