//! Host function boundary scaffold.
//!
//! Spec: `PHASE-3-HOST-BOUNDARY-CONTRACT.md`

use vm_core::value::Value;

/// Host-callable function handle scaffold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFunctionId(pub u32);

/// Result of a host function invocation at the boundary.
#[derive(Debug, Clone, PartialEq)]
pub enum HostCallResult {
    Return(Value),
    Error(String),
}