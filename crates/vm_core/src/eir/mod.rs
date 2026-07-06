//! Closed EIR representation and validation.
//!
//! Spec: `PHASE-3-EIR-SCHEMA-CLOSURE.md`

pub mod fixtures;
pub(crate) mod resolve;
pub mod schema;
pub(crate) mod wire;
pub mod validate;

pub use schema::{
    EirBlock, EirFunction, EirModule, EirOp, EirOpKind, EirOpKindTag, EirTerminator,
    EirTerminatorKindTag, RootMap, RootMapTable, SafepointRecord, SafepointTable,
};
pub use validate::{
    validate_eir_module, EirModuleInput, EirValidationContext, EirValidationError,
    HelperRegistryView,
};