//! Runtime helper registry.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-REGISTRY.md`

pub mod canonical;
pub mod dispatch;
pub mod h1;
pub mod h2;
pub mod h3;
pub mod h4;
pub mod h5;
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
    DEFAULT_MAX_CALL_DEPTH, HELPER_ALLOC_OBJECT_ID, HELPER_ASSERT_FAIL_ID,
    HELPER_ATTACH_SUPPRESSED_ID, HELPER_BIND_METHOD_ID, HELPER_CALL_BUILTIN_ID,
    HELPER_CHECK_ARITY_ID, HELPER_CHECK_CALLABLE_ID, HELPER_CHECK_HASHABLE_ID,
    HELPER_CHECK_TYPE_CONTRACT_ID, HELPER_CLOSE_RESOURCE_ID, HELPER_COMPARE_ID,
    HELPER_CONSTRUCT_ENUM_ID, HELPER_CONSTRUCT_ERROR_ID, HELPER_CONSTRUCT_MAP_ID,
    HELPER_CONSTRUCT_RECORD_ID, HELPER_DISPLAY_ID, HELPER_EXECUTE_DEFER_ID,
    HELPER_GENERIC_CALL_ID, HELPER_GET_ATTRIBUTE_ID, HELPER_IMPORT_MODULE_ID,
    HELPER_IMPORT_NAMED_ID, HELPER_INDEX_READ_ID, HELPER_INDEX_WRITE_ID,
    HELPER_INITIALIZE_MODULE_ID, HELPER_NUMERIC_BINARY_ID, HELPER_PERFORM_UNWIND_ID,
    HELPER_RAISE_ID, HELPER_REGISTER_DEFER_ID, HELPER_REGISTER_RESOURCE_ID,
    HELPER_RESOLVE_MODULE_ID, HELPER_SEAL_EXPORTS_ID, HELPER_SET_ATTRIBUTE_ID,
    HELPER_SLICE_READ_ID, HELPER_WRITE_BARRIER_ID,
};
pub use h3::CallSiteFeedback;
pub use validate::{
    eir_validation_view, validate_helper_reference, validate_registry, RegistryBuildError,
    RegistryValidationError,
};