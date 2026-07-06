//! Helper registry validation.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-REGISTRY.md` §8, `PHASE-3-VALIDATION-MATRIX.md` (P3-V6)

use vm_core::eir::validate::HelperRegistryView;
use vm_core::id::RuntimeHelperId;

use super::registry::RuntimeHelperRegistry;
use super::schema::{
    HelperJitCallPolicy, HelperSourceMappingPolicy, RuntimeHelperFamily,
};

/// Errors while constructing a helper registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryBuildError {
    DuplicateHelperId(RuntimeHelperId),
    DuplicateHelperName(String),
}

impl std::fmt::Display for RegistryBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for RegistryBuildError {}

/// Helper registry validation failure codes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryValidationError {
    DuplicateHelperId(RuntimeHelperId),
    DuplicateHelperName(String),
    MissingHelper(RuntimeHelperId),
    DescriptorWithoutImplementation(RuntimeHelperId),
    ImplementationWithoutDescriptor(RuntimeHelperId),
    MayCollectWithoutRootsVisible(RuntimeHelperId),
    MayRaiseWithoutSourceMappingPolicy(RuntimeHelperId),
    JitCallableWithoutJitPolicy(RuntimeHelperId),
    CapabilityHelperWithoutMetadata(RuntimeHelperId),
}

impl std::fmt::Display for RegistryValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for RegistryValidationError {}

/// Validate helper descriptor policy constraints for every registry entry.
pub fn validate_registry(registry: &RuntimeHelperRegistry) -> Result<(), RegistryValidationError> {
    let mut seen_ids = std::collections::BTreeSet::new();
    let mut seen_names = std::collections::BTreeSet::new();

    for descriptor in registry.descriptors() {
        if !seen_ids.insert(descriptor.helper_id) {
            return Err(RegistryValidationError::DuplicateHelperId(descriptor.helper_id));
        }
        if !seen_names.insert(descriptor.name.clone()) {
            return Err(RegistryValidationError::DuplicateHelperName(descriptor.name.clone()));
        }

        if descriptor.may_collect() && !descriptor.requires_roots_visible {
            return Err(RegistryValidationError::MayCollectWithoutRootsVisible(
                descriptor.helper_id,
            ));
        }

        if descriptor.may_raise
            && descriptor.source_mapping_policy == HelperSourceMappingPolicy::NotRequired
        {
            return Err(RegistryValidationError::MayRaiseWithoutSourceMappingPolicy(
                descriptor.helper_id,
            ));
        }

        if descriptor.is_jit_callable()
            && descriptor.jit_call_policy == HelperJitCallPolicy::NotJitCallable
        {
            return Err(RegistryValidationError::JitCallableWithoutJitPolicy(
                descriptor.helper_id,
            ));
        }

        if descriptor.family == RuntimeHelperFamily::Capability
            && (descriptor.required_capability.is_none() || descriptor.effect.is_none())
        {
            return Err(RegistryValidationError::CapabilityHelperWithoutMetadata(
                descriptor.helper_id,
            ));
        }
    }
    Ok(())
}

/// Reject RuntimeHelperOp references to helpers absent from the registry.
pub fn validate_helper_reference(
    registry: &RuntimeHelperRegistry,
    helper_id: RuntimeHelperId,
) -> Result<(), RegistryValidationError> {
    if registry.contains(helper_id) {
        Ok(())
    } else {
        Err(RegistryValidationError::MissingHelper(helper_id))
    }
}

/// Build an EIR validation view from the canonical helper registry.
#[must_use]
pub fn eir_validation_view(registry: &RuntimeHelperRegistry) -> HelperRegistryView {
    HelperRegistryView::from_ids(registry.helper_ids())
        .with_may_collect(registry.may_collect_helper_ids())
        .with_may_raise(registry.may_raise_helper_ids())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::canonical::canonical_descriptors;
    use crate::helpers::registry::{HelperImplementationRegistry, RuntimeHelperRegistry};
    use crate::helpers::schema::{
        HelperGcBehavior, HelperJitCallPolicy, HelperResultType, HelperSourceMappingPolicy,
        RuntimeHelperDescriptor, RuntimeHelperFamily, RuntimeHelperSignature,
    };

    #[test]
    fn canonical_registry_validates() {
        let registry = RuntimeHelperRegistry::canonical().expect("canonical build");
        assert!(registry.validate().is_ok());
    }

    #[test]
    fn duplicate_helper_id_is_rejected_at_build() {
        let mut descriptors = canonical_descriptors();
        descriptors[1].helper_id = descriptors[0].helper_id;
        let err = RuntimeHelperRegistry::from_descriptors(descriptors).unwrap_err();
        assert!(matches!(
            err,
            RegistryBuildError::DuplicateHelperId(_)
        ));
    }

    #[test]
    fn duplicate_helper_name_is_rejected_at_build() {
        let mut descriptors = canonical_descriptors();
        let duplicate_name = descriptors[0].name.clone();
        descriptors[1].name = duplicate_name;
        let err = RuntimeHelperRegistry::from_descriptors(descriptors).unwrap_err();
        assert!(matches!(
            err,
            RegistryBuildError::DuplicateHelperName(_)
        ));
    }

    #[test]
    fn missing_helper_reference_is_rejected() {
        let registry = RuntimeHelperRegistry::canonical().expect("canonical build");
        let err = validate_helper_reference(&registry, RuntimeHelperId::new(999)).unwrap_err();
        assert!(matches!(err, RegistryValidationError::MissingHelper(_)));
    }

    #[test]
    fn may_raise_helper_requires_source_mapping_policy() {
        let bad = RuntimeHelperDescriptor {
            helper_id: RuntimeHelperId::new(100),
            name: "helper_bad_raise".to_string(),
            family: RuntimeHelperFamily::TypeCheck,
            signature: RuntimeHelperSignature {
                result: HelperResultType::Value,
                calling_convention: super::super::schema::HelperCallingConvention::InterpreterDirect,
            },
            may_allocate: false,
            may_raise: true,
            may_unwind: false,
            is_safepoint: false,
            requires_roots_visible: false,
            required_capability: None,
            effect: None,
            gc_behavior: HelperGcBehavior::NoAllocation,
            jit_call_policy: HelperJitCallPolicy::InterpreterOnly,
            source_mapping_policy: HelperSourceMappingPolicy::NotRequired,
        };
        let extended = RuntimeHelperRegistry::from_descriptors({
            let mut d = canonical_descriptors();
            d.push(bad);
            d
        })
        .expect("build extended");
        let err = extended.validate().unwrap_err();
        assert!(matches!(
            err,
            RegistryValidationError::MayRaiseWithoutSourceMappingPolicy(_)
        ));
    }

    #[test]
    fn may_collect_helper_requires_roots_visible_policy() {
        let bad = RuntimeHelperDescriptor {
            helper_id: RuntimeHelperId::new(101),
            name: "helper_bad_collect".to_string(),
            family: RuntimeHelperFamily::Allocation,
            signature: RuntimeHelperSignature {
                result: HelperResultType::Value,
                calling_convention: super::super::schema::HelperCallingConvention::GcRuntimeCall,
            },
            may_allocate: true,
            may_raise: false,
            may_unwind: false,
            is_safepoint: true,
            requires_roots_visible: false,
            required_capability: None,
            effect: None,
            gc_behavior: HelperGcBehavior::MayAllocateMayCollect,
            jit_call_policy: HelperJitCallPolicy::GcRuntimeCall,
            source_mapping_policy: HelperSourceMappingPolicy::NotRequired,
        };
        let extended = RuntimeHelperRegistry::from_descriptors({
            let mut d = canonical_descriptors();
            d.push(bad);
            d
        })
        .expect("build extended");
        let err = extended.validate().unwrap_err();
        assert!(matches!(
            err,
            RegistryValidationError::MayCollectWithoutRootsVisible(_)
        ));
    }

    #[test]
    fn implementation_consistency_placeholder_checks() {
        let registry = RuntimeHelperRegistry::canonical().expect("canonical build");
        let mut implementations = HelperImplementationRegistry::new();
        implementations.mark_implemented(RuntimeHelperId::new(0));
        let err = registry
            .validate_implementations(&implementations)
            .unwrap_err();
        assert!(matches!(
            err,
            RegistryValidationError::DescriptorWithoutImplementation(_)
        ));

        let mut full = HelperImplementationRegistry::new();
        full.mark_all_from_registry(&registry);
        assert!(registry.validate_implementations(&full).is_ok());
    }

    #[test]
    fn eir_validation_view_includes_may_collect_helpers() {
        let registry = RuntimeHelperRegistry::canonical().expect("canonical build");
        let view = eir_validation_view(&registry);
        assert!(view.contains(RuntimeHelperId::new(0)));
        assert!(view.may_collect(RuntimeHelperId::new(0)));
        assert!(!view.may_collect(RuntimeHelperId::new(2)));
    }

    #[test]
    fn eir_validation_view_includes_may_raise_helpers() {
        let registry = RuntimeHelperRegistry::canonical().expect("canonical build");
        let view = eir_validation_view(&registry);
        assert!(view.may_raise(RuntimeHelperId::new(3)));
        assert!(!view.may_raise(RuntimeHelperId::new(1)));
    }
}