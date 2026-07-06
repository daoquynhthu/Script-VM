//! Runtime helper registry construction and lookup.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-REGISTRY.md`

use std::collections::{BTreeMap, BTreeSet};

use vm_core::digest::Digest;
use vm_core::id::RuntimeHelperId;

use super::canonical::canonical_descriptors;
use super::schema::RuntimeHelperDescriptor;
use super::validate::{RegistryBuildError, RegistryValidationError, validate_registry};

/// Placeholder tracking which helpers have native implementations (WP-17+).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HelperImplementationRegistry {
    implemented: BTreeSet<RuntimeHelperId>,
}

impl HelperImplementationRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark_implemented(&mut self, helper_id: RuntimeHelperId) {
        self.implemented.insert(helper_id);
    }

    pub fn mark_all_from_registry(&mut self, registry: &RuntimeHelperRegistry) {
        for id in registry.helper_ids() {
            self.implemented.insert(id);
        }
    }

    #[must_use]
    pub fn contains(&self, helper_id: RuntimeHelperId) -> bool {
        self.implemented.contains(&helper_id)
    }
}

/// Canonical runtime helper lookup table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeHelperRegistry {
    entries: BTreeMap<u32, RuntimeHelperDescriptor>,
    name_index: BTreeMap<String, RuntimeHelperId>,
    digest: Digest,
}

impl RuntimeHelperRegistry {
    /// Build the frozen canonical registry with all implementations marked present.
    pub fn canonical() -> Result<Self, RegistryBuildError> {
        let registry = Self::from_descriptors(canonical_descriptors())?;
        Ok(registry)
    }

    /// Build a registry from descriptors, rejecting duplicate ids/names at construction.
    pub fn from_descriptors(
        descriptors: Vec<RuntimeHelperDescriptor>,
    ) -> Result<Self, RegistryBuildError> {
        let mut entries = BTreeMap::new();
        let mut name_index = BTreeMap::new();

        for descriptor in descriptors {
            let id_key = descriptor.helper_id.raw();
            if entries.contains_key(&id_key) {
                return Err(RegistryBuildError::DuplicateHelperId(descriptor.helper_id));
            }
            if name_index.contains_key(&descriptor.name) {
                return Err(RegistryBuildError::DuplicateHelperName(descriptor.name));
            }
            name_index.insert(descriptor.name.clone(), descriptor.helper_id);
            entries.insert(id_key, descriptor);
        }

        let digest = compute_registry_digest(&entries);
        Ok(Self {
            entries,
            name_index,
            digest,
        })
    }

    /// Validate registry descriptor policy constraints.
    pub fn validate(&self) -> Result<(), RegistryValidationError> {
        validate_registry(self)
    }

    /// Validate descriptor/implementation consistency placeholder.
    pub fn validate_implementations(
        &self,
        implementations: &HelperImplementationRegistry,
    ) -> Result<(), RegistryValidationError> {
        for id in self.helper_ids() {
            if !implementations.contains(id) {
                return Err(RegistryValidationError::DescriptorWithoutImplementation(id));
            }
        }
        for id in &implementations.implemented {
            if !self.contains(*id) {
                return Err(RegistryValidationError::ImplementationWithoutDescriptor(*id));
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn digest(&self) -> Digest {
        self.digest
    }

    #[must_use]
    pub fn contains(&self, helper_id: RuntimeHelperId) -> bool {
        helper_id.is_valid() && self.entries.contains_key(&helper_id.raw())
    }

    #[must_use]
    pub fn lookup(&self, helper_id: RuntimeHelperId) -> Option<&RuntimeHelperDescriptor> {
        self.entries.get(&helper_id.raw())
    }

    #[must_use]
    pub fn lookup_by_name(&self, name: &str) -> Option<&RuntimeHelperDescriptor> {
        let id = self.name_index.get(name)?;
        self.lookup(*id)
    }

    #[must_use]
    pub fn helper_ids(&self) -> impl Iterator<Item = RuntimeHelperId> + '_ {
        self.entries.values().map(|entry| entry.helper_id)
    }

    #[must_use]
    pub fn may_collect_helper_ids(&self) -> Vec<RuntimeHelperId> {
        self.entries
            .values()
            .filter(|entry| entry.may_collect())
            .map(|entry| entry.helper_id)
            .collect()
    }

    #[must_use]
    pub fn may_raise_helper_ids(&self) -> Vec<RuntimeHelperId> {
        self.entries
            .values()
            .filter(|entry| entry.may_raise)
            .map(|entry| entry.helper_id)
            .collect()
    }

    #[must_use]
    pub fn descriptors(&self) -> impl Iterator<Item = &RuntimeHelperDescriptor> {
        self.entries.values()
    }
}

fn compute_registry_digest(entries: &BTreeMap<u32, RuntimeHelperDescriptor>) -> Digest {
    let mut hash: u64 = 0xcbf29ce484222325;
    for entry in entries.values() {
        hash ^= entry.helper_id.raw() as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        for byte in entry.name.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= entry.family as u8 as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Digest(hash)
}