//! Host root map scaffold.
//!
//! Spec: `PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md`, `PHASE-3-HOST-BOUNDARY-CONTRACT.md`

use crate::host_object::HostObjectRef;

/// Roots exposed by the host for safepoint/GC coordination.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HostRootMap {
    roots: Vec<HostObjectRef>,
}

impl HostRootMap {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_root(&mut self, root: HostObjectRef) {
        self.roots.push(root);
    }

    #[must_use]
    pub fn roots(&self) -> &[HostObjectRef] {
        &self.roots
    }
}