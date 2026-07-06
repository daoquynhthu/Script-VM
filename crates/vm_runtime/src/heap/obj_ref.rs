//! Typed object handles over internal `ObjectId` storage.
//!
//! Spec: `PHASE-3-VM-RUNTIME-ROUND1.md` §3

use std::marker::PhantomData;

use vm_core::id::ObjectId;

/// Generational heap handle; not a public language ABI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjRef<T = ()> {
    id: ObjectId,
    generation: u32,
    _marker: PhantomData<fn() -> T>,
}

impl<T> ObjRef<T> {
    #[must_use]
    pub const fn new(id: ObjectId, generation: u32) -> Self {
        Self {
            id,
            generation,
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub const fn id(self) -> ObjectId {
        self.id
    }

    #[must_use]
    pub const fn generation(self) -> u32 {
        self.generation
    }

    #[must_use]
    pub fn erase(self) -> ObjRef<()> {
        ObjRef::new(self.id, self.generation)
    }
}

impl ObjRef<()> {
    #[must_use]
    pub fn typed<T>(self) -> ObjRef<T> {
        ObjRef::new(self.id, self.generation)
    }
}