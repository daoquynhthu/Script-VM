//! FrameMap runtime representation.
//!
//! Spec: `PHASE-3-GC-METADATA-OWNERSHIP.md` §3

use std::collections::BTreeMap;

use vm_core::id::{BindingId, EirFunctionId, FrameMapId, FunctionId, ModuleId, SlotId, SlotLayoutId};
use vm_diag::source_span::SourceSpanId;

/// Binding visibility for debug/deopt reconstruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BindingVisibility {
    Local,
    Parameter,
    Capture,
    Module,
    Hidden,
}

/// Single visible binding in a frame map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisibleBinding {
    pub binding_id: BindingId,
    pub slot_id: SlotId,
    pub visibility: BindingVisibility,
    pub source_span: Option<SourceSpanId>,
}

/// Schema placeholder for active region stack state projection.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RegionStateSchema {
    pub max_region_depth: u32,
}

/// Frame layout metadata for diagnostics, deopt, and root enumeration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameMap {
    pub frame_map_id: FrameMapId,
    pub owner_function: EirFunctionId,
    pub source_function: Option<FunctionId>,
    pub module_id: ModuleId,
    pub slot_layout: SlotLayoutId,
    pub visible_bindings: Vec<VisibleBinding>,
    pub region_state_schema: RegionStateSchema,
    pub source_span: Option<SourceSpanId>,
}

/// Registry of frame maps keyed by id.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FrameMapTable {
    pub maps: BTreeMap<u32, FrameMap>,
}

impl FrameMapTable {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn get(&self, id: FrameMapId) -> Option<&FrameMap> {
        self.maps.get(&id.raw())
    }
}