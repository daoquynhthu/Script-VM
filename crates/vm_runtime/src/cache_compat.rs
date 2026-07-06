//! Internal cache compatibility keys, digest collection, and stale rejection.
//!
//! Spec: `PHASE-3-CACHE-COMPATIBILITY-MATRIX.md`

use std::collections::BTreeMap;

use vm_core::cache::RuntimePlanCacheKey;
use vm_core::digest::Digest;
use vm_core::error::registry::VmStructuralErrorCode;
use vm_core::profile::{ProfileRef, RuntimeTargetProfile, Version};
use vm_core::runtime_plan::RuntimePlan;

use crate::gc::profile::GcProfile;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};

use super::cache::{canonical_helper_registry_digest, runtime_plan_cache_key};

/// Schema version constants for GC metadata cache identity.
pub const ROOT_MAP_SCHEMA_VERSION: u32 = 1;
pub const FRAME_MAP_SCHEMA_VERSION: u32 = 1;
pub const SAFEPOINT_SCHEMA_VERSION: u32 = 1;

/// EIR cache key per frozen §5.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EirCacheKey {
    pub runtime_plan_digest: Digest,
    pub eir_schema_version: Version,
    pub vm_version: Version,
    pub target_profile_fingerprint: u64,
    pub helper_registry_digest: Digest,
    pub gc_metadata_schema_version: u32,
    pub source_map_digest: Option<Digest>,
}

/// GC metadata cache key per frozen §8.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GcMetadataCacheKey {
    pub root_map_schema_version: u32,
    pub frame_map_schema_version: u32,
    pub safepoint_schema_version: u32,
    pub gc_profile_digest: u64,
    pub heap_profile_ref: ProfileRef,
    pub value_layout_profile_ref: ProfileRef,
    pub write_barrier_required: bool,
    pub moving_policy: bool,
}

/// Collected digest inputs for cache key construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigestInputSet {
    pub runtime_plan_key: RuntimePlanCacheKey,
    pub helper_registry_digest: Digest,
    pub gc_profile_digest: u64,
    pub heap_profile_ref: ProfileRef,
    pub value_layout_profile_ref: ProfileRef,
}

/// Collect digest inputs from a RuntimePlan and canonical helper registry.
#[must_use]
pub fn collect_digest_inputs(plan: &RuntimePlan) -> DigestInputSet {
    let runtime_plan_key = runtime_plan_cache_key(plan);
    let helper_registry_digest = canonical_helper_registry_digest();
    DigestInputSet {
        gc_profile_digest: plan.target_profile.gc_profile.0,
        heap_profile_ref: plan.target_profile.heap_profile,
        value_layout_profile_ref: plan.target_profile.value_layout_profile,
        runtime_plan_key,
        helper_registry_digest,
    }
}

/// Build EIR cache key from digest inputs.
#[must_use]
pub fn eir_cache_key(plan: &RuntimePlan, runtime_plan_digest: Digest) -> EirCacheKey {
    let inputs = collect_digest_inputs(plan);
    let profile_fp = inputs.runtime_plan_key.target_profile_fingerprint;
    EirCacheKey {
        runtime_plan_digest,
        eir_schema_version: plan.plan_version,
        vm_version: plan.vm_version,
        target_profile_fingerprint: profile_fp,
        helper_registry_digest: inputs.helper_registry_digest,
        gc_metadata_schema_version: ROOT_MAP_SCHEMA_VERSION,
        source_map_digest: None,
    }
}

/// Build GC metadata cache key from profile and GC policy.
#[must_use]
pub fn gc_metadata_cache_key(profile: &RuntimeTargetProfile, gc: &GcProfile) -> GcMetadataCacheKey {
    GcMetadataCacheKey {
        root_map_schema_version: ROOT_MAP_SCHEMA_VERSION,
        frame_map_schema_version: FRAME_MAP_SCHEMA_VERSION,
        safepoint_schema_version: SAFEPOINT_SCHEMA_VERSION,
        gc_profile_digest: profile.gc_profile.0,
        heap_profile_ref: profile.heap_profile,
        value_layout_profile_ref: profile.value_layout_profile,
        write_barrier_required: gc.requires_write_barrier,
        moving_policy: gc.moving,
    }
}

/// Kind of internal discardable cache artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheArtifactKind {
    RuntimePlan,
    Eir,
    GcMetadata,
    HelperRegistry,
}

/// Stored cache entry with key for compatibility checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheEntry<K, V> {
    pub key: K,
    pub value: V,
}

/// Internal discardable cache with stale-entry rejection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternalCacheStore<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    entries: BTreeMap<u64, CacheEntry<K, V>>,
    next_id: u64,
}

impl<K, V> InternalCacheStore<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.entries.insert(id, CacheEntry { key, value });
        id
    }

    /// Lookup by exact key match; rejects stale entries when key differs.
    pub fn lookup(&self, key: &K) -> Result<&V, RuntimeFailure> {
        for entry in self.entries.values() {
            if &entry.key == key {
                return Ok(&entry.value);
            }
        }
        Err(RuntimeFailure::structural(
            VmStructuralErrorCode::InvalidRuntimePlanError,
            "cache miss: no entry for key",
        ))
    }

    /// Reject reuse when stored key does not match requested key (stale cache).
    pub fn reject_stale(&self, stored_key: &K, requested_key: &K) -> RuntimeResult<()> {
        if stored_key != requested_key {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidRuntimePlanError,
                "stale cache entry: key mismatch; discard and rebuild",
            ));
        }
        Ok(())
    }
}

/// Reject public-bytecode cache claims (internal-only boundary).
pub fn reject_public_bytecode_cache_claim(is_public_bytecode: bool) -> RuntimeResult<()> {
    if is_public_bytecode {
        return Err(RuntimeFailure::structural(
            VmStructuralErrorCode::BackendViolationError,
            "Phase 3 caches are internal only; public bytecode cache is forbidden",
        ));
    }
    Ok(())
}

/// Verify helper registry digest matches canonical registry for cache compatibility.
pub fn reject_helper_registry_mismatch(stored: Digest, expected: Digest) -> RuntimeResult<()> {
    if stored != expected {
        return Err(RuntimeFailure::structural(
            VmStructuralErrorCode::InvalidRuntimePlanError,
            "helper registry digest mismatch; invalidate dependent caches",
        ));
    }
    Ok(())
}

/// Verify profile fingerprint matches for cache compatibility.
pub fn reject_profile_mismatch(stored: u64, expected: u64) -> RuntimeResult<()> {
    if stored != expected {
        return Err(RuntimeFailure::structural(
            VmStructuralErrorCode::InvalidRuntimePlanError,
            "target profile fingerprint mismatch; invalidate caches",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::RuntimeHelperRegistry;
    use vm_core::runtime_plan::fixtures::minimal_valid_plan;

    #[test]
    fn digest_inputs_include_helper_registry() {
        let plan = minimal_valid_plan();
        let inputs = collect_digest_inputs(&plan);
        assert_eq!(
            inputs.helper_registry_digest,
            RuntimeHelperRegistry::canonical().expect("registry").digest()
        );
    }

    #[test]
    fn stale_cache_key_mismatch_rejected() {
        let plan = minimal_valid_plan();
        let key_a = runtime_plan_cache_key(&plan);
        let mut plan_b = plan.clone();
        plan_b.source_sir_digest = Digest(0xABCD);
        let key_b = runtime_plan_cache_key(&plan_b);
        let store = InternalCacheStore::<RuntimePlanCacheKey, u32>::new();
        let err = store.reject_stale(&key_a, &key_b).expect_err("stale");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    #[test]
    fn helper_registry_digest_mismatch_rejected() {
        let canonical = canonical_helper_registry_digest();
        let err = reject_helper_registry_mismatch(canonical, Digest(0xFFFF))
            .expect_err("mismatch");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    #[test]
    fn profile_mismatch_rejected() {
        let err = reject_profile_mismatch(1, 2).expect_err("mismatch");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    #[test]
    fn public_bytecode_cache_claim_rejected() {
        let err = reject_public_bytecode_cache_claim(true).expect_err("public");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    #[test]
    fn gc_metadata_cache_key_reflects_moving_policy() {
        let profile = RuntimeTargetProfile::bootstrap();
        let moving = GcProfile::moving_compacting();
        let non_moving = GcProfile::bootstrap_non_moving();
        let key_moving = gc_metadata_cache_key(&profile, &moving);
        let key_static = gc_metadata_cache_key(&profile, &non_moving);
        assert_ne!(key_moving, key_static);
        assert!(key_moving.moving_policy);
        assert!(!key_static.moving_policy);
    }
}