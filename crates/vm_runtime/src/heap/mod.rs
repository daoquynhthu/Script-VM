//! Heap storage and object handles.
//!
//! Spec: `PHASE-3-VM-RUNTIME-ROUND1.md` §3, `PHASE-3-GC-METADATA-OWNERSHIP.md`

pub mod heap;
pub mod obj_ref;
pub mod object;

pub use heap::{value_is_mutable_aggregate, Heap};
pub use obj_ref::ObjRef;
pub use object::{HeapObject, HeapObjectKind};