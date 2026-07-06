//! RuntimePlan cache key construction with helper registry digest.
//!
//! Spec: `PHASE-3-CACHE-COMPATIBILITY-MATRIX.md` §4, §6

use vm_core::cache::RuntimePlanCacheKey;
use vm_core::digest::Digest;
use vm_core::runtime_plan::RuntimePlan;

use crate::helpers::RuntimeHelperRegistry;

/// Build a RuntimePlan cache key including the canonical helper registry digest.
pub fn runtime_plan_cache_key(plan: &RuntimePlan) -> RuntimePlanCacheKey {
    let registry = RuntimeHelperRegistry::canonical().expect("canonical helper registry");
    RuntimePlanCacheKey::from_plan_with_helper_registry_digest(plan, Some(registry.digest()))
}

/// Canonical helper registry digest for cache identity.
#[must_use]
pub fn canonical_helper_registry_digest() -> Digest {
    RuntimeHelperRegistry::canonical()
        .expect("canonical helper registry")
        .digest()
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::cache::RuntimePlanCacheKey;
    use vm_core::runtime_plan::fixtures::minimal_valid_plan;

    #[test]
    fn runtime_plan_cache_key_includes_helper_registry_digest() {
        let plan = minimal_valid_plan();
        let key = runtime_plan_cache_key(&plan);
        assert_eq!(
            key.helper_registry_digest,
            Some(canonical_helper_registry_digest())
        );
    }

    #[test]
    fn cache_key_changes_when_helper_registry_digest_changes() {
        let plan = minimal_valid_plan();
        let key_a = runtime_plan_cache_key(&plan);
        let key_b = RuntimePlanCacheKey::from_plan_with_helper_registry_digest(
            &plan,
            Some(Digest(0xBEEF)),
        );
        assert_ne!(key_a, key_b);
    }
}