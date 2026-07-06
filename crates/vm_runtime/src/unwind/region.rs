//! Runtime region frames carrying cleanup state.
//!
//! Spec: `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md` §2.2

use vm_core::id::{ControlRegionId, EirBlockId};
use vm_diag::source_span::SourceSpanId;

use super::catch::CatchEntry;
use super::cleanup::CleanupState;
use crate::control::PendingControl;

/// Kind of control region owning cleanup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlRegionKind {
    Block,
    TryFinally,
    TryCatch,
    Function,
    Loop,
}

/// Region frame on the runtime region stack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeRegionFrame {
    pub region_id: ControlRegionId,
    pub region_kind: ControlRegionKind,
    pub cleanup_state: CleanupState,
    pub loop_target: Option<EirBlockId>,
    pub finally_entry: Option<EirBlockId>,
    pub catch_entries: Vec<CatchEntry>,
    pub source_span: Option<SourceSpanId>,
}

impl RuntimeRegionFrame {
    #[must_use]
    pub fn new(region_id: ControlRegionId, region_kind: ControlRegionKind) -> Self {
        Self {
            region_id,
            region_kind,
            cleanup_state: CleanupState::new(),
            loop_target: None,
            finally_entry: None,
            catch_entries: Vec::new(),
            source_span: None,
        }
    }

    pub fn with_catch_entries(mut self, entries: Vec<CatchEntry>) -> Self {
        self.catch_entries = entries;
        self
    }

    pub fn with_cleanup(mut self, cleanup: CleanupState) -> Self {
        self.cleanup_state = cleanup;
        self
    }
}

/// Unwind execution context: region stack + pending control.
#[derive(Debug, Clone, PartialEq)]
pub struct UnwindContext {
    pub region_frames: Vec<RuntimeRegionFrame>,
    pub pending: PendingControl,
}

impl UnwindContext {
    #[must_use]
    pub fn with_pending(pending: PendingControl) -> Self {
        Self {
            region_frames: Vec::new(),
            pending,
        }
    }

    pub fn push_region(&mut self, frame: RuntimeRegionFrame) {
        self.region_frames.push(frame);
    }

    #[must_use]
    pub fn top_region(&self) -> Option<&RuntimeRegionFrame> {
        self.region_frames.last()
    }

    #[must_use]
    pub fn top_region_mut(&mut self) -> Option<&mut RuntimeRegionFrame> {
        self.region_frames.last_mut()
    }

    pub fn pop_region(&mut self) -> Option<RuntimeRegionFrame> {
        self.region_frames.pop()
    }
}

/// Whether exiting `region` resolves the pending break/continue/return target.
#[must_use]
pub fn target_resolved_by_region(pending: &PendingControl, frame: &RuntimeRegionFrame) -> bool {
    match (pending, frame.region_kind) {
        (PendingControl::Return(_), ControlRegionKind::Function) => true,
        (PendingControl::Break(target), ControlRegionKind::Loop) => *target == frame.region_id,
        (PendingControl::Continue(target), ControlRegionKind::Loop) => *target == frame.region_id,
        _ => false,
    }
}

/// Whether pending control is already satisfied inside `region` with no cleanup left.
#[must_use]
pub fn resolved_inside_region(pending: &PendingControl, frame: &RuntimeRegionFrame) -> bool {
    frame.cleanup_state.cleanup_progress == super::cleanup::CleanupProgress::Complete
        && !frame.cleanup_state.has_cleanup_work()
        && target_resolved_by_region(pending, frame)
}