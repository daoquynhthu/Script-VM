//! Runtime value scaffold.
//!
//! Spec: `PHASE-3-VALUE-KEY-STRING-SEMANTICS.md`, `PHASE-3-VM-RUNTIME-ROUND1.md`

use crate::id::{ErrorHandle, ObjectId};

/// Runtime value representation.
///
/// Heap-backed aggregates are referenced via `ObjectRef`; semantics live in `vm_runtime`.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    ObjectRef(ObjectId),
    /// Language-level error value for raise/control-flow paths.
    Error(ErrorHandle),
}

impl Value {
    #[must_use]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}