//! Fast interpreter scaffold.
//!
//! Spec: `PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md`, `PHASE-3-CONTROL-STATE-MODEL.md`

use vm_core::control::ControlState;
use vm_core::value::Value;
use vm_core::runtime_plan::RuntimePlan;

/// Interpreter execution context scaffold.
#[derive(Debug, Default)]
pub struct Interpreter {
    pub halted: bool,
}

impl Interpreter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Minimal execution entry; full interpreter arrives in WP-17.
    pub fn run_plan(&mut self, _plan: &RuntimePlan) -> ControlState {
        self.halted = true;
        ControlState::normal(Some(Value::None))
    }
}