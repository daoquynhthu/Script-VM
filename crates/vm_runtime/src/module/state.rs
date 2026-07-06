//! Module lifecycle state and transition enforcement.
//!
//! Spec: `PHASE-3-MODULE-RUNTIME-CONTRACT.md` §2–§3

use vm_core::error::registry::VmStructuralErrorCode;

use crate::runtime_error::{RuntimeFailure, RuntimeResult};

/// Canonical module lifecycle states (Phase 3 only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleState {
    Unloaded,
    Loading,
    Initializing,
    Initialized,
    Failed,
}

impl ModuleState {
    /// Whether imports may read initialized exports from this module.
    #[must_use]
    pub const fn allows_export_reads(self) -> bool {
        matches!(self, Self::Initializing | Self::Initialized)
    }

    /// Whether ordinary imports must fail without explicit host retry.
    #[must_use]
    pub const fn blocks_ordinary_import(self) -> bool {
        matches!(self, Self::Failed)
    }
}

/// Validate a state transition per frozen contract.
pub fn validate_transition(from: ModuleState, to: ModuleState) -> RuntimeResult<()> {
    if is_allowed_transition(from, to) {
        Ok(())
    } else {
        Err(RuntimeFailure::structural(
            VmStructuralErrorCode::InvalidRuntimePlanError,
            format!("forbidden module state transition: {from:?} -> {to:?}"),
        ))
    }
}

/// Validate transition with explicit host retry flag for `Failed -> Loading`.
pub fn validate_transition_with_retry(
    from: ModuleState,
    to: ModuleState,
    explicit_host_retry: bool,
) -> RuntimeResult<()> {
    if from == ModuleState::Failed && to == ModuleState::Loading && !explicit_host_retry {
        return Err(RuntimeFailure::structural(
            VmStructuralErrorCode::InvalidRuntimePlanError,
            "automatic retry after Failed is forbidden; host must request explicit retry",
        ));
    }
    validate_transition(from, to)
}

fn is_allowed_transition(from: ModuleState, to: ModuleState) -> bool {
    use ModuleState::*;
    matches!(
        (from, to),
        (Unloaded, Loading)
            | (Loading, Initializing)
            | (Initializing, Initialized)
            | (Initializing, Failed)
            | (Loading, Failed)
            | (Failed, Loading)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_transitions_pass() {
        assert!(validate_transition(ModuleState::Unloaded, ModuleState::Loading).is_ok());
        assert!(validate_transition(ModuleState::Loading, ModuleState::Initializing).is_ok());
        assert!(validate_transition(ModuleState::Initializing, ModuleState::Initialized).is_ok());
        assert!(validate_transition(ModuleState::Initializing, ModuleState::Failed).is_ok());
        assert!(validate_transition(ModuleState::Loading, ModuleState::Failed).is_ok());
        assert!(
            validate_transition_with_retry(ModuleState::Failed, ModuleState::Loading, true).is_ok()
        );
    }

    #[test]
    fn forbidden_transitions_rejected() {
        assert!(validate_transition(ModuleState::Initialized, ModuleState::Loading).is_err());
        assert!(validate_transition(ModuleState::Initialized, ModuleState::Initializing).is_err());
        assert!(validate_transition(ModuleState::Failed, ModuleState::Initializing).is_err());
        assert!(validate_transition(ModuleState::Failed, ModuleState::Initialized).is_err());
        assert!(validate_transition(ModuleState::Unloaded, ModuleState::Initializing).is_err());
    }

    #[test]
    fn automatic_retry_after_failed_rejected() {
        let err = validate_transition_with_retry(ModuleState::Failed, ModuleState::Loading, false)
            .expect_err("must reject automatic retry");
        assert!(matches!(
            err,
            RuntimeFailure::Structural(_)
        ));
    }
}