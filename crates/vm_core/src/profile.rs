//! Target and runtime profile types.
//!
//! Spec: `PHASE-3-TARGET-PROFILE-SCHEMAS.md`, `PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md` §3

use crate::digest::Digest;

/// Semantic version tuple for profile compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

/// Target architecture descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetArchitecture {
    Unknown,
    X86_64,
    Aarch64,
}

/// Reference to an internal profile artifact participating in cache identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProfileRef(pub u64);

/// VM-internal compatibility descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTargetProfile {
    pub vm_version: Version,
    pub architecture: TargetArchitecture,
    pub pointer_width: u32,
    pub value_layout_profile: ProfileRef,
    pub heap_profile: ProfileRef,
    pub gc_profile: ProfileRef,
    pub interpreter_profile: ProfileRef,
    pub jit_profile: Option<ProfileRef>,
    pub capability_environment_digest: Option<Digest>,
}

impl RuntimeTargetProfile {
    #[must_use]
    pub fn bootstrap() -> Self {
        Self {
            vm_version: Version::new(0, 1, 0),
            architecture: TargetArchitecture::X86_64,
            pointer_width: 64,
            value_layout_profile: ProfileRef(1),
            heap_profile: ProfileRef(1),
            gc_profile: ProfileRef(1),
            interpreter_profile: ProfileRef(1),
            jit_profile: None,
            capability_environment_digest: None,
        }
    }
}