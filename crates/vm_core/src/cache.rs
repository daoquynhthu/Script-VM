//! Cache identity for RuntimePlan and derived artifacts.
//!
//! Spec: `PHASE-3-CACHE-COMPATIBILITY-MATRIX.md`, `PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md` §16

use crate::digest::Digest;
use crate::profile::{RuntimeTargetProfile, Version};
use crate::runtime_plan::schema::RuntimePlan;

/// Cache key components for RuntimePlan and derived artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheIdentity {
    pub plan_schema_version: u32,
    pub profile_fingerprint: u64,
}

impl CacheIdentity {
    #[must_use]
    pub fn from_profile(profile: &RuntimeTargetProfile) -> Self {
        let profile_fingerprint = (profile.vm_version.major as u64) << 32
            | (profile.vm_version.minor as u64) << 16
            | profile.vm_version.patch as u64
            | (profile.pointer_width as u64) << 48;
        Self {
            plan_schema_version: 1,
            profile_fingerprint,
        }
    }
}

/// Full RuntimePlan cache key per frozen §16 requirements.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RuntimePlanCacheKey {
    pub source_sir_digest: Digest,
    pub phase2_schema_version: Version,
    pub vm_version: Version,
    pub plan_version: Version,
    pub target_profile_fingerprint: u64,
    pub capability_environment_digest: Option<Digest>,
    pub helper_registry_digest: Option<Digest>,
}

impl RuntimePlanCacheKey {
    #[must_use]
    pub fn from_plan(plan: &RuntimePlan) -> Self {
        Self::from_plan_with_helper_registry_digest(plan, None)
    }

    #[must_use]
    pub fn from_plan_with_helper_registry_digest(
        plan: &RuntimePlan,
        helper_registry_digest: Option<Digest>,
    ) -> Self {
        let base = CacheIdentity::from_profile(&plan.target_profile);
        Self {
            source_sir_digest: plan.source_sir_digest,
            phase2_schema_version: plan.phase2_schema_version,
            vm_version: plan.vm_version,
            plan_version: plan.plan_version,
            target_profile_fingerprint: base.profile_fingerprint,
            capability_environment_digest: plan
                .target_profile
                .capability_environment_digest
                .or(plan.capability_gate_plan.environment_digest),
            helper_registry_digest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_plan::fixtures::minimal_valid_plan;

    #[test]
    fn cache_key_includes_plan_identity_fields() {
        let plan = minimal_valid_plan();
        let key_a = RuntimePlanCacheKey::from_plan(&plan);
        let key_b = RuntimePlanCacheKey::from_plan(&plan);
        assert_eq!(key_a, key_b);
        assert_eq!(key_a.source_sir_digest, plan.source_sir_digest);
        assert_eq!(key_a.vm_version, plan.vm_version);
    }

    #[test]
    fn cache_key_changes_when_sir_digest_changes() {
        let mut plan = minimal_valid_plan();
        let key_a = RuntimePlanCacheKey::from_plan(&plan);
        plan.source_sir_digest = Digest(0x1234);
        let key_b = RuntimePlanCacheKey::from_plan(&plan);
        assert_ne!(key_a, key_b);
    }
}