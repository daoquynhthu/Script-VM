//! Runtime helper registry.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-REGISTRY.md`

pub mod canonical;
pub mod dispatch;
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
pub use dispatch::{dispatch_helper, HELPER_PERFORM_UNWIND_ID};
pub use validate::{
    eir_validation_view, validate_helper_reference, validate_registry, RegistryBuildError,
    RegistryValidationError,
};