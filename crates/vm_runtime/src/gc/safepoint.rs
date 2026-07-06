//! SafepointRecord runtime representation.
//!
//! Spec: `PHASE-3-GC-METADATA-OWNERSHIP.md` §4, `PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md` §17

use std::collections::BTreeMap;

use vm_core::id::{CallSiteId, DeoptId, FrameMapId, RootMapId, SafepointId};
use vm_core::runtime_plan::schema::{EirLocation, SafepointKind};
use vm_diag::source_span::SourceSpanId;

/// Owner of a safepoint placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SafepointOwner {
    Interpreter,
    EirFunction,
    RuntimeHelper,
    JitCompiledFunction,
    HostCall,
}

/// Location discriminator for safepoint placement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafepointLocation {
    Eir(EirLocation),
    HelperCallSite(CallSiteId),
    MachineCodeOffset(u32),
    HostBoundary,
    ModuleImportBoundary,
}

/// Canonical safepoint metadata linking roots and frames.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafepointRecord {
    pub safepoint_id: SafepointId,
    pub kind: SafepointKind,
    pub owner: SafepointOwner,
    pub location: SafepointLocation,
    pub root_map: RootMapId,
    pub frame_map: Option<FrameMapId>,
    pub deopt_id: Option<DeoptId>,
    pub source_span: Option<SourceSpanId>,
}

impl SafepointRecord {
    /// Whether GC may run at this safepoint (requires linked RootMap).
    #[must_use]
    pub const fn gc_may_run(&self) -> bool {
        matches!(
            self.kind,
            SafepointKind::Allocation
                | SafepointKind::FunctionCall
                | SafepointKind::HelperCall
                | SafepointKind::HostCall
                | SafepointKind::LoopBackedge
        )
    }
}

/// Registry of safepoint records.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SafepointTable {
    pub records: BTreeMap<u32, SafepointRecord>,
}

impl SafepointTable {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn get(&self, id: SafepointId) -> Option<&SafepointRecord> {
        self.records.get(&id.raw())
    }
}