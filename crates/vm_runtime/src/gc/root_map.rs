//! RootMap runtime representation.
//!
//! Spec: `PHASE-3-GC-METADATA-OWNERSHIP.md` §2

use std::collections::BTreeMap;

use vm_core::eir::schema::{RootMap as EirRootMap, RootMapOwner, RootMapTable as EirRootMapTable};
use vm_core::id::{FrameMapId, RootMapId, SafepointId};
use vm_diag::source_span::SourceSpanId;

use super::root_location::RootLocation;

/// Runtime RootMap with moving-GC updateability hook.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootMap {
    pub root_map_id: RootMapId,
    pub owner: RootMapOwner,
    pub safepoint_id: Option<SafepointId>,
    pub frame_map_id: Option<FrameMapId>,
    pub roots: Vec<RootLocation>,
    pub source_span: Option<SourceSpanId>,
    /// MUST be true when moving GC profile is enabled.
    pub updateable: bool,
}

impl RootMap {
    pub fn from_eir(eir: &EirRootMap) -> Self {
        Self {
            root_map_id: eir.root_map_id,
            owner: eir.owner,
            safepoint_id: eir.safepoint_id,
            frame_map_id: eir.frame_map_id,
            roots: eir
                .roots
                .iter()
                .map(|slot| RootLocation::Slot(*slot))
                .collect(),
            source_span: eir.source_span,
            updateable: false,
        }
    }

    pub fn with_updateable(mut self, updateable: bool) -> Self {
        self.updateable = updateable;
        self
    }
}

/// Registry of runtime root maps.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RootMapTable {
    pub maps: BTreeMap<u32, RootMap>,
}

impl RootMapTable {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_eir_table(table: &EirRootMapTable) -> Self {
        let maps = table
            .maps
            .values()
            .map(|m| (m.root_map_id.raw(), RootMap::from_eir(m)))
            .collect();
        Self { maps }
    }

    #[must_use]
    pub fn get(&self, id: RootMapId) -> Option<&RootMap> {
        self.maps.get(&id.raw())
    }
}