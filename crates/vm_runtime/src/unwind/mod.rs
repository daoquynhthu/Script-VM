//! Structured unwinding implementation.
//!
//! Spec: `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md`

pub mod catch;
pub mod cleanup;
pub mod combine;
pub mod perform;
pub mod region;

pub use catch::{dispatch_catch_handlers, CatchDispatchResult, CatchEntry};
pub use cleanup::{
    CleanupProgress, CleanupState, DeferredCallable, FinallyState, ResourceCleanup, ResourceState,
};
pub use combine::{
    attach_suppressed_to_primary, combine_cleanup_result, finally_override, CleanupStepResult,
};
pub use perform::{perform_unwind, UnwindExecutor, UnwindOutcome};
pub use region::{
    target_resolved_by_region, ControlRegionKind, RuntimeRegionFrame, UnwindContext,
};