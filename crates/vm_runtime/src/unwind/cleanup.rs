//! Cleanup stack records for structured unwinding.
//!
//! Spec: `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md` §2.3–§2.4

/// Progress through defer → resource → finally cleanup phases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CleanupProgress {
    #[default]
    NotStarted,
    RunningDefers,
    RunningResources,
    RunningFinally,
    Complete,
}

/// Registered defer callable (evaluated at registration; invoked at unwind).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeferredCallable {
    pub callable_id: u32,
    pub label: String,
}

/// Resource lifecycle for exactly-once close.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceState {
    Open,
    Closing,
    Closed,
    Failed,
}

/// Registered resource cleanup entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceCleanup {
    pub resource_id: u32,
    pub state: ResourceState,
    pub label: String,
}

impl ResourceCleanup {
    #[must_use]
    pub fn new(resource_id: u32, label: impl Into<String>) -> Self {
        Self {
            resource_id,
            state: ResourceState::Open,
            label: label.into(),
        }
    }

    pub fn begin_close(&mut self) -> Result<(), ResourceCloseError> {
        match self.state {
            ResourceState::Open => {
                self.state = ResourceState::Closing;
                Ok(())
            }
            ResourceState::Closing => Err(ResourceCloseError::AlreadyClosing),
            ResourceState::Closed => Err(ResourceCloseError::AlreadyClosed),
            ResourceState::Failed => Err(ResourceCloseError::PreviouslyFailed),
        }
    }

    pub fn mark_closed(&mut self) {
        self.state = ResourceState::Closed;
    }

    pub fn mark_failed(&mut self) {
        self.state = ResourceState::Failed;
    }
}

/// Exactly-once close violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceCloseError {
    AlreadyClosing,
    AlreadyClosed,
    PreviouslyFailed,
}

/// Finally block execution state for a region.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FinallyState {
    #[default]
    None,
    Pending {
        action_id: u32,
    },
    Complete,
}

/// Per-region cleanup stacks and progress.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CleanupState {
    pub defer_stack: Vec<DeferredCallable>,
    pub resource_stack: Vec<ResourceCleanup>,
    pub finally_state: FinallyState,
    pub cleanup_progress: CleanupProgress,
}

impl CleanupState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_defer(&mut self, callable_id: u32, label: impl Into<String>) {
        self.defer_stack.push(DeferredCallable {
            callable_id,
            label: label.into(),
        });
    }

    pub fn register_resource(&mut self, resource_id: u32, label: impl Into<String>) {
        self.resource_stack.push(ResourceCleanup::new(resource_id, label));
    }

    pub fn set_finally(&mut self, action_id: u32) {
        self.finally_state = FinallyState::Pending { action_id };
    }

    #[must_use]
    pub fn has_cleanup_work(&self) -> bool {
        !self.defer_stack.is_empty()
            || !self.resource_stack.is_empty()
            || matches!(self.finally_state, FinallyState::Pending { .. })
    }
}