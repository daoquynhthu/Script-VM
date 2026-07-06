//! VM host boundary interface
//!
//! This crate defines the host boundary contract and wrappers.
//!
//! Spec: `PHASE-3-HOST-BOUNDARY-CONTRACT.md`

pub mod call;
pub mod error;
pub mod host_function;
pub mod host_object;
pub mod host_root;

pub use call::{execute_host_call, runtime_failure_from_host_error, HostCallContext};
pub use error::{normalize_host_call_result, NormalizedHostError};
pub use host_function::{
    ArityShape, HostCallable, HostCallResult, HostFunctionDescriptor, HostFunctionId,
    HostFunctionWrapper, HostParameterPolicy, HostResultPolicy,
};
pub use host_object::{
    HostObjectDescriptor, HostObjectId, HostObjectLifetime, HostObjectRef, HostObjectWrapper,
};
pub use host_root::{
    HostBoundaryId, HostRootEntry, HostRootId, HostRootLifetime, HostRootRegistry,
};