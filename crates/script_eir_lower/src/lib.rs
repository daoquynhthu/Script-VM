//! T-P3L bootstrap: validated SIR → EIR for `vm_eval`.
//!
//! Canonical pipeline: `compile_source_via_sir`  
//! (frontend → materialize_sir → validate_ir_unit → lower_sir_to_eir).
//!
//! Distinct from T-DEMO `script_codegen` (AST→EIR shortcut).

pub mod error;
pub mod lower;
pub mod pipeline;
pub mod program;

pub use error::EirLowerError;
pub use lower::lower_sir_to_eir;
pub use pipeline::{compile_source_via_sir, eir_from_source_sir};
pub use program::EirProgram;
