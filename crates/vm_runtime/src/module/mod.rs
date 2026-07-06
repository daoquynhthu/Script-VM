//! Module runtime: state, imports, exports, resolver boundary.
//!
//! Spec: `PHASE-3-MODULE-RUNTIME-CONTRACT.md`, `PHASE-3-HOST-BOUNDARY-CONTRACT.md` §11

pub mod export;
pub mod import;
pub mod instance;
pub mod registry;
pub mod resolver;
pub mod runtime;
pub mod state;
pub mod validate;

pub use export::{ExportEntry, ExportTable};
pub use import::{
    bind_named_import, bind_whole_module_import, is_uninitialized_circular_access,
    read_named_export, reject_failed_provider,
};
pub use instance::{ModuleInstance, ModuleInterfaceDescriptor};
pub use registry::ModuleRegistry;
pub use resolver::{
    resolve_with_capability, CapabilityGatedResolver, CapabilitySet, HostModuleResolver,
    ModuleResolverRequest, StubModuleResolver,
};
pub use runtime::{
    ModuleRuntime, HELPER_IMPORT_MODULE_ID, HELPER_IMPORT_NAMED_ID, HELPER_INITIALIZE_MODULE_ID,
    HELPER_RESOLVE_MODULE_ID, HELPER_SEAL_EXPORTS_ID,
};
pub use state::{validate_transition, validate_transition_with_retry, ModuleState};
pub use validate::{reject_top_level_control, validate_export_plan};