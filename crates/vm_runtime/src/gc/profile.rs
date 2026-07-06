//! GC profile bootstrap for root policy checks.
//!
//! Spec: `PHASE-3-TARGET-PROFILE-SCHEMAS.md` §5

use vm_core::error::registry::VmStructuralErrorCode;

use crate::runtime_error::{RuntimeFailure, RuntimeResult};

use super::root_map::RootMap;

/// Collection model per frozen GcProfile schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectionModel {
    NoCollectionBootstrap,
    NonMovingTracing,
    GenerationalTracing,
    MovingCompacting,
    Incremental,
    Concurrent,
}

/// Bootstrap GC profile descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GcProfile {
    pub gc_profile_id: String,
    pub collection_model: CollectionModel,
    pub moving: bool,
    pub generational: bool,
    pub incremental: bool,
    pub concurrent: bool,
    pub requires_write_barrier: bool,
    pub requires_precise_roots: bool,
}

impl GcProfile {
    #[must_use]
    pub fn bootstrap_non_moving() -> Self {
        Self {
            gc_profile_id: "bootstrap-non-moving".to_string(),
            collection_model: CollectionModel::NoCollectionBootstrap,
            moving: false,
            generational: false,
            incremental: false,
            concurrent: false,
            requires_write_barrier: false,
            requires_precise_roots: false,
        }
    }

    #[must_use]
    pub fn moving_compacting() -> Self {
        Self {
            gc_profile_id: "moving-compacting".to_string(),
            collection_model: CollectionModel::MovingCompacting,
            moving: true,
            generational: false,
            incremental: false,
            concurrent: false,
            requires_write_barrier: true,
            requires_precise_roots: true,
        }
    }

    /// Moving GC requires precise, updateable root maps.
    pub fn validate_root_map_policy(&self, root_map: &RootMap) -> RuntimeResult<()> {
        if self.moving && !self.requires_precise_roots {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidRootMapError,
                "moving GC requires requires_precise_roots = true",
            ));
        }
        if self.moving && !root_map.updateable {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidRootMapError,
                "RootMap MUST be updateable under moving GC profile",
            ));
        }
        if self.moving && root_map.roots.is_empty() {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidRootMapError,
                "moving GC MUST NOT rely on conservative scanning; precise roots required",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::eir::schema::RootMapOwner;
    use vm_core::id::RootMapId;
    use super::super::root_location::RootLocation;

    fn sample_root_map(updateable: bool) -> RootMap {
        RootMap {
            root_map_id: RootMapId::new(0),
            owner: RootMapOwner::InterpreterFrame,
            safepoint_id: None,
            frame_map_id: None,
            roots: vec![RootLocation::Slot(vm_core::id::SlotId::new(0))],
            source_span: None,
            updateable,
        }
    }

    #[test]
    fn moving_gc_rejects_non_updateable_root_map() {
        let profile = GcProfile::moving_compacting();
        let map = sample_root_map(false);
        let err = profile.validate_root_map_policy(&map).expect_err("policy");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    #[test]
    fn moving_gc_accepts_updateable_precise_roots() {
        let profile = GcProfile::moving_compacting();
        let map = sample_root_map(true);
        assert!(profile.validate_root_map_policy(&map).is_ok());
    }
}