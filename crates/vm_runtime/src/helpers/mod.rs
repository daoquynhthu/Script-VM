//! Runtime helper registry.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-REGISTRY.md`

pub mod canonical;
pub mod dispatch;
pub mod h1;
pub mod registry;
pub mod schema;
pub mod validate;

pub use canonical::canonical_descriptors;
pub use registry::{
    HelperImplementationRegistry, RuntimeHelperRegistry,
};
pub use schema::{
    HelperCallingConvention, HelperGcBehavior, HelperJitCallPolicy, HelperResultType,
    HelperSourceMappingPolicy, RuntimeHelperDescriptor, RuntimeHelperFamily,
    RuntimeHelperSignature,
};
pub use dispatch::{
    dispatch_helper, dispatch_helper_unwind_only, HelperDispatchEnv, HelperDispatchOutcome,
    HELPER_ALLOC_OBJECT_ID, HELPER_CHECK_CALLABLE_ID, HELPER_CHECK_HASHABLE_ID,
    HELPER_CHECK_TYPE_CONTRACT_ID, HELPER_CONSTRUCT_ERROR_ID, HELPER_PERFORM_UNWIND_ID,
    HELPER_WRITE_BARRIER_ID,
};
pub use validate::{
    eir_validation_view, validate_helper_reference, validate_registry, RegistryBuildError,
    RegistryValidationError,
};